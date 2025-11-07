//! Simple Spikard Rust baseline server for benchmarking
//!
//! This is a minimal HTTP server using raw Axum (what Spikard uses internally)
//! to establish performance baseline.

use axum::{
    Router,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get port from command line or default to 8000
    let port: u16 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(8000);

    let app = Router::new()
        .route("/health", get(health))
        .route("/query", get(query_handler))
        .route("/users", post(create_user))
        .route("/users/{user_id}", get(get_user))
        .route("/validated", post(validated));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Starting server on port {}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok"}))
}

/// Simple GET with query parameter
#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

async fn query_handler(Query(params): Query<QueryParams>) -> impl IntoResponse {
    format!("foo bar {}", params.query)
}

/// Create user with JSON body
#[derive(Deserialize, Serialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
    let user = UserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    (StatusCode::CREATED, Json(user))
}

/// Get user by path parameter
async fn get_user(Path(user_id): Path<u32>) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": user_id,
        "name": format!("User {}", user_id)
    }))
}

/// Validated input
#[derive(Deserialize)]
struct ValidatedRequest {
    value: i32,
}

async fn validated(Json(payload): Json<ValidatedRequest>) -> impl IntoResponse {
    // Simple validation: value must be > 0
    if payload.value <= 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "value must be greater than 0"
            })),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "value": payload.value
        })),
    )
}
