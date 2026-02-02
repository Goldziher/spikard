# gRPC Fixture Testing Framework

## Overview

Fixture-driven testing is the canonical approach for validating gRPC streaming functionality across all language bindings in Spikard. Rather than writing separate tests for each language, we define fixtures once in JSON and execute them identically across Python, TypeScript, Ruby, and PHP.

### Why Fixture-Driven Testing?

1. **Single Source of Truth**: Behavior is defined once in fixtures; all languages must pass identical tests
2. **Cross-Language Parity**: Enforces consistent behavior across Python, TypeScript, Ruby, and PHP bindings
3. **Maintainability**: New test cases added once; automatically tested in all languages
4. **Reproducibility**: Fixtures are deterministic JSON; no flaky random test generation
5. **Documentation**: Fixtures serve as executable specifications for expected behavior

Fixtures are stored in `testing_data/protobuf/streaming/` organized by streaming category.

---

## Architecture: 5-Layer Testing Pyramid

Spikard follows a rigorous 5-layer testing pyramid with fixtures as the authoritative layer:

```
     Layer 5: E2E Integration Tests (Multi-Language)
              ↑ All 33+ fixtures run in Python, TypeScript, Ruby, PHP
     Layer 4: Cross-Language Behavior Tests ← fixtures drive these
              ↑ Each language tests identical fixture scenarios
     Layer 3: FFI Binding Tests
              ↑ Language-specific FFI testing (PyO3, NAPI-RS, Magnus, ext-php-rs)
     Layer 2: Code Generation Quality Tests
              ↑ Protobuf code generation validation
     Layer 1: Rust Unit Tests (2,062 tests)
              ↑ Core gRPC streaming implementation
```

**Key Design Principles**:
- Layer 1 (Rust) is comprehensive (95%+ coverage); layers 2-5 validate bindings, not logic
- Fixtures connect Layer 4 to Layer 5; all languages must pass identically
- Fixture validation (schema check) is the gate before any language-specific testing
- Coverage targets: Rust 95%, all other languages 80%+ (PHP 85%+)

---

## Fixture Structure

Each fixture is a JSON file conforming to `testing_data/protobuf/streaming/schema.json`.

### Required Fields

```json
{
  "name": "Server streaming - empty stream",
  "description": "Tests server streaming RPC that returns an empty stream...",
  "category": "server_streaming",
  "protobuf": { /* See Protobuf Definition */ },
  "handler": { /* See Handler Configuration */ },
  "request": { /* See Request Section */ },
  "expected_response": { /* See Expected Response */ },
  "tags": ["server_streaming", "empty", "edge_cases"]
}
```

### name
Human-readable test case identifier. Used in test output and parametrization IDs.

**Example**: `"Server streaming - empty stream"`

### description
Detailed explanation of what the test validates. Should describe the behavior and why it matters.

**Example**:
```
"Tests server streaming RPC that returns an empty stream.
 The server opens the stream but sends no messages before completing successfully."
```

### category
Classification of the streaming pattern being tested.

**Valid Values**: `server_streaming`, `client_streaming`, `bidirectional_streaming`, `error`

### protobuf
Complete protobuf schema including messages and service definitions.

**Structure**:
```json
{
  "package": "example.v1",
  "messages": [
    {
      "name": "EmptyStreamRequest",
      "fields": [
        {
          "name": "request_id",
          "type": "string",
          "number": 1,
          "label": "required"
        }
      ]
    },
    {
      "name": "EmptyStreamResponse",
      "fields": [
        {
          "name": "id",
          "type": "int32",
          "number": 1,
          "label": "required"
        },
        {
          "name": "value",
          "type": "string",
          "number": 2,
          "label": "required"
        }
      ]
    }
  ],
  "services": [
    {
      "name": "StreamService",
      "methods": [
        {
          "name": "GetEmptyStream",
          "input_type": "EmptyStreamRequest",
          "output_type": "EmptyStreamResponse",
          "client_streaming": false,
          "server_streaming": true
        }
      ]
    }
  ]
}
```

