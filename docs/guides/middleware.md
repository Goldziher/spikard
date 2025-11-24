# Middleware

Middleware is the right place for cross-cutting behavior like logging, auth, or request shaping. The API mirrors per-language conventions but calls into the same Rust pipeline.

## Add middleware

=== "Python"

    --8<-- "snippets/python/middleware_basic.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_basic.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_basic.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_basic.md"

## Patterns
- **Auth guards**: check headers/cookies, enrich context with the authenticated principal, and short-circuit on failures.
- **Observability**: emit structured logs and traces; forward request IDs/correlation IDs.
- **Request shaping**: normalize headers, coerce params, or inject tenant/feature flags.

## Tips
- Keep middleware pure and side-effect free when possible; expensive IO should be async.
- Prefer per-route middleware for sensitive endpoints.
- Use shared context keys to pass data to handlers; keep namespaced to avoid collisions.
