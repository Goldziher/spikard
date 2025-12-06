#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"
pnpm exec napi build --platform --dts index.d.ts
mkdir -p ../../typescript-defs
cp index.d.ts ../../typescript-defs/
cp index.js ../../typescript-defs/ || true
