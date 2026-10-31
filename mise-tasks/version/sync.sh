#!/usr/bin/env bash
#MISE description="Synchronize version across all package manifests and regenerate everything"
#MISE alias="versions:sync"
set -euo pipefail

alef=(cargo run -p spikard-alef --release --)
"${alef[@]}" sync-versions
# ~keep `alef all --clean` regenerates every Alef-managed output (readme, docs, bindings,
# scaffold, stubs, e2e, test-apps) in the correct internal order. Do NOT re-expand this into
# individual sub-commands: hand-ordering `docs` before `generate --clean` lets the clean wipe
# docs/reference/ after it is written. `all` runs them in dependency order.
"${alef[@]}" all --clean
# ~keep The cbindgen header (crates/spikard-ffi/include/spikard.h) is emitted by spikard-ffi's
# build.rs, not by `alef all`; build it so the header stays fresh.
cargo build -p spikard-ffi
# ~keep dart FRB codegen + fix_handler_executor_calls() only run during cargo build, not during
# `alef all` (which emits Rust source only).
cargo build --manifest-path packages/dart/rust/Cargo.toml 2>&1 | grep -v "^warning:" || true
version=$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
for manifest in packages/dart/rust packages/ruby/ext/spikard_rb/native test_apps/rust; do
  cargo update --manifest-path "$manifest/Cargo.toml" -p spikard --precise "$version" >/dev/null 2>&1 || true
done
