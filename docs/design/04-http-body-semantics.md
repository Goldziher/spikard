# HTTP Request Body Semantics

## Problem Statement

Should Spikard enforce request body schemas based on HTTP method (e.g., require `body_schema` for POST/PUT/PATCH)?

## Research & Standards (2024-2025)

### RFC 9110 (HTTP Semantics, June 2022 - Current Standard)

The current HTTP specification (RFC 9110) defines method semantics as follows:

| Method | Body Semantics | RFC 9110 Language |
|--------|----------------|-------------------|
| GET | Semantically undefined | Bodies "have no defined semantics" but not forbidden |
| POST | Expected | "A payload within a POST request message represents information or data to be processed" |
| PUT | Expected | PUT requests convey "a representation of the resource to create or replace" |
| DELETE | Optional | Bodies allowed but not required |
| PATCH | Implied required (RFC 5789) | "The enclosed entity contains a set of instructions" |
| OPTIONS | Optional | Bodies allowed but not required |
| TRACE | Forbidden | No request body semantics |
| HEAD | Not typically used | Mirrors GET semantics |

### Key Findings

1. **No method syntactically requires a body** - The HTTP spec uses semantic language ("expected", "represents") not syntactic requirements ("MUST", "REQUIRED")

2. **Any method can have a body** - RFC 9110 Section 6.4:
   > "A client SHOULD NOT generate content in a request message unless the intended semantics of that request method includes defined semantics for the content."

3. **DELETE can have bodies** - While uncommon, DELETE with body is valid HTTP

4. **POST/PUT/PATCH can omit bodies** - Valid use cases:
   - POST to trigger actions without data (e.g., `/orders/123/cancel`)
   - PUT with all data in URL (rare but valid)
   - PATCH for operations expressed via custom headers

## Decision

**Make `body_schema` optional for ALL HTTP methods.**

### Rationale

1. **Standards Compliance**: HTTP spec doesn't mandate bodies for any method
2. **Developer Freedom**: Let API designers make semantic decisions
3. **Real-World Use Cases**: Many valid APIs use POST/PUT/PATCH without bodies
4. **Principle of Least Surprise**: Don't enforce rules stricter than HTTP itself

### Implementation

```python
# All methods accept optional body_schema
def post(path: str, *, body_schema: dict[str, Any] | None = None)
def put(path: str, *, body_schema: dict[str, Any] | None = None)
def patch(path: str, *, body_schema: dict[str, Any] | None = None)
def delete(path: str, *, body_schema: dict[str, Any] | None = None)
def get(path: str, *, body_schema: dict[str, Any] | None = None)
```

### Validation Behavior

When `body_schema` is provided:
- Rust validates request body against schema
- Returns 422 on validation failure
- Passes validated JSON to Python handler

When `body_schema` is `None`:
- No validation performed
- Body passed through as-is (if present)
- Python handler receives raw request data

## Examples

### Valid Use Cases Without Body Schema

```python
# POST for actions
@post("/orders/123/cancel")
async def cancel_order():
    return {"status": "cancelled"}

# PUT for toggle/flag operations
@put("/users/me/notifications")
async def enable_notifications():
    return {"notifications": True}

# PATCH via headers
@patch("/documents/123", body_schema=None)
async def patch_document(headers: dict):
    # Operation described in custom header
    return {"ok": True}

# DELETE with body (for bulk operations)
@delete("/items", body_schema={"type": "array", "items": {"type": "string"}})
async def delete_items(body: list[str]):
    return {"deleted": len(body)}
```

### Typical Use Cases With Body Schema

```python
# POST for resource creation
@post("/users", body_schema={
    "type": "object",
    "required": ["email", "name"],
    "properties": {
        "email": {"type": "string", "format": "email"},
        "name": {"type": "string"}
    }
})
async def create_user(body: dict):
    return {"id": 1, **body}

# PUT for full replacement
@put("/users/123", body_schema={
    "type": "object",
    "required": ["email", "name"],
    "properties": {
        "email": {"type": "string"},
        "name": {"type": "string"}
    }
})
async def update_user(body: dict):
    return {"id": 123, **body}
```

## Consequences

### Positive
- ✅ HTTP standards compliant
- ✅ Maximum developer flexibility
- ✅ Supports all valid HTTP patterns
- ✅ No artificial restrictions

### Negative
- ⚠️ Developers must understand HTTP semantics
- ⚠️ No framework-enforced "best practices"
- ⚠️ Possible confusion for beginners

### Mitigation
- Document HTTP method semantics clearly
- Provide examples showing typical patterns
- Lint/warn about unusual patterns (e.g., GET with body)

## Alternative Considered

**Alternative**: Require `body_schema` for POST/PUT/PATCH

**Rejected because**:
- Violates HTTP spec (no method requires bodies)
- Breaks valid use cases (action endpoints, toggles)
- Opinionated beyond framework responsibilities
- Incompatible with REST hypermedia patterns

## References

- [RFC 9110: HTTP Semantics](https://httpwg.org/specs/rfc9110.html) (June 2022)
- [RFC 5789: PATCH Method for HTTP](https://datatracker.ietf.org/doc/html/rfc5789) (March 2010)
- RFC 9110 Section 9.3: Method Definitions
- RFC 9110 Section 6.4: Content

## Status

**Decided**: 2025-01-01

## Implementation Tracking

- [ ] Update routing.py: Make body_schema optional for all methods
- [ ] Update test generator: Don't require body_schema for any method
- [ ] Update documentation: Add HTTP body semantics guide
- [ ] Add examples: Show both with-body and without-body patterns
