#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

os_name="${1:?OS name required}"
php_version="${2:?PHP version required}"

case "${os_name}" in
ubuntu-latest) os_label="linux" ;;
macos-14) os_label="macos" ;;
windows-latest) os_label="windows" ;;
*) os_label="unknown" ;;
esac

artifact_name="php-extension-${os_label}-${php_version}"
mkdir -p "${artifact_name}"

if [ -f "${REPO_ROOT}/target/release/libspikard_php.so" ]; then
	cp "${REPO_ROOT}/target/release/libspikard_php.so" "${artifact_name}/" || true
fi
if [ -f "${REPO_ROOT}/target/release/spikard_php.dll" ]; then
	cp "${REPO_ROOT}/target/release/spikard_php.dll" "${artifact_name}/" || true
fi
if [ -f "${REPO_ROOT}/target/release/libspikard_php.dylib" ]; then
	cp "${REPO_ROOT}/target/release/libspikard_php.dylib" "${artifact_name}/" || true
fi

tar -czf "${artifact_name}.tar.gz" "${artifact_name}"
rm -rf "${artifact_name}"
