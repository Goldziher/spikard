# Routing Basics

Routing is uniform across bindings: define an `App`, register routes with typed parameters, and return typed responses.

## Declare routes

=== "Python"

    ```python
    from spikard import App

    app = App()

    @app.get("/health")
    async def health() -> dict:
        return {"status": "ok"}

    @app.post("/users")
    async def create_user(user: User) -> User:
        return user
    ```

=== "TypeScript"

    ```typescript
    import { App } from "spikard";

    const app = new App();

    app.get("/health", () => ({ status: "ok" }));
    app.post("/users", ({ body }) => body);
    ```

=== "Ruby"

    ```ruby
    require "spikard"

    App = Spikard::App.new

    App.get("/health") { { status: "ok" } }
    App.post("/users") { |ctx| ctx.json }
    ```

=== "Rust"

    ```rust
    use spikard::prelude::*;

    let mut app = App::new();

    app.route(get("/health"), |_ctx: Context| async { Ok(Json(json!({"status": "ok"}))) })?;
    app.route(post("/users"), |ctx: Context| async move {
        let user: serde_json::Value = ctx.json()?;
        Ok(Json(user))
    })?;
    ```

## Path and query params

=== "Python"

    ```python
    @app.get("/orders/{order_id:int}")
    async def get_order(order_id: int, include_details: bool = False) -> dict:
        return {"id": order_id, "details": include_details}
    ```

=== "TypeScript"

    ```typescript
    app.get("/orders/:orderId", ({ params, query }) => ({
      id: Number(params.orderId),
      details: query.details === "true",
    }));
    ```

=== "Ruby"

    ```ruby
    App.get("/orders/:order_id") do |ctx|
      {
        id: ctx.params[:order_id].to_i,
        details: ctx.query["details"] == "true",
      }
    end
    ```

=== "Rust"

    ```rust
    app.route(get("/orders/:order_id"), |ctx: Context| async move {
        let id = ctx.path_param("order_id").unwrap_or("0");
        let details: serde_json::Value = ctx.query()?;
        Ok(Json(json!({
            "id": id.parse::<i64>().unwrap_or_default(),
            "details": details.get("details").and_then(|d| d.as_bool()).unwrap_or(false)
        })))
    })?;
    ```

## Best practices
- Keep handlers small and pure; push IO into services.
- Prefer DTOs for shared schemas so codegen can derive OpenAPI/AsyncAPI.
- Use per-route middleware when sensitive endpoints need extra auth/logging.
