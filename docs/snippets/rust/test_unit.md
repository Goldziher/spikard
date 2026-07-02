```rust
use axum::{routing::post, Json, Router};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn create_user(Json(input): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        name: input.name,
        email: input.email,
    })
}

#[tokio::test]
async fn test_user_creation() {
    let app = Router::new().route("/users", post(create_user));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/users")
        .json(&json!({
            "name": "Alice",
            "email": "alice@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), 200);

    let user: User = response.json();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}
```
