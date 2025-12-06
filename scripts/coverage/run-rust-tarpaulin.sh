#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "${REPO_ROOT}"

export DYLD_FALLBACK_LIBRARY_PATH="${DYLD_FALLBACK_LIBRARY_PATH:-/usr/local/lib}"

mkdir -p target/tarpaulin

cargo tarpaulin \
	--workspace \
	--exclude spikard-wasm \
	--exclude spikard-php \
	--timeout 300 \
	--out Stdout \
	--out Html \
	--out Lcov
