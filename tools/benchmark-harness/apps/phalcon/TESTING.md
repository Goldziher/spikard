# Phalcon Benchmark Server - Testing Guide

## Overview

This guide covers testing the Phalcon benchmark server for correctness, performance, and compliance.

## Unit-Level Testing

### Code Syntax Validation

```bash
# Check PHP syntax
php -l server.php
php -l index.php

# More strict checks
php -d display_errors=On -l server.php
```

### Type Analysis

The codebase uses full type declarations:
- All function parameters typed
- All return types specified
- Nullable types where appropriate
- Array type hints with element types

```php
// Example of full typing:
public static function create(array $data): array { }
public static function get(int $id): ?array { }
public static function update(int $id, array $data): ?array { }
public static function delete(int $id): bool { }
```

## Integration Testing

### Automated Verification Script

```bash
./verify.sh
```

This script performs:
1. **Pre-flight checks**: PHP version, Phalcon extension, files
2. **Static analysis**: Syntax validation, strict types
3. **Server startup**: Verify server starts on port 8000
4. **Functional tests**: All API endpoints
5. **Error handling**: 404 responses, validation

### Manual Testing

#### 1. Health Endpoint

```bash
curl -v http://localhost:8000/health
```

Expected response:
- Status: 200 OK
- Content-Type: application/json
- Body: `{"status":"ok"}`

#### 2. Create User (POST)

```bash
curl -v -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com","age":28}'
```

Expected response:
- Status: 201 Created
- Body includes: id, name, email, age
- ID is auto-incremented integer

#### 3. Read User (GET)

```bash
curl -v http://localhost:8000/users/1
```

Expected response:
- Status: 200 OK
- Body matches the created user
- All fields preserved

#### 4. Update User (PUT)

```bash
curl -v -X PUT http://localhost:8000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"email":"newemail@example.com"}'
```

Expected response:
- Status: 200 OK
- Email field updated
- Other fields preserved (name, age unchanged)
- ID unchanged

#### 5. Delete User (DELETE)

```bash
curl -v -X DELETE http://localhost:8000/users/1
```

Expected response:
- Status: 200 OK
- Body: `{"message":"User deleted"}`

#### 6. Get Deleted User (404)

```bash
curl -v http://localhost:8000/users/1
```

Expected response:
- Status: 404 Not Found
- Body: `{"error":"User not found"}`

## Performance Testing

### Apache Bench

#### Setup

```bash
# Install Apache Bench (if not present)
# macOS: brew install httpd
# Linux: sudo apt-get install apache2-utils
# Windows: Download from https://httpd.apache.org/

# Prepare test data
cat > user.json << 'EOF'
{"name":"Benchmark User","email":"bench@example.com","age":25}
EOF
```

#### Warm-up Run

```bash
ab -n 100 -c 10 http://localhost:8000/health
```

This performs 100 requests with 10 concurrent connections.

#### Health Check Benchmark

```bash
ab -n 5000 -c 50 http://localhost:8000/health
```

Metrics to observe:
- **Requests/sec**: Framework throughput
- **Time/request**: Average latency
- **Connection Times**:
  - Connect: network latency
  - Processing: server processing
  - Waiting: time until response

#### POST Benchmark (User Creation)

```bash
ab -n 1000 -c 50 -p user.json -T application/json http://localhost:8000/users
```

Key metrics:
- **Throughput**: Requests per second
- **Latency**: Time per request
- **Processing time**: Server-side computation

#### GET Benchmark (User Retrieval)

```bash
ab -n 5000 -c 50 http://localhost:8000/users/1
```

#### PUT Benchmark (User Update)

```bash
# First create a user
curl -s -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","email":"test@example.com"}' | jq .id

# Benchmark updates (replace {id} with actual ID)
ab -n 1000 -c 50 -p user.json -T application/json -X PUT http://localhost:8000/users/1
```

## Stress Testing

### Load Generation with Siege

```bash
# Install Siege
# macOS: brew install siege
# Linux: sudo apt-get install siege

# Create URL file
cat > urls.txt << 'EOF'
http://localhost:8000/health
http://localhost:8000/users
http://localhost:8000/users/1
EOF

# Run stress test
siege -f urls.txt -c 100 -r 100 -b
```

### Load Generation with wrk

```bash
# Install wrk
# macOS: brew install wrk
# Linux: git clone https://github.com/wg/wrk && cd wrk && make

# Simple benchmark
wrk -t 4 -c 100 -d 30s http://localhost:8000/health

# With custom script
cat > script.lua << 'EOF'
request = function()
   wrk.method = "GET"
   wrk.path = "/users/1"
   return wrk.format(nil, nil)
end
EOF

wrk -t 4 -c 100 -d 30s -s script.lua http://localhost:8000/
```

