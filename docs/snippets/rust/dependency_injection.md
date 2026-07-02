```rust
use axum::response::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::{get, App, RequestContext};
use spikard_http::ServerConfig;
use std::sync::Arc;

#[derive(Serialize, Deserialize, JsonSchema)]
struct Health {
    db: String,
    env: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register dependencies on the server config
    let config = ServerConfig::builder()
        .provide_value("config", "postgresql://localhost/app".to_string())
        .provide_factory("db_pool", |resolved| async move {
            let url: Arc<String> = resolved.get("config").ok_or("missing config")?;
            Ok(format!("pool({})", url))
        })
        .build();

    let mut app = App::new().config(config);

    app.route(get("/stats"), |ctx: RequestContext| async move {
        let deps = ctx.dependencies();
        let db = deps.and_then(|d| d.get::<String>("db_pool")).cloned().unwrap_or_default();
        let env = deps
            .and_then(|d| d.get::<String>("config"))
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        Ok(Json(Health { db, env }).into())
    })?;

    app.run().await?;
    Ok(())
}
```
