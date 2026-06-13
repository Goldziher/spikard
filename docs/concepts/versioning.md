# Versioning and stability

Spikard follows semantic versioning 2.0 with additional guarantees on the C ABI and per-language wire formats.

## Semantic versioning

Version numbers follow `MAJOR.MINOR.PATCH`:

- **MAJOR** — incompatible API or ABI changes, breaking runtime behavior
- **MINOR** — backward-compatible feature additions, deprecation warnings
- **PATCH** — bug fixes, internal optimization, no API changes

## Stability guarantees

### Rust core

Rust API follows Cargo semver conventions. All public items are documented and tested. Adding a public item is a minor bump; removing or changing a signature is a major bump.

### Language bindings

Each binding exports a stable public API documented in the per-language reference. Removing a method or changing a signature is a major bump. Adding a method or optional parameter is a minor bump.

When the Rust core adds a feature, the binding may expose it in a minor version if the new functionality is additive (e.g., a new exception type, a new optional config field). If the Rust core change is breaking, all bindings bump to the same major version.

### C ABI

The C FFI layer (spikard-ffi) follows a stricter layout-freeze policy:

- **MAJOR.MINOR boundaries freeze struct layouts.** If you build against spikard-ffi 1.2, the C struct sizes, field offsets, and function signatures remain unchanged for the lifetime of 1.2.x releases.
- **PATCH releases** add new functions, never remove or change struct layouts.
- A struct layout change requires a MINOR version bump at minimum (e.g., 1.2 → 1.3).
- Function pointer signatures never change; new callbacks are added as new function pointers, not mutations of existing ones.

This means C consumers, JNI wrappers, and other direct-FFI users can safely pin to `~> 1.2` in their dependency declarations without rebuilding.

### Pre-1.0 caveat

Versions < 1.0 (including v0.16) are development versions. **MINOR bumps may include breaking changes.** Pin dependencies exactly during pre-1.0: `spikard == 0.16.0` in Rust, `pip install spikard==0.16.0` in Python, `@spikard/node@0.16.0` in npm. When 1.0 is released, semantic versioning becomes strict.

## Deprecation timeline

When an API is scheduled for removal:

1. **Minor N** — API marked deprecated in docs and code comments. Runtime warnings or linter hints where the language supports them (Python `DeprecationWarning`, Rust lint, TypeScript `@deprecated`). The API still works.
2. **Minor N+1** — Deprecation remains. Migration guide published. No code changes required.
3. **Major N+1** — API removed.

Example: If `app.old_config()` is deprecated in v0.16, it will continue to work through v0.17. It will be removed in v1.0 (or v0.18 if a major bump comes first).

Exceptions require a note in the migration guide when:

- The replacement has significantly different syntax or semantics.
- Removal affects a majority of applications (e.g., core routing syntax).

## Release cadence

Spikard releases new MINOR versions every 4-6 weeks with new features and binding improvements. PATCH versions ship as needed for critical bugs. There is no fixed end-of-life date for older MAJOR versions during pre-1.0; the team will announce any deprecation plan when moving toward 1.0.

## Version tracking

- **Rust core:** crate version in `crates/spikard/Cargo.toml`
- **Python:** package version in `packages/python/pyproject.toml`
- **Node/TS:** package version in `packages/typescript/package.json`
- **Ruby:** gem version in `packages/ruby/spikard.gemspec`
- **PHP:** package version in `packages/php/composer.json`
- **Elixir:** package version in `packages/elixir/mix.exs`
- **Go:** module version in `packages/go/go.mod` and git tag `go/vX.Y.Z`
- **Java:** artifact version in `packages/java/pom.xml`
- **C#:** package version in `packages/csharp/Directory.Build.props`
- **Kotlin / Android:** artifact version in `packages/kotlin/build.gradle.kts`
- **Dart:** package version in `packages/dart/pubspec.yaml`
- **Swift:** package version in `packages/swift/Package.swift`
- **Zig:** package version in `packages/zig/build.zig.zon`
- **C FFI:** library version in `crates/spikard-ffi/Cargo.toml`

All per-language versions are synced to the Rust core version at release. Patch releases for critical fixes may increment the per-language version independently (e.g., Python 0.16.1 published before Node 0.16.1), but the next MINOR or MAJOR bump realigns them.
