#!/usr/bin/env bash
# Comprehensive workload benchmark script
# Tests all implemented workloads: JSON, Path params, Query params, Multipart, URL-encoded

set -e

WORKSPACE_ROOT="/Users/naamanhirschfeld/workspace/spikard"
SPIKARD_RUST_SERVER="$WORKSPACE_ROOT/tools/benchmark-harness/apps/spikard-rust/target/release/spikard-rust-bench"
PYTHON_SERVER="$WORKSPACE_ROOT/tools/benchmark-harness/apps/spikard-python-workloads/server.py"

# Test configuration
DURATION="10s"
CONCURRENCY=50

# JSON payloads for testing
JSON_SMALL='{\"id\":12345,\"name\":\"test_item\",\"active\":true,\"count\":42,\"tags\":[\"tag1\",\"tag2\",\"tag3\"]}'

# Generate medium JSON (1-10KB)
JSON_MEDIUM=$(cat <<'EOF'
{
  "id": 12345,
  "name": "test_item_medium",
  "description": "This is a medium-sized JSON payload for benchmarking purposes. It contains significantly more data than the small payload to test performance with larger request bodies.",
  "active": true,
  "count": 42,
  "price": 99.99,
  "currency": "USD",
  "category": "electronics",
  "subcategory": "smartphones",
  "brand": "TestBrand",
  "model": "TestModel-X1000",
  "sku": "TB-X1000-BLK-256",
  "tags": ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10"],
  "attributes": {
    "color": "black",
    "storage": "256GB",
    "ram": "8GB",
    "screen_size": "6.5 inches",
    "battery": "5000mAh",
    "camera": "108MP"
  },
  "availability": {
    "in_stock": true,
    "quantity": 150,
    "warehouse": "US-WEST-1",
    "shipping_time": "2-3 business days"
  },
  "ratings": {
    "average": 4.5,
    "count": 1234,
    "distribution": {
      "5_star": 800,
      "4_star": 300,
      "3_star": 100,
      "2_star": 20,
      "1_star": 14
    }
  },
  "metadata": {
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-20T15:45:00Z",
    "version": 3,
    "source": "import-system",
    "verified": true
  }
}
EOF
)

# Generate large JSON (10-100KB) - expanded with arrays
generate_large_json() {
    python3 -c '
import json
data = {
    "id": 12345,
    "name": "large_test_item",
    "description": "Large JSON payload for stress testing" * 10,
    "items": [
        {
            "id": i,
            "name": f"item_{i}",
            "value": i * 100,
            "tags": [f"tag_{j}" for j in range(10)],
            "metadata": {"key": f"value_{i}", "index": i}
        }
        for i in range(100)
    ],
    "matrix": [[j * i for j in range(20)] for i in range(20)],
    "nested": {
        f"level_{i}": {
            "data": [k for k in range(50)],
            "meta": {"index": i}
        }
        for i in range(10)
    }
}
print(json.dumps(data))
'
}

# URL-encoded form data
URLENCODED_SIMPLE="name=John+Doe&email=john%40example.com&age=30&city=New+York"
URLENCODED_COMPLEX="name=John+Doe&email=john%40example.com&age=30&city=New+York&country=USA&phone=%2B1234567890&address=123+Main+St&zip=10001&interests=tech&interests=sports&interests=music&company=TestCorp&position=Engineer&department=Engineering&salary=100000&start_date=2020-01-15&active=true&verified=true&newsletter=true&terms=true"

echo "=== Comprehensive Workload Benchmark ==="
echo "Duration: $DURATION per test"
echo "Concurrency: $CONCURRENCY connections"
echo ""

# Create results directory
RESULTS_DIR="/tmp/benchmark-results"
mkdir -p "$RESULTS_DIR"

