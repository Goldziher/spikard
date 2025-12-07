#!/usr/bin/env bash
set -euo pipefail

# Run Python tests with code coverage measurement
#
# This script synchronizes dependencies, builds the extension module in development mode,
# and runs pytest with coverage reporting. It generates both HTML and LCOV format reports.
#
# Coverage Reports:
#   - HTML: htmlcov/
#   - LCOV: coverage.lcov (for CI upload)
#
# Usage:
#   ./scripts/ci/python/run-coverage.sh

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Change to Python package directory
cd "${REPO_ROOT}/packages/python"

echo "Synchronizing dependencies..."
uv sync --all-extras

echo "Building extension module (development mode)..."
uv run maturin develop --release --features extension-module

echo "Running tests with coverage..."
uv run pytest \
	--cov=_spikard \
	--cov-report=html:htmlcov \
	--cov-report=lcov:coverage.lcov \
	-v

echo "Coverage reports generated:"
echo "  - HTML: $(pwd)/htmlcov/index.html"
echo "  - LCOV: $(pwd)/coverage.lcov"
