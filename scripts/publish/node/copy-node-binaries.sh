#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

shopt -s nullglob
for node_file in "${REPO_ROOT}"/crates/spikard-node/*.node; do
	filename=$(basename "${node_file}")
	platform="${filename#spikard-node.}"
	platform="${platform%.node}"
	dest_dir="${REPO_ROOT}/crates/spikard-node/npm/${platform}"
	if [ -d "${dest_dir}" ]; then
		echo "Copying ${filename} to ${dest_dir}/"
		cp "${node_file}" "${dest_dir}/"
	else
		echo "::warning::Platform directory ${dest_dir} not found for ${filename}"
	fi
done
