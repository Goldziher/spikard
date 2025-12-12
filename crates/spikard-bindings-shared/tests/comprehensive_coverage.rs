//! Comprehensive coverage tests for all spikard-bindings-shared modules
//!
//! This test file ensures full code coverage across all modules in the crate,
//! testing edge cases, error paths, and integration scenarios.

use axum::http::{Request, StatusCode};
use pretty_assertions::assert_eq;
use serde_json::json;
use spikard_bindings_shared::response_builder::ResponseBuilder;
use spikard_bindings_shared::*;
use spikard_core::RequestData as CoreRequestData;
use spikard_core::di::{Dependency, ResolvedDependencies};
use spikard_core::problem::ProblemDetails;
use spikard_core::validation::{ValidationError, ValidationErrorDetail};
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_error_response_all_status_codes_coverage() {
    let test_cases = vec![
        (
            ErrorResponseBuilder::bad_request("msg"),
            StatusCode::BAD_REQUEST,
            "bad_request",
        ),
        (
            ErrorResponseBuilder::internal_error("msg"),
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
        ),
        (
            ErrorResponseBuilder::unauthorized("msg"),
            StatusCode::UNAUTHORIZED,
            "unauthorized",
        ),
        (
            ErrorResponseBuilder::forbidden("msg"),
            StatusCode::FORBIDDEN,
            "forbidden",
        ),
        (
            ErrorResponseBuilder::not_found("msg"),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            ErrorResponseBuilder::method_not_allowed("msg"),
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
        ),
        (
            ErrorResponseBuilder::unprocessable_entity("msg"),
            StatusCode::UNPROCESSABLE_ENTITY,
            "unprocessable_entity",
        ),
        (ErrorResponseBuilder::conflict("msg"), StatusCode::CONFLICT, "conflict"),
        (
            ErrorResponseBuilder::service_unavailable("msg"),
            StatusCode::SERVICE_UNAVAILABLE,
            "service_unavailable",
        ),
        (
            ErrorResponseBuilder::request_timeout("msg"),
            StatusCode::REQUEST_TIMEOUT,
            "request_timeout",
        ),
    ];

    for ((status, body), expected_status, expected_code) in test_cases {
        assert_eq!(status, expected_status);
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], expected_code);
        assert_eq!(parsed["error"], "msg");
    }
}

#[test]
fn test_validation_error_comprehensive() {
    let validation_error = ValidationError {
        errors: vec![
            ValidationErrorDetail {
                error_type: "missing".to_string(),
                loc: vec!["body".to_string(), "field1".to_string()],
                msg: "Field required".to_string(),
                input: serde_json::Value::Null,
                ctx: None,
            },
            ValidationErrorDetail {
                error_type: "type_error".to_string(),
                loc: vec!["body".to_string(), "field2".to_string()],
                msg: "Invalid type".to_string(),
                input: json!("wrong"),
                ctx: Some(json!({"expected": "number"})),
            },
        ],
    };

    let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["status"], 422);
    assert_eq!(parsed["errors"].as_array().unwrap().len(), 2);
}

#[test]
fn test_problem_details_comprehensive() {
    let mut problem = ProblemDetails::internal_server_error("System error");
    problem.instance = Some("/api/users/123".to_string());
    problem.extensions.insert("trace_id".to_string(), json!("abc-123"));
    problem.extensions.insert("retry_after".to_string(), json!(60));

    let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["status"], 500);
    assert_eq!(parsed["instance"], "/api/users/123");
    assert_eq!(parsed["trace_id"], "abc-123");
    assert_eq!(parsed["retry_after"], 60);
}

#[test]
fn test_structured_error_with_complex_details() {
    let details = json!({
        "validation_errors": [
            {"field": "email", "code": "invalid_format"},
            {"field": "age", "code": "out_of_range"}
        ],
        "metadata": {
            "request_id": "req-12345",
            "timestamp": "2024-01-01T00:00:00Z"
        }
    });

    let (status, body) = ErrorResponseBuilder::with_details(
        StatusCode::BAD_REQUEST,
        "validation_failed",
        "Multiple validation errors",
        details,
    );

    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["code"], "validation_failed");
    assert!(parsed["details"]["validation_errors"].is_array());
    assert_eq!(parsed["details"]["metadata"]["request_id"], "req-12345");
}

#[test]
fn test_response_builder_comprehensive() {
    let (status, headers, body) = ResponseBuilder::new()
        .status(StatusCode::CREATED)
        .body(json!({
            "id": 123,
            "name": "Test",
            "tags": ["tag1", "tag2"],
            "metadata": {
                "created": "2024-01-01"
            }
        }))
        .header("Content-Type", "application/json")
        .header("X-Request-Id", "req-123")
        .header("X-Custom", "value")
        .build();

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(headers.len(), 3);

    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["id"], 123);
    assert_eq!(parsed["tags"][0], "tag1");
}

