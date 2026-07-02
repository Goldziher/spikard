# ADR 0011: gRPC FFI Bindings Strategy

**Status**: Accepted
**Date**: 2025-12-31

## Context

Spikard's gRPC runtime is implemented in Rust (`spikard-http`) using Tonic and needs to be exposed to four languages (Python, TypeScript/Node.js, Ruby, PHP) through FFI bindings. The challenge is bridging:

1. **Binary protocol** - Protobuf messages as raw bytes across FFI boundary
2. **Async execution** - gRPC uses async I/O, but some language runtimes (Ruby, PHP) are blocking
3. **Streaming support** - Four streaming modes (unary, server, client, bidirectional)
4. **Metadata handling** - gRPC headers/trailers as key-value pairs
5. **Error mapping** - 17 gRPC status codes → language-native exceptions

Each language has different FFI mechanisms (PyO3, napi-rs, Magnus, ext-php-rs) with varying capabilities and limitations.

## Decision

### Architecture

**Shared gRPC Core**: `crates/spikard-http/src/grpc/`

- Common metadata conversion logic (MetadataMap ↔ HashMap)
- gRPC status code constants and mappings
- Tonic runtime and request/response handling

**Language-Specific Bindings**:

```text
crates/
├── spikard-py/src/          # Python (PyO3) - includes gRPC support
├── spikard-node/src/        # Node.js (napi-rs) - includes gRPC support
├── spikard-php/src/         # PHP (ext-php-rs) - includes gRPC support
└── packages/ruby/ext/spikard_rb/native/src/  # Ruby (Magnus) - includes gRPC support
```

### Core Principles

1. **Binary Opaque Payloads**
   - FFI layer treats protobuf messages as opaque byte strings
   - Language-side handles serialization/deserialization
   - No proto parsing in FFI boundary

2. **Keyword Argument Pattern**
   - All constructors accept keyword arguments for clarity
   - Example: `GrpcRequest(service_name=..., method_name=..., payload=...)`
   - Handles both positional and keyword args for flexibility

3. **Metadata as Dictionaries**
   - Metadata exposed as `dict[str, str]` (Python), `Object` (TS), `Hash` (Ruby), `array` (PHP)
   - Internal conversion to/from Tonic's `MetadataMap`
   - Case-sensitive keys (gRPC spec requirement)

4. **Status Code Mapping**
   - Language exceptions → gRPC status codes
   - Python: `ValueError` → INVALID_ARGUMENT, `PermissionError` → PERMISSION_DENIED
   - TypeScript: Error subclasses for each status code
   - Ruby: StandardError hierarchy
   - PHP: Exception hierarchy

### FFI Type Mapping

**Request/Response Flow**:

```text
Language Handler
    ↓ (binary payload)
FFI Boundary (PyGrpcRequest, RubyGrpcRequest, etc.)
    ↓ (GrpcRequestData)
Rust Runtime (Tonic)
    ↓ (GrpcResponseData)
FFI Boundary (PyGrpcResponse, RubyGrpcResponse, etc.)
    ↑ (binary payload)
Language Handler
```

**Data Structures**:

**Rust Core** (`spikard-http/src/grpc/mod.rs`):

```rust
pub struct GrpcRequestData {
    pub service_name: String,
    pub method_name: String,
    pub payload: Bytes,              // Binary protobuf
    pub metadata: MetadataMap,
}

pub struct GrpcResponseData {
    pub payload: Bytes,              // Binary protobuf
    pub metadata: MetadataMap,
}
```

**Python Binding** (`spikard-py/src/grpc/handler.rs`):

```rust
#[pyclass]
pub struct PyGrpcRequest {
    service_name: String,
    method_name: String,
    payload: Vec<u8>,
    metadata: HashMap<String, String>,
}

#[pymethods]
impl PyGrpcRequest {
    #[getter]
    fn service_name(&self) -> &str { &self.service_name }

    #[getter]
    fn payload<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.payload)
    }

    #[getter]
    fn metadata(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        for (k, v) in &self.metadata {
            dict.set_item(k, v)?;
        }
        Ok(dict.into())
    }
}
```

**Ruby Binding** (`packages/ruby/ext/spikard_rb/native/src/grpc/handler.rs`):

```rust
#[magnus::wrap(class = "Spikard::Grpc::Request")]
pub struct RubyGrpcRequest {
    service_name: String,
    method_name: String,
    payload: Vec<u8>,
    metadata: HashMap<String, String>,
}

impl RubyGrpcRequest {
    fn service_name(&self) -> String { self.service_name.clone() }

    fn payload(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.str_from_slice(&rb_self.payload).as_value()
    }

    fn metadata(ruby: &Ruby, rb_self: &Self) -> Result<Value, Error> {
        let hash = ruby.hash_new();
        for (key, value) in &rb_self.metadata {
            hash.aset(ruby.str_new(key), ruby.str_new(value))?;
        }
        Ok(hash.as_value())
    }
}
```

