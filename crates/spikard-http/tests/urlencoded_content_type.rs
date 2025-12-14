#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Integration coverage for validate_content_type_middleware with urlencoded bodies.

use axum::http::{HeaderMap, StatusCode};
use axum::middleware;
use axum::routing::post;
use axum::{Extension, Router};
use spikard_http::middleware::{RouteInfo, RouteRegistry, validate_content_type_middleware};
use std::collections::HashMap;
use std::sync::Arc;

/// Build a router with the content-type middleware and a route registry entry.
fn build_router(registry: RouteRegistry) -> Router {
    Router::new()
        .route(
            "/forms",
            post(|headers: HeaderMap, body: axum::body::Bytes| async move {
                let content_type = headers
                    .get(axum::http::header::CONTENT_TYPE)
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or_default()
                    .to_string();
                let body_str = String::from_utf8(body.to_vec()).unwrap();
                (
                    StatusCode::OK,
                    axum::Json(serde_json::json!({ "content_type": content_type, "body": body_str })),
                )
            }),
        )
        .layer(Extension(registry))
        .layer(middleware::from_fn(validate_content_type_middleware))
}

#[tokio::test]
async fn urlencoded_body_is_transformed_to_json() {
    let registry: RouteRegistry = Arc::new(HashMap::from([(
        ("POST".to_string(), "/forms".to_string()),
        RouteInfo {
            expects_json_body: true,
        },
    )]));

    let app = build_router(registry);
    let server = axum_test::TestServer::new(app).expect("start test server");

    let response = server
        .post("/forms")
        .text("name=alice&active=true&count=3&empty=")
        .content_type("application/x-www-form-urlencoded")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let payload: serde_json::Value = response.json();
    assert_eq!(payload["content_type"], "application/json");

    let body_json: serde_json::Value =
        serde_json::from_str(payload["body"].as_str().expect("body string")).expect("valid json");
    assert_eq!(body_json["name"], "alice");
    assert_eq!(body_json["active"], true);
    assert_eq!(body_json["count"], 3);
    assert_eq!(body_json["empty"], "");
}

#[tokio::test]
async fn invalid_charset_on_json_returns_unsupported_media_type() {
    let registry: RouteRegistry = Arc::new(HashMap::from([(
        ("POST".to_string(), "/forms".to_string()),
        RouteInfo {
            expects_json_body: true,
        },
    )]));

    let app = build_router(registry);
    let server = axum_test::TestServer::new(app).expect("start test server");

    let response = server
        .post("/forms")
        .text("{\"name\":\"alice\"}")
        .content_type("application/json; charset=utf-16")
        .await;

    assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let body: serde_json::Value = response.json();
    assert_eq!(
        body["type"],
        serde_json::Value::String("https://spikard.dev/errors/unsupported-charset".to_string())
    );
}
