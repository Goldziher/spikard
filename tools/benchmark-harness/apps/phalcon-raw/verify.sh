#!/bin/bash

###############################################################################
# Phalcon Benchmark Server Verification Script
#
# This script verifies that the Phalcon benchmark server meets all requirements
# and that the API endpoints function correctly.
###############################################################################

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="${SCRIPT_DIR}"
PORT="${PORT:-8000}"
BASE_URL="http://localhost:${PORT}"
STARTUP_WAIT=3

# Counters
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

###############################################################################
# Helper Functions
###############################################################################

log_info() {
	echo -e "${GREEN}✓${NC} $1"
}

log_error() {
	echo -e "${RED}✗${NC} $1"
}

log_warn() {
	echo -e "${YELLOW}⚠${NC} $1"
}

log_section() {
	echo ""
	echo "================================================================================"
	echo "  $1"
	echo "================================================================================"
}

###############################################################################
# Pre-flight Checks
###############################################################################

log_section "Pre-flight Checks"

# Check PHP version
PHP_VERSION=$(php -v | head -n 1)
if [[ $PHP_VERSION == *"8.2"* ]] || [[ $PHP_VERSION == *"8.3"* ]] || [[ $PHP_VERSION == *"8.4"* ]]; then
	log_info "PHP version meets requirements: ${PHP_VERSION%% *}"
	((TESTS_PASSED++))
else
	log_error "PHP 8.2+ required, found: ${PHP_VERSION%% *}"
	((TESTS_FAILED++))
	exit 1
fi

# Check Phalcon extension
if php -m | grep -q phalcon; then
	PHALCON_VERSION=$(php -r "echo phpversion('phalcon');" 2>/dev/null || echo "unknown")
	log_info "Phalcon extension is loaded (version: ${PHALCON_VERSION})"
	((TESTS_PASSED++))
else
	log_error "Phalcon extension not found"
	((TESTS_FAILED++))
	log_warn "Install Phalcon: https://phalcon.io/docs/install"
	exit 1
fi

# Check Composer dependencies
if [ ! -d "${APP_DIR}/vendor" ]; then
	log_warn "Composer dependencies not installed, running 'composer install'..."
	cd "${APP_DIR}"
	composer install --quiet
fi
log_info "Composer dependencies present"
((TESTS_PASSED++))

# Check required files
for file in server.php index.php composer.json; do
	if [ -f "${APP_DIR}/${file}" ]; then
		log_info "Required file present: ${file}"
		((TESTS_PASSED++))
	else
		log_error "Required file missing: ${file}"
		((TESTS_FAILED++))
		exit 1
	fi
done

###############################################################################
# Static Analysis
###############################################################################

log_section "Static Analysis"

# Check PHP syntax
if php -l "${APP_DIR}/server.php" >/dev/null 2>&1; then
	log_info "server.php syntax is valid"
	((TESTS_PASSED++))
else
	log_error "server.php has syntax errors"
	((TESTS_FAILED++))
fi

if php -l "${APP_DIR}/index.php" >/dev/null 2>&1; then
	log_info "index.php syntax is valid"
	((TESTS_PASSED++))
else
	log_error "index.php has syntax errors"
	((TESTS_FAILED++))
fi

# Check for strict_types declaration
if grep -q "declare(strict_types=1)" "${APP_DIR}/server.php"; then
	log_info "server.php declares strict_types=1"
	((TESTS_PASSED++))
else
	log_error "server.php missing strict_types declaration"
	((TESTS_FAILED++))
fi

###############################################################################
# Start Server
###############################################################################

log_section "Server Startup"

# Kill any existing process on the port
if lsof -Pi ":${PORT}" -sTCP:LISTEN -t >/dev/null 2>&1; then
	log_warn "Port ${PORT} already in use, attempting to free it..."
	kill -9 "$(lsof -Pi ":${PORT}" -sTCP:LISTEN -t)" || true
	sleep 1
fi

# Start PHP built-in server in background
log_info "Starting Phalcon server on port ${PORT}..."
cd "${APP_DIR}"
php -S "0.0.0.0:${PORT}" -t "${APP_DIR}" >/tmp/phalcon-benchmark.log 2>&1 &
SERVER_PID=$!
sleep ${STARTUP_WAIT}

# Check if server started successfully
if ! kill -0 ${SERVER_PID} 2>/dev/null; then
	log_error "Server failed to start (PID: ${SERVER_PID})"
	cat /tmp/phalcon-benchmark.log
	((TESTS_FAILED++))
	exit 1
fi

log_info "Server started successfully (PID: ${SERVER_PID})"
((TESTS_PASSED++))

# Set trap to kill server on exit
trap 'kill "$SERVER_PID" 2>/dev/null || true' EXIT

