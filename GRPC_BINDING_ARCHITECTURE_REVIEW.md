# gRPC Binding Architectural Consistency Review

**Date:** 2026-01-10
**Scope:** 5 Language Bindings (Python, Node.js, Ruby, PHP, WASM)
**Reference Implementation:** Python (`crates/spikard-py/src/grpc/handler.rs`)
**Review Status:** ⚠️ **CRITICAL INCONSISTENCIES FOUND**

---

## Executive Summary

This architectural consistency review identified **8 critical inconsistencies** across the 5 gRPC handler implementations that violate the **thin-binding-pattern-architecture** rule and create maintenance burden, API surface divergence, and potential security gaps.

### Key Findings:

- **Trait Implementation**: All 5 bindings implement the `GrpcHandler` trait, but with WILDLY DIFFERENT streaming support levels
- **Error Handling**: Inconsistent error mapping and status code usage across bindings
- **Streaming Support**: Python ✅ (4/4), Ruby ✅ (4/4), PHP ✅ (4/4), Node.js ❌ (1/4), WASM ⚠️ (0/4)
- **Metadata Handling**: Ruby/PHP use shared helper, Python/Node/WASM use custom code
- **Type Naming**: Inconsistent naming across bindings (PyGrpcRequest vs GrpcRequest vs PhpGrpcRequest)
- **Testing Coverage**: Only Python has integration tests; others have unit tests only
- **Documentation**: Inconsistent and missing in Node.js/WASM streaming methods

### Impact:
- **Breaking API Parity**: Users expect identical API surface across all bindings
- **Maintenance Debt**: 5 different implementations of similar logic
- **Security Risk**: Inconsistent error handling may leak different information across languages
- **User Confusion**: Streaming works differently on each platform

---

## Detailed Findings by Category

### 1. TRAIT IMPLEMENTATION COMPLETENESS (Critical Priority)

All bindings correctly implement `GrpcHandler` trait with 4 methods required:
- `call()` - Unary RPC
- `call_server_stream()` - Server streaming
- `call_client_stream()` - Client streaming
- `call_bidi_stream()` - Bidirectional streaming

However, **streaming support varies significantly**:

| Binding | call() | server_stream() | client_stream() | bidi_stream() | Status |
|---------|--------|-----------------|-----------------|---------------|--------|
| **Python** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | COMPLETE |
| **Ruby** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | COMPLETE |
| **PHP** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | COMPLETE |
| **Node.js** | ✅ Full | ❌ Unimplemented | ❌ Unimplemented | ❌ Unimplemented | INCOMPLETE |
| **WASM** | ❌ Stub | ❌ Stub | ❌ Stub | ❌ Stub | NOT IMPLEMENTED |

**Issues:**
- Node.js: Returns `tonic::Status::unimplemented()` for all 3 streaming modes (lines 288-386)
- WASM: No actual handler implementation; only type definitions and placeholder `GrpcMessageStream`

**Violates Rule:** `thin-binding-pattern-architecture` - All bindings should have parity in what's supported

---

### 2. REQUEST/RESPONSE TYPE NAMING INCONSISTENCY (High Priority)

Type naming varies across bindings, creating API confusion:

```rust
// Python
#[pyclass(name = "GrpcRequest")]
pub struct PyGrpcRequest { ... }

// Node.js
#[napi(object)]
pub struct GrpcRequest { ... }

// Ruby
#[magnus::wrap(class = "Spikard::Grpc::Request")]
pub struct RubyGrpcRequest { ... }

// PHP
#[php_class]
pub struct PhpGrpcRequest { ... }

// WASM
#[wasm_bindgen(getter_with_clone)]
pub struct GrpcRequest { ... }
```

**Issues:**
- No consistent naming pattern: Python uses `Py` prefix, Ruby uses `Ruby` prefix, PHP uses `Php` prefix
- Node.js/WASM use generic `GrpcRequest` (confusing: are they FFI-neutral?)
- Violates principle: Binding code should be clearly identifiable as language-specific

