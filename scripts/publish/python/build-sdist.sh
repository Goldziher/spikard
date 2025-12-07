#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

mkdir -p "${REPO_ROOT}/target/wheels"
cd "${REPO_ROOT}/packages/python" && uv tool run maturin sdist --out "${REPO_ROOT}/target/wheels"
