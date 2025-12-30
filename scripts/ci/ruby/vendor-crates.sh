#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../../.." && pwd -P)"

echo "Vendoring internal Rust crates for Ruby gem..."

cd "$REPO_ROOT"

VENDOR_DIR="packages/ruby/vendor/crates"
rm -rf "$VENDOR_DIR"
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
	rm -rf "${VENDOR_DIR:?}/$crate"
	cp -r "crates/$crate" "$VENDOR_DIR/"
done

for crate in spikard-core spikard-http spikard-bindings-shared spikard-rb spikard-rb-macros; do
	if [ ! -f "${VENDOR_DIR}/${crate}/Cargo.toml" ]; then
		echo "Missing vendored crate ${crate} in ${VENDOR_DIR}" >&2
		exit 1
	fi
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
	# Use perl for more flexible matching that handles all formats

	# Handle { workspace = true } format
	perl -i.bak -pe 's/serde = \{ workspace = true \}/serde = { version = "1.0", features = ["derive"] }/g' "$file"

	# Fallback: handle .workspace = true format
	perl -i.bak -pe 's/serde\.workspace = true/serde = { version = "1.0", features = ["derive"] }/g' "$file"
	perl -i.bak -pe 's/serde_json\.workspace = true/serde_json = "1.0"/g' "$file"
	perl -i.bak -pe 's/tracing\.workspace = true/tracing = "0.1"/g' "$file"
	perl -i.bak -pe 's/thiserror\.workspace = true/thiserror = "2.0"/g' "$file"
	perl -i.bak -pe 's/jsonschema\.workspace = true/jsonschema = { version = "0.37", default-features = false }/g' "$file"
	perl -i.bak -pe 's/flate2\.workspace = true/flate2 = { version = "=1.1.5", default-features = false, features = ["rust_backend"] }/g' "$file"
	# Process tower-http BEFORE http to avoid partial matches
	perl -i.bak -pe 's/tower-http\.workspace = true/tower-http = { version = "0.6.8", features = ["trace", "request-id", "compression-gzip", "compression-br", "timeout", "limit", "fs", "set-header", "sensitive-headers"] }/g' "$file"
	perl -i.bak -pe 's/\bhttp\.workspace = true/http = "1.4"/g' "$file"
	perl -i.bak -pe 's/\btokio\.workspace = true/tokio = { version = "1", features = ["full"] }/g' "$file"
	perl -i.bak -pe 's/\btower\.workspace = true/tower = "0.5"/g' "$file"
	perl -i.bak -pe 's/tracing-subscriber\.workspace = true/tracing-subscriber = { version = "0.3", features = ["env-filter"] }/g' "$file"
	perl -i.bak -pe 's/tower_governor\.workspace = true/tower_governor = "0.8"/g' "$file"
	perl -i.bak -pe 's/jsonwebtoken\.workspace = true/jsonwebtoken = { version = "10.2", features = ["use_pem", "rust_crypto"] }/g' "$file"
	perl -i.bak -pe 's/utoipa\.workspace = true/utoipa = { version = "5", features = ["axum_extras", "chrono", "uuid"] }/g' "$file"
	perl -i.bak -pe 's/utoipa-swagger-ui\.workspace = true/utoipa-swagger-ui = { version = "9", features = ["axum"] }/g' "$file"
	perl -i.bak -pe 's/utoipa-redoc\.workspace = true/utoipa-redoc = { version = "6", features = ["axum"] }/g' "$file"
	perl -i.bak -pe 's/axum\.workspace = true/axum = { version = "0.8", features = ["multipart", "ws"] }/g' "$file"

	# Handle { workspace = true, ... } formats with features or optionals
	perl -i.bak -pe 's/axum = \{ workspace = true, features = (.*?)\}/axum = { version = "0.8", features = $1 }/g' "$file"
	perl -i.bak -pe 's/tokio = \{ workspace = true,/tokio = { version = "1", features = ["full"],/g' "$file"
	perl -i.bak -pe 's/thiserror = \{ workspace = true, optional = true/thiserror = { version = "2.0", optional = true/g' "$file"

	# Handle simple { workspace = true } format without other attrs
	perl -i.bak -pe 's/axum = \{ workspace = true \}/axum = { version = "0.8", features = ["multipart", "ws"] }/g' "$file"
	perl -i.bak -pe 's/tokio = \{ workspace = true \}/tokio = { version = "1", features = ["full"] }/g' "$file"
	perl -i.bak -pe 's/http = \{ workspace = true \}/http = "1.4"/g' "$file"

	# Internal dependencies use path (handle both .workspace = true and { workspace = true } formats)
	perl -i.bak -pe 's/spikard-core = \{ workspace = true/spikard-core = { path = "..\/spikard-core"/g' "$file"
	perl -i.bak -pe 's/spikard-http = \{ workspace = true/spikard-http = { path = "..\/spikard-http"/g' "$file"
	perl -i.bak -pe 's/spikard-core\.workspace = true/spikard-core = { path = "..\/spikard-core" }/g' "$file"
	perl -i.bak -pe 's/spikard-http\.workspace = true/spikard-http = { path = "..\/spikard-http" }/g' "$file"

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
