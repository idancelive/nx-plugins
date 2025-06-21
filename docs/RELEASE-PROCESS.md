# Release Process

This document describes the release process for the Goodie-Bag monorepo, which
uses Claude commands for intelligent changelog generation and publishing.

## Overview

The release process consists of three steps:

1. **Development & Claude Commands** - Generate changelog-rc.md files using AI
2. **CI Validation** - Automated code quality checks (lint → test → build)
3. **Release Publishing** - Publish packages using Claude commands

## Workflow Architecture

```
 Development      Claude Commands       CI (PR)         Release
     │                  │                │                │
     ▼                  ▼                ▼                ▼
┌─────────┐       ┌──────────┐     ┌──────────┐     ┌─────────┐
│ Code    │       │ release- │     │ Validate │     │ release-│
│ Changes │──────▶│ commit   │────▶│ & Build  │────▶│ publish │
│         │manual │          │ PR  │          │merge│         │
└─────────┘       └──────────┘     └──────────┘     └─────────┘
                        │                │               │
                        ▼                ▼               ▼
                  changelog-rc.md   cached builds   npm packages
```

### Claude Command Integration

Leverages Claude's AI directly for dual-level release management:

1. **AI-Powered Analysis**: Claude analyzes conventional commits for both
   packages and root project
2. **Intelligent Changelog**: Generates professional changelog-rc.md with
   categorized changes
3. **Dual-Level Releases**: Handles both individual packages and monorepo
   infrastructure
4. **Developer Control**: Explicit command execution rather than automatic hooks

## 🔄 Complete Release Flow

### 1. Development with Claude Commands

```bash
# Make changes to packages and/or infrastructure
git checkout -b feat/new-feature
# ... make changes to packages/nx-surrealdb/ and .github/workflows/ ...

# When ready for release, use Claude to generate changelog and commit
claude
/project:release-commit

# Claude will:
# 1. Detect affected packages via nx affected
# 2. Detect root project changes (CI/CD, docs, config)
# 3. Analyze conventional commits since last release
# 4. Calculate appropriate versions (patch/minor/major)
# 5. Generate changelog-rc.md files with RC versions
# 6. Commit and push changes

# Results created:
# - packages/nx-surrealdb/changelog-rc.md (0.2.1-rc.1703123456)
# - changelog-rc.md (goodiebag 1.1.0-rc.1703123456)
```

**Alternative**: Use `/project:changelog` to generate changelog-rc.md without
committing for review first.

### 2. CI Workflow (PR Validation)

When you create a PR, the **CI workflow** validates code quality:

```
┌─────────────┐
│   detect    │ ← Finds affected packages using NX
│     ✅      │
└─────────────┘
       │
   ┌───┴───┐
   │       │
┌──────┐ ┌──────┐
│ lint │ │ test │  ← Parallel validation
│ ✅   │ │ ✅   │
└──────┘ └──────┘
       │
   ┌───┴───┐
   │       │
┌──────┐ ┌──────┐
│build │ │cache │  ← Build and cache artifacts
│ ✅   │ │ ✅   │
└──────┘ └──────┘
```

#### **What Happens:**

1. **🔍 Detection**: NX finds affected packages
2. **⚡ Parallel Validation**: Lint, test, and build each package
3. **📦 Build Caching**: Artifacts cached for release step
4. **📝 Changelog Review**: Team reviews changelog-rc.md in PR

**Note**: The changelog-rc.md file is visible in the PR for team review!

### 3. Release Workflow (Post-merge)

**Use Claude commands** when ready to release:

```bash
# After PR is merged to main
claude
/project:release-publish

# Claude will:
# 1. Find packages and root project with changelog-rc.md files
# 2. Strip RC suffix to get final versions
# 3. Merge changelogs into CHANGELOG.md files
# 4. Update package.json versions
# 5. Publish packages to npm (root project gets GitHub release only)
# 6. Create git tags: goodiebag-v1.1.0, nx-surrealdb-v0.2.1
# 7. Create GitHub releases for both levels
# 8. Clean up RC files and commit changes
```

#### **What Happens:**

1. **🔍 Detection**: Find packages with `changelog-rc.md` files

2. **📝 Changelog Processing**:

   - Strip `-rc.{timestamp}` from version (e.g., `1.2.3-rc.1703123456` →
     `1.2.3`)
   - Merge `changelog-rc.md` content into main `CHANGELOG.md`
   - Update `package.json` with final version

3. **📦 Publishing**:

   - Use **cached builds** from CI
   - Publish to npm with `pnpm publish`
   - Create git tag: `{package}-v{version}`
   - Create GitHub release with changelog

4. **🔄 Finalization**:
   - Delete `changelog-rc.md` files
   - Commit version updates:
     `chore(release): {package}@{version} [skip-changelog]`
   - Push commits and tags

**Safety**: Command includes authentication checks and build validation before
publishing.

## 📋 Version Strategy

### Pre-commit RC Versions

**Packages:**

- **Format**: `x.y.z-rc.{timestamp}` (e.g., `0.2.1-rc.1703123456789`)
- **Location**: `packages/{package}/changelog-rc.md`
- **Purpose**: Preview version for PR review

**Root Project:**

- **Format**: `1.y.z-rc.{timestamp}` (e.g., `1.1.0-rc.1703123456789`)
- **Location**: `changelog-rc.md` (root directory)
- **Tag Pattern**: `goodiebag-v{version}`

### Release Versions

**Packages:**

- **Format**: `x.y.z` (semantic versioning)
- **npm tag**: `latest`
- **GitHub**: Tagged as `{package}-v{version}`

**Root Project:**

