```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spikard::prelude::*;

#[derive(Deserialize, Serialize, JsonSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: Context| async move {
        let id: u64 = ctx.path_param("id").unwrap_or("0").parse().unwrap_or(0);
        Ok(Json(User {
            id,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }))
    })?;

    app.route(
        post("/users")
            .request_body::<User>()
            .response_body::<User>(),
        |ctx: Context| async move {
            let user: User = ctx.json()?;
            Ok(Json(user))
        },
    )?;

    app.run().await?;
    Ok(())
}
```
