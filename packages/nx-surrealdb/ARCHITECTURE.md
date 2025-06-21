# NX SurrealDB Migrations Plugin - Architecture Documentation

## Overview

This plugin implements a comprehensive database migration system for SurrealDB
within NX monorepos, following the **Repository Pattern** with clean separation
of concerns and domain-driven design principles.

## Architectural Principles

### 1. Repository Pattern

- **Data Access Layer**: `MigrationRepository` handles all database operations
- **Business Logic Layer**: `MigrationService` orchestrates workflows and
  business rules
- **Clean Separation**: No business logic in data access, no data access in
  business logic

### 2. Domain-Driven Design

- **Infrastructure**: Database connectivity, utilities, external dependencies
- **Configuration**: Settings, types, environment management
- **Filesystem**: File operations, NX Tree integration
- **Domain**: Core business logic, migration workflows

### 3. Single Responsibility Principle

- Each class has one clear purpose and responsibility
- Methods are focused and cohesive
- Minimal coupling between components

## Directory Structure

```
src/lib/
├── infrastructure/          # External dependencies and utilities
│   ├── client.ts           # SurrealDB client wrapper
│   ├── debug.ts            # Debugging and logging utilities
│   ├── env.ts              # Environment variable handling
│   └── project.ts          # NX project path resolution
├── configuration/          # Configuration and types
│   ├── config-loader.ts    # Module configuration loading
│   └── types.ts           # TypeScript type definitions
├── filesystem/            # File system operations
│   ├── migration-file-processor.ts  # Migration file parsing/processing
│   └── tree-utils.ts                # NX Tree API utilities
└── domain/               # Core business logic
    ├── dependency-resolver.ts       # Module dependency management
    ├── migration-repository.ts      # Data access layer (Repository)
    └── migration-service.ts         # Business logic layer (Service)
```

## Component Responsibilities

### Infrastructure Layer

#### `SurrealDBClient`

- **Purpose**: Database connectivity abstraction
- **Responsibilities**:
  - Connection management (connect, disconnect)
  - Query execution with error handling
  - Transaction management
  - Result formatting

#### `Debug`

- **Purpose**: Centralized logging and debugging
- **Responsibilities**:
  - Scoped logging (per component)
  - Conditional debug output
  - Error logging and formatting

#### `env.ts`

- **Purpose**: Environment variable management
- **Responsibilities**:
  - Variable interpolation (`${VAR}` replacement)
  - `.env` file loading
  - Environment validation

#### `project.ts`

- **Purpose**: NX project integration
- **Responsibilities**:
  - Project path resolution
  - Workspace context handling

### Configuration Layer

#### `ConfigLoader`

- **Purpose**: Module configuration management
- **Responsibilities**:
  - Load and parse `config.json`/`config.yaml`
  - Configuration validation
  - Default value handling
  - Error reporting for malformed configs

#### `types.ts`

- **Purpose**: TypeScript type definitions
- **Responsibilities**:
  - Interface definitions for all data structures
  - Type safety across the application
  - Contract definitions between layers

### Filesystem Layer

#### `MigrationFileProcessor`

- **Purpose**: Migration file operations
- **Responsibilities**:
  - Parse migration filenames (`0001_name_up.surql`)
  - Read and process file contents
  - Content transformation (variable substitution)
  - Checksum generation
  - Migration file validation

#### `TreeUtils`

- **Purpose**: NX Tree API utilities
- **Responsibilities**:
  - Directory operations (find, create, list)
  - File operations (read, write, copy)
  - Module directory discovery
  - Migration number sequence management

### Domain Layer

#### `DependencyResolver`

- **Purpose**: Module dependency management
- **Responsibilities**:
  - Dependency graph construction
  - Topological sorting for execution order
  - Circular dependency detection
  - Rollback order calculation
  - Dependency conflict validation

#### `MigrationRepository` (Data Access Layer)

- **Purpose**: Database operations for migration state
- **Responsibilities**:
  - CRUD operations on `system_migrations` table
  - Migration status queries
  - Data validation (required fields, formats)
  - Raw database interactions
  - Schema initialization

**Methods:**

```typescript
// Simple database operations
async addMigration(record: MigrationRecord): Promise<void>
async getLatestMigrationStatus(number: string, name: string): Promise<Migration | null>
async findLastMigrations(moduleIds: string[]): Promise<Migration[]>
async getMigrationsByDirectionAndPath(direction: string, path: string): Promise<Migration[]>
async updateMigrationStatus(recordId: string, status: 'success' | 'fail'): Promise<void>
```

**What it should NOT do:**

- Business logic about when migrations can be applied
- File operations (reading schema files)
- Complex validation rules
- Workflow decisions

#### `MigrationService` (Business Logic Layer)

- **Purpose**: Migration workflow orchestration
- **Responsibilities**:
  - Coordinate between repository, resolver, and file processor
  - Complex business rules (migration applicability)
  - Workflow management (execution order, error handling)
  - High-level operations (migrate, rollback, status)
  - Transaction coordination

**Methods:**

