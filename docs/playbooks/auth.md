# Auth & Guards

Add lightweight auth/guard logic across bindings. Prefer middleware/hooks for cross-cutting checks.

## Token check middleware

=== "Python"

    --8<-- "snippets/python/auth_middleware.md"

=== "TypeScript"

    --8<-- "snippets/typescript/auth_middleware.md"

=== "Ruby"

    --8<-- "snippets/ruby/auth_middleware.md"

=== "PHP"

    --8<-- "snippets/php/auth_middleware.md"

=== "Rust"

    --8<-- "snippets/rust/auth_layer.md"

## Tips
- Normalize header casing before checks (bindings expose lowercased headers).
- Short-circuit unauthorized requests with a structured body.
- Add per-route middleware for sensitive endpoints (admin, payments).