**Recommendation:** Standardize to `{Language}GrpcRequest` pattern:
- `PyGrpcRequest` ✅
- `NodeGrpcRequest` (change from `GrpcRequest`)
- `RubyGrpcRequest` ✅
- `PhpGrpcRequest` ✅
- `WasmGrpcRequest` (change from `GrpcRequest`)

---

### 3. ERROR HANDLING PATTERNS (Critical Priority)

Error handling shows 3 distinct patterns across bindings:

#### Pattern A: Exception Mapping (Python)
```rust
// Lines 240-279: Custom exception-to-status mapping
fn pyerr_to_grpc_status(err: PyErr) -> tonic::Status {
    Python::attach(|py| {
        let err_type = err.get_type(py);
        if err_type.is_subclass_of::<pyo3::exceptions::PyValueError>() {
            tonic::Status::invalid_argument(err_msg)
        } else if err_type.is_subclass_of::<pyo3::exceptions::PyPermissionError>() {
            tonic::Status::permission_denied(err_msg)
        }
        // ... 6 exception types mapped
    })
}
```
**Benefit:** Maps language exceptions to gRPC codes semantically

#### Pattern B: Direct Status Conversion (Ruby, PHP)
```rust
// Ruby: Simple wrapping in tonic::Status
tonic::Status::internal(format!("Ruby gRPC handler failed: {}", err))

// PHP: Similar approach
tonic::Status::internal(format!("Failed to invoke PHP handler: {:?}", e))
```
**Issue:** All errors map to INTERNAL status; loses semantic information

#### Pattern C: Promise-based (Node.js)
```rust
// Lines 226-231: No exception handling shown
let js_response = handler_fn
    .call_async(js_request)
    .await
    .map_err(|e| tonic::Status::internal(format!("Handler call failed for {}: {}", service_name, e)))?
```
**Issue:** Maps all JavaScript errors to INTERNAL; inconsistent with Python

#### Pattern D: Missing (WASM)
```rust
// No error handling implemented for handlers
// GrpcStatus type defined but not used
```

**Violations:**
1. **Inconsistent error semantics**: Python maps to 7 different status codes; others use INTERNAL
2. **Information leakage**: Different amounts of error detail exposed per binding
3. **Non-DRY**: Ruby/PHP/Node each implement error conversion separately
4. **No custom errors**: WASM not implemented

**Recommendation:** Create shared error mapping in `spikard_bindings_shared::grpc_metadata` or new `grpc_errors` module:
```rust
pub fn map_handler_error(
    error_description: &str,
    language: &str,
) -> tonic::Status {
    // Standardized error mapping across all bindings
}
```

---

### 4. METADATA HANDLING (High Priority)

Metadata conversion shows 2 approaches:

#### Approach A: Shared Helpers (Ruby, PHP) ✅
```rust
// Ruby (line 40)
let metadata = extract_metadata_to_hashmap(&request.metadata, true);

// PHP (line 89)
let metadata = extract_metadata_to_hashmap(&request.metadata, true);
```
Uses: `spikard_bindings_shared::grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata}`

**Benefits:**
- Single implementation in shared crate
- Consistent behavior across languages
- Easy to update for all bindings at once

#### Approach B: Custom Implementation (Python, Node.js)
```rust
// Python (lines 94-135): Custom metadata_map_to_pydict and pydict_to_metadata_map
fn metadata_map_to_pydict<'py>(py: Python<'py>, metadata: &MetadataMap) -> PyResult<Bound<'py, PyDict>> {
    let py_dict = PyDict::new(py);
    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value {
            // ...
        }
    }
    Ok(py_dict)
}

// Node.js (lines 88-119): Similar custom metadata_to_hashmap and hashmap_to_metadata
fn metadata_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for key_and_value in metadata.iter() {
        match key_and_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                if let Ok(value_str) = value.to_str() {
                    map.insert(key.as_str().to_string(), value_str.to_string());
                }
            }
            // ...
        }
    }
    map
}
```

**Issues:**
- Python: 45 lines of metadata conversion code (lines 94-135)
- Node.js: 32 lines of metadata conversion code (lines 88-119)
- Code duplication of `extract_metadata_to_hashmap` logic
- Both implementations handle binary metadata differently:
  - Ruby/PHP: Skip with optional debug logging
  - Node.js: Skip silently
  - Python: Skip without logging

