#!/usr/bin/env bash
set -euo pipefail

# Spikard PHP Benchmark App Verification Script
#
# Starts the server, tests each endpoint category, and validates responses.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8000}"
SERVER_PID=""
BASE_URL="http://localhost:${PORT}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
	echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
	echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
	echo -e "${RED}[ERROR]${NC} $*"
}

cleanup() {
	if [[ -n "${SERVER_PID}" ]] && kill -0 "${SERVER_PID}" 2>/dev/null; then
		log_info "Stopping server (PID ${SERVER_PID})..."
		kill "${SERVER_PID}" 2>/dev/null || true
		wait "${SERVER_PID}" 2>/dev/null || true
	fi
}

trap cleanup EXIT INT TERM

start_server() {
	log_info "Starting Spikard PHP server on port ${PORT}..."

	cd "${SCRIPT_DIR}"
	php server.php "${PORT}" >/dev/null 2>&1 &
	SERVER_PID=$!

	log_info "Server started (PID ${SERVER_PID})"

	# Wait for server to be ready
	log_info "Waiting for server to be ready..."
	for i in {1..30}; do
		if curl -s "${BASE_URL}/health" >/dev/null 2>&1; then
			log_info "Server is ready!"
			return 0
		fi
		sleep 0.5
	done

	log_error "Server failed to start within timeout"
	return 1
}

test_health_check() {
	log_info "Testing health check endpoint..."

	response=$(curl -s "${BASE_URL}/health")

	if echo "${response}" | grep -q '"status"'; then
		log_info "✓ Health check passed"
		return 0
	else
		log_error "✗ Health check failed: ${response}"
		return 1
	fi
}

test_json_small() {
	log_info "Testing small JSON payload..."

	payload='{"name":"Widget","price":9.99,"description":"A useful widget"}'
	response=$(curl -s -X POST "${BASE_URL}/items" \
		-H "Content-Type: application/json" \
		-d "${payload}")

	if echo "${response}" | grep -q '"name"'; then
		log_info "✓ Small JSON test passed"
		return 0
	else
		log_error "✗ Small JSON test failed: ${response}"
		return 1
	fi
}

test_json_nested() {
	log_info "Testing nested JSON payload..."

	payload='{"user":{"name":"John","email":"john@example.com"},"order":{"id":123,"total":99.99}}'
	response=$(curl -s -X POST "${BASE_URL}/payment" \
		-H "Content-Type: application/json" \
		-d "${payload}")

	if echo "${response}" | grep -q '"user"'; then
		log_info "✓ Nested JSON test passed"
		return 0
	else
		log_error "✗ Nested JSON test failed: ${response}"
		return 1
	fi
}

test_json_array() {
	log_info "Testing JSON array payload..."

	payload='{"items":[{"id":1,"name":"Item 1"},{"id":2,"name":"Item 2"}]}'
	response=$(curl -s -X POST "${BASE_URL}/items/list" \
		-H "Content-Type: application/json" \
		-d "${payload}")

	if echo "${response}" | grep -q '"items"'; then
		log_info "✓ JSON array test passed"
		return 0
	else
		log_error "✗ JSON array test failed: ${response}"
		return 1
	fi
}

test_path_params() {
	log_info "Testing path parameters..."

	payload='{"name":"Updated Widget"}'
	response=$(curl -s -X PATCH "${BASE_URL}/items/abc123" \
		-H "Content-Type: application/json" \
		-d "${payload}")

	if echo "${response}" | grep -q '"id"' && echo "${response}" | grep -q 'abc123'; then
		log_info "✓ Path parameters test passed"
		return 0
	else
		log_error "✗ Path parameters test failed: ${response}"
		return 1
	fi
}

test_url_encoded() {
	log_info "Testing URL-encoded form..."

	response=$(curl -s -X POST "${BASE_URL}/login/" \
		-H "Content-Type: application/x-www-form-urlencoded" \
		-d "username=testuser&password=testpass&remember=true")

	if echo "${response}" | grep -q 'username'; then
		log_info "✓ URL-encoded form test passed"
		return 0
	else
		log_error "✗ URL-encoded form test failed: ${response}"
		return 1
	fi
}

test_multipart() {
	log_info "Testing multipart form..."

	# Create temp file
	temp_file=$(mktemp)
	echo "test content" >"${temp_file}"

	response=$(curl -s -X POST "${BASE_URL}/files/upload" \
		-F "file=@${temp_file}")

	rm -f "${temp_file}"

	if echo "${response}" | grep -q '"files_received"'; then
		log_info "✓ Multipart form test passed"
		return 0
	else
		log_error "✗ Multipart form test failed: ${response}"
		return 1
	fi
}

test_large_payload() {
	log_info "Testing large JSON payload..."

	# Generate a larger payload (~10KB)
	items='[]'
	for i in {1..100}; do
		items="${items:0:-1},{\"id\":${i},\"name\":\"Item ${i}\",\"price\":${i}.99}]"
	done
	payload="{\"data\":${items}}"

	response=$(curl -s -X POST "${BASE_URL}/data" \
		-H "Content-Type: application/json" \
		-d "${payload}")

	if echo "${response}" | grep -q '"data"'; then
		log_info "✓ Large payload test passed"
		return 0
	else
		log_error "✗ Large payload test failed: ${response}"
		return 1
	fi
}

main() {
	log_info "Starting Spikard PHP benchmark app verification"
	log_info "================================================"

	if ! command -v curl >/dev/null 2>&1; then
		log_error "curl is required but not installed"
		exit 1
	fi

	if ! command -v php >/dev/null 2>&1; then
		log_error "php is required but not installed"
		exit 1
	fi

	# Check PHP version
	php_version=$(php -r "echo PHP_VERSION;")
	log_info "PHP version: ${php_version}"

	start_server || exit 1

	# Run tests
	failed=0
	test_health_check || ((failed++))
	test_json_small || ((failed++))
	test_json_nested || ((failed++))
	test_json_array || ((failed++))
	test_path_params || ((failed++))
	test_url_encoded || ((failed++))
	test_multipart || ((failed++))
	test_large_payload || ((failed++))

	log_info "================================================"
	if [[ ${failed} -eq 0 ]]; then
		log_info "All tests passed! ✓"
		return 0
	else
		log_error "${failed} test(s) failed ✗"
		return 1
	fi
}

main "$@"
