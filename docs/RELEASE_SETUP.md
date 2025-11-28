# PHP Release Setup and Workflow

This document describes the PHP publishing workflow for Spikard and the secrets/credentials required to run it.

## Overview

The PHP release workflow publishes Spikard PHP bindings to:
1. **GitHub Releases** - Pre-built extension binaries (.so, .dylib, .dll)
2. **Packagist** - Composer package registry for easy installation
3. **PIE Source Bundle** - For users who need to compile locally

## Architecture

The release workflow is defined in `.github/workflows/release-php.yml` and orchestrated through Taskfile tasks.

### Workflow Stages

1. **Prepare** - Validates tags, computes versions, detects if it's a tag push
2. **Build** - Compiles PHP extensions for multiple platforms (Matrix):
   - Linux x86_64 (PHP 8.2, 8.3)
   - Linux ARM64 (PHP 8.2, 8.3)
   - macOS arm64 (PHP 8.2, 8.3)
   - Windows x86_64 (PHP 8.2, 8.3)
3. **PIE Source Bundle** - Creates source distribution for local compilation
4. **Smoke Tests** - Validates extension loading on each platform
5. **Linting** - Runs PHPStan static analysis
6. **Version Check** - Verifies version consistency across manifests
7. **Upload** - Publishes artifacts to GitHub Releases (tag releases only)
8. **Publish** - Notifies Packagist of new version

## Required GitHub Secrets

Add these secrets to your GitHub repository settings (Settings > Secrets and variables > Actions):

### PACKAGIST_VENDOR_API_TOKEN (Required)

**Purpose**: Authenticate with Packagist API to trigger package updates

**How to obtain**:
1. Visit https://packagist.org/profile/
2. Log in with your Packagist account (or create one)
3. Navigate to "API token" section
4. Generate a new API token
5. Copy the token

**Security**:
- Store in GitHub as an organization secret (not repository)
- Rotate every 90 days
- Use minimal scope (vendor-level access only)

**Example value format**:
```
token abc123def456ghi789jkl012mno345
```

### GITHUB_TOKEN (Auto-provided)

**Purpose**: Authenticate with GitHub API for release uploads

**How it works**:
- Automatically provided by GitHub Actions
- Scoped to the repository
- Has `contents: write` permission for this workflow
- No manual setup needed

## Setup Instructions

### 1. Register Spikard Package on Packagist

If not already registered:

1. Visit https://packagist.org/submit
2. Enter GitHub repository URL: `https://github.com/Goldziher/spikard`
3. Click "Check"
4. Review package name (should auto-populate as `spikard/extension`)
5. Click "Submit"

### 2. Enable GitHub Webhook

1. Go to https://packagist.org/profile/ and log in
2. Find "spikard/extension" package
3. Click "Configure" or "Edit"
4. In "GitHub" section, ensure webhook is enabled
5. If auto-update is not enabled, click "Update" or trigger manually

### 3. Add GitHub Secret

1. Go to repository Settings
2. Navigate to "Secrets and variables" > "Actions"
3. Click "New repository secret"
4. **Name**: `PACKAGIST_VENDOR_API_TOKEN`
5. **Value**: Paste your Packagist API token
6. Click "Add secret"

### 4. Verify composer.json Version

Ensure `packages/php/composer.json` has a `version` field:

```json
{
  "name": "spikard/extension",
  "version": "0.1.3",
  "description": "PHP bindings for the Spikard runtime..."
}
```

If missing, run:
```bash
task version:sync:php
```

## Release Workflow

### Releasing a New Version

#### Step 1: Update Versions

Run the version sync to ensure all manifests match:

```bash
# This updates both Cargo.toml and composer.json to the same version
task version:sync
```

Verify changes:
```bash
git diff Cargo.toml packages/php/composer.json
```

#### Step 2: Commit Version Changes

```bash
git add Cargo.toml packages/php/composer.json crates/spikard-php/Cargo.toml
git commit -m "chore: bump version to v0.1.4"
```

#### Step 3: Test Release Workflow (Optional)

Perform a dry-run to validate the workflow before publishing:

```bash
# Builds all artifacts without publishing
task release:php:dry-run -- --set TAG=v0.1.4-test --set REF=main
```

This will:
- Build all platform binaries
- Create PIE source bundle
- Run smoke tests
- Run PHPStan linting
- Verify versions
- Skip Packagist publishing

#### Step 4: Create and Push Tag

```bash
# Create an annotated tag (recommended with GPG signing)
git tag -s v0.1.4 -m "Release v0.1.4: PHP bindings"

# Push tag to trigger workflow
git push origin v0.1.4
```

#### Step 5: Monitor Release Workflow

The workflow will automatically trigger. Monitor progress:

```bash
# View workflow status in GitHub
gh run list --workflow release-php.yml --limit 1

# Stream logs
gh run view --exit-status <run-id> --log
```

The workflow will:
1. Build all platform binaries
2. Create PIE source bundle
3. Run smoke tests
4. Verify versions match
5. Upload to GitHub Releases
6. Notify Packagist

#### Step 6: Verify Release

After workflow completes:

1. **Check GitHub Releases**: https://github.com/Goldziher/spikard/releases
   - Should show v0.1.4 with all binaries and PIE bundle

