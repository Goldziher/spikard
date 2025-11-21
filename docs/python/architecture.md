# Python Binding Architecture

This document describes the architecture of Spikard's Python bindings, focusing on how Python handlers integrate with the Rust-powered HTTP server.

## Overview

Spikard's Python bindings use PyO3 to provide a Pythonic API for building web services while leveraging Rust's performance for HTTP handling. The core design principle is **Rust drives, Python handles**: the Rust layer manages the HTTP server, routing, and middleware, while Python code implements business logic.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Spikard Application                       │
│                  (packages/python/spikard/)                  │
│                                                               │
│  @app.get("/users")                                          │
│  async def get_users() -> list[User]:                        │
│      return await db.fetch_all_users()                       │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            │ PyO3 FFI Boundary
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Rust HTTP Server Layer                     │
│                  (crates/spikard-py/src/)                    │
│                                                               │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Axum Server (Main Thread - Tokio Runtime)            │  │
│  │  • HTTP request parsing                                │  │
│  │  • Routing & middleware                                │  │
│  │  • Connection management                               │  │
│  └─────────────┬──────────────────────────┬───────────────┘  │
│                │                          │                   │
│                ▼                          ▼                   │
│  ┌──────────────────────┐  ┌──────────────────────────────┐ │
│  │ Blocking Thread Pool │  │ Python Event Loop Thread     │ │
│  │ (tokio::spawn_       │  │ (Dedicated Daemon)           │ │
│  │  blocking)           │  │                              │ │
│  │                      │  │ • asyncio.run_forever()      │ │
│  │ • Acquire GIL        │  │ • uvloop integration         │ │
│  │ • Call Python        │  │ • Executes async handlers    │ │
│  │ • Submit coroutines  │  │                              │ │
│  │ • Wait for results   │  │                              │ │
│  └──────────────────────┘  └──────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Threading Model

Spikard uses a multi-threaded architecture with specialized thread types:

### 1. Main Thread (Tokio Runtime)
- Runs the Axum HTTP server
- Handles incoming connections and HTTP parsing
- Manages request routing and middleware execution
- Delegates to blocking threads for Python handler invocation

### 2. Blocking Thread Pool
- Tokio's `spawn_blocking` thread pool
- Used for all Python handler invocations (sync and async)
- Acquires the GIL to call Python code
- For async handlers: submits coroutines to the event loop thread

### 3. Python Event Loop Thread (Dedicated Daemon)
- Created at server startup in `init_python_event_loop()` (handler.rs:34-73)
- Runs `asyncio.run_forever()` in an infinite loop
- Uses uvloop if available for enhanced performance
- Executes all async Python handler coroutines
- Thread-safe via `asyncio.run_coroutine_threadsafe()`

## Request Flow

### Synchronous Handler Flow

```
HTTP Request
    ↓
Axum Router (Tokio)
    ↓
tokio::spawn_blocking()
    ↓
Acquire GIL
    ↓
Call Python Handler → handler(**kwargs)
    ↓
Return Value
    ↓
Convert to Rust ResponseResult
    ↓
Release GIL
    ↓
HTTP Response
```

**Implementation**: `handler.rs:242-272`

### Asynchronous Handler Flow

```
HTTP Request
    ↓
Axum Router (Tokio)
    ↓
tokio::spawn_blocking()
    ↓
Acquire GIL
    ↓
Call Python Handler → Returns Coroutine
    ↓
asyncio.run_coroutine_threadsafe(coro, loop)
    ↓                              │
    │                              ▼
    │                    Python Event Loop Thread
    │                    Executes Coroutine
    │                              │
    │◄─────────────────────────────┘
    │      concurrent.futures.Future
    ↓
future.result() (blocks, GIL released)
    ↓
Convert to Rust ResponseResult
    ↓
Release GIL
    ↓
HTTP Response
```

**Implementation**: `handler.rs:177-241`

**Key Code Locations**:
- Event loop initialization: `handler.rs:34-73`
- Async handler execution: `handler.rs:177-241`
- Coroutine submission: `handler.rs:212-213`
- Result blocking: `handler.rs:216`

## Key Components

### 1. Event Loop Management

The Python event loop is created once at server startup and runs continuously in a daemon thread:

```rust
// crates/spikard-py/src/handler.rs:34-73
pub fn init_python_event_loop() -> PyResult<()> {
    Python::attach(|py| {
        // Install uvloop if available
        if let Ok(uvloop) = py.import("uvloop") {
            uvloop.call_method0("install")?;
        }

        // Create event loop
        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;

        // Store globally
        PYTHON_EVENT_LOOP.set(event_loop.into())?;

        // Start daemon thread running loop.run_forever()
        let threading = py.import("threading")?;
        // ... thread creation code ...
    })
}
```

