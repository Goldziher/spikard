#!/usr/bin/env bash
set -euo pipefail

tag="${1:?release tag required}"
shopt -s nullglob
files=(dist/cli/spikard-cli-*)
if [ ${#files[@]} -eq 0 ]; then
	echo "No CLI artifacts to upload" >&2
	exit 1
fi

for file in "${files[@]}"; do
	gh release upload "${tag}" "${file}" --clobber
done
