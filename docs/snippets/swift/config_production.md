```swift
import Foundation
import Spikard
import RustBridge

// Build the full server configuration as JSON and parse it through the
// generated `serverConfigFromJson` bridge function, matching the field
// names on the Rust `ServerConfig` struct.
let productionConfigJson = """
{
  "host": "0.0.0.0",
  "port": 8080,
  "workers": 4,
  "enable_request_id": true,
  "max_body_size": 10485760,
  "request_timeout": 60,
  "compression": {
    "gzip": true,
    "brotli": true,
    "min_size": 1024,
    "quality": 6
  },
  "rate_limit": {
    "per_second": 100,
    "burst": 200,
    "ip_based": true
  },
  "static_files": [],
  "graceful_shutdown": true,
  "shutdown_timeout": 30,
  "background_tasks": {
    "max_queue_size": 1024,
    "max_concurrent_tasks": 128,
    "drain_timeout_secs": 30
  },
  "enable_http_trace": false
}
"""

let productionConfig = try serverConfigFromJson(productionConfigJson)

let app = App()
try app.config(host: productionConfig.host().toString(), port: productionConfig.port())

try app.get({ _ in #"{"status":"healthy"}"# }, path: "/health")

try app.run()
```
