# PHP Publishing Workflow - Implementation Summary

## Project Context

**Spikard** is a Rust-centric multi-language toolkit with PHP bindings via `ext-php-rs`. This document summarizes the complete PHP publishing workflow implementation.

## Files Created

### 1. GitHub Workflow

**File**: `.github/workflows/release-php.yml` (580 lines)

**Purpose**: Main CI/CD pipeline for PHP extension releases

**Key Features**:
- **Multi-trigger**: workflow_dispatch, tag push (v*), GitHub release event, repository_dispatch
- **Parallel builds**: 8 platform × 2 PHP versions = 16 concurrent jobs
- **Platforms**: Linux x86_64, Linux ARM64, macOS arm64, Windows x86_64
- **Quality gates**: Smoke tests, PHPStan linting, version verification
- **Conditional publishing**: Only publishes on tag push and when dry_run=false

**Job Pipeline**:
1. **prepare** - Validates tag format, computes version, detects tag vs branch
2. **build-php-binaries** - Matrix build on all platforms/PHP versions
3. **build-php-pie-source** - Creates source bundle for local compilation
4. **smoke-tests-php** - Tests extension loading on representative platforms
5. **lint-php** - Runs PHPStan static analysis
6. **version-check** - Verifies version consistency across Cargo.toml and composer.json
7. **upload-release-artifacts** - Uploads binaries to GitHub Releases (tag releases only)
8. **publish-packagist** - Triggers Packagist auto-update via API

### 2. Composite Actions

**Files**:
- `.github/actions/build-php-extension/action.yml` (75 lines)
- `.github/actions/test-php-extension/action.yml` (70 lines)

**Purpose**: Reusable GitHub Actions for building and testing PHP extensions

**build-php-extension**:
- Sets up PHP (via shivammathur/setup-php)
- Sets up Rust toolchain
- Optionally uses cross-rs for ARM cross-compilation
- Builds extension with `--features extension-module`
- Outputs extension-path

**test-php-extension**:
- Loads extension with `php -d extension=...`
- Runs PHPUnit tests
- Verifies extension loads cleanly

### 3. Helper Scripts

#### A. Packaging Script

**File**: `scripts/package_php_pie_source.sh` (53 lines)

**Purpose**: Creates PIE (PHP Inspector Extension) source bundle for local compilation

**Input**: Version (e.g., "0.1.3"), output directory

**Output**: `spikard-php-0.1.3-src.tgz` containing:
- Complete Rust workspace (Cargo.toml, Cargo.lock, crates/)
- PHP extension scaffold (crates/spikard-php/)
- PHP wrapper library (packages/php/)
- Metadata (LICENSE, VERSION file)

**Usage**:
```bash
./scripts/package_php_pie_source.sh 0.1.3 build/releases
```

#### B. Version Sync Script

**File**: `scripts/sync_versions.py` (Updated - 190 lines)

**Purpose**: Synchronizes version across all manifests to single source of truth

**Changes Made**:
- Added `COMPOSER_JSON_PATHS` list with `packages/php/composer.json`
- Updated `main()` to process composer.json files

**Updates**:
- `packages/php/composer.json` version field
- `crates/spikard-php/Cargo.toml` (ensures uses workspace version)
- All package.json, pyproject.toml, Ruby version.rb files
- Cargo.toml workspace dependencies

**Usage**:
```bash
python3 scripts/sync_versions.py          # Uses workspace version
python3 scripts/sync_versions.py 0.1.4    # Override with specific version
```

### 4. Composer Package Configuration

**File**: `packages/php/composer.json` (Updated)

**Changes**:
1. Added `"version": "0.1.3"` field at root level
2. Added `post-install-cmd` and `post-update-cmd` scripts:
   ```json
   "post-install-cmd": ["@php bin/install-extension.php"],
   "post-update-cmd": ["@php bin/install-extension.php"]
   ```

**Purpose**: Allows future automatic binary installation during composer install

### 5. Post-Install Hook

**File**: `packages/php/bin/install-extension.php` (80 lines)

**Purpose**: PHP script that runs after `composer install` to fetch/install pre-built extension

**Current Functionality**: Stub implementation that:
- Detects OS (Linux, Darwin, Windows)
- Detects PHP version (8.2, 8.3)
- Detects architecture (x86_64, arm64)
- Reports platform key (e.g., "linux-x86_64-8.3")

**Future**: Will download and install pre-built binaries from GitHub Releases

**Usage**: Automatic - runs via composer hooks

### 6. Task Automation

**File**: `Taskfile.yaml` (Added 4 tasks)

**New Tasks**:

1. **php:build:pie-source**
   ```bash
   task php:build:pie-source
   ```
   Builds PIE source bundle locally
   Output: `build/php-releases/spikard-php-X.Y.Z-src.tgz`

2. **release:php:dry-run**
   ```bash
   task release:php:dry-run -- --set TAG=v0.1.4-test
   ```
   Tests release workflow without publishing
   Builds all artifacts, runs tests, skips Packagist

