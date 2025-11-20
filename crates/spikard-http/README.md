# spikard-http

Axum-based HTTP runtime that hosts Spikard routes, enforces JSON-schema validation, and coordinates cross-language handler execution.

## Components
- `router` and `route` translate Python route metadata into strongly typed Rust handlers.
- `validation` checks headers, cookies, query params, and bodies against the fixtures in `testing_data/`.
- `server` wraps Axum/Tokio bootstrapping and exposes defaults through `ServerConfig`.

## Performance

Native Rust implementation using Axum and tower-http middleware. Benchmarks on macOS (Darwin 24.6.0) with 50 concurrent connections:

| Workload | Throughput | Mean Latency | P95 Latency | P99 Latency | Memory |
|----------|------------|--------------|-------------|-------------|--------|
| Baseline | 165,228 req/s | 0.30ms | 0.36ms | 0.45ms | 17.4 MB |
| JSON Bodies | *pending* | *pending* | *pending* | *pending* | *pending* |
| Multipart Forms | *pending* | *pending* | *pending* | *pending* | *pending* |
| URL-Encoded | *pending* | *pending* | *pending* | *pending* | *pending* |

**Architecture Highlights:**
- **Zero-overhead abstraction**: Handler trait with `Pin<Box<dyn Future>>` enables language-agnostic integration
- **Tower middleware stack**: Compression, rate limiting, timeouts, and CORS with minimal latency impact
- **Efficient routing**: Axum's path matching with zero allocations for static routes
- **Low memory baseline**: ~17 MB with efficient memory pooling and minimal allocations

The native Rust implementation provides ~38% higher throughput compared to Python bindings while maintaining even lower latency characteristics. Startup time averages 1.01s with first response in 908ms.

Full benchmark methodology: `tools/benchmark-harness/`

## Development
- Build with `cargo build -p spikard-http` or `task build:http`.
- Execute tests and fixture validations via `cargo test -p spikard-http`.
- When altering schemas, sync the Python fixtures and regenerate bindings before rerunning the CLI.
