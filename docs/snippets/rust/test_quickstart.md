```rust
use axum::{routing::get, Router};
use axum_test::TestServer;
use serde_json::json;

async fn hello() -> axum::Json<serde_json::Value> {
    axum::Json(json!({"message": "Hello, World!"}))
}

#[tokio::test]
async fn test_hello() {
    let app = Router::new().route("/hello", get(hello));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/hello").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.json::<serde_json::Value>(),
        json!({"message": "Hello, World!"})
    );
}
```
