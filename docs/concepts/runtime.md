# Runtime Model

The runtime is designed to keep language bindings thin while centralizing concurrency, IO, and lifecycle management in Rust.

## Execution Threads
- **Main Tokio runtime** – accepts connections, runs Axum router + middleware, and orchestrates background tasks.
- **Binding bridges** – per-language glue that converts requests/responses and coordinates async execution (e.g., Python event loop thread).
- **Worker pools** – CPU-bound tasks can be offloaded via `spawn_blocking` or binding-specific pools to keep the async reactor responsive.

## Lifecycle
1. **Bootstrap**: CLI or host app creates an `App` and registers routes/middleware.
2. **Serve**: Runtime binds to a socket, initializes tracing/logging, and configures graceful shutdown.
3. **Handle**: Each request passes through middleware, validation, and the binding bridge before reaching user code.
4. **Shutdown**: Drains in-flight requests, flushes telemetry, and tears down binding-specific resources.

## Configuration Surface
- **Server**: host, port, TLS (planned), concurrency limits, timeouts.
- **Middleware**: logging, tracing, auth, CORS, compression, and custom layers.
- **Validation**: JSON Schema enforcement with language-native type hints and DTOs.
- **Codegen hooks**: generate DTOs/handlers and keep fixtures in sync with OpenAPI/AsyncAPI specs.

More detail on the binding side lives in [Runtime and Middleware ADR](../adr/0002-runtime-and-middleware.md).
