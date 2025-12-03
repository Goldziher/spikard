//! Native PHP test client for HTTP testing.
//!
//! This module implements `NativeTestClient`, a PHP class that provides
//! HTTP testing capabilities against a Spikard server without network overhead.

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Request, StatusCode};
use axum::routing::get;
use bytes::Bytes;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, ZendHashTable, Zval};
use http_body_util::BodyExt;
use serde_json::Value as JsonValue;
use spikard_http::server::handler::ValidatingHandler;
use spikard_http::server::lifecycle_execution::execute_with_lifecycle_hooks;
use spikard_http::testing::{SseStream, WebSocketConnection};
use spikard_http::validation::SchemaValidator;
use spikard_http::websocket::websocket_handler;
use spikard_http::{LifecycleHooks, Method, ParameterValidator, RequestData, Route};
use std::collections::HashMap;
use std::sync::Arc;

use super::{PhpRequest, json_to_php_table, zval_to_json};
use crate::php::sse::create_sse_state;
use crate::php::websocket::create_websocket_state;

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

/// Native test client for PHP.
///
/// This provides direct HTTP testing without network overhead by
/// directly invoking PHP handlers.
#[php_class]
#[php(name = "Spikard\\Testing\\NativeTestClient")]
#[derive(Default)]
pub struct PhpTestClient {
    // No state needed - each request invokes the handler directly
}

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
        handler: &Zval,
        query: Option<String>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("GET", &path, handler, None, query, headers, None, None, None)
    }

    /// Execute a POST request.
    #[php(name = "post")]
    pub fn post_request(
        &self,
        path: String,
        handler: &Zval,
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("POST", &path, handler, body, None, headers, None, None, None)
    }

    /// Execute a PUT request.
    #[php(name = "put")]
    pub fn put_request(
        &self,
        path: String,
        handler: &Zval,
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("PUT", &path, handler, body, None, headers, None, None, None)
    }

    /// Execute a PATCH request.
    #[php(name = "patch")]
    pub fn patch_request(
        &self,
        path: String,
        handler: &Zval,
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("PATCH", &path, handler, body, None, headers, None, None, None)
    }

    /// Execute a DELETE request.
    #[php(name = "delete")]
    pub fn delete_request(
        &self,
        path: String,
        handler: &Zval,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request("DELETE", &path, handler, None, None, headers, None, None, None)
    }

    /// Execute a generic request with any HTTP method.
    #[php(name = "request")]
    pub fn request(
        &self,
        method: String,
        path: String,
        handler: &Zval,
        body: Option<&Zval>,
        query: Option<String>,
        headers: Option<HashMap<String, String>>,
        request_schema: Option<&Zval>,
        parameter_schema: Option<&Zval>,
        hooks: Option<&Zval>,
        path_template: Option<String>,
        websocket: Option<bool>,
        sse: Option<bool>,
        websocket_message_schema: Option<&Zval>,
        websocket_response_schema: Option<&Zval>,
        sse_event_schema: Option<&Zval>,
    ) -> PhpResult<PhpTestResponse> {
        execute_test_request(
            &method,
            &path,
            handler,
            body,
            query,
            headers,
            request_schema,
            parameter_schema,
            hooks,
            path_template,
            websocket.unwrap_or(false),
            sse.unwrap_or(false),
            websocket_message_schema,
            websocket_response_schema,
            sse_event_schema,
        )
    }
}

