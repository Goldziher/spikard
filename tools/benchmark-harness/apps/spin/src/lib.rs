//! Spin SDK benchmark app.

use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/json/small", json_small);
    router.get("/json/medium", json_medium);
    router.get("/json/large", json_large);
    router.post("/json/echo", json_echo);
    router.get("/path-params/*", path_params);
    router.get("/query-params", query_params);
    router.get("/health", health);
    Ok(router.handle(req))
}

fn json_small(_req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(r#"{"message":"hello"}"#)
        .build())
}

fn json_medium(_req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    let items: Vec<_> = (0..100)
        .map(|i| serde_json::json!({"id": i, "name": format!("item_{i}"), "active": i % 2 == 0}))
        .collect();
    let body = serde_json::to_string(&serde_json::json!({"items": items, "total": 100}))?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn json_large(_req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    let items: Vec<_> = (0..1000)
        .map(|i| serde_json::json!({"id": i, "name": format!("item_{i}"), "value": i * 42}))
        .collect();
    let body = serde_json::to_string(&serde_json::json!({"items": items, "total": 1000}))?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn json_echo(req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    let body = req.body();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body.clone())
        .build())
}

fn path_params(req: Request, params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    let wildcard = params.wildcard().unwrap_or("");
    let segments: Vec<&str> = wildcard.split('/').filter(|s| !s.is_empty()).collect();
    let body = serde_json::to_string(&serde_json::json!({"segments": segments}))?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn query_params(req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    let query = req.query();
    let body = serde_json::to_string(&serde_json::json!({"query": query}))?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

fn health(_req: Request, _params: spin_sdk::http::Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(r#"{"status":"ok"}"#)
        .build())
}
