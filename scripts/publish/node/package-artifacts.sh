#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"
pnpm exec napi artifacts --output-dir ./artifacts
if [ ! -d npm ]; then
	echo "npm artifact directory missing" >&2
	exit 1
fi

tar -czf ../../node-bindings-"${TARGET}".tar.gz -C . npm
