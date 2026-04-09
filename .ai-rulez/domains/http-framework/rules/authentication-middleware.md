---
name: authentication-middleware
priority: high
---

JWT validation must check algorithm, audience, issuer, and expiration claims.
API Key validation must support multiple keys and custom header names.
Auth failures must return 401 Unauthorized with proper ProblemDetails.
Auth config must be per-route via route metadata, not global.
