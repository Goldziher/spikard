# Packagist Release Verification Report for spikard/spikard v0.2.1

## Executive Summary
**Status: FAILED - NOT PUBLISHED**
- GitHub release tag v0.2.1 exists and is properly signed
- Packagist only shows v0.2.0 (released 2025-11-30)
- v0.2.1 is NOT published to Packagist
- Critical version mismatches in workspace dependencies

---

## 1. Packagist Publication Status

### Current Published Version
- **Latest on Packagist**: v0.2.0
- **Downloads**: 0 (no adoption)
- **Publish Date**: 2025-11-30 15:02:40 UTC
- **v0.2.1 Status**: NOT PUBLISHED

### Version Check
```
curl https://repo.packagist.org/p2/spikard/spikard.json
```
Result: Only `dev-main` and `0.2.0` in versions array

---

## 2. Package Metadata Issues

### Autoload Path Problem
**Issue Found**: Incorrect autoload path in Packagist metadata

```json
{
  "autoload": {
    "psr-4": {
      "Spikard\\": "packages/php/src/"
    }
  }
}
```

**Expected (per composer.json)**:
```json
{
  "autoload": {
    "psr-4": {
      "Spikard\\": "src/"
    }
  }
}
```

**Impact**: When installed via Composer, the autoload path resolves from the package root:
- Packagist expects: `vendor/spikard/spikard/packages/php/src/`
- Correct path: `vendor/spikard/spikard/src/`

This will cause autoloading failures when consumers try to use the package.

---

## 3. PHP Version Requirements

### Declared Requirement
- **Requirement**: PHP ^8.2
- **Minimum Version**: PHP 8.2
- **Status**: CORRECT

### Verified in
- `/packages/php/composer.json`: `"php": "^8.2"`
- Packagist metadata: `"require": {"php": "^8.2"}`

---

## 4. Composer Installation Test

### Test Command
```bash
cd /tmp/composer-test
composer require spikard/spikard:^0.2.1
```

### Result: FAILED
```
Problem 1
  - Root composer.json requires spikard/spikard ^0.2.1
  - Found spikard/spikard[dev-main, 0.2.0] but it does not match constraint

Installation failed, deleting ./composer.json.
```

**Reason**: v0.2.1 not published to Packagist

---

## 5. Dependency Conflicts

### Root Cause Identified: Version Mismatch in Cargo.toml

**Workspace Version**: 0.2.1
```toml
[workspace.package]
version = "0.2.1"  # Root workspace version
```

**Workspace Dependency Versions**: 0.2.0 (INCORRECT)
```toml
[workspace.dependencies]
spikard = { version = "0.2.0", path = "crates/spikard" }          # MISMATCH
spikard-core = { version = "0.2.0", path = "crates/spikard-core" } # MISMATCH
```

### Affected Crates
All Rust crates that depend on spikard/spikard-core will have inconsistent versions:
- crates/spikard-http
- crates/spikard-cli
- crates/spikard-py
- crates/spikard-node
- crates/spikard-rb
- crates/spikard-php
- crates/spikard-wasm
- tools/test-generator

---

## 6. GitHub Release Status

### Release Tag Information
- **Tag**: v0.2.1
- **Commit Hash**: 2b77e8833d22f610cfbde9a43ba6fa5a381f5dc7
- **Date**: 2025-11-30 20:30:23 UTC
- **GPG Signature**: Valid (signed by Goldziher)
- **Release Notes**: "Bug fixes for Python, Ruby, and Node.js package distribution"

### GitHub Tag Verification
```json
{
  "name": "v0.2.1",
  "commit": {
    "sha": "2b77e8833d22f610cfbde9a43ba6fa5a381f5dc7"
  }
}
```
Status: VALID AND SIGNED

---

## 7. Package Structure Verification

### PHP Package Layout
```
packages/php/
├── src/
│   ├── App.php
│   ├── Attributes/
│   ├── Config/
│   ├── DI/
│   ├── Handlers/
│   ├── Http/
│   └── ...
├── tests/
├── composer.json  (v0.2.1 ✓)
├── phpunit.xml
├── phpstan.neon
└── composer.lock
```

Status: Structure looks correct

