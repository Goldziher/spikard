# Requests & Responses

Handlers receive a context object tailored to each binding but backed by the same Rust data model.

## Read request data

=== "Python"

    ```python
    from typing import Optional

    @app.post("/orders/{order_id:int}")
    async def update_order(order_id: int, order: Order, verbose: Optional[bool] = False) -> Order:
        # order is validated before the handler runs
        if verbose:
            print("updating", order_id)
        return order
    ```

=== "TypeScript"

    ```typescript
    app.post("/orders/:orderId", ({ params, query, headers, body }) => {
      const order = Order.parse(body);
      return {
        ...order,
        id: Number(params.orderId),
        requestId: headers["x-request-id"],
        verbose: query.verbose === "true",
      };
    });
    ```

=== "Ruby"

    ```ruby
    App.post("/orders/:order_id") do |ctx|
      order = ctx.json
      {
        **order,
        id: ctx.params[:order_id].to_i,
        request_id: ctx.headers["x-request-id"],
      }
    end
    ```

=== "Rust"

    ```rust
    app.route(post("/orders/:order_id"), |ctx: Context| async move {
        let mut order: serde_json::Value = ctx.json()?;
        let id = ctx.path_param("order_id").unwrap_or("0");
        let verbose: serde_json::Value = ctx.query().unwrap_or_default();
        if let Some(map) = order.as_object_mut() {
            map.insert("id".into(), serde_json::json!(id.parse::<i64>().unwrap_or_default()));
            map.insert("verbose".into(), verbose.get("verbose").cloned().unwrap_or(serde_json::json!(false)));
        }
        Ok(Json(order))
    })?;
    ```

## Return responses

=== "Python"

    ```python
    @app.get("/health")
    async def health() -> dict:
        return {"status": "ok"}
    ```

=== "TypeScript"

    ```typescript
    app.get("/health", () => ({ status: "ok" }));
    ```

=== "Ruby"

    ```ruby
    App.get("/health") { { status: "ok" } }
    ```

=== "Rust"

    ```rust
    app.route(get("/health"), |_ctx: Context| async {
        Ok(Json(serde_json::json!({"status": "ok"})))
    })?;
    ```

## Tips
- Use DTOs/schemas so validation runs before your handler executes.
- Prefer returning plain values/structs; the runtime will serialize and set content types.
- For streaming/WebSocket/SSE, see the streaming section in the concepts docs.
