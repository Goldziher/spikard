# ADR 0012: gRPC Testing Infrastructure
**Status**: Accepted
**Date**: 2025-12-31

## Context

Spikard's gRPC/Protobuf implementation spans multiple layers:
1. **Rust Runtime** - Tonic-based server and routing (`spikard-http`)
2. **Code Generation** - Proto → language types (`spikard-cli`)
3. **FFI Bindings** - Python, TypeScript, Ruby, PHP, WASM (`spikard-{py,node,rb,php,wasm}`)
4. **Language Packages** - User-facing APIs in each language

Each layer needs comprehensive testing, but traditional unit tests alone cannot catch:
- **Integration failures** - Runtime + FFI + language code working together
- **Cross-language inconsistencies** - Different behavior across Python vs Ruby vs PHP
- **Code quality regressions** - Generated code fails type checking after changes
- **Edge cases** - Large payloads, Unicode, binary data, concurrent requests

The testing strategy must ensure **100% test passing** across all languages while maintaining high coverage (80% minimum, 85% for PHP).

## Decision

### Testing Architecture

**Five-Layer Testing Pyramid**:
```
┌─────────────────────────────────────────┐
│  E2E Integration Tests (Reserved)      │  ← Future: Full server/client tests
├─────────────────────────────────────────┤
│  Cross-Language Behavior Tests         │  ← 672+ tests across 5 languages
├─────────────────────────────────────────┤
│  FFI Binding Tests                     │  ← 103-144 tests per language
├─────────────────────────────────────────┤
│  Code Generation Quality Tests         │  ← Generated code passes linters
├─────────────────────────────────────────┤
│  Rust Unit Tests                       │  ← 275+ tests in runtime + codegen
└─────────────────────────────────────────┘
```

### Layer 1: Rust Unit Tests

**Location**: `crates/spikard-http/src/grpc/`, `crates/spikard-cli/src/codegen/protobuf/`

**Coverage**:
- gRPC server routing and handler dispatch
- Metadata conversion (MetadataMap ↔ HashMap)
- Status code mappings (all 17 codes)
- Proto3 schema parsing
- Type mapping logic
- Generator utilities (case conversion, escaping)

**Test Count**: 275+ tests
- `spikard-http`: 202 integration tests (gRPC server, metadata, error handling)
- `spikard-cli`: 69 codegen tests (spec parsing, generators)

**Example** (`crates/spikard-http/tests/grpc_server_integration.rs`):
```rust
#[tokio::test]
async fn test_grpc_unary_request_response() {
    let handler = MockGrpcHandler::new("mock.Service", b"response");
    let server = TestServer::with_grpc_handler(handler);

    let response = server.grpc_unary("mock.Service/Method", b"request").await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.payload, b"response");
}
```

### Layer 2: Code Generation Quality Tests

**Location**: `crates/spikard-cli/tests/protobuf_quality.rs`

**Strategy**: Fixture-driven quality validation
1. Load `.proto` schema from `testing_data/protobuf/`
2. Generate code for each language (Python, TypeScript, Ruby, PHP, Rust)
3. Run language-specific quality tools on generated code
4. Fail if any linter/formatter/type checker reports errors

**Quality Tools per Language**:
- **Python**: `mypy --strict`, `ruff check`, `ruff format --check`
- **TypeScript**: `tsc --noEmit`, `biome check`
- **Ruby**: `ruby -c`, `steep check`, `rubocop`
- **PHP**: `php -l`, `phpstan --level=max`, `php-cs-fixer`
- **Rust**: `cargo check`, `cargo clippy`

**Example**:
```rust
#[test]
fn test_generated_python_passes_mypy() {
    let proto = load_fixture("user_service.proto");
    let generated = generate_python(&proto);

    // Write to temp file
    let temp = NamedTempFile::new().unwrap();
    fs::write(&temp, generated).unwrap();

    // Run mypy --strict
    let output = Command::new("mypy")
        .args(&["--strict", temp.path().to_str().unwrap()])
        .output()
        .unwrap();

    assert!(output.status.success(), "mypy found errors: {}",
        String::from_utf8_lossy(&output.stderr));
}
```

### Layer 3: FFI Binding Tests

