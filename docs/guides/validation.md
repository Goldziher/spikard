# Validation Flows

Validation keeps handlers simple by enforcing contracts at the edge.

## Validate requests

=== "Python"

    --8<-- "snippets/python/validation_basic.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_basic.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_basic.md"

=== "Rust"

    --8<-- "snippets/rust/validation_basic.md"

## Validate responses

Enable response validation on routes that require strict contracts by registering response DTOs/schemas, as in the Rust example above. Keep schemas in version control so generated clients and fixtures stay aligned.

## Testing contracts
- Use the CLI generators to create fixtures/tests from OpenAPI/AsyncAPI.
- Keep schemas in version control; run `task test` to ensure parity across bindings.
- Add ADR updates when changing validation behavior.