**Recommendation:** Migrate Python and Node.js to use shared helpers:
```rust
// In crates/spikard-py/src/grpc/handler.rs
use spikard_bindings_shared::grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata};

// Remove lines 94-135, 112-135; replace with:
let metadata = extract_metadata_to_hashmap(&request.metadata, false);  // No logging for Python
```

---

### 5. STREAM CONVERSION PATTERNS (Critical Priority)

Four completely different approaches to converting language streams to/from Rust MessageStream:

#### Pattern A: Python - Async Iterator Wrapper
```rust
// Lines 22-81: PyGrpcMessageStream with __aiter__/__anext__
// Lines 141-218: python_async_generator_to_message_stream via async_stream!
// Lines 224-229: create_py_stream_iterator
```

**Mechanism:**
- Wraps `MessageStream` in `PyGrpcMessageStream` with `Arc<tokio::sync::Mutex<Option<MessageStream>>>`
- Implements async iterator protocol: `__aiter__()`, `__anext__()`
- Converts Python async generator to `MessageStream` using `async_stream!` macro
- Uses `pyo3_async_runtimes::into_future_with_locals()` for event loop integration

**Strengths:**
- Full streaming support for all 4 RPC modes
- Zero-copy payload handling
- Proper Python async/await integration

#### Pattern B: Ruby - Enumerator Conversion
```rust
// Lines 152-171: RubyGrpcMessageStream wrapper
// Lines 365-412: ruby_enumerator_to_message_stream via futures_util::stream::iter
```

**Mechanism:**
- Wraps stream in `RubyGrpcMessageStream` struct
- Converts Ruby Enumerator to array via `to_a` method (synchronous!)
- Creates `MessageStream` from `Vec<Result<Bytes, tonic::Status>>`

**Weaknesses:**
- Collects entire stream to memory before returning (not true streaming)
- Calls to `to_a` on enumerator can be expensive
- Limited to 100,000 messages per handler call

#### Pattern C: PHP - Message Collection
```rust
// Lines 534-563: php_generator_to_message_stream
// Lines 566-661: collect_php_generator_messages
// Lines 664-675: collect_message_stream_to_vec (synchronous blocking)
```

**Mechanism:**
- Collects `MessageStream` to `Vec<Bytes>` using `block_in_place()` (lines 395-398)
- Iterates PHP Generator via `rewind()`/`valid()`/`current()`/`next()` methods
- Converts back to `MessageStream` via `futures_util::stream::iter`

**Issues:**
- Uses `std::thread::block_in_place()` to bridge sync/async (lines 395-398, 434-437)
- Collects entire stream into memory for client/bidi streaming
- PHP Generator iteration with 100,000 message limit (line 640-642)
- Not true streaming

#### Pattern D: Node.js - Channel-based (Partial)
```rust
// Lines 125-152: create_js_stream_iterator using mpsc::unbounded_channel
```

**Mechanism:**
- Creates `mpsc::unbounded_channel` to convert stream to channel
- Spawns task to forward messages from stream to channel receiver
- Returns `GrpcMessageStream` wrapping the receiver

**Critical Issues:**
- **UNIMPLEMENTED for server streaming** (lines 267-274)
- **UNIMPLEMENTED for client streaming** (lines 326-352)
- **UNIMPLEMENTED for bidi streaming** (lines 355-385)
- Only `call()` (unary) works; all streaming returns `UNIMPLEMENTED`

#### Pattern E: WASM - Placeholder
```rust
// Lines 179-224: Placeholder GrpcMessageStream with stub implementation
// Lines 237-323: javascript_async_generator_to_message_stream (not used)
```

**Issues:**
- Not integrated with actual handler implementation
- Stub methods return placeholder values
- No actual gRPC handler implementation

**Analysis:**

This is the most critical inconsistency. Streaming patterns range from **fully functional** (Python, Ruby, PHP) to **completely unimplemented** (Node.js, WASM).

