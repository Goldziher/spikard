#!/usr/bin/env bash
# Quick workload benchmark test script

set -e

WORKSPACE_ROOT="/Users/naamanhirschfeld/workspace/spikard"
SPIKARD_RUST_SERVER="$WORKSPACE_ROOT/tools/benchmark-harness/apps/spikard-rust/target/release/spikard-rust-bench"
AXUM_SERVER="$WORKSPACE_ROOT/tools/benchmark-harness/apps/axum-baseline/target/release/spikard-rust-bench"
PYTHON_SERVER="$WORKSPACE_ROOT/tools/benchmark-harness/apps/spikard-python-workloads/server.py"

# JSON payload for testing
JSON_SMALL='{"id":12345,"name":"test_item","active":true,"count":42,"tags":["tag1","tag2","tag3"]}'

echo "=== Workload Benchmark Test ==="
echo ""

# Test Spikard-Rust server
echo "Starting Spikard-Rust server on port 8100..."
$SPIKARD_RUST_SERVER 8100 >/tmp/spikard-rust-server.log 2>&1 &
SPIKARD_RUST_PID=$!
sleep 2

echo "Testing Spikard-Rust server health..."
if curl -sf http://localhost:8100/health >/dev/null; then
	echo "✓ Spikard-Rust server healthy"
else
	echo "✗ Spikard-Rust server failed to start"
	kill $SPIKARD_RUST_PID 2>/dev/null || true
	exit 1
fi

echo "Running benchmark against Spikard-Rust server..."
oha -z 10s -c 50 \
	-m POST \
	-H "Content-Type: application/json" \
	-d "$JSON_SMALL" \
	--output-format json \
	http://localhost:8100/json/small \
	>/tmp/spikard-rust-bench.json

echo "Spikard-Rust results:"
cat /tmp/spikard-rust-bench.json | jq '.summary'

kill $SPIKARD_RUST_PID 2>/dev/null || true
sleep 1

echo ""
echo "Starting Python server on port 8200..."
cd $WORKSPACE_ROOT
PYTHONPATH=packages/python uv run python $PYTHON_SERVER 8200 >/tmp/python-server.log 2>&1 &
PYTHON_PID=$!
sleep 3

echo "Testing Python server health..."
if curl -sf http://localhost:8200/health >/dev/null; then
	echo "✓ Python server healthy"
else
	echo "✗ Python server failed to start"
	kill $PYTHON_PID 2>/dev/null || true
	exit 1
fi

echo "Running benchmark against Python server..."
oha -z 10s -c 50 \
	-m POST \
	-H "Content-Type: application/json" \
	-d "$JSON_SMALL" \
	--output-format json \
	http://localhost:8200/json/small \
	>/tmp/python-bench.json

echo "Python results:"
cat /tmp/python-bench.json | jq '.summary'

kill $PYTHON_PID 2>/dev/null || true

echo ""
echo "=== Comparison ==="
echo -n "Spikard-Rust RPS: "
cat /tmp/spikard-rust-bench.json | jq -r '.summary.requestsPerSec'
echo -n "Python RPS:       "
cat /tmp/python-bench.json | jq -r '.summary.requestsPerSec'

SPIKARD_RUST_RPS=$(cat /tmp/spikard-rust-bench.json | jq -r '.summary.requestsPerSec')
PYTHON_RPS=$(cat /tmp/python-bench.json | jq -r '.summary.requestsPerSec')
RATIO=$(echo "scale=2; $SPIKARD_RUST_RPS / $PYTHON_RPS" | bc)
echo "Ratio: ${RATIO}x (Spikard-Rust/Python)"

echo ""
echo "Full results saved to:"
echo "  - /tmp/spikard-rust-bench.json"
echo "  - /tmp/python-bench.json"
