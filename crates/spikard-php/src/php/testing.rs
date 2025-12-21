//! Native PHP test client for HTTP testing.
//!
//! This module implements `NativeTestClient`, a PHP class that provides
//! HTTP testing capabilities against a Spikard server without network overhead.

use axum::routing::get;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, ZendHashTable, Zval};
use serde_json::Value as JsonValue;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::testing::{
    ResponseSnapshot, SseStream as CoreSseStream, TestClient as CoreTestClient, WebSocketConnection,
};
use spikard_http::{Handler, Route, RouteMetadata, ServerConfig};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use super::{PhpHandler, PhpRequest, json_to_php_table, zval_to_json};

type MultipartPayload = Option<(Vec<(String, String)>, Vec<spikard_http::testing::MultipartFilePart>)>;

/// Test response data exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Testing\\TestResponse")]
pub struct PhpTestResponse {
    pub(crate) status: i64,
    pub(crate) body: String,
    pub(crate) headers: HashMap<String, String>,
}

#[php_impl]
impl PhpTestResponse {
    /// Get the HTTP status code.
    #[php(name = "getStatus")]
    pub fn get_status(&self) -> i64 {
        self.status
    }

    /// Alias for status code.
    #[php(name = "getStatusCode")]
    pub fn get_status_code(&self) -> i64 {
        self.status
    }

    /// Get the response body as a string.
    #[php(name = "getBody")]
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    /// Get the response body parsed as JSON.
    #[php(name = "json")]
    pub fn json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let value: JsonValue =
            serde_json::from_str(&self.body).map_err(|e| PhpException::default(format!("Invalid JSON body: {}", e)))?;
        super::json_to_php_table(&value)
    }

    /// Get response headers as a PHP array.
    #[php(name = "getHeaders")]
    pub fn get_headers(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.headers {
            table.insert(k.as_str(), v.as_str())?;
        }
        Ok(table)
    }

    /// Get a specific header value.
    #[php(name = "getHeader")]
    pub fn get_header(&self, name: String) -> Option<String> {
        let name_lower = name.to_ascii_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_ascii_lowercase() == name_lower)
            .map(|(_, v)| v.clone())
    }

    /// Check if response was successful (2xx status).
    #[php(name = "isSuccess")]
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Check if response was a redirect (3xx status).
    #[php(name = "isRedirect")]
    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    /// Check if response was a client error (4xx status).
    #[php(name = "isClientError")]
    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    /// Check if response was a server error (5xx status).
    #[php(name = "isServerError")]
    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }
}

struct NativeClientInner {
    client: Arc<CoreTestClient>,
    #[allow(dead_code)]
    handlers: Vec<Zval>,
}

struct ParsedRoute {
    payload: crate::php::start::RegisteredRoutePayload,
    handler: Zval,
    websocket: bool,
    sse: bool,
}

/// Native test client for PHP that uses the full Rust HTTP stack.
#[php_class]
#[php(name = "Spikard\\Native\\TestClient")]
pub struct PhpNativeTestClient {
    inner: RefCell<Option<NativeClientInner>>,
}

