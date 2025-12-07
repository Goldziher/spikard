#!/usr/bin/env bash
set -euo pipefail

# wasm-pack installer URL
INSTALLER_URL="https://rustwasm.github.io/wasm-pack/installer/init.sh"

# Create temporary file for installer script
INSTALLER_TEMP=$(mktemp)
trap 'rm -f "${INSTALLER_TEMP}"' EXIT

# Download the wasm-pack installer with error handling
if ! curl -sSfL "${INSTALLER_URL}" -o "${INSTALLER_TEMP}"; then
	echo "Error: Failed to download wasm-pack installer from ${INSTALLER_URL}" >&2
	exit 1
fi

# Verify the file was downloaded and has content
if [ ! -s "${INSTALLER_TEMP}" ]; then
	echo "Error: Downloaded wasm-pack installer is empty" >&2
	exit 1
fi

# Execute the installer
if ! bash "${INSTALLER_TEMP}"; then
	echo "Error: Failed to execute wasm-pack installer" >&2
	exit 1
fi

echo "wasm-pack installed successfully"
