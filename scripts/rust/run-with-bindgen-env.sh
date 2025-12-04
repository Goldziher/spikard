#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" == "Darwin" ]]; then
	if [[ -z "${LIBCLANG_PATH:-}" && -d "/opt/homebrew/opt/llvm/lib" ]]; then
		export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
		export DYLD_FALLBACK_LIBRARY_PATH="/opt/homebrew/opt/llvm/lib"
	fi
	if [[ -z "${BINDGEN_EXTRA_CLANG_ARGS:-}" && -x "/usr/bin/xcrun" ]]; then
		MACOS_SDK_PATH="$(xcrun --show-sdk-path)"
		export BINDGEN_EXTRA_CLANG_ARGS="-nostdinc -I${LIBCLANG_PATH:-/opt/homebrew/opt/llvm/lib}/clang/21/include -isysroot ${MACOS_SDK_PATH} -I${MACOS_SDK_PATH}/usr/include"
	fi
fi

if [[ $# -eq 0 ]]; then
	echo "Usage: $0 <command...>" >&2
	exit 1
fi

exec "$@"