2. **Check Packagist**: https://packagist.org/packages/spikard/extension
   - New version should appear (may take 1-2 minutes)

3. **Test Installation**:
   ```bash
   # Should be installable via Composer
   composer require spikard/extension:0.1.4
   ```

### Manual Trigger

If tag push doesn't work or you need to re-release:

```bash
# Trigger workflow with custom tag and ref
task release:php -- --set TAG=v0.1.4
```

## Pre-Release Checklist

Before creating a tag:

- [ ] All tests pass: `task test`
- [ ] PHP code lints: `task php:lint:check`
- [ ] Version updated in Cargo.toml
- [ ] `task version:sync` run and committed
- [ ] CHANGELOG.md updated (if applicable)
- [ ] E2E tests pass: `task test:e2e:php`
- [ ] Git working tree clean: `git status`

## Troubleshooting

### Workflow Fails to Build

**Check Rust build**: Run locally first
```bash
cargo build --release -p spikard-php --features extension-module
```

**Check PHP version**: Ensure PHP 8.2+ is in use
```bash
php --version
```

### Version Mismatch Error

Occurs when tag doesn't match Cargo.toml/composer.json version.

**Fix**:
```bash
# Get current workspace version
grep '^version = ' Cargo.toml

# Update to match
task version:sync

# Re-commit and re-tag
git add .
git commit -m "chore: sync versions"
git tag -s v0.1.4 -m "Release v0.1.4"
git push origin v0.1.4
```

### Packagist Update Fails

**Check token**: Verify `PACKAGIST_VENDOR_API_TOKEN` is set
```bash
# In GitHub Actions logs, look for "PACKAGIST_VENDOR_API_TOKEN not set" warning
```

**Manual trigger**: Packagist has a webhook; if auto-update doesn't work:
1. Go to https://packagist.org/packages/spikard/extension
2. Click "Force update" or manually trigger GitHub webhook

### Extension Fails to Load in Smoke Tests

**Check platform match**: Extension must match OS/PHP version

**Verify build output**: Check GitHub Actions logs for build errors

**Test locally**:
```bash
# Build locally
cargo build --release -p spikard-php --features extension-module

# Test loading
php -d extension=target/release/libspikard_php.dylib -r "echo 'OK\n';"
```

## Security Considerations

### Token Management

- **Packagist Token**: Scoped to vendor-level access, rotate every 90 days
- **GitHub Token**: Auto-scoped by Actions, no rotation needed
- Use organization secrets (not repository) for Packagist token
- Audit who has access to repository settings

### Artifact Integrity

- All workflows run on GitHub-hosted runners (no self-hosted)
- Artifacts signed by GitHub (SLSA provenance in beta)
- SHA256 checksums in release notes for manual verification

### Code Signing

For enhanced security, sign git tags with GPG:

```bash
# Configure GPG
git config user.signingkey <your-gpg-key-id>

# Sign tag
git tag -s v0.1.4 -m "Release v0.1.4"

# Verify signature (GitHub will show "Verified" badge)
git tag -v v0.1.4
```

## Rollback Plan

### Automatic Rollback

If any check fails (smoke tests, linting, version checks), the workflow stops and doesn't publish:
- Artifacts remain in GitHub Actions (14 day retention)
- Packagist is NOT updated
- No cleanup needed (just fix the issue and retry)

### Manual Rollback

If a bad version is published to Packagist:

1. **Delete tag** (prevents re-publishing):
   ```bash
   git tag -d v0.1.4
   git push origin --delete v0.1.4
   ```

2. **Yank on Packagist** (mark as unavailable):
   ```bash
   # Go to https://packagist.org/packages/spikard/extension
   # Find version, click "Yank" or use API
   curl -X DELETE \
     https://packagist.org/packages/spikard/extension/v0.1.4 \
     -H "Authorization: token $PACKAGIST_VENDOR_API_TOKEN"
   ```

3. **Create security advisory** (if needed):
   - Go to repository Settings > Security > Advisories
   - Document issue and link patched version

4. **Release patch version**:
   ```bash
   # Bump version and apply fix
   git tag -s v0.1.5 -m "Release v0.1.5: Security patch"
   git push origin v0.1.5
   ```

## File Structure

```
.github/
  workflows/
    release-php.yml              # Main release workflow
  actions/
    build-php-extension/         # Composite action for building
      action.yml
    test-php-extension/          # Composite action for testing
      action.yml

scripts/
  sync_versions.py               # Syncs version across manifests
  package_php_pie_source.sh      # Creates PIE source bundle

packages/php/
  composer.json                  # Packagist metadata (version field)
  bin/
    install-extension.php        # Post-install hook (future binary downloader)

Taskfile.yaml
  release:php:dry-run            # Test release without publishing
  release:php:                   # Trigger actual release
  php:build:pie-source           # Build PIE bundle locally
  version:sync:php               # Sync PHP version
```

## References

- [GitHub Workflows Documentation](https://docs.github.com/en/actions/using-workflows)
- [Packagist API Documentation](https://packagist.org/apidoc)
- [Composer Post Install Hooks](https://getcomposer.org/doc/articles/scripts.md)
- [PHP Inspector Extension (PIE)](https://github.com/noelyoo/php-inspector)
