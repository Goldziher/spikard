# Logging & Tracing

Standardize request IDs, structured logs, and tracing across bindings.

## Inject request IDs

=== "Python"

    ```python
    @app.on_request
    async def request_id(request: dict[str, object]):
        headers = request.get("headers", {}) if isinstance(request, dict) else {}
        request_id = headers.get("x-request-id") or request.get("request_id")
        if isinstance(headers, dict):
            headers = {**headers, "x-request-id": request_id}
        return {**request, "headers": headers, "request_id": request_id}
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

    app.on_request do |request|
      headers = request[:headers] || {}
      headers["x-request-id"] ||= SecureRandom.uuid
      request.merge(headers: headers)
    end
    ```

=== "PHP"

    ```php
    use Spikard\Lifecycle\Request;
    use Spikard\Lifecycle\LifecycleResult;

    $app->onRequest(function (Request $request): LifecycleResult {
        $requestId = $request->headers['x-request-id'] ?? \Ramsey\Uuid\Uuid::uuid4()->toString();
        $request->headers['x-request-id'] = $requestId;
        error_log("Request received: {$requestId}");
        return LifecycleResult::continue($request);
    });
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;
    use uuid::Uuid;

    let mut app = App::new();
    app.on_request(|mut request: Request| {
        let request_id = request
            .headers
            .get("x-request-id")
            .and_then(|h| h.to_str().ok())
            .unwrap_or_else(|| &Uuid::new_v4().to_string());
        request.headers.insert("x-request-id", request_id.parse()?);
        tracing::info!(request_id = %request_id, "request received");
        Ok(request)
    });
    ```

## Tips
- Forward `x-request-id` from clients or generate one; include it in logs and errors.
- Prefer structured logs (JSON) and tracing exporters (OTel) where available.
- Keep log volume low in hot paths; push verbose data to debug-only logs.
