#!/usr/bin/env bash
set -euo pipefail

publish_log="$(mktemp)"
set +e
npm publish --access public 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e

if [ "${status}" -ne 0 ]; then
	if grep -q "previously published versions" "${publish_log}"; then
		echo "::notice::@spikard/wasm already published; skipping."
		echo "@spikard/wasm already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
	else
		exit "${status}"
	fi
fi
