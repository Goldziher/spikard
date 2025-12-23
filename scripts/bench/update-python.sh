#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

for app in fastapi-uvicorn-validation fastapi-uvicorn-raw fastapi-granian-validation fastapi-granian-raw robyn-validation robyn-raw spikard-python-validation spikard-python-raw; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && uv run uv-bump && uv sync --upgrade
	cd - >/dev/null
done