/// Execute a test request by directly invoking the PHP handler.
///
/// This bypasses the HTTP stack and calls the handler directly, which is
/// much faster for unit testing.
fn execute_test_request(
    method: &str,
    path: &str,
    handler: &Zval,
    body: Option<&Zval>,
    query: Option<String>,
    headers: Option<HashMap<String, String>>,
    request_schema: Option<&Zval>,
    parameter_schema: Option<&Zval>,
    hooks: Option<&Zval>,
    path_template: Option<String>,
    websocket: bool,
    sse: bool,
    websocket_message_schema: Option<&Zval>,
    websocket_response_schema: Option<&Zval>,
    sse_event_schema: Option<&Zval>,
) -> PhpResult<PhpTestResponse> {
    let (body_value, raw_body) = body_to_json_and_raw(body)?;

    let raw_query = parse_query_string(query.as_deref());
    let query_params = raw_query_to_value(&raw_query);
    let header_input = headers.unwrap_or_default();
    let mut header_map = axum::http::HeaderMap::new();
    let mut headers = HashMap::new();
    for (key, value) in header_input {
        let name_lower = key.to_ascii_lowercase();
        headers.insert(name_lower.clone(), value.clone());
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_bytes(name_lower.as_bytes()),
            HeaderValue::from_str(&value),
        ) {
            header_map.insert(name, val);
        }
    }

    let mut path_params = derive_path_params(path, path_template.as_deref());
    let cookies = spikard_http::server::request_extraction::extract_cookies(&header_map);

    let method_upper = method.to_ascii_uppercase();

    let request_validator = request_schema
        .map(|schema| {
            let value =
                zval_to_json(schema).map_err(|e| PhpException::default(format!("Invalid request schema: {e}")))?;
            SchemaValidator::new(value)
                .map(Arc::new)
                .map_err(|e| PhpException::default(format!("Failed to compile request schema: {e}")))
        })
        .transpose()?;

    let mut parameter_schema_json: Option<JsonValue> = None;
    let parameter_validator = parameter_schema
        .map(|schema| {
            let value =
                zval_to_json(schema).map_err(|e| PhpException::default(format!("Invalid parameter schema: {e}")))?;
            parameter_schema_json = Some(value.clone());
            ParameterValidator::new(value)
                .map_err(|e| PhpException::default(format!("Failed to compile parameter schema: {e}")))
        })
        .transpose()?;

    if path_params.is_empty() {
        if let Some(schema) = parameter_schema_json.as_ref() {
            path_params = derive_path_params_from_schema(path, schema);
        }
    }

    if websocket {
        let message_schema = websocket_message_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid websocket message schema: {}", e)))?;
        let response_schema = websocket_response_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid websocket response schema: {}", e)))?;
        let state = create_websocket_state(
            handler,
            Some("php_websocket".to_string()),
            message_schema,
            response_schema,
        )
        .map_err(|e| PhpException::default(format!("Failed to register WebSocket handler: {}", e)))?;
        let path_norm = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        let router = axum::Router::new().route(
            &path_norm,
            get(websocket_handler::<crate::php::websocket::PhpWebSocketHandler>).with_state(state),
        );
        let server = spikard_http::testing::TestServer::new(router)
            .map_err(|e| PhpException::default(format!("Failed to create test server: {}", e)))?;
        let mut conn = spikard_http::testing::connect_websocket(&server, &path_norm).await;
        let text = conn.receive_text().await;
        return Ok(PhpTestResponse {
            status: 101,
            body: text,
            headers: HashMap::new(),
        });
    }

    if sse {
        let event_schema = sse_event_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid SSE event schema: {}", e)))?;
        let state = create_sse_state(handler, event_schema)
            .map_err(|e| PhpException::default(format!("Failed to register SSE producer: {}", e)))?;
        let path_norm = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        let router = axum::Router::new().route(
            &path_norm,
            get(spikard_http::sse::sse_handler::<crate::php::sse::PhpSseEventProducer>).with_state(state),
        );
        let server = spikard_http::testing::TestServer::new(router)
            .map_err(|e| PhpException::default(format!("Failed to create test server: {}", e)))?;
        let response = server.get(&path_norm).await;
        let snapshot = spikard_http::testing::snapshot_response(response)
            .await
            .map_err(|e| PhpException::default(format!("Failed to read SSE response: {}", e)))?;
        return Ok(PhpTestResponse {
            status: snapshot.status as i64,
            body: snapshot.text().unwrap_or_default(),
            headers: snapshot.headers,
        });
    }

    let handler = crate::php::handler::PhpHandler::register_from_zval(
        handler,
        "php_handler".to_string(),
        method.to_string(),
        path.to_string(),
    )
    .map_err(|e| PhpException::default(e))?;

    let route = Route {
        method: method_upper.parse().unwrap_or(Method::GET),
        path: path.to_string(),
        handler_name: "php_handler".to_string(),
        request_validator,
        response_validator: None,
        parameter_validator,
        file_params: None,
        is_async: false,
        cors: None,
        expects_json_body: body_value.is_object() || body_value.is_array(),
        handler_dependencies: vec![],
    };

    let handler: Arc<dyn spikard_http::Handler> = Arc::new(handler);
    let handler: Arc<dyn spikard_http::Handler> =
        if route.request_validator.is_some() || route.parameter_validator.is_some() {
            Arc::new(ValidatingHandler::new(handler, &route))
        } else {
            handler
        };

    let lifecycle_hooks: Option<Arc<LifecycleHooks>> = hooks
        .map(crate::php::start::extract_lifecycle_hooks_from_php)
        .transpose()
        .map_err(|e| PhpException::default(format!("Invalid lifecycle hooks: {}", e)))?;

    let request_data = RequestData {
        path_params: Arc::new(path_params.clone()),
        query_params,
        raw_query_params: Arc::new(raw_query.clone()),
        body: body_value,
        raw_body: raw_body.map(bytes::Bytes::from),
        headers: Arc::new(headers.clone()),
        cookies: Arc::new(cookies.clone()),
        method: method_upper.clone(),
        path: path.to_string(),
    };

    let request_body_bytes = raw_body
        .clone()
        .map(|b| b.to_vec())
        .or_else(|| serde_json::to_vec(&body_value).ok())
        .unwrap_or_default();

    let mut builder = Request::builder().method(&method_upper).uri(path);
    for (name, value) in header_map.iter() {
        builder = builder.header(name, value);
    }

    let mut request = builder
        .body(Body::from(request_body_bytes))
        .map_err(|e| PhpException::default(format!("Failed to build request: {e}")))?;
    request.extensions_mut().insert(Arc::new(request_data.clone()));

    let runtime = crate::php::handler::get_runtime()?;
    let dispatch_result = runtime.block_on(async move {
        match execute_with_lifecycle_hooks(request, request_data, handler, lifecycle_hooks).await {
            Ok(response) => {
                let status = response.status();
                let headers = response
                    .headers()
                    .iter()
                    .filter_map(|(name, value)| value.to_str().ok().map(|v| (name.to_string(), v.to_string())))
                    .collect::<HashMap<_, _>>();
                response
                    .into_body()
                    .collect()
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Body read failed: {e}")))
                    .map(|collected| (status, headers, collected.to_bytes()))
            }
            Err(err) => Err(err),
        }
    });

    match dispatch_result {
        Ok((status, headers, bytes)) => Ok(PhpTestResponse {
            status: status.as_u16() as i64,
            body: String::from_utf8_lossy(&bytes).to_string(),
            headers,
        }),
        Err((status, body)) => Ok(PhpTestResponse {
            status: status.as_u16() as i64,
            body,
            headers: HashMap::new(),
        }),
    }
}