| Binding | Unary | Server Stream | Client Stream | Bidi Stream | Issue |
|---------|-------|---------------|---------------|-------------|-------|
| Python | ✅ | ✅ | ✅ | ✅ | None |
| Ruby | ✅ | ✅ (memory) | ✅ (memory) | ✅ (memory) | Collects entire stream to memory |
| PHP | ✅ | ✅ (memory) | ✅ (memory) | ✅ (memory) | Uses `block_in_place()` to bridge sync/async |
| Node.js | ✅ | ❌ | ❌ | ❌ | Completely unimplemented |
| WASM | ❌ | ❌ | ❌ | ❌ | Placeholder only |

**Violations:**
1. **API Parity**: Users cannot use streaming on Node.js/WASM
2. **Architecture**: Streaming implementation details leaked (Ruby/PHP collect to memory)
3. **DRY**: Python's streaming pattern is superior but not reused in Ruby/PHP

---

### 6. DOCUMENTATION CONSISTENCY (High Priority)

Documentation quality varies significantly:

#### Python (Excellent)
- 836 lines total
- Detailed rustdoc comments on all types and methods
- Examples in tests (lines 725-835)
- Handler pattern clearly documented (lines 18-22)

#### Ruby (Good)
- 566 lines total
- Rustdoc on public types
- Module initialization with Ruby class definitions (lines 452-475)
- Test coverage of all patterns

#### PHP (Good)
- 825 lines total
- Comprehensive rustdoc and helper function documentation
- Thread-local registry documentation (lines 161-182)
- Panic safety documentation

#### Node.js (Adequate)
- 451 lines total
- Basic rustdoc on types
- **Missing:** Documentation of why streaming is unimplemented (lines 267-274, 326-352, 355-385)
- Comment explaining the limitation but no guidance on workarounds

#### WASM (Poor)
- 359 lines total
- 68 lines of architecture documentation (lines 7-67) that describes ideal behavior, not actual implementation
- Handler patterns documented (lines 26-67) but not implemented
- Code comments acknowledge "not integrated" (lines 184-188)
- Tests are stubs with no implementation (lines 343-358)

**Issues:**
- Documentation-code mismatch: WASM docs describe features that don't exist
- Node.js doesn't document workaround strategy
- No cross-binding compatibility matrix

---

### 7. ERROR MESSAGE CONSISTENCY (Medium Priority)

Error messages follow inconsistent patterns:

#### Example: Streaming Not Supported

**Node.js** (lines 295-297):
```
"Server streaming requires JavaScript handler to implement streaming via callback or return pre-collected messages"
```

**WASM** (not shown - method not implemented):
```
(no error message provided)
```

**Python** (no such error - streaming works):
```
(no error)
```

**Ruby/PHP** (no such error - streaming works):
```
(no error)
```

#### Example: Handler Call Failure

**Python** (line 63):
```
"gRPC error: {}"
```

**Node.js** (line 229):
```
"Handler call failed for {}: {}"
```

**Ruby** (line 238):
```
"Ruby gRPC handler failed: {}"
```

**PHP** (line 342):
```
"PHP gRPC handler '{}' failed: {:?}"
```

**WASM**:
```
(not implemented)
```

**Issue:** Error messages vary in detail level, structure, and field ordering. Users migrating between languages see different error formats.

---

### 8. TESTING COVERAGE (Medium Priority)

Testing approaches vary:

#### Python
- **Unit tests:** Basic request/response creation (lines 737-834)
- **Integration tests:** Full Python suite with fixtures (external)
- **Coverage:** Metadata, request/response serialization
- **Status:** ✅ Comprehensive

#### Ruby
- **Unit tests:** Request creation, metadata extraction, response conversion (lines 483-565)
- **Coverage:** 7 test cases
- **Status:** ✅ Good

#### PHP
- **Unit tests:** Request/response creation, metadata, payload sizes (lines 707-824)
- **Coverage:** 12 test cases
- **Status:** ✅ Good

#### Node.js
- **Unit tests:** Metadata conversion only (lines 402-450)
- **Coverage:** 5 test cases
- **Missing:** Request/response type tests, streaming tests
- **Status:** ⚠️ Incomplete

#### WASM
- **Unit tests:** Stub tests with no assertions (lines 343-358)
- **Coverage:** 0%
- **Status:** ❌ None

