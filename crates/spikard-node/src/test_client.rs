//! Test client for making HTTP requests to Spikard applications
//!
//! Implements JavaScript handler bridge using napi-rs ThreadsafeFunction
//! for async handler invocation, following patterns from kreuzberg.

use crate::response::TestResponse;
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::http::{HeaderMap, Method, Uri};
use axum::routing::{any, delete, get, patch, post, put};
use axum_test::TestServer;
use http_body_util::BodyExt;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use serde_json::Value;
use spikard_http::{Route, RouteMetadata};
use std::collections::HashMap;
use std::sync::Arc;

/// JavaScript handler wrapper that can be called from Rust async context
#[derive(Clone)]
struct JsHandler {
    /// Thread-safe reference to the JavaScript async function
    /// Takes JSON string, returns Promise<JSON string>
    /// Full signature: ThreadsafeFunction<InputType, OutputType, ReturnType, ErrorType, IsWeakRef>
    handler_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
}

// SAFETY: ThreadsafeFunction from napi-rs is designed to be Send + Sync.
// - ThreadsafeFunction uses internal synchronization to safely call JavaScript from any thread
// - NAPI-RS guarantees thread-safe execution by marshaling through Node.js event loop
// - The JavaScript function reference is managed by Node.js runtime
// - Arc provides shared ownership with atomic reference counting
unsafe impl Send for JsHandler {}
unsafe impl Sync for JsHandler {}

impl JsHandler {
    /// Create a new handler from a JavaScript function
    fn new(_env: Env, js_fn: Function<String, Promise<String>>) -> Result<Self> {
        // Build ThreadsafeFunction with callback to wrap arguments
        let tsfn = js_fn.build_threadsafe_function().build_callback(|ctx| {
            // Wrap value in vec so JS receives it as separate argument
            Ok(vec![ctx.value])
        })?;

        Ok(Self {
            handler_fn: Arc::new(tsfn),
        })
    }

