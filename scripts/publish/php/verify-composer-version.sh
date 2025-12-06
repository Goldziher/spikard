#!/usr/bin/env bash
set -euo pipefail

expected="${1:?version required}"

composer_version=$(grep '"version"' packages/php/composer.json | head -1 | sed -E 's/.*"version": "(.*)".*/\1/')
if [ "${composer_version}" != "${expected}" ]; then
	echo "Version mismatch! composer.json: ${composer_version}, tag: ${expected}" >&2
	exit 1
fi

echo "composer.json version matches tag: ${composer_version}"
