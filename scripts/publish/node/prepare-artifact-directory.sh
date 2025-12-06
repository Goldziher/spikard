#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

rm -rf "${REPO_ROOT}/crates/spikard-node/npm"
mkdir -p "${REPO_ROOT}/crates/spikard-node/npm"
shopt -s nullglob
for pkg in "${REPO_ROOT}"/node-artifacts/*.tar.gz; do
	echo "Unpacking ${pkg}"
	tmpdir=$(mktemp -d)
	tar -xzf "${pkg}" -C "${tmpdir}"
	if [ ! -d "${tmpdir}/npm" ]; then
		echo "::warning::npm directory missing inside ${pkg}"
		rm -rf "${tmpdir}"
		continue
	fi
	while IFS= read -r -d '' dir; do
		name=$(basename "${dir}")
		dest="${REPO_ROOT}/crates/spikard-node/npm/${name}"
		rm -rf "${dest}"
		cp -R "${dir}" "${dest}"
	done < <(find "${tmpdir}/npm" -mindepth 1 -maxdepth 1 -type d -print0)
	rm -rf "${tmpdir}"
done

if [ -d "${REPO_ROOT}/typescript-defs" ]; then
	cp "${REPO_ROOT}/typescript-defs/index.js" "${REPO_ROOT}/typescript-defs/index.d.ts" "${REPO_ROOT}/crates/spikard-node/" || true
fi
