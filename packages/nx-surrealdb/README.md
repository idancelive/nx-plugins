# nx-surrealdb

A comprehensive SurrealDB toolkit for [Nx](https://nx.dev/) monorepos featuring
migration management, dependency resolution, and extensible tooling
architecture.

## Features

### 🚀 **Migration Management**

- **`migrate`**: Apply pending migrations with dependency resolution
- **`rollback`**: Safe rollback with dependency conflict detection
- **`status`**: Rich status visualization with dependency graphs

### 🔧 **Extensible Architecture**

- **Modular Design**: Foundation for expanding SurrealDB tooling beyond
  migrations
- **Future Features**: Schema generators, seeder utilities, query builders, and
  more
- **Plugin Architecture**: Extensible framework for SurrealDB development
  workflows

### 🔄 **Dependency Management**

- **Module Dependencies**: JSON/YAML configuration with explicit dependency
  declarations
- **Topological Sorting**: Automatic execution order based on dependency graphs
- **Circular Detection**: Prevents circular dependency configurations
- **Safety Validation**: Blocks unsafe rollbacks that would break dependent
  modules

### 📊 **Rich Visualization**

- **ASCII Dependency Trees**: Beautiful console visualization of module
  relationships
- **Status Indicators**: Clear up-to-date vs pending migration states
- **JSON Output**: Machine-readable output for automation and CI/CD integration
- **Detailed Mode**: Show specific pending migration files and metadata

### 🛡️ **Safety & Reliability**

- **Migration Tracking**: Complete history in `system_migrations` table with
  checksums and timing
- **Rollback Safety**: Pre-validation prevents dependency conflicts
- **Force Override**: Bypass safety checks when needed for emergency situations
- **Dry-Run Mode**: Preview operations without executing changes
- **Transaction Control**: Optional transaction wrapping for atomicity

### 🎯 **Developer Experience**

- **Smart Module Targeting**: Reference modules by index (`1`), name (`auth`),
  number (`10`), or full path (`010_auth`)
- **Granular File Targeting**: Reference specific migration files by index
  (`1`), name (`auth`), or full filename
- **Multiple Reference Patterns**: Mix and match module and filename patterns in
  a single command (`--module 0,auth,20 --filename 1,2`)
- **Environment Variables**: Full `.env` support with variable interpolation
- **Rich Logging**: Emoji-enhanced console output with detailed execution
  statistics
- **Error Handling**: Comprehensive error messages with actionable guidance

## Prerequisites

- **Node.js**: Version 16 or higher
- **Nx**: Version 16 or higher (`npm install -g nx`)
- **SurrealDB**: A running SurrealDB instance (local or cloud)
- **TypeScript**: Recommended for type safety (included in Nx projects)

## Installation

1. **Add the Plugin to Your Nx Workspace**:

   ```bash
   npm install @deepbrainspace/nx-surrealdb --save-dev
   # or
   pnpm add -D @deepbrainspace/nx-surrealdb
   ```

2. **Alternative: GitHub Releases**

   Download packages directly from
   [GitHub Releases](https://github.com/deepbrainspace/goodiebag/releases):

   - **Production releases**: Tagged versions (e.g., `nx-surrealdb-v1.0.0`)
   - **SHA-based releases**: Main branch builds with commit SHA (e.g.,
     `nx-surrealdb-a59d989`)

   ```bash
   # Download and install from GitHub release
   curl -L https://github.com/deepbrainspace/goodiebag/releases/download/nx-surrealdb-v1.0.0/deepbrainspace-nx-surrealdb-1.0.0.tgz -o package.tgz
   npm install package.tgz
   ```

3. **Verify Installation**:
   ```bash
   nx list @deepbrainspace/nx-surrealdb
   ```

## Quick Start

### 1. Create a Database Project

```bash
# Generate a new database project
nx g @nx/node:application database --bundler=webpack --framework=none

# Or add to existing project by updating project.json
```

### 2. Configure Project Targets

Update your `database/project.json`:

```json
{
  "name": "database",
  "targets": {
    "migrate": {
      "executor": "@deepbrainspace/nx-surrealdb:migrate",
      "options": {
        "url": "${SURREALDB_URL}",
        "user": "${SURREALDB_ROOT_USER}",
        "pass": "${SURREALDB_ROOT_PASS}",
        "namespace": "${SURREALDB_NAMESPACE}",
        "database": "${SURREALDB_DATABASE}",
        "initPath": "database"
      }
    },
    "rollback": {
      "executor": "@deepbrainspace/nx-surrealdb:rollback",
      "options": {
        "url": "${SURREALDB_URL}",
        "user": "${SURREALDB_ROOT_USER}",
        "pass": "${SURREALDB_ROOT_PASS}",
        "namespace": "${SURREALDB_NAMESPACE}",
        "database": "${SURREALDB_DATABASE}",
        "initPath": "database"
      }
    },
    "status": {
      "executor": "@deepbrainspace/nx-surrealdb:status",
      "options": {
        "url": "${SURREALDB_URL}",
        "user": "${SURREALDB_ROOT_USER}",
        "pass": "${SURREALDB_ROOT_PASS}",
        "namespace": "${SURREALDB_NAMESPACE}",
        "database": "${SURREALDB_DATABASE}",
        "initPath": "database"
      }
    }
  }
}
```

### 3. Set Up Environment Variables

Create `.env` in your workspace root:

```bash
# SurrealDB Connection
SURREALDB_URL=ws://localhost:8000
SURREALDB_ROOT_USER=root
SURREALDB_ROOT_PASS=root
SURREALDB_NAMESPACE=myapp
SURREALDB_DATABASE=main
```

### 4. Create Module Configuration

Create `database/config.json`:

```json
{
  "modules": {
    "000_admin": {
      "name": "System Administration",
      "description": "Core system setup and administrative functions",
      "depends": []
    },
    "010_auth": {
      "name": "Authentication & Users",
      "description": "User authentication and authorization system",
      "depends": ["000_admin"]
    },
    "020_schema": {
      "name": "Application Schema",
      "description": "Core application data models and relationships",
      "depends": ["010_auth"]
    }
  },
  "settings": {
    "configFormat": "json",
    "useTransactions": true,
    "defaultNamespace": "myapp",
    "defaultDatabase": "main"
  }
}
```

### 5. Create Migration Directory Structure

```
database/
├── 000_admin/
│   ├── 0001_setup_up.surql
│   └── 0001_setup_down.surql
├── 010_auth/
│   ├── 0001_users_up.surql
│   ├── 0001_users_down.surql
│   ├── 0002_sessions_up.surql
│   └── 0002_sessions_down.surql
├── 020_schema/
│   └── ...
└── config.json
```

## Usage

### Generate New Migrations

```bash
# Generate migration in existing module
nx g @deepbrainspace/nx-surrealdb:migration create-users --project database --module auth

# Generate migration with new module creation
nx g @deepbrainspace/nx-surrealdb:migration setup-notifications --project database --module notifications --createModule
```

### Apply Migrations

```bash
# Apply all pending migrations
nx run database:migrate

# Apply migrations for specific module (multiple ways)
nx run database:migrate --module 1          # Index-based (2nd module)
nx run database:migrate --module 10         # Number-based (module 010)
nx run database:migrate --module auth       # Name-based
nx run database:migrate --module 010_auth   # Full name

# Apply multiple modules
nx run database:migrate --module 0,1        # First two modules
nx run database:migrate --module admin,auth # By names
nx run database:migrate --module 0,auth,20  # Mixed patterns

# Dry run to preview what would be applied
nx run database:migrate --dryRun
nx run database:migrate --module auth --dryRun

# Force apply even if already applied
nx run database:migrate --force

# Target specific migration files
nx run database:migrate --filename 1          # First migration file
nx run database:migrate --filename auth       # Migration file with 'auth' in name
nx run database:migrate --filename 0001_setup_up.surql  # Exact filename

# Combine module and filename targeting
nx run database:migrate --module auth --filename 1      # First migration in auth module
nx run database:migrate --module 0,1 --filename 2       # Second migration in first two modules

# Target multiple specific files
nx run database:migrate --filename 1,2,auth    # Multiple files using mixed patterns
```

### Check Status

```bash
# Show overall migration status
nx run database:status

# Show status for specific module (multiple ways)
nx run database:status --module 1          # Index-based (2nd module)
nx run database:status --module 10         # Number-based (module 010)
nx run database:status --module auth       # Name-based
nx run database:status --module 010_auth   # Full name

# Show status for multiple modules
nx run database:status --module 0,1,2      # All modules by index
nx run database:status --module admin,auth # Multiple by name

# Show detailed information with file names and timing
nx run database:status --detailed
nx run database:status --module auth --detailed

# Output as JSON for automation
nx run database:status --json
nx run database:status --module auth --json

# Check status of specific migration files
nx run database:status --filename 1           # Status of first migration file
nx run database:status --filename auth        # Status of auth migration file
nx run database:status --filename 1,2,auth --detailed  # Multiple files with details

# Combine module and filename for precise status
nx run database:status --module auth --filename 1 --json
```

### Rollback Migrations

```bash
# Rollback specific module (with dependency safety validation)
nx run database:rollback --module 1          # Index-based (2nd module)
nx run database:rollback --module 10         # Number-based (module 010)
nx run database:rollback --module auth       # Name-based
nx run database:rollback --module 010_auth   # Full name

# Rollback multiple modules (in dependency-safe order)
nx run database:rollback --module 1,0        # Rollback auth, then admin
nx run database:rollback --module auth,admin # Same as above, by name

# Dry run to preview what would be rolled back
nx run database:rollback --module auth --dryRun
nx run database:rollback --module 0,1 --dryRun

# Show detailed rollback information
nx run database:rollback --module auth --detailed
nx run database:rollback --module auth --dryRun --detailed

# Force rollback (bypass dependency safety checks - use with caution!)
nx run database:rollback --module auth --force

# Rollback specific number of steps
nx run database:rollback --module auth --steps 2

# Rollback specific migration files (automatically finds _down.surql files)
nx run database:rollback --filename 1           # Rollback first migration file
nx run database:rollback --filename auth        # Rollback auth migration file
nx run database:rollback --filename 0001_setup_up.surql  # Rollback specific file

# Combine module and filename for precise rollback
nx run database:rollback --module auth --filename 1     # Rollback first migration in auth module
nx run database:rollback --module 0,1 --filename 2      # Rollback second migration in first two modules

# Rollback multiple specific files
nx run database:rollback --filename 1,2,auth    # Multiple files using mixed patterns
```

## Common Workflows

### Quick Development Workflow

```bash
# Check what needs to be done
nx run database:status

# Apply pending migrations to the auth module
nx run database:migrate --module 1          # Using index (quick to type)

# Check status again
nx run database:status --module 1 --detailed
```

### Safe Production Deployment

```bash
# Preview all changes first
nx run database:migrate --dryRun

# Apply migrations module by module for safety
nx run database:migrate --module admin      # Core system first
nx run database:migrate --module auth       # Then authentication
nx run database:migrate --module schema     # Finally application schema

# Verify everything is applied
nx run database:status
```

### Emergency Rollback Workflow

```bash
# Check current state
nx run database:status --detailed

# Preview rollback (recommended first step)
nx run database:rollback --module auth --dryRun --detailed

# Safe rollback with dependency validation
nx run database:rollback --module auth

# If blocked by dependencies, rollback dependents first
nx run database:rollback --module schema    # Rollback dependent module first
nx run database:rollback --module auth      # Then target module

# Emergency override (use with extreme caution)
# nx run database:rollback --module auth --force
```

### Team Development Best Practices

```bash
# Always check status before starting work
nx run database:status

# Use descriptive module names for clarity in team scripts
nx run database:migrate --module authentication
nx run database:status --module user-management

# Use indices for quick interactive commands
nx run database:status --module 0,1,2       # Check first three modules
nx run database:migrate --module 1          # Quick migrate second module
```

### Working with Locked Modules

```bash
# Check which modules are locked
nx run database:status --detailed

# Attempt rollback (will be blocked)
nx run database:rollback --module admin
# Output: 🔒 Rollback locked - cannot rollback protected modules!

# Preview what would happen with force override
nx run database:rollback --module admin --dryRun --force

# Emergency rollback with force override (use with extreme caution)
nx run database:rollback --module admin --force

# Migrations work normally on locked modules
nx run database:migrate --module admin      # ✅ Allowed
```

## Configuration

### Module Dependencies

The `config.json` file defines module dependencies:

```json
{
  "modules": {
    "000_admin": {
      "name": "System Administration",
      "depends": []
    },
    "010_auth": {
      "name": "Authentication",
      "depends": ["000_admin"]
    },
    "020_messaging": {
      "name": "Messaging System",
      "depends": ["010_auth"]
    },
    "030_notifications": {
      "name": "Notifications",
      "depends": ["010_auth", "020_messaging"]
    }
  }
}
```

### Module Lock Protection

Protect critical modules from accidental rollbacks by adding lock configuration:

```json
{
  "modules": {
    "000_admin": {
      "name": "System Administration",
      "description": "Core database setup and administrative functions",
      "depends": [],
      "locked": true,
      "lockReason": "Critical system module - contains core admin setup and permissions"
    },
    "010_auth": {
      "name": "Authentication & Users",
      "description": "User authentication and authorization system",
      "depends": ["000_admin"],
      "locked": true,
      "lockReason": "Core authentication system - rollback would break user access"
    },
    "020_schema": {
      "name": "Application Schema",
      "description": "Core application data models and relationships",
      "depends": ["010_auth"]
    }
  }
}
```

#### Lock Configuration Properties

- **`locked`** (boolean, optional): When `true`, prevents rollback of this
  module
- **`lockReason`** (string, optional): Human-readable explanation for why the
  module is locked

#### Lock Protection Features

- 🔒 **Visual Indicators**: Locked modules display with lock icons in status
  output
- 🛡️ **Rollback Prevention**: Automatically blocks rollback attempts on locked
  modules
- 📝 **Clear Messaging**: Shows specific lock reasons when rollback is blocked
- ⚡ **Force Override**: Use `--force` flag to bypass lock protection for
  emergencies
- 🎯 **Selective Locking**: Lock only critical modules, leave development
  modules unlocked

### Executor Options

#### Common Options (all executors)

- `url`: SurrealDB connection URL
- `user`: SurrealDB username
- `pass`: SurrealDB password
- `namespace`: SurrealDB namespace
- `database`: SurrealDB database
- `module`: Target specific module (string or number)
- `filename`: Target specific migration file (string or number)
- `envFile`: Path to environment file
- `initPath`: Path to migrations directory (default: "database")
- `configPath`: Path to config file (default: auto-detected)

#### Migrate-specific Options

- `dryRun`: Preview migrations without applying
- `force`: Apply migrations even if already applied
- `useTransactions`: Wrap migrations in transactions (default: true)

#### Rollback-specific Options

- `dryRun`: Preview rollbacks without applying
- `force`: Bypass safety validation checks
- `steps`: Number of migration steps to rollback (default: 1)

#### Status-specific Options

- `detailed`: Show detailed migration file information
- `json`: Output as JSON instead of human-readable format

## Migration File Format

### Up Migration (`*_up.surql`)

```sql
-- Create users table
DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS email ON users TYPE string;
DEFINE FIELD IF NOT EXISTS password ON users TYPE string;
DEFINE INDEX IF NOT EXISTS email_idx ON users FIELDS email UNIQUE;
```

### Down Migration (`*_down.surql`)

```sql
-- Remove users table
REMOVE INDEX IF EXISTS email_idx ON users;
REMOVE TABLE IF EXISTS users;
```

## Module Structure

### Gapped Numbering

Use gapped numbering (000, 010, 020, 030) to allow insertion of new modules:

```
000_admin     # System administration
010_auth      # Authentication
020_schema    # Core schema
030_messaging # Messaging system
040_reporting # Reporting (can be inserted later)
```

### Module Reference Patterns

The plugin supports multiple intuitive ways to specify modules, making it easy
for developers to target the modules they need:

#### **Index-Based (Most User-Friendly)**

Reference modules by their position in sorted order:

- `--module 0` → `000_admin` (first module)
- `--module 1` → `010_auth` (second module)
- `--module 2` → `020_schema` (third module)

#### **Number-Based (Direct Mapping)**

Reference modules by their numeric prefix:

- `--module 10` → `010_auth`
- `--module 20` → `020_schema`
- `--module 0` → `000_admin`

#### **Name-Based (Semantic)**

Reference modules by their descriptive name:

- `--module auth` → `010_auth`
- `--module admin` → `000_admin`
- `--module schema` → `020_schema`

#### **Full Name (Explicit)**

Reference modules by their complete directory name:

- `--module 010_auth` → `010_auth`
- `--module 000_admin` → `000_admin`
- `--module 020_schema` → `020_schema`

#### **Multiple Modules**

Combine any reference patterns with comma separation:

- `--module 0,1` → `000_admin,010_auth`
- `--module admin,auth,schema` → `000_admin,010_auth,020_schema`
- `--module 0,auth,20` → `000_admin,010_auth,020_schema`

**💡 Pro Tip**: Index-based referencing (`--module 1`) is often the quickest for
interactive use, while name-based (`--module auth`) is most readable for scripts
and documentation.

### Filename Reference Patterns

The plugin also supports granular filename targeting within modules, allowing
you to run specific migration files instead of entire modules:

#### **Numeric Patterns**

Reference migration files by their numeric sequence:

- `--filename 1` → `0001_setup_up.surql` (first migration file)
- `--filename 2` → `0002_users_up.surql` (second migration file)

#### **Name Patterns**

Reference migration files by their descriptive name:

- `--filename auth` → `0001_authentication_up.surql`
- `--filename users` → `0002_users_up.surql`

#### **Full Filename**

Reference migration files by their complete name:

- `--filename 0001_authentication_up.surql` → exact match

#### **Combined Module + Filename Targeting**

For precise control, combine module and filename patterns:

- `--module auth --filename 1` → First migration in auth module only
- `--module 0,1 --filename 2` → Second migration in first two modules

#### **Multiple Filenames**

Target multiple files with comma separation:

- `--filename 1,2,auth` → Multiple files using mixed patterns
- `--filename 0001,0002` → Multiple files by number

**💡 Pro Tip**: Filename patterns work with all executors (migrate, rollback,
status) and automatically handle `_up.surql` vs `_down.surql` file resolution.

## Console Output Examples

### Status Command

```
📊 Checking migration status...

📈 Migration Status Summary
   Total Applied: 8
   Total Pending: 2
   🔄 2 migration(s) pending

📋 Module Details:

   ✅ 000_admin [UP-TO-DATE] 🔒
      Applied: 3 migration(s)
      Last Applied: 2024-01-15T10:30:00.000Z
      🔒 Locked: Critical system module - contains core admin setup and permissions

   🔄 010_auth [PENDING] 🔒
      Applied: 2 migration(s)
      Pending: 1 migration(s)
      Dependencies: 000_admin
      🔒 Locked: Core authentication system - rollback would break user access

   ✅ 020_schema [UP-TO-DATE]
      Applied: 3 migration(s)
      Dependencies: 010_auth

🌐 Dependency Graph:
   000_admin (root)
   └─ 010_auth
      └─ 020_schema
```

### Migrate Command

```
🚀 Starting migration execution...
✅ Migration completed successfully!
   Files processed: 3
   Files skipped: 0
   Execution time: 1,247ms

📊 Migration Details:
   ✅ 000_admin/0001_setup_up.surql
   ✅ 010_auth/0001_users_up.surql
   ✅ 010_auth/0002_sessions_up.surql
```

### Rollback Safety Validation

```
🔍 Validating rollback safety...
❌ Rollback validation failed!
   Blocked by dependencies:
   • 020_schema
   • 030_messaging
   Warnings:
   • Module 010_auth has active dependents

💡 Use --force to bypass safety checks
```

### Module Lock Protection

```
🔒 Rollback locked - cannot rollback protected modules!
   Locked modules:
   🔒 000_admin: Critical system module - contains core admin setup and permissions

💡 To resolve this:
   Option 1: Remove modules from the locked list in config.json
   Option 2: Use --force to bypass lock protection (use with extreme caution)
```

## Best Practices

### 1. **Module Organization**

- Use descriptive module names that reflect functional areas
- Keep modules focused on single concerns
- Use gapped numbering to allow future insertions

### 2. **Migration Writing**

- Always write corresponding down migrations
- Use `IF NOT EXISTS` and `IF EXISTS` for idempotent operations
- Test migrations in development before applying to production

### 3. **Dependency Management**

- Clearly define module dependencies in config.json
- Avoid circular dependencies
- Keep dependency chains shallow when possible

### 4. **Module Lock Protection**

- Lock critical modules to prevent accidental rollbacks
- Use descriptive lock reasons to explain why modules are protected
- Reserve locks for essential infrastructure modules (admin, core schema)
- Document locked modules in team procedures

### 5. **Safety Practices**

- Use dry-run mode to preview changes
- Validate rollback safety before applying
- Use force flag sparingly and with caution
- Test migration paths in development environments

### 6. **Environment Management**

- Use environment variables for all connection details
- Never commit credentials to version control
- Use different databases for different environments

## Troubleshooting

### Common Issues

#### 1. **Connection Errors**

```bash
# Verify SurrealDB is running
surreal start --log trace --user root --pass root memory

# Check environment variables
echo $SURREALDB_URL
```

#### 2. **Module Not Found**

```bash
# List available modules
nx run database:status

# Check module naming (case sensitive)
ls database/
```

#### 3. **Dependency Conflicts**

```bash
# Check dependency graph
nx run database:status --detailed

# Validate rollback safety
nx run database:rollback --module mymodule --dryRun
```

#### 4. **Migration State Issues**

```bash
# Check current state
nx run database:status --module mymodule --detailed

# Force apply if needed (use with caution)
nx run database:migrate --module mymodule --force
```

#### 5. **Module Lock Issues**

```bash
# Check which modules are locked
nx run database:status --detailed

# Identify locked modules blocking rollback
nx run database:rollback --module mymodule --dryRun

# Remove lock from config.json (for non-critical modules)
# Edit database/config.json and remove "locked": true

# Emergency override (use extreme caution)
nx run database:rollback --module mymodule --force
```

#### 6. **Lock Configuration Errors**

```bash
# Validate config syntax
nx run database:status

# Common config issues:
# - Missing comma after "depends": ["other_module"]
# - Typo in "locked": true (must be boolean)
# - Missing quotes around lockReason string
```

## Code Architecture

### 🏗️ **Repository Pattern Architecture**

This plugin follows the **Repository Pattern** with clean separation of concerns
and domain-driven design:

```
src/lib/
├── infrastructure/          # Database connectivity, utilities
│   ├── client.ts           # SurrealDB client wrapper
│   ├── debug.ts            # Debugging utilities
│   ├── env.ts              # Environment variable handling
│   └── project.ts          # NX project integration
├── configuration/          # Configuration management
│   ├── config-loader.ts    # Module config loading
│   └── types.ts           # Type definitions
├── filesystem/            # File system operations
│   ├── migration-file-processor.ts  # Migration file handling
│   └── tree-utils.ts                # NX Tree utilities
└── domain/               # Core business logic
    ├── dependency-resolver.ts       # Module dependency management
    ├── migration-repository.ts      # Data access layer
    ├── migration-service.ts         # Business logic orchestration
    └── module-lock-manager.ts       # Module lock protection
```

### 🔄 **Repository Pattern Implementation**

#### **MigrationRepository** (Data Access Layer)

**Responsibility**: Database operations for migration state management

```typescript
// Simple CRUD operations
async addMigration(record: MigrationRecord): Promise<void>
async findLastMigrations(moduleIds: string[]): Promise<Migration[]>
async getLatestMigrationStatus(number: string, name: string): Promise<Migration | null>
```

#### **MigrationService** (Business Logic Layer)

**Responsibility**: Orchestrate migration workflows and business rules

```typescript
// Complex workflow and rules
async executeMigrations(modules?: string[]): Promise<MigrationResult>
async validateRollback(modules: string[]): Promise<RollbackValidation>
async findPendingMigrations(modules?: string[]): Promise<MigrationFile[]>
```

#### **ModuleLockManager** (Security Layer)

**Responsibility**: Module lock protection and validation

```typescript
// Lock validation and management
validateRollbackLock(moduleIds: string[]): { canRollback: boolean; blockedModules: string[] }
validateMigrationLock(moduleIds: string[]): { canMigrate: boolean; blockedModules: string[] }
isModuleLocked(moduleId: string): boolean
```

#### **Communication Pattern**

```
MigrationService (Business Logic)
    ↓ delegates data operations
MigrationRepository (Data Access)
    ↓ executes queries
SurrealDBClient (Database)

ModuleLockManager (Security)
    ↓ validates lock policies
MigrationService (Business Logic)
```

### 🎯 **Design Benefits**

✅ **Single Responsibility**: Each class has one clear purpose  
✅ **Testability**: Data access can be mocked, business logic tested
separately  
✅ **Maintainability**: Changes to business rules don't affect data access  
✅ **Scalability**: Can swap database implementations without changing business
logic  
✅ **Repository Pattern**: Industry-standard data access abstraction

### 🧪 **Test-Driven Development**

- **Comprehensive Test Coverage**: All components follow TDD methodology
- **Layer Testing**: Repository and Service layers tested independently
- **Integration Tests**: End-to-end workflow validation
- **Mock-friendly**: Clean interfaces enable easy mocking for unit tests

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Ensure all tests pass: `nx test nx-surrealdb`
5. Submit a pull request

## Development & Contributing

### Local Development

```bash
# Build the plugin
nx build nx-surrealdb

# Run tests
nx test nx-surrealdb

# Run linting
nx lint nx-surrealdb

# Test locally by copying to node_modules
cp -r dist/* node_modules/@deepbrainspace/nx-surrealdb/
```

### Local Development & Testing

```bash
# Build the plugin
nx build nx-surrealdb

# Run tests with coverage
nx test nx-surrealdb --code-coverage

# Run linting (zero warnings required)
nx lint nx-surrealdb

# Test locally in another project
cd packages/nx-surrealdb
npm pack
# Copy *.tgz to test project and: npm install package.tgz
```

### Testing Your Plugin

Create a test NX workspace to validate functionality:

```bash
# Create test workspace
npx create-nx-workspace@latest test-workspace --preset=empty
cd test-workspace

# Install your local plugin
npm install /path/to/nx-plugins/packages/nx-surrealdb/*.tgz

# Test plugin functionality
nx g @deepbrainspace/nx-surrealdb:init database
nx g @deepbrainspace/nx-surrealdb:migration setup --project=database
```

### Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed system design, including:

- Repository Pattern implementation
- Domain-Driven Design principles
- Component interaction diagrams
- Data flow documentation

## License

MIT License - see LICENSE file for details.

## Support

- [GitHub Issues](https://github.com/deepbrainspace/goodiebag/issues)
- [Documentation](https://github.com/deepbrainspace/goodiebag/tree/main/packages/nx-surrealdb)
- [SurrealDB Community](https://surrealdb.com/community)
