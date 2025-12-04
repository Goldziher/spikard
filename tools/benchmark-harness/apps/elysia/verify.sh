#!/bin/bash
# Verification script for Elysia benchmark app

set -e

echo "=== Verifying Elysia Benchmark App ==="
echo

# Check if Bun is installed
if ! command -v bun &>/dev/null; then
	echo "Error: Bun is not installed"
	echo "Install from: https://bun.sh"
	echo "  curl -fsSL https://bun.sh/install | bash"
	exit 1
fi

echo "Bun version: $(bun --version)"
echo

# Install dependencies
echo "Installing dependencies..."
bun install
echo

# Start server in background
echo "Starting Elysia server on port 8299..."
timeout 5 bun run server.ts 8299 &
PID=$!
sleep 2

# Test endpoints
echo "Testing endpoints..."
echo

# Health check
echo -n "  GET /health: "
RESPONSE=$(curl -s http://127.0.0.1:8299/health || echo "FAILED")
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓"
else
	echo "✗ FAILED: $RESPONSE"
	kill $PID 2>/dev/null || true
	exit 1
fi

# Root endpoint
echo -n "  GET /: "
RESPONSE=$(curl -s http://127.0.0.1:8299/ || echo "FAILED")
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓"
else
	echo "✗ FAILED: $RESPONSE"
	kill $PID 2>/dev/null || true
	exit 1
fi

# Query parameters
echo -n "  GET /query/few?name=test: "
RESPONSE=$(curl -s "http://127.0.0.1:8299/query/few?name=test" || echo "FAILED")
if [[ "$RESPONSE" == *"test"* ]]; then
	echo "✓"
else
	echo "✗ FAILED: $RESPONSE"
	kill $PID 2>/dev/null || true
	exit 1
fi

# Path parameters
echo -n "  GET /path/simple/123: "
RESPONSE=$(curl -s http://127.0.0.1:8299/path/simple/123 || echo "FAILED")
if [[ "$RESPONSE" == *"123"* ]]; then
	echo "✓"
else
	echo "✗ FAILED: $RESPONSE"
	kill $PID 2>/dev/null || true
	exit 1
fi

# JSON payload
echo -n "  POST /json/small: "
RESPONSE=$(curl -s -X POST http://127.0.0.1:8299/json/small \
	-H "Content-Type: application/json" \
	-d '{"name":"Test","description":"A test product","price":99.99}' ||
	echo "FAILED")
if [[ "$RESPONSE" == *"Test"* ]]; then
	echo "✓"
else
	echo "✗ FAILED: $RESPONSE"
	kill $PID 2>/dev/null || true
	exit 1
fi

# Clean up
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true

echo
echo "=== Elysia App Verified Successfully ==="
