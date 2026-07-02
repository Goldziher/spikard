```rust
use axum::response::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::{get, App, ServerConfig};

#[derive(Serialize, Deserialize, JsonSchema)]
struct HealthResponse {
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::builder()
        .host("0.0.0.0")
        .port(8080)
        .workers(4)
        .request_timeout_secs(60)
        .max_body_size(5 * 1024 * 1024)  // 5MB
        .build();

    let mut app = App::with_config(config);

    app.route(get("/health"), |_ctx| async move {
        Ok(Json(HealthResponse { status: "ok".into() }).into())
    })?;

    app.run().await?;
    Ok(())
}
```
