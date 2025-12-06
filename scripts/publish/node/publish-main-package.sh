#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/crates/spikard-node"
publish_log="$(mktemp)"
set +e
pnpm publish --access public --no-git-checks 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e
if [ "${status}" -ne 0 ]; then
	if grep -q "previously published versions" "${publish_log}"; then
		echo "::notice::@spikard/node already published; skipping."
		echo "@spikard/node already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
	else
		exit "${status}"
	fi
fi
