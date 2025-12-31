# Validation Flows

Validation keeps handlers simple by enforcing contracts at the edge.

## Request body validation

Define schemas to automatically validate incoming JSON payloads. Invalid requests are rejected before reaching your handler.

=== "Python"

    --8<-- "snippets/python/validation_request_body.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_request_body.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_request_body.md"

=== "PHP"

    --8<-- "snippets/php/validation_request_body.md"

=== "Rust"

    --8<-- "snippets/rust/validation_request_body.md"

## Query parameter validation

Validate URL query strings with type constraints and custom rules.

=== "Python"

    --8<-- "snippets/python/validation_query.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_query.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_query.md"

=== "PHP"

    --8<-- "snippets/php/validation_query.md"

=== "Rust"

    --8<-- "snippets/rust/validation_query.md"

## Path parameter validation

Validate URL path segments with type checking and format constraints.

=== "Python"

    --8<-- "snippets/python/validation_path.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_path.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_path.md"

=== "PHP"

    --8<-- "snippets/php/validation_path.md"

=== "Rust"

    --8<-- "snippets/rust/validation_path.md"

## Response validation

Validate outgoing responses to ensure API contracts are maintained. This catches serialization errors and schema violations before sending data to clients.

=== "Python"

    --8<-- "snippets/python/validation_response.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_response.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_response.md"

=== "PHP"

    --8<-- "snippets/php/validation_response.md"

=== "Rust"

    --8<-- "snippets/rust/validation_response.md"

## Custom error formatting

Customize validation error responses to match your API style and provide clear feedback to clients.

=== "Python"

    --8<-- "snippets/python/validation_error_format.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_error_format.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_error_format.md"

=== "PHP"

    --8<-- "snippets/php/validation_error_format.md"

=== "Rust"

    --8<-- "snippets/rust/validation_error_format.md"

## Testing validation

Verify that schemas correctly validate inputs and reject invalid data.

=== "Python"

    --8<-- "snippets/python/validation_testing.md"

=== "TypeScript"

    --8<-- "snippets/typescript/validation_testing.md"

=== "Ruby"

    --8<-- "snippets/ruby/validation_testing.md"

=== "PHP"

    --8<-- "snippets/php/validation_testing.md"

=== "Rust"

    --8<-- "snippets/rust/validation_testing.md"

## Best practices

- **Keep schemas in version control** - Track schema changes alongside code changes to maintain API contract history
- **Generate OpenAPI/AsyncAPI specs** - Use CLI generators to create fixtures and tests from schemas
- **Test validation thoroughly** - Add tests for both valid inputs and all rejection cases
- **Document validation rules** - Add comments or descriptions to schema fields explaining constraints
- **Use semantic error codes** - Return structured error responses that clients can handle programmatically
