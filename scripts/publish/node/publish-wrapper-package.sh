#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

cd "${REPO_ROOT}/packages/node"

# Build the TypeScript wrapper (tsup)
pnpm run build || {
	echo "::error::Failed to build @spikard/node wrapper package"
	exit 1
}

package_name="$(node -p "require('./package.json').name")"
package_version="$(node -p "require('./package.json').version")"

if npm view "${package_name}@${package_version}" version >/dev/null 2>&1; then
	echo "::notice::${package_name}@${package_version} already published; skipping."
	echo "${package_name}@${package_version} already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
	exit 0
fi

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
