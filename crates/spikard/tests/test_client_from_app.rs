//! Smoke test for `spikard::testing::test_client_from_app`.
//!
//! Verifies that the public `App` type can be converted into a `TestClient`
//! and used to make in-process HTTP requests.

use axum::body::Body;
use axum::http::StatusCode;
use spikard::{App, Method, RouteBuilder, testing::test_client_from_app};

#[tokio::test]
async fn test_client_from_app_get_hello() {
    let mut app = App::new();
    app.route(
        RouteBuilder::new(Method::Get, "/"),
        |_ctx: spikard::RequestContext| async move {
            let body = serde_json::json!({"hello": "world"});
            let response = axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap();
            Ok(response)
        },
    )
    .expect("route registration");

    let client = test_client_from_app(app).expect("test client from app");

    let snapshot = client.get("/", None, None).await.expect("GET /");

    assert_eq!(snapshot.status, 200);

    let json = snapshot.json().expect("parse body as json");
    assert_eq!(json["hello"], "world");
}

#[tokio::test]
async fn test_client_from_app_post_echo() {
    let mut app = App::new();
    app.route(
        RouteBuilder::new(Method::Post, "/echo"),
        |ctx: spikard::RequestContext| async move {
            let body = ctx.body_value().clone();
            let response = axum::http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap();
            Ok(response)
        },
    )
    .expect("route registration");

    let client = test_client_from_app(app).expect("test client");

    let payload = serde_json::json!({"key": "value"});
    let snapshot = client
        .post("/echo", Some(payload.clone()), None, None, None, None)
        .await
        .expect("POST /echo");

    assert_eq!(snapshot.status, 200);
    let json = snapshot.json().expect("parse json");
    assert_eq!(json["key"], "value");
}
