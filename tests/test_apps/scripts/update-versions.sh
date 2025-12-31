#!/usr/bin/env bash
set -euo pipefail

# Update version pins across all test apps to match the published version
# Usage: ./update-versions.sh <version>

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
	echo "Usage: $0 <version>"
	echo "Example: $0 0.7.0"
	exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_APPS_DIR="$(dirname "$SCRIPT_DIR")"

echo "Updating test apps to version $VERSION..."

# Python: pyproject.toml
if [[ -f "$TEST_APPS_DIR/python/pyproject.toml" ]]; then
	sed -i.bak "s/spikard==[0-9.]*/spikard==$VERSION/" "$TEST_APPS_DIR/python/pyproject.toml"
	rm -f "$TEST_APPS_DIR/python/pyproject.toml.bak"
	echo "✓ Updated Python app to $VERSION"
fi

# Node: package.json (when implemented)
if [[ -f "$TEST_APPS_DIR/node/package.json" ]]; then
	# Use jq if available, otherwise sed
	if command -v jq &>/dev/null; then
		jq ".dependencies[\"@spikard/node\"] = \"$VERSION\"" "$TEST_APPS_DIR/node/package.json" >"$TEST_APPS_DIR/node/package.json.tmp"
		mv "$TEST_APPS_DIR/node/package.json.tmp" "$TEST_APPS_DIR/node/package.json"
	else
		# Fallback sed for scoped packages
		sed -i.bak "s|\"@spikard/node\": \"[^\"]*\"|\"@spikard/node\": \"$VERSION\"|" "$TEST_APPS_DIR/node/package.json"
		rm -f "$TEST_APPS_DIR/node/package.json.bak"
	fi
	echo "✓ Updated Node app to $VERSION"
fi

# Node: test expectations
if [[ -f "$TEST_APPS_DIR/node/test.spec.ts" ]]; then
	sed -i.bak "/@spikard\\/node/ s|toBe(\"[0-9.]*\")|toBe(\"$VERSION\")|" "$TEST_APPS_DIR/node/test.spec.ts"
	rm -f "$TEST_APPS_DIR/node/test.spec.ts.bak"
	echo "✓ Updated Node tests to $VERSION"
fi

# Ruby: Gemfile (when implemented)
if [[ -f "$TEST_APPS_DIR/ruby/Gemfile" ]]; then
	sed -i.bak "s/gem 'spikard', '.*'/gem 'spikard', '$VERSION'/" "$TEST_APPS_DIR/ruby/Gemfile"
	rm -f "$TEST_APPS_DIR/ruby/Gemfile.bak"
	echo "✓ Updated Ruby app to $VERSION"
fi

if [[ -f "$TEST_APPS_DIR/ruby/spec/app_spec.rb" ]]; then
	sed -i.bak "s/spikard ([0-9.][0-9.]*)/spikard ($VERSION)/" "$TEST_APPS_DIR/ruby/spec/app_spec.rb"
	rm -f "$TEST_APPS_DIR/ruby/spec/app_spec.rb.bak"
	echo "✓ Updated Ruby tests to $VERSION"
fi

# WASM: package.json (when implemented)
if [[ -f "$TEST_APPS_DIR/wasm/package.json" ]]; then
	echo "Updating WASM test app..."
	if command -v jq &>/dev/null; then
		jq ".dependencies[\"@spikard/wasm\"] = \"$VERSION\"" "$TEST_APPS_DIR/wasm/package.json" >"$TEST_APPS_DIR/wasm/package.json.tmp"
		mv "$TEST_APPS_DIR/wasm/package.json.tmp" "$TEST_APPS_DIR/wasm/package.json"
	else
		sed -i.bak "s|\"@spikard/wasm\": \"[^\"]*\"|\"@spikard/wasm\": \"$VERSION\"|" "$TEST_APPS_DIR/wasm/package.json"
		rm -f "$TEST_APPS_DIR/wasm/package.json.bak"
	fi
	echo "✓ Updated WASM app to $VERSION"
fi

if [[ -f "$TEST_APPS_DIR/wasm/test.spec.js" ]]; then
	sed -i.bak "/@spikard\\/wasm/ s|toBe(\"[0-9.]*\")|toBe(\"$VERSION\")|" "$TEST_APPS_DIR/wasm/test.spec.js"
	rm -f "$TEST_APPS_DIR/wasm/test.spec.js.bak"
	echo "✓ Updated WASM tests to $VERSION"
fi

# PHP: composer.json (when implemented)
if [[ -f "$TEST_APPS_DIR/php/composer.json" ]]; then
	if command -v jq &>/dev/null; then
		jq ".require[\"spikard/spikard\"] = \"$VERSION\"" "$TEST_APPS_DIR/php/composer.json" >"$TEST_APPS_DIR/php/composer.json.tmp"
		mv "$TEST_APPS_DIR/php/composer.json.tmp" "$TEST_APPS_DIR/php/composer.json"
	else
		sed -i.bak "s|\"spikard/spikard\": \"[^\"]*\"|\"spikard/spikard\": \"$VERSION\"|" "$TEST_APPS_DIR/php/composer.json"
		rm -f "$TEST_APPS_DIR/php/composer.json.bak"
	fi
	echo "✓ Updated PHP app to $VERSION"
fi

if [[ -f "$TEST_APPS_DIR/php/tests/AppTest.php" ]]; then
	sed -E -i.bak "s/'[0-9]+\\.[0-9]+\\.[0-9]+'/'$VERSION'/" "$TEST_APPS_DIR/php/tests/AppTest.php"
	rm -f "$TEST_APPS_DIR/php/tests/AppTest.php.bak"
	echo "✓ Updated PHP tests to $VERSION"
fi

# Rust: Cargo.toml + test expectations
if [[ -f "$TEST_APPS_DIR/rust/Cargo.toml" ]]; then
	sed -i.bak "s/^spikard = \".*\"/spikard = \"$VERSION\"/" "$TEST_APPS_DIR/rust/Cargo.toml"
	rm -f "$TEST_APPS_DIR/rust/Cargo.toml.bak"
	echo "✓ Updated Rust app to $VERSION"
fi

if [[ -f "$TEST_APPS_DIR/rust/tests/integration.rs" ]]; then
	sed -i.bak "s/spikard = \"[0-9.]*\"/spikard = \"$VERSION\"/" "$TEST_APPS_DIR/rust/tests/integration.rs"
	rm -f "$TEST_APPS_DIR/rust/tests/integration.rs.bak"
	echo "✓ Updated Rust tests to $VERSION"
fi

# Python: test expectations
if [[ -f "$TEST_APPS_DIR/python/test_published.py" ]]; then
	sed -E -i.bak "s/[0-9]+\\.[0-9]+\\.[0-9]+/$VERSION/g" "$TEST_APPS_DIR/python/test_published.py"
	rm -f "$TEST_APPS_DIR/python/test_published.py.bak"
	echo "✓ Updated Python tests to $VERSION"
fi

echo ""
echo "Version update complete. Next steps:"
echo "1. Review changes: git diff tests/test_apps/"
echo "2. Commit: git add tests/test_apps/ && git commit -m 'Update test apps to v$VERSION'"
