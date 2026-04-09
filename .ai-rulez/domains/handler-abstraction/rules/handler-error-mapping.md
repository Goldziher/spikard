---
name: handler-error-mapping
priority: high
---

Handler errors must convert to HTTP status:

- ValidationError -> 400 Bad Request
- NotFound -> 404 Not Found
- Unauthorized -> 401 Unauthorized
- InternalError -> 500 Internal Server Error
- All errors must return ProblemDetails response
