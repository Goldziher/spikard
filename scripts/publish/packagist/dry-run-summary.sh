#!/usr/bin/env bash
set -euo pipefail

echo "Dry run requested; Packagist notification skipped." >>"${GITHUB_STEP_SUMMARY}"
