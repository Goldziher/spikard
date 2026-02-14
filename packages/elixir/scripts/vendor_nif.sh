#!/usr/bin/env bash
# Vendor the spikard-elixir Rust source code into native/spikard_elixir/
# This creates a standalone crate that can be built by Rustler without workspace dependencies.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ELIXIR_PKG_DIR="$(dirname "$SCRIPT_DIR")"
NATIVE_DIR="$ELIXIR_PKG_DIR/native/spikard_elixir"
SOURCE_CRATE="$ELIXIR_PKG_DIR/../../crates/spikard-elixir"

echo "Vendoring spikard-elixir NIF source..."
echo "  Source: $SOURCE_CRATE"
echo "  Target: $NATIVE_DIR"

# Create native directory if it doesn't exist
mkdir -p "$NATIVE_DIR"

# Copy src directory
if [ -d "$SOURCE_CRATE/src" ]; then
	rm -rf "$NATIVE_DIR/src"
	cp -r "$SOURCE_CRATE/src" "$NATIVE_DIR/src"
	echo "  Copied src/"
else
	echo "ERROR: Source directory $SOURCE_CRATE/src not found"
	exit 1
fi

# Generate standalone Cargo.toml (no workspace inheritance)
# Read version from root Cargo.toml
VERSION=$(grep -A5 '\[workspace.package\]' "$ELIXIR_PKG_DIR/../../Cargo.toml" | grep 'version' | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')

cat >"$NATIVE_DIR/Cargo.toml" <<EOF
[package]
name = "spikard_elixir"
version = "$VERSION"
edition = "2024"
license = "MIT"
description = "Elixir bindings for Spikard HTTP framework"
repository = "https://github.com/Goldziher/spikard"

[lib]
name = "spikard_elixir"
crate-type = ["cdylib"]

# Empty workspace table to prevent inheriting from parent workspace
[workspace]

[dependencies]
axum = "0.8"
bytes = "1.5"
http-body-util = "0.1"
once_cell = "1.19"
rustler = "0.37"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
spikard-core = { path = "../../../../crates/spikard-core", features = ["di"] }
spikard-http = { path = "../../../../crates/spikard-http", features = ["di"] }
spikard-bindings-shared = { path = "../../../../crates/spikard-bindings-shared" }
tokio = { version = "1", features = ["full"] }
tower = { version = "0.5", features = ["util"] }
tracing = "0.1"
urlencoding = "2.1"
EOF

echo "  Generated Cargo.toml (version: $VERSION)"

# Copy Cargo.lock if it exists (for reproducible builds)
if [ -f "$SOURCE_CRATE/Cargo.lock" ]; then
	cp "$SOURCE_CRATE/Cargo.lock" "$NATIVE_DIR/Cargo.lock"
	echo "  Copied Cargo.lock"
fi

echo "Done! NIF source vendored to $NATIVE_DIR"
