#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"

# napi artifacts moves .node files from build output into npm platform directories
# --output-dir: where napi build placed the .node files
# --npm-dir: where to create platform-specific npm packages
pnpm exec napi artifacts --output-dir ./artifacts --npm-dir ./npm

# Verify npm directory was created
if [ ! -d npm ]; then
	echo "npm artifact directory missing" >&2
	ls -la
	exit 1
fi

# Package the npm directory for artifact upload
tar -czf ../../node-bindings-"${TARGET}".tar.gz -C . npm