#[php_impl]
impl PhpNativeTestClient {
    #[php(constructor)]
    pub fn __construct(routes: &Zval) -> PhpResult<Self> {
        let parsed_routes = parse_native_routes(routes)?;

        let mut handler_refs = Vec::new();
        let mut route_pairs: Vec<(Route, Arc<dyn Handler>)> = Vec::new();
        let mut route_metadata: Vec<RouteMetadata> = Vec::new();
        let mut websocket_routes = Vec::new();
        let mut sse_routes = Vec::new();

        for route in parsed_routes {
            handler_refs.push(route.handler.shallow_clone());
            if route.websocket {
                websocket_routes.push(route);
                continue;
            }
            if route.sse {
                sse_routes.push(route);
                continue;
            }

            let request_schema = route.payload.request_schema.clone();
            let response_schema = route.payload.response_schema.clone();
            let parameter_schema = route.payload.parameter_schema.clone();
            let jsonrpc_method = route.payload.jsonrpc_method.clone();
            let handler_name = route.payload.handler_name.clone();
            let method = route.payload.method.clone();
            let path = route.payload.path.clone();

            let handler =
                PhpHandler::register_from_zval(&route.handler, handler_name.clone(), method.clone(), path.clone())
                    .map_err(|e| PhpException::default(format!("Failed to register handler: {}", e)))?;

            let mut route_def = route
                .payload
                .into_route()
                .map_err(|e| PhpException::default(format!("Failed to build route: {}", e)))?;

            if let Some(schema) = request_schema.clone() {
                let compiled = spikard_core::validation::SchemaValidator::new(schema)
                    .map_err(|e| PhpException::default(format!("Invalid request schema: {}", e)))?;
                route_def.request_validator = Some(Arc::new(compiled));
            }
            if let Some(schema) = response_schema.clone() {
                let compiled = spikard_core::validation::SchemaValidator::new(schema)
                    .map_err(|e| PhpException::default(format!("Invalid response schema: {}", e)))?;
                route_def.response_validator = Some(Arc::new(compiled));
            }
            if let Some(schema) = parameter_schema.clone() {
                let compiled =
                    spikard_http::ParameterValidator::new(schema).map_err(|e| PhpException::default(e.to_string()))?;
                route_def.parameter_validator = Some(compiled);
            }

            route_metadata.push(RouteMetadata {
                method,
                path,
                handler_name,
                request_schema,
                response_schema,
                parameter_schema,
                file_params: None,
                is_async: true,
                cors: None,
                body_param_name: None,
                handler_dependencies: Some(Vec::new()),
                jsonrpc_method,
            });

            route_pairs.push((route_def, Arc::new(handler) as Arc<dyn Handler>));
        }

        let mut router = build_router_with_handlers_and_config(route_pairs, ServerConfig::default(), route_metadata)
            .map_err(|e| PhpException::default(format!("Failed to build router: {}", e)))?;

        for route in websocket_routes {
            let path = route.payload.path;
            let handler_name = route.payload.handler_name;
            let message_schema = route.payload.request_schema.clone();
            let response_schema = route.payload.response_schema.clone();
            let ws_state =
                super::create_websocket_state(&route.handler, Some(handler_name), message_schema, response_schema)
                    .map_err(|e| PhpException::default(format!("Failed to build WebSocket state: {}", e)))?;
            router = router.route(
                &path,
                get(spikard_http::websocket_handler::<super::PhpWebSocketHandler>).with_state(ws_state),
            );
        }

        for route in sse_routes {
            let path = route.payload.path;
            let sse_state = super::create_sse_state(&route.handler)
                .map_err(|e| PhpException::default(format!("Failed to build SSE state: {}", e)))?;
            router = router.route(
                &path,
                get(spikard_http::sse_handler::<super::PhpSseEventProducer>).with_state(sse_state),
            );
        }

        let client = Arc::new(
            CoreTestClient::from_router(router)
                .map_err(|e| PhpException::default(format!("Test client error: {}", e)))?,
        );

        Ok(Self {
            inner: RefCell::new(Some(NativeClientInner {
                client,
                handlers: handler_refs,
            })),
        })
    }

    /// Execute an HTTP request using the full Rust HTTP stack.
    #[php(name = "request")]
    pub fn request(&self, method: String, path: String, options: Option<&Zval>) -> PhpResult<super::PhpResponse> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let request_options = parse_request_options(options)?;
        let headers = build_header_list(request_options.headers, request_options.cookies);

        let runtime = super::get_runtime()?;
        let response = runtime.block_on(async {
            dispatch_request(
                &inner.client,
                method,
                path,
                request_options.body,
                request_options.multipart,
                headers,
            )
            .await
        })?;

