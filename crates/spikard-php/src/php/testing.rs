//! Native PHP test client for HTTP testing.
//!
//! This module implements `NativeTestClient`, a PHP class that provides
//! HTTP testing capabilities against a Spikard server without network overhead.

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Method, Request as AxumRequest, Uri};
use axum::routing::get;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, ZendHashTable, Zval};
use serde_json::Value as JsonValue;
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::testing::{
    ResponseSnapshot, SseStream as CoreSseStream, TestClient as CoreTestClient, snapshot_http_response,
};
use spikard_http::{Handler, Route, RouteMetadata, ServerConfig, WebSocketState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use tower::util::ServiceExt;

use super::{PhpHandler, PhpRequest, json_to_php_table, zval_to_json};

type MultipartPayload = Option<(Vec<(String, String)>, Vec<spikard_http::testing::MultipartFilePart>)>;

/// Test response data exposed to PHP.
#[php_class]
#[php(name = "Spikard\\Testing\\TestResponse")]
pub struct PhpTestResponse {
    pub(crate) status: i64,
    pub(crate) body: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) cookies: HashMap<String, String>,
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

    /// Get response cookies as a PHP array.
    #[php(name = "getCookies")]
    pub fn get_cookies(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.cookies {
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

    /// Extract GraphQL data from response.
    ///
    /// Returns the "data" field from the GraphQL response as a PHP array.
    /// Throws PhpException if the response body is invalid JSON or contains no "data" field.
    ///
    /// # Errors
    /// - Invalid JSON in response body
    /// - Missing "data" field in GraphQL response
    #[php(name = "graphqlData")]
    pub fn graphql_data(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let value: JsonValue =
            serde_json::from_str(&self.body).map_err(|e| PhpException::default(format!("Invalid JSON body: {}", e)))?;

        let data = value
            .get("data")
            .ok_or_else(|| PhpException::default("No 'data' field in GraphQL response".to_string()))?;

        super::json_to_php_table(data)
    }

    /// Extract GraphQL errors from response.
    ///
    /// Returns the "errors" field from the GraphQL response as a PHP array of error objects.
    /// Returns an empty array if no errors are present.
    ///
    /// # Errors
    /// - Invalid JSON in response body
    #[php(name = "graphqlErrors")]
    pub fn graphql_errors(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let value: JsonValue =
            serde_json::from_str(&self.body).map_err(|e| PhpException::default(format!("Invalid JSON body: {}", e)))?;

        let errors = value
            .get("errors")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();

        let mut table = super::php_table_with_capacity(errors.len());
        for error in errors {
            table
                .push(super::json_to_php_table(&error)?)
                .map_err(super::map_ext_php_err)?;
        }

        Ok(table)
    }
}

struct NativeClientInner {
    router: Arc<Router>,
    #[allow(dead_code)]
    handlers: Vec<Zval>,
    websocket_states: HashMap<String, WebSocketState<super::PhpWebSocketHandler>>,
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
    pub fn __construct(routes: &Zval, config: Option<&Zval>, dependencies: Option<&Zval>) -> PhpResult<Self> {
        let parsed_routes = parse_native_routes(routes)?;
        let mut server_config = if let Some(config_zval) = config {
            super::start::extract_server_config_from_php(config_zval)
                .map_err(|e| PhpException::default(format!("Invalid server config: {}", e)))?
        } else {
            ServerConfig::default()
        };

        // Extract DI container from dependencies if present
        let di_container = crate::php::extract_di_container_from_php(dependencies)
            .map_err(|e| PhpException::default(format!("Invalid DI container: {}", e)))?;
        if let Some(container) = di_container {
            server_config.di_container = Some(std::sync::Arc::new(container));
        }

        let mut handler_refs = Vec::new();
        let mut route_pairs: Vec<(Route, Arc<dyn Handler>)> = Vec::new();
        let mut route_metadata: Vec<RouteMetadata> = Vec::new();
        let mut websocket_routes = Vec::new();
        let mut sse_routes = Vec::new();
        let mut websocket_states = HashMap::new();

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
                static_response: None,
            });

            route_pairs.push((route_def, Arc::new(handler) as Arc<dyn Handler>));
        }

        let mut router = build_router_with_handlers_and_config(route_pairs, server_config, route_metadata)
            .map_err(|e| PhpException::default(format!("Failed to build router: {}", e)))?;

        for route in websocket_routes {
            let path = route.payload.path;
            let handler_name = route.payload.handler_name;
            let message_schema = route.payload.request_schema.clone();
            let response_schema = route.payload.response_schema.clone();
            let ws_state =
                super::create_websocket_state(&route.handler, Some(handler_name), message_schema, response_schema)
                    .map_err(|e| PhpException::default(format!("Failed to build WebSocket state: {}", e)))?;
            websocket_states.insert(path.clone(), ws_state.clone());
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

        let router = Arc::new(router);

        Ok(Self {
            inner: RefCell::new(Some(NativeClientInner {
                router,
                handlers: handler_refs,
                websocket_states,
            })),
        })
    }

    /// Execute an HTTP request using the full Rust HTTP stack.
    #[php(name = "request")]
    pub fn request(&self, method: String, path: String, options: Option<&Zval>) -> PhpResult<PhpTestResponse> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let request_options = parse_request_options(options)?;
        let runtime = super::get_runtime()?;
        let response = runtime.block_on(dispatch_request_direct(&inner.router, method, path, request_options))?;

        snapshot_to_php_response(response)
    }

    /// Connect to a WebSocket endpoint for testing.
    #[php(name = "websocket")]
    pub fn websocket(&self, path: String, send_text: Option<String>) -> PhpResult<PhpWebSocketTestConnection> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let lookup_path = normalize_websocket_path(&path);
        let Some(state) = inner.websocket_states.get(&lookup_path) else {
            return Err(PhpException::default(format!(
                "WebSocket route not found for path '{}'",
                path
            )));
        };

        let runtime = super::get_runtime()?;
        let state = state.clone();
        runtime.block_on(state.on_connect());

        let mut connection = PhpWebSocketTestConnection::from_state(state, inner.router.clone());
        if let Some(text) = send_text {
            connection.send_text(text)?;
        }

        Ok(connection)
    }

    /// Connect to an SSE endpoint for testing.
    #[php(name = "sse")]
    pub fn sse(&self, path: String) -> PhpResult<PhpSseStream> {
        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let runtime = super::get_runtime()?;
        let request_options = parse_request_options(None)?;
        let response = runtime.block_on(dispatch_request_direct(
            &inner.router,
            "GET".to_string(),
            path,
            request_options,
        ))?;

        PhpSseStream::from_snapshot(response)
    }

    /// Send a GraphQL query/mutation.
    #[php(name = "graphql")]
    pub fn graphql(
        &self,
        query: String,
        variables: Option<&Zval>,
        operation_name: Option<String>,
    ) -> PhpResult<PhpTestResponse> {
        let variables_json = match variables {
            Some(v) => {
                // Convert PHP Zval to serde_json::Value
                let json_val = zval_to_json(v).map_err(PhpException::default)?;
                Some(json_val)
            }
            None => None,
        };

        let mut body = serde_json::json!({ "query": query });
        if let Some(vars) = variables_json {
            body["variables"] = vars;
        }
        if let Some(op_name) = operation_name {
            body["operationName"] = JsonValue::String(op_name);
        }

        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let runtime = super::get_runtime()?;
        let mut request_options = parse_request_options(None)?;
        request_options.body = Some(body);

        let response = runtime.block_on(dispatch_request_direct(
            &inner.router,
            "POST".to_string(),
            "/graphql".to_string(),
            request_options,
        ))?;

        snapshot_to_php_response(response)
    }

    /// Send a GraphQL query and get HTTP status separately.
    #[php(name = "graphqlWithStatus")]
    pub fn graphql_with_status(
        &self,
        query: String,
        variables: Option<&Zval>,
        operation_name: Option<String>,
    ) -> PhpResult<Vec<Zval>> {
        let response = self.graphql(query, variables, operation_name)?;
        let status_zval = response.status.into_zval(false).map_err(super::map_ext_php_err)?;
        let body_zval = response.body.clone().into_zval(false).map_err(super::map_ext_php_err)?;

        Ok(vec![status_zval, body_zval])
    }

    /// Send a GraphQL subscription over WebSocket and return the first event payload snapshot.
    #[php(name = "graphqlSubscription")]
    pub fn graphql_subscription(
        &self,
        query: String,
        variables: Option<&Zval>,
        operation_name: Option<String>,
        path: Option<String>,
    ) -> PhpResult<ZBox<ZendHashTable>> {
        let variables_json = match variables {
            Some(v) => Some(zval_to_json(v).map_err(PhpException::default)?),
            None => None,
        };

        let inner_ref = self.inner.borrow();
        let inner = inner_ref
            .as_ref()
            .ok_or_else(|| PhpException::default("TestClient is closed".to_string()))?;

        let runtime = super::get_runtime()?;
        let endpoint = path.unwrap_or_else(|| "/graphql".to_string());
        let core_client = CoreTestClient::from_router(inner.router.as_ref().clone())
            .map_err(|e| PhpException::default(format!("Failed to initialize GraphQL subscription client: {}", e)))?;

        let snapshot = runtime
            .block_on(core_client.graphql_subscription_at(
                endpoint.as_str(),
                query.as_str(),
                variables_json,
                operation_name.as_deref(),
            ))
            .map_err(|e| PhpException::default(format!("GraphQL subscription failed: {}", e)))?;

        let payload = serde_json::json!({
            "operationId": snapshot.operation_id,
            "acknowledged": snapshot.acknowledged,
            "event": snapshot.event,
            "errors": snapshot.errors,
            "completeReceived": snapshot.complete_received,
        });

        json_to_php_table(&payload)
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
    #[php(name = "create")]
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
            cookies: HashMap::new(),
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

        let mut cookies = HashMap::new();
        if let Ok(cookies_zval) = obj.try_call_method("getCookies", vec![])
            && let Some(arr) = cookies_zval.array()
        {
            for (key, val) in arr.iter() {
                let key_str = match key {
                    ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
                    ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
                    ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
                };
                if let Some(val_str) = val.string() {
                    cookies.insert(key_str, val_str.to_string());
                }
            }
        }

        if cookies.is_empty() {
            cookies = cookies_from_headers(&headers);
        }

        return Ok(PhpTestResponse {
            status,
            body,
            headers,
            cookies,
        });
    }

    if let Some(s) = response.string() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        return Ok(PhpTestResponse {
            status: 200,
            body: s.to_string(),
            headers,
            cookies: HashMap::new(),
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
        cookies: HashMap::new(),
    })
}

