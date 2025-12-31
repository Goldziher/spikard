```ruby
require "spikard"

config = Spikard::ServerConfig.new(
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  request_timeout: 60,
  max_body_size: 10 * 1024 * 1024,

  # High-quality compression
  compression: Spikard::CompressionConfig.new(
    gzip: true,
    brotli: true,
    min_size: 1024,
    quality: 6
  ),

  # Protect against abuse
  rate_limit: Spikard::RateLimitConfig.new(
    per_second: 100,
    burst: 200,
    ip_based: true
  ),

  # Auto-generated docs
  openapi: Spikard::OpenApiConfig.new(
    enabled: true,
    title: "Production API",
    version: "1.0.0"
  ),

  # Graceful shutdown
  graceful_shutdown: true,
  shutdown_timeout: 30
)

app = Spikard::App.new(config: config)
```
