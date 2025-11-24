```rust
use axum::response::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::{get, post, App, RequestContext};

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: RequestContext| async move {
        let id = ctx.path_param("id").unwrap_or("0").parse::<i64>().unwrap_or_default();
        Ok(Json(User { id, name: "Alice".into() }).into())
    })?;

    app.route(
        post("/users").request_body::<User>().response_body::<User>(),
        |ctx: RequestContext| async move {
            let user: User = ctx.json()?;
            Ok(Json(user).into())
        },
    )?;

    app.run().await?;
    Ok(())
}
```