### handler
Configuration specifying which service/method the fixture tests.

**Fields**:
- `service`: Fully qualified service name (package + service)
- `method`: RPC method name
- `timeout_ms` (optional): Timeout in milliseconds

**Example**:
```json
{
  "service": "example.v1.StreamService",
  "method": "GetEmptyStream",
  "timeout_ms": 5000
}
```

### request
Input data for the RPC call. Structure varies by streaming mode.

**For Unary/Server Streaming** (single message):
```json
{
  "metadata": {
    "content-type": "application/grpc",
    "authorization": "Bearer token123"
  },
  "message": {
    "request_id": "empty-stream-001"
  }
}
```

**For Client Streaming** (stream of messages):
```json
{
  "metadata": {
    "content-type": "application/grpc"
  },
  "stream": [
    {"value": 10},
    {"value": 20},
    {"value": 30}
  ]
}
```

**For Large Streams** (auto-generated):
```json
{
  "metadata": {},
  "stream_generator": "sequential counter from 1 to N",
  "stream_size": 1000,
  "stream_note": "Each message contains index and value field"
}
```

### expected_response
Expected outcome of the RPC call.

**For Successful Unary/Client Streaming** (single response):
```json
{
  "status_code": "OK",
  "message": {
    "total": 60
  }
}
```

**For Successful Server/Bidirectional Streaming** (stream):
```json
{
  "status_code": "OK",
  "stream": [
    {"id": 1, "value": "item_1"},
    {"id": 2, "value": "item_2"}
  ]
}
```

**For Error Cases**:
```json
{
  "status_code": "INVALID_ARGUMENT",
  "error": {
    "code": "INVALID_ARGUMENT",
    "message": "Request validation failed"
  }
}
```

### tags
Array of strings for test categorization and filtering.

**Common Tags**: `server_streaming`, `client_streaming`, `bidirectional_streaming`, `error`, `empty`, `large_payload`, `metadata`, `unicode`, `edge_cases`, `performance`

**Example**: `["server_streaming", "empty", "edge_cases"]`

---

## Adding New Fixtures

### Step-by-Step Guide

#### 1. Determine Category and Location

First, identify which streaming pattern and error type:
- **Server Streaming**: `/testing_data/protobuf/streaming/server/`
- **Client Streaming**: `/testing_data/protobuf/streaming/client/`
- **Bidirectional**: `/testing_data/protobuf/streaming/bidirectional/`
- **Error Cases**: `/testing_data/protobuf/streaming/errors/` (if separate)

#### 2. Create JSON File

Name the file descriptively: `{number}_{category}_{scenario}.json`

**Numbering Convention**:
- Server: 20-29
- Client: 30-39
- Bidirectional: 40-49
- Errors: 50-99

**Example**: `streaming/server/25_stream_unicode_characters.json`

#### 3. Write Fixture Content

Use this template:

```json
{
  "name": "Server streaming - [scenario name]",
  "description": "[What is being tested and why it matters]",
  "category": "server_streaming",
  "protobuf": {
    "package": "example.v1",
    "messages": [
      {
        "name": "[RequestType]",
        "fields": [
          {
            "name": "[field_name]",
            "type": "[field_type]",
            "number": 1,
            "label": "[required|optional|repeated]"
          }
        ]
      },
      {
        "name": "[ResponseType]",
        "fields": [
          {
            "name": "[field_name]",
            "type": "[field_type]",
            "number": 1,
            "label": "[required|optional|repeated]"
          }
        ]
      }
    ],
    "services": [
      {
        "name": "StreamService",
        "methods": [
          {
            "name": "[MethodName]",
            "input_type": "[RequestType]",
            "output_type": "[ResponseType]",
            "client_streaming": false,
            "server_streaming": true
          }
        ]
      }
    ]
  },
  "handler": {
    "service": "example.v1.StreamService",
    "method": "[MethodName]"
  },
  "request": {
    "metadata": {
      "content-type": "application/grpc"
    },
    "message": {
      "field_name": "value"
    }
  },
  "expected_response": {
    "status_code": "OK",
    "stream": [
      {"field": "value1"},
      {"field": "value2"}
    ]
  },
  "tags": ["server_streaming", "scenario_category"]
}
```

