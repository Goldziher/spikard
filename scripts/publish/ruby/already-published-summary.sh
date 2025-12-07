#!/usr/bin/env bash
set -euo pipefail

version="${1:?version required}"
echo "RubyGem version ${version} already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
