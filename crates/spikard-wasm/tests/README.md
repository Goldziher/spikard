# WASM FFI Test Suite

Comprehensive Rust-level tests for the wasm-bindgen FFI binding in `spikard-wasm`.

## Test Files

### 1. `wasm_bindgen_tests.rs`

**Target**: `wasm32` architecture (WASM environment)

**Scope**: Complete FFI testing in browser/WASM runtime

**Tests**: 26+ comprehensive tests covering:
- Module initialization and panic hook setup
- TestClient construction and validation
- HTTP method handlers (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, TRACE)
- Configuration deserialization
- Route definition parsing
- Error handling across FFI boundary
- Memory safety and type conversions
- Lifecycle hooks integration
- Streaming response handling

**Running WASM Tests**:
```bash
# Headless Firefox (automated)
wasm-pack test --headless --firefox crates/spikard-wasm

# Headless Chrome (automated)
wasm-pack test --headless --chrome crates/spikard-wasm

# Interactive browser testing
wasm-pack test --firefox crates/spikard-wasm
wasm-pack test --chrome crates/spikard-wasm

# With verbose output
wasm-pack test --headless --firefox crates/spikard-wasm -- --nocapture
```

### 2. `host_ffi_tests.rs`

**Target**: Native (non-WASM) architecture

**Scope**: Documentation and validation of test coverage

**Tests**: 4 documentation tests that verify:
- Comprehensive test coverage summary (26+ tests)
- WASM build configuration
- HTTP method support (8 methods)
- Error scenario documentation (10+ scenarios)

**Running Host Tests**:
```bash
cargo test -p spikard-wasm --test host_ffi_tests
```

## Test Coverage by Category

### 1. Module Initialization (1 test)
- `test_init_sets_panic_hook`: Verifies panic hook initialization

### 2. TestClient Construction (7 tests)
- Valid JSON routes deserialization
- Invalid JSON rejection
- Handler type validation
- Multiple routes support
- Empty routes array
- No panic on valid input
- Multiple client instances

### 3. Configuration Deserialization (7 tests)
- Valid server config parsing
- Invalid config type rejection
- Malformed JSON rejection
- Null/undefined config handling
- Compression config parsing
- Rate limiting config parsing
- Static files manifest parsing

### 4. Route Definitions (6 tests)
- Routes with validation schemas
- Complex route definitions with all optional fields
- Path parameters (`:param`)
- Greedy path parameters (`*path`)
- HTTP method case-insensitivity
- All 8 HTTP methods support

### 5. HTTP Method Handlers (8 tests)
- GET returns Promise
- POST returns Promise
- PUT returns Promise
- PATCH returns Promise
- DELETE returns Promise
- HEAD returns Promise
- OPTIONS returns Promise
- TRACE returns Promise

### 6. Error Handling (2 tests)
- FFI error propagation
- Descriptive error messages

### 7. FFI Safety (3 tests)
- Memory safety with handler references
- Lifecycle hooks FFI boundary
- Streaming response FFI handling

**Total: 26+ Tests**

## FFI Coverage Areas

### Type Conversions
- `JsValue` ↔ Rust types
- JSON string ↔ serde_json::Value
- Handler map (JS Object) ↔ HashMap<String, Function>
- Config object ↔ ServerConfig struct
- Route metadata ↔ RouteDefinition

### Error Propagation
- Invalid JSON → JsValue error
- Type mismatches → JsValue error
- Schema validation failures → structured JSON response
- Handler failures → error response object
- Promise rejections → error handling

### Async/Promise Handling
- All HTTP methods return Promise
- Promise resolution with response object
- Promise rejection with error handling
- Lifecycle hook promise integration

### Memory Management
- Handler Function references
- Route validator Arc wrapping
- Lifecycle hooks Option/Arc management
- No panics across FFI boundary
- Proper lifetime management

## Running All Tests

### Quick Validation
```bash
# Check compilation
cargo check -p spikard-wasm --tests

# Run documentation tests (host)
cargo test -p spikard-wasm --test host_ffi_tests
```

### Complete Test Suite
```bash
# Run WASM tests with Firefox
wasm-pack test --headless --firefox crates/spikard-wasm

# Or with Chrome
wasm-pack test --headless --chrome crates/spikard-wasm

# Run host documentation tests
cargo test -p spikard-wasm --test host_ffi_tests

# Show test list without running
cargo test -p spikard-wasm --test wasm_bindgen_tests -- --list
```

## Test Configuration

### wasm-bindgen-test Framework
The tests use `wasm-bindgen-test v0.3` which:
- Compiles Rust tests to WASM
- Runs in headless browser or Node.js
- Provides async test support
- Integrates with wasm-pack tooling

### Dependencies
- `wasm-bindgen-test = "0.3"` (dev-dependency)
- `wasm-bindgen = "0.2"`
- `js-sys = "0.3"`
- `serde-wasm-bindgen = "0.6"`

## WASM-Specific Considerations

### Panic Hook
The `init()` function sets `console_error_panic_hook` to display panics in console instead of cryptic wasm errors.

### Promise Returns
All HTTP methods return JavaScript Promise objects via:
- `wasm_bindgen::prelude::Promise`
- `wasm_bindgen_futures::future_to_promise()`

### Type Conversions
FFI uses:
- `serde_wasm_bindgen` for JSON ↔ JS values
- `js_sys` for JS object/array/string manipulation
- `JsValue` as universal type bridge

### Memory Safety
- All FFI calls wrapped in Result
- No unwrap() across boundaries
- Proper error conversion to JsValue
- Handler Function refs properly cloned

## CI/CD Integration

For GitHub Actions or similar CI systems:

```yaml
# WASM tests require headless browser
- name: Run WASM FFI Tests
  run: wasm-pack test --headless --firefox crates/spikard-wasm

# Host documentation tests
- name: Run Test Documentation
  run: cargo test -p spikard-wasm --test host_ffi_tests
```

## Troubleshooting

### wasm-pack not found
```bash
cargo install wasm-pack
```

### Firefox not available for testing
```bash
wasm-pack test --headless --chrome crates/spikard-wasm
```

### Tests fail in browser
1. Check console for panic messages
2. Verify handler function signatures
3. Validate JSON route definitions
4. Check Promise resolution in handlers

### Compilation issues
```bash
# Clean and rebuild
cargo clean -p spikard-wasm
cargo check -p spikard-wasm --tests
```

## Future Test Enhancements

1. **Streaming Response Tests**: More comprehensive streaming scenarios
2. **Lifecycle Hook Tests**: Additional hook execution scenarios
3. **Performance Tests**: Benchmark FFI call overhead
4. **Integration Tests**: Full request/response cycle validation
5. **Edge Case Tests**: Large payloads, deeply nested JSON, etc.
6. **Cross-Browser Tests**: Safari, Edge compatibility
7. **Node.js Tests**: Separate test suite for Node.js environment

## Test Maintenance

When adding new features:
1. Add corresponding test in appropriate category
2. Update test count in `host_ffi_tests.rs`
3. Update error scenario documentation
4. Run full test suite before committing
5. Update this README with new test descriptions

## References

- [wasm-bindgen testing docs](https://rustwasm.org/wasm-bindgen/wasm-bindgen-test/)
- [wasm-pack documentation](https://rustwasm.org/docs/wasm-pack/)
- [MDN: WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
