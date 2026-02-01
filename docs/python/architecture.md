# Python Binding Architecture

This document describes the architecture of Spikard's Python bindings, focusing on how Python handlers integrate with the Rust-powered HTTP server.

## Overview

Spikard's Python bindings use PyO3 to provide a Pythonic API for building web services while leveraging Rust's performance for HTTP handling. The core design principle is **Rust drives, Python handles**: the Rust layer manages the HTTP server, routing, and middleware, while Python code implements business logic.

## Architecture Diagram

```
+-------------------------------------------------------------+
|                    Spikard Application                       |
|                  (packages/python/spikard/)                  |
|                                                               |
|  @app.get("/users")                                          |
|  async def get_users() -> list[User]:                        |
|      return await db.fetch_all_users()                       |
+---------------------------+----------------------------------+
                            |
                            | PyO3 FFI Boundary
                            v
+-------------------------------------------------------------+
|                   Rust HTTP Server Layer                     |
|                  (crates/spikard-py/src/)                    |
|                                                               |
|  +-------------------------------------------------------+  |
|  |  Axum Server (Tokio Runtime)                           |  |
|  |  * HTTP request parsing                                |  |
|  |  * Routing & middleware                                |  |
|  |  * Connection management                               |  |
|  +---------------------------+---------------------------+  |
|                              |                               |
|                              v                               |
|  +-------------------------------------------------------+  |
|  |  pyo3_async_runtimes Bridge                            |  |
|  |  * Converts Python coroutines to Tokio futures         |  |
|  |  * TaskLocals stored in OnceCell (init once)           |  |
|  |  * GIL released during Rust future execution           |  |
|  +-------------------------------------------------------+  |
+-------------------------------------------------------------+
```

## Async Model: pyo3_async_runtimes

The current architecture uses `pyo3_async_runtimes::tokio::into_future()` to convert Python coroutines directly into Rust futures on the Tokio runtime. This eliminates the previous design's dedicated event loop thread and `run_coroutine_threadsafe` overhead.

### Key Design:
- **TaskLocals** are initialized once in a `OnceCell` at server startup
- Python coroutines are converted to Tokio futures via `into_future()`
- The GIL is acquired only to call into Python, then released during async I/O
- No dedicated Python event loop thread needed

## Request Flow

### Asynchronous Handler Flow

```
HTTP Request
    |
Axum Router (Tokio)
    |
Acquire GIL briefly
    |
Call Python handler -> returns coroutine
    |
pyo3_async_runtimes::into_future(coroutine)
    |
Tokio executes the future (GIL released)
    |
Result returned to Rust
    |
HTTP Response
```

### Synchronous Handler Flow

```
HTTP Request
    |
Axum Router (Tokio)
    |
tokio::task::spawn_blocking
    |
Acquire GIL -> Call handler -> Release GIL
    |
HTTP Response
```

## Key Components

### 1. Zero-Copy JSON Conversion

For performance, JSON values are converted directly from `serde_json::Value` to Python objects without intermediate string serialization:

```rust
// crates/spikard-py/src/handler.rs:json_to_python()
match value {
    Value::Object(map) => {
        let dict = PyDict::new(py);
        for (k, v) in map {
            dict.set_item(k, json_to_python(py, v)?)?;
        }
        Ok(dict.into())
    }
    // ... other types ...
}
```

### 2. Dependency Injection

Type-based DI is the recommended pattern. Dependencies keyed by type are stored as `__type__module.qualname` strings. The routing layer normalizes handler parameter type annotations to match these keys, enabling automatic injection.

## Performance Characteristics

### Advantages

- **Direct coroutine conversion**: `into_future()` eliminates cross-thread communication overhead
- **Single Tokio runtime**: No separate Python event loop thread needed
- **Zero-Copy JSON**: Direct Python object construction from `serde_json::Value`
- **Rust HTTP Layer**: All HTTP parsing, routing, and middleware in Rust
- **Proper GIL Management**: GIL released during async I/O operations

### Trade-offs

- **GIL acquisition**: Still needed when calling into Python code
- **spawn_blocking for sync handlers**: Sync handlers use blocking thread pool

## Code References

### Rust Implementation
- **Async handler execution**: `crates/spikard-py/src/handler.rs`
- **JSON conversion**: `crates/spikard-py/src/handler.rs:json_to_python()`
- **Python handler trait**: `crates/spikard-py/src/handler.rs:PythonHandler`
- **Server entry point**: `crates/spikard-py/src/lib.rs:run_server()`

### Python API
- **Application class**: `packages/python/spikard/app.py:Spikard`
- **Router**: `packages/python/spikard/routing.py:Router`
- **DI**: `packages/python/spikard/di.py:Provide`
- **Testing**: `packages/python/spikard/testing.py:TestClient`
- **Configuration**: `packages/python/spikard/config.py:ServerConfig`

## Related Documentation

- **ADR: Validation & Fixtures**: `docs/adr/0003-validation-and-fixtures.md`
- **ADR: Runtime & Middleware**: `docs/adr/0002-runtime-and-middleware.md`
- **ADR: Lifecycle Hooks**: `docs/adr/0005-lifecycle-hooks.md`
