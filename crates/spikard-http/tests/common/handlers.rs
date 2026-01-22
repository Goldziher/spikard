//! Standard mock handlers for testing
//!
//! This module provides reusable mock handler implementations that cover common
//! test scenarios without requiring language bindings. All handlers implement the
//! Handler trait and are suitable for integration testing.

use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use serde_json::json;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::future::Future;
use std::pin::Pin;

/// Handler that always returns 200 OK with plain text response
///
/// Useful for testing basic handler execution and middleware chains.
///
/// # Example
///
/// ```ignore
/// let handler = SuccessHandler;
/// assert_eq!(handler.call(request, request_data).await.is_ok(), true);
/// ```
pub struct SuccessHandler;

impl Handler for SuccessHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/plain")
                .body(Body::from("OK"))
                .unwrap();
            Ok(response)
        })
    }
}

/// Handler that always returns 400 Bad Request with error message
///
/// Useful for testing error handling in middleware and response processing.
///
/// # Example
///
/// ```ignore
/// let handler = ErrorHandler;
/// assert_eq!(handler.call(request, request_data).await.is_err(), true);
/// ```
pub struct ErrorHandler;

impl Handler for ErrorHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move { Err((StatusCode::BAD_REQUEST, "Bad Request".to_string())) })
    }
}

/// Handler that intentionally panics during execution
///
/// Useful for testing panic recovery and error handling in the HTTP server.
/// This handler demonstrates that panics in handlers should be caught and
/// converted to HTTP error responses.
///
/// # Example
///
/// ```ignore
/// let handler = PanicHandler;
/// // This will panic, should be caught by server middleware
/// ```
pub struct PanicHandler;

impl Handler for PanicHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            panic!("Intentional panic for testing");
        })
    }
}

/// Handler that echoes the request body back as response
///
/// Useful for testing request body parsing and serialization/deserialization.
/// The response will be a JSON object containing the received body.
///
/// # Example
///
/// ```ignore
/// let handler = EchoHandler;
/// // Request with body {"key": "value"} returns same body
/// ```
pub struct EchoHandler;

impl Handler for EchoHandler {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(request_data.body.to_string()))
                .unwrap();
            Ok(response)
        })
    }
}

/// Handler that returns a JSON response with configurable status code and body
///
/// Useful for testing content negotiation and JSON serialization.
/// The response body is configurable via the `body` field, and the status code
/// via the `status_code` field.
///
/// # Example
///
/// ```ignore
/// let handler = JsonHandler {
///     status_code: StatusCode::CREATED,
///     body: json!({"id": 1, "name": "test"}),
/// };
/// ```
pub struct JsonHandler {
    /// HTTP status code for the response
    pub status_code: StatusCode,
    /// JSON body to return in response
    pub body: serde_json::Value,
}

impl JsonHandler {
    /// Create a new JSON handler with given status code and body
    pub const fn new(status_code: StatusCode, body: serde_json::Value) -> Self {
        Self { status_code, body }
    }

    /// Create a JSON handler with 200 OK status
    pub const fn ok(body: serde_json::Value) -> Self {
        Self {
            status_code: StatusCode::OK,
            body,
        }
    }

    /// Create a JSON handler with 201 Created status
    pub const fn created(body: serde_json::Value) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            body,
        }
    }

    /// Create a JSON handler with 400 Bad Request status
    pub const fn bad_request(body: serde_json::Value) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            body,
        }
    }

    /// Create a JSON handler with 500 Internal Server Error status
    pub const fn server_error(body: serde_json::Value) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body,
        }
    }
}

impl Handler for JsonHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let status_code = self.status_code;
        let body = self.body.clone();

        Box::pin(async move {
            let response = Response::builder()
                .status(status_code)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap();
            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn create_test_request_data() -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(serde_json::Value::Null),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!({"test": "data"})),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    #[tokio::test]
    async fn test_success_handler() {
        let handler = SuccessHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_error_handler() {
        let handler = ErrorHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_err());

        let (status, msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(msg, "Bad Request");
    }

    #[tokio::test]
    async fn test_echo_handler() {
        let handler = EchoHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data.clone()).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_json_handler_ok() {
        let body = json!({"id": 1, "name": "test"});
        let handler = JsonHandler::ok(body.clone());
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_json_handler_created() {
        let body = json!({"id": 1});
        let handler = JsonHandler::created(body);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_json_handler_custom() {
        let body = json!({"error": "Custom error"});
        let handler = JsonHandler::new(StatusCode::NOT_FOUND, body);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
