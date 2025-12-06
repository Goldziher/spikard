#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

crate="${1:?crate name required}"
summary_label="${2:-${crate}}"

publish_log="$(mktemp)"
set +e
(
	cd "${REPO_ROOT}"
	cargo publish -p "${crate}" --token "${CARGO_REGISTRY_TOKEN}"
) 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e

if [ "${status}" -ne 0 ]; then
	if grep -qi "already uploaded" "${publish_log}"; then
		echo "::notice::${summary_label} already published; skipping."
		echo "${summary_label} already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
	else
		exit "${status}"
	fi
fi
