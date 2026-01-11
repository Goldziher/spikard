#!/bin/bash
#
# Run all gRPC fixture tests across all language bindings.
#
# This script:
# 1. Starts the Python gRPC test server in the background
# 2. Runs fixture tests for Python, Node.js, Ruby, and PHP
# 3. Collects results and stops the server
# 4. Exits with non-zero if any tests fail
#
# Usage:
#     ./scripts/run_all_grpc_tests.sh
#

set -e # Exit on error
set -o pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "========================================================================"
echo "gRPC Fixture Test Suite - All Language Bindings"
echo "========================================================================"
echo ""

# Change to project root
cd "$PROJECT_ROOT"

# Track test results
PYTHON_RESULT=0
NODE_RESULT=0
RUBY_RESULT=0
PHP_RESULT=0
TOTAL_TESTS=0
PASSED_TESTS=0

# Function to stop server on exit
# shellcheck disable=SC2329
cleanup() {
	if [ ! -z "$SERVER_PID" ]; then
		echo ""
		echo "Stopping gRPC server (PID: $SERVER_PID)..."
		kill "$SERVER_PID" 2>/dev/null || true
		wait "$SERVER_PID" 2>/dev/null || true
		echo "Server stopped."
	fi
}
trap cleanup EXIT

echo "Step 1: Starting Python gRPC test server..."
echo "--------------------------------------------------------------------"

# Start server in background
cd "$PROJECT_ROOT"
uv run --directory packages/python python "$PROJECT_ROOT/scripts/start_grpc_test_server.py" >/dev/null 2>&1 &
SERVER_PID=$!

# Wait for server to be ready
echo "Waiting for server to start (PID: $SERVER_PID)..."
sleep 3

# Check if server is still running
if ! ps -p $SERVER_PID >/dev/null; then
	echo -e "${RED}ERROR: Server failed to start${NC}"
	exit 1
fi

echo -e "${GREEN}✓ Server started successfully${NC}"
echo ""

# ==============================================================================
# Python Tests
# ==============================================================================

echo "Step 2: Running Python fixture tests..."
echo "--------------------------------------------------------------------"

cd "$PROJECT_ROOT/packages/python"

if uv run pytest tests/test_grpc_fixtures.py -v --tb=short 2>&1 | tee /tmp/python_tests.log; then
	PYTHON_PASSED=$(grep -c "PASSED" /tmp/python_tests.log || echo "0")
	echo -e "${GREEN}✓ Python: ${PYTHON_PASSED}/37 tests passed${NC}"
	PASSED_TESTS=$((PASSED_TESTS + PYTHON_PASSED))
	PYTHON_RESULT=0
else
	PYTHON_PASSED=$(grep -c "PASSED" /tmp/python_tests.log || echo "0")
	echo -e "${RED}✗ Python: ${PYTHON_PASSED}/37 tests passed${NC}"
	PYTHON_RESULT=1
fi

TOTAL_TESTS=$((TOTAL_TESTS + 37))
echo ""

# ==============================================================================
# Node.js Tests
# ==============================================================================

echo "Step 3: Running Node.js fixture tests..."
echo "--------------------------------------------------------------------"

cd "$PROJECT_ROOT/packages/node"

if pnpm test grpc_fixtures.spec.ts 2>&1 | tee /tmp/node_tests.log; then
	NODE_PASSED=$(grep -c "✓" /tmp/node_tests.log || echo "0")
	echo -e "${GREEN}✓ Node.js: ${NODE_PASSED}/37 tests passed${NC}"
	PASSED_TESTS=$((PASSED_TESTS + NODE_PASSED))
	NODE_RESULT=0
else
	NODE_PASSED=$(grep -c "✓" /tmp/node_tests.log || echo "0")
	echo -e "${RED}✗ Node.js: ${NODE_PASSED}/37 tests passed${NC}"
	NODE_RESULT=1
fi

TOTAL_TESTS=$((TOTAL_TESTS + 37))
echo ""

# ==============================================================================
# Ruby Tests
# ==============================================================================

echo "Step 4: Running Ruby fixture tests..."
echo "--------------------------------------------------------------------"

