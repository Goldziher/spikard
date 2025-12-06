#!/usr/bin/env bash
set -euo pipefail

shopt -s nullglob
mapfile -d '' -t gems < <(find . -maxdepth 1 -name 'spikard-*.gem' -print0 | sort -z)
if [ ${#gems[@]} -eq 0 ]; then
	echo "No gem artifacts found" >&2
	exit 1
fi

for gem in "${gems[@]}"; do
	echo "Pushing ${gem}"
	gem push "${gem}"
done
