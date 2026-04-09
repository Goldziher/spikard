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
spikard = "0.13.0"
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

## Route Registration

### RouteBuilder API

```rust
use spikard::{get, post, put, patch, delete};
use schemars::JsonSchema;

#[derive(JsonSchema)]
struct UserParams {
    id: u64,
}

#[derive(JsonSchema)]
struct CreateUser {
    name: String,
    email: String,
}

let route = get("/users/:id")
    .handler_name("get_user_by_id")
    .params::<UserParams>();

let create_route = post("/users")
    .request_body::<CreateUser>()
    .response_body::<User>();
```

### With Raw JSON Schema

```rust
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "email": { "type": "string", "format": "email" }
    },
    "required": ["name", "email"]
});

let route = post("/users")
    .request_schema_json(schema);
```

## Request Context

Access request data in handlers:

```rust
use spikard::prelude::*;

async fn handler(ctx: Context) -> HandlerResult {
    // Parse JSON body
    let body: MyStruct = ctx.json()?;

    // Query parameters
    let query: QueryParams = ctx.query()?;

    // Path parameters
    let id = ctx.path_param("id").unwrap();
    let path_data: PathParams = ctx.path()?;

    // Headers
    let auth = ctx.header("authorization");

    // Cookies
    let session = ctx.cookie("session_id");

    // Request metadata
    let method = ctx.method();
    let path = ctx.path_str();

    Ok(Json(body))
}
```

## Configuration

```rust
use spikard::{
    App, ServerConfig, CompressionConfig, RateLimitConfig,
    JwtConfig, StaticFilesConfig, OpenApiConfig
};

let config = ServerConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    workers: 4,
    enable_request_id: true,
    max_body_size: Some(10 * 1024 * 1024),
    request_timeout: Some(30),
    compression: Some(CompressionConfig {
        gzip: true,
        brotli: true,
        min_size: 1024,
        quality: 6,
    }),
    rate_limit: Some(RateLimitConfig {
        per_second: 100,
        burst: 200,
        ip_based: true,
    }),
    jwt_auth: Some(JwtConfig {
        secret: "your-secret".to_string(),
        algorithm: "HS256".to_string(),
        audience: None,
        issuer: None,
        leeway: 0,
    }),
    static_files: vec![
        StaticFilesConfig {
            directory: "./public".to_string(),
            route_prefix: "/static".to_string(),
            index_file: true,
            cache_control: Some("public, max-age=3600".to_string()),
        }
    ],
    openapi: Some(OpenApiConfig {
        enabled: true,
        title: "My API".to_string(),
        version: "1.0.0".to_string(),
        description: Some("API documentation".to_string()),
        swagger_ui_path: "/docs".to_string(),
        redoc_path: "/redoc".to_string(),
        ..Default::default()
    }),
    ..Default::default()
};

let app = App::new().config(config);
```

## Lifecycle Hooks

```rust
use spikard::{LifecycleHooks, request_hook, response_hook, HookResult};
use std::sync::Arc;

let hooks = LifecycleHooks::builder()
    .on_request(request_hook("logger", |req| async move {
        println!("Request: {} {}", req.method(), req.uri());
        Ok(HookResult::Continue(req))
    }))
    .pre_validation(request_hook("auth", |req| async move {
        // Authentication check
        Ok(HookResult::Continue(req))
    }))
    .pre_handler(request_hook("rate_limit", |req| async move {
        // Rate limiting
        Ok(HookResult::Continue(req))
    }))
    .on_response(response_hook("headers", |mut resp| async move {
        resp.headers_mut().insert(
            "X-Frame-Options",
            axum::http::HeaderValue::from_static("DENY")
        );
        Ok(HookResult::Continue(resp))
    }))
    .on_error(response_hook("error_log", |resp| async move {
        eprintln!("Error: {}", resp.status());
        Ok(HookResult::Continue(resp))
    }))
    .build();

let config = ServerConfig {
    lifecycle_hooks: Some(Arc::new(hooks)),
    ..Default::default()
};
```

## WebSockets

```rust
use spikard::WebSocketHandler;
use serde_json::Value;

struct EchoHandler;

impl WebSocketHandler for EchoHandler {
    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send {
        async move { Some(message) } // Echo back
    }

    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {
            println!("Client connected");
        }
    }

    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        async {
            println!("Client disconnected");
        }
    }
}

app.websocket("/ws", EchoHandler);
```

## Server-Sent Events

```rust
use spikard::{SseEventProducer, SseEvent};
use serde_json::json;

struct TickerProducer {
    count: std::sync::atomic::AtomicU64,
}

impl SseEventProducer for TickerProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        let n = self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if n < 10 {
            Some(SseEvent::new(json!({"tick": n})))
        } else {
            None
        }
    }
}

app.sse("/events", TickerProducer {
    count: std::sync::atomic::AtomicU64::new(0),
});
```

## File Uploads

```rust
use spikard::UploadFile;
use serde::Deserialize;

#[derive(Deserialize)]
struct UploadRequest {
    file: UploadFile,
    description: String,
}

async fn upload_handler(ctx: Context) -> HandlerResult {
    let upload: UploadRequest = ctx.json()?;
    let content = upload.file.as_bytes();
    let filename = &upload.file.filename;

    // Process upload...

    Ok(/* response */)
}
```

## Testing

```rust
use spikard::testing::TestServer;
use axum::http::Request;

#[tokio::test]
async fn test_api() {
    let mut app = App::new();
    // ... configure routes

    let server = TestServer::from_app(app).unwrap();

    let request = Request::builder()
        .uri("http://localhost/users")
        .method("GET")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = server.call(request).await.unwrap();
    assert_eq!(response.status, 200);

    let json = response.json().unwrap();
    // assertions...
}
```

## Integration with Axum

Merge custom Axum routers:

```rust
use axum::{Router, routing::get};

async fn health() -> &'static str {
    "OK"
}

let custom_router = Router::new()
    .route("/health", get(health));

let app = App::new()
    .merge_axum_router(custom_router);
```

## Language Bindings

Spikard is available for multiple languages:

### Python

```bash
pip install spikard
```

See [spikard-py](../spikard-py/README.md) for details.

### Node.js / TypeScript

```bash
npm install spikard
```

See [spikard-node](../spikard-node/README.md) for details.

### Ruby

```bash
gem install spikard
```

See [spikard-rb](../spikard-rb/README.md) for details.

### PHP

```bash
composer require spikard/spikard
```

See [spikard-php](../spikard-php/README.md) for details.

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

MIT - See [LICENSE](../../LICENSE) for details
