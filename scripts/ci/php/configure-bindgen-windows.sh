#!/usr/bin/env bash
set -euo pipefail

# Extract PHP DevKit path and configure bindgen for ext-php-rs
# This script sets up the necessary environment variables for LLVM/Clang
# to properly compile ext-php-rs bindings on Windows

# Detect PHP installation path
PHP_PREFIX=$(php -r "echo PHP_INSTALLATION_DIR;" || php -r "echo dirname(PHP_EXECUTABLE);" || true)

if [[ -z "${PHP_PREFIX:-}" ]]; then
	echo "Warning: Could not detect PHP installation path; bindgen may not configure correctly"
	exit 0
fi

# Normalize paths for Windows
PHP_PREFIX="${PHP_PREFIX//\\//}"

# Extract LLVM/Clang path from environment or use common defaults
if [[ -n "${LLVM_HOME:-}" ]]; then
	LLVM_PREFIX="${LLVM_HOME//\\//}"
elif [[ -n "${CLANG_HOME:-}" ]]; then
	LLVM_PREFIX="${CLANG_HOME//\\//}"
else
	# Try to find LLVM in common Windows locations
	LLVM_PREFIX="C:/Program Files/LLVM"
fi

if [[ -d "${LLVM_PREFIX}/include" ]]; then
	echo "Detected LLVM at: ${LLVM_PREFIX}"
	BINDGEN_EXTRA_CLANG_ARGS="-I${LLVM_PREFIX}/include"

	# Add PHP include paths
	if [[ -d "${PHP_PREFIX}/include" ]]; then
		BINDGEN_EXTRA_CLANG_ARGS="${BINDGEN_EXTRA_CLANG_ARGS} -I${PHP_PREFIX}/include"
	fi

	# Set environment variable for bindgen
	echo "BINDGEN_EXTRA_CLANG_ARGS=${BINDGEN_EXTRA_CLANG_ARGS}" >>"${GITHUB_ENV}"
	echo "Configured bindgen with extra clang args: ${BINDGEN_EXTRA_CLANG_ARGS}"
else
	echo "Warning: LLVM not found at ${LLVM_PREFIX}; bindgen may not configure correctly"
	exit 0
fi