#### 4. Validate Fixture

Run validation to ensure JSON conforms to schema:

```bash
task validate:fixtures
```

Expected output:
```
✓ All fixtures valid
```

#### 5. Run Tests in All Languages

Once fixture is added, tests automatically discover it:

```bash
# Run all languages
task test:grpc:fixtures

# Or individually
task test:grpc:python
task test:grpc:typescript
task test:grpc:ruby
task test:grpc:php
```

### Complete Example: Bidirectional Echo

```json
{
  "name": "Bidirectional streaming - echo conversation",
  "description": "Tests bidirectional streaming RPC that echoes all received messages. Server responds to each client message with an identical copy, demonstrating full-duplex communication.",
  "category": "bidirectional_streaming",
  "protobuf": {
    "package": "example.v1",
    "messages": [
      {
        "name": "EchoMessage",
        "fields": [
          {
            "name": "id",
            "type": "int32",
            "number": 1,
            "label": "required"
          },
          {
            "name": "text",
            "type": "string",
            "number": 2,
            "label": "required"
          }
        ]
      }
    ],
    "services": [
      {
        "name": "EchoService",
        "methods": [
          {
            "name": "Echo",
            "input_type": "EchoMessage",
            "output_type": "EchoMessage",
            "client_streaming": true,
            "server_streaming": true
          }
        ]
      }
    ]
  },
  "handler": {
    "service": "example.v1.EchoService",
    "method": "Echo"
  },
  "request": {
    "metadata": {
      "content-type": "application/grpc"
    },
    "stream": [
      {"id": 1, "text": "hello"},
      {"id": 2, "text": "world"},
      {"id": 3, "text": "test"}
    ]
  },
  "expected_response": {
    "status_code": "OK",
    "stream": [
      {"id": 1, "text": "hello"},
      {"id": 2, "text": "world"},
      {"id": 3, "text": "test"}
    ]
  },
  "tags": ["bidirectional_streaming", "echo", "conversation"]
}
```

---

## Running Tests

### All Languages

```bash
task test:grpc:fixtures
```

Runs parametrized tests in Python, TypeScript, Ruby, and PHP simultaneously.

### Individual Languages

```bash
# Python
task test:grpc:python

# TypeScript
task test:grpc:typescript

# Ruby
task test:grpc:ruby

# PHP
task test:grpc:php
```

### Specific Test Categories

```bash
# Python - server streaming only
cd packages/python
pytest tests/test_grpc_fixtures.py::test_server_streaming_fixture -v

# TypeScript - error cases only
cd packages/typescript
pnpm vitest run tests/grpc_fixtures.spec.ts -t "error"

# Ruby - bidirectional only
cd packages/ruby
bundle exec rspec spec/grpc_fixtures_spec.rb --pattern "*bidirectional*"

# PHP - client streaming only
cd packages/php
composer run test -- --filter "ClientStreaming"
```

### With Coverage Reports

```bash
# Python
cd packages/python
pytest tests/test_grpc_fixtures.py --cov=spikard_py --cov-report=html

# TypeScript
cd packages/typescript
pnpm vitest run --coverage

# Ruby
cd packages/ruby
bundle exec rspec --require simplecov

# PHP
cd packages/php
composer run test -- --coverage-html coverage
```

---

## Stream Generators

For large or procedurally-generated streams, use the `stream_generator` field instead of hardcoding messages.

### Supported Generators

#### Sequential Counter

```json
{
  "stream_generator": "sequential counter from 1 to N",
  "stream_size": 100
}
```

Generates `{"index": 0, "value": "message_0"}`, `{"index": 1, "value": "message_1"}`, ...

#### Random Values

```json
{
  "stream_generator": "random values 0-1000",
  "stream_size": 50
}
```

Generates `{"index": i, "random_value": <0-1000>}` for each message.

#### Timestamp Sequence

```json
{
  "stream_generator": "timestamp for each message",
  "stream_size": 25
}
```

