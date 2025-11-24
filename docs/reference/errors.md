# Errors

Spikard returns structured error responses that align with RFC 9457 so clients can reliably parse failures.

## Format
```json
{
  "type": "https://spikard.dev/errors/validation",
  "title": "Validation failed",
  "status": 422,
  "detail": "Field 'email' is not a valid address",
  "instance": "/users",
  "errors": [
    { "path": "/email", "message": "must be an email" }
  ],
  "request_id": "c90d52c2-4c34-4b42-b028-a5a48e57b1f7"
}
```

## Categories
- **Validation** – 400/422 with field-level errors
- **Auth** – 401/403 with reason codes
- **Not Found** – 404 with path metadata
- **Server** – 500 with opaque message; detailed traces stay in logs unless explicitly exposed

## Best Practices
- Keep error types stable; clients should depend on `type` and `status` rather than raw messages.
- Add correlation/request IDs via middleware and return them in error payloads for debugging.
- Use domain-specific `type` URLs when exposing business errors.

## Binding notes
- **Python/TypeScript/Ruby**: return structured objects/hashes; include `status`/`statusCode` and a predictable body shape.
- **Rust**: use `Json(..).with_status(StatusCode::...)` or return typed error responses from handlers/middleware.
