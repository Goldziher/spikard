# Configuration Reference

Configuration is code-first with environment overrides. Use these settings consistently across bindings and the CLI.

## Server (Rust core / CLI)
- `host` (default `0.0.0.0`)
- `port` (default `8000`)
- `workers` / runtime threads
- `graceful_shutdown_timeout` (seconds)
- `request_timeout` (seconds)
- `max_body_size` (bytes)
- `enable_request_id` (bool)
- `compression` (gzip/brotli, quality, min_size)
- `rate_limit` (per_second, burst, ip_based)
- `static_files` (directory, route_prefix, index_file, cache_control)
- `openapi` (enabled, title, version, swagger_ui_path, redoc_path)
- `jwt_auth` (secret, algorithm, audience, issuer, leeway)

## Validation
- Enable request/response validation globally or per route (bindings surface this via schemas/DTOs).
- Schema sources: derived (`JsonSchema`, msgspec/Typed schemas, Zod) or explicit JSON Schema objects.

## CLI Flags (common)
- `--host`, `--port` for `spikard run`
- `--workers` (planned multi-worker support)
- `--reload` (planned) for dev-mode restarts

## Environment Variables (convention)
- `SPIKARD_HOST`, `SPIKARD_PORT`
- `SPIKARD_LOG` / `RUST_LOG` for log levels
- `SPIKARD_OTEL_EXPORTER` for tracing destinations

See [Configuration guide](../guides/configuration.md) for usage patterns; binding-specific pages cover how these surface in each runtime.
