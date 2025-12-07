#!/usr/bin/env bash
set -euo pipefail

# Validate parameters
if [[ $# -lt 2 ]]; then
	echo "Usage: $0 <VERSION> <TAG>" >&2
	echo "  VERSION: Release version (e.g., 1.0.0)" >&2
	echo "  TAG:     Git tag (e.g., v1.0.0)" >&2
	exit 1
fi

VERSION="$1"
TAG="$2"

# Validate that VERSION is not empty
if [[ -z "${VERSION}" ]]; then
	echo "Error: VERSION parameter is empty" >&2
	exit 1
fi

# Validate that TAG is not empty
if [[ -z "${TAG}" ]]; then
	echo "Error: TAG parameter is empty" >&2
	exit 1
fi

# Output the formatted commit message
cat <<EOF
chore(homebrew): update spikard to ${VERSION}

Auto-update from release ${TAG}
EOF
