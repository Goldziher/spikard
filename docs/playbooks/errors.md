# Error Handling

Standardize errors so clients can rely on status codes and payload shape.

## Basic patterns

=== "Python"

    ```python
    from spikard import Response

    @app.get("/fail")
    async def fail() -> Response:
        return Response({"error": "bad"}, status=400)
    ```

=== "TypeScript"

    ```typescript
    import { Spikard } from "spikard";

    const app = new Spikard();

    app.addRoute(
      { method: "GET", path: "/fail", handler_name: "fail", is_async: true },
      async () => ({ statusCode: 400, body: { error: "bad" } }),
    );
    ```

=== "Ruby"

    ```ruby
    app.get "/fail" do |_request|
      [{ error: "bad" }, 400]
    end
    ```

=== "PHP"

    ```php
    use Spikard\Http\Response;

    #[Get("/fail")]
    public function fail(): Response
    {
        return Response::json(['error' => 'bad'], 400);
    }
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;

    app.route(get("/fail"), |_ctx: Context| async {
        Ok(Json(json!({"error": "bad"})).with_status(StatusCode::BAD_REQUEST))
    })?;
    ```

## Tips
- Prefer structured bodies (RFC 9457 style) with `type`, `title`, `detail`, `status` fields.
- Propagate request IDs in errors for tracing.
- Short-circuit auth/validation failures in middleware when possible.
