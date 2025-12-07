#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Create a temporary directory for the build output
TEMP_WHEEL_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_WHEEL_DIR}"' EXIT

# Build to the temporary directory
cd "${REPO_ROOT}/packages/python"
uv build --wheel --out-dir "${TEMP_WHEEL_DIR}"

# Copy the built wheel to the final location
mkdir -p "${REPO_ROOT}/target/wheels"
cp "${TEMP_WHEEL_DIR}"/*.whl "${REPO_ROOT}/target/wheels/"
