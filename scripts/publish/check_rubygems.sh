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
# Exit codes:
#   0: Check succeeded (exists=true or exists=false)
#   1: Check succeeded but version doesn't exist (exists=false)
#   2: Check failed due to transient error (exists=unknown)
#
# Implements retry logic with exponential backoff to handle transient network failures.
# Distinguishes between "version doesn't exist" (exit 1, non-blocking)
# and "check failed" (exit 2, should be retried at workflow level).
#

set -uo pipefail

if [[ $# -lt 1 ]]; then
	echo "Usage: $0 <version>" >&2
	exit 2
fi

version="$1"

# Call Python check script and capture exit code
# Exit codes: 0=exists, 1=not found, 2=check failed
python scripts/publish/ruby/check_rubygems_version.py "${version}"
exit_code=$?

case $exit_code in
	0)
		echo "::notice::Ruby gem spikard==${version} already exists on RubyGems"
		exit 0
		;;
	1)
		echo "::notice::Ruby gem spikard==${version} not found on RubyGems, will publish"
		exit 1
		;;
	2)
		echo "::error::Failed to check RubyGems - transient error, job will be retried"
		exit 2
		;;
	*)
		echo "::error::Unexpected exit code from check_rubygems_version.py: $exit_code"
		exit 2
		;;
esac
