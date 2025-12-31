#!/usr/bin/env bash
set -euo pipefail

# Verify all test apps use version 0.7.0

EXPECTED_VERSION="0.7.0"
ERRORS=0

echo "Verifying package versions across all test apps..."
echo "Expected version: $EXPECTED_VERSION"
echo ""

# Node.js
echo "Checking Node.js test app..."
NODE_VERSION=$(jq -r '.dependencies["@spikard/node"]' tests/test_apps/node/package.json)
if [ "$NODE_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ Node.js: $NODE_VERSION"
else
	echo "❌ Node.js: Expected $EXPECTED_VERSION, got $NODE_VERSION"
	ERRORS=$((ERRORS + 1))
fi

# WASM
echo "Checking WASM test app..."
WASM_VERSION=$(jq -r '.dependencies["@spikard/wasm"]' tests/test_apps/wasm/package.json)
if [ "$WASM_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ WASM: $WASM_VERSION"
else
	echo "❌ WASM: Expected $EXPECTED_VERSION, got $WASM_VERSION"
	ERRORS=$((ERRORS + 1))
fi

# Ruby
echo "Checking Ruby test app..."
RUBY_VERSION=$(grep "gem 'spikard'" tests/test_apps/ruby/Gemfile | grep -o "'[0-9.]*'" | tr -d "'")
if [ "$RUBY_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ Ruby: $RUBY_VERSION"
else
	echo "❌ Ruby: Expected $EXPECTED_VERSION, got $RUBY_VERSION"
	ERRORS=$((ERRORS + 1))
fi

# PHP
echo "Checking PHP test app..."
PHP_VERSION=$(jq -r '.require["spikard/spikard"]' tests/test_apps/php/composer.json)
if [ "$PHP_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ PHP: $PHP_VERSION"
else
	echo "❌ PHP: Expected $EXPECTED_VERSION, got $PHP_VERSION"
	ERRORS=$((ERRORS + 1))
fi

# Rust
echo "Checking Rust test app..."
RUST_VERSION=$(grep '^spikard = ' tests/test_apps/rust/Cargo.toml | grep -o '"[0-9.]*"' | tr -d '"' | head -1)
if [ "$RUST_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ Rust: $RUST_VERSION"
else
	echo "❌ Rust: Expected $EXPECTED_VERSION, got $RUST_VERSION"
	ERRORS=$((ERRORS + 1))
fi

# Python
echo "Checking Python test app..."
PYTHON_VERSION=$(grep 'spikard==' tests/test_apps/python/pyproject.toml | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+')
if [ "$PYTHON_VERSION" = "$EXPECTED_VERSION" ]; then
	echo "✅ Python: $PYTHON_VERSION"
else
	echo "❌ Python: Expected $EXPECTED_VERSION, got $PYTHON_VERSION"
	ERRORS=$((ERRORS + 1))
fi

echo ""
if [ $ERRORS -eq 0 ]; then
	echo "✅ All test apps use version $EXPECTED_VERSION"
	exit 0
else
	echo "❌ Found $ERRORS version mismatch(es)"
	exit 1
fi