Generates `{"index": i, "timestamp": <unix_time>}`.

### Using Generators in Fixtures

```json
{
  "name": "Client streaming - large batch of 1000 messages",
  "category": "client_streaming",
  "request": {
    "metadata": {},
    "stream_generator": "sequential counter",
    "stream_size": 1000,
    "stream_note": "Messages 0-999, each with index and value fields"
  },
  "expected_response": {
    "status_code": "OK",
    "message": {
      "total_received": 1000,
      "sum": 499500
    }
  }
}
```

The test framework (in `test_grpc_fixtures.py`) automatically expands generators before executing the test.

---

## Metadata & Timeouts

### Request Metadata

gRPC metadata (headers) are specified in the `request.metadata` field:

```json
{
  "request": {
    "metadata": {
      "authorization": "Bearer eyJhbGciOiJIUzI1NiIs...",
      "x-request-id": "550e8400-e29b-41d4-a716-446655440000",
      "x-api-version": "v1",
      "custom-header": "custom-value"
    },
    "message": {
      "data": "test"
    }
  }
}
```

All metadata is passed to the gRPC client and sent with the request.

### Timeouts

Specify RPC timeout in milliseconds via `handler.timeout_ms`:

```json
{
  "handler": {
    "service": "example.v1.StreamService",
    "method": "SlowMethod",
    "timeout_ms": 10000
  }
}
```

Test framework converts to seconds before passing to client:
```python
timeout = timeout_ms / 1000 if timeout_ms else None
await client.execute_server_streaming(..., timeout=timeout)
```

### Combining Metadata and Timeout

```json
{
  "handler": {
    "service": "example.v1.AuthService",
    "method": "AuthenticatedStream",
    "timeout_ms": 5000
  },
  "request": {
    "metadata": {
      "authorization": "Bearer token"
    },
    "message": {"user_id": "123"}
  }
}
```

---

## Error Testing

Error fixtures validate that the server properly returns gRPC error codes and messages.

### Error Fixture Structure

```json
{
  "name": "Server streaming - validation error",
  "description": "Tests that invalid input is rejected with INVALID_ARGUMENT status",
  "category": "error",
  "protobuf": { /* ... */ },
  "handler": {
    "service": "example.v1.StreamService",
    "method": "GetStream"
  },
  "request": {
    "metadata": {},
    "message": {
      "request_id": ""
    }
  },
  "expected_response": {
    "status_code": "INVALID_ARGUMENT",
    "error": {
      "code": "INVALID_ARGUMENT",
      "message": "request_id cannot be empty"
    }
  },
  "tags": ["error", "validation"]
}
```

### gRPC Error Codes

Common status codes used in error fixtures:

| Code | Meaning |
|------|---------|
| `OK` | Success (0) |
| `CANCELLED` | Cancelled (1) |
| `UNKNOWN` | Unknown error (2) |
| `INVALID_ARGUMENT` | Invalid argument (3) |
| `DEADLINE_EXCEEDED` | Deadline exceeded (4) |
| `NOT_FOUND` | Not found (5) |
| `ALREADY_EXISTS` | Already exists (6) |
| `PERMISSION_DENIED` | Permission denied (7) |
| `RESOURCE_EXHAUSTED` | Resource exhausted (8) |
| `FAILED_PRECONDITION` | Failed precondition (9) |
| `ABORTED` | Aborted (10) |
| `OUT_OF_RANGE` | Out of range (11) |
| `UNIMPLEMENTED` | Unimplemented (12) |
| `INTERNAL` | Internal error (13) |
| `UNAVAILABLE` | Unavailable (14) |
| `DATA_LOSS` | Data loss (15) |
| `UNAUTHENTICATED` | Unauthenticated (16) |

### Partial Stream Delivery

For errors that occur mid-stream, indicate how many messages were delivered:

```json
{
  "name": "Server streaming - error mid-stream",
  "description": "Error occurs after sending some messages",
  "expected_response": {
    "status_code": "INTERNAL",
    "error": {
      "code": "INTERNAL",
      "message": "Processing error"
    },
    "partial_delivery": true,
    "note": "Client receives 5 messages before error occurs"
  }
}
```