/// Parse a query string into a HashMap.
fn parse_query_string(query: Option<&str>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    if let Some(q) = query {
        for (key, value) in url::form_urlencoded::parse(q.as_bytes()) {
            result
                .entry(key.into_owned())
                .or_insert_with(Vec::new)
                .push(value.into_owned());
        }
    }

    result
}

fn body_to_json_and_raw(body: Option<&Zval>) -> PhpResult<(JsonValue, Option<Bytes>)> {
    match body {
        Some(zval) => {
            let raw_body = zval.string().map(|s| Bytes::from(s.to_string()));
            let value = zval_to_json(zval).map_err(|e| PhpException::default(format!("Invalid body payload: {e}")))?;
            Ok((value, raw_body))
        }
        None => Ok((JsonValue::Null, None)),
    }
}

fn raw_query_to_value(raw_query: &HashMap<String, Vec<String>>) -> JsonValue {
    let mut map = serde_json::Map::new();
    for (key, values) in raw_query {
        let json_values: Vec<JsonValue> = values.iter().map(|v| JsonValue::String(v.clone())).collect();
        map.insert(key.clone(), JsonValue::Array(json_values));
    }
    JsonValue::Object(map)
}

pub(crate) fn derive_path_params(actual_path: &str, template: Option<&str>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let Some(template_path) = template else {
        return params;
    };

    let actual_segments: Vec<&str> = actual_path.trim_start_matches('/').split('/').collect();
    let template_segments: Vec<&str> = template_path.trim_start_matches('/').split('/').collect();

    if actual_segments.len() != template_segments.len() {
        return params;
    }

    for (actual, template_seg) in actual_segments.iter().zip(template_segments.iter()) {
        if let Some(name) = template_seg
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .or_else(|| template_seg.strip_prefix(':'))
        {
            params.insert(name.to_string(), (*actual).to_string());
        }
    }

    params
}

