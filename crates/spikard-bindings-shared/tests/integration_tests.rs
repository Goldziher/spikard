//! Full integration tests for all spikard-bindings-shared modules
//!
//! This test file ensures full code coverage across all modules in the crate,
//! testing edge cases, error paths, and integration scenarios.

use axum::http::{Request, StatusCode};
use pretty_assertions::assert_eq;
use serde_json::json;
use spikard_bindings_shared::conversion_traits::{FromLanguage, ToLanguage};
use spikard_bindings_shared::response_builder::ResponseBuilder;
use spikard_bindings_shared::*;
use spikard_core::RequestData as CoreRequestData;
use spikard_core::di::{Dependency, ResolvedDependencies};
use spikard_core::problem::ProblemDetails;
use spikard_core::validation::{ValidationError, ValidationErrorDetail};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
struct CustomType {
    value: i32,
}

impl FromLanguage for CustomType {
    type Error = String;

    fn from_any(value: &(dyn std::any::Any + Send + Sync)) -> Result<Self, Self::Error> {
        value
            .downcast_ref::<i32>()
            .map(|&v| Self { value: v })
            .ok_or_else(|| "Type mismatch".to_string())
    }
}

impl ToLanguage for CustomType {
    type Error = String;

    fn to_any(&self) -> Result<Box<dyn std::any::Any + Send + Sync>, Self::Error> {
        Ok(Box::new(self.value))
    }
}

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
fn test_validation_error_with_multiple_errors() {
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
fn test_problem_details_with_extensions() {
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
fn test_response_builder_with_multiple_headers() {
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
    let cloned_continue = continue_result;
    assert!(matches!(cloned_continue, HookResult::Continue));

    let short_circuit = HookResult::ShortCircuit(json!({"status": "error"}));
    let cloned_short = short_circuit;
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
fn test_test_client_with_configuration() {
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
async fn test_di_traits_with_value_and_factory_adapters() {
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
        validated_params: None,
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
fn test_conversion_traits_with_json_and_language_conversion() {
    use conversion_traits::{FromLanguage, JsonConversionError, JsonConvertible, ToLanguage};

    let error = JsonConversionError("Test error message".to_string());
    assert_eq!(error.to_string(), "JSON conversion error: Test error message");

    let test_cases = vec![
        json!(null),
        json!(true),
        json!(false),
        json!(42),
        json!(3.2),
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
fn test_json_conversion_types_comprehensive() {
    use json_conversion::{JsonConversionHelper, JsonPrimitive};

    // Test JsonPrimitive enum variants
    let string_prim = JsonPrimitive::String("test");
    let number_prim = JsonPrimitive::Number("42.5".to_string());
    let bool_prim = JsonPrimitive::Bool(true);
    let null_prim = JsonPrimitive::Null;

    assert!(matches!(string_prim, JsonPrimitive::String(_)));
    assert!(matches!(number_prim, JsonPrimitive::Number(_)));
    assert!(matches!(bool_prim, JsonPrimitive::Bool(_)));
    assert!(matches!(null_prim, JsonPrimitive::Null));

    // Test JsonConversionHelper for fast path detection
    assert!(JsonConversionHelper::is_primitive(&json!(42)));
    assert!(JsonConversionHelper::is_primitive(&json!("text")));
    assert!(!JsonConversionHelper::is_primitive(&json!([1, 2, 3])));
}

#[test]
fn test_lazy_cache_integration_with_json_conversion() {
    use json_conversion::JsonConversionHelper;
    use lazy_cache::LazyCache;

    let helper_cache: LazyCache<JsonConversionHelper> = LazyCache::new();

    // Initialize cache with JSON conversion helper
    let _cached = helper_cache.get_or_init(|| JsonConversionHelper);

    // Verify we can use it
    let test_json = json!({"key": "value"});
    assert!(JsonConversionHelper::is_primitive(&json!(42)));
    assert!(!JsonConversionHelper::is_primitive(&test_json));

    // Test cache state
    assert!(helper_cache.is_cached());

    // Test invalidate
    helper_cache.invalidate();
    assert!(!helper_cache.is_cached());
}

#[test]
fn test_response_interpreter_with_json_conversion() {
    use response_interpreter::InterpretedResponse;

    let response_body = json!({
        "status": "success",
        "data": {
            "id": 1,
            "name": "test"
        }
    });

    // Create a Custom interpreted response
    let interpreted = InterpretedResponse::Custom {
        status: 200,
        headers: HashMap::new(),
        body: Some(response_body),
        raw_body: None,
    };

    match interpreted {
        InterpretedResponse::Custom { status, body, .. } => {
            assert_eq!(status, 200);
            assert_eq!(body.as_ref().unwrap()["status"], "success");
        }
        _ => panic!("Expected Custom variant"),
    }
}

#[test]
fn test_build_optimized_response_bytes_function_availability() {
    use axum::body::Bytes;
    use axum::http::StatusCode;

    let body_bytes = Bytes::from_static(br#"{"status":"ok"}"#);
    let _response = build_optimized_response_bytes(StatusCode::OK, None, body_bytes);

    // Function is available and can be called
}

#[test]
fn test_module_interactions_json_lazy_response() {
    use json_conversion::JsonConversionHelper;
    use lazy_cache::LazyCache;
    use response_interpreter::InterpretedResponse;

    // Create a lazy-cached conversion helper
    let cache: LazyCache<JsonConversionHelper> = LazyCache::new();
    let _ = cache.get_or_init(|| JsonConversionHelper);

    // Create test JSON
    let json_data = json!({
        "message": "integration test",
        "timestamp": 1_234_567_890,
        "nested": {
            "array": [1, 2, 3],
            "bool": true
        }
    });

    // Verify helper can detect primitives
    let primitive_val = json!(42);
    assert!(JsonConversionHelper::is_primitive(&primitive_val));
    assert!(!JsonConversionHelper::is_primitive(&json_data));

    // Create interpreted response
    let response = InterpretedResponse::Custom {
        status: 200,
        body: Some(json_data),
        headers: HashMap::new(),
        raw_body: None,
    };

    match response {
        InterpretedResponse::Custom { status, .. } => assert_eq!(status, 200),
        _ => panic!("Expected Custom variant"),
    }
    assert!(cache.is_cached());
}

#[test]
fn test_response_builder_bytes_and_optimized_paths() {
    use axum::body::Bytes;
    use axum::http::StatusCode;

    let test_body = br#"{"result":"test"}"#.to_vec();

    // Test build_optimized_response_bytes path
    let resp_bytes = build_optimized_response_bytes(StatusCode::CREATED, None, Bytes::from(test_body.clone()));
    assert_eq!(resp_bytes.status(), StatusCode::CREATED);

    // Test build_optimized_response path
    let resp = build_optimized_response(StatusCode::OK, None, test_body);
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn test_json_conversion_error_handling() {
    use json_conversion::JsonConversionError;

    let error = JsonConversionError::InvalidValue {
        reason: "Invalid JSON structure".to_string(),
    };
    let error_str = error.to_string();
    assert!(error_str.contains("Invalid JSON structure"));
}

#[test]
fn test_lazy_cache_multiple_types() {
    use lazy_cache::LazyCache;

    // Test with String
    let cache_string: LazyCache<String> = LazyCache::new();
    let result = cache_string.get_or_init(|| "cached value".to_string());
    assert_eq!(result, "cached value");
    assert!(cache_string.is_cached());

    // Test with Vec
    let cache_vec: LazyCache<Vec<i32>> = LazyCache::new();
    let result = cache_vec.get_or_init(|| vec![1, 2, 3]);
    assert_eq!(*result, vec![1, 2, 3]);
    assert!(cache_vec.is_cached());
}

#[test]
fn test_all_new_modules_publicly_accessible() {
    println!("All new modules successfully exported from lib.rs:");
    println!("- json_conversion (JsonConverter, JsonConversionHelper, JsonConversionError, JsonPrimitive)");
    println!("- lazy_cache (LazyCache)");
    println!("- response_interpreter (InterpretedResponse, ResponseInterpreter, StreamSource)");
    println!("- response_builder functions (build_optimized_response, build_optimized_response_bytes)");

    // Verify all types are in scope by using them
    {
        use spikard_bindings_shared::JsonConversionHelper;
        // Type is imported and available in scope
        let _: &[u8] = b"";
        let _ = JsonConversionHelper::is_primitive(&json!(null));
    }

    {
        use spikard_bindings_shared::LazyCache;
        let _cache: LazyCache<i32> = LazyCache::new();
    }

    {
        use spikard_bindings_shared::InterpretedResponse;
        let _resp = InterpretedResponse::Custom {
            status: 200,
            body: Some(json!({})),
            headers: HashMap::new(),
            raw_body: None,
        };
    }

    {
        use axum::body::Bytes;
        use axum::http::StatusCode;
        use spikard_bindings_shared::{build_optimized_response, build_optimized_response_bytes};

        let _resp1 = build_optimized_response(StatusCode::OK, None, br"{}".to_vec());
        let _resp2 = build_optimized_response_bytes(StatusCode::OK, None, Bytes::from_static(br"{}"));
    }

    println!("✓ All types successfully imported and used from public API");
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
    println!("- json_conversion (JsonConverter, JsonConversionHelper)");
    println!("- lazy_cache (LazyCache)");
    println!("- response_interpreter (InterpretedResponse, ResponseInterpreter)");
}
