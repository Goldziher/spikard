#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"

# Map Rust target to napi platform directory name
case "${TARGET}" in
x86_64-apple-darwin)
	PLATFORM_DIR="darwin-x64"
	;;
aarch64-apple-darwin)
	PLATFORM_DIR="darwin-arm64"
	;;
x86_64-unknown-linux-gnu)
	PLATFORM_DIR="linux-x64-gnu"
	;;
x86_64-pc-windows-msvc)
	PLATFORM_DIR="win32-x64-msvc"
	;;
*)
	echo "::error::Unknown target: ${TARGET}"
	exit 1
	;;
esac

# Verify npm directory was created by napi build --platform
if [ ! -d npm ]; then
	echo "::error::npm directory not found (napi build --platform should create it)"
	echo "Current directory: $(pwd)"
	ls -la
	exit 1
fi

# Verify the platform-specific directory exists and contains a .node file
echo "=== Verifying npm/${PLATFORM_DIR} for target ${TARGET} ==="
if [ ! -d "npm/${PLATFORM_DIR}" ]; then
	echo "::error::Platform directory npm/${PLATFORM_DIR} not found"
	echo "Available directories:"
	ls -la npm/
	exit 1
fi

shopt -s nullglob
nodes=()
while IFS= read -r -d '' file; do
	nodes+=("$file")
done < <(find "npm/${PLATFORM_DIR}" -maxdepth 1 -name "*.node" -print0)

if [ ${#nodes[@]} -eq 0 ]; then
	echo "::error::No .node file found in npm/${PLATFORM_DIR}"
	ls -la "npm/${PLATFORM_DIR}/"
	exit 1
fi

echo "✓ Found ${#nodes[@]} .node file(s) in npm/${PLATFORM_DIR}"
for node in "${nodes[@]}"; do
	ls -lh "$node"
done

echo "=== Creating tarball for ${TARGET} ==="
# Package the npm directory for artifact upload
tar -czf ../../node-bindings-"${TARGET}".tar.gz -C . npm
echo "✓ Created tarball: node-bindings-${TARGET}.tar.gz"
