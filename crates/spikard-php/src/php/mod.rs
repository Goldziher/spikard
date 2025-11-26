//! ext-php-rs implementation bridging PHP handlers to the Rust core.
//!
//! This module exposes a native TestClient backed by spikard_http's router and
//! axum-test so PHPUnit exercises the real middleware/validation stack.

use axum::body::Body;
use axum::http::{Request, Response as AxumResponse};
use axum_test::TestServer;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use serde_json::Value;
use spikard_http::testing::{call_test_server, snapshot_response};
use spikard_http::{Handler, HandlerResult, RequestData, Route, Router, Server, ServerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use urlencoding::decode;

mod request;
mod response;

pub use request::PhpRequest;
pub use response::PhpResponse;

/// Simple registry to keep runtimes and servers alive and allow shutdown.
struct HandleRegistry {
    next_id: std::sync::atomic::AtomicI64,
    handles: parking_lot::RwLock<HashMap<i64, RuntimeHandle>>,
}

struct RuntimeHandle {
    runtime: Runtime,
    shutdown_tx: tokio::sync::oneshot::Sender<()>,
}

impl HandleRegistry {
    const fn new() -> Self {
        Self {
            next_id: std::sync::atomic::AtomicI64::new(1),
            handles: parking_lot::RwLock::new(HashMap::new()),
        }
    }

    fn register(&self, runtime: Runtime, router: axum::Router, config: ServerConfig) -> Result<i64, PhpException> {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        runtime.spawn(async move {
            let addr = format!("{}:{}", config.host, config.port);
            let listener = match tokio::net::TcpListener::bind(&addr).await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("spikard-php: failed to bind {}: {}", addr, e);
                    return;
                }
            };

            let server_fut = axum::serve(listener, router);
            tokio::select! {
                _ = server_fut => {},
                _ = rx => {},
            }
        });

        self.handles.write().insert(
            id,
            RuntimeHandle {
                runtime,
                shutdown_tx: tx,
            },
        );
        Ok(id)
    }

    fn stop(&self, id: i64) -> bool {
        if let Some(handle) = self.handles.write().remove(&id) {
            let _ = handle.shutdown_tx.send(());
            true
        } else {
            false
        }
    }
}

static HANDLE_REGISTRY: HandleRegistry = HandleRegistry::new();

#[derive(Clone)]
struct PhpHandlerWrapper {
    handler: Zval,
}

impl Handler for PhpHandlerWrapper {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        let handler = self.handler.clone();
        Box::pin(async move {
            let php_req = php_request_from_request_data(&request_data);
            let matches = handler
                .try_call_method("matches", vec![&php_req])
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            if !matches.bool().unwrap_or(false) {
                return Err((axum::http::StatusCode::NOT_FOUND, "No handler matched".to_string()));
            }

            let resp_zval = handler
                .try_call_method("handle", vec![&php_req])
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let php_resp = response_from_zval(&resp_zval)
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let axum_resp = axum_response_from_php(&php_resp);
            Ok(axum_resp)
        })
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function("spikard_version", spikard_version)
        .function("spikard_echo_response", spikard_echo_response)
        .function("spikard_start_server", spikard_start_server)
        .function("spikard_stop_server", spikard_stop_server)
        .class::<NativeTestClient>()
        .class::<PhpRequest>()
        .class::<PhpResponse>()
}

/// Return the crate version.
#[php_function]
pub fn spikard_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Placeholder entrypoint: echo a response for sanity checks.
#[php_function]
pub fn spikard_echo_response(body: &str) -> PhpResponse {
    PhpResponse::json(body.into(), Some(200))
}

/// Start a background server; returns a handle id.
#[php_function]
pub fn spikard_start_server(routes: Vec<Zval>, config: HashMap<String, Zval>) -> Result<i64, PhpException> {
    let runtime = Runtime::new().map_err(|e| PhpException::default(e.to_string()))?;

    // Build router and handlers
    let (router, metadata) = build_router_from_php(routes)?;

    // TODO: parse lifecycle hooks, DI, SSE/WS once exposed on PHP side.
    let server_config = ServerConfig::default();
    let axum_router = spikard_http::router::build_router_with_config(router, server_config.clone(), metadata)
        .map_err(|e| PhpException::default(e.to_string()))?;

    // Bind and serve in background; capture handle
    let handle_id = HANDLE_REGISTRY.register(runtime, axum_router, server_config)?;
    Ok(handle_id)
}

