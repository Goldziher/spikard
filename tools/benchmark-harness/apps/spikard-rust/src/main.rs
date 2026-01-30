//! Spikard Rust HTTP server for workload benchmarking.
//!
//! This server uses the actual Spikard Rust crate to test performance
//! with Rust handlers (no FFI overhead).
//!
//! Provides both raw endpoints (no validation) and validated endpoints (with schema validation)
//! at the /validated/ prefix.

use axum::body::Body;
use axum::http::{Response, StatusCode};
use clap::Parser;
use pprof::ProfilerGuard;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use spikard::{App, RequestContext, ServerConfig, get, post};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;
use chrono::NaiveDate;

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

#[derive(Debug, Serialize, Deserialize)]
struct PathUuid {
    uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct PathDate {
    date: NaiveDate,
}

const REQUEST_SCHEMAS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../schemas/request_schemas.json"));
const RESPONSE_SCHEMAS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../schemas/response_schemas.json"));
const PARAMETER_SCHEMAS: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../schemas/parameter_schemas.json"));

fn load_schema_map(raw: &str) -> HashMap<String, Value> {
    serde_json::from_str(raw).expect("schema json should parse")
}

fn schema_value(map: &HashMap<String, Value>, key: &str) -> Value {
    map.get(key).cloned().unwrap_or(Value::Null)
}

// ============================================================================
// RAW HANDLERS (no validation)
// ============================================================================

async fn root(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({ "status": "ok" });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_small(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: SmallPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: MediumPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: LargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_very_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: VeryLargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_small(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_large(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_urlencoded_simple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses urlencoded to JSON
    let body = ctx.body_value();
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_urlencoded_complex(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses urlencoded to JSON
    let body = ctx.body_value();
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_simple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx.path_param("id").unwrap_or("unknown");
    let result = serde_json::json!({ "id": id });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_multiple(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let user_id = ctx.path_param("user_id").unwrap_or("unknown");
    let post_id = ctx.path_param("post_id").unwrap_or("unknown");
    let result = serde_json::json!({
        "user_id": user_id,
        "post_id": post_id
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_int(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx
        .path_param("id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: id".to_string()))?
        .parse::<i64>()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Invalid path param id: {err}")))?;
    let result = serde_json::json!({ "id": id });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_uuid(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let PathUuid { uuid } = ctx.path().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let result = serde_json::json!({ "uuid": uuid });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_date(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let PathDate { date } = ctx.path().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let result = serde_json::json!({ "date": date.to_string() });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryFew {
    q: String,
    page: Option<i32>,
    limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryMedium {
    search: String,
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
    filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryMany {
    q: String,
    category: Option<String>,
    subcategory: Option<String>,
    brand: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    color: Option<String>,
    size: Option<String>,
    material: Option<String>,
    rating: Option<i32>,
    sort: Option<String>,
    order: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
    in_stock: Option<bool>,
    on_sale: Option<bool>,
}

async fn get_query_few(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryFew = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_query_medium(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryMedium = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_query_many(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryMany = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn health(_ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let result = serde_json::json!({ "status": "ok" });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn benchmark_profile_start(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: HashMap<String, String> = ctx.query().unwrap_or_default();
    let Some(output) = params.get("output").filter(|s| !s.is_empty()) else {
        let result = serde_json::json!({ "ok": false, "error": "missing_output" });
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
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
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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
        return Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    };
    let Some(output_path) = output_path else {
        let result = serde_json::json!({ "ok": false, "error": "missing_output" });
        return Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(result.to_string()))
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
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
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// ============================================================================
// VALIDATED HANDLERS (with schema validation)
// ============================================================================

fn validate_string_param(value: &str, param_name: &str) -> Result<(), (StatusCode, String)> {
    if value.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Parameter '{}' cannot be empty", param_name),
        ));
    }
    if value.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Parameter '{}' exceeds maximum length of 255 characters", param_name),
        ));
    }
    // Check alphanumeric with - and _
    if !value.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Parameter '{}' contains invalid characters (only alphanumeric, '-', and '_' allowed)", param_name),
        ));
    }
    Ok(())
}

fn coerce_bool(value: Value) -> Value {
    match value {
        Value::Bool(_) => value,
        Value::String(ref s) => {
            if s.eq_ignore_ascii_case("true") {
                Value::Bool(true)
            } else if s.eq_ignore_ascii_case("false") {
                Value::Bool(false)
            } else {
                value
            }
        },
        _ => value,
    }
}

fn coerce_i64(value: Value) -> Value {
    match value {
        Value::Number(_) => value,
        Value::String(value) => value
            .parse::<i64>()
            .map_or(Value::String(value), |parsed| Value::Number(parsed.into())),
        _ => value,
    }
}

fn coerce_field(map: &mut Map<String, Value>, key: &str, coerce: fn(Value) -> Value) {
    if let Some(value) = map.remove(key) {
        map.insert(key.to_string(), coerce(value));
    }
}

fn coerce_urlencoded_simple(body: Value) -> Value {
    let mut map = match body {
        Value::Object(map) => map,
        _ => return body,
    };
    coerce_field(&mut map, "age", coerce_i64);
    coerce_field(&mut map, "subscribe", coerce_bool);
    Value::Object(map)
}

fn coerce_urlencoded_complex(body: Value) -> Value {
    let mut map = match body {
        Value::Object(map) => map,
        _ => return body,
    };
    coerce_field(&mut map, "age", coerce_i64);
    for key in [
        "subscribe",
        "newsletter",
        "terms_accepted",
        "privacy_accepted",
        "marketing_consent",
        "two_factor_enabled",
    ] {
        coerce_field(&mut map, key, coerce_bool);
    }
    Value::Object(map)
}

async fn post_json_small_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: SmallPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_medium_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: MediumPayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_large_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: LargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_json_very_large_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let body: VeryLargePayload = ctx.json().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_small_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    if files_received == 0 {
        return Err((StatusCode::BAD_REQUEST, "No files received".to_string()));
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_medium_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    if files_received == 0 {
        return Err((StatusCode::BAD_REQUEST, "No files received".to_string()));
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_multipart_large_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses multipart to JSON
    let body = ctx.body_value();

    let mut files_received = 0;
    let mut total_bytes = 0;

    if let Some(file_obj) = body.get("file") {
        if let Some(size) = file_obj.get("size").and_then(|s| s.as_u64()) {
            files_received = 1;
            total_bytes = size;
        }
    }

    if files_received == 0 {
        return Err((StatusCode::BAD_REQUEST, "No files received".to_string()));
    }

    let result = serde_json::json!({
        "files_received": files_received,
        "total_bytes": total_bytes
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_urlencoded_simple_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses urlencoded to JSON
    let body = coerce_urlencoded_simple(ctx.body_value().clone());
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn post_urlencoded_complex_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    // Spikard middleware already parses urlencoded to JSON
    let body = coerce_urlencoded_complex(ctx.body_value().clone());
    let json = serde_json::to_string(&body).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_simple_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx
        .path_param("id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: id".to_string()))?;
    validate_string_param(id, "id")?;

    let result = serde_json::json!({ "id": id });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_multiple_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let user_id = ctx
        .path_param("user_id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: user_id".to_string()))?;
    let post_id = ctx
        .path_param("post_id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: post_id".to_string()))?;

    validate_string_param(user_id, "user_id")?;
    validate_string_param(post_id, "post_id")?;

    let result = serde_json::json!({
        "user_id": user_id,
        "post_id": post_id
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_deep_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let org = ctx
        .path_param("org")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: org".to_string()))?;
    let team = ctx
        .path_param("team")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: team".to_string()))?;
    let project = ctx
        .path_param("project")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: project".to_string()))?;
    let resource = ctx
        .path_param("resource")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: resource".to_string()))?;
    let id = ctx
        .path_param("id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: id".to_string()))?;

    validate_string_param(org, "org")?;
    validate_string_param(team, "team")?;
    validate_string_param(project, "project")?;
    validate_string_param(resource, "resource")?;
    validate_string_param(id, "id")?;

    let result = serde_json::json!({
        "org": org,
        "team": team,
        "project": project,
        "resource": resource,
        "id": id
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_int_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let id = ctx
        .path_param("id")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing path param: id".to_string()))?
        .parse::<i64>()
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("Invalid path param id: {err}")))?;
    let result = serde_json::json!({ "id": id });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_uuid_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let PathUuid { uuid } = ctx.path().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let result = serde_json::json!({ "uuid": uuid });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_path_date_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let PathDate { date } = ctx.path().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let result = serde_json::json!({ "date": date.to_string() });
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(result.to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_query_few_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryFew = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_query_medium_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryMedium = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn get_query_many_validated(ctx: RequestContext) -> Result<Response<Body>, (StatusCode, String)> {
    let params: QueryMany = ctx.query().map_err(|e| (StatusCode::BAD_REQUEST, format!("Missing required parameter: {e}")))?;
    let json = serde_json::to_string(&params).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
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

    let request_schemas = load_schema_map(REQUEST_SCHEMAS);
    let response_schemas = load_schema_map(RESPONSE_SCHEMAS);
    let parameter_schemas = load_schema_map(PARAMETER_SCHEMAS);

    // ============================================================================
    // RAW ENDPOINTS (no validation)
    // ============================================================================

    app.route(get("/"), root).unwrap();
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
    app.route(
        get("/path/deep/{org}/{team}/{project}/{resource}/{id}"),
        get_path_deep,
    )
    .unwrap();
    app.route(get("/path/int/{id}"), get_path_int).unwrap();
    app.route(get("/path/uuid/{uuid}"), get_path_uuid).unwrap();
    app.route(get("/path/date/{date}"), get_path_date).unwrap();

    app.route(get("/query/few"), get_query_few).unwrap();
    app.route(get("/query/medium"), get_query_medium).unwrap();
    app.route(get("/query/many"), get_query_many).unwrap();

    // ============================================================================
    // VALIDATED ENDPOINTS (with schema validation at /validated/ prefix)
    // ============================================================================

    app.route(
        get("/validated/health").response_schema_json(schema_value(&response_schemas, "health")),
        health,
    )
    .unwrap();

    app.route(
        post("/validated/json/small")
            .request_schema_json(schema_value(&request_schemas, "json/small"))
            .response_schema_json(schema_value(&response_schemas, "json/small")),
        post_json_small_validated,
    )
    .unwrap();
    app.route(
        post("/validated/json/medium")
            .request_schema_json(schema_value(&request_schemas, "json/medium"))
            .response_schema_json(schema_value(&response_schemas, "json/medium")),
        post_json_medium_validated,
    )
    .unwrap();
    app.route(
        post("/validated/json/large")
            .request_schema_json(schema_value(&request_schemas, "json/large"))
            .response_schema_json(schema_value(&response_schemas, "json/large")),
        post_json_large_validated,
    )
    .unwrap();
    app.route(
        post("/validated/json/very-large")
            .request_schema_json(schema_value(&request_schemas, "json/very-large"))
            .response_schema_json(schema_value(&response_schemas, "json/very-large")),
        post_json_very_large_validated,
    )
    .unwrap();

    app.route(
        post("/validated/multipart/small")
            .request_schema_json(schema_value(&request_schemas, "multipart/small"))
            .response_schema_json(schema_value(&response_schemas, "multipart/small")),
        post_multipart_small_validated,
    )
    .unwrap();
    app.route(
        post("/validated/multipart/medium")
            .request_schema_json(schema_value(&request_schemas, "multipart/medium"))
            .response_schema_json(schema_value(&response_schemas, "multipart/medium")),
        post_multipart_medium_validated,
    )
    .unwrap();
    app.route(
        post("/validated/multipart/large")
            .request_schema_json(schema_value(&request_schemas, "multipart/large"))
            .response_schema_json(schema_value(&response_schemas, "multipart/large")),
        post_multipart_large_validated,
    )
    .unwrap();

    app.route(
        post("/validated/urlencoded/simple")
            .request_schema_json(schema_value(&request_schemas, "urlencoded/simple"))
            .response_schema_json(schema_value(&response_schemas, "urlencoded/simple")),
        post_urlencoded_simple_validated,
    )
    .unwrap();
    app.route(
        post("/validated/urlencoded/complex")
            .request_schema_json(schema_value(&request_schemas, "urlencoded/complex"))
            .response_schema_json(schema_value(&response_schemas, "urlencoded/complex")),
        post_urlencoded_complex_validated,
    )
    .unwrap();

    app.route(
        get("/validated/path/simple/{id}")
            .params_schema_json(schema_value(&parameter_schemas, "path/simple"))
            .response_schema_json(schema_value(&response_schemas, "path/simple")),
        get_path_simple_validated,
    )
    .unwrap();
    app.route(
        get("/validated/path/multiple/{user_id}/{post_id}")
            .params_schema_json(schema_value(&parameter_schemas, "path/multiple"))
            .response_schema_json(schema_value(&response_schemas, "path/multiple")),
        get_path_multiple_validated,
    )
    .unwrap();
    app.route(
        get("/validated/path/deep/{org}/{team}/{project}/{resource}/{id}")
            .params_schema_json(schema_value(&parameter_schemas, "path/deep"))
            .response_schema_json(schema_value(&response_schemas, "path/deep")),
        get_path_deep_validated,
    )
    .unwrap();
    app.route(
        get("/validated/path/int/{id}")
            .params_schema_json(schema_value(&parameter_schemas, "path/int"))
            .response_schema_json(schema_value(&response_schemas, "path/int")),
        get_path_int_validated,
    )
    .unwrap();
    app.route(
        get("/validated/path/uuid/{uuid}")
            .params_schema_json(schema_value(&parameter_schemas, "path/uuid"))
            .response_schema_json(schema_value(&response_schemas, "path/uuid")),
        get_path_uuid_validated,
    )
    .unwrap();
    app.route(
        get("/validated/path/date/{date}")
            .params_schema_json(schema_value(&parameter_schemas, "path/date"))
            .response_schema_json(schema_value(&response_schemas, "path/date")),
        get_path_date_validated,
    )
    .unwrap();

    app.route(
        get("/validated/query/few")
            .params_schema_json(schema_value(&parameter_schemas, "query/few"))
            .response_schema_json(schema_value(&response_schemas, "query/few")),
        get_query_few_validated,
    )
    .unwrap();
    app.route(
        get("/validated/query/medium")
            .params_schema_json(schema_value(&parameter_schemas, "query/medium"))
            .response_schema_json(schema_value(&response_schemas, "query/medium")),
        get_query_medium_validated,
    )
    .unwrap();
    app.route(
        get("/validated/query/many")
            .params_schema_json(schema_value(&parameter_schemas, "query/many"))
            .response_schema_json(schema_value(&response_schemas, "query/many")),
        get_query_many_validated,
    )
    .unwrap();

    eprintln!("Spikard Rust benchmark server listening on 0.0.0.0:{}", args.port);

    app.run().await.unwrap();
}
