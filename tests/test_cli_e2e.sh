#!/usr/bin/env bash
# End-to-end test for unified Spikard CLI
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "🧪 Spikard CLI End-to-End Test"
echo "================================"

# Check if CLI is built
if [ ! -f "target/release/spikard" ]; then
    echo "❌ CLI not found. Building with Python support..."
    cargo build --release -p spikard-cli --features python
fi

# Test 1: Features command
echo -e "\n📋 Test 1: Features command"
output=$(target/release/spikard features)
if echo "$output" | grep -q "Python: ✓"; then
    echo -e "${GREEN}✓${NC} Python support detected"
else
    echo -e "${RED}✗${NC} Python support not found"
    exit 1
fi

# Test 2: Create a simple test app
echo -e "\n📋 Test 2: Create test app"
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

cat > "$TEST_DIR/test_app.py" << 'EOF'
from spikard import Spikard, get, post

app = Spikard()

@get("/")
async def root():
    return {"message": "Hello from unified CLI!"}

@get("/health")
async def health():
    return {"status": "healthy"}

@post("/echo")
async def echo():
    return {"echo": "received"}
EOF

echo -e "${GREEN}✓${NC} Test app created at $TEST_DIR/test_app.py"

# Test 3: Run server in background
echo -e "\n📋 Test 3: Start server"
PORT=18765
target/release/spikard run "$TEST_DIR/test_app.py" --port $PORT > "$TEST_DIR/server.log" 2>&1 &
SERVER_PID=$!

# Trap to kill server on exit
trap "kill $SERVER_PID 2>/dev/null || true; rm -rf $TEST_DIR" EXIT

# Wait for server to be ready
echo "Waiting for server to start..."
MAX_WAIT=10
for i in $(seq 1 $MAX_WAIT); do
    if curl -s "http://localhost:$PORT/health" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} Server started on port $PORT"
        break
    fi
    if [ $i -eq $MAX_WAIT ]; then
        echo -e "${RED}✗${NC} Server did not start within ${MAX_WAIT}s"
        echo "Server log:"
        cat "$TEST_DIR/server.log"
        exit 1
    fi
    sleep 1
done

# Test 4: Test GET endpoint
echo -e "\n📋 Test 4: GET /"
response=$(curl -s "http://localhost:$PORT/")
if echo "$response" | grep -q "Hello from unified CLI"; then
    echo -e "${GREEN}✓${NC} GET / works: $response"
else
    echo -e "${RED}✗${NC} GET / failed: $response"
    exit 1
fi

# Test 5: Test health endpoint
echo -e "\n📋 Test 5: GET /health"
response=$(curl -s "http://localhost:$PORT/health")
if echo "$response" | grep -q "healthy"; then
    echo -e "${GREEN}✓${NC} GET /health works: $response"
else
    echo -e "${RED}✗${NC} GET /health failed: $response"
    exit 1
fi

# Test 6: Test POST endpoint
echo -e "\n📋 Test 6: POST /echo"
response=$(curl -s -X POST "http://localhost:$PORT/echo" \
    -H "Content-Type: application/json" \
    -d '{"test": "data"}')
if echo "$response" | grep -q "received"; then
    echo -e "${GREEN}✓${NC} POST /echo works: $response"
else
    echo -e "${RED}✗${NC} POST /echo failed: $response"
    exit 1
fi

# Clean up
kill $SERVER_PID 2>/dev/null || true

echo -e "\n${GREEN}✅ All tests passed!${NC}"
