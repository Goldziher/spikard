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

## Examples per binding

=== "Python"

    ```python
    from spikard import Response

    @app.get("/fail")
    async def fail() -> Response:
        return Response(
            {
                "type": "https://spikard.dev/errors/validation",
                "title": "Validation failed",
                "detail": "email is invalid",
                "status": 422,
            },
            status=422,
        )
    ```

=== "TypeScript"

    ```typescript
    const app = new Spikard();

    app.addRoute(
      { method: "GET", path: "/fail", handler_name: "fail", is_async: true },
      async () => ({
        statusCode: 422,
        body: {
          type: "https://spikard.dev/errors/validation",
          title: "Validation failed",
          detail: "email is invalid",
          status: 422,
        },
      }),
    );
    ```

=== "Ruby"

    ```ruby
    app.get "/fail" do |_request|
      [{ type: "https://spikard.dev/errors/validation", title: "Validation failed", status: 422 }, 422]
    end
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;

    app.route(get("/fail"), |_ctx: Context| async move {
        Ok(Json(json!({
            "type": "https://spikard.dev/errors/validation",
            "title": "Validation failed",
            "status": 422
        })).with_status(StatusCode::UNPROCESSABLE_ENTITY))
    })?;
    ```
