# CI Failures Report - 2025-12-26

## Overview
Two critical failures identified in GitHub Actions workflows:
1. **Comparative Benchmarks** - Path expansion issue
2. **Publish Release v0.6.2** - Ruby gem corruption

---

## 1. Comparative Benchmarks Failure

**Run ID:** 20529935965
**Status:** Failed
**Workflow:** Comparative Benchmarks
**Date:** 2025-12-26T21:46:08Z

### Error
```
chmod: cannot access '~/.cargo/bin/benchmark-harness': No such file or directory
chmod: cannot access '~/.cargo/bin/oha': No such file or directory
```

### Root Cause
Tilde expansion failure in `.github/actions/setup-benchmark-tools/action.yaml` (line 31).

**Problem:**
- Input default: `default: "~/.cargo/bin"`
- The `actions/download-artifact@v7` does NOT expand `~` when used as `path:` parameter
- The `chmod` command tries to access `~/.cargo/bin/benchmark-harness` which doesn't exist
- Files were downloaded to a different location (literal `~/.cargo/bin` or current directory)

**Affected Files:**
- `.github/actions/setup-benchmark-tools/action.yaml` (lines 9-12, 25, 31, 40)
- `.github/workflows/comparative-benchmarks.yaml` (line 85)

### Solution
Replace tilde with `$HOME` or use absolute paths in GitHub Actions:

```yaml
# Option 1: Use $HOME explicitly
- name: Make benchmark tools executable
  shell: bash
  run: |
    install_dir="${{ inputs.install-dir }}"
    install_dir="${install_dir/#\~/$HOME}"
    chmod +x "$install_dir/benchmark-harness" "$install_dir/oha"

# Option 2: Change default to absolute path
inputs:
  install-dir:
    default: "$HOME/.cargo/bin"
```

---

## 2. Ruby Gem Publishing Failure

**Run ID:** 20529927479
**Status:** Failed
**Workflow:** Publish Release v0.6.2
**Date:** 2025-12-26T21:45:22Z

### Error
```
Exception while verifying ./spikard-0.6.2.gem
/opt/hostedtoolcache/Ruby/3.4.8/x64/lib/ruby/3.4.0/rubygems/package.rb:714:in 'Gem::Package#verify_gz':
invalid compressed data -- crc error in data.tar.gz (Gem::Package::FormatError)
```

### Root Cause
Gem archive corruption due to **vendored dependency configuration mismatch** between main crates and vendored crates.

**Chain of Events:**
1. `scripts/ci/ruby/vendor-crates.sh` copies crates to `packages/ruby/vendor/crates/`
2. Vendoring script has incomplete dependency handling
3. Main `crates/spikard-core/Cargo.toml` had `thiserror` as optional in `di` feature (fixed in 9eab2837)
4. Vendored version not synchronized
5. Native extension build with `--locked` flag fails/produces incomplete output
6. `data.tar.gz` packed with corrupted/incomplete `.so`/`.bundle` binary
7. CRC check fails during gem verification

**Recent Fix Attempts:**
- **Commit 9eab2837**: "fix(ruby): remove thiserror from di feature - it's not optional"
  - Fixed main Cargo.toml
  - May not have updated vendored version

- **Commit 6d178e70**: "fix(ruby): comprehensive vendoring script"
  - Improved dependency pattern handling
  - Still may miss edge cases

**Affected Files:**
- `.github/workflows/publish.yaml` (lines 681-765)
- `scripts/publish/ruby/build-gem-unix.sh`
- `scripts/publish/ruby/build-gem-windows.ps1`
- `scripts/ci/ruby/vendor-crates.sh` - Incomplete dependency mapping
- `packages/ruby/vendor/crates/spikard-core/Cargo.toml` - Out of sync
- `packages/ruby/ext/spikard_rb/extconf.rb` - Uses `--locked` flag
- `scripts/publish/ruby/publish-gems.sh` (verification at lines 35-36)

### Build Process
```bash
# From build-gem-unix.sh
cd packages/ruby
bundle exec rake clean
bundle exec rake build

# Rakefile compiles with:
# - cargo build --locked (requires exact Cargo.lock match)
# - If vendored Cargo.toml differs from main, compilation fails
# - Corrupted binary gets packed into gem
```

### Solution
1. **Immediate:** Re-run vendoring script after Cargo.toml changes
2. **Short-term:** Add CI validation step:
   ```bash
   cd packages/ruby/vendor
   cargo check --locked
   ```
3. **Long-term:**
   - Automate vendor synchronization
   - Add pre-publish gem integrity checks
   - Validate gem extraction before upload

---

## Benchmark Data

**Note:** The Comparative Benchmarks workflow failed before executing actual benchmarks. No performance data available for this run.

**Previous Successful Run:** CodeQL workflow (20530958237) completed successfully.

---

## Next Steps

### Critical (Blocking Release)
1. Fix tilde expansion in `setup-benchmark-tools/action.yaml`
2. Verify vendored crates synchronization for Ruby gems
3. Re-run release workflow for v0.6.2

### Important (Prevent Recurrence)
1. Add CI step to validate vendored Cargo.toml files
2. Add pre-publish gem integrity checks
3. Document vendoring requirements in contributing guide
4. Consider automating vendor updates via git hooks or CI

### Monitoring
- Watch for `thiserror` dependency across all binding crates
- Monitor workspace reference changes in main Cargo.toml files
- Track vendoring script coverage of new dependencies
