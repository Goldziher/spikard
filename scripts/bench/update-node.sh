#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

run_with_timeout() {
	local timeout_seconds="$1"
	shift
	if command -v timeout >/dev/null 2>&1; then
		timeout "${timeout_seconds}" "$@"
	elif command -v gtimeout >/dev/null 2>&1; then
		gtimeout "${timeout_seconds}" "$@"
	elif command -v python3 >/dev/null 2>&1; then
		python3 - "$timeout_seconds" "$@" <<'PY'
import subprocess
import sys

timeout = int(sys.argv[1])
cmd = sys.argv[2:]
try:
    subprocess.run(cmd, check=True, timeout=timeout)
except subprocess.TimeoutExpired:
    print(f"Command timed out after {timeout}s: {' '.join(cmd)}", file=sys.stderr)
    sys.exit(124)
PY
	elif command -v python >/dev/null 2>&1; then
		python - "$timeout_seconds" "$@" <<'PY'
import subprocess
import sys

timeout = int(sys.argv[1])
cmd = sys.argv[2:]
try:
    subprocess.run(cmd, check=True, timeout=timeout)
except subprocess.TimeoutExpired:
    print(f"Command timed out after {timeout}s: {' '.join(cmd)}", file=sys.stderr)
    sys.exit(124)
PY
	else
		"$@"
	fi
}

BUN_UPDATE_TIMEOUT="${BUN_UPDATE_TIMEOUT:-1200}"
PNPM_UPDATE_TIMEOUT="${PNPM_UPDATE_TIMEOUT:-900}"

for package_json in tools/benchmark-harness/apps/*/package.json; do
	if [ ! -f "$package_json" ]; then
		continue
	fi

	app_dir="$(dirname "$package_json")"
	app_name="$(basename "$app_dir")"

	if [ -f "$app_dir/bun.lock" ] || [ -f "$app_dir/bun.lockb" ]; then
		if command -v bun >/dev/null 2>&1; then
			echo "Updating $app_name (bun)..."
			cd "$app_dir" && run_with_timeout "$BUN_UPDATE_TIMEOUT" bun update
			cd - >/dev/null
		else
			echo "Skipping $app_name (bun not installed)"
		fi
		continue
	fi

	echo "Updating $app_name..."
	cd "$app_dir" && run_with_timeout "$PNPM_UPDATE_TIMEOUT" pnpm up --latest
	cd - >/dev/null
done
