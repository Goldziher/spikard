#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"

# List .node files in current directory before copying
echo "=== .node files in current directory ==="
ls -la -- *.node 2>/dev/null || echo "No .node files in current directory"

# Copy .node files from root into npm platform directories
bash "${SCRIPT_DIR}/copy-node-binaries.sh"

# List contents of npm directories after copying
echo "=== Contents of npm directories after copy ==="
shopt -s nullglob
for dir in npm/*/; do
	echo "Directory: $dir"
	ls -la "$dir"
done

# napi artifacts organizes .node files from npm platform directories
# --output-dir . tells it to look for .node files in current dir (default is ./artifacts)
pnpm exec napi artifacts --output-dir . --npm-dir ./npm

# List contents of npm directories after napi artifacts
echo "=== Contents of npm directories after napi artifacts ==="
for dir in npm/*/; do
	echo "Directory: $dir"
	ls -la "$dir"
done

# Verify npm directory was created
if [ ! -d npm ]; then
	echo "npm artifact directory missing" >&2
	ls -la
	exit 1
fi

# Verify .node files exist in npm directories
missing=0
for dir in npm/*/; do
	nodes=("$dir"*.node)
	if [ ${#nodes[@]} -eq 0 ]; then
		echo "::error::Missing .node file in $dir after packaging"
		missing=1
	fi
done

if [ "${missing}" -ne 0 ]; then
	echo "::error::Some npm directories are missing .node files before tarball creation"
	exit 1
fi

# Package the npm directory for artifact upload
tar -czf ../../node-bindings-"${TARGET}".tar.gz -C . npm
