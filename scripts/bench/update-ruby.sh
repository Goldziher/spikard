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

# Expect these to be passed as environment variables from Taskfile
RBENV_VERSION="${RBENV_VERSION:-}"
RBENV_BIN="${RBENV_BIN:-/opt/homebrew/bin/rbenv}"
BUNDLE_UPDATE_TIMEOUT="${BUNDLE_UPDATE_TIMEOUT:-600}"

USE_RBENV=0
if command -v "$RBENV_BIN" >/dev/null 2>&1; then
	USE_RBENV=1
fi

if [[ "$USE_RBENV" -eq 1 && -z "$RBENV_VERSION" ]]; then
	echo "Error: RBENV_VERSION environment variable not set"
	exit 1
fi

for app in hanami-api roda spikard-ruby; do
	echo "Updating $app..."
	app_dir="tools/benchmark-harness/apps/$app"

	if [[ ! -f "$app_dir/Gemfile" ]]; then
		echo "  Skipping $app (no Gemfile)"
		continue
	fi

	cd "$app_dir"

	if [[ "$USE_RBENV" -eq 1 ]]; then
		run_with_timeout "$BUNDLE_UPDATE_TIMEOUT" env RBENV_VERSION="$RBENV_VERSION" "$RBENV_BIN" exec bundle update --all
	else
		run_with_timeout "$BUNDLE_UPDATE_TIMEOUT" bundle update --all
	fi
	cd - >/dev/null
done