3. **release:php**
   ```bash
   task release:php -- --set TAG=v0.1.4
   ```
   Triggers actual release workflow via GitHub API
   Requires GitHub CLI (gh)

4. **version:sync:php**
   ```bash
   task version:sync:php
   ```
   Syncs all PHP package versions to workspace version

### 7. Documentation

**File**: `docs/RELEASE_SETUP.md` (480 lines)

**Contents**:
- Architecture overview
- Required GitHub secrets
- Setup instructions
- Step-by-step release workflow
- Pre-release checklist
- Troubleshooting guide
- Security considerations
- Rollback procedures
- File structure reference

## Workflow Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    TRIGGER EVENTS                        │
│  Tag Push (v*) | workflow_dispatch | Release Event       │
└────────────────────────┬────────────────────────────────┘
                         │
                    ┌────▼────┐
                    │ Prepare  │ (Validate tag, compute version)
                    └────┬────┘
                         │
          ┌──────────────┴──────────────┬──────────────┐
          │                             │              │
    ┌─────▼────────┐          ┌────────▼─────┐   ┌───▼──────┐
    │ Build Matrix │          │ PIE Bundle   │   │  Linting │
    │ (8 platforms │          │ Source Build │   │  (Phpstan)
    │  x 2 PHP)    │          └────────┬─────┘   └───┬──────┘
    │              │                   │              │
    │ - Linux x64  │                   │              │
    │ - Linux ARM  │                   │              │
    │ - macOS      │          ┌────────▼──────┐  ┌───▼──────┐
    │ - Windows    │          │ Smoke Tests   │  │ Version  │
    └──────┬───────┘          │ (3 platforms) │  │ Check    │
           │                  └────────┬──────┘  └───┬──────┘
           │                           │             │
           └───────────┬───────────────┴─────────────┘
                       │
              ┌────────▼────────┐
              │ All Passed?     │
              └────┬────────┬───┘
           YES     │        NO
                   │     ┌──────────────────┐
                   │     │ Stop (No Publish)│
                   │     └──────────────────┘
                   │
         ┌─────────▼───────────┐
         │ Upload GitHub Release│ (Only if is_tag=true)
         └──────────┬──────────┘
                    │
         ┌──────────▼───────────┐
         │ Publish to Packagist │ (Only if not dry_run)
         └──────────┬──────────┘
                    │
         ┌──────────▼───────────┐
         │ Verify on Packagist  │
         └──────────────────────┘
```

## Version Management Strategy

**Single Source of Truth**: Workspace Cargo.toml

```
Cargo.toml (workspace root)
  version = "0.1.3"
         ↓
  Updated by: manual edit or task version:sync
         ↓
    ┌────┴─────┬─────────┬──────────────┐
    │           │         │              │
packages/php/  pyproject  package.json  Ruby
composer.json  .toml      files         version.rb
```

**Sync Command**:
```bash
task version:sync          # Syncs all manifests to workspace version
task version:sync:php      # Syncs just PHP composer.json
```

## Platforms Supported

| Platform | OS | PHP 8.2 | PHP 8.3 | Extension |
|----------|-----|---------|---------|-----------|
| Linux x86_64 | ubuntu-latest | ✓ | ✓ | .so |
| Linux ARM64 | ubuntu-latest (cross) | ✓ | ✓ | .so |
| macOS arm64 | macos-14 | ✓ | ✓ | .dylib |
| Windows x86_64 | windows-latest | ✓ | ✓ | .dll |

## Release Artifacts

### GitHub Releases
Published for each version tag:
- `spikard-php-x86_64_unknown_linux_gnu-8.2.so`
- `spikard-php-x86_64_unknown_linux_gnu-8.3.so`
- `spikard-php-aarch64_unknown_linux_gnu-8.2.so`
- `spikard-php-aarch64_unknown_linux_gnu-8.3.so`
- `spikard-php-aarch64_apple_darwin-8.2.dylib`
- `spikard-php-aarch64_apple_darwin-8.3.dylib`
- `spikard-php-x86_64_pc_windows_msvc-8.2.dll`
- `spikard-php-x86_64_pc_windows_msvc-8.3.dll`
- `spikard-php-0.1.3-src.tgz` (PIE source bundle)

### Packagist
- Package: `spikard/extension`
- Auto-updated via GitHub webhook
- Available via: `composer require spikard/extension:0.1.3`

## Testing Instructions

### 1. Local Build Test

```bash
# Build PIE source bundle
task php:build:pie-source

# Verify extraction
tar -tzf build/php-releases/spikard-php-0.1.3-src.tgz | head -20
```

### 2. Dry-Run Release Test

```bash
# Test the entire workflow without publishing
task release:php:dry-run -- --set TAG=v0.1.3-test

