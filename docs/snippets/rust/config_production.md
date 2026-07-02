```rust
use spikard::{
    App, CompressionConfig, OpenApiConfig, RateLimitConfig, ServerConfig,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Production logging setup
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Panic handling for production
    std::panic::set_hook(Box::new(|info| {
        tracing::error!("Panic occurred: {:?}", info);
    }));

    let config = ServerConfig::builder()
        .host("0.0.0.0")
        .port(8080)
        .workers(4)
        .request_timeout_secs(60)
        .max_body_size(10 * 1024 * 1024)

        // High-quality compression
        .compression(
            CompressionConfig::builder()
                .gzip(true)
                .brotli(true)
                .min_size(1024)
                .quality(6)
                .build(),
        )

        // Protect against abuse
        .rate_limit(
            RateLimitConfig::builder()
                .per_second(100)
                .burst(200)
                .ip_based(true)
                .build(),
        )

        // Auto-generated docs
        .openapi(
            OpenApiConfig::builder()
                .enabled(true)
                .title("Production API")
                .version("1.0.0")
                .build(),
        )

        // Graceful shutdown
        .graceful_shutdown(true)
        .shutdown_timeout_secs(30)
        .build();

    let app = App::with_config(config);
    app.run().await?;
    Ok(())
}
```
