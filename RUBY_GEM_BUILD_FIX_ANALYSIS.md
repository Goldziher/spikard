# Ruby Gem Build Fix Analysis - Spikard CI Workflow

## Executive Summary

The proposed fix to add `bundle exec` before `rake vendor:sync` calls in the publish.yaml workflow is **CORRECT and SAFE to apply**. This fix addresses a critical bundler dependency resolution issue that causes Ruby gem builds to fail.

---

## Issue Analysis

### Problem Identified

**Location:** `.github/workflows/publish.yaml` lines 831 and 839

Two rake invocations execute without `bundle exec`:
- **Line 831 (Unix):** `run: rake vendor:sync`
- **Line 839 (Windows):** `ridk exec bash -lc "cd $gemdir && rake vendor:sync"`

Meanwhile, subsequent rake calls properly use `bundle exec`:
- **Line 847 (Unix):** `bundle exec rake clean`
- **Line 848 (Unix):** `bundle exec rake build`
- **Line 856 (Windows):** `bundle exec rake clean && bundle exec rake build`

### Root Cause

When `rake` is invoked without `bundle exec`, it runs the system-installed rake (if any) rather than the bundled gem specified in the `Gemfile`. This causes bundler to lose track of gem dependencies, leading to missing gems at runtime.

Key evidence:
- `Gemfile` specifies `gem 'rake', '~> 13.0'` (line 5)
- `Gemfile` specifies `gem 'rake-compiler', '~> 1.2'` (line 6)
- The setup-ruby action includes `bundler-cache: true` (line 798), which installs gems into a managed cache
- Calling `rake` directly bypasses this cache

---

## Bundler Best Practices Analysis

### 1. Bundler-Cache Configuration

**Status: PROPERLY CONFIGURED**

```yaml
- name: Set up Ruby
  uses: ruby/setup-ruby@v1
  with:
    ruby-version: "3.3"
    bundler-cache: true  # ✓ Gems are cached
    working-directory: packages/ruby
```

The `ruby/setup-ruby@v1` action with `bundler-cache: true`:
- Automatically runs `bundle install` based on the Gemfile.lock
- Caches gems in GitHub Actions cache
- Isolates gems from the system ruby environment

### 2. Bundle Install Steps

**Status: CORRECT EXECUTION**

Both Unix and Windows install dependencies properly:

```yaml
# Unix (line 817)
run: bundle install --jobs 4 --retry 3

# Windows (line 826)
ridk exec bash -lc "cd $gemdir && ... && bundle install --jobs 4 --retry 3"
```

These calls ensure all gems in `Gemfile` and `Gemfile.lock` are installed.

### 3. Rake Dependency Analysis

**Rakefile dependency chain (packages/ruby/Rakefile):**

```ruby
require 'bundler/gem_tasks'  # Requires bundler to be available
require 'fileutils'
require 'rbconfig'

namespace :ext do
  task build: [:clean, 'vendor:sync'] do
    # Line 21: sh({ 'CARGO_PROFILE' => profile }, 'bundle exec ruby extconf.rb')
    # Line 22: sh({ 'CARGO_PROFILE' => profile }, 'make')
  end
end

namespace :vendor do
  desc 'Copy workspace crates into vendor directory for gem distribution'
  task :sync do
    # Pure Ruby code, no external dependencies
    # But the namespace requires rake itself to execute
  end
end
```

The `require 'bundler/gem_tasks'` at the top of Rakefile is crucial—it requires bundler to be properly loaded, which only happens when invoking rake through `bundle exec`.

---

## Fix Correctness Verification

### Proposed Changes

1. **Line 831:** Change `run: rake vendor:sync` to `run: bundle exec rake vendor:sync`
2. **Line 839:** Change `ridk exec bash -lc "cd $gemdir && rake vendor:sync"` to `ridk exec bash -lc "cd $gemdir && bundle exec rake vendor:sync"`

### Why This Fix Is Safe

1. **Consistency with existing code patterns:**
   - Lines 847-848: Already use `bundle exec rake`
   - Line 856: Already uses `bundle exec rake` on Windows

2. **No logic changes:**
   - `bundle exec` is a wrapper that doesn't alter Rakefile behavior
   - It ensures the bundled `rake` gem is used (same version as in Gemfile.lock)
   - All task definitions and actions remain identical

3. **Environment compatibility:**
   - Unix: Works with standard bash/sh
   - Windows: Works with ridk exec (Ruby DevKit shim) which handles bundle exec properly

4. **Dependency satisfaction:**
   - `bundle install` (line 817 & 826) ensures rake is available
   - `bundle exec` guarantees access to installed gems
   - No circular dependencies or conflicts

### What Won't Break

1. **Build process:** The vendor:sync task contains only pure Ruby file operations
2. **Cross-platform compatibility:** Both Unix and Windows use bundle exec consistently
3. **Cache behavior:** bundler-cache remains effective; no additional gem installations needed
4. **Subsequent build steps:** Lines 847-848 already expect bundled rake

---

## Additional Observations

### Line 839 (Windows) Consideration

The Windows implementation uses `ridk exec bash -lc` (Ruby DevKit shim). This is necessary because:
- Rust toolchain configuration requires GNU bash environment on Windows
- Bundle exec works properly within this bash environment
- The same pattern is already used successfully for bundle install (line 826) and bundle exec rake (line 856)

### No Other Uncovered Rake Calls

Grep analysis confirms only lines 831 and 839 lack `bundle exec`:

```
831:        run: rake vendor:sync
839:          ridk exec bash -lc "cd $gemdir && rake vendor:sync"
847:          bundle exec rake clean        ✓
848:          bundle exec rake build        ✓
856:          ridk exec bash -lc "cd ... && bundle exec rake clean && bundle exec rake build"  ✓
```

---

## Testing Recommendations

After applying the fix, verify:

1. **Local testing:**
   ```bash
   cd packages/ruby
   bundle install
   bundle exec rake vendor:sync  # Should succeed
   ```

2. **CI validation:**
   - Run the publish workflow with `dry_run: true` to stage artifacts
   - Confirm all three platforms (Linux, macOS, Windows) build gems successfully
   - Verify gem contents include vendored crates

3. **Smoke test:**
   - Install the built gem: `gem install spikard-*.gem`
   - Verify native extension loads: `ruby -rspikard -e "puts Spikard::VERSION"`

---

## Conclusion

**RECOMMENDATION: APPLY THE FIX**

The proposed changes are:
- **Correct:** Align with Ruby/Bundler best practices
- **Safe:** No behavioral changes, only environment isolation
- **Necessary:** Prevents bundler from losing track of installed gems
- **Consistent:** Matches existing patterns in the same workflow

The fix should be applied to both lines 831 (Unix) and 839 (Windows) simultaneously to maintain platform parity.

---

## Reference Files Examined

- `/Users/naamanhirschfeld/workspace/spikard/.github/workflows/publish.yaml` (lines 754-863, Ruby job)
- `/Users/naamanhirschfeld/workspace/spikard/packages/ruby/Rakefile` (task definitions)
- `/Users/naamanhirschfeld/workspace/spikard/packages/ruby/Gemfile` (gem dependencies)
