```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ListUsersQuery {
    #[serde(default = "default_page")]
    page: i32,
    #[serde(default = "default_limit")]
    limit: i32,
    sort_by: Option<String>,
    min_age: Option<i32>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 10 }

app.route(
    get("/users"),
    |ctx: Context| async move {
        let query: ListUsersQuery = ctx.query()?;

        if query.limit > 100 {
            return Err(Error::BadRequest("limit cannot exceed 100"));
        }

        Ok(Json(json!({
            "page": query.page,
            "limit": query.limit,
            "users": []
        })))
    }
)?;
```
