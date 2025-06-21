# NX SurrealDB Migrations Plugin - Claude Development Guide

## Architecture Overview

This plugin follows a **Repository Pattern** with **Domain-Driven Design**
principles and clean separation of concerns.

**📖 See [ARCHITECTURE.md](./ARCHITECTURE.md) for complete system design,
component responsibilities, data flows, and architectural principles.**

## Key Development Rules for Claude

### **Repository Pattern Enforcement**

- **MigrationService** should NEVER directly access `client` for migration data
  operations
- **Always use MigrationRepository methods** for database operations
- **Keep business logic in Service layer**, data operations in Repository layer

### **Layer Dependencies (Allowed)**

- **Infrastructure** ← All Layers (can be used by any layer)
- **Configuration** ← Domain/Filesystem
- **Filesystem** ← Domain
- **Domain** → Repository → Database

### **Forbidden Communications**

- ❌ Repository calling Service methods
- ❌ Service bypassing Repository for database operations
- ❌ Infrastructure depending on Domain logic

## Core Plugin Architecture

### **Domain Layer** (`src/lib/domain/`)

- **`MigrationRepository`**: All database operations for migration state
- **`MigrationService`**: Orchestrates migration workflows (business logic)
- **`DependencyResolver`**: Module dependency management with topological
  sorting

### **Configuration Layer** (`src/lib/configuration/`)

- **`ConfigLoader`**: Loads and validates `config.json`/`config.yaml`
- Auto-discovers modules if no config exists
- Validates module IDs (pattern: `XXX_name` - e.g., `000_admin`, `010_auth`)

### **Filesystem Layer** (`src/lib/filesystem/`)

- **`MigrationFileProcessor`**: Migration file operations and content processing
- **`TreeUtils`**: NX Tree API utilities for directory/file operations

### **Infrastructure Layer** (`src/lib/infrastructure/`)

- Database client, environment handling, debug utilities

## Plugin Components

### **Executors** (`src/executors/`)

- **`migrate`**: Apply pending migrations with dependency resolution
- **`rollback`**: Rollback applied migrations with safety checks
- **`status`**: Show migration status and dependency info
- **`reset`**: Clear migration tracking table

### **Generators** (`src/generators/`)

- **`init`**: Initialize database project structure (creates config.json,
  project.json, module folders)
- **`migration`**: Create new migration files (up/down pair)
- **`export-module`**: Export module as reusable package
- **`import-module`**: Import module package

**⚠️ IMPORTANT**: The `init` generator exists but is NOT in `generators.json` -
must be added manually!

## Expected File Structure

```
database/                    # Root migrations directory (configurable via initPath)
├── config.json             # Module configuration (required)
├── project.json            # NX project configuration
├── 000_admin/              # System admin module (locked)
│   ├── 0001_init_up.surql
│   ├── 0001_init_down.surql
│   └── 0002_permissions_*.surql
├── 010_auth/               # Auth module (depends on 000_admin)
│   ├── 0001_users_*.surql
│   └── 0002_sessions_*.surql
└── 020_schema/             # Application schema (depends on 010_auth)
    └── 0001_tables_*.surql
```

### **Module ID Pattern**: `XXX_name`

- `XXX`: 3-digit number with gaps (000, 010, 020, etc.)
- `name`: lowercase identifier
- Migration files: `NNNN_name_direction.surql` (0001, 0002, etc.)

## Configuration System

### **config.json Structure**

```json
{
  "modules": {
    "000_admin": {
      "name": "System Administration",
      "depends": [],
      "locked": true,
      "lockReason": "Critical system module"
    },
    "010_auth": {
      "name": "Authentication",
      "depends": ["000_admin"]
    }
  },
  "settings": {
    "useTransactions": true,
    "defaultNamespace": "development",
    "defaultDatabase": "main"
  }
}
```

## Testing and Usage Workflow

### **Development Workflow**

1. **Build package**: `nx run nx-surrealdb:build` (from package folder or root)
2. **Test locally**: Copy `dist/` to
   `node_modules/@deepbrainspace/nx-surrealdb/`
3. **Initialize project**: `nx g @deepbrainspace/nx-surrealdb:init [name]`
4. **Create migrations**:
   `nx g @deepbrainspace/nx-surrealdb:migration --name=my-migration --project=database`
5. **Run migrations**: `nx run database:migrate`

### **Path Resolution**

- Plugin is installed in `node_modules/@deepbrainspace/nx-surrealdb/`
- Database folder is created at project root (e.g., `database/`)
- Executors resolve paths relative to workspace root using NX project
  configuration

## Build Commands

- Build package: `nx run nx-surrealdb:build` (builds to
  `packages/nx-surrealdb/dist/`)
- Run tests: `nx run nx-surrealdb:test`
- Run single test: `nx run nx-surrealdb:test --testFile=src/lib/client.spec.ts`
- Pack for testing: `pnpm pack` (from package directory)

## Package Manager Rules

**CRITICAL**: NEVER use npm. Always use NX commands first, then pnpm.

- ✅ `nx build nx-surrealdb`, `nx test nx-surrealdb`
- ✅ `pnpm install`, `pnpm publish`, `pnpm pack`
- ❌ `npm install`, `npm publish`, `npm pack` (FORBIDDEN)

## Critical Rule: NEVER Skip Tests or Lints

**MANDATORY**: All tests and lints MUST pass before any publish or release.

- ❌ NEVER skip tests or lints
- ❌ NEVER publish with failing tests
- ✅ Always fix the root cause of test/lint failures

## Critical Reminders

- **NEVER edit contents in dist or compiled folder manually**
- Always rebuild the package after making changes
- Schema files are copied during build process
- The `init` generator exists but needs to be added to `generators.json`
- Use existing library functions to avoid code duplication
- Follow Repository Pattern - Services coordinate, Repositories handle data
  access
