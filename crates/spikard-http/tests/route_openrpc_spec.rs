//! A route may supply a literal `OpenRPC` document via `Route.openrpc_spec`; when it does, that
//! document is what `/openrpc.json` serves, instead of the one derived from the registered
//! JSON-RPC methods. The corpus supplies such documents at
//! `fixtures/openrpc.json` -> `http.handler.middleware.openrpc.spec`, and the spec below is the
//! `openrpc_method_handler_basic` payload verbatim.

use axum::http::StatusCode;
use serde_json::{Value, json};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    Handler, HandlerResult, JsonRpcConfig, JsonRpcMethodInfo, Method, RequestData, Route, ServerConfig,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from("ok"))
                .unwrap())
        })
    }
}

/// `fixtures/openrpc.json`, id `openrpc_method_handler_basic`, `middleware.openrpc.spec`.
fn supplied_spec() -> Value {
    json!({
        "openrpc": "1.3.2",
        "info": {"title": "Math API", "version": "1.0.0"},
        "methods": [
            {
                "name": "add",
                "params": [
                    {"name": "a", "required": true, "schema": {"type": "integer"}},
                    {"name": "b", "required": true, "schema": {"type": "integer"}}
                ],
                "result": {"name": "sum", "schema": {"type": "integer"}}
            }
        ]
    })
}

fn rpc_route(path: &str, openrpc_spec: Option<Value>) -> Route {
    Route {
        method: Method::Post,
        path: path.to_string(),
        handler_name: "add".to_string(),
        expects_json_body: true,
        jsonrpc_method: Some(JsonRpcMethodInfo {
            method_name: "add".to_string(),
            description: Some("Adds two integers".to_string()),
            params_schema: None,
            result_schema: None,
            deprecated: false,
            tags: vec![],
        }),
        openrpc_spec,
        ..Default::default()
    }
}

fn build_router(routes: Vec<Route>) -> Result<axum::Router, String> {
    let config = ServerConfig {
        jsonrpc: Some(JsonRpcConfig::default()),
        ..ServerConfig::default()
    };
    let routes = routes
        .into_iter()
        .map(|route| (route, Arc::new(OkHandler) as Arc<dyn Handler>))
        .collect();
    build_router_with_handlers_and_config(routes, config, Vec::new())
}

async fn fetch_openrpc_document(routes: Vec<Route>) -> Value {
    let server = axum_test::TestServer::new(build_router(routes).expect("router"));

    let response = server.get("/openrpc.json").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    serde_json::from_str(&response.text()).expect("openrpc.json body is JSON")
}

#[tokio::test]
async fn route_supplied_openrpc_document_is_served_verbatim() {
    let document = fetch_openrpc_document(vec![rpc_route("/add", Some(supplied_spec()))]).await;

    assert_eq!(
        document,
        supplied_spec(),
        "a route that supplies an OpenRPC document must have that exact document published at \
         /openrpc.json, not a spec re-derived from the registered methods"
    );
}

#[tokio::test]
async fn openrpc_document_is_derived_when_no_route_supplies_one() {
    let document = fetch_openrpc_document(vec![rpc_route("/add", None)]).await;

    assert_eq!(
        document.pointer("/info/title").and_then(Value::as_str),
        Some("Spikard JSON-RPC API"),
        "with no supplied document the derived spec must still be published"
    );
    assert_eq!(
        document.pointer("/methods/0/name").and_then(Value::as_str),
        Some("add"),
        "the derived spec describes the methods registered from route metadata"
    );
    assert_ne!(
        document,
        supplied_spec(),
        "deriving must not accidentally reproduce the supplied document, or the test above proves \
         nothing"
    );
}

/// The supplied document replaces the derived one wholesale — including its `servers` block, which
/// derivation always emits and this document does not carry at all. Asserting the absence pins
/// that the supplied document is served as-is rather than merged into a derived skeleton.
#[tokio::test]
async fn route_supplied_document_is_not_merged_with_derived_fields() {
    let supplied = fetch_openrpc_document(vec![rpc_route("/add", Some(supplied_spec()))]).await;
    let derived = fetch_openrpc_document(vec![rpc_route("/add", None)]).await;

    assert!(
        derived.get("servers").is_some(),
        "derivation is expected to emit a servers block"
    );
    assert!(
        supplied.get("servers").is_none(),
        "the supplied document carries no servers block, so the served document must not have one"
    );
}

#[tokio::test]
async fn two_routes_supplying_the_same_document_agree() {
    let document = fetch_openrpc_document(vec![
        rpc_route("/add", Some(supplied_spec())),
        rpc_route("/add-alias", Some(supplied_spec())),
    ])
    .await;

    assert_eq!(document, supplied_spec());
}

#[test]
fn conflicting_route_supplied_documents_fail_the_router_build() {
    let mut other = supplied_spec();
    other["info"]["title"] = json!("Other API");

    let error = build_router(vec![
        rpc_route("/add", Some(supplied_spec())),
        rpc_route("/add-alias", Some(other)),
    ])
    .expect_err("two routes supplying different OpenRPC documents must not build");

    assert!(
        error.contains("different OpenRPC spec documents"),
        "the build error must name the conflict; got: {error}"
    );
}
