# Security Guide

## Environment Variables & Secrets Management

This repository uses [git-crypt](https://github.com/AGWA/git-crypt) to encrypt
sensitive files before committing them to the repository.

### Setup git-crypt

1. **Install git-crypt**:

   ```bash
   # macOS
   brew install git-crypt

   # Ubuntu/Debian
   sudo apt-get install git-crypt

   # Windows (via chocolatey)
   choco install git-crypt
   ```

2. **Initialize git-crypt** (one time setup):

   ```bash
   git-crypt init
   git-crypt add-gpg-user YOUR_GPG_KEY_ID
   ```

3. **Create your environment file**:

   ```bash
   cp .env.example .env
   # Edit .env with your actual values
   ```

4. **Verify encryption**:
   ```bash
   git add .env
   git-crypt status
   # Should show: .env: encrypted
   ```

### Encrypted Files

The following files are automatically encrypted when committed:

- `.env` - Main environment variables
- `.env.local` - Local overrides
- `.env.production` - Production environment
- `.env.staging` - Staging environment
- `secrets/**` - Any files in the secrets directory

### Required Environment Variables

| Variable                | Purpose                        | Where to Get                                                       |
| ----------------------- | ------------------------------ | ------------------------------------------------------------------ |
| `NPM_TOKEN`             | Publishing to npm registry     | [npmjs.com/settings/tokens](https://www.npmjs.com/settings/tokens) |
| `GITHUB_TOKEN`          | Creating GitHub releases       | [github.com/settings/tokens](https://github.com/settings/tokens)   |
| `NX_CLOUD_ACCESS_TOKEN` | Distributed caching (optional) | [nx.app](https://nx.app)                                           |
| `SURREALDB_*`           | Local testing database         | Your SurrealDB instance                                            |

### CircleCI Setup

Add these environment variables to your CircleCI project settings:

1. Go to CircleCI project settings
2. Navigate to "Environment Variables"
3. Add each required variable from your `.env` file

### Local Development

1. Copy the example file:

   ```bash
   cp .env.example .env
   ```

2. Fill in your actual values in `.env`

3. The `.env` file will be encrypted when you commit it

### Adding New Team Members

To give a new team member access to encrypted files:

1. **Get their GPG public key**:

   ```bash
   gpg --import their-public-key.asc
   ```

2. **Add them to git-crypt**:

   ```bash
   git-crypt add-gpg-user THEIR_GPG_KEY_ID
   git add .git-crypt/
   git commit -m "Add new team member to git-crypt"
   ```

3. **They can now unlock the repository**:
   ```bash
   git-crypt unlock
   ```

### Security Best Practices

- ✅ **Never commit unencrypted secrets**
- ✅ **Use specific, minimal permissions for tokens**
- ✅ **Rotate tokens regularly**
- ✅ **Use different tokens for different environments**
- ✅ **Review token access periodically**
- ❌ **Don't share tokens via chat/email**
- ❌ **Don't use personal tokens for CI/CD**
