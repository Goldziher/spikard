//! ValidatingHandler wrapper that executes request/parameter validation before handler

use crate::handler_trait::{Handler, HandlerResult, RequestData};
use axum::body::Body;
use futures::FutureExt;
use serde_json::Value;
use spikard_core::errors::StructuredError;
use spikard_core::{ParameterValidator, ProblemDetails, SchemaValidator};
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;

/// Wrapper that runs request/parameter validation before calling the user handler.
pub struct ValidatingHandler {
    inner: Arc<dyn Handler>,
    request_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
}

impl ValidatingHandler {
    /// Create a new validating handler wrapping the inner handler with schema validators
    pub fn new(inner: Arc<dyn Handler>, route: &crate::Route) -> Self {
        Self {
            inner,
            request_validator: route.request_validator.clone(),
            parameter_validator: route.parameter_validator.clone(),
        }
    }
}

impl Handler for ValidatingHandler {
    fn prefers_raw_json_body(&self) -> bool {
        self.inner.prefers_raw_json_body()
    }

    fn prefers_parameter_extraction(&self) -> bool {
        self.inner.prefers_parameter_extraction()
    }

    fn call(
        &self,
        req: axum::http::Request<Body>,
        mut request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        // Performance: Use references where possible to avoid Arc clones on every request.
        // The Arc clones here are cheap (atomic increment), but we store references
        // to Option<Arc<...>> to avoid cloning when validators are None (common case).
        let inner = &self.inner;
        let request_validator = &self.request_validator;
        let parameter_validator = &self.parameter_validator;

        Box::pin(async move {
            let is_json_body = request_data.body.is_null()
                && request_data.raw_body.is_some()
                && request_data
                    .headers
                    .get("content-type")
                    .is_some_and(|ct| crate::middleware::validation::is_json_like_str(ct));

            if is_json_body
                && request_validator.is_none()
                && !inner.prefers_raw_json_body()
                && let Some(raw_bytes) = request_data.raw_body.as_ref()
            {
                request_data.body = Arc::new(
                    serde_json::from_slice::<Value>(raw_bytes)
                        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?,
                );
            }

            if let Some(validator) = request_validator {
                if request_data.body.is_null()
                    && let Some(raw_bytes) = request_data.raw_body.as_ref()
                {
                    request_data.body = Arc::new(
                        serde_json::from_slice::<Value>(raw_bytes)
                            .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?,
                    );
                }

                if let Err(errors) = validator.validate(&request_data.body) {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((problem.status_code(), body));
                }
            }

            if let Some(validator) = parameter_validator
                && !inner.prefers_parameter_extraction()
            {
                match validator.validate_and_extract(
                    &request_data.query_params,
                    &request_data.raw_query_params,
                    &request_data.path_params,
                    &request_data.headers,
                    &request_data.cookies,
                ) {
                    Ok(validated) => {
                        request_data.validated_params = Some(Arc::new(validated));
                    }
                    Err(errors) => {
                        let problem = ProblemDetails::from_validation_error(&errors);
                        let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                        return Err((problem.status_code(), body));
                    }
                }
            }

            match AssertUnwindSafe(async { inner.call(req, request_data).await })
                .catch_unwind()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    let panic_payload = StructuredError::simple("panic", "Unexpected panic in handler");
                    let body = serde_json::to_string(&panic_payload)
                        .unwrap_or_else(|_| r#"{"error":"panic","code":"panic","details":{}}"#.to_string());
                    Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, body))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, Response, StatusCode};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Create a minimal RequestData for testing
    fn create_request_data(body: Value) -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(body),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    /// Create RequestData with raw body bytes
    fn create_request_data_with_raw_body(raw_body: Vec<u8>) -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: Some(bytes::Bytes::from(raw_body)),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    /// Success handler that echoes request body
    struct SuccessEchoHandler;

    impl Handler for SuccessEchoHandler {
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

    /// Panic handler for testing panic recovery
    struct PanicHandlerImpl;

    impl Handler for PanicHandlerImpl {
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

    /// Test 1: Handler with no validators passes through to inner handler
    #[tokio::test]
    async fn test_no_validation_passes_through() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"name": "Alice"}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Handler should succeed without validators");
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 1b: JSON body is parsed even without request schema validation.
    #[tokio::test]
    async fn test_json_body_parsed_without_request_validator() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: Some(bytes::Bytes::from(br#"{"name":"Alice"}"#.to_vec())),
            headers: Arc::new(headers),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let response = validator_handler
            .call(request, request_data)
            .await
            .expect("handler should succeed");
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read body");
        let echoed: Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(echoed["name"], "Alice");
    }

    /// Test 2: Request body validation - Valid input passes
    #[tokio::test]
    async fn test_request_body_validation_valid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"name": "Alice", "age": 30}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Valid request should pass validation");
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 3: Request body validation - Invalid input returns 422
    #[tokio::test]
    async fn test_request_body_validation_invalid() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"age": 30}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Invalid request should fail validation");
        let (status, body) = result.unwrap_err();
        assert_eq!(
            status,
            StatusCode::UNPROCESSABLE_ENTITY,
            "Should return 422 for validation error"
        );

        let problem: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        assert_eq!(problem["type"], "https://spikard.dev/errors/validation-error");
        assert_eq!(problem["title"], "Request Validation Failed");
        assert_eq!(problem["status"], 422);
        assert!(problem["errors"].is_array(), "Should contain errors array extension");
        assert!(
            problem["errors"][0]["loc"][0] == "body",
            "Error location should start with 'body'"
        );
    }