    /// Call the JavaScript handler with request parameters
    ///
    /// Uses double await pattern:
    /// - First await: enqueues callback on Node.js event loop
    /// - Second await: waits for JavaScript Promise to resolve
    async fn call(&self, params_json: String) -> Result<Value> {
        // Call async JavaScript function (double await pattern from kreuzberg)
        let result_json = self
            .handler_fn
            .call_async(params_json)
            .await
            .map_err(|e| Error::from_reason(format!("Handler call failed: {}", e)))?
            .await
            .map_err(|e| Error::from_reason(format!("Handler promise failed: {}", e)))?;

        // Parse JSON response
        serde_json::from_str(&result_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse handler response: {}", e)))
    }
}

/// Test client for making HTTP requests to a Spikard application
#[napi]
pub struct TestClient {
    server: Arc<TestServer>,
}

#[napi]
impl TestClient {
    /// Create a new test client from routes and handlers
    ///
    /// # Arguments
    /// * `routes_json` - JSON array of route metadata objects
    /// * `handlers_map` - JavaScript object mapping handler names to handler functions
    #[napi(factory)]
    pub fn new(env: Env, routes_json: String, handlers_map: Object) -> Result<Self> {
        // Parse routes
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        // Extract handler functions from the JavaScript object
        let mut handlers: HashMap<String, JsHandler> = HashMap::new();

        for route_meta in &routes_data {
            let handler_name = &route_meta.handler_name;

            // Get the JavaScript function from the handlers map
            // Type it as Function<InputType, OutputType> to enable proper ThreadsafeFunction creation
            let js_fn: Function<String, Promise<String>> = handlers_map
                .get_named_property(handler_name)
                .map_err(|e| Error::from_reason(format!("Failed to get handler '{}': {}", handler_name, e)))?;

            // Create a JsHandler wrapper
            let js_handler = JsHandler::new(env, js_fn)?;
            handlers.insert(handler_name.clone(), js_handler);
        }

        // Convert to Route objects
        let routes: Vec<Route> = routes_data
            .into_iter()
            .filter_map(|metadata| Route::from_metadata(metadata).ok())
            .collect();

        // Build Axum router with JavaScript handlers
        let mut axum_router = AxumRouter::new();

        for route in routes {
            let handler = handlers
                .get(&route.handler_name)
                .ok_or_else(|| Error::from_reason(format!("Handler '{}' not found", route.handler_name)))?;

            let handler_clone = handler.clone();
            let path = route.path.clone();
            let has_path_params = path.contains('{');

            // Create handler function that calls JavaScript handler
            let route_handler = move |path_params: Option<Path<HashMap<String, String>>>,
                                      uri: Uri,
                                      _headers: HeaderMap,
                                      request: Request| {
                let handler = handler_clone.clone();
                async move {
                    // Build parameters object
                    let mut params = serde_json::Map::new();

                    // Add path parameters
                    if let Some(path_params) = path_params {
                        for (key, value) in path_params.0 {
                            params.insert(key, serde_json::Value::String(value));
                        }
                    }

                    // Add query parameters
                    if let Some(query) = uri.query() {
                        for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
                            params.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                        }
                    }

                    // Try to parse body if present
                    let (_parts, body) = request.into_parts();
                    if let Ok(bytes) = body.collect().await {
                        let body_bytes = bytes.to_bytes();
                        #[allow(clippy::collapsible_if)]
                        if !body_bytes.is_empty() {
                            if let Ok(body_json) = serde_json::from_slice::<Value>(&body_bytes) {
                                params.insert("body".to_string(), body_json);
                            }
                        }
                    }

                    let params_json = serde_json::to_string(&params).unwrap_or_default();

                    // Call the JavaScript handler (using double await pattern)
                    match handler.call(params_json).await {
                        Ok(result) => {
                            // Convert result to JSON response
                            let json_bytes = serde_json::to_vec(&result).unwrap_or_default();
                            axum::response::Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(Body::from(json_bytes))
                                .unwrap()
                        }
                        Err(e) => {
                            let error = serde_json::json!({
                                "error": "Handler failed",
                                "message": e.to_string()
                            });
                            let json_bytes = serde_json::to_vec(&error).unwrap_or_default();
                            axum::response::Response::builder()
                                .status(500)
                                .header("content-type", "application/json")
                                .body(Body::from(json_bytes))
                                .unwrap()
                        }
                    }
                }
            };

            // Strip type hints from path for Axum compatibility
            let axum_path = spikard_http::type_hints::strip_type_hints(&path);

            // Register route based on HTTP method
            axum_router = match route.method.as_str() {
                "GET" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            get(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            get(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "POST" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            post(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            post(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "PUT" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            put(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            put(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "DELETE" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            delete(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            delete(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "PATCH" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            patch(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      req: Request| async move {
                                    handler(Some(path), uri, headers, req).await
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            patch(move |uri: Uri, headers: HeaderMap, req: Request| async move {
                                handler(None, uri, headers, req).await
                            }),
                        )
                    }
                }
                "HEAD" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      method: Method,
                                      req: Request| async move {
                                    if method == Method::HEAD {
                                        handler(Some(path), uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |uri: Uri, headers: HeaderMap, method: Method, req: Request| async move {
                                    if method == Method::HEAD {
                                        handler(None, uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    }
                }
                "OPTIONS" => {
                    if has_path_params {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |path: Path<HashMap<String, String>>,
                                      uri: Uri,
                                      headers: HeaderMap,
                                      method: Method,
                                      req: Request| async move {
                                    if method == Method::OPTIONS {
                                        handler(Some(path), uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    } else {
                        let handler = route_handler;
                        axum_router.route(
                            &axum_path,
                            any(
                                move |uri: Uri, headers: HeaderMap, method: Method, req: Request| async move {
                                    if method == Method::OPTIONS {
                                        handler(None, uri, headers, req).await
                                    } else {
                                        axum::response::Response::builder()
                                            .status(405)
                                            .body(Body::empty())
                                            .unwrap()
                                    }
                                },
                            ),
                        )
                    }
                }
                _ => {
                    return Err(Error::from_reason(format!(
                        "Unsupported HTTP method: {}",
                        route.method.as_str()
                    )));
                }
            };
        }

        // Create test server from the router
        let server = TestServer::new(axum_router.into_make_service())
            .map_err(|e| Error::from_reason(format!("Failed to create test server: {}", e)))?;

        Ok(Self {
            server: Arc::new(server),
        })
    }

    /// Make a GET request
    #[napi]
    pub async fn get(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("GET", path, headers, None).await
    }

    /// Make a POST request
    #[napi]
    pub async fn post(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("POST", path, headers, json).await
    }

    /// Make a PUT request
    #[napi]
    pub async fn put(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("PUT", path, headers, json).await
    }

    /// Make a DELETE request
    #[napi]
    pub async fn delete(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("DELETE", path, headers, None).await
    }

    /// Make a PATCH request
    #[napi]
    pub async fn patch(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        self.request("PATCH", path, headers, json).await
    }

    /// Make a HEAD request
    #[napi]
    pub async fn head(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("HEAD", path, headers, None).await
    }

    /// Make an OPTIONS request
    #[napi]
    pub async fn options(&self, path: String, headers: Option<serde_json::Value>) -> Result<TestResponse> {
        self.request("OPTIONS", path, headers, None).await
    }

    /// Generic request method using axum-test
    async fn request(
        &self,
        method: &str,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> Result<TestResponse> {
        // Build request using axum-test
        let mut request = match method {
            "GET" => self.server.get(&path),
            "POST" => {
                let mut req = self.server.post(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "PUT" => {
                let mut req = self.server.put(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "DELETE" => self.server.delete(&path),
            "PATCH" => {
                let mut req = self.server.patch(&path);
                if let Some(json_data) = json {
                    req = req.json(&json_data);
                }
                req
            }
            "HEAD" | "OPTIONS" => {
                // Use method() for HEAD and OPTIONS
                self.server.method(
                    axum::http::Method::from_bytes(method.as_bytes())
                        .map_err(|e| Error::from_reason(format!("Invalid method: {}", e)))?,
                    &path,
                )
            }
            _ => return Err(Error::from_reason(format!("Unsupported method: {}", method))),
        };

        // Add headers if provided
        #[allow(clippy::collapsible_if)]
        if let Some(headers_val) = headers {
            if let Some(headers_obj) = headers_val.as_object() {
                for (name, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        request = request.add_header(
                            axum::http::HeaderName::from_bytes(name.as_bytes())
                                .map_err(|e| Error::from_reason(format!("Invalid header name: {}", e)))?,
                            axum::http::HeaderValue::from_str(value_str)
                                .map_err(|e| Error::from_reason(format!("Invalid header value: {}", e)))?,
                        );
                    }
                }
            }
        }

        // Execute request
        let response = request.await;

        // Extract response parts
        let status = response.status_code().as_u16();
        let headers_map = response.headers();

        // Convert headers to JSON map
        let mut headers_json = serde_json::Map::new();
        for (name, value) in headers_map.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_json.insert(name.to_string(), Value::String(value_str.to_string()));
            }
        }

        // Get body bytes
        let body_bytes = response.into_bytes().to_vec();

        Ok(TestResponse::new(status, headers_json, body_bytes))
    }
}
