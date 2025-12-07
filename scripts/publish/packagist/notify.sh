#!/usr/bin/env bash
set -euo pipefail

tag="${1:?release tag required}"

if [ -z "${PACKAGIST_TOKEN:-}" ]; then
	echo "::warning::Packagist token not available; skipping automatic package update. Package will be indexed on next scheduled refresh."
	echo "Packagist token not available; manual refresh may be needed." >>"${GITHUB_STEP_SUMMARY}"
	exit 0
fi

curl -X POST https://packagist.org/api/github \
	-H "Authorization: Bearer ${PACKAGIST_TOKEN}" \
	-H "Content-Type: application/json" \
	-d "{\"repository\":\"https://github.com/Goldziher/spikard\",\"tag\":\"${tag}\"}" ||
	echo "::warning::Failed to notify Packagist; package may require manual refresh."