**Issues:**
- Node.js/WASM don't test their own request/response types
- No cross-binding integration tests
- WASM tests provide no actual validation

---

## Summary of Critical Issues

### Priority 1: CRITICAL (Blocks Release)

1. **Streaming Implementation Parity** (Lines vary by binding)
   - Node.js returns UNIMPLEMENTED for 3/4 streaming modes
   - WASM has no handler implementation
   - Violates thin-binding-pattern-architecture rule
   - **Fix:** Implement streaming or document as non-streaming binding

2. **Error Handling Inconsistency** (Python: 240-279, Ruby: 238, PHP: 342, Node: 229)
   - 4 different error mapping strategies
   - Python maps to 7 codes; others use only INTERNAL
   - Violates fixture-aligned-error-handling rule
   - **Fix:** Shared error mapping utility

3. **WASM Implementation Incomplete** (359 lines)
   - Documentation describes features not implemented
   - Handler trait not actually implemented for handlers
   - Stub GrpcMessageStream with no functionality
   - **Fix:** Either implement fully or remove

### Priority 2: HIGH (Should Fix Before Release)

4. **Type Naming Inconsistency** (Lines 26-120 across bindings)
   - No consistent naming pattern across bindings
   - Creates API confusion for users
   - **Fix:** Standardize to `{Language}GrpcRequest` pattern

5. **Metadata Handling Duplication** (Python: 94-135, Node: 88-119)
   - Python/Node re-implement shared logic
   - Ruby/PHP use shared helpers
   - **Fix:** Migrate Python/Node to shared helpers

6. **Streaming Pattern Duplication** (All bindings)
   - Ruby/PHP duplicate each other's memory collection pattern
   - Python's pattern is superior but not documented for reuse
   - **Fix:** Document Python's pattern as reference; improve Ruby/PHP

### Priority 3: MEDIUM (Should Address in Next Release)

7. **Documentation Mismatch** (WASM especially: lines 7-67)
   - WASM docs describe unimplemented features
   - Node.js doesn't explain streaming limitation strategy
   - **Fix:** Align documentation with actual implementation

8. **Error Message Inconsistency** (Varies by binding)
   - Different formats and detail levels across bindings
   - Creates confusion for users
   - **Fix:** Standardize error message format

9. **Testing Coverage Gaps** (Node.js, WASM)
   - Node.js missing request/response tests
   - WASM has stub tests
   - **Fix:** Add comprehensive test coverage

---

## Thin Binding Pattern Violations

The **thin-binding-pattern-architecture** rule (HIGH priority) requires:

> All language bindings must follow the "thin binding" pattern: expose only language-idiomatic APIs over the Rust core. NEVER duplicate business logic, validation, middleware, or routing across bindings.

**Violations Found:**

1. **Metadata Conversion** (DRY violation)
   - Python/Node: 45-32 lines each of custom code
   - Ruby/PHP: 1 line using shared helper
   - Ruby/PHP follow the rule; Python/Node don't

2. **Error Handling** (DRY violation)
   - Each binding implements custom exception-to-status mapping
   - Python maps to 7 codes; others use INTERNAL
   - No shared logic despite identical requirement

3. **Stream Conversion** (DRY violation)
   - Python's async iterator pattern is superior
   - Ruby/PHP duplicate memory collection pattern
   - No shared abstraction for streaming conversion

4. **Type Naming** (API Consistency violation)
   - Inconsistent naming patterns across bindings
   - Creates user-facing API inconsistency

---

## Recommendations by Category

### 1. Immediate Actions (Before Release)

#### 1a. Fix Node.js Streaming (Critical)
**Current State:** All 3 streaming modes return UNIMPLEMENTED
**Options:**
- A) Implement streaming support (recommended)
- B) Document as "unary only" binding and remove from release

**Implementation for Option A:**
```rust
// In crates/spikard-node/src/grpc/handler.rs

// Implement streaming using channel-based approach similar to current create_js_stream_iterator
fn call_server_stream(...) -> Pin<Box<...>> {
    // Convert MessageStream to GrpcMessageStream
    // Call JavaScript handler with method detection
    // Convert result back to MessageStream
}
```