        snapshot_to_php_response(response)
    }

    /// Connect to a WebSocket endpoint for testing.
    #[php(name = "websocket")]
    pub fn websocket(&self, path: String, send_text: Option<String>) -> PhpResult<PhpWebSocketTestConnection> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let runtime = super::get_runtime()?;
        let mut ws = runtime.block_on(spikard_http::testing::connect_websocket(inner.client.server(), &path));

        if let Some(text) = send_text {
            runtime.block_on(ws.send_text(text));
        }

        Ok(PhpWebSocketTestConnection::from_connection(ws, inner.client.clone()))
    }

    /// Connect to an SSE endpoint for testing.
    #[php(name = "sse")]
    pub fn sse(&self, path: String) -> PhpResult<PhpSseStream> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let runtime = super::get_runtime()?;
        let response = runtime.block_on(async {
            inner
                .client
                .get(&path, None, None)
                .await
                .map_err(|e| PhpException::default(format!("SSE request failed: {}", e)))
        })?;

        PhpSseStream::from_snapshot(response)
    }

    /// Close the test client and release resources.
    #[php(name = "close")]
    pub fn close(&self) -> PhpResult<()> {
        *self.inner.borrow_mut() = None;
        Ok(())
    }
}

/// Native test client for PHP.
///
/// This provides direct HTTP testing without network overhead by
/// directly invoking PHP handlers.
#[php_class]
#[php(name = "Spikard\\Testing\\NativeTestClient")]
#[derive(Default)]
pub struct PhpTestClient {}

#[php_impl]
impl PhpTestClient {
    /// Create a new test client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a GET request.
    ///
    /// This is a simplified implementation that directly calls the handler.
    #[php(name = "get")]
    pub fn get_request(
        &self,
        path: String,
        handler: ZendCallable,
        query: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("GET", &path, handler, None, query, headers)
    }

    /// Execute a POST request.
    #[php(name = "post")]
    pub fn post_request(
        &self,
        path: String,
        handler: ZendCallable,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("POST", &path, handler, body, None, headers)
    }

    /// Execute a PUT request.
    #[php(name = "put")]
    pub fn put_request(
        &self,
        path: String,
        handler: ZendCallable,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("PUT", &path, handler, body, None, headers)
    }

    /// Execute a PATCH request.
    #[php(name = "patch")]
    pub fn patch_request(
        &self,
        path: String,
        handler: ZendCallable,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("PATCH", &path, handler, body, None, headers)
    }

    /// Execute a DELETE request.
    #[php(name = "delete")]
    pub fn delete_request(
        &self,
        path: String,
        handler: ZendCallable,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("DELETE", &path, handler, None, None, headers)
    }

    /// Execute a generic request with any HTTP method.
    #[php(name = "request")]
    pub fn request(
        &self,
        method: String,
        path: String,
        handler: ZendCallable,
        body: Option<String>,
        query: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request(&method, &path, handler, body, query, headers)
    }
}

/// Execute a test request by directly invoking the PHP handler.
///
/// This bypasses the HTTP stack and calls the handler directly, which is
/// much faster for unit testing.
fn execute_test_request(
    method: &str,
    path: &str,
    handler: ZendCallable,
    body: Option<String>,
    query: Option<String>,
    headers: Option<HashMap<String, String>>,
) -> PhpResult<PhpTestResponse> {
    let body_value = body
        .as_ref()
        .map(|b| serde_json::from_str(b).unwrap_or(JsonValue::String(b.clone())))
        .unwrap_or(JsonValue::Null);

    let raw_query = parse_query_string(query.as_deref());

    let php_request = PhpRequest::from_parts(
        method.to_string(),
        path.to_string(),
        body_value,
        JsonValue::Object(serde_json::Map::new()),
        body.map(|b| bytes::Bytes::from(b.into_bytes())),
        headers.clone().unwrap_or_default(),
        HashMap::new(),
        raw_query,
        extract_path_params(path),
        None,
    );

    let request_zval = php_request
        .into_zval(false)
        .map_err(|e| PhpException::default(format!("Failed to create request object: {:?}", e)))?;

    let response_zval = handler
        .try_call(vec![&request_zval])
        .map_err(|e| PhpException::default(format!("Handler failed: {:?}", e)))?;

    zval_to_test_response(&response_zval)
}

/// Parse a query string into a HashMap.
fn parse_query_string(query: Option<&str>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    if let Some(q) = query {
        for pair in q.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = urlencoding::decode(key).unwrap_or_else(|_| key.into()).to_string();
                let value = urlencoding::decode(value).unwrap_or_else(|_| value.into()).to_string();
                result.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
    }

    result
}

