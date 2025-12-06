#!/usr/bin/env bash
set -euo pipefail

echo "Dry run requested; cargo publish skipped." >>"${GITHUB_STEP_SUMMARY}"
