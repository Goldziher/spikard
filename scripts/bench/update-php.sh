#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

for app in phalcon-raw trongate-raw spikard-php-raw spikard-php-validation; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && composer update --no-interaction --no-progress
	cd - >/dev/null
done