# Function to start server and wait for health
start_server() {
    local server_type=$1
    local port=$2
    local pid_var=$3

    if [ "$server_type" = "rust" ]; then
        echo "Starting Spikard-Rust server on port $port..."
        $SPIKARD_RUST_SERVER $port > /tmp/rust-server.log 2>&1 &
        eval "$pid_var=$!"
    else
        echo "Starting Spikard-Python server on port $port..."
        cd "$WORKSPACE_ROOT"
        PYTHONPATH=packages/python uv run python $PYTHON_SERVER $port > /tmp/python-server.log 2>&1 &
        eval "$pid_var=$!"
    fi

    sleep 3

    # Health check with retries
    for i in {1..10}; do
        if curl -sf http://localhost:$port/health > /dev/null 2>&1; then
            echo "✓ Server healthy on port $port"
            return 0
        fi
        sleep 1
    done

    echo "✗ Server failed to start on port $port"
    return 1
}

# Function to run benchmark
run_benchmark() {
    local name=$1
    local method=$2
    local url=$3
    local output_file=$4
    shift 4

    echo "  Testing: $name"
    oha -z "$DURATION" -c "$CONCURRENCY" \
        -m "$method" \
        --output-format json \
        "$@" \
        "$url" \
        > "$output_file" 2>/dev/null || echo "  ⚠ Benchmark failed for $name"
}

# Function to benchmark server
benchmark_server() {
    local server_type=$1
    local port=$2
    local base_url="http://localhost:$port"
    local prefix="${server_type}"

    echo ""
    if [ "$server_type" = "rust" ]; then
        echo "=== Benchmarking Spikard-Rust ==="
    else
        echo "=== Benchmarking Spikard-Python ==="
    fi
    echo ""

    # JSON Body Workloads
    echo "JSON Body Workloads:"
    run_benchmark "JSON Small" POST "$base_url/json/small" \
        "$RESULTS_DIR/${prefix}-json-small.json" \
        -H "Content-Type: application/json" \
        -d "$JSON_SMALL"

    run_benchmark "JSON Medium" POST "$base_url/json/medium" \
        "$RESULTS_DIR/${prefix}-json-medium.json" \
        -H "Content-Type: application/json" \
        -d "$JSON_MEDIUM"

    LARGE_JSON=$(generate_large_json)
    run_benchmark "JSON Large" POST "$base_url/json/large" \
        "$RESULTS_DIR/${prefix}-json-large.json" \
        -H "Content-Type: application/json" \
        -d "$LARGE_JSON"

    # Path Parameter Workloads
    echo ""
    echo "Path Parameter Workloads:"
    run_benchmark "Path Simple" GET "$base_url/path/simple/test123" \
        "$RESULTS_DIR/${prefix}-path-simple.json"

    run_benchmark "Path Multiple" GET "$base_url/path/multiple/user456/post789" \
        "$RESULTS_DIR/${prefix}-path-multiple.json"

    run_benchmark "Path Deep" GET "$base_url/path/deep/acme/engineering/backend/api/item123" \
        "$RESULTS_DIR/${prefix}-path-deep.json"

    run_benchmark "Path Int" GET "$base_url/path/int/42" \
        "$RESULTS_DIR/${prefix}-path-int.json"

    run_benchmark "Path UUID" GET "$base_url/path/uuid/550e8400-e29b-41d4-a716-446655440000" \
        "$RESULTS_DIR/${prefix}-path-uuid.json"

    run_benchmark "Path Date" GET "$base_url/path/date/2024-01-15" \
        "$RESULTS_DIR/${prefix}-path-date.json"

    # Query Parameter Workloads
    echo ""
    echo "Query Parameter Workloads:"
    run_benchmark "Query Few" GET "$base_url/query/few?q=search&page=1&limit=20" \
        "$RESULTS_DIR/${prefix}-query-few.json"

    run_benchmark "Query Medium" GET "$base_url/query/medium?category=electronics&tags=phone,smart&min_price=100&max_price=1000&sort=price&order=asc&page=1&limit=20" \
        "$RESULTS_DIR/${prefix}-query-medium.json"

    run_benchmark "Query Many" GET "$base_url/query/many?q=search&page=1&limit=20&sort=date&order=desc&filter=active&category=tech&subcategory=mobile&brand=test&min_price=0&max_price=999&rating=4&verified=true&in_stock=true&shipping=fast&color=blue" \
        "$RESULTS_DIR/${prefix}-query-many.json"

    # URL-Encoded Form Workloads
    echo ""
    echo "URL-Encoded Form Workloads:"
    run_benchmark "URL-Encoded Simple" POST "$base_url/urlencoded/simple" \
        "$RESULTS_DIR/${prefix}-urlencoded-simple.json" \
        -H "Content-Type: application/x-www-form-urlencoded" \
        -d "$URLENCODED_SIMPLE"

    run_benchmark "URL-Encoded Complex" POST "$base_url/urlencoded/complex" \
        "$RESULTS_DIR/${prefix}-urlencoded-complex.json" \
        -H "Content-Type: application/x-www-form-urlencoded" \
        -d "$URLENCODED_COMPLEX"

    # Multipart Form Workloads (placeholder - generates minimal data)
    echo ""
    echo "Multipart Form Workloads:"
    echo "  Testing: Multipart Small"
    echo "small file content" > /tmp/small.txt
    curl -X POST "$base_url/multipart/small" \
        -F "file=@/tmp/small.txt" \
        > /dev/null 2>&1 && echo "  ✓ Multipart Small endpoint responding" || echo "  ⚠ Multipart Small failed"

    echo "  Testing: Multipart Medium"
    dd if=/dev/zero of=/tmp/medium.bin bs=1024 count=10 2>/dev/null
    curl -X POST "$base_url/multipart/medium" \
        -F "file=@/tmp/medium.bin" \
        > /dev/null 2>&1 && echo "  ✓ Multipart Medium endpoint responding" || echo "  ⚠ Multipart Medium failed"

    echo "  Testing: Multipart Large"
    dd if=/dev/zero of=/tmp/large.bin bs=1024 count=100 2>/dev/null
    curl -X POST "$base_url/multipart/large" \
        -F "file=@/tmp/large.bin" \
        > /dev/null 2>&1 && echo "  ✓ Multipart Large endpoint responding" || echo "  ⚠ Multipart Large failed"

    echo ""
    if [ "$server_type" = "rust" ]; then
        echo "✓ Spikard-Rust benchmarks complete"
    else
        echo "✓ Spikard-Python benchmarks complete"
    fi
}

