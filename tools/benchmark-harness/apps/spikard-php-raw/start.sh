#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../../.." && pwd -P)"
EXT_DIR="${REPO_ROOT}/target/release"

if [[ -f "${EXT_DIR}/libspikard_php.dylib" ]]; then
	EXT_FILE="${EXT_DIR}/libspikard_php.dylib"
elif [[ -f "${EXT_DIR}/libspikard_php.so" ]]; then
	EXT_FILE="${EXT_DIR}/libspikard_php.so"
else
	echo "Spikard PHP extension not found in ${EXT_DIR}" >&2
	exit 1
fi

exec php -d "extension=${EXT_FILE}" server.php "$@"
