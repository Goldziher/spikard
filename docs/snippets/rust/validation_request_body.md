```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct CreateUserRequest {
    email: String,
    #[schemars(range(min = 18))]
    age: i32,
    username: String,
}

app.route(
    post("/users").request_body::<CreateUserRequest>(),
    |ctx: Context| async move {
        let user: CreateUserRequest = ctx.json()?;
        Ok(Json(json!({
            "id": "usr_123",
            "email": user.email,
            "age": user.age,
            "username": user.username
        })))
    }
)?;
```
