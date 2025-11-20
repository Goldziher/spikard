# Lifecycle Hooks E2E Test Fixtures

This directory contains comprehensive test fixtures for the lifecycle hooks feature, covering all five hook points in the Spikard framework.

## Overview

Lifecycle hooks allow you to intercept and modify requests and responses at different stages of the request lifecycle:

1. **onRequest**: Early request processing (logging, request ID generation)
2. **preValidation**: Pre-validation checks (rate limiting before validation)
3. **preHandler**: Authentication and authorization (before handler execution)
4. **onResponse**: Response post-processing (security headers, timing)
5. **onError**: Error handling and formatting

## Test Fixtures

### Basic Hook Examples

- **01-on-request-logging.json**: Tests onRequest hooks for logging and request ID generation
- **08-on-response-security-headers.json**: Tests onResponse hooks adding security headers
- **09-on-response-timing.json**: Tests onResponse hooks with timing information
- **10-on-error-logging.json**: Tests onError hooks for error logging and formatting

### Short-Circuit Behavior

These fixtures test hooks that return early responses, preventing further processing:

- **03-pre-validation-rate-limit-exceeded.json**: Rate limiter returns 429 Too Many Requests
- **05-pre-handler-authentication-failed.json**: Auth hook returns 401 Unauthorized
- **07-pre-handler-authorization-forbidden.json**: Authorization hook returns 403 Forbidden

### Happy Path Examples

- **02-pre-validation-rate-limit.json**: Rate limit passes, allows request through
- **04-pre-handler-authentication.json**: Valid authentication, grants access
- **06-pre-handler-authorization.json**: Authorized user, access granted

### Complex Scenarios

- **11-multiple-hooks-all-phases.json**: Demonstrates all five hook types working together in a complete request lifecycle
- **12-hook-execution-order.json**: Verifies that multiple hooks of the same type execute in registration order

## Fixture Structure

Each fixture follows this structure:

```json
{
  "name": "Fixture name",
  "description": "What this fixture tests",
  "category": "lifecycle_hooks",
  "handler": {
    "route": "/api/endpoint",
    "method": "GET",
    "middleware": {
      "lifecycle_hooks": {
        "on_request": [...],
        "pre_validation": [...],
        "pre_handler": [...],
        "on_response": [...],
        "on_error": [...]
      }
    }
  },
  "request": {
    "method": "GET",
    "path": "/api/endpoint"
  },
  "expected_response": {
    "status_code": 200,
    "body": {...},
    "headers": {...}
  }
}
```

## Generated Test Code

The test generator (`tools/test-generator`) processes these fixtures and generates:

1. **Mock hook functions** that demonstrate expected behavior
2. **Hook registration code** using the Spikard lifecycle hooks API
3. **E2E tests** that verify the hooks execute correctly

### Example Generated Code

For Python:

```python
# Generated hook function
async def handler_name_hook_name_on_request_0(request: Any) -> Any:
    """onRequest hook: request_logger"""
    # Mock implementation for testing
    return request

# Generated app factory with hook registration
def create_app_handler_name() -> Spikard:
    app = Spikard()
    # Register lifecycle hooks
    app.on_request(handler_name_hook_name_on_request_0)
    # Register handler
    app.register_route("GET", "/api/test", ...)(handler_name)
    return app
```

## Testing Strategy

These fixtures enable comprehensive e2e testing of:

- ✅ Hook execution order (multiple hooks run sequentially)
- ✅ Short-circuit behavior (hooks can stop request processing)
- ✅ Request transformation (hooks can modify requests)
- ✅ Response transformation (hooks can modify responses)
- ✅ Error handling (hooks can format error responses)
- ✅ Cross-phase interaction (context passing between hooks)

## Usage

Generate tests from these fixtures:

```bash
# Generate Python e2e tests
task generate:tests:python

# Generate all language tests
task generate:tests

# Run generated tests
task test:e2e:python
```

## Implementation Status

- ✅ **Python**: Test generator extended with full lifecycle hooks support
- ⏳ **Rust**: Test generator pending
- ⏳ **Node.js**: Test generator pending
- ⏳ **Ruby**: Test generator pending

## Related Documentation

- Design & implementation: `docs/adr/0005-lifecycle-hooks.md`
- Rust Example: `examples/rust-lifecycle-hooks/`
