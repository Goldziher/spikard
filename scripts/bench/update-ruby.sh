#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

# Expect these to be passed as environment variables from Taskfile
RBENV_VERSION="${RBENV_VERSION:-}"
RBENV_BIN="${RBENV_BIN:-/opt/homebrew/bin/rbenv}"
BUNDLER_VERSION="${BUNDLER_VERSION:-2.7.2}"

if [[ -z "$RBENV_VERSION" ]]; then
	echo "Error: RBENV_VERSION environment variable not set"
	exit 1
fi

for app in hanami-api-dto hanami-api-raw roda-dto roda-raw spikard-ruby; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app" && RBENV_VERSION="$RBENV_VERSION" "$RBENV_BIN" exec bundle "_${BUNDLER_VERSION}_" update --all
	cd - >/dev/null
done
