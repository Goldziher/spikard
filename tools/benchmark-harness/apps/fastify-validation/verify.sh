#!/bin/bash
# Verification script for Fastify benchmark server

set -e

echo "Starting Fastify server on port 9995..."
pnpm start 9995 >/tmp/fastify_verify.log 2>&1 &
SERVER_PID=$!
sleep 2

echo "Testing endpoints..."

# Test counters
PASSED=0
FAILED=0

test_endpoint() {
	local name=$1
	local method=$2
	local url=$3
	local data=$4
	local expected=$5

	echo -n "Testing $name... "

	if [ "$method" = "GET" ]; then
		response=$(curl -s "$url")
	else
		response=$(curl -s -X "$method" "$url" -H "Content-Type: application/json" -d "$data")
	fi

	if echo "$response" | grep -q "$expected"; then
		echo "✓ PASSED"
		((PASSED++))
	else
		echo "✗ FAILED"
		echo "  Expected: $expected"
		echo "  Got: $response"
		((FAILED++))
	fi
}

# Health checks
test_endpoint "Health" "GET" "http://localhost:9995/health" "" "status"
test_endpoint "Root" "GET" "http://localhost:9995/" "" "status"

# JSON endpoints
test_endpoint "JSON Small" "POST" "http://localhost:9995/json/small" \
	'{"name":"test","description":"desc","price":99.99}' "\"ok\":true"

test_endpoint "JSON Medium" "POST" "http://localhost:9995/json/medium" \
	'{"name":"John","email":"j@ex.com","age":30,"address":{"street":"123 St","city":"City","state":"ST","zip_code":"12345"},"tags":["a"]}' "\"ok\":true"

# Path parameters
test_endpoint "Path Simple" "GET" "http://localhost:9995/path/simple/123" "" "123"
test_endpoint "Path Multiple" "GET" "http://localhost:9995/path/multiple/user1/post2" "" "user1"
test_endpoint "Path Deep" "GET" "http://localhost:9995/path/deep/o/t/p/r/i" "" "\"org\":\"o\""
test_endpoint "Path Int" "GET" "http://localhost:9995/path/int/42" "" "42"
test_endpoint "Path UUID" "GET" "http://localhost:9995/path/uuid/550e8400-e29b-41d4-a716-446655440000" "" "550e8400"
test_endpoint "Path Date" "GET" "http://localhost:9995/path/date/2024-01-01" "" "2024-01-01"

# Query parameters
test_endpoint "Query Few" "GET" "http://localhost:9995/query/few?a=1&b=2" "" "\"ok\":true"
test_endpoint "Query Medium" "GET" "http://localhost:9995/query/medium?a=1&b=2&c=3" "" "\"ok\":true"
test_endpoint "Query Many" "GET" "http://localhost:9995/query/many?a=1&b=2&c=3&d=4&e=5&f=6" "" "\"ok\":true"

# URL encoded
test_endpoint "URL Encoded Simple" "POST" "http://localhost:9995/urlencoded/simple" \
	'{"username":"test","password":"pass"}' "\"ok\":true"

# Multipart
test_endpoint "Multipart Small" "POST" "http://localhost:9995/multipart/small" "" "files_received"
test_endpoint "Multipart Medium" "POST" "http://localhost:9995/multipart/medium" "" "10240"
test_endpoint "Multipart Large" "POST" "http://localhost:9995/multipart/large" "" "102400"

echo ""
echo "==================================="
echo "Results: $PASSED passed, $FAILED failed"
echo "==================================="

# Cleanup
kill $SERVER_PID 2>/dev/null || true

if [ $FAILED -eq 0 ]; then
	echo "✓ All tests passed!"
	exit 0
else
	echo "✗ Some tests failed"
	exit 1
fi
