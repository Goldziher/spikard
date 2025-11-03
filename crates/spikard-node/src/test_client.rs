//! Test client for making HTTP requests to Spikard applications
//!
//! This is a simplified version that echoes parameters back.
//! TODO: Implement proper JavaScript handler bridge with promises.

use crate::response::TestResponse;
use axum::Router as AxumRouter;
use axum::body::Body;
use axum::extract::{Path, Request};
use axum::http::{HeaderMap, Method, Uri};
use axum::routing::{any, delete, get, patch, post, put};
use axum_test::TestServer;
use http_body_util::BodyExt;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;
use spikard_http::{Route, RouteMetadata};
use std::collections::HashMap;
use std::sync::Arc;

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
    pub fn new(_env: Env, routes_json: String, _handlers_map: Object) -> Result<Self> {
        // Parse routes
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        // Convert to Route objects
        let routes: Vec<Route> = routes_data
            .into_iter()
            .filter_map(|metadata| Route::from_metadata(metadata).ok())
            .collect();

        // Build Axum router
        // For now, routes just echo back parameters
        let mut axum_router = AxumRouter::new();

        for route in routes {
            let path_clone = route.path.clone();
            let has_path_params = path_clone.contains('{');

            // Create handler that echoes parameters back
            let echo_handler = move |path_params: Option<Path<HashMap<String, String>>>,
                                     uri: Uri,
                                     _headers: HeaderMap,
                                     request: Request| async move {
                // Build response object with all parameters
                let mut result = serde_json::Map::new();

                // Add path parameters
                if let Some(path_params) = path_params {
                    for (key, value) in path_params.0 {
                        result.insert(key, serde_json::Value::String(value));
                    }
                }

                // Add query parameters
                if let Some(query) = uri.query() {
                    for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
                        result.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                    }
                }

                // Try to parse body if present
                let (_parts, body) = request.into_parts();
                if let Ok(bytes) = body.collect().await {
                    let body_bytes = bytes.to_bytes();
                    #[allow(clippy::collapsible_if)]
                    if !body_bytes.is_empty() {
                        if let Ok(body_json) = serde_json::from_slice::<Value>(&body_bytes) {
                            result.insert("body".to_string(), body_json);
                        }
                    }
                }

                let response_json = serde_json::Value::Object(result);
                let json_bytes = serde_json::to_vec(&response_json).unwrap_or_default();

                axum::response::Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::from(json_bytes))
                    .unwrap()
            };

            // Strip type hints from path for Axum compatibility
            let axum_path = spikard_http::type_hints::strip_type_hints(&path_clone);

            // Register route based on HTTP method
            axum_router = match route.method.as_str() {
                "GET" => {
                    if has_path_params {
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
                        let handler = echo_handler;
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
