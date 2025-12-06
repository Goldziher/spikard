#!/usr/bin/env bash
set -euo pipefail

# Runs the full Rust workspace test suite. The PyO3 extension-module feature
# stays disabled because it conflicts with test binaries that embed Python.
cargo test --workspace "${@}"
