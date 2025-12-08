#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

rm -rf "${REPO_ROOT}/crates/spikard-node/npm"
mkdir -p "${REPO_ROOT}/crates/spikard-node/npm"
shopt -s nullglob
for pkg in "${REPO_ROOT}"/node-artifacts/*.tar.gz; do
	echo "=== Unpacking ${pkg} ==="
	tmpdir=$(mktemp -d)
	tar -xzf "${pkg}" -C "${tmpdir}"

	if [ ! -d "${tmpdir}/npm" ]; then
		echo "::warning::npm directory missing inside ${pkg}"
		rm -rf "${tmpdir}"
		continue
	fi

	echo "Contents of extracted tarball:"
	find "${tmpdir}/npm" -type f -name "*.node" -o -type f -name "package.json" | head -20

	while IFS= read -r -d '' dir; do
		name=$(basename "${dir}")
		dest="${REPO_ROOT}/crates/spikard-node/npm/${name}"

		echo "Copying platform ${name}:"
		echo "  Source: ${dir}"
		echo "  Dest: ${dest}"

		# List .node files before copy
		find "${dir}" -name "*.node" -exec ls -lh {} \;

		# Use mkdir + cp with dot notation for more reliable copying
		mkdir -p "${dest}"
		cp -a "${dir}"/. "${dest}/"

		# Verify .node files after copy
		echo "  After copy:"
		find "${dest}" -name "*.node" -exec ls -lh {} \; || echo "    No .node files found!"

	done < <(find "${tmpdir}/npm" -mindepth 1 -maxdepth 1 -type d -print0)
	rm -rf "${tmpdir}"
done

if [ -d "${REPO_ROOT}/typescript-defs" ]; then
	cp "${REPO_ROOT}/typescript-defs/index.js" "${REPO_ROOT}/typescript-defs/index.d.ts" "${REPO_ROOT}/crates/spikard-node/" || true
fi
