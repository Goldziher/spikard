# ADR 0007: Observability and OpenTelemetry
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard ships a Rust-first runtime with built-in observability. Today the core instrumentation is in Rust; bindings should configure it without reimplementing telemetry. We want consistent traces/metrics/logs across languages with minimal setup and no vendor lock-in.

## Decision
- **Tracing backend**: Use `tracing` and `tracing-subscriber` in `crates/spikard-http` for spans and structured logs. Attach request IDs, route names, status codes, and error contexts.
- **OTel export**: Provide optional OpenTelemetry exporters (HTTP/gRPC) wired through `tracing-opentelemetry`. Defaults to a no-op subscriber; enabling OTEL is opt-in via server config (endpoint, protocol, headers, sampler, resource/service metadata).
- **Scope of instrumentation**:
  - Server startup/shutdown spans.
  - Per-request span with attributes: method, path pattern, status, duration, request_id, remote_addr (if available), rate_limit decisions, timeout/body-limit hits, compression result, static file cache hits/misses.
  - Middleware spans for CORS, rate limiting, timeouts, compression, static files.
  - Lifecycle hooks and handler execution span; errors mapped to RFC 9457 payloads include trace context.
- **Binding configuration**: Expose telemetry options through `ServerConfig` (and per-language wrappers) so Python/Node/Ruby/WASM enable OTEL by toggling config, not custom code.
- **Sampling/log levels**: Respect user-specified sampling; default to parent/always-on in dev and parent/trace-id-based in prod. Log level configurable separately from trace sampling.

## Consequences
- Bindings must forward telemetry config to the Rust runtime; no per-language exporters.
- Adding new middleware or handlers requires adding span attributes for observability parity.
- Exporters must fail closed (fallback to stdout/logger) without crashing the server if endpoints are unreachable or misconfigured.
- Tests and examples should keep OTEL disabled by default to avoid network dependency; feature flags or env-driven toggles gate exporters.

## References
- Runtime tracing: `crates/spikard-http` (tracing layers and middleware spans)
- Config surface: `ServerConfig` telemetry/otel options in `crates/spikard-core` and language wrappers
