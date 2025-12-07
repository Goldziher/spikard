#!/usr/bin/env bash
set -euo pipefail

php_config="$(command -v php-config || true)"
if [[ -z "${php_config:-}" ]]; then
	echo "php-config not found; skipping PHP extension build on this platform"
	exit 0
fi
echo "PHP_CONFIG=${php_config}" >>"${GITHUB_ENV}"
echo "Found PHP config at: ${php_config}"
"${php_config}" --version || true
