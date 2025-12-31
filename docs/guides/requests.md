# Requests & Responses

Handlers receive a context object tailored to each binding but backed by the same Rust data model.

## Read request data

=== "Python"

    --8<-- "snippets/python/request_data.md"

=== "TypeScript"

    --8<-- "snippets/typescript/request_data.md"

=== "Ruby"

    --8<-- "snippets/ruby/request_data.md"

=== "PHP"

    --8<-- "snippets/php/request_data.md"

=== "Rust"

    --8<-- "snippets/rust/request_data.md"

## Return responses

=== "Python"

    --8<-- "snippets/python/response_basic.md"

=== "TypeScript"

    --8<-- "snippets/typescript/response_basic.md"

=== "Ruby"

    --8<-- "snippets/ruby/response_basic.md"

=== "PHP"

    --8<-- "snippets/php/response_basic.md"

=== "Rust"

    --8<-- "snippets/rust/response_basic.md"

## Tips
- Use DTOs/schemas so validation runs before your handler executes.
- Prefer returning plain values/structs; the runtime will serialize and set content types.
- For streaming/WebSocket/SSE, see the streaming section in the concepts docs.
