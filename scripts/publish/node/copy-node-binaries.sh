#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

# Map Rust target names to npm platform names
map_rust_to_npm_platform() {
	local rust_target="$1"
	case "${rust_target}" in
	x86_64-apple-darwin) echo "darwin-x64" ;;
	aarch64-apple-darwin) echo "darwin-arm64" ;;
	x86_64-unknown-linux-gnu) echo "linux-x64-gnu" ;;
	x86_64-pc-windows-msvc) echo "win32-x64-msvc" ;;
	# Also handle short form names that might come from napi build
	darwin-x64) echo "darwin-x64" ;;
	darwin-arm64) echo "darwin-arm64" ;;
	linux-x64-gnu) echo "linux-x64-gnu" ;;
	win32-x64-msvc) echo "win32-x64-msvc" ;;
	*) echo "" ;;
	esac
}

shopt -s nullglob
for node_file in "${REPO_ROOT}"/crates/spikard-node/*.node; do
	filename=$(basename "${node_file}")
	# Extract the platform name from the filename
	# Handle both formats: spikard-node.aarch64-apple-darwin.node and spikard-node.darwin-arm64.node
	platform="${filename#spikard-node.}"
	platform="${platform%.node}"

	# Map the platform name if needed
	npm_platform=$(map_rust_to_npm_platform "${platform}")

	if [ -z "${npm_platform}" ]; then
		echo "::warning::Could not map platform '${platform}' from ${filename}"
		continue
	fi

	dest_dir="${REPO_ROOT}/crates/spikard-node/npm/${npm_platform}"
	if [ -d "${dest_dir}" ]; then
		echo "Copying ${filename} to ${dest_dir}/"
		cp "${node_file}" "${dest_dir}/"
	else
		echo "::warning::Platform directory ${dest_dir} not found for ${filename}"
	fi
done
