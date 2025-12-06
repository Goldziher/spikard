#!/bin/bash
# Verification script for all benchmark apps

set -e

APPS_DIR="$(dirname "$0")"
cd "$APPS_DIR"

echo "=== Verifying Benchmark Apps ==="
echo

# Test FastAPI
echo "Testing FastAPI..."
cd fastapi
timeout 5 uv run python server.py 8201 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8201/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ FastAPI OK"
else
	echo "✗ FastAPI FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Litestar
echo "Testing Litestar..."
cd litestar
timeout 5 uv run python server.py 8207 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8207/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Litestar OK"
else
	echo "✗ Litestar FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Fastify
echo "Testing Fastify..."
cd fastify
timeout 5 node server.js 8202 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8202/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Fastify OK"
else
	echo "✗ Fastify FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Express
echo "Testing Express..."
cd express
timeout 5 node server.js 8203 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8203/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Express OK"
else
	echo "✗ Express FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Hono
echo "Testing Hono..."
cd hono
timeout 5 node server.js 8204 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8204/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Hono OK"
else
	echo "✗ Hono FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Roda
echo "Testing Roda..."
cd roda
timeout 5 ruby server.rb 8205 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8205/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Roda OK"
else
	echo "✗ Roda FAILED: $RESPONSE"
	exit 1
fi
cd ..

# Test Hanami API
echo "Testing Hanami API..."
cd hanami-api
timeout 5 ruby server.rb 8206 &
PID=$!
sleep 2
RESPONSE=$(curl -s http://127.0.0.1:8206/health || echo "FAILED")
kill $PID 2>/dev/null || true
wait $PID 2>/dev/null || true
if [[ "$RESPONSE" == *"ok"* ]]; then
	echo "✓ Hanami API OK"
else
	echo "✗ Hanami API FAILED: $RESPONSE"
	exit 1
fi
cd ..

echo
echo "=== All Apps Verified Successfully ==="
