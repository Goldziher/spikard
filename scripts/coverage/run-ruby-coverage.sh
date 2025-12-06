#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}/packages/ruby"

echo "Running Ruby test suite with coverage..."
bundle exec rspec

echo ""
echo "Coverage report generated at coverage/index.html"
echo "LCOV report at coverage/lcov.info"
