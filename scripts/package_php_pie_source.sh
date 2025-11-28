#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <version> <output-dir>" >&2
  exit 1
fi

VERSION="$1"
DEST_DIR="$2"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STAGING="$(mktemp -d "${ROOT}/.pie-src.XXXXXX")"

cleanup() {
  rm -rf "$STAGING"
}
trap cleanup EXIT

mkdir -p "$DEST_DIR"

# Rust workspace files
mkdir -p "$STAGING/workspace"
cp "$ROOT/Cargo.toml" "$STAGING/workspace/"
cp "$ROOT/Cargo.lock" "$STAGING/workspace/"
rsync -a --exclude 'target' --exclude 'debug' "$ROOT/crates" "$STAGING/workspace/"

# PHP extension scaffold
rsync -a "$ROOT/crates/spikard-php/" "$STAGING/php-ext/"

# PHP package wrapper
mkdir -p "$STAGING/packages/php"
rsync -a --exclude 'vendor' --exclude '.phpunit.result.cache' \
  "$ROOT/packages/php/" "$STAGING/packages/php/"

# Metadata
cp "$ROOT/LICENSE" "$STAGING/"
cp "$ROOT/README.md" "$STAGING/" 2>/dev/null || true
echo "$VERSION" > "$STAGING/VERSION"

# Create archive
ARCHIVE_NAME="spikard-php-${VERSION}-src.tgz"
tar -czf "${DEST_DIR}/${ARCHIVE_NAME}" -C "$STAGING" .

echo "Created ${DEST_DIR}/${ARCHIVE_NAME}"
