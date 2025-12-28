#!/usr/bin/env bash
set -euo pipefail

INPUT_FILE="${1:-./benchmark-results/aggregated.json}"
OUTPUT_DIR="${2:-docs/assets/benchmarks}"

if [ ! -f "${INPUT_FILE}" ]; then
	echo "❌ Input file not found: ${INPUT_FILE}"
	echo "Run aggregate-results.sh first"
	exit 1
fi

# Ensure output directory exists
mkdir -p "${OUTPUT_DIR}"

# Install Python dependencies if needed
if ! python3 -c "import plotly" 2>/dev/null; then
	echo "📦 Installing Python dependencies..."
	pip install -q plotly pandas numpy
fi

# Generate charts
echo "📊 Generating visualization charts..."
./target/release/benchmark-harness visualize \
	--input "${INPUT_FILE}" \
	--output "${OUTPUT_DIR}" \
	--charts all

# Copy aggregated.json for download link
cp "${INPUT_FILE}" "${OUTPUT_DIR}/aggregated.json"

echo "✅ Charts generated in ${OUTPUT_DIR}"
ls -lh "${OUTPUT_DIR}"/*.html 2>/dev/null || echo "No HTML files generated"