**Effort:** Medium (2-3 days)

#### 1b. Implement WASM Handler (Critical)
**Current State:** Placeholder only; no handler implementation
**Action:** Either implement full handler or remove from release

**Implementation Required:**
```rust
// In crates/spikard-wasm/src/grpc/handler.rs
pub struct WasmGrpcHandler {
    handler_fn: js_sys::Function,
    service_name: String,
}

impl GrpcHandler for WasmGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<...>> {
        // Convert to JS types
        // Call JS handler
        // Convert response back
    }
    // ... implement all 4 streaming methods
}
```

**Effort:** High (5-7 days)

#### 1c. Fix Documentation Mismatch (High)
**Current State:** WASM docs describe unimplemented features
**Action:** Update docs to match actual implementation

**Files to Update:**
- `crates/spikard-wasm/src/grpc/handler.rs` (lines 7-67)
- `crates/spikard-node/src/grpc/handler.rs` (lines 267-385)

**Effort:** Low (0.5 day)

### 2. Code Quality Improvements (Before Release)

#### 2a. Standardize Type Naming (High)
**Current:** Mixed naming patterns
**Target:** All bindings use `{Language}GrpcRequest` pattern

**Changes Required:**
- Node.js: `GrpcRequest` → `NodeGrpcRequest`
- WASM: `GrpcRequest` → `WasmGrpcRequest`
- Rename wasm_bindgen exports accordingly

**Files:**
- `crates/spikard-node/src/grpc/handler.rs` (lines 25-48)
- `crates/spikard-wasm/src/grpc/handler.rs` (lines 107-177)

**Effort:** Low (2-4 hours)

#### 2b. Centralize Metadata Handling (High)
**Current:** Duplication in Python and Node.js
**Target:** All bindings use `spikard_bindings_shared::grpc_metadata`

**Changes:**
```rust
// Remove from Python handler.rs:
// - metadata_map_to_pydict (45 lines)
// - pydict_to_metadata_map (23 lines)
// - option_hashmap_to_pydict (8 lines)

// Use instead:
use spikard_bindings_shared::grpc_metadata::extract_metadata_to_hashmap;
let metadata = extract_metadata_to_hashmap(&request.metadata, false);
```

**Files:**
- `crates/spikard-py/src/grpc/handler.rs`
- `crates/spikard-node/src/grpc/handler.rs`

**Effort:** Low-Medium (1-2 days)

#### 2c. Create Shared Error Mapping (High)
**Current:** Each binding implements custom mapping
**Target:** Centralized error handling in `spikard_bindings_shared`

**Implementation:**
```rust
// In crates/spikard-bindings-shared/src/grpc_errors.rs

pub struct GrpcErrorContext {
    pub handler_service: String,
    pub handler_language: &'static str,
}

pub fn map_handler_error(
    error: &str,
    context: &GrpcErrorContext,
) -> tonic::Status {
    // Consistent mapping across all bindings
}

pub fn map_python_exception(exception: &PyErr) -> tonic::Status {
    // Keep Python's semantic mapping
}
```

**Files to Update:**
- All handler.rs files (error mapping sections)

**Effort:** Medium (2-3 days)

### 3. Architecture Improvements (Next Release)

#### 3a. Document Streaming Patterns (Medium)
**Create:** `docs/adr/0007-grpc-streaming-architecture.md`

**Content:**
- Python's async iterator pattern (reference implementation)
- Ruby/PHP's memory-collection pattern (trade-offs)
- Node.js implementation strategy
- WASM streaming constraints

**Effort:** Low (4-6 hours)

#### 3b. Standardize Error Messages (Low)
**Target:** Consistent format across all bindings

**Pattern:**
```
{Action} for service '{service}': {details}
```

**Example:**
```
"Failed to invoke handler for service 'mypackage.MyService': invalid metadata key"
```

**Effort:** Low (1 day)

#### 3c. Improve Testing (Medium)
**Node.js:**
- Add request/response type tests
- Add streaming pattern tests

**WASM:**
- Replace stub tests with real assertions
- Add integration tests with actual handlers

