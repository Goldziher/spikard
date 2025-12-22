#!/usr/bin/env bash
#
# Check if Ruby gem exists on RubyGems
#
# Arguments:
#   $1: Package version (required)
#
# Environment variables:
#   - GITHUB_OUTPUT: Path to GitHub Actions output file
#
# Usage:
#   ./check_rubygems.sh "0.5.0"
#

set -euo pipefail

if [[ $# -lt 1 ]]; then
	echo "Usage: $0 <version>" >&2
	exit 1
fi

version="$1"
python scripts/publish/ruby/check_rubygems_version.py "${version}"
