# spikard-http

High-performance HTTP server for Spikard with a complete tower-http middleware stack, JSON Schema validation, and cross-language handler execution.

## Status & Badges

[![Crates.io](https://img.shields.io/crates/v/spikard-http.svg)](https://crates.io/crates/spikard-http)
[![Downloads](https://img.shields.io/crates/d/spikard-http.svg)](https://crates.io/crates/spikard-http)
[![Documentation](https://docs.rs/spikard-http/badge.svg)](https://docs.rs/spikard-http)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **Axum-based routing** - Fast, ergonomic router with zero-allocation path matching
- **Tower middleware stack** - Compression (gzip/brotli), rate limiting, timeouts, CORS, request IDs, auth
- **JSON Schema validation** - Request/response validation against JSON schemas
- **Cross-language handlers** - Handler trait for Python, Node.js, Ruby, PHP, and WASM bindings
- **OpenAPI generation** - Automatic OpenAPI 3.1 and AsyncAPI spec generation
- **WebSocket & SSE support** - Real-time bidirectional and server-sent event communication
- **Graceful shutdown** - Proper shutdown handling with in-flight request completion
- **Static file serving** - Efficient static file serving with caching support

## Architecture

### Components

- `router` - Translates route metadata into strongly typed Rust handlers with path extraction
- `validation` - Enforces JSON schemas for headers, cookies, query params, and request bodies
- `server` - Wraps Axum/Tokio bootstrapping and exposes configuration via `ServerConfig`
- `handler` - Language-agnostic Handler trait for FFI integration
- `middleware` - Tower middleware stack with sensible defaults

## Performance

Native Rust implementation using Axum and tower-http middleware. Benchmarks on macOS (Darwin 24.6.0) with 50 concurrent connections:

| Workload | Throughput | Mean Latency | P95 Latency | P99 Latency | Memory |
|----------|------------|--------------|-------------|-------------|--------|
| Baseline | 165,228 req/s | 0.30ms | 0.36ms | 0.45ms | 17.4 MB |
| JSON Bodies | *pending* | *pending* | *pending* | *pending* | *pending* |
| Multipart Forms | *pending* | *pending* | *pending* | *pending* | *pending* |
| URL-Encoded | *pending* | *pending* | *pending* | *pending* | *pending* |

**Architecture Highlights:**
- **Zero-overhead abstraction**: Handler trait with `Pin<Box<dyn Future>>` enables language-agnostic integration
- **Tower middleware stack**: Compression, rate limiting, timeouts, and CORS with minimal latency impact
- **Efficient routing**: Axum's path matching with zero allocations for static routes
- **Low memory baseline**: ~17 MB with efficient memory pooling and minimal allocations

The native Rust implementation provides ~38% higher throughput compared to Python bindings while maintaining even lower latency characteristics. Startup time averages 1.01s with first response in 908ms.

Full benchmark methodology: `tools/benchmark-harness/`

## Installation

```toml
[dependencies]
spikard-http = "0.10.0"
tokio = { version = "1", features = ["full"] }
axum = "0.8"
```

### Optional Features

```toml
[dependencies]
spikard-http = { version = "0.10.0", features = ["di"] }
```

- `di` - Enables dependency injection support

## Quick Start

```rust
use spikard_http::{ServerConfig, start_server};
use spikard_core::{RouteConfig, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: 8080,
        ..Default::default()
    };

    // Create routes with schemas
    let routes = vec![
        RouteConfig::get("/health")
            .handler("health", |_req| async {
                Ok(Response::new(200).with_body(r#"{"status": "ok"}"#))
            }),
    ];

    start_server(config, routes).await?;
    Ok(())
}
```

## Middleware Stack

The default middleware stack (in order):

1. **Compression** - gzip/brotli compression (configurable)
2. **Request ID** - Unique request tracking
3. **Timeout** - Request timeout enforcement
4. **Rate Limit** - Per-IP rate limiting (if configured)
5. **Authentication** - JWT/Bearer token validation (if configured)
6. **User-Agent** - User agent parsing and validation
7. **CORS** - Cross-origin resource sharing (if configured)
8. **Handler** - Your application logic

See `ServerConfig` documentation for detailed configuration options.

## Validation

Validate requests against JSON schemas:

```rust
use spikard_http::validation::ValidateRequest;
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "integer", "minimum": 0 }
    },
    "required": ["name"]
});

request.validate_body(&schema)?;
```

## Development

- Build with `cargo build -p spikard-http` or `task build:http`
- Execute tests and fixture validations via `cargo test -p spikard-http`
- Benchmarks: `tools/benchmark-harness/`
- When altering schemas, sync the Python fixtures and regenerate bindings before rerunning the CLI

## Related Crates

- [spikard](../spikard/README.md) - High-level HTTP framework
- [spikard-core](../spikard-core/README.md) - Core primitives
- [spikard-py](../spikard-py/README.md) - Python bindings

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-http)
- [Architecture Decision Records](../../docs/adr/0002-runtime-and-middleware.md)

## License

MIT
