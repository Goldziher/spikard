#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

missing=0
shopt -s nullglob
for dir in "${REPO_ROOT}"/crates/spikard-node/npm/*/; do
	nodes=("$dir"/*.node)
	if [ ${#nodes[@]} -eq 0 ]; then
		echo "::error::Missing native binary in $dir"
		missing=1
	fi
done

if [ "${missing}" -ne 0 ]; then
	exit 1
fi
