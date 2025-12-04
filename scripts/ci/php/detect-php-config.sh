#!/usr/bin/env bash
set -euo pipefail

php_config="$(command -v php-config || echo "php-config")"
echo "PHP_CONFIG=${php_config}" >>"${GITHUB_ENV}"
echo "Found PHP config at: ${php_config}"
"${php_config}" --version || true