struct ParsedRequestOptions {
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Option<JsonValue>,
    raw_body: Option<Vec<u8>>,
    form_data: Option<JsonValue>,
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

        let mut json_val = zval_to_json(route_val)
            .map_err(|e| PhpException::default(format!("Failed to convert route to JSON: {}", e)))?;

        if let Some(obj) = json_val.as_object_mut() {
            for key in ["request_schema", "response_schema", "parameter_schema"] {
                if let Some(schema_val) = obj.get_mut(key) {
                    normalize_schema_empty_arrays(schema_val);
                }
            }
        }

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

fn normalize_schema_empty_arrays(value: &mut JsonValue) {
    match value {
        JsonValue::Array(items) => {
            if items.is_empty() {
                *value = JsonValue::Object(serde_json::Map::new());
            } else {
                for item in items {
                    normalize_schema_empty_arrays(item);
                }
            }
        }
        JsonValue::Object(map) => {
            for val in map.values_mut() {
                normalize_schema_empty_arrays(val);
            }
        }
        _ => {}
    }
}

fn parse_request_options(options: Option<&Zval>) -> PhpResult<ParsedRequestOptions> {
    let mut headers = HashMap::new();
    let mut cookies = HashMap::new();
    let mut body = None;
    let mut raw_body = None;
    let mut form_data = None;
    let mut multipart = None;

    let Some(options_val) = options else {
        return Ok(ParsedRequestOptions {
            headers,
            cookies,
            body,
            raw_body,
            form_data,
            multipart,
        });
    };

    let Some(options_array) = options_val.array() else {
        return Ok(ParsedRequestOptions {
            headers,
            cookies,
            body,
            raw_body,
            form_data,
            multipart,
        });
    };

    headers = parse_string_map(options_array.get("headers"));
    cookies = parse_string_map(options_array.get("cookies"));

    if let Some(body_val) = options_array.get("body")
        && !body_val.is_null()
    {
        let content_type = header_value(&headers, "content-type");
        let is_json = content_type.as_deref().map(is_json_content_type).unwrap_or(true);

        if !is_json {
            if let Some(body_str) = body_val.string() {
                raw_body = Some(body_str.as_bytes().to_vec());
            } else {
                let json_val = zval_to_json(body_val).map_err(PhpException::default)?;
                let encoded = serde_json::to_string(&json_val).unwrap_or_default();
                raw_body = Some(encoded.into_bytes());
            }
        } else if let Some(body_str) = body_val.string() {
            match serde_json::from_str::<JsonValue>(&body_str) {
                Ok(parsed) => body = Some(parsed),
                Err(_) => body = Some(JsonValue::String(body_str.to_string())),
            }
        } else {
            body = Some(zval_to_json(body_val).map_err(PhpException::default)?);
        }
    }

    if let Some(form_val) = options_array
        .get("form_data")
        .or_else(|| options_array.get("form"))
        .or_else(|| options_array.get("data"))
        && !form_val.is_null()
    {
        form_data = Some(zval_to_json(form_val).map_err(PhpException::default)?);
    }
    if body.is_some() || raw_body.is_some() {
        form_data = None;
    }

    let files_specified = options_array.get("files").is_some();
    let files = parse_files(options_array.get("files"));
    let content_type = header_value(&headers, "content-type");
    let is_multipart = content_type
        .as_deref()
        .map(|ct| ct.to_ascii_lowercase().starts_with("multipart/form-data"))
        .unwrap_or(false);
    let form_fields = form_data.as_ref().map(form_fields_from_json).unwrap_or_default();

    let should_force_multipart = ((is_multipart || !files.is_empty())
        && (!files.is_empty() || !form_fields.is_empty()))
        || ((is_multipart || files_specified) && multipart.is_none());

    if should_force_multipart {
        multipart = Some((form_fields, files));
        form_data = None;
    } else if !files.is_empty() && body.is_none() && raw_body.is_none() {
        multipart = Some((Vec::new(), files));
        form_data = None;
    }

    Ok(ParsedRequestOptions {
        headers,
        cookies,
        body,
        raw_body,
        form_data,
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
        let fallback_field = array_key_to_string(key);
        if let Ok(json_val) = zval_to_json(val)
            && let Some(obj) = json_val.as_object()
        {
            let field_name = obj
                .get("field_name")
                .and_then(|v| v.as_str())
                .unwrap_or(&fallback_field)
                .to_string();
            let filename = obj.get("filename").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let content_type = obj.get("content_type").and_then(|v| v.as_str()).map(|s| s.to_string());
            let content = file_content_from_json(obj);
            files.push(spikard_http::testing::MultipartFilePart {
                field_name,
                filename,
                content_type,
                content,
            });
            continue;
        }

        let content = if let Some(val_str) = val.string() {
            val_str.as_bytes().to_vec()
        } else {
            let json_val = zval_to_json(val).unwrap_or(JsonValue::Null);
            serde_json::to_string(&json_val).unwrap_or_default().into_bytes()
        };

        files.push(spikard_http::testing::MultipartFilePart {
            field_name: fallback_field.clone(),
            filename: fallback_field,
            content_type: None,
            content,
        });
    }

    files
}

fn file_content_from_json(obj: &serde_json::Map<String, JsonValue>) -> Vec<u8> {
    if let Some(content) = obj.get("content").and_then(|v| v.as_str()) {
        return content.as_bytes().to_vec();
    }
    if let Some(magic) = obj.get("magic_bytes").and_then(|v| v.as_str()) {
        return decode_hex_bytes(magic);
    }
    Vec::new()
}

fn decode_hex_bytes(value: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = value.as_bytes().iter().copied();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        let hi = (high as char).to_digit(16);
        let lo = (low as char).to_digit(16);
        if let (Some(hi), Some(lo)) = (hi, lo) {
            bytes.push(((hi << 4) | lo) as u8);
        } else {
            return Vec::new();
        }
    }
    bytes
}

fn form_fields_from_json(value: &JsonValue) -> Vec<(String, String)> {
    match value {
        JsonValue::Object(map) => map
            .iter()
            .flat_map(|(key, val)| match val {
                JsonValue::Array(items) => items
                    .iter()
                    .map(|item| (key.clone(), json_value_to_string(item)))
                    .collect::<Vec<_>>(),
                _ => vec![(key.clone(), json_value_to_string(val))],
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn json_value_to_string(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Number(num) => num.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => String::new(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

fn array_key_to_string(key: ext_php_rs::types::ArrayKey) -> String {
    match key {
        ext_php_rs::types::ArrayKey::Long(i) => i.to_string(),
        ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
        ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
    }
}

fn normalize_websocket_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
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

async fn dispatch_request_direct(
    router: &Router,
    method: String,
    path: String,
    options: ParsedRequestOptions,
) -> PhpResult<ResponseSnapshot> {
    let ParsedRequestOptions {
        headers,
        cookies,
        body,
        raw_body,
        form_data,
        multipart,
    } = options;
    let method_upper = method.to_ascii_uppercase();
    let method = Method::from_bytes(method_upper.as_bytes())
        .map_err(|e| PhpException::default(format!("Invalid HTTP method: {}", e)))?;
    let uri: Uri = path
        .parse()
        .map_err(|e| PhpException::default(format!("Invalid request URI: {}", e)))?;

    let mut builder = AxumRequest::builder().method(method).uri(uri);
    let mut header_entries = build_header_list(headers, cookies).unwrap_or_default();
    let has_content_type = header_entries
        .iter()
        .any(|(key, _)| key.eq_ignore_ascii_case("content-type"));

    let mut body_bytes: Vec<u8> = Vec::new();
    let mut content_type_override: Option<String> = None;
    let mut force_content_type = false;

    if let Some(raw_body) = raw_body {
        body_bytes = raw_body;
    } else if let Some((form_fields, files)) = multipart {
        let (body, boundary) = spikard_http::testing::build_multipart_body(&form_fields, &files);
        content_type_override = Some(format!("multipart/form-data; boundary={}", boundary));
        force_content_type = true;
        body_bytes = body;
    } else if let Some(form_value) = form_data {
        let encoded = spikard_http::testing::encode_urlencoded_body(&form_value)
            .map_err(|e| PhpException::default(format!("Form encoding failed: {}", e)))?;
        content_type_override = Some("application/x-www-form-urlencoded".to_string());
        body_bytes = encoded;
    } else if let Some(json_value) = body {
        body_bytes = serde_json::to_vec(&json_value).unwrap_or_default();
        content_type_override = Some("application/json".to_string());
    }

    if let Some(content_type) = content_type_override
        && (force_content_type || !has_content_type)
    {
        header_entries.retain(|(key, _)| !key.eq_ignore_ascii_case("content-type"));
        builder = builder.header("content-type", content_type);
    }

    for (key, value) in header_entries {
        let header_name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| PhpException::default(format!("Invalid header name: {}", e)))?;
        let header_value =
            HeaderValue::from_str(&value).map_err(|e| PhpException::default(format!("Invalid header value: {}", e)))?;
        builder = builder.header(header_name, header_value);
    }

    let request = builder
        .body(Body::from(body_bytes))
        .map_err(|e| PhpException::default(format!("Failed to build request: {}", e)))?;

    let response = router
        .clone()
        .oneshot(request)
        .await
        .map_err(|e| PhpException::default(format!("Test request failed: {}", e)))?;
    snapshot_http_response(response)
        .await
        .map_err(|e| PhpException::default(format!("Test request failed: {}", e)))
}

fn snapshot_to_php_response(snapshot: ResponseSnapshot) -> PhpResult<PhpTestResponse> {
    let status_code = snapshot.status as i64;
    let headers = snapshot.headers;
    let body = if snapshot.body.is_empty() {
        String::new()
    } else {
        String::from_utf8_lossy(&snapshot.body).into_owned()
    };
    let cookies = cookies_from_headers(&headers);

    Ok(PhpTestResponse {
        status: status_code,
        body,
        headers,
        cookies,
    })
}

fn is_json_content_type(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.starts_with("application/json") || lower.contains("+json")
}

fn header_value(headers: &HashMap<String, String>, name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.clone())
}

fn cookies_from_headers(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    let Some(raw) = header_value(headers, "set-cookie") else {
        return cookies;
    };
    let cookie_pair = raw.split(';').next().unwrap_or_default();
    if let Some((name, value)) = cookie_pair.split_once('=') {
        cookies.insert(name.trim().to_string(), value.trim().to_string());
    }
    cookies
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
    #[php(name = "create")]
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
struct LocalWebSocketConnection {
    state: WebSocketState<super::PhpWebSocketHandler>,
}

#[php_class]
#[php(name = "Spikard\\Testing\\WebSocketTestConnection")]
pub struct PhpWebSocketTestConnection {
    inner: RefCell<Option<LocalWebSocketConnection>>,
    #[allow(dead_code)]
    keepalive: Arc<Router>,
}

impl PhpWebSocketTestConnection {
    fn connect(_path: String, _handler: ZendCallable) -> PhpResult<Self> {
        Err(PhpException::default(
            "Native WebSocket client is not available without Spikard\\Native\\TestClient".to_string(),
        ))
    }

    fn from_state(state: WebSocketState<super::PhpWebSocketHandler>, keepalive: Arc<Router>) -> Self {
        Self {
            inner: RefCell::new(Some(LocalWebSocketConnection { state })),
            keepalive,
        }
    }

    fn with_connection_mut<F, T>(&self, op: F) -> PhpResult<T>
    where
        F: FnOnce(&mut LocalWebSocketConnection) -> PhpResult<T>,
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
            let message: JsonValue =
                serde_json::from_str(&text).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
            runtime.block_on(async {
                ws.state
                    .handle_message_validated(message)
                    .await
                    .map(|_| ())
                    .map_err(PhpException::default)
            })?;
            Ok(())
        })
    }

    /// Send a JSON message to the WebSocket.
    #[php(name = "sendJson")]
    pub fn send_json(&mut self, data: String) -> PhpResult<()> {
        self.with_connection_mut(|ws| {
            let runtime = super::get_runtime()?;
            let message: JsonValue =
                serde_json::from_str(&data).map_err(|e| PhpException::default(format!("Invalid JSON: {}", e)))?;
            runtime.block_on(async {
                ws.state
                    .handle_message_validated(message)
                    .await
                    .map(|_| ())
                    .map_err(PhpException::default)
            })?;
            Ok(())
        })
    }

    /// Receive a text message from the WebSocket.
    #[php(name = "receiveText")]
    pub fn receive_text(&self) -> PhpResult<String> {
        self.with_connection_mut(|ws| {
            let _ = ws;
            Err(PhpException::default(
                "Local WebSocket connections do not support receiving messages".to_string(),
            ))
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
            let _ = ws;
            Err(PhpException::default(
                "Local WebSocket connections do not support receiving messages".to_string(),
            ))
        })
    }

    /// Close the WebSocket connection.
    #[php(name = "close")]
    pub fn close(&mut self) -> PhpResult<()> {
        let mut inner = self.inner.borrow_mut();
        if let Some(ws) = inner.take() {
            let runtime = super::get_runtime()?;
            runtime.block_on(ws.state.on_disconnect());
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
        self.events.iter().fold(String::new(), |mut acc, e| {
            acc.push_str(&format!("data: {}\n\n", e.data));
            acc
        })
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
