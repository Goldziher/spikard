#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"
args=(--platform --release --target "${TARGET}" -o .)
if [[ "${USE_NAPI_CROSS:-false}" == "true" ]]; then
	args+=("--use-napi-cross")
fi
if [[ "${USE_CROSS:-false}" == "true" ]]; then
	args+=("--use-cross")
fi

pnpm exec napi build "${args[@]}"
