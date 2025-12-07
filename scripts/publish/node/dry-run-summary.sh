#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

echo "Dry run requested; Node binding tarballs staged:" >>"${GITHUB_STEP_SUMMARY}"
find "${REPO_ROOT}/node-artifacts" -name '*.tar.gz' -print >>"${GITHUB_STEP_SUMMARY}"
