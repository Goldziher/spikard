#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

echo "Dry run requested; artifacts staged for PyPI:" >>"${GITHUB_STEP_SUMMARY}"
ls -1 "${REPO_ROOT}/dist" >>"${GITHUB_STEP_SUMMARY}"
