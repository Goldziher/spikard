#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

for pyproject in tools/benchmark-harness/apps/*/pyproject.toml; do
	if [ ! -f "$pyproject" ]; then
		continue
	fi
	app_dir="$(dirname "$pyproject")"
	app_name="$(basename "$app_dir")"
	echo "Updating $app_name..."
	cd "$app_dir" && uv run uv-bump && uv sync --upgrade
	cd - >/dev/null
done
