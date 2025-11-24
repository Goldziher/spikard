# Routing Basics

Routing is uniform across bindings: define an `App`, register routes with typed parameters, and return typed responses.

## Declare routes

=== "Python"

    --8<-- "snippets/python/routing_basic.md"

=== "TypeScript"

    --8<-- "snippets/typescript/routing_basic.md"

=== "Ruby"

    --8<-- "snippets/ruby/routing_basic.md"

=== "Rust"

    --8<-- "snippets/rust/routing_basic.md"

## Path and query params

=== "Python"

    --8<-- "snippets/python/path_params.md"

=== "TypeScript"

    --8<-- "snippets/typescript/path_params.md"

=== "Ruby"

    --8<-- "snippets/ruby/path_params.md"

=== "Rust"

    --8<-- "snippets/rust/path_params.md"

## Best practices
- Keep handlers small and pure; push IO into services.
- Prefer DTOs for shared schemas so codegen can derive OpenAPI/AsyncAPI.
- Use per-route middleware when sensitive endpoints need extra auth/logging.
