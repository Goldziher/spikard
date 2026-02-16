#!/usr/bin/env bash
#
# Check if PHP package version exists on Packagist
#
# Arguments:
#   $1: Package version (required)
#
# Environment variables:
#   - GITHUB_OUTPUT: Path to GitHub Actions output file
#
# Usage:
#   ./check_packagist.sh "0.5.0"
#

set -euo pipefail

if [[ $# -lt 1 ]]; then
	echo "Usage: $0 <version>" >&2
	exit 1
fi

version="$1"
url="https://repo.packagist.org/p2/spikard/spikard.json"
max_attempts=3
attempt=1
http_code=""
tmp_json="$(mktemp)"

cleanup() {
	rm -f "${tmp_json}"
}
trap cleanup EXIT

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking Packagist for spikard/spikard==${version} (attempt ${attempt}/${max_attempts})"

	http_code=$(curl \
		--silent \
		--show-error \
		--retry 3 \
		--retry-delay 5 \
		--connect-timeout 30 \
		--max-time 60 \
		-w "%{http_code}" \
		-o "${tmp_json}" \
		"$url" 2>/dev/null || echo "000")

	if [ "$http_code" = "200" ] || [ "$http_code" = "404" ]; then
		break
	fi

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::Packagist check failed (HTTP $http_code), retrying in ${sleep_time}s..."
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$http_code" = "404" ]; then
	echo "exists=false" >>"$GITHUB_OUTPUT"
	echo "::notice::Packagist package spikard/spikard not found, will notify publish."
	exit 0
fi

if [ "$http_code" != "200" ]; then
	echo "::error::Failed to check Packagist after $max_attempts attempts (last HTTP code: $http_code)"
	exit 1
fi

if jq -e --arg v "$version" '.packages["spikard/spikard"][] | .version | select(. == $v or . == ("v" + $v))' "${tmp_json}" >/dev/null; then
	echo "exists=true" >>"$GITHUB_OUTPUT"
	echo "::notice::PHP package spikard/spikard==${version} already exists on Packagist"
else
	echo "exists=false" >>"$GITHUB_OUTPUT"
	echo "::notice::PHP package spikard/spikard==${version} not found on Packagist, will notify publish"
fi
