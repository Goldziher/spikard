#!/bin/bash

# Trongate Benchmark App Verification Script

BASE_URL="http://localhost:8000"
TIMEOUT=5

echo "Trongate Benchmark Server Verification"
echo "======================================"
echo ""
echo "Testing endpoints..."

passed=0
failed=0

# Test 1: Health check
echo -n "GET /health ... "
if curl -s --max-time $TIMEOUT "$BASE_URL/health" 2>/dev/null | grep -q "status"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 2: POST user
echo -n "POST /users ... "
if curl -s -X POST -H "Content-Type: application/json" -d '{"name":"test"}' --max-time $TIMEOUT "$BASE_URL/users" 2>/dev/null | grep -q "id"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 3: GET user
echo -n "GET /users/1 ... "
if curl -s --max-time $TIMEOUT "$BASE_URL/users/1" 2>/dev/null | grep -q "name"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 4: PUT user
echo -n "PUT /users/1 ... "
if curl -s -X PUT -H "Content-Type: application/json" -d '{"name":"updated"}' --max-time $TIMEOUT "$BASE_URL/users/1" 2>/dev/null | grep -q "updated"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 5: DELETE user
echo -n "DELETE /users/1 ... "
if curl -s -X DELETE --max-time $TIMEOUT "$BASE_URL/users/1" 2>/dev/null; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 6: Echo endpoint
echo -n "POST /items ... "
if curl -s -X POST -H "Content-Type: application/json" -d '{"test":1}' --max-time $TIMEOUT "$BASE_URL/items" 2>/dev/null | grep -q "test"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 7: Nested JSON
echo -n "POST /items/nested ... "
if curl -s -X POST -H "Content-Type: application/json" -d '{"data":{"nested":true}}' --max-time $TIMEOUT "$BASE_URL/items/nested" 2>/dev/null | grep -q "nested"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Test 8: Form endpoint
echo -n "POST /login/ ... "
if curl -s -X POST -H "Content-Type: application/json" -d '{"username":"test"}' --max-time $TIMEOUT "$BASE_URL/login/" 2>/dev/null | grep -q "username"; then
	echo "OK"
	passed=$((passed + 1))
else
	echo "FAIL"
	failed=$((failed + 1))
fi

# Results
echo ""
echo "======================================"
echo "Results: $passed passed, $failed failed"
echo ""

if [ $failed -eq 0 ]; then
	echo "All tests passed!"
	exit 0
else
	echo "Some tests failed"
	exit 1
fi
