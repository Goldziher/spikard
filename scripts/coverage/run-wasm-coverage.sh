#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}/packages/wasm"

echo "Installing WASM dependencies..."
pnpm install --frozen-lockfile

echo "Running WASM test suite with coverage..."
pnpm test:cov

echo "Coverage reports generated:"
echo "  - HTML: packages/wasm/coverage/index.html"
echo "  - LCOV: packages/wasm/coverage/lcov.info"