### Streaming Support Strategy

**Current State**: Unary RPC only (request → response)

**Future Expansion**:

1. **Server Streaming**: Return iterator/generator from handler
2. **Client Streaming**: Accept iterator/generator as request
3. **Bidirectional**: Both request and response as async iterators

**Design Constraints**:

- PyO3: Async generators via `async_generator` crate
- napi-rs: AsyncIterator support in Node.js
- Magnus: Ruby Enumerator for lazy iteration
- ext-php-rs: PHP Generator objects

### Error Handling Strategy

**Exception → Status Code Mapping**:

**Python** (`pyerr_to_grpc_status`):

```rust
fn pyerr_to_grpc_status(err: PyErr) -> tonic::Status {
    Python::with_gil(|py| {
        let err_type = err.get_type(py);
        if err_type.is_subclass_of::<PyValueError>().unwrap_or(false) {
            tonic::Status::invalid_argument(err.to_string())
        } else if err_type.is_subclass_of::<PyPermissionError>().unwrap_or(false) {
            tonic::Status::permission_denied(err.to_string())
        } else if err_type.is_subclass_of::<PyNotImplementedError>().unwrap_or(false) {
            tonic::Status::unimplemented(err.to_string())
        } else {
            tonic::Status::internal(err.to_string())
        }
    })
}
```

**TypeScript** (custom Error classes):

```typescript
export class GrpcInvalidArgumentError extends Error {
  status = "INVALID_ARGUMENT";
}

export class GrpcPermissionDeniedError extends Error {
  status = "PERMISSION_DENIED";
}
```

**Ruby** (exception hierarchy):

```ruby
module Spikard::Grpc
  class Error < StandardError
    def grpc_status; 'INTERNAL'; end
  end

  class InvalidArgumentError < Error
    def grpc_status; 'INVALID_ARGUMENT'; end
  end
end
```

### Memory Management

**Python (PyO3)**:

- GIL handling: Acquire only when needed
- Bytes to/from Python: `PyBytes::new()` copies, `as_bytes()` borrows
- Reference counting: PyO3 handles automatically

**Ruby (Magnus)**:

- GC marking: Custom `mark()` functions for Rust-owned Ruby values
- String handling: `str_from_slice()` creates Ruby String
- Opaque values for storing Ruby objects in Rust structs

**Node.js (napi-rs)**:

- V8 isolate management: Automatic via napi-rs
- Buffer handling: `Buffer::from()` for zero-copy where possible
- Threadsafe functions for async callbacks

**PHP (ext-php-rs)**:

- Zend engine integration: Manual refcount management
- String handling: `ZendStr` wrapper
- Object lifecycle tied to PHP GC

## Consequences

**Benefits**:

- **Single Runtime**: One Tonic-based runtime serves all languages
- **Type Safety**: Rust compiler catches FFI boundary issues
- **Performance**: Minimal FFI overhead (binary payloads, no parsing)
- **Consistency**: Same behavior across all language bindings
- **Maintainability**: Shared logic in `spikard-bindings-shared`

**Trade-offs**:

- **Complex FFI Code**: Each language requires custom bridging
- **Testing Burden**: Must test each language binding independently
- **Async Challenges**: Blocking languages (Ruby, PHP) require thread pool
- **Version Compatibility**: Must track FFI framework versions (PyO3, Magnus, etc.)

**Performance Characteristics**:

- **FFI Call Overhead**: ~50-100ns per boundary crossing
- **Binary Data**: Zero-copy in Rust, copy required for language objects
- **Metadata Conversion**: O(n) HashMap construction per request/response
- **GIL Impact (Python)**: Minimal - only held during FFI calls, released during Rust async

**Security**:

- No unsafe code in FFI layer except for controlled lifetime extensions
- All user input (payloads, metadata) treated as untrusted
- Panic shielding prevents Rust panics from crashing language runtime

**Known Limitations**:

- **Streaming**: Not yet implemented for any language
- **Compression**: gRPC compression not exposed via FFI
- **Deadlines/Timeouts**: Not propagated across FFI boundary
- **Custom Metadata Types**: Only string key-value pairs supported

## References

- gRPC runtime: `crates/spikard-http/src/grpc/`
- Python bindings: `crates/spikard-py/src/` (gRPC support in progress)
- Node.js bindings: `crates/spikard-node/src/` (gRPC support in progress)
- Ruby bindings: `packages/ruby/ext/spikard_rb/native/src/` (gRPC support in progress)
- PHP bindings: `crates/spikard-php/src/` (gRPC support in progress)
- Test fixtures: `testing_data/protobuf/` (fixture-driven gRPC test coverage)
