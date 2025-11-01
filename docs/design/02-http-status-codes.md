# HTTP Status Code Strategy

**Date**: 2025-10-31
**Status**: Accepted
**Standards**: RFC 9110 (HTTP Semantics, June 2022, Internet Standard 97)
**Related**: [Validation Strategy](01-validation-strategy.md)

## Overview

This document defines Spikard's strategy for HTTP status codes, particularly distinguishing between **400 Bad Request** and **422 Unprocessable Content** for error responses.

## Guiding Principles

We follow IETF standards (RFC 9110) rather than framework-specific conventions. Our approach prioritizes clear semantic distinctions between syntactic errors (400) and semantic validation failures (422), aligned with the current HTTP specification.

## Status Code Usage

### 400 Bad Request

Use **400 Bad Request** for errors where the server cannot understand or parse the request. These are structural/syntactic errors:

**Examples:**
- Malformed JSON syntax (trailing comma, missing quotes, etc.)
- Invalid Content-Type header
- Wrong JSON value types (string instead of integer)
- Missing required Content-Length header with mismatch
- Unparseable URL encoding
- Missing required fields in request structure
- Field names that don't match the expected schema

**Code location**: Middleware layer (`crates/spikard-http/src/middleware.rs`)

**Example response:**
```json
{
  "error": "Content-Length header does not match actual body size"
}
```

### 422 Unprocessable Content

Use **422 Unprocessable Content** for errors where:
- The request is syntactically correct and the server understands it
- The content type is supported (not a 415 error)
- The JSON is valid and parseable
- Field types are correct
- But the values fail semantic validation or business logic rules

**Note**: RFC 9110 (June 2022) renamed this from "Unprocessable Entity" (RFC 4918) to "Unprocessable Content" to better reflect its semantics.

**Examples:**
- String doesn't match regex pattern (e.g., `pattern: r"^[A-Z]{3}$"`)
- Number outside allowed range (e.g., `gt=0`, `le=100`)
- String length constraints violated (e.g., `min_length=3`, `max_length=50`)
- Enum value not in allowed set
- UUID format invalid
- Date/datetime format invalid
- Email format invalid (semantic, not syntax)
- Business rule violations (insufficient funds, resource locked, etc.)

**Code location**: Parameter validator (`crates/spikard-http/src/parameters.rs`) and JSON Schema validation

**Example response:**
```json
{
  "detail": [
    {
      "type": "missing",
      "loc": ["query", "code"],
      "msg": "Field required",
      "input": null
    }
  ]
}
```

## Decision Matrix

| Error Type | Status Code | Location |
|-----------|-------------|----------|
| Malformed JSON | 400 | Middleware |
| Wrong Content-Type | 400 | Middleware |
| Content-Length mismatch | 400 | Middleware |
| Missing required parameter | 422 | Validator |
| Parameter type mismatch | 422 | Validator |
| Pattern validation failure | 422 | Validator |
| Range validation failure | 422 | Validator |
| Length constraint violation | 422 | Validator |
| Enum value not allowed | 422 | Validator |
| Format validation failure | 422 | Validator |

## Rationale

### Standards-Based Approach

Per **RFC 9110 Section 15.5.1** (400 Bad Request):
> "The 400 (Bad Request) status code indicates that the server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing)."

Per **RFC 9110 Section 15.5.23** (422 Unprocessable Content):
> "The 422 (Unprocessable Content) status code indicates that the server understands the content type of the request content (hence a 415 (Unsupported Media Type) status code is inappropriate), and the syntax of the request content is correct, but it was unable to process the contained instructions."

### Industry Alignment

Our approach aligns with:
- **IETF HTTP Standards**: RFC 9110 (June 2022)
- **GitHub's API**: Uses 400 for structural errors and 422 for validation errors
- **Common practice**: Separating parsing/structural errors from validation errors

### Implementation Benefits

1. **Clearer debugging**: Developers immediately know if the issue is request structure (400) or value validation (422)
2. **Better error handling**: Clients can handle 400 and 422 differently (e.g., retry logic, user feedback)
3. **Standards compliance**: Aligns with HTTP semantics and industry best practices
4. **Maintainability**: Clear guidelines for where to return each status code

## Migration Path

### Existing Tests

Tests currently expecting certain status codes need to be updated:

1. **Missing required parameters**: 422 → Keep as 422 (validation error)
2. **Malformed JSON**: No code → Add as 400
3. **Content-Length mismatch**: No code → Add as 400
4. **Pattern/range/length validation**: 422 → Keep as 422

### Fixtures to Update

Review and update these fixture categories:
- `testing_data/validation_errors/*.json` - Verify all expect 422
- `testing_data/query_params/*_missing.json` - Change to 422 if not already
- `testing_data/json_bodies/*_required.json` - Verify expect 422
- `testing_data/cookies/*_missing.json` - Change to 422 if not already

Any fixtures testing structural/parsing errors should expect 400.

## Future Considerations

### Other Status Codes

- **401 Unauthorized**: Authentication required but not provided
- **403 Forbidden**: Authenticated but not authorized for this resource
- **404 Not Found**: Route/resource doesn't exist
- **409 Conflict**: Request conflicts with current state (e.g., duplicate creation)
- **413 Payload Too Large**: Request body exceeds size limits
- **415 Unsupported Media Type**: Content-Type not supported

### Streaming and Multipart

For streaming multipart requests, parsing errors during stream processing should still return 400 if they're structural, or 422 if they're validation failures on parsed content.

## References

### IETF Standards
- [RFC 9110: HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110.html) (June 2022, Internet Standard 97)
  - [Section 15.5.1: 400 Bad Request](https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.1)
  - [Section 15.5.23: 422 Unprocessable Content](https://www.rfc-editor.org/rfc/rfc9110.html#section-15.5.23)
- [RFC 9111: HTTP Caching](https://www.rfc-editor.org/rfc/rfc9111.html) (June 2022, Internet Standard 98)
- [RFC 9112: HTTP/1.1](https://www.rfc-editor.org/rfc/rfc9112.html) (June 2022, Internet Standard 99)

### Related Resources
- [GitHub API - Client Errors](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#client-errors)
- [HTTP Status Code Registry](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)

### Historical Context
- RFC 7231 (obsoleted by RFC 9110) - Previous HTTP semantics specification
- RFC 4918 (WebDAV) - Originally introduced 422 as "Unprocessable Entity", now part of RFC 9110 as "Unprocessable Content"
