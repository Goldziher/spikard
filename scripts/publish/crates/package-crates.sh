#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}"

# Package spikard-core first (dependency of spikard)
core_packaged=0
core_status=0
cargo package -p spikard-core --allow-dirty || core_status=$?
if [ "${core_status}" -eq 0 ]; then
	core_packaged=1
else
	echo "::warning::Skipping spikard-core crate packaging."
fi

# Package spikard (depends on spikard-core)
spikard_packaged=0
spikard_status=0
cargo package -p spikard --allow-dirty --no-verify || spikard_status=$?
if [ "${spikard_status}" -eq 0 ]; then
	spikard_packaged=1
else
	echo "::warning::Skipping spikard crate packaging; spikard-core ${RELEASE_VERSION:-unknown} may not be available on crates.io."
fi

# Package spikard-http (depends on spikard-core)
http_packaged=0
http_status=0
cargo package -p spikard-http --allow-dirty --no-verify || http_status=$?
if [ "${http_status}" -eq 0 ]; then
	http_packaged=1
else
	echo "::warning::Skipping spikard-http crate packaging; dependencies may not be available on crates.io."
fi

# Skip spikard-cli (has optional Python binding dependencies not on crates.io)
echo "::notice::Skipping spikard-cli packaging (not published to crates.io; use cargo install --git or homebrew)"

mkdir -p "${REPO_ROOT}/crate-artifacts"
if [ "${core_packaged}" -eq 1 ]; then
	cp "${REPO_ROOT}"/target/package/spikard-core-*.crate "${REPO_ROOT}/crate-artifacts/"
fi
if [ "${spikard_packaged}" -eq 1 ]; then
	cp "${REPO_ROOT}"/target/package/spikard-[0-9]*.crate "${REPO_ROOT}/crate-artifacts/"
fi
if [ "${http_packaged}" -eq 1 ]; then
	cp "${REPO_ROOT}"/target/package/spikard-http-*.crate "${REPO_ROOT}/crate-artifacts/"
fi
