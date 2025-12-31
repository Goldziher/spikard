```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: String,
    email: String,
    age: i32,
}

#[derive(Serialize, JsonSchema)]
struct UserListResponse {
    users: Vec<User>,
    total: i32,
    page: i32,
}

app.route(
    get("/users").response_body::<UserListResponse>(),
    |_ctx: Context| async move {
        let users = vec![
            User {
                id: "usr_1".to_string(),
                email: "alice@example.com".to_string(),
                age: 30,
            },
            User {
                id: "usr_2".to_string(),
                email: "bob@example.com".to_string(),
                age: 25,
            },
        ];

        let response = UserListResponse {
            total: users.len() as i32,
            page: 1,
            users,
        };

        // Serialization validates against schema
        Ok(Json(response))
    }
)?;
```
