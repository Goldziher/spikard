#!/usr/bin/env bash
set -euo pipefail

shopt -s nullglob
mapfile -d '' -t gems < <(find . -maxdepth 1 -name 'spikard-*.gem' -print0 | sort -z)
if [ ${#gems[@]} -eq 0 ]; then
	echo "No gem artifacts found" >&2
	exit 1
fi

declare -A published
while IFS=$'\t' read -r version platform; do
	if [ -n "${version}" ] && [ -n "${platform}" ]; then
		published["${version}|${platform}"]=1
	fi
done < <(
	python - <<'PY'
import json
import urllib.request

try:
    with urllib.request.urlopen("https://rubygems.org/api/v1/versions/spikard.json", timeout=10) as resp:
        data = json.load(resp)
except Exception:
    data = []

for entry in data:
    version = entry.get("number")
    platform = entry.get("platform")
    if version and platform:
        print(f"{version}\t{platform}")
PY
)

for gem in "${gems[@]}"; do
	spec_info="$(ruby -r rubygems/package -e 'spec = Gem::Package.new(ARGV[0]).spec; puts [spec.version.to_s, spec.platform.to_s].join("\t")' "${gem}")"
	version="${spec_info%%$'\t'*}"
	platform="${spec_info#*$'\t'}"
	key="${version}|${platform}"

	if [ -n "${published[${key}]+x}" ]; then
		echo "::notice::Skipping already published spikard ${version} (${platform})"
		echo "Skipping already published spikard ${version} (${platform})" >>"${GITHUB_STEP_SUMMARY:-/dev/null}" 2>/dev/null || true
		continue
	fi

	echo "Pushing ${gem} (spikard ${version}, ${platform})"
	publish_log="$(mktemp)"
	set +e
	gem push "${gem}" 2>&1 | tee "${publish_log}"
	status=${PIPESTATUS[0]}
	set -e

	if [ "${status}" -ne 0 ]; then
		if grep -Eq "previously published versions|Repushing of gem versions is not allowed" "${publish_log}"; then
			echo "::notice::spikard ${version} (${platform}) already published; skipping."
			echo "spikard ${version} (${platform}) already published; skipping." >>"${GITHUB_STEP_SUMMARY:-/dev/null}" 2>/dev/null || true
			published["${key}"]=1
			continue
		fi

		exit "${status}"
	fi

	published["${key}"]=1
done
