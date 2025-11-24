# Middleware

Middleware is the right place for cross-cutting behavior like logging, auth, or request shaping. The API mirrors per-language conventions but calls into the same Rust pipeline.

## Adding Middleware
- **Python**: `app.use(middleware_fn)` where the function receives context and a `next` callable.
- **TypeScript**: `app.use((ctx, next) => { ... })` with async support.
- **Ruby**: `App.use do |ctx, next_middleware| ... end`.
- **Rust**: add Tower layers to the `App` via helpers or `app.layer(...)`.

## Patterns
- **Auth guards**: check headers/cookies, enrich context with the authenticated principal, and short-circuit on failures.
- **Observability**: emit structured logs and traces; forward request IDs/correlation IDs.
- **Request shaping**: normalize headers, coerce params, or inject tenant/feature flags.

## Tips
- Keep middleware pure and side-effect free when possible; expensive IO should be async.
- Prefer per-route middleware for sensitive endpoints.
- Use shared context keys to pass data to handlers; keep namespaced to avoid collisions.

Reference designs are captured in [ADR 0002](../adr/0002-runtime-and-middleware.md).
