//! Test builder utilities for fluent test construction
//!
//! This module provides builder APIs for constructing mock handlers and test requests,
//! eliminating boilerplate and improving test readability. All builders follow a
//! fluent API pattern enabling method chaining.
//!
//! # Examples
//!
//! ```ignore
//! // Build a mock handler
//! let handler = HandlerBuilder::new()
//!     .status(200)
//!     .json_body(json!({"message": "ok"}))
//!     .delay(Duration::from_millis(50))
//!     .build();
//!
//! // Build a test request
//! let (request, request_data) = RequestBuilder::new()
//!     .method(Method::POST)
//!     .path("/api/users")
//!     .json_body(json!({"name": "test"}))
//!     .build();
//! ```

use axum::body::Body;
use axum::http::{Method, Request, Response, StatusCode};
use serde_json::{Value, json};
use spikard_http::{Handler, HandlerResult, RequestData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Fluent builder for creating mock handlers with customizable behavior
///
/// Provides a fluent API for configuring handler responses without needing to
/// implement the Handler trait manually. Useful for testing middleware, routing,
/// and error handling without language bindings.
///
/// # Example
///
/// ```ignore
/// let handler = HandlerBuilder::new()
///     .status(StatusCode::CREATED)
///     .json_body(json!({"id": 1, "created": true}))
///     .build();
///
/// let response = handler.call(request, request_data).await?;
/// assert_eq!(response.status(), StatusCode::CREATED);
/// ```
pub struct HandlerBuilder {
    status: StatusCode,
    body: Value,
    delay: Option<Duration>,
    should_panic: bool,
}

impl HandlerBuilder {
    /// Create a new handler builder with default 200 OK status
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            body: json!({}),
            delay: None,
            should_panic: false,
        }
    }

    /// Set the HTTP status code for the response
    ///
    /// Default: 200 OK
    pub fn status(mut self, code: u16) -> Self {
        self.status = StatusCode::from_u16(code).unwrap_or(StatusCode::OK);
        self
    }

    /// Set the JSON body to return in the response
    ///
    /// Default: empty object `{}`
    pub fn json_body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// Add a delay to the handler response for testing timeouts
    ///
    /// Useful for simulating slow handlers and testing timeout middleware.
    pub const fn delay(mut self, duration: Duration) -> Self {
        self.delay = Some(duration);
        self
    }

    /// Configure the handler to panic when called
    ///
    /// Useful for testing panic recovery and error handling middleware.
    pub const fn panics(mut self) -> Self {
        self.should_panic = true;
        self
    }

    /// Build the configured handler into an Arc<dyn Handler>
    ///
    /// Returns a handler ready for use in tests.
    pub fn build(self) -> Arc<dyn Handler> {
        Arc::new(ConfiguredHandler {
            status: self.status,
            body: self.body,
            delay: self.delay,
            should_panic: self.should_panic,
        })
    }
}

impl Default for HandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal handler implementation constructed by `HandlerBuilder`
struct ConfiguredHandler {
    status: StatusCode,
    body: Value,
    delay: Option<Duration>,
    should_panic: bool,
}

impl Handler for ConfiguredHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let status = self.status;
        let body = self.body.clone();
        let delay = self.delay;
        let should_panic = self.should_panic;

        Box::pin(async move {
            assert!(!should_panic, "Handler configured to panic");

            if let Some(duration) = delay {
                sleep(duration).await;
            }

            let response = Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap();

            Ok(response)
        })
    }
}

/// Fluent builder for constructing test HTTP requests
///
/// Provides a fluent API for building both hyper `Request` objects and `RequestData`
/// structures needed for handler testing. Handles typical test scenarios without
/// requiring manual construction of all components.
///
/// # Example
///
/// ```ignore
/// let (request, request_data) = RequestBuilder::new()
///     .method(Method::POST)
///     .path("/api/users")
///     .headers(vec![("authorization".to_string(), "Bearer token".to_string())])
///     .json_body(json!({"name": "Alice", "email": "alice@example.com"}))
///     .build();
///
/// assert_eq!(request_data.method, "POST");
/// assert_eq!(request_data.path, "/api/users");
/// ```
pub struct RequestBuilder {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: Value,
    query_params: HashMap<String, Vec<String>>,
}

