# Rust Binding

Use Spikard natively for zero-FFI routing, validation, and streaming. The Rust API mirrors binding ergonomics while exposing full Axum/Tower power.

## Quickstart

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::prelude::*;

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: Context| async move {
        let id: i32 = ctx.path_param("id").unwrap_or("0").parse().unwrap_or_default();
        Ok(Json(User { id, name: "Alice".into() }))
    })?;

    app.route(
        post("/users").request_body::<User>().response_body::<User>(),
        |ctx: Context| async move {
            let user: User = ctx.json()?;
            Ok(Json(user))
        },
    )?;

    app.run().await?;
    Ok(())
}
```

## Validation
- Derive `JsonSchema` (schemars) to register request/response schemas.
- Accept raw JSON Schema via `request_schema_json` when needed.

## Middleware & Hooks
- Add Tower layers via `.layer(...)` or use `ServerConfig` lifecycle hooks for request/response interception.
- WebSockets: `app.websocket("/ws", handler)`; SSE via `app.sse`.

## Testing
- Use `spikard::testing::TestServer` for in-memory integration tests.

## Deployment
- Add `spikard` to your Cargo manifest and run with `cargo run`.
- Configure host/port/timeouts via `ServerConfig`.