# Start Rust server and benchmark
if start_server "rust" 8100 RUST_PID; then
    benchmark_server "rust" 8100
    kill $RUST_PID 2>/dev/null || true
    sleep 2
else
    echo "Failed to start Rust server, skipping benchmarks"
    exit 1
fi

# Start Python server and benchmark
if start_server "python" 8200 PYTHON_PID; then
    benchmark_server "python" 8200
    kill $PYTHON_PID 2>/dev/null || true
    sleep 2
else
    echo "Failed to start Python server, skipping benchmarks"
    exit 1
fi

# Generate comparison report
echo ""
echo "=== Generating Comparison Report ==="
echo ""

# Function to extract RPS from result file
get_rps() {
    local file=$1
    if [ -f "$file" ]; then
        jq -r '.summary.requestsPerSec // "N/A"' "$file" 2>/dev/null || echo "N/A"
    else
        echo "N/A"
    fi
}

# Function to calculate ratio
calc_ratio() {
    local rust_rps=$1
    local python_rps=$2
    if [ "$rust_rps" != "N/A" ] && [ "$python_rps" != "N/A" ]; then
        echo "scale=2; $rust_rps / $python_rps" | bc
    else
        echo "N/A"
    fi
}

echo "Workload Comparison (Requests/sec):"
echo "======================================"
printf "%-25s | %-12s | %-12s | %-8s\n" "Workload" "Rust" "Python" "Ratio"
echo "----------------------------------------------------------------------"

