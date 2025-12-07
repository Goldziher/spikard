#!/usr/bin/env bash
set -euo pipefail

artifact_dir="${1:-cli-artifact}"

chmod +x "${artifact_dir}/spikard"
"${artifact_dir}/spikard" --version
"${artifact_dir}/spikard" --help