---

## Cross-Language Parity

### The Parity Requirement

All language bindings must pass **identical fixtures**. If a fixture passes in Python but fails in TypeScript, this indicates an implementation divergence that must be fixed.

### Ensuring Parity

1. **Run all languages in same test suite**:
   ```bash
   task test:grpc:fixtures
   ```

2. **Verify identical results**:
   ```
   Python: 33 tests passed
   TypeScript: 33 tests passed
   Ruby: 33 tests passed
   PHP: 33 tests passed
   ```

3. **Investigate divergences immediately**:
   - Language-specific behavior (e.g., type coercion) is acceptable if documented
   - Logic differences in streaming behavior are unacceptable
   - Document parity exceptions in fixture `tags` field

### Example Parity Variance

Acceptable (documented in fixture):
```json
{
  "tags": ["server_streaming", "parity:type_coercion"],
  "note": "Python returns int64 as string; documented in binding API"
}
```

Unacceptable (must be fixed):
```json
{
  "expected_response": {
    "stream": [{"count": 5}]
  }
  // Python returns [{"count": 5}, {"count": 5}] - different number of messages
}
```

---

## Coverage Requirements

### By Language

| Language | Target | Measurement | Command |
|----------|--------|-------------|---------|
| Python | 80%+ | Line coverage | `pytest --cov=spikard_py --cov-fail-under=80` |
| TypeScript | 80%+ | Statement coverage | `pnpm vitest --coverage --coverage.thresholds.lines=80` |
| Ruby | 80%+ | Line coverage | Bundle with SimpleCov; check `.resultset.json` |
| PHP | 85%+ | Line coverage | `phpunit --coverage-text` |

### Verification Script

Run all coverage checks automatically:

```bash
task verify:coverage
```

Expected output:
```
Coverage Results:
  ✓ PASS Python: 80%+
  ✓ PASS TypeScript: 80%+
  ✓ PASS Ruby: 80%+
  ✓ PASS PHP: 85%+
```

### Coverage Gap Analysis

If coverage is below threshold:

1. **Identify uncovered lines**:
   ```bash
   cd packages/python
   pytest --cov=spikard_py --cov-report=html
   open htmlcov/index.html  # View uncovered lines
   ```

2. **Add fixtures to cover gaps**:
   - New error scenarios
   - Edge cases (empty streams, large payloads, unicode)
   - Metadata and timeout handling
   - Streaming mode combinations

3. **Re-run coverage verification**:
   ```bash
   task verify:coverage
   ```

---

## CI Integration

### Automated Fixture Validation

On every push to `main` or PR:

1. **Validate all fixtures** against schema:
   ```yaml
   validate-fixtures:
     runs-on: ubuntu-latest
     steps:
       - run: python scripts/validate_fixtures.py
   ```

2. **Run fixture tests** in each language:
   ```yaml
   test-python:
     runs-on: ubuntu-latest
     needs: validate-fixtures
     steps:
       - run: task test:grpc:python
       - run: pytest --cov --cov-fail-under=80
   ```

3. **Enforce coverage thresholds**:
   ```yaml
   coverage:
     runs-on: ubuntu-latest
     needs: [test-python, test-typescript, test-ruby, test-php]
     steps:
       - run: task verify:coverage
   ```

### CI Triggering

Fixtures tests run on:
- Push to `main` branch
- Pull requests modifying:
  - `crates/spikard-http/src/grpc/**`
  - `testing_data/protobuf/streaming/**`
  - `packages/*/tests/**grpc**`

### Local CI Simulation

Run CI checks locally before pushing:

```bash
# Validate fixtures
task validate:fixtures

# Run all language tests
task test:grpc:fixtures

# Verify coverage
task verify:coverage

# Full suite (as CI will run)
task test
```

---

## Troubleshooting

### Fixture Validation Fails

**Problem**: `task validate:fixtures` fails with schema error.