cd "$PROJECT_ROOT/packages/ruby"

if bundle exec rspec spec/grpc_fixtures_spec.rb --format documentation 2>&1 | tee /tmp/ruby_tests.log; then
	RUBY_PASSED=$(grep -c "examples, 0 failures" /tmp/ruby_tests.log || echo "0")
	if [ "$RUBY_PASSED" -gt 0 ]; then
		RUBY_PASSED=37 # All passed
	else
		RUBY_PASSED=$(grep -oP '\d+ examples?, 0 failures' /tmp/ruby_tests.log | grep -oP '^\d+' || echo "0")
	fi
	echo -e "${GREEN}✓ Ruby: ${RUBY_PASSED}/37 tests passed${NC}"
	PASSED_TESTS=$((PASSED_TESTS + RUBY_PASSED))
	RUBY_RESULT=0
else
	RUBY_PASSED=$(grep -oP '\d+ examples?, \d+ failures?' /tmp/ruby_tests.log | grep -oP '^\d+' || echo "0")
	RUBY_FAILURES=$(grep -oP '\d+ failures?' /tmp/ruby_tests.log | grep -oP '\d+' || echo "37")
	RUBY_PASSED=$((37 - RUBY_FAILURES))
	echo -e "${RED}✗ Ruby: ${RUBY_PASSED}/37 tests passed${NC}"
	RUBY_RESULT=1
fi

TOTAL_TESTS=$((TOTAL_TESTS + 37))
echo ""

# ==============================================================================
# PHP Tests
# ==============================================================================

echo "Step 5: Running PHP fixture tests..."
echo "--------------------------------------------------------------------"

cd "$PROJECT_ROOT/packages/php"

if vendor/bin/phpunit tests/GrpcFixturesTest.php --testdox 2>&1 | tee /tmp/php_tests.log; then
	PHP_PASSED=$(grep -c "✔" /tmp/php_tests.log || echo "0")
	echo -e "${GREEN}✓ PHP: ${PHP_PASSED}/37 tests passed${NC}"
	PASSED_TESTS=$((PASSED_TESTS + PHP_PASSED))
	PHP_RESULT=0
else
	PHP_PASSED=$(grep -c "✔" /tmp/php_tests.log || echo "0")
	echo -e "${RED}✗ PHP: ${PHP_PASSED}/37 tests passed${NC}"
	PHP_RESULT=1
fi

TOTAL_TESTS=$((TOTAL_TESTS + 37))
echo ""

# ==============================================================================
# Summary
# ==============================================================================

echo "========================================================================"
echo "Test Results Summary"
echo "========================================================================"
echo ""

if [ $PYTHON_RESULT -eq 0 ]; then
	echo -e "${GREEN}✓${NC} Python:  37/37 passed"
else
	echo -e "${RED}✗${NC} Python:  ${PYTHON_PASSED}/37 passed"
fi

if [ $NODE_RESULT -eq 0 ]; then
	echo -e "${GREEN}✓${NC} Node.js: 37/37 passed"
else
	echo -e "${RED}✗${NC} Node.js: ${NODE_PASSED}/37 passed"
fi

if [ $RUBY_RESULT -eq 0 ]; then
	echo -e "${GREEN}✓${NC} Ruby:    37/37 passed"
else
	echo -e "${RED}✗${NC} Ruby:    ${RUBY_PASSED}/37 passed"
fi

if [ $PHP_RESULT -eq 0 ]; then
	echo -e "${GREEN}✓${NC} PHP:     37/37 passed"
else
	echo -e "${RED}✗${NC} PHP:     ${PHP_PASSED}/37 passed"
fi

echo ""
echo "Total: ${PASSED_TESTS}/${TOTAL_TESTS} tests passed"
echo ""

# Determine overall result
TOTAL_RESULT=$((PYTHON_RESULT + NODE_RESULT + RUBY_RESULT + PHP_RESULT))

if [ $TOTAL_RESULT -eq 0 ]; then
	echo -e "${GREEN}✓ All tests passed!${NC}"
	exit 0
else
	echo -e "${RED}✗ Some tests failed${NC}"
	exit 1
fi
