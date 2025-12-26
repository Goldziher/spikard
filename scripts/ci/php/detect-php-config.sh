#!/usr/bin/env bash
set -euo pipefail

# Detect if running on Windows
is_windows() {
	[[ -n "${MSYSTEM:-}" ]] || [[ -n "${MINGW_PREFIX:-}" ]] || [[ "${OS:-}" == "Windows_NT" ]] || [[ -n "${ProgramFiles:-}" ]]
}

# Extract PHP configuration from php -i on Windows
php_info_extract() {
	local key="$1"
	php -i 2>/dev/null | grep -i "^$key" | head -1 | awk -F' => ' '{print $2}' | xargs
}

if is_windows; then
	echo "Detected Windows environment"

	# On Windows, create a wrapper script since php-config may not exist
	php_version="$(php -r 'echo phpversion();')" || {
		echo "php-config not found; skipping PHP extension build on this platform"
		exit 0
	}

	echo "Detected PHP version: $php_version"

	# Extract PHP paths from php -i
	php_prefix="$(php_info_extract 'extension_dir' | sed 's|/ext$||')" || php_prefix="$(php -r 'echo dirname(PHP_EXECUTABLE);' 2>/dev/null || true)"

	if [[ -z "${php_prefix:-}" ]]; then
		echo "Warning: Could not detect PHP installation directory"
		exit 0
	fi

	# Normalize Windows paths to use forward slashes for consistency
	php_prefix="${php_prefix//\\//}"

	echo "Detected PHP prefix: $php_prefix"

	# Create a temporary php-config wrapper script
	php_config_wrapper="/tmp/php-config"
	mkdir -p "$(dirname "$php_config_wrapper")"

	cat >"$php_config_wrapper" <<'PHPCONFIG'
#!/usr/bin/env bash
# Wrapper script for php-config on Windows
# Extracts PHP configuration from php -i since php-config is not available

php_info_extract() {
	local key="$1"
	php -i 2>/dev/null | grep -i "^$key" | head -1 | awk -F' => ' '{print $2}' | xargs
}

php_prefix="$(php_info_extract 'extension_dir' | sed 's|/ext$||')" || php_prefix="$(php -r 'echo dirname(PHP_EXECUTABLE);')"
php_prefix="${php_prefix//\\//}"

case "$1" in
	--version)
		php -r 'echo "PHP ".phpversion();'
		;;
	--includes)
		echo "-I${php_prefix}/include -I${php_prefix}/include/php -I${php_prefix}/include/php/main -I${php_prefix}/include/php/TSRM"
		;;
	--libs)
		# Return minimal libs for Windows
		echo "-L${php_prefix}/lib -lphp8"
		;;
	--ldflags)
		echo "-L${php_prefix}/lib"
		;;
	*)
		echo "Unknown option: $1" >&2
		exit 1
		;;
esac
PHPCONFIG

	chmod +x "$php_config_wrapper"
	php_config="$php_config_wrapper"

else
	# Unix-like systems: use native php-config
	php_config="$(command -v php-config || true)"
	if [[ -z "${php_config:-}" ]]; then
		echo "php-config not found; skipping PHP extension build on this platform"
		exit 0
	fi
fi

echo "PHP_CONFIG=${php_config}" >>"${GITHUB_ENV}"
echo "Found PHP config at: ${php_config}"
"${php_config}" --version || true
