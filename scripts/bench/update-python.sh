#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

for app in fastapi-uvicorn-dto fastapi-uvicorn-raw fastapi-granian-dto fastapi-granian-raw robyn-dto robyn-raw; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && uv run uv-bump && uv sync --upgrade
	cd - >/dev/null
done
