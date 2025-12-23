#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

# Expect these to be passed as environment variables from Taskfile
RBENV_VERSION="${RBENV_VERSION:-}"
RBENV_BIN="${RBENV_BIN:-/opt/homebrew/bin/rbenv}"
BUNDLER_VERSION="${BUNDLER_VERSION:-2.7.2}"

USE_RBENV=0
if command -v "$RBENV_BIN" >/dev/null 2>&1; then
	USE_RBENV=1
fi

if [[ "$USE_RBENV" -eq 1 && -z "$RBENV_VERSION" ]]; then
	echo "Error: RBENV_VERSION environment variable not set"
	exit 1
fi

for app in hanami-api-validation hanami-api-raw roda-validation roda-raw spikard-ruby-validation spikard-ruby-raw; do
	echo "Updating $app..."
	cd "tools/benchmark-harness/apps/$app"

	# For spikard-ruby, regenerate Gemfile.lock to avoid platform mismatch
	# since it's a path-based gem with native extensions
	if [[ "$app" == "spikard-ruby" ]]; then
		rm -f Gemfile.lock
	fi

	if [[ "$USE_RBENV" -eq 1 ]]; then
		RBENV_VERSION="$RBENV_VERSION" "$RBENV_BIN" exec bundle "_${BUNDLER_VERSION}_" update --all
	else
		bundle "_${BUNDLER_VERSION}_" update --all
	fi
	cd - >/dev/null
done
