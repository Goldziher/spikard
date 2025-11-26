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
use spikard_http::lifecycle::{HookResult, LifecycleHooks};
use spikard_http::testing::{call_test_server, snapshot_response};
use spikard_http::websocket::{WebSocketHandler as CoreWebSocketHandler, WebSocketState};
use spikard_http::{
    CompressionConfig, CorsConfig, Handler, HandlerResult, RateLimitConfig, RequestData, Route, Router, Server,
    ServerConfig, StaticFilesConfig,
};
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

#[derive(Clone)]
struct PhpWebSocketWrapper {
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

impl CoreWebSocketHandler for PhpWebSocketWrapper {
    fn handle_message(
        &self,
        message: serde_json::Value,
    ) -> impl std::future::Future<Output = Option<serde_json::Value>> + Send {
        let handler = self.handler.clone();
        async move {
            let payload = Zval::from(message.to_string());
            handler
                .try_call_method("onMessage", vec![&payload])
                .ok()
                .and_then(|zv| zv.string())
                .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        }
    }

    fn on_connect(&self) -> impl std::future::Future<Output = ()> + Send {
        let handler = self.handler.clone();
        async move {
            let _ = handler.try_call_method("onConnect", vec![]);
        }
    }

    fn on_disconnect(&self) -> impl std::future::Future<Output = ()> + Send {
        let handler = self.handler.clone();
        async move {
            let _ = handler.try_call_method("onClose", vec![&Zval::from(1000i64), &Zval::from("")]);
        }
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
pub fn spikard_start_server(
    routes: Vec<Zval>,
    config: HashMap<String, Zval>,
    lifecycle: Option<HashMap<String, Zval>>,
) -> Result<i64, PhpException> {
    let runtime = Runtime::new().map_err(|e| PhpException::default(e.to_string()))?;

    let (router, metadata) = build_router_from_php(routes)?;
    let server_config = extract_server_config(config)?;
    let hooks = extract_lifecycle_hooks(lifecycle.unwrap_or_default())?;

    let axum_router =
        spikard_http::router::build_router_with_config_and_hooks(router, server_config.clone(), metadata, hooks)
            .map_err(|e| PhpException::default(e.to_string()))?;

    let handle_id = HANDLE_REGISTRY.register(runtime, axum_router, server_config)?;
    Ok(handle_id)
}

#[php_function]
pub fn spikard_stop_server(handle: i64) -> bool {
    HANDLE_REGISTRY.stop(handle)
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
        let axum_router =
            spikard_http::router::build_router_for_testing(router, route_metadata).map_err(PhpException::default)?;
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

        let mut builder = Request::builder()
            .method(&uppercase_method)
            .uri(&path)
            .map_err(|e| PhpException::default(e.to_string()))?;

        for (k, v) in &headers {
            builder = builder.header(k, v);
        }
        if !cookies.is_empty() {
            let cookie_header = cookies
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("; ");
            builder = builder.header("cookie", cookie_header);
        }

        let body_bytes = body_to_bytes(&body);
        let request = builder
            .body(Body::from(body_bytes))
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
        data.raw_body.as_ref().map(|b| b.to_vec()),
        Some((*data.headers).clone()),
        Some((*data.cookies).clone()),
        Some((*data.raw_query_params).clone()),
        Some((*data.path_params).clone()),
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
            // Try to coerce to JSON if possible
            if let Ok(nested) = zval_to_json(&v) {
                map.insert(key, nested);
            } else {
                map.insert(key, Value::String(v.to_string()));
            }
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

fn body_to_bytes(body: &Value) -> Vec<u8> {
    match body {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Null => Vec::new(),
        other => serde_json::to_vec(other).unwrap_or_default(),
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
        let is_ws = array
            .get("websocket")
            .ok()
            .flatten()
            .and_then(|v| v.bool())
            .unwrap_or(false);
        let is_sse = array.get("sse").ok().flatten().and_then(|v| v.bool()).unwrap_or(false);

        let handler_zval = array
            .get("handler")
            .ok()
            .flatten()
            .cloned()
            .unwrap_or_else(|| Zval::new());

        if is_ws {
            let handler = Arc::new(PhpWebSocketWrapper { handler: handler_zval });
            let state = WebSocketState::new(handler);
            router.route_ws(&path, state);
            continue;
        }

        if is_sse {
            // TODO: implement native SSE mapping; for now fall through to HTTP handler semantics.
        }

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

fn extract_server_config(config: HashMap<String, Zval>) -> Result<ServerConfig, PhpException> {
    let mut server = ServerConfig::default();

    if let Some(c) = config.get("compression") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(c) {
            let gzip = arr.get("gzip").ok().and_then(|v| v.bool()).unwrap_or(true);
            let brotli = arr.get("brotli").ok().and_then(|v| v.bool()).unwrap_or(true);
            let min_size = arr.get("minSize").ok().and_then(|v| v.long()).unwrap_or(1024) as usize;
            let quality = arr.get("quality").ok().and_then(|v| v.long()).unwrap_or(6) as u32;
            server.compression = Some(CompressionConfig {
                gzip,
                brotli,
                min_size,
                quality,
            });
        }
    }

    if let Some(r) = config.get("rateLimit") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(r) {
            if let (Some(per_second), Some(burst)) = (
                arr.get("perSecond").ok().and_then(|v| v.long()),
                arr.get("burst").ok().and_then(|v| v.long()),
            ) {
                let ip_based = arr.get("ipBased").ok().and_then(|v| v.bool()).unwrap_or(true);
                server.rate_limit = Some(RateLimitConfig {
                    per_second: per_second as u64,
                    burst: burst as u32,
                    ip_based,
                });
            }
        }
    }

    if let Some(c) = config.get("cors") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(c) {
            let allow_origins = arr
                .get("allowOrigins")
                .ok()
                .and_then(|v| to_string_vec(v))
                .unwrap_or_default();
            let allow_methods = arr
                .get("allowMethods")
                .ok()
                .and_then(|v| to_string_vec(v))
                .unwrap_or_default();
            let allow_headers = arr
                .get("allowHeaders")
                .ok()
                .and_then(|v| to_string_vec(v))
                .unwrap_or_default();
            let expose_headers = arr
                .get("exposeHeaders")
                .ok()
                .and_then(|v| to_string_vec(v))
                .unwrap_or_default();
            let allow_credentials = arr.get("allowCredentials").ok().and_then(|v| v.bool()).unwrap_or(true);
            let max_age = arr.get("maxAge").ok().and_then(|v| v.long()).map(|v| v as u64);

            server.cors = Some(CorsConfig {
                allow_origins,
                allow_methods,
                allow_headers,
                expose_headers,
                allow_credentials,
                max_age,
            });
        }
    }

    if let Some(s) = config.get("staticFiles") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(s) {
            let mut files = Vec::new();
            for (_, item) in arr.iter() {
                if let Ok(item_arr) = ext_php_rs::types::array::Array::try_from(item) {
                    if let (Some(directory), Some(route_prefix)) = (
                        item_arr.get("directory").ok().and_then(|v| v.string()),
                        item_arr.get("routePrefix").ok().and_then(|v| v.string()),
                    ) {
                        let index_file = item_arr.get("indexFile").ok().and_then(|v| v.bool()).unwrap_or(true);
                        let cache_control = item_arr
                            .get("cacheControl")
                            .ok()
                            .and_then(|v| v.string())
                            .map(|s| s.to_string());
                        files.push(StaticFilesConfig {
                            directory: directory.to_string(),
                            route_prefix: route_prefix.to_string(),
                            index_file,
                            cache_control,
                        });
                    }
                }
            }
            server.static_files = files;
        }
    }

    // Host/port
    if let Some(h) = config.get("host").and_then(|v| v.string()) {
        server.host = h.to_string();
    }
    if let Some(p) = config.get("port").and_then(|v| v.long()) {
        server.port = p as u16;
    }

    Ok(server)
}

fn extract_lifecycle_hooks(hooks: HashMap<String, Zval>) -> Result<LifecycleHooks, PhpException> {
    let make_hook = |key: &str, hooks: &HashMap<String, Zval>| -> Option<Arc<PhpHandlerWrapper>> {
        hooks
            .get(key)
            .map(|cb| Arc::new(PhpHandlerWrapper { handler: cb.clone() }))
    };

    let on_request = make_hook("onRequest", &hooks);
    let pre_validation = make_hook("preValidation", &hooks);
    let pre_handler = make_hook("preHandler", &hooks);
    let on_error = make_hook("onError", &hooks);
    let on_response = make_hook("onResponse", &hooks);

    Ok(LifecycleHooks {
        on_request: on_request.map(|handler| {
            Box::new(move |_req, _ctx| {
                let _ = handler
                    .handler
                    .try_call_method("invoke", vec![])
                    .map_err(|e| eprintln!("onRequest hook error: {}", e));
                HookResult::Continue
            }) as _
        }),
        pre_validation: pre_validation.map(|handler| {
            Box::new(move |_req, _ctx| {
                let _ = handler
                    .handler
                    .try_call_method("invoke", vec![])
                    .map_err(|e| eprintln!("preValidation hook error: {}", e));
                HookResult::Continue
            }) as _
        }),
        pre_handler: pre_handler.map(|handler| {
            Box::new(move |_req, _ctx| {
                let _ = handler
                    .handler
                    .try_call_method("invoke", vec![])
                    .map_err(|e| eprintln!("preHandler hook error: {}", e));
                HookResult::Continue
            }) as _
        }),
        on_error: on_error.map(|handler| {
            Box::new(move |_req, _err, _ctx| {
                let _ = handler
                    .handler
                    .try_call_method("invoke", vec![])
                    .map_err(|e| eprintln!("onError hook error: {}", e));
                HookResult::Continue
            }) as _
        }),
        on_response: on_response.map(|handler| {
            Box::new(move |_req, _res, _ctx| {
                let _ = handler
                    .handler
                    .try_call_method("invoke", vec![])
                    .map_err(|e| eprintln!("onResponse hook error: {}", e));
                HookResult::Continue
            }) as _
        }),
    })
}
