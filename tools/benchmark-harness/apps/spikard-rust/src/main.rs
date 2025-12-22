//! Spikard Rust HTTP server for workload benchmarking.
//!
//! This server uses the actual Spikard Rust crate to test performance
//! with Rust handlers (no FFI overhead).

use axum::body::Body;
use axum::http::{Response, StatusCode};
use clap::Parser;
use pprof::ProfilerGuard;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spikard::{App, RequestContext, ServerConfig, get, post};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Parser, Debug)]
#[command(name = "spikard-rust-bench")]
#[command(about = "Spikard Rust HTTP server for baseline benchmarking")]
struct Args {
    /// Port to listen on
    #[arg(default_value = "8000")]
    port: u16,
}

static CPU_PROFILER: OnceLock<Mutex<Option<ProfilerGuard<'static>>>> = OnceLock::new();
static CPU_PROFILE_OUTPUT: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
struct SmallPayload {
    name: String,
    description: String,
    price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MediumPayload {
    name: String,
    price: f64,
    image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    url: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    name: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    country: Country,
}

#[derive(Debug, Serialize, Deserialize)]
struct SellerWithAddress {
    name: String,
    address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
struct LargePayload {
    name: String,
    price: f64,
    seller: SellerWithAddress,
}

#[derive(Debug, Serialize, Deserialize)]
struct VeryLargePayload {
    name: String,
    tags: Vec<String>,
    images: Vec<Image>,
}

async fn post_json_small(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: SmallPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: MediumPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: LargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

async fn post_json_very_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: VeryLargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap())
}


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


async fn health(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({ "status": "ok" });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn benchmark_profile_start(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: HashMap<String, String> = ctx.query().unwrap_or_default();
    let Some(output) = params.get("output").filter(|s| !s.is_empty()) else {
        let result = serde_json::json!({ "ok": false, "error": "missing_output" });
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .unwrap());
    };

    let output_path = PathBuf::from(output);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let guard = ProfilerGuard::new(100).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let profiler_slot = CPU_PROFILER.get_or_init(|| Mutex::new(None));
    let output_slot = CPU_PROFILE_OUTPUT.get_or_init(|| Mutex::new(None));

    *profiler_slot.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "profiler_lock".to_string()))? =
        Some(guard);
    *output_slot.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "output_lock".to_string()))? =
        Some(output_path);

    let result = serde_json::json!({ "ok": true });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

async fn benchmark_profile_stop(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let profiler_slot = CPU_PROFILER.get_or_init(|| Mutex::new(None));
    let output_slot = CPU_PROFILE_OUTPUT.get_or_init(|| Mutex::new(None));

    let guard = profiler_slot
        .lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "profiler_lock".to_string()))?
        .take();
    let output_path = output_slot
        .lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "output_lock".to_string()))?
        .take();

    let Some(guard) = guard else {
        let result = serde_json::json!({ "ok": false, "error": "not_running" });
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .unwrap());
    };
    let Some(output_path) = output_path else {
        let result = serde_json::json!({ "ok": false, "error": "missing_output" });
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .unwrap());
    };

    let report = guard
        .report()
        .build()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let file = std::fs::File::create(&output_path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    report
        .flamegraph(file)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = serde_json::json!({ "ok": true });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: args.port,
        graceful_shutdown: true,
        ..Default::default()
    };

    let mut app = App::new().config(config);

    app.route(get("/health"), health).unwrap();
    if std::env::var("SPIKARD_PROFILE_ENABLED").ok().as_deref() == Some("1") {
        app.route(get("/__benchmark__/profile/start"), benchmark_profile_start)
            .unwrap();
        app.route(get("/__benchmark__/profile/stop"), benchmark_profile_stop)
            .unwrap();
    }

    app.route(post("/json/small"), post_json_small).unwrap();
    app.route(post("/json/medium"), post_json_medium).unwrap();
    app.route(post("/json/large"), post_json_large).unwrap();
    app.route(post("/json/very-large"), post_json_very_large).unwrap();

    app.route(post("/multipart/small"), post_multipart_small).unwrap();
    app.route(post("/multipart/medium"), post_multipart_medium).unwrap();
    app.route(post("/multipart/large"), post_multipart_large).unwrap();

    app.route(post("/urlencoded/simple"), post_urlencoded_simple).unwrap();
    app.route(post("/urlencoded/complex"), post_urlencoded_complex).unwrap();

    app.route(get("/path/simple/{id}"), get_path_simple).unwrap();
    app.route(get("/path/multiple/{user_id}/{post_id}"), get_path_multiple)
        .unwrap();
    app.route(get("/path/deep/{org}/{team}/{project}/{resource}/{id}"), get_path_deep)
        .unwrap();
    app.route(get("/path/int/{id}"), get_path_int).unwrap();
    app.route(get("/path/uuid/{uuid}"), get_path_uuid).unwrap();
    app.route(get("/path/date/{date}"), get_path_date).unwrap();

    app.route(get("/query/few"), get_query_few).unwrap();
    app.route(get("/query/medium"), get_query_medium).unwrap();
    app.route(get("/query/many"), get_query_many).unwrap();

    eprintln!("Spikard Rust benchmark server listening on 0.0.0.0:{}", args.port);

    app.run().await.unwrap();
}
