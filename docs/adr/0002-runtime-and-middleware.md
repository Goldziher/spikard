# ADR 0002: HTTP Runtime and Middleware Pipeline
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard’s runtime must provide consistent HTTP behavior—routing, validation, middleware, and lifecycle hooks—across all bindings while prioritizing performance and standards compliance.

## Decision
- **Runtime stack**: `crates/spikard-http` builds on tokio + hyper + axum + tower. Routing uses axum-style patterns (e.g., `/users/{id}`) with typed parameter extraction for strings, numbers, UUIDs, dates, and booleans.
- **Middleware order (in Rust)**:
  1. Request ID (configurable header + UUID)
  2. Timeouts
  3. Rate limiting
  4. Body size limits
  5. Compression (gzip/brotli)
  6. Static files (optional)
  7. CORS
  8. Handler execution
- **Lifecycle hooks**: on_request, pre_validation, pre_handler, on_response, on_error—executed in Rust with optional short-circuit to a response. Bindings register hooks and forward closures/function pointers into the runtime.
- **Problem details**: Errors use RFC 9457-compatible payloads and align with `testing_data/validation_errors`.
- **Static files**: Served via tower-http with ETag/Last-Modified and optional precompressed assets when configured.

## Consequences
- All bindings must forward `ServerConfig` and related middleware configs without altering order or defaults.
- Hook registration in bindings must avoid overhead when no hooks are registered (Rust keeps fast-path checks).
- Middleware additions must update fixtures, configs, and e2e suites across languages.

## References
- Runtime: `crates/spikard-http`
- Configs: `crates/spikard-core/src/config.rs`, language wrappers in `packages/*/`
- Fixtures: `testing_data/validation_errors`, `testing_data/*_limits`, `testing_data/cors`, `testing_data/static_files`, `testing_data/rate_limit`
