# Configuration

Spikard favors code-based configuration with environment overrides. The same knobs exist regardless of binding.

## Server Settings
- `host` / `port` – network binding for the HTTP server
- `workers` – concurrency; defaults to the Tokio runtime defaults
- `keep_alive` / `request_timeout` – tune to match upstream load balancers

## Middleware Defaults
- Logging/tracing enabled by default with request IDs
- CORS/compression configurable per app or per route
- Add custom middleware to inject tenant data, auth, or feature flags

## Validation Controls
- Enable/disable request and response validation globally or per route
- Provide JSON Schemas or rely on derived DTOs
- Customize error formatting for clients

## Environment Overrides
- Prefer env vars for deploy-time changes (e.g., `SPIKARD_PORT=8080`)
- Keep secrets (API keys, DSNs) in env or secret managers; middleware can pull them into context

A detailed option matrix will live in [Reference: Configuration](../reference/configuration.md). Use the Taskfile (`task docs:serve`) to iterate on examples locally.
