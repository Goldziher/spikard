#!/bin/bash
# Quick test script for roda-raw server

set -e

PORT=8001
SERVER_PID=""

cleanup() {
	if [ -n "$SERVER_PID" ]; then
		echo "Stopping server (PID: $SERVER_PID)..."
		kill $SERVER_PID 2>/dev/null || true
		wait $SERVER_PID 2>/dev/null || true
	fi
}

trap cleanup EXIT

echo "=== Roda-Raw Server Test ==="
echo

# Check if dependencies are installed
if ! bundle check &>/dev/null; then
	echo "Installing dependencies..."
	bundle install
	echo
fi

# Start server in background
echo "Starting server on port $PORT..."
ruby server.rb $PORT &
SERVER_PID=$!

# Wait for server to be ready
sleep 2

echo "Server started (PID: $SERVER_PID)"
echo

# Test endpoints
echo "Testing endpoints..."
echo

# Test 1: Health check
echo "1. GET / (health check)"
curl -s http://localhost:$PORT/ | jq .
echo

# Test 2: Health endpoint
echo "2. GET /health"
curl -s http://localhost:$PORT/health | jq .
echo

# Test 3: Small JSON
echo "3. POST /json/small"
curl -s -X POST http://localhost:$PORT/json/small \
	-H "Content-Type: application/json" \
	-d '{"name":"test","description":"test item","price":9.99,"tax":0.5}' | jq .
echo

# Test 4: Medium JSON
echo "4. POST /json/medium"
curl -s -X POST http://localhost:$PORT/json/medium \
	-H "Content-Type: application/json" \
	-d '{"user_id":123,"username":"testuser","email":"test@example.com","is_active":true,"address":{"street":"123 Main","city":"NYC","state":"NY","zip_code":"10001"},"tags":["tag1","tag2"]}' | jq .
echo

# Test 5: Path parameter
echo "5. GET /path/simple/123"
curl -s http://localhost:$PORT/path/simple/123 | jq .
echo

# Test 6: Multiple path parameters
echo "6. GET /path/multiple/user123/post456"
curl -s http://localhost:$PORT/path/multiple/user123/post456 | jq .
echo

# Test 7: Query parameters
echo "7. GET /query/few?q=test&page=1&limit=10"
curl -s "http://localhost:$PORT/query/few?q=test&page=1&limit=10" | jq .
echo

# Test 8: URL-encoded form
echo "8. POST /urlencoded/simple"
curl -s -X POST http://localhost:$PORT/urlencoded/simple \
	-H "Content-Type: application/x-www-form-urlencoded" \
	-d "name=John+Doe&email=john@example.com" | jq .
echo

# Test 9: Multipart (static response)
echo "9. POST /multipart/small"
curl -s -X POST http://localhost:$PORT/multipart/small \
	-F "file=@server.rb" | jq .
echo

echo "=== All tests completed! ==="
echo
echo "Note: This is a NO-VALIDATION server."
echo "It accepts any JSON input and echoes it back."
echo "Try sending invalid data - it will still work!"
echo

# Test with invalid data
echo "10. POST /json/small (INVALID DATA - should still work)"
curl -s -X POST http://localhost:$PORT/json/small \
	-H "Content-Type: application/json" \
	-d '{"completely":"wrong","fields":true,"price":"not-a-number"}' | jq .
echo

echo "Success! Invalid data was accepted (no validation)."
