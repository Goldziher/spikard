# Error Handling

Standardize errors so clients can rely on status codes and payload shape.

## Basic patterns

=== "Python"

    ```python
    from spikard import App, Response

    app = App()

    async def fail(ctx):
        return {"error": "bad"}

    app.get("/fail", fail)
    ```

=== "TypeScript"

    ```typescript
    import { App } from "@spikard/node";

    const app = new App();

    app.get("/fail", async () => {
      return { error: "bad" };
    });
    ```

=== "Ruby"

    ```ruby
    require "spikard"

    app = Spikard::App.new

    app.get("/fail") do |_request|
      { error: "bad" }
    end
    ```

=== "PHP"

    ```php
    <?php
    declare(strict_types=1);

    namespace App;

    use Spikard\Php\App;
    use Spikard\Php\Response;

    $app = new App();

    $app->route(
        "GET",
        "/fail",
        function () {
            return new Response(
                status: 400,
                headers: ["content-type" => "application/json"],
                body: json_encode(["error" => "bad"])
            );
        }
    );
    ```

=== "Rust"

    ```rust
    use spikard::{App, get, RequestContext, Response};
    use axum::http::StatusCode;
    use serde_json::json;

    let mut app = App::new();

    app.route(
        get("/fail"),
        |_ctx: RequestContext| async {
            let response = Response {
                status_code: 400,
                content: Some(json!({"error": "bad"})),
                headers: Default::default(),
            };
            Ok(axum::http::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(axum::body::Body::from(response.content.unwrap().to_string()))?)
        },
    )?;
    ```

## Tips

- Prefer structured bodies (RFC 9457 style) with `type`, `title`, `detail`, `status` fields.
- Propagate request IDs in errors for tracing.
- Short-circuit auth/validation failures in middleware when possible.