### 2. Coroutine Submission

Async handlers submit coroutines to the event loop thread using Python's thread-safe API:

```rust
// crates/spikard-py/src/handler.rs:212-216
let future = asyncio.call_method1(
    "run_coroutine_threadsafe",
    (coroutine, loop_obj)
)?;

// Blocks until coroutine completes, GIL is released during wait
let result = future.call_method0("result")?;
```

This uses Python's `asyncio.run_coroutine_threadsafe()` which returns a `concurrent.futures.Future`. The calling thread blocks on `future.result()`, releasing the GIL while waiting.

### 3. Zero-Copy JSON Conversion

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

**Reference**: See `docs/design/msgspec-type-conversion.md` for detailed type mappings.

## Performance Characteristics

### Advantages

✅ **Single Persistent Event Loop**: Created once at startup, eliminating per-request event loop creation overhead (30-40% improvement over previous `asyncio.run()` approach)

✅ **uvloop Integration**: Automatically uses uvloop when available for high-performance async I/O operations

✅ **Zero-Copy JSON**: Direct Python object construction from `serde_json::Value` without JSON string round-trips

✅ **Rust HTTP Layer**: All HTTP parsing, routing, and middleware handled by high-performance Rust code

✅ **Proper GIL Management**: GIL is released during async coroutine execution and I/O waits

### Trade-offs

⚠️ **Thread Pool Overhead**: Each request uses `spawn_blocking`, incurring thread scheduling overhead

⚠️ **GIL Contention**: Multiple blocking threads compete for GIL when calling Python code

⚠️ **Cross-Thread Communication**: `run_coroutine_threadsafe()` involves queue operations and thread synchronization

⚠️ **Blocking Wait**: Calling thread blocks on `future.result()` even though async work happens elsewhere

## Comparison to Alternatives

### Current Architecture: Rust-Driven with Event Loop Thread
```
Pros:
• Rust handles HTTP efficiently
• Single persistent event loop
• Works with existing Python async code
• uvloop integration

Cons:
• Thread pool overhead per request
• GIL contention
• Cross-thread communication cost
```

### Alternative 1: ASGI (Python-Driven)
```
Pros:
• Python manages its own event loop naturally
• No cross-thread communication
• Direct async integration
• Ecosystem compatibility (ASGI middleware)

Cons:
• Python drives HTTP (slower parsing/routing)
• Loses Rust's HTTP performance benefits
• More complex integration with Rust middleware
```

### Alternative 2: RSGI (Hybrid)
```
Pros:
• Rust HTTP layer with Python event loop integration
• Balance of performance and Python async support
• Could match ASGI interface

Cons:
• Requires significant architectural changes
• New standard to define and maintain
```

## Future Directions

The current architecture is stable and functional. Potential future improvements:

1. **ASGI/RSGI Support**: Add deployment modes where Python manages the event loop for async-heavy workloads
   - Status: Documented as future option in README
   - Use case: Applications with heavy async I/O (database queries, external APIs)

2. **GIL-Free Python**: When PEP 703 (free-threaded Python) becomes stable, eliminate GIL contention entirely
   - Status: Experimental in Python 3.13+
   - Impact: Would remove one of the main performance bottlenecks

3. **Async Runtime Bridge**: Use `pyo3_async_runtimes` to directly convert Python coroutines to Tokio futures
   - Status: Attempted but requires Python-driven event loop
   - Blocked by: Current Rust-driven architecture

## Code References

### Rust Implementation
- **Event loop initialization**: `crates/spikard-py/src/handler.rs:34-73`
- **Async handler execution**: `crates/spikard-py/src/handler.rs:177-241`
- **Sync handler execution**: `crates/spikard-py/src/handler.rs:242-272`
- **JSON conversion**: `crates/spikard-py/src/handler.rs:json_to_python()`
- **Python handler trait**: `crates/spikard-py/src/handler.rs:PythonHandler`
- **Server entry point**: `crates/spikard-py/src/lib.rs:run_server()`

### Python API
- **Application class**: `packages/python/spikard/app.py:Spikard`
- **Route decorators**: `packages/python/spikard/decorators.py`
- **Type system**: `packages/python/spikard/types.py`
- **Configuration**: `packages/python/spikard/config.py:ServerConfig`

## Related Documentation

- **Overall Architecture**: `docs/design/architecture.md`
- **Type Conversion**: `docs/design/msgspec-type-conversion.md`
- **Middleware & Lifecycle**: `docs/design/middleware-lifecycle-optimization.md`
- **Validation Strategy**: `docs/design/validation-strategy.md`