/// Stop server by handle (no-op placeholder until a real handle is wired).
#[php_function]
pub fn spikard_stop_server(_handle: i64) -> bool {
    HANDLE_REGISTRY.stop(_handle)
}

/// Native TestClient backed by spikard_http router and axum-test.
#[php_class(name = "Spikard\\Native\\TestClient")]
pub struct NativeTestClient {
    server: Arc<TestServer>,
}

#[php_impl]
impl NativeTestClient {
    #[constructor]
    pub fn __construct(routes: Vec<Zval>) -> Result<Self, PhpException> {
        let (router, route_metadata) = build_router_from_php(routes)?;
        let axum_router = spikard_http::router::build_router_for_testing(router, route_metadata)
            .map_err(|e| PhpException::default(e))?;
        let server = TestServer::new(axum_router)
            .map_err(|e| PhpException::default(format!("Failed to create test server: {e}")))?;

        Ok(Self {
            server: Arc::new(server),
        })
    }

    /// Dispatch an HTTP request through the Rust router and return a PHP response.
    pub fn request(
        &self,
        method: String,
        path: String,
        options: Option<HashMap<String, Zval>>,
    ) -> Result<PhpResponse, PhpException> {
        let uppercase_method = method.to_ascii_uppercase();
        let (path_only, query) = split_path_and_query(&path);

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut cookies: HashMap<String, String> = HashMap::new();
        let mut body: Value = Value::Null;

        if let Some(opts) = options {
            if let Some(h) = opts.get("headers") {
                if let Ok(arr) = ext_php_rs::types::array::Array::try_from(h) {
                    for (k, v) in arr.iter() {
                        if let (Some(key), Some(val)) = (k.string(), v.string()) {
                            headers.insert(key.to_ascii_lowercase().to_string(), val.to_string());
                        }
                    }
                }
            }
            if let Some(c) = opts.get("cookies") {
                if let Ok(arr) = ext_php_rs::types::array::Array::try_from(c) {
                    for (k, v) in arr.iter() {
                        if let (Some(key), Some(val)) = (k.string(), v.string()) {
                            cookies.insert(key.to_string(), val.to_string());
                        }
                    }
                }
            }
            if let Some(b) = opts.get("body") {
                body = zval_to_json(b).unwrap_or(Value::Null);
            }
        }

        let request = Request::builder()
            .method(&uppercase_method)
            .uri(path)
            .body(Body::empty())
            .map_err(|e| PhpException::default(e.to_string()))?;

        let snapshot = async_std::task::block_on(async {
            let resp: AxumResponse<Body> = call_test_server(&self.server, request).await.into();
            snapshot_response(resp).await
        })
        .map_err(|e| PhpException::default(e.to_string()))?;

        Ok(PhpResponse::json(
            serde_json::from_slice(&snapshot.body).unwrap_or(Value::Null),
            Some(snapshot.status as i64),
        )
        .with_headers(snapshot.headers))
    }
}

fn php_request_from_request_data(data: &RequestData) -> PhpRequest {
    PhpRequest::new(
        data.method.clone(),
        data.path.clone(),
        Some(data.body.clone()),
        Some((*data.headers).clone()),
        Some((*data.cookies).clone()),
        Some((*data.raw_query_params).clone()),
    )
}

fn split_path_and_query(path: &str) -> (String, HashMap<String, Vec<String>>) {
    if let Some((p, q)) = path.split_once('?') {
        let mut query_map = HashMap::new();
        for pair in q.split('&') {
            if pair.is_empty() {
                continue;
            }
            let mut iter = pair.splitn(2, '=');
            let key = iter.next().unwrap_or("");
            let val = iter.next().unwrap_or("");
            let decoded_key = decode(key).unwrap_or_else(|_| key.into()).to_string();
            let decoded_val = decode(val).unwrap_or_else(|_| val.into()).to_string();
            query_map.entry(decoded_key).or_insert_with(Vec::new).push(decoded_val);
        }
        (p.to_string(), query_map)
    } else {
        (path.to_string(), HashMap::new())
    }
}

