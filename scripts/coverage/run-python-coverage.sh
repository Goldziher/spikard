#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}/packages/python"
uv run pytest tests/ \
	--cov=spikard \
	--cov-report=html \
	--cov-report=term \
	--cov-report=lcov:coverage.lcov \
	--cov-fail-under=80