/// Derive path parameters from a parameter schema when no explicit template is available.
///
/// Heuristic: map the right-most path segments to the declared path parameters (in schema order).
/// This preserves typical patterns like `/users/{id}/orders/{order_id}` without requiring the template.
fn derive_path_params_from_schema(actual_path: &str, schema: &JsonValue) -> HashMap<String, String> {
    let mut params = HashMap::new();

    let path_param_names: Vec<String> = schema
        .get("properties")
        .and_then(JsonValue::as_object)
        .map(|props| {
            props
                .iter()
                .filter_map(|(name, prop)| match prop.get("source").and_then(JsonValue::as_str) {
                    Some("path") => Some(name.clone()),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default();

    if path_param_names.is_empty() {
        return params;
    }

    let segments: Vec<&str> = actual_path
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    if path_param_names.len() > segments.len() {
        return params;
    }

    for (name, segment) in path_param_names.iter().rev().zip(segments.iter().rev()) {
        params.insert(name.clone(), (*segment).to_string());
    }

    params
}

/// Extract path parameters from a path.
/// This is a simple implementation - real path params come from the router.
fn extract_path_params(_path: &str) -> HashMap<String, String> {
    // Path params are extracted by the router, not here
    HashMap::new()
}

/// Convert a Zval response to a PhpTestResponse.
fn zval_to_test_response(response: &Zval) -> PhpResult<PhpTestResponse> {
    // Handle null response
    if response.is_null() {
        return Ok(PhpTestResponse {
            status: 204,
            body: String::new(),
            headers: HashMap::new(),
        });
    }

    // Try to extract PhpResponse - check if object has our expected methods
    if let Some(obj) = response.object()
        && let Ok(class_name) = obj.get_class_name()
        && class_name.contains("Response")
    {
        // Try to call getStatus method
        if let Ok(status_zval) = obj.try_call_method("getStatus", vec![]) {
            let status = status_zval.long().unwrap_or(200);

            // Try to get body
            let body = if let Ok(body_zval) = obj.try_call_method("getBody", vec![]) {
                body_zval.string().map(|s| s.to_string()).unwrap_or_default()
            } else {
                String::new()
            };

            // Try to get headers
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
    }

    // If it's a string, return as-is
    if let Some(s) = response.string() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        return Ok(PhpTestResponse {
            status: 200,
            body: s.to_string(),
            headers,
        });
    }

    // Try to convert to JSON
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

/// Advanced test client that uses axum-test for full HTTP stack testing.
///
/// This client creates an in-memory HTTP server and sends real HTTP requests,
/// which tests the full middleware stack.
#[php_class]
#[php(name = "Spikard\\Testing\\HttpTestClient")]
#[derive(Default)]
pub struct PhpHttpTestClient {
    // Server is created on-demand for each test
}

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
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<PhpTestResponse> {
        // Reuse native test pipeline so validation/lifecycle paths match Rust behavior.
        execute_test_request(&method, &path, &handler, body, None, headers, None, None, None, None)
    }

    /// Connect to a WebSocket endpoint for testing.
    ///
    /// This calls the WebSocket handler and returns a connection object.
    #[php(name = "websocket")]
    pub fn websocket(
        &self,
        path: String,
        handler: &Zval,
        message_schema: Option<&Zval>,
        response_schema: Option<&Zval>,
    ) -> PhpResult<PhpWebSocketTestConnection> {
        let message_schema = message_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid websocket message schema: {}", e)))?;
        let response_schema = response_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid websocket response schema: {}", e)))?;

        PhpWebSocketTestConnection::connect(path, handler, message_schema, response_schema)
    }

    /// Connect to a Server-Sent Events endpoint for testing.
    ///
    /// This calls the SSE producer and returns a stream object.
    #[php(name = "sse")]
    pub fn sse(&self, path: String, handler: &Zval, event_schema: Option<&Zval>) -> PhpResult<PhpSseStream> {
        let event_schema = event_schema
            .map(zval_to_json)
            .transpose()
            .map_err(|e| PhpException::default(format!("Invalid SSE event schema: {}", e)))?;
        PhpSseStream::connect(path, handler, event_schema)
    }
}

/// WebSocket test connection for PHP.
///
/// This provides methods to send and receive WebSocket messages in tests.
#[php_class]
#[php(name = "Spikard\\Testing\\WebSocketTestConnection")]
pub struct PhpWebSocketTestConnection {
    server: Option<spikard_http::testing::TestServer>,
    connection: Option<spikard_http::testing::WebSocketConnection>,
}

impl PhpWebSocketTestConnection {
    fn connect(
        path: String,
        handler: &Zval,
        message_schema: Option<JsonValue>,
        response_schema: Option<JsonValue>,
    ) -> PhpResult<Self> {
        let runtime = crate::php::handler::get_runtime()?;
        let path_norm = if path.starts_with('/') {
            path.clone()
        } else {
            format!("/{}", path)
        };

        let handler_zval = handler.shallow_clone();
        let state = create_websocket_state(
            &handler_zval,
            Some("php_websocket".to_string()),
            message_schema,
            response_schema,
        )
        .map_err(|e| PhpException::default(format!("Failed to register WebSocket handler: {}", e)))?;

        runtime
            .block_on(async move {
                let router = axum::Router::new().route(
                    &path_norm,
                    get(websocket_handler::<crate::php::websocket::PhpWebSocketHandler>).with_state(state),
                );
                let server = spikard_http::testing::TestServer::new(router)
                    .map_err(|e| PhpException::default(format!("Failed to create test server: {}", e)))?;
                let mut conn = spikard_http::testing::connect_websocket(&server, &path_norm).await;

                Ok(Self {
                    server: Some(server),
                    connection: Some(conn),
                })
            })
            .map_err(|e| PhpException::default(format!("Failed to establish WebSocket: {}", e)))
    }
}

#[php_impl]
impl PhpWebSocketTestConnection {
    /// Send a text message to the WebSocket.
    #[php(name = "sendText")]
    pub fn send_text(&mut self, text: String) -> PhpResult<()> {
        let Some(conn) = self.connection.as_mut() else {
            return Err(PhpException::default("WebSocket connection is closed".to_string()));
        };
        let runtime = crate::php::handler::get_runtime()?;
        runtime.block_on(async { conn.send_text(text).await });
        Ok(())
    }

    /// Send a JSON message to the WebSocket.
    #[php(name = "sendJson")]
    pub fn send_json(&mut self, data: String) -> PhpResult<()> {
        // Validate JSON
        let _: JsonValue =
            serde_json::from_str(&data).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
        self.send_text(data)
    }

    /// Receive a text message from the WebSocket.
    #[php(name = "receiveText")]
    pub fn receive_text(&mut self) -> PhpResult<String> {
        let conn = self
            .connection
            .as_mut()
            .ok_or_else(|| PhpException::default("WebSocket connection is closed".to_string()))?;
        let runtime = crate::php::handler::get_runtime()?;
        let msg = runtime.block_on(async { conn.receive_text().await });
        Ok(msg)
    }

    /// Receive a JSON message from the WebSocket.
    #[php(name = "receiveJson")]
    pub fn receive_json(&mut self) -> PhpResult<ZBox<ZendHashTable>> {
        let text = self.receive_text()?;
        let value: JsonValue =
            serde_json::from_str(&text).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
        json_to_php_table(&value)
    }

    /// Receive raw bytes from the WebSocket.
    #[php(name = "receiveBytes")]
    pub fn receive_bytes(&mut self) -> PhpResult<Vec<u8>> {
        let text = self.receive_text()?;
        Ok(text.into_bytes())
    }

    /// Close the WebSocket connection.
    #[php(name = "close")]
    pub fn close(&mut self) -> PhpResult<()> {
        if let Some(conn) = self.connection.take() {
            let runtime = crate::php::handler::get_runtime()?;
            runtime.block_on(async { conn.close().await });
        }
        self.server = None;
        Ok(())
    }

    /// Check if the connection is closed.
    #[php(name = "isClosed")]
    pub fn is_closed(&self) -> bool {
        self.connection.is_none()
    }
}

/// SSE stream for PHP testing.
///
/// This provides methods to read Server-Sent Events in tests.
#[php_class]
#[php(name = "Spikard\\Testing\\SseStream")]
pub struct PhpSseStream {
    events: Vec<PhpSseEvent>,
    body: String,
}

impl PhpSseStream {
    fn connect(path: String, handler: &Zval, event_schema: Option<JsonValue>) -> PhpResult<Self> {
        let runtime = crate::php::handler::get_runtime()?;
        let path_norm = if path.starts_with('/') {
            path.clone()
        } else {
            format!("/{}", path)
        };
        let handler_zval = handler.shallow_clone();

        runtime
            .block_on(async move {
                let state = create_sse_state(&handler_zval, event_schema)
                    .map_err(|e| PhpException::default(format!("Failed to register SSE producer: {}", e)))?;
                let router = axum::Router::new().route(
                    &path_norm,
                    get(spikard_http::sse::sse_handler::<crate::php::sse::PhpSseEventProducer>).with_state(state),
                );
                let server = spikard_http::testing::TestServer::new(router)
                    .map_err(|e| PhpException::default(format!("Failed to create test server: {}", e)))?;
                let response = server.get(&path_norm).await;
                let snapshot = spikard_http::testing::snapshot_response(response)
                    .await
                    .map_err(|e| PhpException::default(format!("Failed to read SSE response: {}", e)))?;
                let stream = spikard_http::testing::SseStream::from_response(&snapshot)
                    .map_err(|e| PhpException::default(format!("Failed to parse SSE stream: {}", e)))?;
                let events = stream
                    .events()
                    .iter()
                    .map(|evt| PhpSseEvent {
                        data: evt.data.clone(),
                        event_type: None,
                        id: None,
                    })
                    .collect();

                Ok(Self {
                    events,
                    body: stream.body().to_string(),
                })
            })
            .map_err(|e| {
                let msg = format!("Failed to establish SSE stream: {}", e);
                PhpException::default(msg)
            })
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
        self.body.clone()
    }

    /// Get the number of events in the stream.
    #[php(name = "count")]
    pub fn count(&self) -> i64 {
        self.events.len() as i64
    }

    /// Return the captured error (if any) as JSON-like array.
    #[php(name = "json")]
    pub fn json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let payload = serde_json::json!({"events": self.events.iter().map(|e| e.data.clone()).collect::<Vec<_>>()});
        json_to_php_table(&payload)
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
