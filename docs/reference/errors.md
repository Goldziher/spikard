# Errors

Spikard uses a canonical structured error format across all languages and transports to ensure consistent error handling and facilitate integration with client applications.

## Error Format

### Internal Error Structure

Spikard uses a canonical `{error, code, details}` structure for all internal errors, FFI boundaries, and panic handling:

```json
{
  "error": "Human-readable error message",
  "code": "machine_readable_error_code",
  "details": {
    "field_name": "additional context",
    "validation_rules": ["rule1", "rule2"]
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `error` | string | Yes | Human-readable error description for logging and display |
| `code` | string | Yes | Machine-readable error code (snake_case) for programmatic handling |
| `details` | object | No | Additional context. Empty object `{}` if not applicable |

### HTTP Validation Errors

For HTTP validation responses (status 422), Spikard uses a Pydantic-style detail array format compatible with FastAPI and OpenAPI:

```json
{
  "detail": [
    {
      "type": "string_too_short",
      "loc": ["body", "email"],
      "msg": "String should have at least 5 characters",
      "input": "ab",
      "ctx": {"min_length": 5}
    }
  ]
}
```

## Common Error Codes

### Validation Errors

```json
{
  "error": "Missing required field: email",
  "code": "validation_error",
  "details": {
    "field": "email",
    "constraint": "required"
  }
}
```

### Type Errors

```json
{
  "error": "Invalid type for field: age",
  "code": "type_error",
  "details": {
    "field": "age",
    "expected": "integer",
    "received": "string"
  }
}
```

### Resource Not Found

```json
{
  "error": "User not found",
  "code": "not_found",
  "details": {
    "resource": "user",
    "id": 42
  }
}
```

### Unauthorized

```json
{
  "error": "Invalid or missing authentication token",
  "code": "unauthorized",
  "details": {
    "scheme": "Bearer",
    "hint": "Include Authorization header with valid token"
  }
}
```

### Forbidden

```json
{
  "error": "Insufficient permissions",
  "code": "forbidden",
  "details": {
    "required_role": "admin",
    "current_role": "user"
  }
}
```

### Rate Limit Exceeded

```json
{
  "error": "Rate limit exceeded",
  "code": "rate_limit_exceeded",
  "details": {
    "limit": 100,
    "window_seconds": 60,
    "retry_after": 45
  }
}
```

### Invalid Request

```json
{
  "error": "Malformed JSON in request body",
  "code": "invalid_request",
  "details": {
    "aspect": "body",
    "format": "json"
  }
}
```

### Internal Server Error

```json
{
  "error": "An unexpected error occurred",
  "code": "internal_error",
  "details": {
    "request_id": "abc-123-def",
    "timestamp": "2025-12-03T12:34:56Z"
  }
}
```

### Panic (FFI Safety)

```json
{
  "error": "Unexpected panic in Rust code",
  "code": "panic",
  "details": {}
}
```

See [ADR 0009: Panic Shielding](../adr/0009-panic-shielding.md) for implementation details.

## HTTP Status Code Mapping

| Error Code | HTTP Status | Reason |
|------------|------------|--------|
| `validation_error`, `type_error`, `invalid_request` | 400 Bad Request | Client error in request format or data |
| `unauthorized` | 401 Unauthorized | Missing or invalid authentication |
| `forbidden` | 403 Forbidden | Authenticated but lacks permission |
| `not_found` | 404 Not Found | Requested resource does not exist |
| `conflict` | 409 Conflict | Request conflicts with existing state |
| `rate_limit_exceeded` | 429 Too Many Requests | Rate limit exceeded |
| `internal_error`, `panic` | 500 Internal Server Error | Server-side error |

## Implementation Across Bindings

### Python (PyO3)

Errors are raised as Python exceptions with the JSON string in the exception message:

```python
import json

try:
    result = spikard.some_operation()
except Exception as e:
    error_dict = json.loads(str(e))
    print(error_dict["code"])  # e.g., "validation_error"
    print(error_dict["error"])  # e.g., "Invalid email"
```

### Node.js (napi-rs)

Errors are thrown as JavaScript Error objects with the payload in `error.message`:

```typescript
try {
    await app.someOperation();
} catch (error) {
    const payload = JSON.parse(error.message);
    console.log(payload.code);  // e.g., "validation_error"
    console.log(payload.error); // e.g., "Invalid email"
}
```

### Ruby (Magnus)

Errors are raised as Ruby exceptions with the JSON payload in the message:

```ruby
begin
    app.some_operation
rescue => e
    payload = JSON.parse(e.message)
    puts payload['code']   # e.g., 'validation_error'
    puts payload['error']  # e.g., 'Invalid email'
