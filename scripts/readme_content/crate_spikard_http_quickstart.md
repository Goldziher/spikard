```rust
use spikard_http::{ServerConfig, start_server};
use spikard_core::{RouteConfig, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: 8080,
        ..Default::default()
    };

    // Create routes with schemas
    let routes = vec![
        RouteConfig::get("/health")
            .handler("health", |_req| async {
                Ok(Response::new(200).with_body(r#"{"status": "ok"}"#))
            }),
    ];

    start_server(config, routes).await?;
    Ok(())
}
```
