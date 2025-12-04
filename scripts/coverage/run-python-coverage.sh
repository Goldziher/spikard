#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"
COVERAGE_RCFILE="${REPO_ROOT}/.coveragerc" uv run pytest "${REPO_ROOT}/packages/python/tests" \
	--cov=spikard \
	--cov-report=html:"${REPO_ROOT}/packages/python/htmlcov" \
	--cov-report=term \
	--cov-report=lcov:"${REPO_ROOT}/packages/python/coverage.lcov" \
	--cov-fail-under=80