end
```

### PHP (ext-php-rs)

Errors are thrown as PHP exceptions with the JSON payload:

```php
try {
    $app->someOperation();
} catch (Exception $e) {
    $payload = json_decode($e->getMessage(), true);
    echo $payload['code'];   // e.g., 'validation_error'
    echo $payload['error'];  // e.g., 'Invalid email'
}
```

### WebAssembly (wasm-bindgen)

Errors are converted to JavaScript Error objects:

```javascript
try {
    await wasmModule.someOperation();
} catch (error) {
    const payload = JSON.parse(error.message);
    console.log(payload.code);   // e.g., 'validation_error'
    console.log(payload.error);  // e.g., 'Invalid email'
}
```

## HTTP Response Examples

### 400 Bad Request

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "Missing required field: email",
  "code": "validation_error",
  "details": {
    "field": "email",
    "constraint": "required"
  }
}
```

### 401 Unauthorized

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json
WWW-Authenticate: Bearer realm="api"

{
  "error": "Invalid or missing authentication token",
  "code": "unauthorized",
  "details": {
    "scheme": "Bearer"
  }
}
```

### 404 Not Found

```http
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "error": "User not found",
  "code": "not_found",
  "details": {
    "resource": "user",
    "id": 42
  }
}
```

### 500 Internal Server Error

```http
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "error": "An unexpected error occurred",
  "code": "internal_error",
  "details": {
    "request_id": "req-12345",
    "timestamp": "2025-12-03T12:34:56Z"
  }
}
```

## Handler Examples

=== "Python"

    ```python
    from spikard import Response

    @app.get("/fail")
    async def fail() -> Response:
        return Response(
            {
                "error": "Validation failed",
                "code": "validation_error",
                "details": {"field": "email"}
            },
            status=400,
        )
    ```

=== "TypeScript"

    ```typescript
    import { Spikard } from "spikard";

    const app = new Spikard();

    app.addRoute(
      { method: "GET", path: "/fail", handler_name: "fail", is_async: true },
      async () => ({
        statusCode: 400,
        body: {
          error: "Validation failed",
          code: "validation_error",
          details: { field: "email" }
        },
      }),
    );
    ```

=== "Ruby"

    ```ruby
    app.get "/fail" do |_request|
      [{
        error: "Validation failed",
        code: "validation_error",
        details: { field: "email" }
      }, 400]
    end
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;

    app.route(get("/fail"), |_ctx: Context| async move {
        Ok(Json(json!({
            "error": "Validation failed",
            "code": "validation_error",
            "details": {"field": "email"}
        })).with_status(StatusCode::BAD_REQUEST))
    })?;
    ```

## Fixtures and Testing

Error scenarios are documented in `testing_data/validation_errors/` with fixtures matching this contract.

### Example Fixture Structure

```
testing_data/validation_errors/
├── schema.json                    # JSON Schema for error payloads
├── missing_field.json             # Example: missing required field
├── invalid_type.json              # Example: wrong field type
├── not_found.json                 # Example: resource doesn't exist
├── unauthorized.json              # Example: auth failure
└── internal_error.json            # Example: server error
```

### Validation with Python Tests

All error responses are validated in `packages/python/tests/test_all_fixtures.py`:

```python
import json
from testing_data import load_fixture

def test_validation_error_schema():
    fixture = load_fixture('validation_errors/missing_field.json')
    payload = json.loads(fixture)

    # Validate structure
    assert 'error' in payload
    assert 'code' in payload
    assert 'details' in payload

    # Validate types
    assert isinstance(payload['error'], str)
    assert isinstance(payload['code'], str)
    assert isinstance(payload['details'], dict)
```

## Best Practices

### For API Designers

1. **Use specific error codes** - Prefer `validation_error` over generic `error`
2. **Include actionable details** - Help clients understand what went wrong
3. **Be consistent** - Use the same code for the same error across endpoints
4. **Document error codes** - Include error code documentation in API specs
5. **Avoid exposing internals** - Don't leak stack traces or internal state in error messages
6. **Add request IDs** - Include correlation IDs via middleware for debugging

### For Client Developers

1. **Check both code and status** - HTTP status indicates class, code provides specifics
2. **Log full payloads** - Include details for debugging
3. **Show user-friendly messages** - Use the `error` field for UI display
4. **Implement retry logic** - Handle `rate_limit_exceeded` with exponential backoff
5. **Handle unknown codes gracefully** - Future versions may add new error codes

### For Middleware Authors

1. **Preserve error format** - Don't transform errors, pass them through as-is
2. **Add context with details** - Use the `details` field to add middleware-specific info
3. **Respect error codes** - Don't override or re-map error codes
4. **Maintain HTTP status** - Map error codes to correct HTTP status consistently

## Versioning and Evolution

This error contract is stable and follows semantic versioning:

- **New error codes** (e.g., `rate_limit_exceeded`) are backwards compatible
- **Changes to payload structure** require major version bump
- **Changes to HTTP status codes** are considered breaking changes
- **New detail fields** are backwards compatible (treat unknown fields as optional)

## References

- [RFC 9457: Problem Details for HTTP APIs](https://tools.ietf.org/html/rfc9457)
- [ADR 0009: Panic Shielding](../adr/0009-panic-shielding.md)
- Core Implementation: `crates/spikard-core/src/errors.rs`
- JSON Schema: `testing_data/validation_errors/schema.json`
- Test Coverage: `packages/python/tests/test_all_fixtures.py`