    /// Test 4: JSON parsing error returns 400
    #[tokio::test]
    async fn test_json_parsing_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data_with_raw_body(b"{invalid json}".to_vec());

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Invalid JSON should fail");
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(
            body.contains("Invalid JSON"),
            "Error message should mention invalid JSON"
        );
    }

    /// Test 5: Panic handling - Inner handler panic is caught and returns 500
    #[tokio::test]
    async fn test_panic_handling() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(PanicHandlerImpl);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Panicking handler should return error");
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR, "Panic should return 500");

        let error: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        assert_eq!(error["code"], "panic");
        assert_eq!(error["error"], "Unexpected panic in handler");
    }

    /// Test 6: Raw body parsing - Body is parsed on-demand from raw_body
    #[tokio::test]
    async fn test_raw_body_parsing() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let raw_body_json = br#"{"name":"Bob"}"#;
        let request_data = create_request_data_with_raw_body(raw_body_json.to_vec());

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Raw body should be parsed successfully");
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 7: Multiple validation error details in response
    #[tokio::test]
    async fn test_multiple_validation_errors() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "email": {"type": "string", "format": "email"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name", "email", "age"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"age": -5}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let problem: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        let errors = problem["errors"].as_array().expect("Should have errors array");
        assert!(
            errors.len() >= 2,
            "Should have multiple validation errors: got {}",
            errors.len()
        );
    }

    /// Test 8: Type mismatch in request body
    #[tokio::test]
    async fn test_type_mismatch_validation() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {"type": "integer"}
            },
            "required": ["age"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"age": "thirty"}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let problem: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        let errors = problem["errors"].as_array().expect("Should have errors array");
        assert!(!errors.is_empty());
        assert_eq!(errors[0]["loc"][1], "age");
    }

    /// Test 9: Successfully validates empty body when not required
    #[tokio::test]
    async fn test_empty_body_validation_optional() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Empty body should be valid when no fields are required");
    }

    /// Test 10: Parameter validation with empty validators passes through
    #[tokio::test]
    async fn test_parameter_validation_empty() {
        let param_validator = spikard_core::ParameterValidator::new(json!({})).expect("Valid empty schema");

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Get,
            path: "/search".to_string(),
            handler_name: "search_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: Some(param_validator),
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("GET")
            .uri("/search")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    /// Test 11: Request body is null when raw_body is None
    #[tokio::test]
    async fn test_null_body_with_no_raw_body() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Null body with no raw_body should fail");
    }

    /// Test 12: Panic error serialization has correct JSON structure
    #[tokio::test]
    async fn test_panic_error_json_structure() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(PanicHandlerImpl);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let error: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        assert!(error.get("error").is_some(), "Should have 'error' field");
        assert!(error.get("code").is_some(), "Should have 'code' field");
        assert_eq!(error["code"], "panic", "Code should be 'panic'");
    }

    /// Test 13: Handler receives request and request_data unchanged
    #[tokio::test]
    async fn test_handler_receives_correct_data() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let original_body = json!({"test": "data"});
        let request_data = create_request_data(original_body.clone());

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 14: Raw body parsing when body is null and raw_body exists
    #[tokio::test]
    async fn test_raw_body_parsing_when_body_null() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "integer"}
            },
            "required": ["id"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: Some(bytes::Bytes::from(br#"{"id":42}"#.to_vec())),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/test".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Should parse raw_body and validate successfully");
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 15: Validation error returns correct status code (422)
    #[tokio::test]
    async fn test_validation_error_status_code() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {"type": "integer", "minimum": 1}
            },
            "required": ["count"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"count": 0}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    /// Test 16: Invalid JSON parsing returns 400 status
    #[tokio::test]
    async fn test_invalid_json_parsing_status() {
        let schema = json!({"type": "object"});
        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data_with_raw_body(b"[invalid]".to_vec());

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    /// Test 17: Handler clones inner handler Arc correctly
    #[tokio::test]
    async fn test_inner_handler_arc_cloning() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let original_arc_ptr = Arc::as_ptr(&inner);

        let validator_handler = ValidatingHandler::new(inner.clone(), &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"data": "test"}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok());
        assert_eq!(Arc::as_ptr(&inner), original_arc_ptr);
    }

    /// Test 18: Panic during panic error serialization falls back to hardcoded JSON
    #[tokio::test]
    async fn test_panic_error_serialization_fallback() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(PanicHandlerImpl);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (_status, body) = result.unwrap_err();

        assert!(
            body.contains("panic") || body.contains("Unexpected"),
            "Body should contain panic-related information"
        );
    }

    /// Test 19: Validation error body is valid JSON
    #[tokio::test]
    async fn test_validation_error_body_is_json() {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {"type": "string", "format": "email"}
            },
            "required": ["email"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (_status, body) = result.unwrap_err();

        let parsed: serde_json::Value = serde_json::from_str(&body).expect("Validation error body must be valid JSON");
        assert!(parsed.is_object(), "Validation error body should be a JSON object");
    }

    /// Test 20: No validators means handler executes without validation
    #[tokio::test]
    async fn test_no_validators_executes_handler_directly() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"any": "data", "is": "ok"}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Without validators, any data should pass through");
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Test 21: Handler correctly uses path params, headers, and cookies from request data
    #[tokio::test]
    async fn test_handler_with_path_headers_cookies() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Get,
            path: "/api/{id}".to_string(),
            handler_name: "handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("GET")
            .uri("/api/123?search=test")
            .body(Body::empty())
            .unwrap();

        let mut request_data = create_request_data(json!({}));
        request_data.path_params = Arc::new({
            let mut m = HashMap::new();
            m.insert("id".to_string(), "123".to_string());
            m
        });
        request_data.headers = Arc::new({
            let mut m = HashMap::new();
            m.insert("x-custom".to_string(), "header-value".to_string());
            m
        });
        request_data.cookies = Arc::new({
            let mut m = HashMap::new();
            m.insert("session".to_string(), "abc123".to_string());
            m
        });

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    /// Test 22: Panic in handler produces correct status 500
    #[tokio::test]
    async fn test_panic_produces_500_status() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(PanicHandlerImpl);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test 23: Valid JSON but invalid schema should fail validation
    #[tokio::test]
    async fn test_valid_json_invalid_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "price": {"type": "number", "minimum": 0, "maximum": 1000}
            },
            "required": ["price"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"price": 2000.0}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Should fail schema validation");
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    /// Test 24: Empty raw body bytes with validator
    #[tokio::test]
    async fn test_empty_raw_body_bytes() {
        let schema = json!({
            "type": "object"
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data_with_raw_body(vec![]);

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Empty raw body should fail JSON parsing");
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    /// Test 25: JSON parsing error message contains useful info
    #[tokio::test]
    async fn test_json_parsing_error_message() {
        let schema = json!({"type": "object"});
        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data_with_raw_body(b"not valid json}}".to_vec());

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (_status, body) = result.unwrap_err();
        assert!(
            body.contains("Invalid JSON"),
            "Error message should mention invalid JSON"
        );
    }

    /// Test 26: Nested object validation in request body
    #[tokio::test]
    async fn test_nested_object_validation() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer"}
                    },
                    "required": ["name"]
                }
            },
            "required": ["user"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"user": {"age": 30}}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let problem: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        assert!(problem["errors"].is_array(), "Should contain errors array");
    }

    /// Test 27: Array validation in request body
    #[tokio::test]
    async fn test_array_validation() {
        let schema = json!({
            "type": "object",
            "properties": {
                "items": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["items"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"items": ["a", "b", "c"]}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok(), "Valid array should pass validation");
    }

    /// Test 28: Array with wrong item type validation error
    #[tokio::test]
    async fn test_array_wrong_item_type() {
        let schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            },
            "required": ["tags"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({"tags": ["tag1", 42, "tag3"]}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err(), "Array with wrong item type should fail");
        let (status, _body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    /// Test 29: Unwind safety with concurrent handler execution
    #[tokio::test]
    async fn test_concurrent_panic_handling() {
        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: None,
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: false,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(PanicHandlerImpl);
        let validator_handler = Arc::new(ValidatingHandler::new(inner, &route));

        let mut join_handles = vec![];

        for i in 0..5 {
            let shared_handler = validator_handler.clone();
            let handle = tokio::spawn(async move {
                let request = Request::builder()
                    .method("POST")
                    .uri("/test")
                    .body(Body::empty())
                    .unwrap();

                let request_data = create_request_data(json!({"id": i}));

                let result = shared_handler.call(request, request_data).await;
                assert!(result.is_err(), "Each concurrent panic should be caught");

                let (status, _body) = result.unwrap_err();
                assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
            });

            join_handles.push(handle);
        }

        for handle in join_handles {
            handle.await.expect("Concurrent test should complete");
        }
    }

    /// Test 30: Problem details status code from validation error
    #[tokio::test]
    async fn test_problem_details_status_code_mapping() {
        let schema = json!({
            "type": "object",
            "properties": {
                "required_field": {"type": "string"}
            },
            "required": ["required_field"]
        });

        let validator = Arc::new(SchemaValidator::new(schema).unwrap());

        let route = spikard_core::Route {
            method: spikard_core::http::Method::Post,
            path: "/test".to_string(),
            handler_name: "test_handler".to_string(),
            request_validator: Some(validator),
            response_validator: None,
            parameter_validator: None,
            file_params: None,
            is_async: true,
            cors: None,
            expects_json_body: true,
            #[cfg(feature = "di")]
            handler_dependencies: vec![],
            jsonrpc_method: None,
        };

        let inner = Arc::new(SuccessEchoHandler);
        let validator_handler = ValidatingHandler::new(inner, &route);

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let request_data = create_request_data(json!({}));

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let problem: serde_json::Value = serde_json::from_str(&body).expect("Should parse as JSON");
        assert_eq!(problem["status"], 422);
    }
}