/// Extract path parameters from a path.
/// This is a simple implementation - real path params come from the router.
fn extract_path_params(_path: &str) -> HashMap<String, String> {
    HashMap::new()
}

/// Convert a Zval response to a PhpTestResponse.
fn zval_to_test_response(response: &Zval) -> PhpResult<PhpTestResponse> {
    if response.is_null() {
        return Ok(PhpTestResponse {
            status: 204,
            body: String::new(),
            headers: HashMap::new(),
        });
    }

    if let Some(obj) = response.object()
        && let Ok(class_name) = obj.get_class_name()
        && class_name.contains("Response")
        && let Ok(status_zval) = obj.try_call_method("getStatus", vec![])
    {
        let status = status_zval.long().unwrap_or(200);

        let body = if let Ok(body_zval) = obj.try_call_method("getBody", vec![]) {
            body_zval.string().map(|s| s.to_string()).unwrap_or_default()
        } else {
            String::new()
        };

        let mut headers = HashMap::new();
        if let Ok(headers_zval) = obj.try_call_method("getHeaders", vec![])
            && let Some(arr) = headers_zval.array()
        {
            for (key, val) in arr.iter() {
                let key_str = match key {
                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                };
                if let Some(val_str) = val.string() {
                    headers.insert(key_str, val_str.to_string());
                }
            }
        }

        return Ok(PhpTestResponse { status, body, headers });
    }

    if let Some(s) = response.string() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        return Ok(PhpTestResponse {
            status: 200,
            body: s.to_string(),
            headers,
        });
    }

    let body_json =
        zval_to_json(response).map_err(|e| PhpException::default(format!("Failed to convert response: {}", e)))?;

    let body = serde_json::to_string(&body_json).unwrap_or_default();
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());

    Ok(PhpTestResponse {
        status: 200,
        body,
        headers,
    })
}

struct ParsedRequestOptions {
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Option<JsonValue>,
    multipart: MultipartPayload,
}

fn parse_native_routes(routes: &Zval) -> PhpResult<Vec<ParsedRoute>> {
    let routes_array = routes
        .array()
        .ok_or_else(|| PhpException::default("Routes must be an array".to_string()))?;

    let mut parsed = Vec::new();

    for (_idx, route_val) in routes_array.iter() {
        let route_array = route_val
            .array()
            .ok_or_else(|| PhpException::default("Route must be an array".to_string()))?;

        let handler = route_array
            .get("handler")
            .ok_or_else(|| PhpException::default("Missing handler callable".to_string()))?;

        let websocket = route_array.get("websocket").and_then(|v| v.bool()).unwrap_or(false);
        let sse = route_array.get("sse").and_then(|v| v.bool()).unwrap_or(false);

        let json_val = zval_to_json(route_val)
            .map_err(|e| PhpException::default(format!("Failed to convert route to JSON: {}", e)))?;

        let payload = serde_json::from_value::<crate::php::start::RegisteredRoutePayload>(json_val)
            .map_err(|e| PhpException::default(format!("Invalid route payload: {}", e)))?;

        parsed.push(ParsedRoute {
            payload,
            handler: handler.shallow_clone(),
            websocket,
            sse,
        });
    }

    Ok(parsed)
}

fn parse_request_options(options: Option<&Zval>) -> PhpResult<ParsedRequestOptions> {
    let mut headers = HashMap::new();
    let mut cookies = HashMap::new();
    let mut body = None;
    let mut multipart = None;

    let Some(options_val) = options else {
        return Ok(ParsedRequestOptions {
            headers,
            cookies,
            body,
            multipart,
        });
    };

    let Some(options_array) = options_val.array() else {
        return Ok(ParsedRequestOptions {
            headers,
            cookies,
            body,
            multipart,
        });
    };

    headers = parse_string_map(options_array.get("headers"));
    cookies = parse_string_map(options_array.get("cookies"));

    if let Some(body_val) = options_array.get("body")
        && !body_val.is_null()
    {
        body = Some(zval_to_json(body_val).map_err(PhpException::default)?);
    }

    let files = parse_files(options_array.get("files"));
    if body.is_none() && !files.is_empty() {
        multipart = Some((Vec::new(), files));
    }

    Ok(ParsedRequestOptions {
        headers,
        cookies,
        body,
        multipart,
    })
}

