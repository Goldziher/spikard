#!/usr/bin/env bash
set -euo pipefail

tag="${1:?release tag required}"

if gh release view "${tag}" >/dev/null 2>&1; then
	echo "Release ${tag} already exists"
else
	gh release create "${tag}" --title "${tag}" --generate-notes
fi
