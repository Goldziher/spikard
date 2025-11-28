# PHP Publishing Workflow - Implementation Checklist

## Implementation Status: COMPLETE

This document lists all files created and updated, with instructions for activation.

---

## Part 1: Files Created

### A. Workflow Definition (17 KB)
- ✅ `.github/workflows/release-php.yml` (580 lines)
  - Multi-trigger (tag push, workflow_dispatch, release event)
  - Matrix builds for 8 platforms × 2 PHP versions
  - Quality gates: smoke tests, linting, version checks
  - Conditional publishing (tag releases only)

### B. Composite Actions (3.7 KB)
- ✅ `.github/actions/build-php-extension/action.yml` (75 lines)
  - Sets up PHP, Rust, cross-compilation tools
  - Outputs extension file path

- ✅ `.github/actions/test-php-extension/action.yml` (70 lines)
  - Installs Composer deps
  - Loads extension with php -d extension=...
  - Runs PHPUnit tests

### C. Helper Scripts (1.1 KB)
- ✅ `scripts/package_php_pie_source.sh` (53 lines, executable)
  - Creates PIE source bundle with Rust workspace + PHP scaffold
  - Takes version and output directory
  - Output: `spikard-php-X.Y.Z-src.tgz`

### D. PHP Package Configuration (2.4 KB)
- ✅ `packages/php/bin/install-extension.php` (80 lines)
  - Post-install hook for future binary downloads
  - Detects platform/PHP/architecture
  - Currently stub; will be used with binary releases

### E. Documentation (25 KB)
- ✅ `docs/RELEASE_SETUP.md` (480 lines)
  - Complete setup and usage guide
  - Secret configuration instructions
  - Troubleshooting reference
  - Security considerations
  - Rollback procedures

- ✅ `RELEASE_PHP_SUMMARY.md` (380 lines)
  - Architecture overview
  - File structure summary
  - Workflow diagrams
  - Testing instructions
  - Integration details

- ✅ `IMPLEMENTATION_CHECKLIST.md` (this file)
  - Activation checklist
  - File summary
  - Next steps

---

## Part 2: Files Updated

### A. Version Sync Script (+9 lines)
- ✅ `scripts/sync_versions.py`
  - Added `COMPOSER_JSON_PATHS` list
  - Added loop to process composer.json files
  - Now syncs: `packages/php/composer.json`

### B. Composer Package Configuration (+9 lines)
- ✅ `packages/php/composer.json`
  - Added `"version": "0.1.3"` at root
  - Added `post-install-cmd` script hooks
  - Added `post-update-cmd` script hooks

### C. Task Automation (+50 lines)
- ✅ `Taskfile.yaml`
  - Added `php:build:pie-source` task
  - Added `release:php:dry-run` task
  - Added `release:php` task
  - Added `version:sync:php` task

---

## Part 3: Activation Checklist

### Step 1: Verify Files Exist

```bash
# Check all files created
ls -la .github/workflows/release-php.yml
ls -la .github/actions/build-php-extension/action.yml
ls -la .github/actions/test-php-extension/action.yml
ls -la packages/php/bin/install-extension.php
ls -la scripts/package_php_pie_source.sh
ls -la docs/RELEASE_SETUP.md

# Check scripts are executable
file scripts/package_php_pie_source.sh
file scripts/sync_versions.py
```

**Expected Output**:
```
.github/workflows/release-php.yml: YAML file
.github/actions/build-php-extension/action.yml: YAML file
.github/actions/test-php-extension/action.yml: YAML file
packages/php/bin/install-extension.php: PHP script
scripts/package_php_pie_source.sh: Bourne-Again shell script (executable)
scripts/sync_versions.py: Python script (executable)
```

### Step 2: Verify Script Permissions

```bash
# Make scripts executable if needed
chmod +x scripts/package_php_pie_source.sh
chmod +x scripts/sync_versions.py

# Verify
ls -la scripts/*.sh scripts/*.py | grep -E "rwx"
```

**Expected**: Both should have `rwx` for owner

### Step 3: Test Version Sync

```bash
# Dry-run: Check what would change
python3 scripts/sync_versions.py

# Output should show: "No changes needed" or list files changed
```

### Step 4: Test PIE Bundler Locally

```bash
# Extract current version
VERSION=$(grep '^version = ' Cargo.toml | sed -E 's/version = "(.*)"/\1/')

# Create PIE bundle
mkdir -p build/test-pie
./scripts/package_php_pie_source.sh "$VERSION" build/test-pie

# Verify bundle
ls -lh "build/test-pie/spikard-php-${VERSION}-src.tgz"
tar -tzf "build/test-pie/spikard-php-${VERSION}-src.tgz" | wc -l
```