fn parse_string_map(value: Option<&Zval>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let Some(array) = value.and_then(|val| val.array()) else {
        return result;
    };

    for (key, val) in array.iter() {
        let key_str = array_key_to_string(key);
        if let Some(val_str) = val.string() {
            result.insert(key_str, val_str.to_string());
        }
    }

    result
}

fn parse_files(value: Option<&Zval>) -> Vec<spikard_http::testing::MultipartFilePart> {
    let mut files = Vec::new();
    let Some(array) = value.and_then(|val| val.array()) else {
        return files;
    };

    for (key, val) in array.iter() {
        let field_name = array_key_to_string(key);
        let content = if let Some(val_str) = val.string() {
            val_str.to_string()
        } else {
            let json_val = zval_to_json(val).unwrap_or(JsonValue::Null);
            serde_json::to_string(&json_val).unwrap_or_default()
        };

        files.push(spikard_http::testing::MultipartFilePart {
            field_name: field_name.clone(),
            filename: field_name,
            content_type: None,
            content: content.into_bytes(),
        });
    }

    files
}

fn array_key_to_string(key: ext_php_rs::types::ArrayKey) -> String {
    match key {
        ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
        ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
        ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
    }
}

fn build_header_list(
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let mut combined: Vec<(String, String)> = headers.into_iter().collect();
    if !cookies.is_empty() {
        let cookie_header = cookies
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("; ");
        combined.push(("cookie".to_string(), cookie_header));
    }
    if combined.is_empty() { None } else { Some(combined) }
}

async fn dispatch_request(
    client: &CoreTestClient,
    method: String,
    path: String,
    body: Option<JsonValue>,
    multipart: MultipartPayload,
    headers: Option<Vec<(String, String)>>,
) -> PhpResult<ResponseSnapshot> {
    let method_upper = method.to_ascii_uppercase();
    let response = match method_upper.as_str() {
        "GET" => client.get(&path, None, headers).await,
        "DELETE" => client.delete(&path, None, headers).await,
        "HEAD" => client.head(&path, None, headers).await,
        "OPTIONS" => client.options(&path, None, headers).await,
        "TRACE" => client.trace(&path, None, headers).await,
        "POST" => client.post(&path, body, None, multipart, None, headers).await,
        "PUT" => client.put(&path, body, None, headers).await,
        "PATCH" => client.patch(&path, body, None, headers).await,
        _ => {
            return Err(PhpException::default(format!(
                "Unsupported HTTP method: {}",
                method_upper
            )));
        }
    };

    response.map_err(|e| PhpException::default(format!("Test request failed: {}", e)))
}

fn snapshot_to_php_response(snapshot: ResponseSnapshot) -> PhpResult<super::PhpResponse> {
    let status_code = snapshot.status as i64;
    let headers = snapshot.headers;

    let body_value = if snapshot.body.is_empty() {
        JsonValue::Null
    } else if is_json_response(&headers) {
        serde_json::from_slice(&snapshot.body)
            .unwrap_or_else(|_| JsonValue::String(String::from_utf8_lossy(&snapshot.body).into_owned()))
    } else {
        JsonValue::String(String::from_utf8_lossy(&snapshot.body).into_owned())
    };

    Ok(super::PhpResponse {
        status_code,
        body: body_value,
        headers,
        cookies: HashMap::new(),
    })
}

fn is_json_response(headers: &HashMap<String, String>) -> bool {
    headers
        .get("content-type")
        .map(|value| value.starts_with("application/json"))
        .unwrap_or(false)
}

/// Advanced test client that uses axum-test for full HTTP stack testing.
///
/// This client creates an in-memory HTTP server and sends real HTTP requests,
/// which tests the full middleware stack.
#[php_class]
#[php(name = "Spikard\\Testing\\HttpTestClient")]
#[derive(Default)]
pub struct PhpHttpTestClient {}