fn zval_to_json(value: &Zval) -> Result<Value, serde_json::Error> {
    if let Some(arr) = ext_php_rs::types::array::Array::try_from(value).ok() {
        let mut map = serde_json::Map::new();
        for (k, v) in arr.iter() {
            let key = k.string().unwrap_or_else(|| "".into()).to_string();
            map.insert(key, Value::String(v.to_string()));
        }
        Ok(Value::Object(map))
    } else if let Some(str_val) = value.string() {
        Ok(Value::String(str_val.to_string()))
    } else if let Some(long) = value.long() {
        Ok(Value::from(long))
    } else if let Some(b) = value.bool() {
        Ok(Value::Bool(b))
    } else {
        Ok(Value::Null)
    }
}

fn response_from_zval(z: &Zval) -> Result<PhpResponse, PhpException> {
    let status = z.get_property("statusCode").ok().and_then(|v| v.long()).unwrap_or(200) as i64;

    let mut headers = HashMap::new();
    if let Ok(h) = z.get_property("headers") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(&h) {
            for (k, v) in arr.iter() {
                if let (Some(key), Some(val)) = (k.string(), v.string()) {
                    headers.insert(key.to_string().to_ascii_lowercase(), val.to_string());
                }
            }
        }
    }

    let body_zval = z.get_property("body").unwrap_or_else(|_| Zval::new());
    let body_val = zval_to_json(&body_zval).unwrap_or(Value::Null);

    Ok(PhpResponse::json(body_val, Some(status)).with_headers(headers))
}

fn axum_response_from_php(resp: &PhpResponse) -> AxumResponse<Body> {
    let mut builder = axum::http::Response::builder().status(resp.status as u16);
    for (k, v) in &resp.headers {
        builder = builder.header(k, v);
    }
    let body_bytes = serde_json::to_vec(&resp.body).unwrap_or_default();
    builder
        .body(Body::from(body_bytes))
        .unwrap_or_else(|_| AxumResponse::new(Body::empty()))
}

fn build_router_from_php(routes: Vec<Zval>) -> Result<(Router, Vec<spikard_core::RouteMetadata>), PhpException> {
    let mut router = Router::new();
    let mut route_metadata = Vec::new();

    for route_val in routes {
        let array =
            ext_php_rs::types::array::Array::try_from(&route_val).map_err(|e| PhpException::default(e.to_string()))?;
        let method: String = array
            .get("method")
            .map_err(|e| PhpException::default(e.to_string()))?
            .and_then(|z| z.string())
            .ok_or_else(|| PhpException::default("missing method".into()))?
            .to_string();
        let path: String = array
            .get("path")
            .map_err(|e| PhpException::default(e.to_string()))?
            .and_then(|z| z.string())
            .ok_or_else(|| PhpException::default("missing path".into()))?
            .to_string();
        let handler_zval = array
            .get("handler")
            .map_err(|e| PhpException::default(e.to_string()))?
            .ok_or_else(|| PhpException::default("missing handler".into()))?
            .clone();

        let route = Route {
            method: method.parse().map_err(|e: String| PhpException::default(e))?,
            path: path.clone(),
            handler_name: "php".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: Vec::new(),
        };
        router.add_route(route.clone());
        route_metadata.push(spikard_core::RouteMetadata {
            method,
            path,
            handler_name: "php".to_string(),
            request_schema: None,
            response_schema: None,
            parameter_schema: None,
            file_params: None,
            is_async: true,
            cors: None,
            body_param_name: None,
            #[cfg(feature = "di")]
            handler_dependencies: None,
        });

        spikard_http::bindings::register_handler(
            &router,
            route.handler_name.clone(),
            Arc::new(PhpHandlerWrapper { handler: handler_zval }),
        );
    }

    Ok((router, route_metadata))
}
