#!/usr/bin/env bash
#MISE description="Regenerate all Alef-managed outputs without formatting or building"
set -euo pipefail

cargo run -p spikard-alef --release -- all --clean
# ~keep Invoke cargo build for spikard-dart to trigger FRB codegen and the
# fix_handler_executor_calls() rewrite. alef's generate phase only regenerates Rust source
# files; FRB codegen and the rewrite only run during cargo build. Best-effort, as before.
if command -v cargo >/dev/null 2>&1; then
  cargo build --manifest-path packages/dart/rust/Cargo.toml 2>&1 | grep -v "^warning:" || true
else
  echo "cargo not available; skipping spikard-dart build"
fi
