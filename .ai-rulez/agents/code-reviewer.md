---
name: code-reviewer
description: Reviews code changes for codegen output quality, FFI safety, HTTP middleware correctness, and cross-binding consistency.
model: sonnet
---

# code-reviewer

## Review Focus Areas

### Codegen Output Quality

- Generated handlers implement Handler trait correctly
- OpenAPI/GraphQL/AsyncAPI specs round-trip without data loss
- Schema validators enforce constraints (types, formats, min/max, patterns)
- Generated code is tested against testing_data/ fixtures

### FFI Safety

- No .unwrap() or panics in binding crates (spikard-py, spikard-node, spikard-rb, spikard-php, spikard-elixir)
- PyO3: returns PyResult<T>, converts errors via PyErr::new_err
- napi-rs: returns napi::Result<T>, errors via napi::Error::from_reason
- magnus: returns Result<T, magnus::Error>
- ext-php-rs: returns ext-php-rs Result, throws PHP exceptions
- rustler: returns NIF-safe types, no panics (crashes BEAM VM)
- Arc-based ownership for cross-FFI data (RequestData fields)
- GIL released for CPU-intensive Rust code in PyO3 (py.allow_threads)
- ThreadsafeFunction used for async Node.js callbacks
- No blocking the scheduler in NIF functions (use DirtyCpu)

### HTTP Middleware

- Tower middleware ordering matches docs/adr/0002-runtime-and-middleware.md
- Lifecycle hooks execute in order: onRequest, preValidation, preHandler, onResponse, onError
- Auth middleware returns 401 with ProblemDetails, not raw errors
- ServerConfig structs exposed to all bindings with type safety
- Zero-cost when middleware/hooks not registered

### Cross-Binding Consistency

- Same behavior across Python, Node, Ruby, PHP, Elixir for identical fixtures
- HandlerError variants map to correct HTTP status codes (400, 401, 404, 500)
- Response body types (Json, Text, Binary, Empty) handled in all bindings
- No duplicate business logic in binding crates