- **Format**: `1.y.z` (semantic versioning)
- **GitHub**: Tagged as `goodiebag-v{version}`
- **No npm**: Root project is private, GitHub release only

### Automatic Release Triggers (TODO)

- **Trigger**: Merge to main with changelog-rc.md present
- **Timing**: Immediate or batched (configurable)
- **Safety**: Require approval for major versions
- **Notification**: Slack/Discord webhook on release

```bash
# Version calculation from commits:
# fix: → patch (0.2.0 → 0.2.1)
# feat: → minor (0.2.0 → 0.3.0)
# feat!: or BREAKING CHANGE: → major (0.2.0 → 1.0.0)

# Install released version
pnpm add @deepbrainspace/nx-surrealdb@latest
# or specific version
pnpm add @deepbrainspace/nx-surrealdb@0.2.1
```

## 🎯 Multi-Package Scenarios

### Single Package Changed

```
PR affects: nx-surrealdb
Result: 1 parallel runner for each phase
```

### Multiple Packages Changed

```
PR affects: nx-surrealdb, mcp-server-claude
Result: 2 parallel runners for each phase
- lint (nx-surrealdb) + lint (mcp-server-claude)
- test (nx-surrealdb) + test (mcp-server-claude)
- build (nx-surrealdb) + build (mcp-server-claude)
- etc.
```

### No Packages Affected

```
PR affects: README.md, docs/
Result: "No packages affected" notification
```

## 🛠️ Available Commands

### Claude Commands for Release Management

```bash
# Start Claude and use project commands:
claude

# Generate changelog without committing (for review)
/project:changelog

# Generate changelog and commit (ready for PR)
/project:release-commit

# Publish packages with changelog-rc.md files
/project:release-publish
```

### Testing and Validation

```bash
# Test affected package detection
nx show projects --affected

# Validate builds before release
nx affected --target=build

# Check npm authentication
pnpm whoami

# Verify git status
git status
```

### Manual Override Options

```bash
# Skip Claude commands and use traditional git
git add -A
git commit -m "feat: manual commit without changelog"

# Create changelog-rc.md manually if needed
# Then use: claude release-publish
```

## 🔍 Monitoring & Debugging

### Claude Command Debugging

```bash
# Check affected packages
nx show projects --affected

# Verify changelog-rc.md generation
find packages -name "changelog-rc.md"

# View Claude command files
ls -la .claude/commands/
```

### CI Pipeline Monitoring

- **GitHub Actions**: View lint/test/build execution in Actions tab
- **Build Cache**: Verify artifacts are cached for release
- **PR Reviews**: Check changelog-rc.md files in pull requests

### Release Process Monitoring

```bash
# Check published versions
pnpm view @deepbrainspace/nx-surrealdb versions --json

# Verify git tags
git tag | grep nx-surrealdb

# Check GitHub releases
gh release list --repo deepbrainspace/goodiebag

# View Claude command output
# Claude commands provide detailed output during execution
```

## 🚨 Troubleshooting

### Claude Command Issues

**Command not found:**

```bash
# Verify Claude CLI is installed
claude --version

# Check command files exist
ls .claude/commands/
```

**Changelog not generated:**

```bash
# Check if packages have changes
nx show projects --affected

# Try alternative command
claude changelog  # Generate without committing

# Manual verification
find packages -name "changelog-rc.md"
```

### Release Issues

**Can't find changelog-rc.md:**

```bash
# Check if files exist
ls packages/*/changelog-rc.md

# Generate manually
claude changelog
```

**Publishing failures:**

```bash
# Check npm authentication
pnpm whoami

# Verify build artifacts
nx affected --target=build

# Check versions
pnpm view @deepbrainspace/nx-surrealdb versions
```

**Git issues:**

```bash
# Check git status
git status

# Verify remote connection
git remote -v

# Check if tags exist
git tag | grep nx-surrealdb
```

## System Benefits

### Developer Experience

- **Automated changelog generation** - Created during commit process
- **Version calculation** - Based on conventional commits
- **Skip capability** - Use --no-verify or [skip-changelog] when needed
- **PR visibility** - Changelog visible for review before merge

### Release Process

- **Clear release signals** - changelog-rc.md indicates readiness
- **Independent packages** - Each package maintains own release cycle
- **Build caching** - Reuses CI artifacts during release
- **Clean commits** - Release commits marked with [skip-changelog]

## 🚀 Implementation Status

### ✅ Phase 1: Claude Commands (Complete)

1. ✅ Created `claude release-commit` command
2. ✅ Created `claude changelog` command
3. ✅ Created `claude release-publish` command
4. ✅ Updated documentation

### 🔄 Phase 2: Testing & Validation (Current)

1. Test `claude release-commit` with nx-surrealdb
2. Validate changelog generation and formatting
3. Test complete flow: changelog → CI → publish
4. Verify npm publishing and GitHub releases

### 📋 Phase 3: Optimization (Future)

1. Refine changelog formatting and categorization
2. Add model specification (prefer Sonnet over Opus)
3. Enhance error handling and validation
4. Add more detailed command output

### 🎯 Phase 4: Automation (Future)

1. Integrate release-publish with GitHub Actions
2. Add webhook notifications (Slack/Discord)
3. Implement automatic triggers for certain conditions
4. Add safety checks for major version releases

## 🔗 Related Documentation

- [Claude CLI Documentation](https://claude.ai/code)
- [NX Affected Documentation](https://nx.dev/ci/features/affected)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [pnpm Publishing](https://pnpm.io/cli/publish)
- [GitHub CLI Releases](https://cli.github.com/manual/gh_release)
