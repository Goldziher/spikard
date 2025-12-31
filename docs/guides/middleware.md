# Middleware

Middleware is the right place for cross-cutting behavior like logging, auth, or request shaping. The API mirrors per-language conventions but calls into the same Rust pipeline.

## Add middleware

=== "Python"

    --8<-- "snippets/python/middleware_basic.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_basic.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_basic.md"

=== "PHP"

    --8<-- "snippets/php/middleware_basic.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_basic.md"

## Patterns

### Auth guards

Check headers/cookies, enrich context with the authenticated principal, and short-circuit on failures.

=== "Python"

    --8<-- "snippets/python/middleware_auth.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_auth.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_auth.md"

=== "PHP"

    --8<-- "snippets/php/middleware_auth.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_auth.md"

### Observability

Emit structured logs and traces; forward request IDs and correlation IDs for distributed tracing.

=== "Python"

    --8<-- "snippets/python/middleware_observability.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_observability.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_observability.md"

=== "PHP"

    --8<-- "snippets/php/middleware_observability.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_observability.md"

### Request shaping

Normalize headers, coerce parameters, inject tenant/feature flags, or apply rate limiting.

=== "Python"

    --8<-- "snippets/python/middleware_shaping.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_shaping.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_shaping.md"

=== "PHP"

    --8<-- "snippets/php/middleware_shaping.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_shaping.md"

## Middleware chaining and execution order

Middleware executes in the order it's registered. Request middleware runs top-to-bottom, response middleware runs bottom-to-top:

```
Request flow:
  → Middleware 1 (observability: log request)
    → Middleware 2 (auth: verify token)
      → Middleware 3 (rate limit: check limits)
        → Handler
      ← Middleware 3 (response shaping: compress)
    ← Middleware 2 (auth: add headers)
  ← Middleware 1 (observability: log response)
```

Register middleware in order of importance:

1. Observability (request ID generation)
2. Security (CORS, auth)
3. Request shaping (rate limiting, normalization)
4. Handler-specific middleware

## Testing middleware

Test middleware in isolation by passing mock request/response objects:

=== "Python"

    --8<-- "snippets/python/middleware_testing.md"

=== "TypeScript"

    --8<-- "snippets/typescript/middleware_testing.md"

=== "Ruby"

    --8<-- "snippets/ruby/middleware_testing.md"

=== "PHP"

    --8<-- "snippets/php/middleware_testing.md"

=== "Rust"

    --8<-- "snippets/rust/middleware_testing.md"

## Tips

- Keep middleware pure and side-effect free when possible; expensive IO should be async.
- Prefer per-route middleware for sensitive endpoints.
- Use shared context keys to pass data to handlers; keep namespaced to avoid collisions.
- Chain middleware thoughtfully: observability first, then security, then request shaping.
- Test middleware in isolation with mock requests to ensure correct error handling.
