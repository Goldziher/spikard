#!/usr/bin/env bash
set -euo pipefail

if ! command -v brew >/dev/null 2>&1; then
	echo "::error::Homebrew not found"
	exit 1
fi

if ! brew --prefix openssl@3 >/dev/null 2>&1; then
	echo "::error::OpenSSL 3 not found via Homebrew"
	exit 1
fi

OPENSSL_PREFIX=$(brew --prefix openssl@3)
echo "OPENSSL_LIB_DIR=${OPENSSL_PREFIX}/lib" >>"$GITHUB_ENV"
echo "OPENSSL_INCLUDE_DIR=${OPENSSL_PREFIX}/include" >>"$GITHUB_ENV"
