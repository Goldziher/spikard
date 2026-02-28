<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# Spikard

High-performance HTTP framework built on Axum and Tower-HTTP with type-safe routing, validation, WebSocket/SSE support, and lifecycle hooks.

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
- **Type-safe routing** with path parameter extraction and compile-time validation
- **JSON Schema validation** via schemars with automatic OpenAPI generation
- **WebSocket and SSE** (Server-Sent Events) support
- **Lifecycle hooks** with zero-cost abstraction (onRequest, preValidation, preHandler, onResponse, onError)
- **Tower middleware** stack (compression, rate limiting, auth, CORS, request IDs, timeouts)
- **OpenAPI 3.1** and AsyncAPI generation with Swagger UI and ReDoc
- **Testing utilities** with in-memory test server
- **File upload** handling with multipart form support
- **Streaming responses** with native async/await
- **Multi-language bindings** (Python, Node.js, Ruby, PHP, Elixir)

## Installation

```toml
[dependencies]
spikard = "0.12.0"
```

## Quick Start

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::prelude::*;

#[derive(Deserialize, Serialize, JsonSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: Context| async move {
        let id: u64 = ctx.path_param("id").unwrap_or("0").parse().unwrap_or(0);
        Ok(Json(User {
            id,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }))
    })?;

    app.route(
        post("/users")
            .request_body::<User>()
            .response_body::<User>(),
        |ctx: Context| async move {
            let user: User = ctx.json()?;
            Ok(Json(user))
        },
    )?;

    app.run().await?;
    Ok(())
}
```

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](../../docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard-rust** | 55,755 | 1.0 | 1.0 |
| spikard-node | 24,283 | 2.22 | 2.22 |
| spikard-php | 20,176 | 2.66 | 2.66 |
| spikard-python | 11,902 | 4.41 | 4.41 |
| spikard-ruby | 8,271 | 6.5 | 6.5 |

Spikard is **the fastest binding, delivering native Rust performance as the reference implementation**.

## Related Crates

- [spikard-http](../spikard-http/README.md) - HTTP runtime
- [spikard-core](../spikard-core/README.md) - Core primitives
- [spikard-cli](../spikard-cli/README.md) - Command-line interface

## Documentation

- [Main Project README](../../README.md)
- [Full API Documentation](https://docs.rs/spikard)
- [spikard.dev](https://spikard.dev)

## License

MIT - See [LICENSE](LICENSE) for details
