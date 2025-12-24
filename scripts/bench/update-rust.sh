#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

# shellcheck disable=SC2043
for app in spikard-rust-raw spikard-rust-validation; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && cargo update
	cd - >/dev/null
done
