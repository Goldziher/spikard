# Logging & Tracing

Spikard can optionally enable request ID generation and HTTP trace logging via `ServerConfig`.

## Inject request IDs

Request ID handling is configured at the server level:

=== "Python"

    ```python
    from spikard import App, ServerConfig

    config = ServerConfig(
        enable_request_id=True,  # Auto-generate or forward x-request-id
        enable_http_trace=True,  # Log all requests/responses
    )

    app = App().config(config)

    async def my_handler(ctx):
        # Request ID is available via headers
        request_id = ctx.header("x-request-id")
        return {"request_id": request_id}

    app.get("/", my_handler)
    ```

=== "TypeScript"

    ```typescript
    import { App, ServerConfig } from "@spikard/node";

    const config = new ServerConfig({
      enableRequestId: true,  // Auto-generate or forward x-request-id
      enableHttpTrace: true,  // Log all requests/responses
    });

    const app = new App().config(config);

    app.get("/", async (ctx) => {
      const requestId = ctx.header("x-request-id");
      return { request_id: requestId };
    });
    ```

=== "Ruby"

    ```ruby
    require "spikard"

    config = Spikard::ServerConfig.new(
      enable_request_id: true,  # Auto-generate or forward x-request-id
      enable_http_trace: true,  # Log all requests/responses
    )

    app = Spikard::App.new.config(config)

    app.get("/") do |ctx|
      request_id = ctx.header("x-request-id")
      { request_id: request_id }
    end
    ```

=== "PHP"

    ```php
    <?php
    declare(strict_types=1);

    namespace App;

    use Spikard\Php\App;
    use Spikard\Php\ServerConfig;

    $config = new ServerConfig();
    $config->enableRequestId(true);  // Auto-generate or forward x-request-id
    $config->enableHttpTrace(true);  // Log all requests/responses

    $app = (new App())->config($config);

    $app->get("/", function ($ctx) {
        $requestId = $ctx->header("x-request-id");
        return ["request_id" => $requestId];
    });
    ```

=== "Rust"

    ```rust
    use spikard::{App, ServerConfig, get, RequestContext};
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let config = ServerConfig::default()
            .with_enable_request_id(true)    // Auto-generate or forward x-request-id
            .with_enable_http_trace(true);   // Log all requests/responses

        let mut app = App::new().config(config);

        app.route(
            get("/"),
            |ctx: RequestContext| async move {
                let request_id = ctx.header("x-request-id").unwrap_or("unknown");
                let response_body = json!({"request_id": request_id});

                Ok(axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .body(axum::body::Body::from(response_body.to_string()))?)
            },
        )?;

        app.run().await?;
        Ok(())
    }
    ```

## Tips

- Request ID generation is opt-in: enable `enable_request_id` in `ServerConfig`
- HTTP trace logging is opt-in: enable `enable_http_trace` in `ServerConfig`
- Request IDs are forwarded via `x-request-id` headers in responses
- Include request IDs in error responses and log output for tracing
