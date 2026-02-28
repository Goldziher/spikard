#!/usr/bin/env bash
set -euo pipefail

WORKFLOW_RUN_ID="${1:-}"
RESULTS_DIR="${2:-./benchmark-results}"

# Get latest successful run if not provided
if [ -z "${WORKFLOW_RUN_ID}" ]; then
  echo "🔍 Finding latest successful benchmark run..."
  WORKFLOW_RUN_ID=$(gh run list \
    --workflow=comparative-benchmarks.yaml \
    --status=success \
    --limit=1 \
    --json databaseId \
    --jq='.[0].databaseId')

  if [ -z "${WORKFLOW_RUN_ID}" ]; then
    echo "❌ No successful runs found"
    exit 1
  fi
  echo "✅ Found run ID: ${WORKFLOW_RUN_ID}"
fi

# Build benchmark-harness if not exists
if [ ! -f "./target/release/benchmark-harness" ]; then
  echo "🔨 Building benchmark-harness..."
  cargo build --release -p benchmark-harness
fi

# Run aggregate command
echo "📥 Aggregating results from run ${WORKFLOW_RUN_ID}..."
./target/release/benchmark-harness aggregate \
  --run-id "${WORKFLOW_RUN_ID}" \
  --output "${RESULTS_DIR}/aggregated.json" \
  --download-dir "${RESULTS_DIR}/artifacts"

echo "✅ Aggregation complete: ${RESULTS_DIR}/aggregated.json"
