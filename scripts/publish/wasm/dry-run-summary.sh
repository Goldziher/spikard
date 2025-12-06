#!/usr/bin/env bash
set -euo pipefail

echo "Dry run requested; WASM publish skipped." >>"${GITHUB_STEP_SUMMARY}"
