#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"

echo "Vendoring internal Rust crates for Ruby gem..."

cd "$REPO_ROOT"

VENDOR_DIR="packages/ruby/vendor/crates"
mkdir -p "$VENDOR_DIR"

# Read workspace version for vendored crates.
WORKSPACE_VERSION="$(
	awk -F'"' '
		/^\[workspace\.package\]$/ { in_ws=1; next }
		in_ws && /^version =/ { print $2; exit }
	' Cargo.toml
)"
if [ -z "${WORKSPACE_VERSION:-}" ]; then
	echo "Failed to detect workspace version from Cargo.toml" >&2
	exit 1
fi

# Copy internal crates
for crate in spikard-core spikard-http spikard-bindings-shared spikard-rb spikard-rb-macros; do
	echo "  Copying $crate..."
	rm -rf "${VENDOR_DIR:?}/${crate:?}"
	cp -r "crates/$crate" "$VENDOR_DIR/"
done

echo "Patching Cargo.toml files to remove workspace references..."

# Function to patch a Cargo.toml file
patch_cargo_toml() {
	local file=$1

	# Remove [workspace] sections
	sed -i.bak '/^\[workspace\]/d' "$file"

	# Replace workspace metadata with explicit values
	sed -i.bak "s/version\\.workspace = true/version = \"${WORKSPACE_VERSION}\"/" "$file"
	sed -i.bak 's/edition\.workspace = true/edition = "2024"/' "$file"
	sed -i.bak 's/authors\.workspace = true/authors = ["Na'\''aman Hirschfeld <nhirschfeld@gmail.com>"]/' "$file"
	sed -i.bak 's/license\.workspace = true/license = "MIT"/' "$file"
	sed -i.bak 's/repository\.workspace = true/repository = "https:\/\/github.com\/Goldziher\/spikard"/' "$file"
	sed -i.bak 's/homepage\.workspace = true/homepage = "https:\/\/github.com\/Goldziher\/spikard"/' "$file"

	# Replace workspace dependencies with explicit versions (external deps)
	# Handle both dot-style (foo.workspace) and brace-style (foo = { workspace = true })
	sed -i.bak 's/serde\.workspace = true/serde = { version = "1.0", features = ["derive"] }/' "$file"
	sed -i.bak 's/serde_json\.workspace = true/serde_json = "1.0"/' "$file"
	sed -i.bak 's/tracing\.workspace = true/tracing = "0.1"/' "$file"
	sed -i.bak 's/thiserror\.workspace = true/thiserror = "2.0"/' "$file"
	sed -i.bak 's/jsonschema\.workspace = true/jsonschema = { version = "0.37", default-features = false }/' "$file"
	sed -i.bak 's/flate2\.workspace = true/flate2 = { version = "=1.1.5", default-features = false, features = ["rust_backend"] }/' "$file"
	sed -i.bak 's/tower-http\.workspace = true/tower-http = { version = "0.6.8", features = ["fs", "trace", "compression-gzip", "compression-br", "compression-deflate", "cors", "request-id", "limit", "timeout"] }/' "$file"
	sed -i.bak 's/^http\.workspace = true$/http = "1.4"/' "$file"
	sed -i.bak 's/axum\.workspace = true/axum = { version = "0.8", features = ["multipart", "ws"] }/' "$file"
	sed -i.bak 's/tokio\.workspace = true/tokio = { version = "1", features = ["full"] }/' "$file"
	sed -i.bak 's/tower\.workspace = true/tower = "0.5"/' "$file"

	# Handle brace-style workspace references
	sed -i.bak 's/axum = { workspace = true }/axum = { version = "0.8", features = ["multipart", "ws"] }/' "$file"
	sed -i.bak 's/tokio = { workspace = true }/tokio = { version = "1", features = ["full"] }/' "$file"
	sed -i.bak 's/http = { workspace = true }/http = "1.4"/' "$file"
	sed -i.bak 's/tower = { workspace = true }/tower = "0.5"/' "$file"

	# Internal dependencies use path
	# Handle both brace-style and dot-style workspace references
	sed -i.bak 's|spikard-core = { workspace = true\(.*\)}|spikard-core = { path = "../spikard-core"\1}|' "$file"
	sed -i.bak 's|spikard-http = { workspace = true\(.*\)}|spikard-http = { path = "../spikard-http"\1}|' "$file"
	sed -i.bak 's|spikard-core\.workspace = true|spikard-core = { path = "../spikard-core" }|' "$file"
	sed -i.bak 's|spikard-http\.workspace = true|spikard-http = { path = "../spikard-http" }|' "$file"
	sed -i.bak 's|spikard-bindings-shared\.workspace = true|spikard-bindings-shared = { path = "../spikard-bindings-shared" }|' "$file"

	# Clean up backup files
	rm -f "$file.bak"
}

# Patch all vendored Cargo.toml files
for toml in "$VENDOR_DIR"/*/Cargo.toml; do
	if [ -f "$toml" ]; then
		echo "  Patching $(basename "$(dirname "$toml")")/Cargo.toml..."
		patch_cargo_toml "$toml"
	fi
done

echo "✓ Vendoring complete"
echo "Note: Using committed Cargo.lock with --locked flag to avoid workspace collisions"
