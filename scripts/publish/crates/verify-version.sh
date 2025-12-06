#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

tag_version="${1:?tag version required}"
cargo_version=$(grep '^\[workspace.package\]' -A 10 "${REPO_ROOT}/Cargo.toml" | grep '^version = ' | head -1 | sed -E 's/version = "(.*)"/\1/')
if [ "${cargo_version}" != "${tag_version}" ]; then
	echo "Version mismatch! Cargo: ${cargo_version}, tag: ${tag_version}" >&2
	exit 1
fi

echo "Cargo.toml version matches tag: ${cargo_version}"