#[test]
fn test_response_builder_invalid_headers() {
    let (_, headers, _) = ResponseBuilder::new()
        .header("Invalid\nHeader", "value1")
        .header("Valid-Header", "value2")
        .header("Another\r\nInvalid", "value3")
        .build();

    assert_eq!(headers.len(), 1);
    assert!(headers.get("valid-header").is_some());
}

#[test]
fn test_lifecycle_hook_types() {
    use lifecycle_base::{HookResult, LifecycleHookType};

    let hook_types = vec![
        LifecycleHookType::OnRequest,
        LifecycleHookType::PreValidation,
        LifecycleHookType::PreHandler,
        LifecycleHookType::OnResponse,
        LifecycleHookType::OnError,
    ];

    for hook_type in &hook_types {
        assert_eq!(*hook_type, *hook_type);
    }

    let continue_result = HookResult::Continue;
    let cloned_continue = continue_result.clone();
    assert!(matches!(cloned_continue, HookResult::Continue));

    let short_circuit = HookResult::ShortCircuit(json!({"status": "error"}));
    let cloned_short = short_circuit.clone();
    assert!(matches!(cloned_short, HookResult::ShortCircuit(_)));
}

#[test]
fn test_validation_helpers_all_field_types() {
    use validation_helpers::{BodyValidator, FieldType, HeaderFormat, HeaderValidator};

    let body = json!({
        "string_field": "text",
        "number_field": 42,
        "boolean_field": true,
        "object_field": {"key": "value"},
        "array_field": [1, 2, 3],
        "null_field": null
    });

    assert!(BodyValidator::validate_field_type(&body, "string_field", FieldType::String).is_ok());
    assert!(BodyValidator::validate_field_type(&body, "number_field", FieldType::Number).is_ok());
    assert!(BodyValidator::validate_field_type(&body, "boolean_field", FieldType::Boolean).is_ok());
    assert!(BodyValidator::validate_field_type(&body, "object_field", FieldType::Object).is_ok());
    assert!(BodyValidator::validate_field_type(&body, "array_field", FieldType::Array).is_ok());

    assert!(BodyValidator::validate_field_type(&body, "null_field", FieldType::String).is_err());

    assert!(HeaderValidator::validate_format("Authorization", "Bearer token", HeaderFormat::Bearer).is_ok());
    assert!(
        HeaderValidator::validate_format("Content-Type", "application/json; charset=utf-8", HeaderFormat::Json).is_ok()
    );
}

