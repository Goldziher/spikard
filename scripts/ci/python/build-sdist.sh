#!/usr/bin/env bash
set -euo pipefail

# Build Python source distribution (sdist) using maturin
#
# This script removes any existing compiled artifacts and then builds a source
# distribution (tarball) using maturin. The resulting sdist can be used to build
# wheels on any platform.
#
# Usage:
#   ./scripts/ci/python/build-sdist.sh

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Change to Python package directory
cd "${REPO_ROOT}/packages/python"

# Clean up any existing compiled artifacts
echo "Cleaning up existing artifacts..."
rm -f _spikard/_spikard.*

# Build source distribution with maturin
echo "Building source distribution..."
uv tool run maturin sdist

echo "Source distribution built successfully at ${REPO_ROOT}/target/wheels/"
