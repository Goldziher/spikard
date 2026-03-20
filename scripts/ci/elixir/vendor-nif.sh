#!/usr/bin/env bash
# Vendor the spikard-elixir Rust source code into packages/elixir/native/spikard_elixir/
# so Rustler can build a standalone crate without workspace inheritance.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"
ELIXIR_PKG_DIR="${REPO_ROOT}/packages/elixir"
NATIVE_DIR="${ELIXIR_PKG_DIR}/native/spikard_elixir"
SOURCE_CRATE="${REPO_ROOT}/crates/spikard-elixir"

echo "Vendoring spikard-elixir NIF source..."
echo "  Source: ${SOURCE_CRATE}"
echo "  Target: ${NATIVE_DIR}"

mkdir -p "${NATIVE_DIR}"

if [ -d "${SOURCE_CRATE}/src" ]; then
  rm -rf "${NATIVE_DIR}/src"
  cp -r "${SOURCE_CRATE}/src" "${NATIVE_DIR}/src"
  echo "  Copied src/"
else
  echo "ERROR: Source directory ${SOURCE_CRATE}/src not found" >&2
  exit 1
fi

# Copy and patch Cargo.toml from the source crate so vendored package metadata
# stays aligned with the Rust binding crate instead of drifting in this script.
VERSION="$(
  grep -A5 '\[workspace.package\]' "${REPO_ROOT}/Cargo.toml" | grep 'version' | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/'
)"
cp "${SOURCE_CRATE}/Cargo.toml" "${NATIVE_DIR}/Cargo.toml"
python3 - "${NATIVE_DIR}/Cargo.toml" "${VERSION}" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
version = sys.argv[2]
content = path.read_text()

replacements = [
    (r'^name = "spikard-elixir"$', 'name = "spikard_elixir"'),
    (r'^version\.workspace = true$', f'version = "{version}"'),
    (r'^edition\.workspace = true$', 'edition = "2024"'),
    (r'^\[lints\]\nworkspace = true\n', '[workspace]\n\n'),
    (r'^axum = \{ workspace = true \}$', 'axum = "0.8"'),
    (r'^tokio = \{ workspace = true \}$', 'tokio = { version = "1", features = ["full"] }'),
    (
        r'^tonic = \{ workspace = true \}$',
        'tonic = { version = "0.14", features = ["transport", "codegen", "gzip"] }',
    ),
    (r'^tower = \{ workspace = true, features = \["util"\] \}$', 'tower = { version = "0.5", features = ["util"] }'),
    (
        r'^spikard-core = \{ path = "\.\./spikard-core", features = \["di"\] \}$',
        'spikard-core = { path = "../../../../crates/spikard-core", features = ["di"] }',
    ),
    (
        r'^spikard-http = \{ path = "\.\./spikard-http", features = \["di"\] \}$',
        'spikard-http = { path = "../../../../crates/spikard-http", features = ["di"] }',
    ),
    (
        r'^spikard-bindings-shared = \{ path = "\.\./spikard-bindings-shared" \}$',
        'spikard-bindings-shared = { path = "../../../../crates/spikard-bindings-shared" }',
    ),
]

for pattern, replacement in replacements:
    content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

path.write_text(content)
PY

echo "  Synced Cargo.toml from crates/spikard-elixir (version: ${VERSION})"

if [ -f "${SOURCE_CRATE}/Cargo.lock" ]; then
  cp "${SOURCE_CRATE}/Cargo.lock" "${NATIVE_DIR}/Cargo.lock"
  echo "  Copied Cargo.lock"
fi

echo "Done! NIF source vendored to ${NATIVE_DIR}"
