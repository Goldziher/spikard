#!/usr/bin/env bash
set -euo pipefail

package_name="$(node -p "require('./package.json').name")"
package_version="$(node -p "require('./package.json').version")"

if npm view "${package_name}@${package_version}" version >/dev/null 2>&1; then
  echo "::notice::${package_name}@${package_version} already published; skipping."
  echo "${package_name}@${package_version} already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
  exit 0
fi

publish_log=$(mktemp)
set +e
npm publish --access public --provenance 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e
if [[ "${status}" -ne 0 ]]; then
  if grep -q "previously published versions" "${publish_log}"; then
    echo "::notice::@spikard/node-native already published; skipping."
  else
    exit "${status}"
  fi
fi
