# Rust API Reference

Use the Rust API when you want zero-overhead access to the runtime without crossing an FFI boundary.

## Crate
- Add: `cargo add spikard`
- Import route builders and types directly from `spikard`

## Core Types
- `App` – build your router and middleware stack
- `RequestContext` – access path/query/header/cookie/body data
- Response helpers such as `HandlerResponse::stream`, plus `axum` response types for JSON/streaming in your handlers
- Dependency injection via `ServerConfig::provide_value` / `provide_factory` and `RequestContext::dependencies`

## Routing
```rust
use axum::response::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::{get, post, App, RequestContext};

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/health"), |_ctx: RequestContext| async move {
        let body = serde_json::json!({"status": "ok"});
        Ok(Json(body).into())
    })?;

    app.route(
        post("/users").request_body::<User>().response_body::<User>(),
        |ctx: RequestContext| async move {
            let user: User = ctx.json()?;
            Ok(Json(user).into())
        },
    )?;

    app.run().await?;
    Ok(())
}
```

## Middleware
Attach middleware via `ServerConfig` or by adding layers to the Axum router produced by `App::into_router`.

## Validation
Integrate serde-based DTOs with JSON Schema derivation to keep contracts aligned with other bindings and code generators.
