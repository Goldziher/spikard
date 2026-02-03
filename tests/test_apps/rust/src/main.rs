use serde_json::json;
use spikard::{App, HandlerResult, RequestContext, ServerConfig, get, post, put, delete, patch};

/// Rust test application for Spikard
///
/// Tests core functionality:
/// - Health check endpoint
/// - Query parameter handling
/// - JSON request/response
/// - Path parameter extraction

async fn health_handler(_ctx: RequestContext) -> HandlerResult {
    let body = json!({ "status": "ok" });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(body.to_string()))
        .unwrap())
}

async fn query_handler(ctx: RequestContext) -> HandlerResult {
    // Use the parsed query_value which returns a JSON value
    let params = ctx.query_value();
    let name = params.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let age = params.get("age").and_then(|v| v.as_u64()).map(|n| n as u32);

    let response = json!({ "name": name, "age": age });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn echo_handler(ctx: RequestContext) -> HandlerResult {
    let body = ctx.body_value().clone();
    let response = json!({
        "received": body,
        "method": ctx.method()
    });

    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn user_handler(ctx: RequestContext) -> HandlerResult {
    let user_id = ctx.path_param("id").unwrap_or("").to_string();
    let response = json!({
        "userId": user_id,
        "type": "String"
    });

    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn put_item_handler(ctx: RequestContext) -> HandlerResult {
    let item_id = ctx.path_param("id").unwrap_or("").to_string();
    let body = ctx.body_value().clone();
    let response = json!({
        "itemId": item_id,
        "updated": body,
        "method": ctx.method()
    });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn delete_item_handler(ctx: RequestContext) -> HandlerResult {
    let item_id = ctx.path_param("id").unwrap_or("").to_string();
    let response = json!({
        "itemId": item_id,
        "deleted": true,
        "method": ctx.method()
    });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn patch_item_handler(ctx: RequestContext) -> HandlerResult {
    let item_id = ctx.path_param("id").unwrap_or("").to_string();
    let body = ctx.body_value().clone();
    let response = json!({
        "itemId": item_id,
        "patched": body,
        "method": ctx.method()
    });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn headers_handler(ctx: RequestContext) -> HandlerResult {
    let custom_header = ctx.header("x-custom-header").unwrap_or("").to_string();
    let response = json!({
        "x-custom-header": custom_header
    });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn cookies_handler(ctx: RequestContext) -> HandlerResult {
    let session = ctx.cookie("session").unwrap_or("").to_string();
    let response = json!({
        "session": session
    });
    Ok(axum::http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(response.to_string()))
        .unwrap())
}

async fn error_handler(_ctx: RequestContext) -> HandlerResult {
    Ok(axum::http::Response::builder()
        .status(500)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(r#"{"error":"Intentional error"}"#))
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(0)
        .build();

    let mut app = App::new().config(config);

    app.route(get("/health"), health_handler)?;
    app.route(get("/query"), query_handler)?;
    app.route(post("/echo"), echo_handler)?;
    app.route(get("/users/{id}"), user_handler)?;
    app.route(put("/items/{id}"), put_item_handler)?;
    app.route(delete("/items/{id}"), delete_item_handler)?;
    app.route(patch("/items/{id}"), patch_item_handler)?;
    app.route(get("/headers"), headers_handler)?;
    app.route(get("/cookies"), cookies_handler)?;
    app.route(get("/error"), error_handler)?;

    // Print server address for test harness to capture
    let router = app.into_router()?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    println!("Server starting on {}", local_addr);

    axum::serve(listener, router).await?;

    Ok(())
}
