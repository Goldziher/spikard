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

BUNDLE_UPDATE_TIMEOUT="${BUNDLE_UPDATE_TIMEOUT:-600}"

for app in hanami-api roda spikard-ruby; do
  echo "Updating $app..."
  app_dir="tools/benchmark-harness/apps/$app"

  if [[ ! -f "$app_dir/Gemfile" ]]; then
    echo "  Skipping $app (no Gemfile)"
    continue
  fi

  cd "$app_dir"
  run_with_timeout "$BUNDLE_UPDATE_TIMEOUT" bundle update --all
  cd - >/dev/null
done