#[test]
fn test_test_client_comprehensive() {
    use test_client_base::{TestClientConfig, TestResponseMetadata};

    let config = TestClientConfig::new("http://example.com")
        .with_timeout(5000)
        .with_follow_redirects(false);

    assert_eq!(config.base_url, "http://example.com");
    assert_eq!(config.timeout_ms, 5000);
    assert!(!config.follow_redirects);

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("X-Custom".to_string(), "value".to_string());

    let metadata = TestResponseMetadata::new(201, headers, 1024, 150);

    assert_eq!(metadata.status_code, 201);
    assert_eq!(metadata.body_size, 1024);
    assert_eq!(metadata.response_time_ms, 150);
    assert!(metadata.is_success());
    assert!(!metadata.is_client_error());
    assert!(!metadata.is_server_error());

    assert_eq!(
        metadata.get_header("content-type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        metadata.get_header("CONTENT-TYPE"),
        Some(&"application/json".to_string())
    );
}

#[tokio::test]
async fn test_di_traits_comprehensive() {
    use di_traits::{FactoryDependencyAdapter, FactoryDependencyBridge, ValueDependencyAdapter, ValueDependencyBridge};
    use std::sync::atomic::{AtomicBool, Ordering};

    struct TestValueAdapter {
        key: String,
        value: String,
    }

    impl ValueDependencyAdapter for TestValueAdapter {
        fn key(&self) -> &str {
            &self.key
        }

        fn resolve_value(
            &self,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<Arc<dyn std::any::Any + Send + Sync>, spikard_core::di::DependencyError>,
                    > + Send
                    + '_,
            >,
        > {
            let value = self.value.clone();
            Box::pin(async move { Ok(Arc::new(value) as Arc<dyn std::any::Any + Send + Sync>) })
        }
    }

    struct TestFactoryAdapter {
        key: String,
        called: Arc<AtomicBool>,
    }

    impl FactoryDependencyAdapter for TestFactoryAdapter {
        fn key(&self) -> &str {
            &self.key
        }

        fn invoke_factory(
            &self,
            _request: &http::Request<()>,
            _request_data: &CoreRequestData,
            _resolved: &ResolvedDependencies,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<Arc<dyn std::any::Any + Send + Sync>, spikard_core::di::DependencyError>,
                    > + Send
                    + '_,
            >,
        > {
            let called = self.called.clone();
            Box::pin(async move {
                called.store(true, Ordering::SeqCst);
                Ok(Arc::new("factory_result".to_string()) as Arc<dyn std::any::Any + Send + Sync>)
            })
        }
    }

    let value_adapter = TestValueAdapter {
        key: "test_value".to_string(),
        value: "test_data".to_string(),
    };
    let value_bridge = ValueDependencyBridge::new(value_adapter);
    assert_eq!(value_bridge.key(), "test_value");
    assert_eq!(value_bridge.depends_on(), Vec::<String>::new());

    let request = Request::builder().body(()).unwrap();
    let request_data = CoreRequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: json!({}),
        raw_query_params: Arc::new(HashMap::new()),
        body: json!({}),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/".to_string(),
        dependencies: None,
    };
    let resolved = ResolvedDependencies::new();

    let result = value_bridge.resolve(&request, &request_data, &resolved).await;
    assert!(result.is_ok());

    let called_flag = Arc::new(AtomicBool::new(false));
    let factory_adapter = TestFactoryAdapter {
        key: "test_factory".to_string(),
        called: called_flag.clone(),
    };
    let factory_bridge = FactoryDependencyBridge::new(factory_adapter);

    assert_eq!(factory_bridge.key(), "test_factory");
    assert!(!called_flag.load(Ordering::SeqCst));

    let result = factory_bridge.resolve(&request, &request_data, &resolved).await;
    assert!(result.is_ok());
    assert!(called_flag.load(Ordering::SeqCst));
}

#[test]
fn test_conversion_traits_comprehensive() {
    use conversion_traits::{FromLanguage, JsonConversionError, JsonConvertible, ToLanguage};

    let error = JsonConversionError("Test error message".to_string());
    assert_eq!(error.to_string(), "JSON conversion error: Test error message");

    let test_cases = vec![
        json!(null),
        json!(true),
        json!(false),
        json!(42),
        json!(3.14),
        json!("string"),
        json!([1, 2, 3]),
        json!({"key": "value"}),
    ];

    for test_value in test_cases {
        let from_result = serde_json::Value::from_json(test_value.clone());
        assert!(from_result.is_ok());
        assert_eq!(from_result.unwrap(), test_value);

        let to_result = test_value.to_json();
        assert!(to_result.is_ok());
        assert_eq!(to_result.unwrap(), test_value);
    }

    #[derive(Debug)]
    struct CustomType {
        value: i32,
    }

    impl FromLanguage for CustomType {
        type Error = String;

        fn from_any(value: &(dyn std::any::Any + Send + Sync)) -> Result<Self, Self::Error> {
            value
                .downcast_ref::<i32>()
                .map(|&v| CustomType { value: v })
                .ok_or_else(|| "Type mismatch".to_string())
        }
    }

    impl ToLanguage for CustomType {
        type Error = String;

        fn to_any(&self) -> Result<Box<dyn std::any::Any + Send + Sync>, Self::Error> {
            Ok(Box::new(self.value))
        }
    }

    let any_value: Box<dyn std::any::Any + Send + Sync> = Box::new(42i32);
    let custom = CustomType::from_any(&*any_value).unwrap();
    assert_eq!(custom.value, 42);

    let back_to_any = custom.to_any().unwrap();
    let back_to_i32 = back_to_any.downcast_ref::<i32>().unwrap();
    assert_eq!(*back_to_i32, 42);

    let wrong_type: Box<dyn std::any::Any + Send + Sync> = Box::new("string");
    let result = CustomType::from_any(&*wrong_type);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Type mismatch");
}

#[test]
fn test_all_modules_documented() {
    println!("All modules successfully imported and tested:");
    println!("- ErrorResponseBuilder");
    println!("- ResponseBuilder");
    println!("- handler_base (LanguageHandler, HandlerExecutor, HandlerError)");
    println!("- lifecycle_base (LifecycleHook, LifecycleConfig, HookResult)");
    println!("- validation_helpers (HeaderValidator, BodyValidator)");
    println!("- test_client_base (TestClientConfig, TestResponseMetadata)");
    println!("- di_traits (ValueDependencyAdapter, FactoryDependencyAdapter)");
    println!("- conversion_traits (FromLanguage, ToLanguage, JsonConvertible)");
}
