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
	echo "Unknown target: ${TARGET}"
	exit 1
	;;
esac

args=(--platform --release --target "${TARGET}" -o "npm/${PLATFORM_DIR}")
if [[ "${USE_NAPI_CROSS:-false}" == "true" ]]; then
	args+=("--use-napi-cross")
fi
if [[ "${USE_CROSS:-false}" == "true" ]]; then
	args+=("--use-cross")
fi

echo "Building for target ${TARGET} -> npm/${PLATFORM_DIR}"
pnpm exec napi build "${args[@]}"
