# Testing

Test your Spikard applications with built-in test clients for all languages. Each test client starts a real server for reliable HTTP, WebSocket, and SSE testing.

## Quick Start

=== "Python"

    --8<-- "snippets/python/test_quickstart.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_quickstart.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_quickstart.md"

=== "PHP"

    --8<-- "snippets/php/test_quickstart.md"

=== "Rust"

    --8<-- "snippets/rust/test_quickstart.md"

## Unit Testing Handlers

Test individual handlers with different inputs:

=== "Python"

    --8<-- "snippets/python/test_unit.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_unit.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_unit.md"

=== "PHP"

    --8<-- "snippets/php/test_unit.md"

=== "Rust"

    --8<-- "snippets/rust/test_unit.md"

## Testing Validation

Test request and response validation with invalid inputs:

=== "Python"

    --8<-- "snippets/python/test_validation.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_validation.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_validation.md"

=== "PHP"

    --8<-- "snippets/php/test_validation.md"

=== "Rust"

    --8<-- "snippets/rust/test_validation.md"

## Testing Middleware

Test middleware behavior and execution order:

=== "Python"

    --8<-- "snippets/python/test_middleware.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_middleware.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_middleware.md"

=== "PHP"

    --8<-- "snippets/php/test_middleware.md"

=== "Rust"

    --8<-- "snippets/rust/test_middleware.md"

## Testing WebSocket

Test WebSocket connections and message exchange:

=== "Python"

    --8<-- "snippets/python/test_websocket.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_websocket.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_websocket.md"

=== "PHP"

    --8<-- "snippets/php/test_websocket.md"

=== "Rust"

    --8<-- "snippets/rust/test_websocket.md"

## Testing Server-Sent Events (SSE)

Test SSE streams:

=== "Python"

    --8<-- "snippets/python/test_sse.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_sse.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_sse.md"

=== "PHP"

    --8<-- "snippets/php/test_sse.md"

=== "Rust"

    --8<-- "snippets/rust/test_sse.md"

## Integration Testing

Test multiple endpoints together:

=== "Python"

    --8<-- "snippets/python/test_integration.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_integration.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_integration.md"

=== "PHP"

    --8<-- "snippets/php/test_integration.md"

=== "Rust"

    --8<-- "snippets/rust/test_integration.md"

## Best Practices

1. **Use context managers** (Python) or cleanup (TypeScript/Ruby) to ensure servers stop
2. **Test error cases** - don't just test happy paths
3. **Test validation** - ensure invalid inputs are rejected
4. **Test middleware** - verify auth, logging, etc. work correctly
5. **Use fixtures** - reuse common test setups
6. **Keep tests fast** - TestClient starts real servers, but they're fast
7. **Test streaming** - verify WebSocket and SSE endpoints work correctly

## Verify It Works

Run your tests:

=== "Python"

    --8<-- "snippets/python/test_run.md"

=== "TypeScript"

    --8<-- "snippets/typescript/test_run.md"

=== "Ruby"

    --8<-- "snippets/ruby/test_run.md"

=== "PHP"

    --8<-- "snippets/php/test_run.md"

=== "Rust"

    --8<-- "snippets/rust/test_run.md"

## Next Steps

- See the [Validation Guide](validation.md) for testing validation schemas
- See the [Middleware Guide](middleware.md) for testing middleware chains
- See the [Streaming Guide](../playbooks/streaming.md) for advanced SSE/WebSocket testing
