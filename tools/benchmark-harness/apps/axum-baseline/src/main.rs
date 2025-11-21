//! Pure Rust HTTP server for baseline workload benchmarking
//!
//! This server implements all workload types to establish performance baselines
//! before testing language bindings.

use axum::{
    extract::{Json, Multipart, Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "spikard-rust-bench")]
#[command(about = "Pure Rust HTTP server for baseline benchmarking")]
struct Args {
    /// Port to listen on
    #[arg(default_value = "8000")]
    port: u16,
}

// ============================================================================
// JSON Body Workloads
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct SmallJson {
    id: u64,
    name: String,
    active: bool,
    count: i32,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MediumJson {
    id: u64,
    metadata: HashMap<String, Value>,
    items: Vec<SmallJson>,
    description: String,
}

async fn post_json_small(Json(payload): Json<SmallJson>) -> Json<SmallJson> {
    Json(payload)
}

async fn post_json_medium(Json(payload): Json<MediumJson>) -> Json<MediumJson> {
    Json(payload)
}

async fn post_json_large(Json(payload): Json<Value>) -> Json<Value> {
    Json(payload)
}

async fn post_json_very_large(Json(payload): Json<Value>) -> Json<Value> {
    Json(payload)
}

// ============================================================================
// Multipart Form Workloads
// ============================================================================

async fn post_multipart_small(mut multipart: Multipart) -> impl IntoResponse {
    let mut files_received = 0;
    let mut total_bytes = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();
        total_bytes += data.len();
        files_received += 1;
        tracing::debug!("Received field {}: {} bytes", name, data.len());
    }

    Json(serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    }))
}

async fn post_multipart_medium(mut multipart: Multipart) -> impl IntoResponse {
    let mut files_received = 0;
    let mut total_bytes = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        total_bytes += data.len();
        files_received += 1;
    }

    Json(serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    }))
}

async fn post_multipart_large(mut multipart: Multipart) -> impl IntoResponse {
    let mut files_received = 0;
    let mut total_bytes = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        total_bytes += data.len();
        files_received += 1;
    }

    Json(serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    }))
}

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

async fn post_urlencoded_simple(
    axum::extract::Form(form): axum::extract::Form<HashMap<String, String>>,
) -> Json<HashMap<String, String>> {
    Json(form)
}

async fn post_urlencoded_complex(
    axum::extract::Form(form): axum::extract::Form<HashMap<String, String>>,
) -> Json<HashMap<String, String>> {
    Json(form)
}

// ============================================================================
// Path Parameter Workloads
// ============================================================================

async fn get_path_simple(Path(id): Path<String>) -> Json<Value> {
    Json(serde_json::json!({ "id": id }))
}

async fn get_path_multiple(Path((user_id, post_id)): Path<(String, String)>) -> Json<Value> {
    Json(serde_json::json!({
        "user_id": user_id,
        "post_id": post_id
    }))
}

async fn get_path_deep(
    Path((org, team, project, resource, id)): Path<(String, String, String, String, String)>,
) -> Json<Value> {
    Json(serde_json::json!({
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id
    }))
}

// Integer path param
async fn get_path_int(Path(id): Path<i64>) -> Json<Value> {
    Json(serde_json::json!({ "id": id }))
}

// UUID path param
async fn get_path_uuid(Path(uuid): Path<uuid::Uuid>) -> Json<Value> {
    Json(serde_json::json!({ "uuid": uuid.to_string() }))
}

// Date path param (as string, then parse)
async fn get_path_date(Path(date): Path<String>) -> Json<Value> {
    Json(serde_json::json!({ "date": date }))
}

// ============================================================================
// Query Parameter Workloads
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct QueryFew {
    q: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
}

async fn get_query_few(Query(params): Query<QueryFew>) -> Json<QueryFew> {
    Json(params)
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryMedium {
    category: Option<String>,
    tags: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    sort: Option<String>,
    order: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
}

async fn get_query_medium(Query(params): Query<QueryMedium>) -> Json<QueryMedium> {
    Json(params)
}

// For many params, just use HashMap
async fn get_query_many(Query(params): Query<HashMap<String, String>>) -> Json<HashMap<String, String>> {
    Json(params)
}

// ============================================================================
// Health Check
// ============================================================================

async fn health() -> Json<Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

// ============================================================================
// Main Server Setup
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spikard_rust_bench=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Build router with all workload endpoints
    let app = Router::new()
        // Health check
        .route("/health", get(health))
        // JSON body workloads
        .route("/json/small", post(post_json_small))
        .route("/json/medium", post(post_json_medium))
        .route("/json/large", post(post_json_large))
        .route("/json/very-large", post(post_json_very_large))
        // Multipart workloads
        .route("/multipart/small", post(post_multipart_small))
        .route("/multipart/medium", post(post_multipart_medium))
        .route("/multipart/large", post(post_multipart_large))
        // URL encoded workloads
        .route("/urlencoded/simple", post(post_urlencoded_simple))
        .route("/urlencoded/complex", post(post_urlencoded_complex))
        // Path parameter workloads
        .route("/path/simple/{id}", get(get_path_simple))
        .route("/path/multiple/{user_id}/{post_id}", get(get_path_multiple))
        .route("/path/deep/{org}/{team}/{project}/{resource}/{id}", get(get_path_deep))
        .route("/path/int/{id}", get(get_path_int))
        .route("/path/uuid/{uuid}", get(get_path_uuid))
        .route("/path/date/{date}", get(get_path_date))
        // Query parameter workloads
        .route("/query/few", get(get_query_few))
        .route("/query/medium", get(get_query_medium))
        .route("/query/many", get(get_query_many))
        // Add tracing layer
        .layer(TraceLayer::new_for_http());

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Listening on {}", addr);
    eprintln!("Spikard Rust benchmark server listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
