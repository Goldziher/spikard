#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

target="${1:?target triple required}"
cd "${REPO_ROOT}"
cargo build --release --target "${target}" --package spikard-cli
