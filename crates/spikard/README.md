# Spikard (Rust)

[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)
[![Crates.io](https://img.shields.io/crates/v/spikard)](https://crates.io/crates/spikard)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance HTTP framework built on Axum and Tower-HTTP with type-safe routing, validation, WebSocket/SSE support, and lifecycle hooks.

## Installation

```toml
[dependencies]
spikard = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
schemars = "0.8"  # For JSON Schema generation
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use spikard::{App, get, post, RequestContext, HandlerResult};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use axum::http::{Response, StatusCode};

#[derive(Deserialize, Serialize, JsonSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

async fn get_user(ctx: RequestContext) -> HandlerResult {
    let id: u64 = ctx.path_param("id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let user = User {
        id,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json))
        .unwrap())
}

async fn create_user(ctx: RequestContext) -> HandlerResult {
    let user: User = ctx.json()?;
    let json = serde_json::to_string(&user).unwrap();
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json))
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), get_user)?;
    app.route(
        post("/users")
            .request_body::<User>()
            .response_body::<User>(),
        create_user
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

## RequestContext

Access request data in handlers:

```rust
use spikard::RequestContext;

async fn handler(ctx: RequestContext) -> HandlerResult {
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

    Ok(/* ... */)
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

async fn upload_handler(ctx: RequestContext) -> HandlerResult {
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

## Running

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    app.run().await?;
    Ok(())
}
```

## Type Safety

All handlers use `RequestContext` and return `HandlerResult`:

```rust
pub type HandlerResult = Result<Response<Body>, (StatusCode, String)>;

pub trait IntoHandler {
    fn into_handler(self) -> Arc<dyn Handler>;
}
```

Handlers can be async functions or closures:

```rust
async fn handler(ctx: RequestContext) -> HandlerResult { /* ... */ }

|ctx: RequestContext| async move { /* ... */ }
```

## Features

- **Type-safe routing** with path parameter extraction
- **JSON Schema validation** via schemars
- **WebSocket and SSE** support
- **Lifecycle hooks** with zero-cost abstraction
- **Tower middleware** (compression, rate limiting, auth, CORS, etc.)
- **OpenAPI 3.1** generation
- **Testing utilities** with in-memory server
- **File upload** handling
- **Streaming responses**

## Performance

Built on:
- **Axum** for routing and handlers
- **Tower-HTTP** for middleware
- **Tokio** for async runtime
- **jsonschema** for validation
- Zero-copy where possible

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)
- [API Documentation](https://docs.rs/spikard) *(coming soon)*

## Examples

See `/examples/rust/` for more examples.

## License

MIT