**Solution**:
1. Check fixture JSON syntax: `jq < testing_data/protobuf/streaming/server/20_*.json`
2. Verify required fields: `name`, `description`, `category`, `protobuf`, `handler`, `request`, `expected_response`, `tags`
3. Ensure category is valid: `server_streaming`, `client_streaming`, `bidirectional_streaming`, `error`
4. Validate against schema: `python scripts/validate_fixtures.py`

### Tests Pass Locally but Fail in CI

**Problem**: Fixture tests work on macOS but fail on Linux CI.

**Solution**:
1. Check for OS-specific paths or line endings
2. Verify timezone-dependent code (if using timestamps)
3. Run CI locally: `act` (GitHub Actions locally)
4. Check gRPC server address hardcoding (should be `localhost:50051` or from env)

### Coverage Below Threshold

**Problem**: Python tests report 75% coverage; need 80%.

**Solution**:
1. Identify uncovered lines: `pytest --cov-report=html`
2. Add fixtures for uncovered scenarios:
   - Error cases (CANCELLED, INTERNAL, etc.)
   - Edge cases (empty streams, very large messages)
   - Metadata handling (custom headers)
   - Timeout scenarios
3. Re-run: `pytest --cov --cov-fail-under=80`

### Timeout Errors in Tests

**Problem**: `pytest` reports timeout expired mid-stream.

**Solution**:
1. Increase fixture timeout: `"timeout_ms": 30000` (30 seconds)
2. Check gRPC server is running and healthy
3. Verify no blocking operations in handler
4. Check system resources (disk space, memory)

### Cross-Language Divergence

**Problem**: Python passes fixture but TypeScript fails same fixture.

**Solution**:
1. Check error message; identify which language is incorrect
2. Debug in failing language:
   ```typescript
   # TypeScript
   pnpm vitest run tests/grpc_fixtures.spec.ts --reporter=verbose
   ```
3. Compare against Python behavior:
   ```bash
   # Python
   pytest tests/test_grpc_fixtures.py::test_server_streaming_fixture -vv
   ```
4. Review binding code in failing language (PyO3 vs NAPI-RS vs Magnus vs ext-php-rs)
5. File issue if binding is incorrect; document if intentional divergence

### Stream Generator Not Expanding

**Problem**: Fixture with `stream_generator` fails; generator not invoked.

**Solution**:
1. Verify generator syntax in fixture: `"stream_generator": "sequential counter from 1 to N"`
2. Check `stream_size` is specified: `"stream_size": 100`
3. Ensure generator is recognized in test client:
   - `sequential` / `counter` → indexed messages
   - `random` → random values
   - `timestamp` → timestamps
4. Debug generator expansion:
   ```python
   from test_grpc_fixtures import generate_stream
   stream = generate_stream("sequential counter", 10)
   print(stream)  # Verify 10 messages generated
   ```

---

## Related Documentation

- **ADR 0003**: [Validation and Fixtures](../adr/0003-validation-and-fixtures.md) - Fixtures as source of truth
- **ADR 0012**: [gRPC Testing Infrastructure](../adr/0012-grpc-testing-infrastructure.md) - gRPC testing architecture

---

## Quick Reference

### Create New Fixture
```bash
cat > testing_data/protobuf/streaming/{category}/{number}_{name}.json << 'EOF'
{
  "name": "...",
  "description": "...",
  "category": "server_streaming",
  ...
}
EOF
task validate:fixtures
```

### Run All Tests
```bash
task test:grpc:fixtures
```

### Check Coverage
```bash
task verify:coverage
```

### Debug Specific Fixture
```bash
# Python
pytest tests/test_grpc_fixtures.py -k "fixture_name" -vv

# TypeScript
pnpm vitest run -t "fixture_name"
```

### Add Stream Generator
```json
{
  "stream_generator": "sequential counter",
  "stream_size": 1000
}
```

### Add Error Case
```json
{
  "expected_response": {
    "status_code": "INVALID_ARGUMENT",
    "error": {
      "code": "INVALID_ARGUMENT",
      "message": "validation failed"
    }
  }
}
```