#[php_impl]
impl PhpHttpTestClient {
    /// Create a new HTTP test client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute a test request using the full HTTP stack.
    ///
    /// This creates a temporary test server and executes the request.
    #[php(name = "execute")]
    pub fn execute(
        &self,
        method: String,
        path: String,
        handler: ZendCallable,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        let body_value = body
            .as_ref()
            .map(|b| serde_json::from_str(b).unwrap_or(JsonValue::String(b.clone())))
            .unwrap_or(JsonValue::Null);

        let php_request = PhpRequest::from_parts(
            method,
            path,
            body_value,
            JsonValue::Object(serde_json::Map::new()),
            body.map(|b| bytes::Bytes::from(b.into_bytes())),
            headers.unwrap_or_default(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            None,
        );

        let request_zval = php_request
            .into_zval(false)
            .map_err(|e| PhpException::default(format!("Failed to create request: {:?}", e)))?;

        let response_zval = handler
            .try_call(vec![&request_zval])
            .map_err(|e| PhpException::default(format!("Handler failed: {:?}", e)))?;

        zval_to_test_response(&response_zval)
    }

    /// Connect to a WebSocket endpoint for testing.
    ///
    /// This calls the WebSocket handler and returns a connection object.
    #[php(name = "websocket")]
    pub fn websocket(&self, path: String, handler: ZendCallable) -> PhpResult<PhpWebSocketTestConnection> {
        PhpWebSocketTestConnection::connect(path, handler)
    }

    /// Connect to a Server-Sent Events endpoint for testing.
    ///
    /// This calls the SSE producer and returns a stream object.
    #[php(name = "sse")]
    pub fn sse(&self, path: String, handler: ZendCallable) -> PhpResult<PhpSseStream> {
        PhpSseStream::connect(path, handler)
    }
}

/// WebSocket test connection for PHP.
///
/// This provides methods to send and receive WebSocket messages in tests.
#[php_class]
#[php(name = "Spikard\\Testing\\WebSocketTestConnection")]
pub struct PhpWebSocketTestConnection {
    inner: RefCell<Option<WebSocketConnection>>,
    #[allow(dead_code)]
    keepalive: Arc<CoreTestClient>,
}

impl PhpWebSocketTestConnection {
    fn connect(_path: String, _handler: ZendCallable) -> PhpResult<Self> {
        Err(PhpException::default(
            "Native WebSocket client is not available without Spikard\\Native\\TestClient".to_string(),
        ))
    }

    fn from_connection(connection: WebSocketConnection, keepalive: Arc<CoreTestClient>) -> Self {
        Self {
            inner: RefCell::new(Some(connection)),
            keepalive,
        }
    }

    fn with_connection_mut<F, T>(&self, op: F) -> PhpResult<T>
    where
        F: FnOnce(&mut WebSocketConnection) -> PhpResult<T>,
    {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| PhpException::default("WebSocket connection is closed".to_string()))?;
        op(ws)
    }
}

#[php_impl]
impl PhpWebSocketTestConnection {
    /// Send a text message to the WebSocket.
    #[php(name = "sendText")]
    pub fn send_text(&mut self, text: String) -> PhpResult<()> {
        self.with_connection_mut(|ws| {
            let runtime = super::get_runtime()?;
            runtime.block_on(ws.send_text(text));
            Ok(())
        })
    }

    /// Send a JSON message to the WebSocket.
    #[php(name = "sendJson")]
    pub fn send_json(&mut self, data: String) -> PhpResult<()> {
        let value: JsonValue =
            serde_json::from_str(&data).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
        self.with_connection_mut(|ws| {
            let runtime = super::get_runtime()?;
            runtime.block_on(ws.send_json(&value));
            Ok(())
        })
    }

    /// Receive a text message from the WebSocket.
    #[php(name = "receiveText")]
    pub fn receive_text(&self) -> PhpResult<String> {
        self.with_connection_mut(|ws| {
            let runtime = super::get_runtime()?;
            let text = runtime.block_on(ws.receive_text());
            Ok(text)
        })
    }

