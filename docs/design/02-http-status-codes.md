# HTTP Status Code Strategy

**Date**: 2025-10-31
**Status**: Draft
**Related**: [Validation Strategy](01-validation-strategy.md)

## Overview

This document defines Spikard's strategy for HTTP status codes, particularly distinguishing between **400 Bad Request** and **422 Unprocessable Entity** for error responses.

## Guiding Principles

We diverge from FastAPI's status code conventions to follow industry best practices based on RFC 7231 and GitHub's API guidelines. Our approach prioritizes clear semantic distinctions between syntax and validation errors.

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

### 422 Unprocessable Entity

Use **422 Unprocessable Entity** for errors where:
- The request is syntactically correct and the server understands it
- The JSON is valid and parseable
- Field types are correct
- But the values fail semantic validation or business logic rules

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

### Why Not Follow FastAPI?

FastAPI returns 422 for almost all client errors, including missing required fields. This overloads 422 with too many error types and loses the semantic distinction between "I don't understand your request" (400) and "I understand your request but the values are invalid" (422).

### Industry Alignment

Our approach aligns with:
- **GitHub's API**: Uses 400 for structural errors and 422 for validation errors
- **RFC 7231**: "The 400 (Bad Request) status code indicates that the server cannot or will not process the request due to something that is perceived to be a client error"
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

- [StackOverflow: 400 vs 422](https://stackoverflow.com/questions/16133923/400-vs-422-response-to-post-of-data)
- [RFC 7231 § 6.5.1 - 400 Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)
- [RFC 4918 § 11.2 - 422 Unprocessable Entity](https://tools.ietf.org/html/rfc4918#section-11.2)
- [GitHub API v3 - Client Errors](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#client-errors)
