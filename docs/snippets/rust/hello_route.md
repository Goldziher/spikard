```rust
use serde::{Deserialize, Serialize};
use spikard::prelude::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    app.route(get("/users/:id"), |ctx: Context| async move {
        let id = ctx.path_param("id").unwrap_or("0").parse::<i64>().unwrap_or_default();
        Ok(Json(User { id, name: "Alice".into() }))
    })?;

    app.run().await?;
    Ok(())
}
```
