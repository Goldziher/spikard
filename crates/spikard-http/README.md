<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# spikard-http

High-performance HTTP server for Spikard with a complete tower-http middleware stack, JSON Schema validation, and cross-language handler execution.

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

## Features
- **Axum-based routing** with zero-allocation path matching
- **Tower middleware stack**: compression, rate limiting, timeouts, CORS, request IDs, auth
- **JSON Schema validation** for request/response
- **Cross-language handlers** via the Handler trait (Python, Node.js, Ruby, PHP, Elixir)
- **OpenAPI 3.1** and AsyncAPI spec generation
- **WebSocket and SSE** support
- **Graceful shutdown** with in-flight request completion
- **Static file serving** with caching

## Installation

```toml
[dependencies]
spikard-http = "0.13.0"
```

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

## Related Crates

- [spikard](../spikard/README.md) - High-level HTTP framework
- [spikard-core](../spikard-core/README.md) - Core primitives

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard-http)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](../../LICENSE) for details
