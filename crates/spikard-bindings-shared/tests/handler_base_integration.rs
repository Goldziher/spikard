//! Integration tests for `handler_base` module
//!
//! These tests cover the validation paths and error handling that aren't
//! covered by unit tests in the module itself.

use axum::body::Body;
use axum::http::Request;
use serde_json::json;
use spikard_bindings_shared::handler_base::{HandlerError, HandlerExecutor, LanguageHandler};
use spikard_core::validation::{SchemaValidator, ValidationError, ValidationErrorDetail};
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct MockHandler {
    should_fail_prepare: bool,
    should_fail_invoke: bool,
    should_fail_interpret: bool,
}

impl LanguageHandler for MockHandler {
    type Input = String;
    type Output = String;

    fn prepare_request(&self, _data: &RequestData) -> Result<Self::Input, HandlerError> {
        if self.should_fail_prepare {
            Err(HandlerError::Internal("Prepare failed".to_string()))
        } else {
            Ok("prepared".to_string())
        }
    }

    fn invoke_handler(
        &self,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output, HandlerError>> + Send + '_>> {
        let should_fail = self.should_fail_invoke;
        Box::pin(async move {
            if should_fail {
                Err(HandlerError::Execution("Handler failed".to_string()))
            } else {
                Ok(format!("output:{input}"))
            }
        })
    }

    fn interpret_response(&self, output: Self::Output) -> Result<axum::http::Response<Body>, HandlerError> {
        if self.should_fail_interpret {
            Err(HandlerError::ResponseConversion("Interpret failed".to_string()))
        } else {
            Ok(axum::http::Response::builder()
                .status(200)
                .body(Body::from(output))
                .unwrap())
        }
    }
}

#[tokio::test]
async fn test_handler_executor_with_validation_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "username": {"type": "string"},
            "age": {"type": "number"}
        },
        "required": ["username", "age"]
    });
    let validator = Arc::new(SchemaValidator::new(schema).unwrap());

    let handler = Arc::new(MockHandler {
        should_fail_prepare: false,
        should_fail_invoke: false,
        should_fail_interpret: false,
    });

    let executor = HandlerExecutor::new(handler, Some(validator));

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({"username": "john"})),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "POST".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handler_executor_prepare_failure() {
    let handler = Arc::new(MockHandler {
        should_fail_prepare: true,
        should_fail_invoke: false,
        should_fail_interpret: false,
    });

    let executor = HandlerExecutor::with_handler(handler);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({})),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handler_executor_invoke_failure() {
    let handler = Arc::new(MockHandler {
        should_fail_prepare: false,
        should_fail_invoke: true,
        should_fail_interpret: false,
    });

    let executor = HandlerExecutor::with_handler(handler);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({})),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handler_executor_interpret_failure() {
    let handler = Arc::new(MockHandler {
        should_fail_prepare: false,
        should_fail_invoke: false,
        should_fail_interpret: true,
    });

    let executor = HandlerExecutor::with_handler(handler);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({})),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_handler_executor_with_request_validator() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"}
        },
        "required": ["name"]
    });
    let request_validator = Arc::new(SchemaValidator::new(schema).unwrap());

    let handler = Arc::new(MockHandler {
        should_fail_prepare: false,
        should_fail_invoke: false,
        should_fail_interpret: false,
    });

    let executor = HandlerExecutor::new(handler, Some(request_validator));

    let request = Request::builder().body(Body::empty()).unwrap();

    let mut headers = HashMap::new();
    headers.insert("x-api-key".to_string(), "test-key".to_string());

    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({"name": "test"})),
        raw_body: None,
        headers: Arc::new(headers),
        cookies: Arc::new(HashMap::new()),
        method: "POST".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_ok());
}

#[test]
fn test_handler_error_from_validation_error() {
    let validation_error = ValidationError {
        errors: vec![ValidationErrorDetail {
            error_type: "missing".to_string(),
            loc: vec!["body".to_string(), "field".to_string()],
            msg: "Field required".to_string(),
            input: json!(null),
            ctx: None,
        }],
    };

    let handler_error: HandlerError = validation_error.into();
    assert!(matches!(handler_error, HandlerError::Validation(_)));
}

#[tokio::test]
async fn test_handler_executor_builder_pattern() {
    let schema = json!({
        "type": "object",
        "properties": {
            "email": {"type": "string", "format": "email"}
        }
    });
    let request_validator = Arc::new(SchemaValidator::new(schema).unwrap());

    let handler = Arc::new(MockHandler {
        should_fail_prepare: false,
        should_fail_invoke: false,
        should_fail_interpret: false,
    });

    let executor = HandlerExecutor::with_handler(handler).with_request_validator(request_validator);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(json!({"email": "test@example.com"})),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "POST".to_string(),
        path: "/test".to_string(),
        dependencies: None,
    };

    let result = executor.call(request, request_data).await;
    assert!(result.is_ok());
}
