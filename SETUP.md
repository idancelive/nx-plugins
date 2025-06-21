# Setup Guide for DeepBrain NX Plugins

This document provides step-by-step instructions for setting up CI/CD and
publishing for the nx-plugins repository.

## Overview

This repository contains NX plugins for DeepBrain projects. The main plugin is
`@deepbrainspace/nx-surrealdb` which provides SurrealDB migration capabilities.

## Current Status

✅ **Completed:**

- Repository structure configured following NX best practices
- Package configuration ready for publishing
- Build process working (`nx build nx-surrealdb` succeeds)
- CI/CD pipelines configured (CircleCI + NX Cloud)
- Documentation created

⏳ **Pending:**

- CI/CD environment setup (tokens and accounts)
- Test suite verification
- First publication to npm
- Integration testing with ileads project

## Setup Instructions

### 1. Repository Structure

The repository is organized as follows:

```
nx-plugins/
├── packages/
│   └── nx-surrealdb/          # Main SurrealDB plugin
│       ├── src/               # TypeScript source code
│       ├── dist/              # Compiled output (generated)
│       ├── package.json       # Package configuration
│       └── project.json       # NX project configuration
├── .github/workflows/         # GitHub Actions (backup)
├── .circleci/config.yml       # CircleCI configuration
├── nx.json                    # NX workspace configuration
├── package.json               # Workspace package.json
└── README.md                  # Main documentation
```

### 2. CircleCI Setup

#### 2.1 Account Setup

1. Go to [CircleCI](https://circleci.com/)
2. Sign up/login with your GitHub account
3. Connect the `nx-plugins` repository

#### 2.2 Environment Variables

Add these environment variables in CircleCI project settings:

| Variable                | Description       | Where to get it                          |
| ----------------------- | ----------------- | ---------------------------------------- |
| `NPM_TOKEN`             | npm publish token | npm → Account → Access Tokens → Generate |
| `NX_CLOUD_ACCESS_TOKEN` | NX Cloud token    | nx.app → Account → Access Tokens         |

#### 2.3 NPM Token Setup

1. Go to [npmjs.com](https://npmjs.com)
2. Create account or login
3. Navigate to Account → Access Tokens
4. Click "Generate New Token"
5. Choose "Automation" type
6. Copy the token to CircleCI environment variables

### 3. NX Cloud Setup

#### 3.1 Account Creation

1. Go to [nx.app](https://nx.app)
2. Sign up with GitHub account
3. Create a new workspace for "nx-plugins"

#### 3.2 Token Configuration

1. Get your access token from NX Cloud dashboard
2. Replace `your-nx-cloud-token-here` in `nx.json` with actual token
3. Or set `NX_CLOUD_ACCESS_TOKEN` environment variable in CircleCI

### 4. First Build Verification

Before setting up CI/CD, verify everything works locally:

```bash
# Install dependencies
pnpm install

# Build the plugin
nx build nx-surrealdb

# Verify build output exists
ls -la packages/nx-surrealdb/dist/

# Test package can be packed
cd packages/nx-surrealdb
npm pack --dry-run
```

### 5. Publishing Workflow

#### 5.1 Automatic Publishing (Recommended)

```bash
# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0

# CircleCI will automatically:
# 1. Run tests
# 2. Build packages
# 3. Publish to npm
```

#### 5.2 Manual Publishing

```bash
# In packages/nx-surrealdb directory
npm version patch
npm publish --dry-run  # Test first
npm publish           # Actual publish
```

### 6. Integration with iLeads Project

Once published, integrate with the ileads project:

#### 6.1 Install the Plugin

```bash
# In ileads project directory
pnpm add @deepbrainspace/nx-surrealdb
```

#### 6.2 Configure NX

Edit `ileads/nx.json`:

```json
{
  "plugins": [
    {
      "plugin": "@deepbrainspace/nx-surrealdb",
      "options": {}
    }
  ]
}
```

#### 6.3 Test Integration

```bash
# Generate a migration
nx g @deepbrainspace/nx-surrealdb:migration test-migration

# Run database commands
nx run database:status
nx run database:migrate
```

## Troubleshooting

### Build Issues

- **TypeScript errors**: Check `skipLibCheck: true` in tsconfig.lib.json
- **Missing files in dist**: Verify project.json asset configuration
- **NX version conflicts**: Ensure consistent versions across package.json files

### CI/CD Issues

- **NPM publish fails**: Verify NPM_TOKEN is correct and has publish permissions
- **NX Cloud errors**: Check access token and workspace configuration
- **CircleCI not triggering**: Ensure repository is connected and config.yml is
  valid

### Plugin Integration Issues

- **Plugin not found**: Verify package is published and installed correctly
- **Generators not working**: Check generators.json and executors.json are
  included in build
- **Schema validation errors**: Verify schema.json files are valid and included

## Next Steps Checklist

- [ ] Create CircleCI account and connect repository
- [ ] Set up npm account and generate publish token
- [ ] Create NX Cloud workspace and get access token
- [ ] Update nx.json with real NX Cloud token
- [ ] Configure environment variables in CircleCI
- [ ] Push code to trigger first CI build
- [ ] Create first release tag to test publishing
- [ ] Install plugin in ileads project
- [ ] Test plugin functionality in ileads
- [ ] Document any additional configuration needed

## Contact & Support

For issues with this setup:

1. Check the troubleshooting section above
2. Review CircleCI build logs for specific errors
3. Verify all environment variables are set correctly
4. Ensure all external accounts (npm, NX Cloud) are properly configured

## Files to Review

When continuing this work in another session, review these key files:

- `packages/nx-surrealdb/package.json` - Package configuration
- `packages/nx-surrealdb/project.json` - Build configuration
- `.circleci/config.yml` - CI/CD pipeline
- `nx.json` - NX workspace configuration
- This `SETUP.md` file for current status