**Effort:** Medium (2-3 days)

---

## Compliance Matrix

### Rule Compliance Summary

| Rule | Python | Ruby | PHP | Node.js | WASM |
|------|--------|------|-----|---------|------|
| **thin-binding-pattern-architecture** | ⚠️ Partial | ✅ Yes | ✅ Yes | ❌ No | ❌ No |
| **fixture-aligned-error-handling** | ✅ Yes | ❌ No | ❌ No | ❌ No | ❌ No |
| **handler-trait-abstraction** | ✅ Yes | ✅ Yes | ✅ Yes | ⚠️ Partial | ❌ No |
| **cross-language-error-boundaries** | ✅ Yes | ✅ Yes | ✅ Yes | ⚠️ Partial | ❌ No |

---

## Recommended Action Plan

### Phase 1: Critical Fixes (Week 1-2)
1. **Day 1-2:** Standardize type naming across all bindings
2. **Day 2-3:** Centralize metadata handling in Python/Node.js
3. **Day 3-4:** Fix WASM documentation mismatch
4. **Day 4-5:** Implement Node.js streaming support

### Phase 2: Code Quality (Week 3)
1. **Day 1-2:** Create shared error mapping utility
2. **Day 2-3:** Update all bindings to use shared error mapping
3. **Day 3-4:** Add missing test coverage (Node.js, WASM)

### Phase 3: Documentation (Week 4)
1. **Day 1:** Create streaming patterns ADR
2. **Day 2-3:** Standardize error messages across bindings
3. **Day 4:** Update handler documentation

---

## Conclusion

The gRPC binding implementations show **good foundation** with Python, Ruby, and PHP providing full functionality. However, **critical gaps** in Node.js/WASM and **significant duplication** in metadata/error handling create maintenance burden and violate architectural rules.

**Key Risk:** Incomplete Node.js/WASM implementations will frustrate users expecting feature parity across platforms.

**Key Opportunity:** Consolidating error handling and metadata conversion into shared utilities (following Ruby/PHP's pattern) will reduce code by ~100 lines and improve consistency.

**Recommendation:** Address Phase 1 critical fixes before releasing gRPC support. Prioritize implementing Node.js streaming and removing or fixing WASM placeholder.

---

## Files Requiring Changes

### Critical
- `crates/spikard-node/src/grpc/handler.rs` (implement streaming)
- `crates/spikard-wasm/src/grpc/handler.rs` (implement or remove)

### High Priority
- `crates/spikard-py/src/grpc/handler.rs` (metadata handling)
- `crates/spikard-node/src/grpc/handler.rs` (type naming, metadata handling)
- `crates/spikard-bindings-shared/src/grpc_metadata.rs` (add error mapping module)

### Medium Priority
- All handler.rs files (error mapping updates)
- All handler.rs files (test coverage improvements)

### Documentation
- `docs/adr/0007-grpc-streaming-architecture.md` (new)
- `crates/spikard-node/src/grpc/handler.rs` (documentation updates)
- `crates/spikard-wasm/src/grpc/handler.rs` (documentation updates)

---

## Appendix: Code Quality Metrics

### Lines of Code by Binding

| Binding | Total | Core Logic | Tests | % Tested |
|---------|-------|-----------|-------|----------|
| Python | 836 | 615 | 221 | 36% |
| Ruby | 566 | 451 | 115 | 20% |
| PHP | 825 | 733 | 92 | 11% |
| Node.js | 451 | 387 | 64 | 14% |
| WASM | 359 | 323 | 36 | 10% |

### Code Duplication

- **Metadata handling:** ~80 lines (Python + Node.js) vs 2 lines (Ruby + PHP) = **40x difference**
- **Error mapping:** ~150 lines custom logic across all bindings (not consolidated)
- **Stream conversion:** ~400 lines with minimal sharing between bindings

### Test Coverage Quality

- **Python:** 8 test cases covering request/response/metadata
- **Ruby:** 7 test cases with integration patterns
- **PHP:** 12 test cases including edge cases
- **Node.js:** 5 test cases (metadata only)
- **WASM:** 2 stub tests (no assertions)
