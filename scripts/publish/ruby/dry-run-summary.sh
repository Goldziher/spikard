#!/usr/bin/env bash
set -euo pipefail

echo "Dry run requested; gem artifacts ready:" >>"$GITHUB_STEP_SUMMARY"
find . -type f -name 'spikard-*.gem' -print >>"$GITHUB_STEP_SUMMARY"
