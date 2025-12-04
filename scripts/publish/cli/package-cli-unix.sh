#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

target="${1:?target triple required}"
stage="${REPO_ROOT}/spikard-cli-${target}"
rm -rf "${stage}"
mkdir -p "${stage}"
cp "${REPO_ROOT}/target/${target}/release/spikard" "${stage}/"
cp "${REPO_ROOT}/LICENSE" "${stage}/" || echo "No LICENSE file"
cp "${REPO_ROOT}/README.md" "${stage}/" || echo "No README file"
tar -czf "${stage}.tar.gz" -C "${REPO_ROOT}" "spikard-cli-${target}"
rm -rf "${stage}"
