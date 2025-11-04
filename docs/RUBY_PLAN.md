# Ruby Binding Plan (WIP)

Reference implementation: `~/workspace/kreuzberg` (has Ruby bindings and function calling).

## Research & Design
- [x] Audit kreuzberg Ruby bindings to confirm toolchain (Rust crates, Ruby gem layout, magnus/helix usage, build scripts).
- [x] Sketch how Spikard’s HTTP engine maps to a thin Ruby wrapper (identify which Rust APIs need to be exposed).
- [x] Decide on Ruby package name (`spikard` vs `spikard-ruby`) and namespace conventions.

## Rust Side (`crates/spikard-ruby`)
- [x] Bootstrap a new crate mirroring kreuzberg’s Rust binding pattern (FFI surface, init hooks, error translation). *(currently exposes version only – expand with HTTP APIs)*
- [ ] Re-export shared logic from the core engine; keep wrapper thin.
- [ ] Add build scripts (if needed) for producing a native Ruby extension (`.bundle` / `.so`).

## Ruby Package (`packages/ruby`)
- [x] Scaffold a Ruby gem (gemspec, `lib/`, extension loader) similar to kreuzberg.
- [ ] Wire up native extension loading, provide ergonomic Ruby API around the Rust functions.
- [ ] Add basic usage docs and examples.

## Test Generator Integration
- [ ] Extend `tools/test-generator` to emit Ruby apps/tests into `e2e/ruby`, mirroring the Rust/Python/Node flow.
- [ ] Ensure generated Ruby code formats cleanly (hook Rubocop/formatter).
- [ ] Update fixture-driven pipelines so Ruby tests assert the same behaviour (validation errors, empty bodies, etc.).

## Tooling & Linting
- [x] Add Rubocop configuration and wire it into `Taskfile.yaml` / lint pipelines (including generated files).
- [ ] Ensure Ruby formatting step runs automatically after generation (possibly via `bundle exec rubocop -A`).

## E2E / CI
- [x] Create `e2e/ruby` project (app factories + test suite) and make sure it runs under Minitest or RSpec (decide which). *(initial version asserts version only)*
- [x] Add a `task test:e2e:ruby` (and integrate with `task test:e2e` aggregate).
- [ ] Update CI to install Ruby toolchain, run linters, and execute the Ruby e2e suite.

## Follow-up
- [ ] Document Ruby binding usage in `README.md`.
- [ ] Consider packaging/publishing pipeline for the Ruby gem (build, versioning).
- [ ] Evaluate cross-platform build story (macOS/Linux, Ruby versions) before release.
