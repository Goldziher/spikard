# RFC 9457 Error Response Format

## Problem Statement

Should Spikard use FastAPI-style error responses (`{"detail": [...]}`) or the standardized RFC 9457 Problem Details format?

## Standards Research (2024-2025)

### RFC 9457 (Problem Details for HTTP APIs)

Published July 2023, **RFC 9457** establishes the standard machine-readable format for HTTP API errors, obsoleting RFC 7807.

#### Required Format

**Content-Type**: `application/problem+json` (or `application/problem+xml` for XML)

**Structure**: JSON object with optional standard fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | URI | Optional | Problem type identifier (defaults to `"about:blank"`) |
| `title` | string | Optional | Brief human-readable summary (stable across occurrences) |
| `status` | number | Optional | HTTP status code (advisory, actual header takes precedence) |
| `detail` | string | Optional | Specific explanation for this occurrence |
| `instance` | URI | Optional | URI identifying this specific problem occurrence |
| `*` | any | Optional | Extension fields for problem-specific data |

#### Example from RFC 9457

```http
HTTP/1.1 403 Forbidden
Content-Type: application/problem+json

{
  "type": "https://example.com/probs/out-of-credit",
  "title": "You do not have enough credit.",
  "detail": "Your current balance is 30, but that costs 50.",
  "instance": "/account/12345/msgs/abc",
  "balance": 30,
  "accounts": ["/account/12345", "/account/67890"]
}
```

### Current Spikard Format (FastAPI-style)

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/json

{
  "detail": [
    {
      "type": "missing",
      "loc": ["body", "username"],
      "msg": "Field required",
      "input": ""
    }
  ]
}
```

## Decision

**Adopt RFC 9457 Problem Details format** for all HTTP error responses.

### Rationale

1. **Industry Standard**: RFC 9457 is the IETF-approved standard for API errors
2. **Interoperability**: Widely supported by tools, clients, and frameworks
3. **Extensibility**: Allows custom fields while maintaining standard structure
4. **Future-Proof**: Standards-based approach over framework-specific format
5. **Better Semantics**: Clear separation of error metadata vs. error details

### Migration Strategy

For validation errors (422), map FastAPI-style details to RFC 9457:

```json
{
  "type": "https://spikard.dev/errors/validation-error",
  "title": "Request Validation Failed",
  "status": 422,
  "detail": "The request body failed validation",
  "errors": [
    {
      "type": "missing",
      "loc": ["body", "username"],
      "msg": "Field required",
      "input": ""
    }
  ]
}
```

## Implementation

### Standard Error Types

Define standard `type` URIs for common errors:

```
https://spikard.dev/errors/validation-error       (422)
https://spikard.dev/errors/not-found              (404)
https://spikard.dev/errors/method-not-allowed     (405)
https://spikard.dev/errors/internal-server-error  (500)
https://spikard.dev/errors/bad-request            (400)
```

### Response Structure

```rust
#[derive(Serialize)]
struct ProblemDetails {
    #[serde(rename = "type")]
    type_uri: String,
    title: String,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    #[serde(flatten)]
    extensions: HashMap<String, Value>,
}
```

### Content-Type Header

Always set:
```
Content-Type: application/problem+json; charset=utf-8
```

### Examples

#### Validation Error (422)

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/problem+json

{
  "type": "https://spikard.dev/errors/validation-error",
  "title": "Request Validation Failed",
  "status": 422,
  "detail": "1 validation error in request body",
  "errors": [
    {
      "type": "missing",
      "loc": ["body", "username"],
      "msg": "Field required",
      "input": ""
    }
  ]
}
```

#### Not Found (404)

```http
HTTP/1.1 404 Not Found
Content-Type: application/problem+json

{
  "type": "https://spikard.dev/errors/not-found",
  "title": "Resource Not Found",
  "status": 404,
  "detail": "No route matches GET /api/users/999"
}
```

#### Internal Server Error (500)

```http
HTTP/1.1 500 Internal Server Error
Content-Type: application/problem+json

{
  "type": "https://spikard.dev/errors/internal-server-error",
  "title": "Internal Server Error",
  "status": 500,
  "detail": "An unexpected error occurred"
}
```

**In debug mode:**

```json
{
  "type": "https://spikard.dev/errors/internal-server-error",
  "title": "Internal Server Error",
  "status": 500,
  "detail": "Python handler raised KeyError",
  "exception": "KeyError: 'username'",
  "traceback": "Traceback (most recent call last):\n  ...",
  "request_data": {
    "path_params": {},
    "query_params": {},
    "body": {}
  }
}
```

## Consequences

### Positive
- ✅ Standards-compliant error responses
- ✅ Better client interoperability
- ✅ Clear, extensible structure
- ✅ Framework-agnostic approach

### Negative
- ⚠️ Breaking change from FastAPI format
- ⚠️ Need to update all tests
- ⚠️ Migration guide needed for users

### Mitigation
- Provide clear migration documentation
- Update all fixtures and tests
- Consider compatibility mode (optional)

## Alternative Considered

**Alternative**: Keep FastAPI-style `{"detail": [...]}` format

**Rejected because**:
- Not a standard (framework-specific)
- Poor interoperability with non-FastAPI tools
- Inconsistent with HTTP best practices
- RFC 9457 is specifically designed for this use case

## References

- [RFC 9457: Problem Details for HTTP APIs](https://www.rfc-editor.org/rfc/rfc9457.html) (July 2023)
- [RFC 9457 Section 3: The Problem Details JSON Object](https://www.rfc-editor.org/rfc/rfc9457.html#section-3)
- [RFC 9457 Section 4.1: application/problem+json](https://www.rfc-editor.org/rfc/rfc9457.html#section-4.1)

## Status

**Decided**: 2025-01-01

## Implementation Tracking

- [ ] Create ProblemDetails struct in Rust
- [ ] Update validation error responses
- [ ] Update handler error responses
- [ ] Update middleware error responses
- [ ] Set Content-Type: application/problem+json
- [ ] Update all test fixtures
- [ ] Update test expectations
- [ ] Add migration guide to docs