## Memory Usage Testing

### Monitor Memory During Load

```bash
# Terminal 1: Start server with memory reporting
php -d memory_limit=256M server.php 8000

# Terminal 2: Monitor memory usage
watch -n 1 'ps aux | grep "server.php" | grep -v grep'

# Terminal 3: Generate load
ab -n 10000 -c 100 http://localhost:8000/health
```

Expected behavior:
- Memory grows slightly as users are created
- Memory stabilizes after load generation
- No memory leaks over time

## Error Case Testing

### Invalid JSON

```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{invalid json}'
```

Expected: 400 Bad Request

### Missing Required Fields

```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{}'
```

Expected: 201 Created with minimal user object

### Invalid User ID (Non-numeric)

```bash
curl http://localhost:8000/users/abc
```

Expected: 404 Not Found (ID coercion handles this)

### Non-existent User ID

```bash
curl http://localhost:8000/users/99999
```

Expected: 404 Not Found

### Empty Request Body

```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d ''
```

Expected: 400 Bad Request

## Comparison with Other Frameworks

### Running Comparative Benchmarks

```bash
# From repository root
cd /path/to/benchmark-harness

# Compare Phalcon with Express
ab -n 5000 -c 50 http://localhost:8000/health  # Phalcon
ab -n 5000 -c 50 http://localhost:3000/health  # Express (on port 3000)

# Generate comparison report
# Use tools like gnuplot to visualize
```

### Metrics to Compare

1. **Throughput** (requests/sec)
   - Framework overhead
   - Request handling efficiency

2. **Latency** (time per request)
   - Response time distribution
   - Percentiles (p50, p95, p99)

3. **Memory Usage** (MB)
   - Baseline framework memory
   - Memory per concurrent connection

4. **CPU Usage** (%)
   - Processing efficiency
   - Scaling characteristics

## Continuous Testing

### Automated Test Suite

```bash
#!/bin/bash
# test-all.sh

set -e

echo "Running Phalcon Benchmark Tests..."

# Syntax check
php -l server.php

# Dependency check
composer validate

# Run verification script
./verify.sh

# Run custom test suite
php -d display_errors=On -r '
  // Could add custom unit tests here
  echo "All tests passed!\n";
'
```

## Docker Testing

### Test in Isolated Container

```bash
# Build image
docker build -t phalcon-benchmark:test .

# Run container
docker run -p 8000:8000 phalcon-benchmark:test

# Test from host
curl http://localhost:8000/health
```

### Multi-container Testing

```bash
docker-compose up -d

# Wait for health
sleep 5

# Run tests
./verify.sh

# Check logs
docker-compose logs phalcon

# Cleanup
docker-compose down
```

## Regression Testing

### Creating a Test Suite

```php
<?php
// tests/HealthCheckTest.php
class HealthCheckTest {
    public function testHealthEndpoint() {
        $response = json_decode(
            file_get_contents('http://localhost:8000/health'),
            true
        );
        assert($response['status'] === 'ok');
    }
}
```

### Running Regression Tests

```bash
# Before code changes
./verify.sh > baseline.txt

# After code changes
./verify.sh > current.txt

# Compare
diff baseline.txt current.txt
```

## Monitoring in Production

### Application Performance Monitoring (APM)

Consider tools for production:
- New Relic
- Datadog
- Elastic APM
- Grafana + Prometheus

### Logging

Enable detailed logging:

```php
error_log('[phalcon] Request: ' . $_SERVER['REQUEST_METHOD'] . ' ' . $_SERVER['REQUEST_URI']);
error_log('[phalcon] Response: ' . $statusCode);
```

### Health Checks

Use `/health` endpoint for:
- Load balancer health monitoring
- Uptime monitoring
- Automated deployments

## Performance Optimization Tips

1. **Enable OPcache**: `opcache.enable=1` in php.ini
2. **Tune Memory Limit**: `memory_limit=128M` (usually sufficient)
3. **Use PHP 8.2+**: Better performance than 8.1
4. **Enable JIT**: PHP 8.0+ JIT compiler
5. **Tune Phalcon**: Review Phalcon configuration options

## Resources

- [Phalcon Testing Guide](https://phalcon.io/docs/testing)
- [Apache Bench Documentation](https://httpd.apache.org/docs/2.4/programs/ab.html)
- [Performance Testing Guide](https://en.wikipedia.org/wiki/Software_performance_testing)
- [Load Testing Tools Comparison](https://www.softwaretestinghelp.com/load-testing-tools/)
