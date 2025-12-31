```rust
use spikard::{App, ServerConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load from .env file if present
    dotenv::dotenv().ok();

    let config = ServerConfig::builder()
        .host(env::var("SPIKARD_HOST").unwrap_or_else(|_| "127.0.0.1".into()))
        .port(
            env::var("SPIKARD_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8000),
        )
        .workers(
            env::var("SPIKARD_WORKERS")
                .ok()
                .and_then(|w| w.parse().ok())
                .unwrap_or(1),
        )
        .request_timeout_secs(
            env::var("SPIKARD_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30),
        )
        .build();

    let app = App::with_config(config);

    // Keep secrets in env
    let api_key = env::var("API_KEY").ok();
    let db_url = env::var("DATABASE_URL").ok();

    app.run().await?;
    Ok(())
}
```
