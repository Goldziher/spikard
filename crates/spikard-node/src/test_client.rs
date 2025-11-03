//! Test client for making HTTP requests to Spikard applications

use crate::response::TestResponse;
use axum::body::Body;
use axum::http::{Method, Request, header};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;
use spikard_http::{Route, RouteMetadata, ServerConfig};
use std::sync::Arc;
use tower::ServiceExt;

/// Test client for making HTTP requests to a Spikard application
#[napi]
pub struct TestClient {
    router: Arc<axum::Router>,
    runtime: Arc<tokio::runtime::Runtime>,
}

#[napi]
impl TestClient {
    /// Create a new test client from routes and handlers
    ///
    /// # Arguments
    /// * `routes_json` - JSON array of route metadata objects
    /// * `handlers_map` - JSON object mapping handler names to handler info
    #[napi(factory)]
    pub fn new(routes_json: String, _handlers_map: serde_json::Value) -> napi::Result<Self> {
        // Parse routes
        let routes_data: Vec<RouteMetadata> = serde_json::from_str(&routes_json)
            .map_err(|e| Error::from_reason(format!("Failed to parse routes: {}", e)))?;

        // Convert to Route objects
        let _routes: Vec<Route> = routes_data
            .into_iter()
            .filter_map(|metadata| Route::from_metadata(metadata).ok())
            .collect();

        // Build Axum router
        // TODO: Implement Node.js handler integration
        // For now, create an empty router - this will be enhanced to call JS handlers
        let _config = ServerConfig::default();
        let axum_router = axum::Router::new();

        // Create runtime
        let runtime = Arc::new(
            tokio::runtime::Runtime::new()
                .map_err(|e| Error::from_reason(format!("Failed to create runtime: {}", e)))?,
        );

        Ok(Self {
            router: Arc::new(axum_router),
            runtime,
        })
    }

    /// Make a GET request
    #[napi]
    pub async fn get(&self, path: String, headers: Option<serde_json::Value>) -> napi::Result<TestResponse> {
        self.request(Method::GET, path, headers, None).await
    }

    /// Make a POST request
    #[napi]
    pub async fn post(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> napi::Result<TestResponse> {
        self.request(Method::POST, path, headers, json).await
    }

    /// Make a PUT request
    #[napi]
    pub async fn put(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> napi::Result<TestResponse> {
        self.request(Method::PUT, path, headers, json).await
    }

    /// Make a DELETE request
    #[napi]
    pub async fn delete(&self, path: String, headers: Option<serde_json::Value>) -> napi::Result<TestResponse> {
        self.request(Method::DELETE, path, headers, None).await
    }

    /// Make a PATCH request
    #[napi]
    pub async fn patch(
        &self,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> napi::Result<TestResponse> {
        self.request(Method::PATCH, path, headers, json).await
    }

    /// Make a HEAD request
    #[napi]
    pub async fn head(&self, path: String, headers: Option<serde_json::Value>) -> napi::Result<TestResponse> {
        self.request(Method::HEAD, path, headers, None).await
    }

    /// Make an OPTIONS request
    #[napi]
    pub async fn options(&self, path: String, headers: Option<serde_json::Value>) -> napi::Result<TestResponse> {
        self.request(Method::OPTIONS, path, headers, None).await
    }

    /// Generic request method
    async fn request(
        &self,
        method: Method,
        path: String,
        headers: Option<serde_json::Value>,
        json: Option<serde_json::Value>,
    ) -> napi::Result<TestResponse> {
        let router = self.router.clone();
        let runtime = self.runtime.clone();

        // Build request
        let mut req_builder = Request::builder().method(method).uri(path);

        // Add headers
        #[allow(clippy::collapsible_if)]
        if let Some(headers_val) = headers {
            if let Some(headers_obj) = headers_val.as_object() {
                for (name, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        req_builder = req_builder.header(name, value_str);
                    }
                }
            }
        }

        // Build body
        let body = if let Some(data) = json {
            req_builder = req_builder.header(header::CONTENT_TYPE, "application/json");
            let json_bytes = serde_json::to_vec(&data)
                .map_err(|e| Error::from_reason(format!("Failed to serialize JSON: {}", e)))?;
            Body::from(json_bytes)
        } else {
            Body::empty()
        };

        let request = req_builder
            .body(body)
            .map_err(|e| Error::from_reason(format!("Failed to build request: {}", e)))?;

        // Execute request
        let response = runtime
            .block_on(async move {
                let router_clone = (*router).clone();
                router_clone.oneshot(request).await
            })
            .map_err(|e| Error::from_reason(format!("Request failed: {}", e)))?;

        // Extract response parts
        let status = response.status().as_u16();
        let headers_map = response.headers().clone();
        let body_bytes = runtime
            .block_on(async move { axum::body::to_bytes(response.into_body(), usize::MAX).await })
            .map_err(|e| Error::from_reason(format!("Failed to read response body: {}", e)))?
            .to_vec();

        // Convert headers to JSON map
        let mut headers_json = serde_json::Map::new();
        for (name, value) in headers_map.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_json.insert(name.to_string(), Value::String(value_str.to_string()));
            }
        }

        Ok(TestResponse::new(status, headers_json, body_bytes))
    }
}
