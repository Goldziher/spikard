#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"

# napi build --platform already creates npm/platform-name/ directories with .node files
# We just need to verify they exist and package them

# Verify npm directory was created by napi build --platform
if [ ! -d npm ]; then
	echo "::error::npm directory not found (napi build --platform should create it)"
	echo "Current directory: $(pwd)"
	ls -la
	exit 1
fi

# List and verify contents of npm directories
echo "=== Verifying npm platform directories ==="
shopt -s nullglob
missing=0
platform_count=0

for dir in npm/*/; do
	platform_count=$((platform_count + 1))
	echo "Checking directory: $dir"

	nodes=("$dir"*.node)
	if [ ${#nodes[@]} -eq 0 ]; then
		echo "::error::Missing .node file in $dir"
		ls -la "$dir"
		missing=1
	else
		echo "✓ Found ${#nodes[@]} .node file(s) in $dir"
		for node in "${nodes[@]}"; do
			ls -lh "$node"
		done
	fi
done

if [ "${platform_count}" -eq 0 ]; then
	echo "::error::No platform directories found in npm/"
	ls -la npm/
	exit 1
fi

if [ "${missing}" -ne 0 ]; then
	echo "::error::Some npm directories are missing .node files"
	exit 1
fi

echo "=== All platforms verified, creating tarball ==="
# Package the npm directory for artifact upload
tar -czf ../../node-bindings-"${TARGET}".tar.gz -C . npm
echo "✓ Created tarball: node-bindings-${TARGET}.tar.gz"
