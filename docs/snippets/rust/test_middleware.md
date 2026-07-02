```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use axum_test::TestServer;
use serde_json::json;

async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(token) if token.starts_with("Bearer ") => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn protected() -> axum::Json<serde_json::Value> {
    axum::Json(json!({"data": "secret"}))
}

#[tokio::test]
async fn test_auth_middleware() {
    let app = Router::new()
        .route("/protected", get(protected))
        .layer(middleware::from_fn(auth_middleware));

    let server = TestServer::new(app).unwrap();

    // Without auth - should fail
    let response = server.get("/protected").await;
    assert_eq!(response.status_code(), 401);

    // With auth - should succeed
    let response = server
        .get("/protected")
        .add_header("authorization", "Bearer token123")
        .await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.json::<serde_json::Value>(),
        json!({"data": "secret"})
    );
}
```
