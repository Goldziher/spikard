```rust
use uuid::Uuid;

app.route(
    get("/users/:user_id/posts/:post_id"),
    |ctx: Context| async move {
        let user_id: Uuid = ctx.param("user_id")?.parse()?;
        let post_id: i32 = ctx.param("post_id")?.parse()?;

        Ok(Json(json!({
            "user_id": user_id.to_string(),
            "post_id": post_id,
            "title": "Sample Post"
        })))
    }
)?;
```
