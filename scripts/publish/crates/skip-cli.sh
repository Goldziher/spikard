#!/usr/bin/env bash
set -euo pipefail

echo "::notice::spikard-cli not published to crates.io (install via: cargo install --git https://github.com/Goldziher/spikard spikard-cli)"
echo "spikard-cli skipped (not published to crates.io)" >>"${GITHUB_STEP_SUMMARY}"
