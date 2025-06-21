# <%= name %> - SurrealDB Migrations

This directory contains the SurrealDB migrations for the <%= name %> project.

## Getting Started

### 1. Configure Environment

Add these to your `.env` file:

```bash
SURREALDB_URL=<%= url %>
SURREALDB_NAMESPACE=<%= namespace %>
SURREALDB_DATABASE=<%= database %>
SURREALDB_ROOT_USER=<%= user %>
SURREALDB_ROOT_PASS=<%= pass %>
```

### 2. Create Module Structure

The project uses a modular architecture. Create your first migration:

```bash
# Generate a migration for a specific module
nx g @deepbrainspace/nx-surrealdb:migration --name=init --module=000_admin --project=<%= name %>
```

Or import pre-built modules (coming soon):

```bash
# Import admin module
nx g @deepbrainspace/nx-surrealdb:import-module --module=@deepbrainspace/surrealdb-module-admin --project=<%= name %>
```

## Structure

```
<%= name %>/
├── config.json             # Module configuration
├── project.json            # NX project configuration
├── 000_admin/              # System administration module
├── 010_auth/               # Authentication module
└── 020_schema/             # Application schema module
```

## Commands

```bash
# Generate a new migration
nx g @deepbrainspace/nx-surrealdb:migration --name=my-migration --module=000_admin --project=<%= name %>

# Run migrations
nx run <%= name %>:migrate

# Check migration status
nx run <%= name %>:status

# Rollback migrations
nx run <%= name %>:rollback

# Reset all migrations (WARNING: destructive)
nx run <%= name %>:reset
```

## Troubleshooting

If you get connection errors:

- Check your .env file has the correct SurrealDB connection details
- Ensure SurrealDB is running (e.g., `surreal start --user root --pass root`)
