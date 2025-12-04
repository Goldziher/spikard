#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"
mkdir -p target/tarpaulin
cargo tarpaulin \
	--config tarpaulin.toml \
	--workspace \
	--exclude spikard-node \
	--exclude spikard-rb \
	--timeout 300 \
	--exclude-files "examples/**" "tools/**" \
	--out Stdout \
	--out Html \
	--out Lcov \
	--fail-under 95