### Composer Configuration
```json
{
  "name": "spikard/spikard",
  "version": "0.2.1",
  "type": "library",
  "require": {
    "php": "^8.2"
  },
  "autoload": {
    "psr-4": {
      "Spikard\\": "src/"
    }
  }
}
```

Status: CORRECT (local file)

---

## 8. Critical Issues for v0.2.2

### Issue #1: Cargo.toml Version Mismatch (BLOCKING)
**Severity**: CRITICAL
**File**: `/Cargo.toml`
**Problem**: Workspace dependencies reference v0.2.0 but workspace is v0.2.1

**Required Fix**:
```toml
# BEFORE (lines 32-33):
spikard = { version = "0.2.0", path = "crates/spikard" }
spikard-core = { version = "0.2.0", path = "crates/spikard-core" }

# AFTER:
spikard = { version = "0.2.1", path = "crates/spikard" }
spikard-core = { version = "0.2.1", path = "crates/spikard-core" }
```

### Issue #2: Autoload Path in Packagist (DISTRIBUTION)
**Severity**: HIGH
**Problem**: Packagist metadata shows `packages/php/src/` but should be `src/`
**Root Cause**: Likely in publish workflow or Packagist webhook processing
**Impact**: Package consumers cannot autoload classes correctly

### Issue #3: v0.2.1 Not Published (BLOCKING)
**Severity**: CRITICAL
**Status**: GitHub release created but not synced to Packagist
**Check Workflows**:
- `.github/workflows/publish.yaml` may not have been triggered for PHP
- Packagist webhook may not be receiving updates

---

## 9. Verification Checklist

| Item | Status | Notes |
|------|--------|-------|
| GitHub Release Tag Created | ✓ PASS | v0.2.1 exists and is signed |
| Package Metadata (local) | ✓ PASS | composer.json shows v0.2.1 |
| PHP Version Requirement | ✓ PASS | PHP ^8.2 declared |
| Package Structure | ✓ PASS | src/ directory exists with classes |
| Autoload Path (local) | ✓ PASS | PSR-4 configured correctly |
| Autoload Path (Packagist) | ✗ FAIL | Shows `packages/php/src/` |
| Packagist Published | ✗ FAIL | Only v0.2.0 available |
| Composer Installation | ✗ FAIL | v0.2.1 not found |
| Cargo.toml Versions | ✗ FAIL | Workspace vs dependency mismatch |

---

## 10. Recommended Actions for v0.2.2

### Immediate (Before Release)
1. **Fix Cargo.toml** - Update workspace dependency versions to 0.2.1
2. **Verify Packagist Sync** - Check if v0.2.1 appears after fix
3. **Test Composer Install** - Confirm `composer require spikard/spikard:^0.2.1` works
4. **Check Autoload Path** - Verify Packagist shows correct `src/` path

### Pre-Release Checklist
```bash
# Verify local package version
cat packages/php/composer.json | grep '"version"'

# Verify Cargo.toml versions match
grep "version" Cargo.toml | head -5

# Test composer installation
cd /tmp && mkdir test-install && cd test-install
composer require spikard/spikard:^0.2.2

# Verify autoloading works
php -r "require 'vendor/autoload.php'; echo class_exists('Spikard\\App') ? 'OK' : 'FAIL';"
```

### Publish Workflow
1. Commit all version fixes
2. Tag release: `git tag v0.2.2`
3. Push tag: `git push origin v0.2.2`
4. Create GitHub release from tag
5. Monitor `.github/workflows/publish.yaml` execution
6. Verify Packagist updates within 5 minutes
7. Test composer installation: `composer create-project spikard/spikard:^0.2.2 test-app`

---

## 11. Summary

| Metric | Result |
|--------|--------|
| GitHub Release | ✓ EXISTS (SIGNED) |
| Packagist Published | ✗ NO (v0.2.0 ONLY) |
| Composer Installable | ✗ NO |
| PHP Version Support | ✓ YES (8.2+) |
| Autoload (Local) | ✓ CORRECT |
| Autoload (Packagist) | ✗ WRONG PATH |
| Version Consistency | ✗ MISMATCHES |

**Conclusion**: v0.2.1 release exists on GitHub but failed to publish to Packagist. Critical version mismatches in Cargo.toml must be fixed before v0.2.2.