**Goal**: Verify FFI boundary correctness for each language

**Python** (`tests/test_grpc_python.py` - 103 tests):
- Request/Response creation with metadata
- Service registration and routing
- Binary payload handling (empty, large, Unicode)
- All 17 gRPC status codes
- Protobuf message integration (google-protobuf)
- Concurrent request handling
- Error propagation across FFI

**TypeScript** (`packages/node/src/grpc.spec.ts` - 84 tests):
- GrpcRequest/GrpcResponse creation
- Service handler registration
- Metadata handling (headers/trailers)
- Buffer payload handling
- Status code errors
- Promise-based async handling

**Ruby** (`packages/ruby/spec/grpc_spec.rb` - 144 tests):
- Request/Response lifecycle
- Metadata getters and setters
- Binary string payloads
- Service registry
- Streaming response enumerators
- All gRPC status codes with custom messages
- Deep nesting, large payloads, Unicode edge cases
- Thread safety

**PHP** (`packages/php/tests/Grpc*Test.php` - 66 tests across 5 files):
- GrpcRequest/GrpcResponse objects
- GrpcService routing
- GrpcFacade factory methods
- Integration scenarios
- Binary payload preservation
- Metadata immutability

**Test Pattern Example** (Ruby):
```ruby
RSpec.describe Spikard::Grpc::Response do
  it 'creates response with binary payload' do
    payload = "\x08\x01\x12\x04test".b
    response = described_class.new(payload: payload)

    expect(response.payload).to eq(payload)
    expect(response.payload.encoding).to eq(Encoding::BINARY)
  end

  it 'handles metadata with special characters' do
    response = described_class.new(payload: "test".b)
    response.metadata = {
      'x-custom' => 'value with spaces',
      'x-unicode' => '日本語'
    }

    expect(response.metadata['x-custom']).to eq('value with spaces')
    expect(response.metadata['x-unicode']).to eq('日本語')
  end
end
```

### Layer 4: Cross-Language Behavior Tests

**Goal**: Ensure consistent behavior across all language bindings

**Test Categories** (mirrored across all languages):

1. **Basic Messages** - Simple types, nested messages, repeated fields
2. **Status Codes** - All 17 gRPC codes with proper error messages
3. **Metadata** - Headers, trailers, case sensitivity
4. **Binary Data** - Null bytes, random data, large payloads (1MB+)
5. **Unicode** - Emoji, CJK characters, RTL text, zero-width characters
6. **Edge Cases** - Empty payloads, deeply nested structures, concurrent access
7. **Error Handling** - Exception mapping, validation errors

**Coverage Requirements**:
- **Minimum**: 80% line coverage (enforced in CI)
- **PHP**: 85% line coverage (strictest requirement)
- **Current Achievement**:
  - Python: 80%+ (103 tests)
  - TypeScript: 80%+ (84 tests)
  - Ruby: 80%+ (144 tests)
  - PHP: 85%+ (66 tests)
  - Rust: 80%+ (275+ tests)

### Layer 5: E2E Integration Tests (Future)

**Reserved for Future Implementation**:
- Full gRPC client/server communication
- Protobuf schema → server → client roundtrip
- Streaming RPC modes (server, client, bidirectional)
- Load testing with realistic traffic patterns

**Location**: `e2e/grpc/` (not yet implemented)

## Testing Infrastructure

### Test Fixtures

**Location**: `testing_data/protobuf/`

**Fixture Schema** (JSON):
```json
{
  "name": "Simple unary RPC - GetUser",
  "description": "Tests basic unary gRPC call",
  "protobuf": {
    "package": "example.v1",
    "messages": [
      {
        "name": "GetUserRequest",
        "fields": [
          {"name": "user_id", "type": "int32", "number": 1}
        ]
      },
      {
        "name": "User",
        "fields": [
          {"name": "id", "type": "int32", "number": 1},
          {"name": "name", "type": "string", "number": 2}
        ]
      }
    ],
    "services": [
      {
        "name": "UserService",
        "methods": [
          {
            "name": "GetUser",
            "input_type": "GetUserRequest",
            "output_type": "User",
            "client_streaming": false,
            "server_streaming": false
          }
        ]
      }
    ]
  },
  "handler": {
    "service": "example.v1.UserService",
    "method": "GetUser"
  },
  "request": {
    "metadata": {"authorization": "Bearer token"},
    "message": {"user_id": 123}
  },
  "expected_response": {
    "status_code": "OK",
    "message": {"id": 123, "name": "Alice"}
  }
}
```

