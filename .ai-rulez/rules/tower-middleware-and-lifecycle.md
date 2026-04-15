---
priority: high
---

# Tower Middleware and Lifecycle Hooks

All middleware is implemented in Rust via tower-http. Bindings only expose typed configuration APIs.

## Middleware Stack

Compression (gzip/brotli), rate limiting (governor), timeouts, request IDs, authentication (JWT/API key), static files. Configuration via ServerConfig structs (CompressionConfig, RateLimitConfig, StaticFilesConfig).

## Lifecycle Hooks

Execute in order: onRequest, preValidation, preHandler, onResponse, onError. Use `Option<Arc<dyn Fn>>` for zero-cost when not registered (<1ns overhead). HookResult enum with Continue/ShortCircuit variants.

## Async Hook Support

- Python: pyo3_async_runtimes for asyncio integration
- TypeScript: ThreadsafeFunction for async callbacks
- Hook errors are logged and included in response metadata, never prevent response transmission

## Security

- JWT validation: check algorithm, audience, issuer, expiration
- Auth is per-route via route metadata, not global
- Auth failures return 401 with ProblemDetails
- Validate all request surface data (headers, query, path, cookies) before reaching handlers
- CORS via testing_data/cors/ fixtures