# JSON workloads
RUST_JSON_SMALL=$(get_rps "$RESULTS_DIR/rust-json-small.json")
PYTHON_JSON_SMALL=$(get_rps "$RESULTS_DIR/python-json-small.json")
RATIO_JSON_SMALL=$(calc_ratio "$RUST_JSON_SMALL" "$PYTHON_JSON_SMALL")
printf "%-25s | %12s | %12s | %8s\n" "JSON Small" "$RUST_JSON_SMALL" "$PYTHON_JSON_SMALL" "${RATIO_JSON_SMALL}x"

RUST_JSON_MEDIUM=$(get_rps "$RESULTS_DIR/rust-json-medium.json")
PYTHON_JSON_MEDIUM=$(get_rps "$RESULTS_DIR/python-json-medium.json")
RATIO_JSON_MEDIUM=$(calc_ratio "$RUST_JSON_MEDIUM" "$PYTHON_JSON_MEDIUM")
printf "%-25s | %12s | %12s | %8s\n" "JSON Medium" "$RUST_JSON_MEDIUM" "$PYTHON_JSON_MEDIUM" "${RATIO_JSON_MEDIUM}x"

RUST_JSON_LARGE=$(get_rps "$RESULTS_DIR/rust-json-large.json")
PYTHON_JSON_LARGE=$(get_rps "$RESULTS_DIR/python-json-large.json")
RATIO_JSON_LARGE=$(calc_ratio "$RUST_JSON_LARGE" "$PYTHON_JSON_LARGE")
printf "%-25s | %12s | %12s | %8s\n" "JSON Large" "$RUST_JSON_LARGE" "$PYTHON_JSON_LARGE" "${RATIO_JSON_LARGE}x"

echo "----------------------------------------------------------------------"

# Path parameter workloads
RUST_PATH_SIMPLE=$(get_rps "$RESULTS_DIR/rust-path-simple.json")
PYTHON_PATH_SIMPLE=$(get_rps "$RESULTS_DIR/python-path-simple.json")
RATIO_PATH_SIMPLE=$(calc_ratio "$RUST_PATH_SIMPLE" "$PYTHON_PATH_SIMPLE")
printf "%-25s | %12s | %12s | %8s\n" "Path Simple" "$RUST_PATH_SIMPLE" "$PYTHON_PATH_SIMPLE" "${RATIO_PATH_SIMPLE}x"

RUST_PATH_MULTIPLE=$(get_rps "$RESULTS_DIR/rust-path-multiple.json")
PYTHON_PATH_MULTIPLE=$(get_rps "$RESULTS_DIR/python-path-multiple.json")
RATIO_PATH_MULTIPLE=$(calc_ratio "$RUST_PATH_MULTIPLE" "$PYTHON_PATH_MULTIPLE")
printf "%-25s | %12s | %12s | %8s\n" "Path Multiple" "$RUST_PATH_MULTIPLE" "$PYTHON_PATH_MULTIPLE" "${RATIO_PATH_MULTIPLE}x"

RUST_PATH_DEEP=$(get_rps "$RESULTS_DIR/rust-path-deep.json")
PYTHON_PATH_DEEP=$(get_rps "$RESULTS_DIR/python-path-deep.json")
RATIO_PATH_DEEP=$(calc_ratio "$RUST_PATH_DEEP" "$PYTHON_PATH_DEEP")
printf "%-25s | %12s | %12s | %8s\n" "Path Deep" "$RUST_PATH_DEEP" "$PYTHON_PATH_DEEP" "${RATIO_PATH_DEEP}x"

RUST_PATH_INT=$(get_rps "$RESULTS_DIR/rust-path-int.json")
PYTHON_PATH_INT=$(get_rps "$RESULTS_DIR/python-path-int.json")
RATIO_PATH_INT=$(calc_ratio "$RUST_PATH_INT" "$PYTHON_PATH_INT")
printf "%-25s | %12s | %12s | %8s\n" "Path Int" "$RUST_PATH_INT" "$PYTHON_PATH_INT" "${RATIO_PATH_INT}x"

