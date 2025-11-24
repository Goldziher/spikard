# Logging & Tracing

Standardize request IDs, structured logs, and tracing across bindings.

## Inject request IDs

=== "Python"

    ```python
    def request_id(ctx, next_fn):
        ctx.state["request_id"] = ctx.headers.get("x-request-id") or ctx.state.get("request_id")
        return next_fn()

    app.use(request_id)
    ```

=== "TypeScript"

    ```typescript
    import { Spikard, type Request } from "spikard";

    const app = new Spikard();

    app.onRequest(async (request: Request) => {
      const requestId = request.headers["x-request-id"] ?? crypto.randomUUID();
      return { ...request, headers: { ...request.headers, "x-request-id": requestId } };
    });
    ```

=== "Ruby"

    ```ruby
    require "securerandom"

    app.use do |ctx, next_middleware|
      ctx.headers["x-request-id"] ||= SecureRandom.uuid
      next_middleware.call
    end
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;
    use tower_http::trace::TraceLayer;

    let mut app = App::new();
    app.layer(TraceLayer::new_for_http());
    ```

## Tips
- Forward `x-request-id` from clients or generate one; include it in logs and errors.
- Prefer structured logs (JSON) and tracing exporters (OTel) where available.
- Keep log volume low in hot paths; push verbose data to debug-only logs.