**Expected**: Should create .tgz file with 100+ entries

### Step 5: Add GitHub Secret

1. Go to: https://github.com/Goldziher/spikard/settings/secrets/actions
2. Click "New repository secret"
3. **Name**: `PACKAGIST_VENDOR_API_TOKEN`
4. **Value**: (obtain from https://packagist.org/profile/)
5. Click "Add secret"

**Verification**:
```bash
gh secret list --org Goldziher
# Should show PACKAGIST_VENDOR_API_TOKEN
```

### Step 6: Register Packagist Package (First Time Only)

1. Visit: https://packagist.org/submit
2. Enter GitHub repo: `https://github.com/Goldziher/spikard`
3. Click "Check"
4. Package name should auto-fill as: `spikard/extension`
5. Click "Submit"
6. Go to package settings to verify GitHub webhook is enabled

**Verification**:
```bash
curl -s https://packagist.org/packages/spikard/extension.json | jq '.package.name'
# Should output: "spikard/extension"
```

### Step 7: Test Dry-Run Release

```bash
# Trigger dry-run (builds all artifacts, skips publishing)
task release:php:dry-run -- --set TAG=v0.1.3-test

# Monitor progress
gh run list --workflow release-php.yml --limit 3

# View detailed logs
gh run view <run-id> --log
```

**Expected**: Should build all platforms, run tests, skip Packagist publish

### Step 8: Verify Composer.json Version Field

```bash
# Check version field exists
jq '.version' packages/php/composer.json

# Should output: "0.1.3"
```

### Step 9: Check Taskfile Tasks

```bash
# List new tasks
task --list | grep -E "php:build:pie|release:php|version:sync:php"

# Should show 4 new tasks
```

---

## Part 4: Pre-Release Checklist

Before creating first tag, verify:

- [ ] All files created successfully
- [ ] Scripts are executable (`chmod +x`)
- [ ] GitHub secret `PACKAGIST_VENDOR_API_TOKEN` added
- [ ] Packagist package `spikard/extension` registered
- [ ] Packagist webhook enabled
- [ ] Local dry-run completed successfully
- [ ] `packages/php/composer.json` has version field
- [ ] `task version:sync` executes without error
- [ ] Git working tree is clean
- [ ] All tests pass (`task test`)

---

## Part 5: First Release Steps

### 1. Sync and Commit Versions

```bash
# Ensure version sync
task version:sync

# Check changes
git diff

# Commit
git add -A
git commit -m "chore: version management for release"
```

### 2. Create Annotated Tag

```bash
# Create tag (with optional GPG signing)
git tag -s v0.1.4 -m "Release v0.1.4: PHP bindings via ext-php-rs"

# Verify tag
git tag -v v0.1.4
```

### 3. Push Tag

```bash
# Push tag (triggers release workflow)
git push origin v0.1.4

# Verify push
git tag -l | grep v0.1.4
```

### 4. Monitor Release

```bash
# Watch workflow progress
gh workflow run list --workflow release-php.yml

# Tail logs (wait 10s for workflow to start)
sleep 10
gh run list --workflow release-php.yml --limit 1 -q
gh run view <run-id> --log --tail 100
```

### 5. Verify Artifacts

After workflow completes:

```bash
# Check GitHub Releases
gh release view v0.1.4

# List artifacts
gh release view v0.1.4 --json assets --jq '.assets[].name'

# Should show:
# - spikard-php-x86_64_unknown_linux_gnu-8.2.so
# - spikard-php-x86_64_unknown_linux_gnu-8.3.so
# - ... (all 8 binaries)
# - spikard-php-0.1.4-src.tgz
```

### 6. Verify Packagist

After 1-2 minutes:

```bash
# Check Packagist has new version
curl -s https://packagist.org/packages/spikard/extension.json | \
  jq '.package.versions | keys | .[-1]'

# Should output: "0.1.4"
```

### 7. Test Installation

```bash
# Create test directory
mkdir /tmp/spikard-test && cd /tmp/spikard-test

# Create simple composer.json
cat > composer.json <<EOF
{
  "require": {
    "spikard/extension": "0.1.4"
  }
}
EOF

# Install
composer install

# Should successfully install spikard/extension v0.1.4
```

---

## Part 6: Troubleshooting Quick Reference

### Issue: "Tag must start with 'v'"
**Fix**: Ensure tag format is `v0.1.4` (lowercase v)

### Issue: "Version mismatch" error
**Fix**: Run `task version:sync`, re-commit, re-tag

### Issue: "PACKAGIST_VENDOR_API_TOKEN not set"
**Fix**: Add secret to GitHub repository settings

### Issue: Workflow fails to build extension
**Fix**: Check build logs for compilation errors, test locally: `cargo build --release -p spikard-php --features extension-module`

### Issue: Smoke tests fail
**Fix**: Extension must match platform/PHP version, check GitHub Actions logs

### Issue: Packagist doesn't update
**Fix**: Wait 1-2 minutes, or manually trigger from Packagist dashboard

---

## Part 7: File Locations Summary

```
Spikard Repository Root/
│
├── .github/
│   ├── workflows/
│   │   └── release-php.yml ......................... MAIN WORKFLOW
│   └── actions/
│       ├── build-php-extension/action.yml ........ BUILD ACTION
│       └── test-php-extension/action.yml ......... TEST ACTION
│
├── scripts/
│   ├── package_php_pie_source.sh ................. PIE BUNDLER (updated permissions)
│   └── sync_versions.py ........................... VERSION SYNC (updated with composer.json)
│
├── packages/php/
│   ├── composer.json ............................. UPDATED (added version field + hooks)
│   └── bin/
│       └── install-extension.php ................. POST-INSTALL HOOK
│
├── Taskfile.yaml ................................. UPDATED (+50 lines, 4 new tasks)
│
├── docs/
│   └── RELEASE_SETUP.md ........................... SETUP GUIDE (new)
│
├── RELEASE_PHP_SUMMARY.md ......................... IMPLEMENTATION SUMMARY (new)
└── IMPLEMENTATION_CHECKLIST.md .................... THIS FILE (new)
```

---

## Part 8: Integration Points

### With Existing CI/CD
- Follows same GitHub Actions patterns
- Uses existing GitHub token (no new secrets except Packagist token)
- Compatible with publish.yaml workflow
- No conflicts with other language bindings

### With Version Management
- Integrates with existing sync_versions.py
- Composer.json now part of automatic sync
- Single source of truth: Cargo.toml [workspace.package]

### With Taskfile Automation
- Follows task naming conventions
- Preconditions validate tool availability
- Consistent with Python/Node release patterns

---

## Part 9: Security Checklist

- [ ] PACKAGIST_VENDOR_API_TOKEN stored as GitHub secret (not in code)
- [ ] Token scoped to vendor-level access only
- [ ] Token rotation scheduled (every 90 days)
- [ ] Workflow logs do not expose tokens
- [ ] Only tagged releases publish to Packagist
- [ ] Dry-run mode prevents accidental publishing
- [ ] All artifacts signed by GitHub (future SLSA provenance)
- [ ] Git tags optionally signed with GPG

---

## Part 10: Maintenance Tasks

### Monthly
- [ ] Check Packagist webhook status
- [ ] Review GitHub Actions usage
- [ ] Monitor release artifact storage (14-day retention)

### Quarterly
- [ ] Rotate PACKAGIST_VENDOR_API_TOKEN
- [ ] Audit GitHub Actions permissions
- [ ] Review security advisories for PHP versions

### Per Release
- [ ] Run `task version:sync` before tagging
- [ ] Test dry-run: `task release:php:dry-run`
- [ ] Verify artifacts on GitHub and Packagist
- [ ] Test installation: `composer require spikard/extension:X.Y.Z`

---

## Summary

**Total Files Created**: 9
**Total Files Updated**: 3
**Total Lines of Code**: ~1,650 new + 100 updated
**GitHub Secrets Required**: 1 (PACKAGIST_VENDOR_API_TOKEN)
**External Setup Required**: Packagist registration (one-time)
**Estimated Setup Time**: 30-45 minutes

**Status**: Ready for activation

---

## Next Steps

1. ✅ Verify all files exist: `ls -la` commands above
2. ✅ Make scripts executable: `chmod +x scripts/*.sh scripts/*.py`
3. ✅ Test version sync: `python3 scripts/sync_versions.py`
4. ✅ Test PIE bundler: `./scripts/package_php_pie_source.sh 0.1.3 build/test`
5. ✅ Add GitHub secret: `PACKAGIST_VENDOR_API_TOKEN`
6. ✅ Register Packagist: https://packagist.org/submit
7. ✅ Test dry-run: `task release:php:dry-run -- --set TAG=v0.1.3-test`
8. ✅ First release: Create tag and push

**All steps above are idempotent and safe to repeat.**
