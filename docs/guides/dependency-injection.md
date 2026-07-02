# Dependency Injection

Spikard's DI system (feature-gated, enabled via `di` feature flag) allows you to declare handler dependencies and have them resolved automatically before execution.

## Core Concepts

- **Dependencies are named**: Declare what your handler needs by key (e.g., `"db_pool"`, `"config"`)
- **Resolution is per-route**: Specify dependencies on each route builder
- **Scope**: Container owns all dependency instances; resolution happens at request-time

## How It Works

1. Define a `DIContainer` with dependency factories
2. Pass it to `ServerConfig.di_container`
3. On each route, declare dependencies via `handler_dependencies(["key1", "key2", ...])`
4. At request-time, the container resolves dependencies and passes them to the handler

## Rust Example

```rust
use spikard::{App, ServerConfig, get, RequestContext};
use spikard_core::di::DIContainer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut container = DIContainer::new();
    
    // Register a dependency factory
    container.register("db_url", Arc::new(|| {
        Arc::new("postgresql://localhost/mydb".to_string())
    }))?;
    
    let mut config = ServerConfig::default();
    config.di_container = Some(Arc::new(container));
    
    let mut app = App::new().config(config);
    
    // Declare dependencies for this route
    app.route(
        get("/users").handler_dependencies(vec!["db_url".to_string()]),
        |ctx: RequestContext| async move {
            // Dependencies are available via ctx.dependencies()
            if let Some(deps) = ctx.dependencies() {
                // Use resolved dependencies...
            }
            Ok(axum::http::Response::builder()
                .status(200)
                .body(axum::body::Body::from("OK"))?)
        },
    )?;
    
    app.run().await?;
    Ok(())
}
```

## Language Binding Support

DI is currently fully implemented in Rust. Language bindings (Python, TypeScript, Ruby, PHP, Elixir) will follow with native APIs aligned to their runtime conventions.

## Notes

- Dependencies are resolved per-request (not cached globally unless you implement caching within the factory)
- Circular or missing dependencies fail fast with clear errors
- The feature is behind a Rust `di` Cargo feature flag