```typescript
// Complex workflow and rules
async initialize(options: MigrationServiceOptions): Promise<void>
async executeMigrations(modules?: string[]): Promise<MigrationResult>
async validateRollback(modules: string[]): Promise<RollbackValidation>
async findPendingMigrations(modules?: string[]): Promise<MigrationFile[]>
async getMigrationStatus(modules?: string[]): Promise<StatusResult>
```

**What it should NOT do:**

- Direct database operations (use repository methods)
- Raw SQL construction

## Data Flow

### Migration Execution Flow

```
1. MigrationService.executeMigrations()
   ↓
2. findPendingMigrations()
   ├─ DependencyResolver.getExecutionOrder()
   ├─ MigrationFileProcessor.findModuleMigrations()
   └─ MigrationRepository.canApplyMigration()
   ↓
3. For each migration:
   ├─ MigrationFileProcessor.processContent()
   ├─ SurrealDBClient.query()
   └─ MigrationRepository.addMigration()
```

### Rollback Validation Flow

```
1. MigrationService.validateRollback()
   ↓
2. For each module:
   ├─ DependencyResolver.validateRollback()
   ├─ MigrationRepository.getAppliedMigrations()
   └─ MigrationFileProcessor.findModuleMigrations()
   ↓
3. Validate rollback safety
   ├─ Check dependency conflicts
   ├─ Verify rollback files exist
   └─ Return validation result
```

## Communication Patterns

### Layer Communication Rules

1. **Infrastructure ← All Layers**: Infrastructure can be used by any layer
2. **Configuration ← Domain/Filesystem**: Configuration used by domain and
   filesystem
3. **Filesystem ← Domain**: Filesystem utilities used by domain layer
4. **Domain → Repository → Database**: Business logic delegates to repository

### Forbidden Communications

- ❌ Repository calling Service methods (creates circular dependency)
- ❌ Service bypassing Repository for database operations
- ❌ Infrastructure depending on Domain logic
- ❌ Configuration containing business logic

## Error Handling Strategy

### Repository Layer

- **Database Errors**: Wrap in descriptive error messages
- **Validation Errors**: Throw with specific field information
- **Connection Errors**: Bubble up with context

### Service Layer

- **Business Logic Errors**: Detailed validation messages
- **Workflow Errors**: Comprehensive error context
- **Dependency Errors**: Clear conflict descriptions

### Error Propagation

```
Domain (Business Logic Errors)
    ↓ wraps/enriches
Repository (Data Errors)
    ↓ wraps/enriches
Infrastructure (Connection/System Errors)
```

## Testing Strategy

### Unit Testing

- **Repository**: Mock SurrealDBClient, test data operations
- **Service**: Mock Repository, test business logic
- **FileProcessor**: Mock filesystem, test parsing logic
- **DependencyResolver**: Test graph algorithms with fixtures

### Integration Testing

- **Repository + Database**: Real database operations
- **Service + Repository**: End-to-end workflow testing
- **File Operations**: Real file system interactions

### Test Isolation

- Each layer can be tested independently
- Clear interfaces enable easy mocking
- Repository pattern enables database abstraction

## Future Architecture Considerations

### Planned Refactoring

1. **Move `canApplyMigration()` from Repository to Service**

   - Business logic should be in Service layer
   - Repository should only handle data operations

2. **Extract file operations from Repository.initialize()**

   - File reading should be in Service layer
   - Repository should receive processed schema content

3. **Add schema initialization method to Repository**
   - `initializeSchema(content: string): Promise<void>`
   - Removes file dependency from Repository

### Extensibility Points

- **Plugin Architecture**: Could add migration plugins for different operations
- **Multiple Databases**: Repository pattern enables easy database swapping
- **Custom Validators**: Service layer can accommodate custom validation rules
- **Event System**: Could add migration events for monitoring/logging

## Performance Considerations

### Query Optimization

- **Batched Queries**: `findLastMigrations()` uses single query for multiple
  modules
- **Selective Fields**: Only query needed fields, not `SELECT *`
- **Indexed Queries**: Use database indexes for migration status queries

### Memory Management

- **Streaming**: Large migration files handled in streams
- **Lazy Loading**: Configuration loaded only when needed
- **Connection Pooling**: Reuse database connections

### Scalability

- **Horizontal Scaling**: Repository pattern enables connection pooling
- **Caching**: Configuration and dependency graphs can be cached
- **Parallel Execution**: Independent modules could run in parallel (future)

## Security Considerations

### Credential Management

- **Environment Variables**: Never hardcode credentials
- **Connection Encryption**: Always use secure connections in production
- **Access Control**: Repository validates all inputs

### SQL Injection Prevention

- **Parameterized Queries**: All queries use parameter binding
- **Input Validation**: Repository validates all inputs
- **Content Sanitization**: Migration content is validated before execution

### Error Information

- **Credential Hiding**: Errors never expose credentials
- **Sanitized Logging**: Debug logs exclude sensitive information

This architecture ensures maintainability, testability, and scalability while
following industry best practices for database migration systems.
