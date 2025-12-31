#!/usr/bin/env bash
set -euo pipefail

# Run all test apps sequentially against published packages
# Usage: ./run-all.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_APPS_DIR="$(dirname "$SCRIPT_DIR")"

echo "Running all test apps..."
echo "======================================"

FAILED_APPS=()

# Python test app
if [[ -d "$TEST_APPS_DIR/python" ]]; then
	echo ""
	echo "Running Python test app..."
	if (
		cd "$TEST_APPS_DIR/python" &&
			UV_VENV_CLEAR=1 uv venv &&
			uv pip install -e ".[dev]" &&
			.venv/bin/python -m pytest test_published.py -v
	); then
		echo "✓ Python test app passed"
	else
		echo "✗ Python test app failed"
		FAILED_APPS+=("Python")
	fi
fi

# Node test app (when implemented)
if [[ -d "$TEST_APPS_DIR/node" ]]; then
	echo ""
	echo "Running Node test app..."
	if (cd "$TEST_APPS_DIR/node" && pnpm install && pnpm test); then
		echo "✓ Node test app passed"
	else
		echo "✗ Node test app failed"
		FAILED_APPS+=("Node")
	fi
fi

# Ruby test app (when implemented)
if [[ -d "$TEST_APPS_DIR/ruby" ]]; then
	echo ""
	echo "Running Ruby test app..."
	if (cd "$TEST_APPS_DIR/ruby" && bundle install && bundle exec rspec); then
		echo "✓ Ruby test app passed"
	else
		echo "✗ Ruby test app failed"
		FAILED_APPS+=("Ruby")
	fi
fi

# PHP test app (when implemented)
if [[ -d "$TEST_APPS_DIR/php" ]]; then
	echo ""
	echo "Running PHP test app..."
	if (cd "$TEST_APPS_DIR/php" && composer install && composer test); then
		echo "✓ PHP test app passed"
	else
		echo "✗ PHP test app failed"
		FAILED_APPS+=("PHP")
	fi
fi

echo ""
echo "======================================"
if [[ ${#FAILED_APPS[@]} -eq 0 ]]; then
	echo "All test apps passed ✓"
	exit 0
else
	echo "Failed test apps: ${FAILED_APPS[*]}"
	exit 1
fi
