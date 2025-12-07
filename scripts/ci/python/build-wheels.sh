#!/usr/bin/env bash
set -euo pipefail

# Build Python wheels for the current platform
#
# This script removes any existing compiled artifacts and then builds wheels
# using maturin with the extension-module feature enabled.
#
# Environment Variables:
#   PYTHON_VERSION: Python version to build for (e.g., "3.11", "3.14")
#                   (optional, maturin will auto-detect if not set)
#   MATURIN_ARGS: Additional arguments to pass to maturin (optional)
#
# Usage:
#   ./scripts/ci/python/build-wheels.sh [PYTHON_VERSION]
#   ./scripts/ci/python/build-wheels.sh "3.14"

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Change to Python package directory
cd "${REPO_ROOT}/packages/python"

# Get Python version from argument or environment
PYTHON_VERSION="${1:-${PYTHON_VERSION:-}}"

# Clean up any existing compiled artifacts
echo "Cleaning up existing artifacts..."
rm -f _spikard/_spikard.*

# Build wheels with maturin
echo "Building Python wheels..."
if [ -n "$PYTHON_VERSION" ]; then
	echo "Building for Python $PYTHON_VERSION..."
	uv tool run maturin build --release --features extension-module -i "$PYTHON_VERSION"
else
	echo "Building with auto-detected Python version..."
	uv tool run maturin build --release --features extension-module
fi

echo "Wheels built successfully at ${REPO_ROOT}/target/wheels/"