**Fixture Categories** (planned):
1. Basic Messages (01-09)
2. Unary RPCs (10-19)
3. Server Streaming (20-29)
4. Client Streaming (30-39)
5. Bidirectional Streaming (40-49)
6. Error Handling (50-59)
7. Metadata (60-69)
8. Large Payloads (70-79)
9. Edge Cases (80-89)
10. Auth & Security (90-99)

### CI/CD Integration

**All tests run in GitHub Actions**:

**Rust** (`.github/workflows/ci-rust.yaml`):
```yaml
- name: Run workspace tests
  run: cargo test --workspace
```

**Python** (`.github/workflows/ci-python.yaml`):
```yaml
- name: Run pytest
  run: uv run pytest packages/python/tests/ -v
```

**TypeScript** (`.github/workflows/ci-node.yaml`):
```yaml
- name: Test TypeScript
  run: pnpm test
```

**Ruby** (`.github/workflows/ci-ruby.yaml`):
```yaml
- name: Run Ruby specs
  run: bundle exec rspec
```

**PHP** (`.github/workflows/ci-php.yaml`):
```yaml
- name: Run PHPUnit
  run: vendor/bin/phpunit
```

**Quality Validation** (`.github/workflows/ci-validate.yaml`):
- Linting and formatting for all languages
- Type checking (mypy, tsc, steep, phpstan)
- License compliance checks

### Coverage Reporting

**All languages upload to Codecov**:
- Python: `coverage.lcov`
- TypeScript: `coverage/lcov.info`
- Ruby: `coverage/lcov.info`
- PHP: `target/clover.xml`
- Rust: `target/tarpaulin/lcov.info`

**Coverage Gates**:
- Fail CI if coverage drops below threshold
- Exclude generated code from coverage metrics
- Track coverage trends over time

## Consequences

**Benefits**:
- **High Confidence**: 672+ tests catch regressions across all layers
- **Quality Enforcement**: Generated code passes strictest quality tools
- **Cross-Language Consistency**: Same test patterns ensure uniform behavior
- **Rapid Feedback**: CI runs all tests on every PR
- **Documentation**: Tests serve as usage examples

**Trade-offs**:
- **Test Maintenance**: 672+ tests require ongoing updates
- **CI Time**: Full test suite takes ~15-20 minutes
- **Fixture Complexity**: JSON fixtures can become verbose for complex schemas
- **Test Duplication**: Some patterns repeated across languages

**Test Execution Time**:
- Rust: ~2-3 minutes (parallel execution)
- Python: ~1 minute (pytest parallelization)
- TypeScript: ~10 seconds (Vitest)
- Ruby: ~0.5 seconds (RSpec)
- PHP: ~0.03 seconds (PHPUnit)

**Known Gaps**:
- **E2E Tests**: No full client/server integration tests
- **Performance Tests**: No load testing or benchmarking
- **Chaos Engineering**: No fault injection or resilience testing

**Future Improvements**:
1. Add streaming RPC test fixtures
2. Implement E2E integration tests with real gRPC clients
3. Add property-based testing (Hypothesis, fast-check)
4. Performance regression tests
5. Cross-language compatibility matrix tests

## References

- Rust tests: `crates/spikard-http/tests/grpc_*.rs`
- Python tests: `tests/test_grpc_python.py`
- TypeScript tests: `packages/node/src/grpc.spec.ts`
- Ruby tests: `packages/ruby/spec/grpc_spec.rb`
- PHP tests: `packages/php/tests/Grpc*Test.php`
- Quality tests: `crates/spikard-cli/tests/protobuf_quality.rs`
- Test fixtures: `testing_data/protobuf/`
- CI workflows: `.github/workflows/ci-*.yaml`
- Coverage config: `pyproject.toml`, `vitest.config.ts`, `.simplecov`, `phpunit.xml`, `tarpaulin.toml`
