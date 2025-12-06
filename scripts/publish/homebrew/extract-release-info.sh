#!/usr/bin/env bash
set -euo pipefail

tag="${1:?release tag required}"
version="${2:?version required}"
url="https://github.com/Goldziher/spikard/archive/${tag}.tar.gz"

{
	echo "tag=${tag}"
	echo "version=${version}"
	echo "url=${url}"
} >>"${GITHUB_OUTPUT}"

echo "Release info:"
echo "  Tag: ${tag}"
echo "  Version: ${version}"
echo "  URL: ${url}"