RUST_PATH_UUID=$(get_rps "$RESULTS_DIR/rust-path-uuid.json")
PYTHON_PATH_UUID=$(get_rps "$RESULTS_DIR/python-path-uuid.json")
RATIO_PATH_UUID=$(calc_ratio "$RUST_PATH_UUID" "$PYTHON_PATH_UUID")
printf "%-25s | %12s | %12s | %8s\n" "Path UUID" "$RUST_PATH_UUID" "$PYTHON_PATH_UUID" "${RATIO_PATH_UUID}x"

RUST_PATH_DATE=$(get_rps "$RESULTS_DIR/rust-path-date.json")
PYTHON_PATH_DATE=$(get_rps "$RESULTS_DIR/python-path-date.json")
RATIO_PATH_DATE=$(calc_ratio "$RUST_PATH_DATE" "$PYTHON_PATH_DATE")
printf "%-25s | %12s | %12s | %8s\n" "Path Date" "$RUST_PATH_DATE" "$PYTHON_PATH_DATE" "${RATIO_PATH_DATE}x"

echo "----------------------------------------------------------------------"

# Query parameter workloads
RUST_QUERY_FEW=$(get_rps "$RESULTS_DIR/rust-query-few.json")
PYTHON_QUERY_FEW=$(get_rps "$RESULTS_DIR/python-query-few.json")
RATIO_QUERY_FEW=$(calc_ratio "$RUST_QUERY_FEW" "$PYTHON_QUERY_FEW")
printf "%-25s | %12s | %12s | %8s\n" "Query Few" "$RUST_QUERY_FEW" "$PYTHON_QUERY_FEW" "${RATIO_QUERY_FEW}x"

RUST_QUERY_MEDIUM=$(get_rps "$RESULTS_DIR/rust-query-medium.json")
PYTHON_QUERY_MEDIUM=$(get_rps "$RESULTS_DIR/python-query-medium.json")
RATIO_QUERY_MEDIUM=$(calc_ratio "$RUST_QUERY_MEDIUM" "$PYTHON_QUERY_MEDIUM")
printf "%-25s | %12s | %12s | %8s\n" "Query Medium" "$RUST_QUERY_MEDIUM" "$PYTHON_QUERY_MEDIUM" "${RATIO_QUERY_MEDIUM}x"

RUST_QUERY_MANY=$(get_rps "$RESULTS_DIR/rust-query-many.json")
PYTHON_QUERY_MANY=$(get_rps "$RESULTS_DIR/python-query-many.json")
RATIO_QUERY_MANY=$(calc_ratio "$RUST_QUERY_MANY" "$PYTHON_QUERY_MANY")
printf "%-25s | %12s | %12s | %8s\n" "Query Many" "$RUST_QUERY_MANY" "$PYTHON_QUERY_MANY" "${RATIO_QUERY_MANY}x"

echo "----------------------------------------------------------------------"

# URL-encoded workloads
RUST_URL_SIMPLE=$(get_rps "$RESULTS_DIR/rust-urlencoded-simple.json")
PYTHON_URL_SIMPLE=$(get_rps "$RESULTS_DIR/python-urlencoded-simple.json")
RATIO_URL_SIMPLE=$(calc_ratio "$RUST_URL_SIMPLE" "$PYTHON_URL_SIMPLE")
printf "%-25s | %12s | %12s | %8s\n" "URL-Encoded Simple" "$RUST_URL_SIMPLE" "$PYTHON_URL_SIMPLE" "${RATIO_URL_SIMPLE}x"

RUST_URL_COMPLEX=$(get_rps "$RESULTS_DIR/rust-urlencoded-complex.json")
PYTHON_URL_COMPLEX=$(get_rps "$RESULTS_DIR/python-urlencoded-complex.json")
RATIO_URL_COMPLEX=$(calc_ratio "$RUST_URL_COMPLEX" "$PYTHON_URL_COMPLEX")
printf "%-25s | %12s | %12s | %8s\n" "URL-Encoded Complex" "$RUST_URL_COMPLEX" "$PYTHON_URL_COMPLEX" "${RATIO_URL_COMPLEX}x"

echo "======================================"
echo ""
echo "Full results available in: $RESULTS_DIR"
echo ""
echo "✓ Comprehensive benchmark complete"