###############################################################################
# Functional Tests
###############################################################################

log_section "Functional Tests"

# Test health check
log_info "Testing GET /health..."
if RESPONSE=$(curl -s -w "\n%{http_code}" -X GET "${BASE_URL}/health" --connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "200" ] && echo "${BODY}" | grep -q '"status"'; then
		log_info "Health check returns 200 with valid JSON"
		((TESTS_PASSED++))
	else
		log_error "Health check failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
	fi
else
	log_error "Failed to connect to server at ${BASE_URL}"
	((TESTS_FAILED++))
fi

# Test user creation
log_info "Testing POST /users..."
USER_DATA='{"name":"John Doe","email":"john@example.com","age":30}'
if RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "${BASE_URL}/users" \
	-H "Content-Type: application/json" \
	-d "${USER_DATA}" \
	--connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "201" ] && echo "${BODY}" | grep -q '"id"'; then
		USER_ID=$(echo "${BODY}" | grep -o '"id":[0-9]*' | grep -o '[0-9]*')
		log_info "User creation returns 201 with ID: ${USER_ID}"
		((TESTS_PASSED++))
	else
		log_error "User creation failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
		USER_ID="1"
	fi
else
	log_error "Failed to create user"
	((TESTS_FAILED++))
	USER_ID="1"
fi

# Test user retrieval
log_info "Testing GET /users/{id}..."
if RESPONSE=$(curl -s -w "\n%{http_code}" -X GET "${BASE_URL}/users/${USER_ID}" \
	--connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "200" ] && echo "${BODY}" | grep -q '"id"'; then
		log_info "User retrieval returns 200 with valid JSON"
		((TESTS_PASSED++))
	else
		log_error "User retrieval failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
	fi
else
	log_error "Failed to retrieve user"
	((TESTS_FAILED++))
fi

# Test user update
log_info "Testing PUT /users/{id}..."
UPDATE_DATA='{"name":"Jane Doe","email":"jane@example.com"}'
if RESPONSE=$(curl -s -w "\n%{http_code}" -X PUT "${BASE_URL}/users/${USER_ID}" \
	-H "Content-Type: application/json" \
	-d "${UPDATE_DATA}" \
	--connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "200" ] && echo "${BODY}" | grep -q '"name"'; then
		log_info "User update returns 200 with updated data"
		((TESTS_PASSED++))
	else
		log_error "User update failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
	fi
else
	log_error "Failed to update user"
	((TESTS_FAILED++))
fi

# Test user deletion
log_info "Testing DELETE /users/{id}..."
if RESPONSE=$(curl -s -w "\n%{http_code}" -X DELETE "${BASE_URL}/users/${USER_ID}" \
	--connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "200" ] && echo "${BODY}" | grep -q '"message"'; then
		log_info "User deletion returns 200 with confirmation"
		((TESTS_PASSED++))
	else
		log_error "User deletion failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
	fi
else
	log_error "Failed to delete user"
	((TESTS_FAILED++))
fi

# Test 404 response
log_info "Testing 404 error handling..."
if RESPONSE=$(curl -s -w "\n%{http_code}" -X GET "${BASE_URL}/users/9999" \
	--connect-timeout 5 2>/dev/null); then
	HTTP_CODE=$(echo "${RESPONSE}" | tail -n 1)
	BODY=$(echo "${RESPONSE}" | head -n -1)
	if [ "${HTTP_CODE}" = "404" ] && echo "${BODY}" | grep -q '"error"'; then
		log_info "404 response is correct"
		((TESTS_PASSED++))
	else
		log_error "404 handling failed: HTTP ${HTTP_CODE}, Body: ${BODY}"
		((TESTS_FAILED++))
	fi
else
	log_error "Failed to test 404 response"
	((TESTS_FAILED++))
fi

###############################################################################
# Results Summary
###############################################################################

log_section "Test Results Summary"

TOTAL=$((TESTS_PASSED + TESTS_FAILED + TESTS_SKIPPED))
echo "Total Tests:  ${TOTAL}"
echo -e "Passed:       ${GREEN}${TESTS_PASSED}${NC}"
if [ ${TESTS_FAILED} -gt 0 ]; then
	echo -e "Failed:       ${RED}${TESTS_FAILED}${NC}"
else
	echo "Failed:       ${TESTS_FAILED}"
fi

if [ ${TESTS_SKIPPED} -gt 0 ]; then
	echo -e "Skipped:      ${YELLOW}${TESTS_SKIPPED}${NC}"
fi

echo ""

if [ ${TESTS_FAILED} -eq 0 ]; then
	log_info "All tests passed!"
	exit 0
else
	log_error "${TESTS_FAILED} test(s) failed"
	exit 1
fi