    /// Receive a JSON message from the WebSocket.
    #[php(name = "receiveJson")]
    pub fn receive_json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let text = self.receive_text()?;
        let value: JsonValue =
            serde_json::from_str(&text).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
        json_to_php_table(&value)
    }

    /// Receive raw bytes from the WebSocket.
    #[php(name = "receiveBytes")]
    pub fn receive_bytes(&self) -> PhpResult<Vec<u8>> {
        self.with_connection_mut(|ws| {
            let runtime = super::get_runtime()?;
            let bytes = runtime.block_on(ws.receive_bytes());
            Ok(bytes.to_vec())
        })
    }

    /// Close the WebSocket connection.
    #[php(name = "close")]
    pub fn close(&mut self) -> PhpResult<()> {
        let mut inner = self.inner.borrow_mut();
        if let Some(ws) = inner.take() {
            let runtime = super::get_runtime()?;
            runtime.block_on(ws.close());
        }
        Ok(())
    }

    /// Check if the connection is closed.
    #[php(name = "isClosed")]
    pub fn is_closed(&self) -> bool {
        self.inner.borrow().is_none()
    }
}

/// SSE stream for PHP testing.
///
/// This provides methods to read Server-Sent Events in tests.
#[php_class]
#[php(name = "Spikard\\Testing\\SseStream")]
pub struct PhpSseStream {
    events: Vec<PhpSseEvent>,
}

impl PhpSseStream {
    fn connect(_path: String, _handler: ZendCallable) -> PhpResult<Self> {
        Err(PhpException::default(
            "Native SSE client is not available without Spikard\\Native\\TestClient".to_string(),
        ))
    }

    fn from_snapshot(snapshot: ResponseSnapshot) -> PhpResult<Self> {
        let stream = CoreSseStream::from_response(&snapshot)
            .map_err(|e| PhpException::default(format!("Failed to parse SSE stream: {}", e)))?;
        let events = stream
            .events()
            .iter()
            .map(|event| PhpSseEvent {
                data: event.data.clone(),
                event_type: None,
                id: None,
            })
            .collect::<Vec<_>>();
        Ok(Self { events })
    }
}

#[php_impl]
impl PhpSseStream {
    /// Get all events from the stream as an array.
    #[php(name = "events")]
    pub fn events(&self) -> PhpResult<Vec<PhpSseEvent>> {
        Ok(self.events.clone())
    }

    /// Get all events as JSON values.
    #[php(name = "eventsAsJson")]
    pub fn events_as_json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for event in &self.events {
            let json_value: JsonValue = serde_json::from_str(&event.data)
                .map_err(|e| PhpException::default(format!("Invalid JSON in event: {}", e)))?;
            table.push(json_to_php_table(&json_value)?)?;
        }
        Ok(table)
    }

    /// Get the raw body of the SSE response.
    #[php(name = "body")]
    pub fn body(&self) -> String {
        self.events
            .iter()
            .map(|e| format!("data: {}\n\n", e.data))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get the number of events in the stream.
    #[php(name = "count")]
    pub fn count(&self) -> i64 {
        self.events.len() as i64
    }
}

/// SSE event for PHP testing.
///
/// Represents a single Server-Sent Event.
#[php_class]
#[php(name = "Spikard\\Testing\\SseEvent")]
#[derive(Clone)]
pub struct PhpSseEvent {
    pub(crate) data: String,
    pub(crate) event_type: Option<String>,
    pub(crate) id: Option<String>,
}

#[php_impl]
impl PhpSseEvent {
    /// Get the data field of the event.
    #[php(name = "getData")]
    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    /// Parse the event data as JSON.
    #[php(name = "asJson")]
    pub fn as_json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let value: JsonValue =
            serde_json::from_str(&self.data).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
        json_to_php_table(&value)
    }

    /// Get the event type if specified.
    #[php(name = "getEventType")]
    pub fn get_event_type(&self) -> Option<String> {
        self.event_type.clone()
    }

    /// Get the event ID if specified.
    #[php(name = "getId")]
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}