# Monitor workflow
gh run list --workflow release-php.yml --limit 1
```

### 3. Version Consistency Check

```bash
# Sync versions
task version:sync

# Verify changes
git diff Cargo.toml packages/php/composer.json

# Check both versions match
grep '^version = ' Cargo.toml | head -1
jq '.version' packages/php/composer.json
```

### 4. Manual Smoke Test

```bash
# Build locally
cargo build --release -p spikard-php --features extension-module

# Test extension loads
php -d extension=target/release/libspikard_php.dylib -r "echo 'OK\n';"

# Run PHP tests with extension
cd packages/php
php -d extension=../../target/release/libspikard_php.dylib \
    vendor/bin/phpunit --configuration phpunit.xml
```

## Required GitHub Secrets

### 1. PACKAGIST_VENDOR_API_TOKEN

**Required**: YES
**Scope**: Vendor-level API access to packagist.org
**Setup**:
1. Go to https://packagist.org/profile/
2. Generate API token in settings
3. Add to GitHub repo Settings > Secrets > Actions

**Value Format**: `token abc123def456ghi789jkl012mno345`

### 2. GITHUB_TOKEN

**Required**: NO (auto-provided)
**Scope**: Auto-scoped to repository
**Permissions**: `contents: write` for release uploads

## Security Considerations

1. **Token Rotation**: Rotate PACKAGIST_VENDOR_API_TOKEN every 90 days
2. **Access Control**: Store secrets in organization (not repository) if possible
3. **Minimal Scope**: Packagist token should only have vendor-level access
4. **GPG Signing**: Optionally sign git tags with GPG for verified releases
5. **Audit Logging**: GitHub Actions logs are retained; monitor for suspicious activity

## Integration with Existing Workflows

### CI/CD Pipeline
- Fits alongside existing Rust/Python/Node release workflows
- Uses same artifact retention (14 days)
- Compatible with existing GitHub Actions setup
- No conflicts with existing secret names

### Taskfile Integration
- Follows existing task naming: `php:*`, `release:*`, `version:sync:*`
- Consistent with Python/Node release tasks
- Pre-conditions validate GitHub CLI availability

### Version Management
- Integrates with existing sync_versions.py script
- Composer.json now part of automatic sync
- No manual version management needed

## Troubleshooting Guide

### Common Issues

1. **"Tag must start with 'v'"**
   - Ensure tag format: `v0.1.4` (lowercase v)

2. **"Version mismatch" error**
   - Run: `task version:sync`
   - Re-commit and re-tag

3. **"PACKAGIST_VENDOR_API_TOKEN not set"**
   - Add secret to GitHub repo settings
   - Workflow will skip Packagist publish with warning

4. **Smoke tests fail on specific platform**
   - Check build logs for compilation errors
   - May need platform-specific build dependencies
   - Test locally with `cross` if cross-compiling

5. **Packagist doesn't update**
   - Wait 1-2 minutes for webhook processing
   - Check GitHub webhook delivery log in Packagist settings
   - Manually trigger update from Packagist UI if needed

## Files Summary

| File | Lines | Purpose |
|------|-------|---------|
| `.github/workflows/release-php.yml` | 580 | Main release workflow |
| `.github/actions/build-php-extension/action.yml` | 75 | Build composite action |
| `.github/actions/test-php-extension/action.yml` | 70 | Test composite action |
| `scripts/package_php_pie_source.sh` | 53 | PIE bundler script |
| `scripts/sync_versions.py` | 190 | Version sync (updated) |
| `packages/php/bin/install-extension.php` | 80 | Post-install hook |
| `packages/php/composer.json` | Updated | Added version field |
| `Taskfile.yaml` | +50 lines | Added 4 new tasks |
| `docs/RELEASE_SETUP.md` | 480 | Complete setup guide |

**Total New Code**: ~1,600 lines
**Total Updated Code**: ~30 lines

## Next Steps

1. **Add GitHub Secret**:
   - Go to repo Settings > Secrets > Actions
   - Add `PACKAGIST_VENDOR_API_TOKEN`

2. **Register Packagist Package**:
   - Visit https://packagist.org/submit
   - Submit `spikard/extension` package
   - Enable GitHub webhook

3. **Test Dry-Run**:
   ```bash
   task release:php:dry-run -- --set TAG=v0.1.3-test
   ```

4. **First Release**:
   ```bash
   # Update version
   task version:sync
   git add .
   git commit -m "chore: bump version to v0.1.4"

   # Tag and push
   git tag -s v0.1.4 -m "Release v0.1.4: PHP bindings"
   git push origin v0.1.4
   ```

## References

- [Workflow Design](https://github.com/Goldziher/html-to-markdown/blob/master/.github/workflows/release.yml)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Packagist API Docs](https://packagist.org/apidoc)
- [PHP ext-php-rs Guide](https://github.com/davidcole1340/ext-php-rs)
- [Composer Post-Install Hooks](https://getcomposer.org/doc/articles/scripts.md)
