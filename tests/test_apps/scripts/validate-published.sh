#!/usr/bin/env bash
set -euo pipefail

# Validate that the specified version is available on package registries
# Usage: ./validate-published.sh <version>

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
	echo "Usage: $0 <version>"
	echo "Example: $0 0.7.0"
	exit 1
fi

echo "Validating version $VERSION on package registries..."
echo "======================================"

FAILED_CHECKS=()

# Check PyPI
echo ""
echo "Checking PyPI..."
if curl -sSf "https://pypi.org/pypi/spikard/$VERSION/json" >/dev/null 2>&1; then
	echo "✓ spikard $VERSION found on PyPI"
else
	echo "✗ spikard $VERSION NOT found on PyPI"
	FAILED_CHECKS+=("PyPI")
fi

# Check npm (when published)
echo ""
echo "Checking npm..."
if curl -sSf "https://registry.npmjs.org/@spikard/node/$VERSION" >/dev/null 2>&1; then
	echo "✓ @spikard/node $VERSION found on npm"
else
	echo "✗ @spikard/node $VERSION NOT found on npm (may not be published yet)"
	# Don't fail on npm for now
fi

# Check RubyGems (when published)
echo ""
echo "Checking RubyGems..."
if curl -sSf "https://rubygems.org/api/v1/versions/spikard.json" | grep -q "\"number\":\"$VERSION\""; then
	echo "✓ spikard $VERSION found on RubyGems"
else
	echo "✗ spikard $VERSION NOT found on RubyGems (may not be published yet)"
	# Don't fail on RubyGems for now
fi

# Check Packagist (when published)
echo ""
echo "Checking Packagist..."
if curl -sSf "https://repo.packagist.org/p2/spikard/spikard.json" | grep -q "\"version\":\"$VERSION\""; then
	echo "✓ spikard/spikard $VERSION found on Packagist"
else
	echo "✗ spikard/spikard $VERSION NOT found on Packagist (may not be published yet)"
	# Don't fail on Packagist for now
fi

echo ""
echo "======================================"
if [[ ${#FAILED_CHECKS[@]} -eq 0 ]]; then
	echo "All critical registry checks passed ✓"
	echo ""
	echo "Next steps:"
	echo "1. Update test apps: ./update-versions.sh $VERSION"
	echo "2. Run all tests: ./run-all.sh"
	exit 0
else
	echo "Failed registry checks: ${FAILED_CHECKS[*]}"
	echo ""
	echo "Please ensure the package is published before running test apps."
	exit 1
fi
