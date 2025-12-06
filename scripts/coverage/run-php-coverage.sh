#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd -P)"

cd "${REPO_ROOT}"

echo "Installing PHP dependencies..."
composer install --working-dir=packages/php --quiet

echo "Running PHP test suite with coverage..."
# phpunit.xml configures HTML, LCOV, and Clover output automatically
php -d extension="target/release/libspikard_php.dylib" \
	packages/php/vendor/bin/phpunit \
	--configuration packages/php/phpunit.xml \
	--coverage-text | tee packages/php/coverage.txt

# Check if coverage is below 80% by parsing coverage output
if [ -f "packages/php/coverage.txt" ]; then
	# Extract Lines percentage from PHPUnit text output
	coverage_percent=$(grep -oE 'Lines:\s+[0-9]+\.[0-9]+%' packages/php/coverage.txt | grep -oE '[0-9]+\.[0-9]+' || echo "0")

	if [ -n "$coverage_percent" ] && [ "$coverage_percent" != "0" ]; then
		# Use awk for portable floating point comparison
		threshold_met=$(awk -v cov="$coverage_percent" 'BEGIN { print (cov >= 80) ? "yes" : "no" }')

		if [ "$threshold_met" = "no" ]; then
			echo ""
			echo "ERROR: Coverage $coverage_percent% is below 80% threshold"
			exit 1
		fi
		echo ""
		echo "✅ Coverage: $coverage_percent% (passing 80% threshold)"
	else
		echo ""
		echo "WARNING: Could not parse coverage percentage from output"
	fi
fi

echo ""
echo "Coverage reports generated:"
echo "  - HTML: packages/php/htmlcov/index.html"
echo "  - LCOV: packages/php/coverage.lcov"
echo "  - Clover: target/clover.xml"
