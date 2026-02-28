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

MIX_UPDATE_TIMEOUT="${MIX_UPDATE_TIMEOUT:-600}"

for mix_file in tools/benchmark-harness/apps/*/mix.exs; do
  if [ ! -f "$mix_file" ]; then
    continue
  fi
  app_dir="$(dirname "$mix_file")"
  app_name="$(basename "$app_dir")"
  echo "Updating $app_name..."
  cd "$app_dir" && run_with_timeout "$MIX_UPDATE_TIMEOUT" mix deps.update --all
  cd - >/dev/null
done
