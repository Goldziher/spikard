```rust
use axum::{
    http::StatusCode,
    routing::post,
    Json, Router,
};
use axum_test::TestServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
struct UserCreate {
    #[validate(length(min = 1))]
    name: String,
    #[validate(range(min = 0, max = 150))]
    age: i32,
}

async fn create_user(
    Json(input): Json<UserCreate>,
) -> Result<Json<UserCreate>, (StatusCode, String)> {
    input
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Validation error: {}", e)))?;
    Ok(Json(input))
}

#[tokio::test]
async fn test_validation_failure() {
    let app = Router::new().route("/users", post(create_user));
    let server = TestServer::new(app).unwrap();

    // Invalid: age is string (will fail JSON parsing)
    let response = server
        .post("/users")
        .json(&json!({
            "name": "Bob",
            "age": "not a number"
        }))
        .await;

    assert_eq!(response.status_code(), 422);

    // Invalid: age out of range
    let response = server
        .post("/users")
        .json(&json!({
            "name": "Bob",
            "age": 200
        }))
        .await;

    assert_eq!(response.status_code(), 400);

    let body = response.text();
    assert!(body.to_lowercase().contains("validation"));
}
```