impl RequestBuilder {
    /// Create a new request builder with default GET method
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            path: "/".to_string(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: json!(null),
            query_params: HashMap::new(),
        }
    }

    /// Set the HTTP method
    ///
    /// Default: GET
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Set the request path
    ///
    /// Default: "/"
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    /// Add or replace headers from a `HashMap`
    ///
    /// Values are stored as-is; no normalization is performed.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Add a single header
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add or replace cookies from a `HashMap`
    pub fn cookies(mut self, cookies: HashMap<String, String>) -> Self {
        self.cookies = cookies;
        self
    }

    /// Add a single cookie
    pub fn cookie(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.cookies.insert(name.into(), value.into());
        self
    }

    /// Set the JSON request body
    ///
    /// Default: null
    pub fn json_body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// Set query parameters as a `HashMap` of name to values
    ///
    /// Values are stored as `Vec<String>` to support multi-valued parameters.
    pub fn query_params(mut self, params: HashMap<String, Vec<String>>) -> Self {
        self.query_params = params;
        self
    }

    /// Add a single query parameter
    pub fn query_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.entry(name.into()).or_default().push(value.into());
        self
    }

    /// Build the request into `(Request<Body>, RequestData)` tuple
    ///
    /// The `Request` can be passed directly to `handler.call()`. `RequestData` contains
    /// all extracted request information (params, body, headers, etc.).
    pub fn build(self) -> (Request<Body>, RequestData) {
        let body = if self.body.is_null() {
            Body::empty()
        } else {
            Body::from(self.body.to_string())
        };

        let mut request_builder = Request::builder().method(self.method.clone()).uri(&self.path);

        for (name, value) in &self.headers {
            request_builder = request_builder.header(name, value);
        }

        let request = request_builder.body(body).unwrap();

        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(build_query_json(&self.query_params)),
            validated_params: None,
            raw_query_params: Arc::new(self.query_params),
            body: Arc::new(self.body),
            raw_body: None,
            headers: Arc::new(self.headers),
            cookies: Arc::new(self.cookies),
            method: self.method.to_string(),
            path: self.path,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        (request, request_data)
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert raw query parameters into JSON format
fn build_query_json(raw_params: &HashMap<String, Vec<String>>) -> Value {
    let mut map = serde_json::Map::new();

    for (key, values) in raw_params {
        if values.is_empty() {
            map.insert(key.clone(), json!(null));
        } else if values.len() == 1 {
            map.insert(key.clone(), json!(values[0].clone()));
        } else {
            map.insert(key.clone(), json!(values.clone()));
        }
    }

    Value::Object(map)
}

/// Load a JSON fixture from the `testing_data` directory
///
/// # Arguments
///
/// * `relative_path` - Path relative to project root, e.g., `"testing_data/headers/01_user_agent_default.json"`
///
/// # Example
///
/// ```ignore
/// let fixture = load_fixture("testing_data/headers/01_user_agent_default.json")?;
/// assert!(fixture.is_object());
/// ```
///
/// # Errors
///
/// Returns error if file doesn't exist or is not valid JSON.
pub fn load_fixture(relative_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while root.pop() {
        if root.join("Cargo.toml").exists() {
            break;
        }
    }

    let path = root.join(relative_path);
    let content = std::fs::read_to_string(&path)?;
    let value = serde_json::from_str(&content)?;
    Ok(value)
}

/// Assert that a response has the expected status code
///
/// # Panics
///
/// Panics if the response status doesn't match the expected value.
///
/// # Example
///
/// ```ignore
/// let response = handler.call(request, request_data).await?;
/// assert_status(&response, StatusCode::CREATED);
/// ```
pub fn assert_status(response: &Response<Body>, expected: StatusCode) {
    assert_eq!(
        response.status(),
        expected,
        "Expected status {} but got {}",
        expected,
        response.status()
    );
}

/// Parse a response body as JSON
///
/// # Errors
///
/// Returns error if the body cannot be read or is not valid JSON.
///
/// # Example
///
/// ```ignore
/// let mut response = handler.call(request, request_data).await?;
/// let json = parse_json_body(&mut response).await?;
/// assert_eq!(json["id"], 123);
/// ```
pub async fn parse_json_body(response: &mut Response<Body>) -> Result<Value, Box<dyn std::error::Error>> {
    use axum::body::to_bytes;
    use std::mem;

    let body = mem::take(response.body_mut());
    let bytes = to_bytes(body, usize::MAX).await?;
    let value = serde_json::from_slice(&bytes)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_builder_default() {
        let handler = HandlerBuilder::new().build();
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!(null)),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_builder_custom_status() {
        let handler = HandlerBuilder::new().status(201).build();
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!(null)),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_handler_builder_with_body() {
        let body = json!({"message": "success", "code": 42});
        let handler = HandlerBuilder::new().json_body(body.clone()).build();

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!(null)),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handler_builder_with_delay() {
        let start = std::time::Instant::now();
        let handler = HandlerBuilder::new().delay(Duration::from_millis(10)).build();

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!(null)),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let _result = handler.call(request, request_data).await;
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_request_builder_default() {
        let (request, request_data) = RequestBuilder::new().build();

        assert_eq!(request.method(), &Method::GET);
        assert_eq!(request_data.path, "/");
        assert_eq!(request_data.method, "GET");
    }

    #[test]
    fn test_request_builder_post_with_body() {
        let body = json!({"name": "Alice", "age": 30});
        let (request, request_data) = RequestBuilder::new()
            .method(Method::POST)
            .path("/users")
            .json_body(body.clone())
            .build();

        assert_eq!(request.method(), &Method::POST);
        assert_eq!(request_data.path, "/users");
        assert_eq!(*request_data.body, body);
    }

    #[test]
    fn test_request_builder_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("authorization".to_string(), "Bearer token".to_string());
        headers.insert("x-custom".to_string(), "value".to_string());

        let (_request, request_data) = RequestBuilder::new().headers(headers.clone()).build();

        assert_eq!(
            request_data.headers.get("authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(request_data.headers.get("x-custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_request_builder_with_single_header() {
        let (_request, request_data) = RequestBuilder::new()
            .header("x-api-key", "secret123")
            .header("accept", "application/json")
            .build();

        assert_eq!(request_data.headers.get("x-api-key"), Some(&"secret123".to_string()));
        assert_eq!(
            request_data.headers.get("accept"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_request_builder_with_cookies() {
        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "abc123".to_string());
        cookies.insert("preferences".to_string(), "dark_mode".to_string());

        let (_request, request_data) = RequestBuilder::new().cookies(cookies).build();

        assert_eq!(request_data.cookies.get("session"), Some(&"abc123".to_string()));
        assert_eq!(request_data.cookies.get("preferences"), Some(&"dark_mode".to_string()));
    }

    #[test]
    fn test_request_builder_with_query_params() {
        let mut params = HashMap::new();
        params.insert("page".to_string(), vec!["1".to_string()]);
        params.insert("sort".to_string(), vec!["name".to_string()]);
        params.insert("filter".to_string(), vec!["active".to_string(), "verified".to_string()]);

        let (_request, request_data) = RequestBuilder::new().query_params(params).build();

        assert_eq!(request_data.query_params["page"], "1");
        assert_eq!(request_data.query_params["sort"], "name");
    }

    #[test]
    fn test_request_builder_single_query_param() {
        let (_request, request_data) = RequestBuilder::new()
            .query_param("limit", "10")
            .query_param("offset", "5")
            .build();

        assert_eq!(request_data.query_params["limit"], "10");
        assert_eq!(request_data.query_params["offset"], "5");
    }

    #[test]
    fn test_request_builder_fluent_api() {
        let body = json!({"name": "Bob"});
        let (_request, request_data) = RequestBuilder::new()
            .method(Method::PUT)
            .path("/users/42")
            .header("authorization", "Bearer abc123")
            .cookie("session", "xyz789")
            .json_body(body.clone())
            .query_param("notify", "true")
            .build();

        assert_eq!(request_data.method, "PUT");
        assert_eq!(request_data.path, "/users/42");
        assert_eq!(*request_data.body, body);
        assert_eq!(
            request_data.headers.get("authorization"),
            Some(&"Bearer abc123".to_string())
        );
        assert_eq!(request_data.cookies.get("session"), Some(&"xyz789".to_string()));
    }

    #[test]
    fn test_query_params_conversion() {
        let mut params = HashMap::new();
        params.insert("single".to_string(), vec!["value".to_string()]);

        let query_json = build_query_json(&params);
        assert_eq!(query_json["single"], "value");
    }
}
