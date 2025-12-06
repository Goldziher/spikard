#!/usr/bin/env bash
set -euo pipefail

TAG="${1:-}"

if [[ -z "${TAG}" ]]; then
	echo "Usage: $0 <tag>" >&2
	exit 1
fi

echo "Triggering release for PHP bindings: ${TAG}"
gh workflow run release-php.yml --ref main -f tag="${TAG}"
