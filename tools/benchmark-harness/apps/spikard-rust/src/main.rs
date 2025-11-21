//! Spikard Rust HTTP server for workload benchmarking.
//!
//! This server uses the actual Spikard Rust crate to test performance
//! with Rust handlers (no FFI overhead).

use axum::http::{Response, StatusCode};
use axum::body::Body;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spikard::{post, get, App, RequestContext, ServerConfig};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "spikard-rust-bench")]
#[command(about = "Spikard Rust HTTP server for baseline benchmarking")]
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

// JSON handlers
async fn post_json_small(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: SmallJson = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: Value = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: Value = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_very_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: Value = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

// ============================================================================
// Multipart Form Workloads
// ============================================================================

async fn post_multipart_small(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({
        "files_received": 1,
        "total_bytes": 1024
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn post_multipart_medium(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({
        "files_received": 2,
        "total_bytes": 10240
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn post_multipart_large(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({
        "files_received": 5,
        "total_bytes": 102400
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

// ============================================================================
// URL Encoded Form Workloads
// ============================================================================

async fn post_urlencoded_simple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: Value = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_urlencoded_complex(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: Value = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

// ============================================================================
// Path Parameter Workloads
// ============================================================================

async fn get_path_simple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx.path_param("id").unwrap_or("unknown");
    let result = serde_json::json!({ "id": id });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn get_path_multiple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let user_id = ctx.path_param("user_id").unwrap_or("unknown");
    let post_id = ctx.path_param("post_id").unwrap_or("unknown");
    let result = serde_json::json!({
        "user_id": user_id,
        "post_id": post_id
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn get_path_deep(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let org = ctx.path_param("org").unwrap_or("unknown");
    let team = ctx.path_param("team").unwrap_or("unknown");
    let project = ctx.path_param("project").unwrap_or("unknown");
    let resource = ctx.path_param("resource").unwrap_or("unknown");
    let id = ctx.path_param("id").unwrap_or("unknown");
    let result = serde_json::json!({
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn get_path_int(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx.path_param("id").unwrap_or("0");
    let result = serde_json::json!({ "id": id.parse::<i64>().unwrap_or(0) });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn get_path_uuid(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let uuid = ctx.path_param("uuid").unwrap_or("unknown");
    let result = serde_json::json!({ "uuid": uuid });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn get_path_date(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let date = ctx.path_param("date").unwrap_or("unknown");
    let result = serde_json::json!({ "date": date });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
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

async fn get_query_few(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryFew = ctx.query().unwrap_or(QueryFew {
        q: None,
        page: None,
        limit: None,
    });
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn get_query_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryMedium = ctx.query().unwrap_or(QueryMedium {
        category: None,
        tags: None,
        min_price: None,
        max_price: None,
        sort: None,
        order: None,
        page: None,
        limit: None,
    });
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn get_query_many(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: HashMap<String, String> = ctx.query().unwrap_or_default();
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

// ============================================================================
// Health Check
// ============================================================================

async fn health(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({ "status": "ok" });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

// ============================================================================
// Main Server Setup
// ============================================================================

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: args.port,
        ..Default::default()
    };

    let mut app = App::new().config(config);

    // Health check
    app.route(get("/health"), health).unwrap();

    // JSON body workloads
    app.route(post("/json/small"), post_json_small).unwrap();
    app.route(post("/json/medium"), post_json_medium).unwrap();
    app.route(post("/json/large"), post_json_large).unwrap();
    app.route(post("/json/very-large"), post_json_very_large).unwrap();

    // Multipart workloads
    app.route(post("/multipart/small"), post_multipart_small).unwrap();
    app.route(post("/multipart/medium"), post_multipart_medium).unwrap();
    app.route(post("/multipart/large"), post_multipart_large).unwrap();

    // URL encoded workloads
    app.route(post("/urlencoded/simple"), post_urlencoded_simple).unwrap();
    app.route(post("/urlencoded/complex"), post_urlencoded_complex).unwrap();

    // Path parameter workloads
    app.route(get("/path/simple/{id}"), get_path_simple).unwrap();
    app.route(get("/path/multiple/{user_id}/{post_id}"), get_path_multiple).unwrap();
    app.route(get("/path/deep/{org}/{team}/{project}/{resource}/{id}"), get_path_deep).unwrap();
    app.route(get("/path/int/{id}"), get_path_int).unwrap();
    app.route(get("/path/uuid/{uuid}"), get_path_uuid).unwrap();
    app.route(get("/path/date/{date}"), get_path_date).unwrap();

    // Query parameter workloads
    app.route(get("/query/few"), get_query_few).unwrap();
    app.route(get("/query/medium"), get_query_medium).unwrap();
    app.route(get("/query/many"), get_query_many).unwrap();

    eprintln!("Spikard Rust benchmark server listening on 0.0.0.0:{}", args.port);

    app.run().await.unwrap();
}
