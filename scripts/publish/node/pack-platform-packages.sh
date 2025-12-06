#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node/npm"
for dir in */; do
	if [ -f "${dir}/package.json" ]; then
		(
			cd "${dir}"
			pnpm pack
			shopt -s nullglob
			for tgz in ./*.tgz; do
				mv "${tgz}" ..
			done
		)
	fi
done
