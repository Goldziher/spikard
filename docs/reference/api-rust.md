# Rust API Reference

Use the Rust API when you want zero-overhead access to the runtime without crossing an FFI boundary.

## Crate
- Add: `cargo add spikard`
- Import via `spikard::prelude::*` for the most common types

## Core Types
- `App` – build your router and middleware stack
- `Context` – access path/query/header/cookie/body data
- Response helpers such as `Json`, streaming bodies, and typed errors

## Routing
```rust
use spikard::prelude::*;

fn main() {
    let app = App::new().get("/health", |_ctx: Context| async { Ok(Json(json!({"status": "ok"}))) });
    app.listen(8000).unwrap();
}
```

## Middleware
Use Tower layers or helper methods on `App` to add logging, tracing, auth, or custom extractors.

## Validation
Integrate serde-based DTOs with JSON Schema derivation to keep contracts aligned with other bindings and code generators.
