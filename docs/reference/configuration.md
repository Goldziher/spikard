# Configuration Reference

Configuration is code-first with environment overrides. Use these settings consistently across bindings and the CLI.

## Server
- `host` (default `0.0.0.0`)
- `port` (default `8000`)
- `workers` / runtime threads
- `graceful_shutdown_timeout`
- `request_timeout`

## Middleware
- Logging/tracing toggles (OpenTelemetry-friendly)
- CORS, compression, caching headers
- Request/response size limits

## Validation
- Enable/disable request validation per route or globally
- Enable/disable response validation
- Schema registry location when loading from OpenAPI/AsyncAPI

## CLI Flags (common)
- `--host`, `--port` for `spikard run`
- `--workers` for future multi-worker support
- `--reload` (planned) for dev-mode restarts

## Environment Variables (convention)
- `SPIKARD_HOST`, `SPIKARD_PORT`
- `SPIKARD_LOG` or `RUST_LOG` for log levels
- `SPIKARD_OTEL_EXPORTER` for tracing destinations

This page will evolve alongside the runtime. For architectural context, see [Configuration guide](../guides/configuration.md).
