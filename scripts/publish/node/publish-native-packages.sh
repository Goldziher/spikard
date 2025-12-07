#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

shopt -s nullglob
first=true
for pkg in "${REPO_ROOT}"/crates/spikard-node/npm/*.tgz; do
	if [ "${first}" = false ]; then
		echo "Waiting 30s to avoid spam detection..."
		sleep 30
	fi
	first=false

	echo "Publishing ${pkg}"
	publish_log="$(mktemp)"
	set +e
	npm publish "${pkg}" --access public --ignore-scripts 2>&1 | tee "${publish_log}"
	status=${PIPESTATUS[0]}
	set -e
	if [ "${status}" -ne 0 ]; then
		if grep -q "previously published versions" "${publish_log}"; then
			echo "::notice::Package ${pkg} already published; skipping."
			echo "Package $(basename "${pkg}") already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
		else
			exit "${status}"
		fi
	fi
done
