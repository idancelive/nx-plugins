# Libs

This directory contains shared libraries used across apps and packages in the
goodie-bag monorepo.

## Structure

- **UI Components**: Reusable React/Vue components
- **Shared Utils**: Common utilities and helpers
- **Brand Assets**: Logos, icons, design tokens
- **Types**: Shared TypeScript definitions
- **Configs**: Shared configurations (ESLint, TypeScript, etc.)

## Types of Libraries

- **Feature Libraries**: Business logic specific to a domain
- **UI Libraries**: Reusable user interface components
- **Utility Libraries**: Common utilities and helpers
- **Data Access Libraries**: API clients, database utilities

## Usage

```bash
# Generate a new library
nx g @nx/js:library my-lib

# Build library
nx build my-lib

# Test library
nx test my-lib

# Lint library
nx lint my-lib
```

Libraries are **not published** to npm - they're internal dependencies used by
apps and packages.
