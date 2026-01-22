#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Tests for ServerConfig builder with dependency injection

mod common;

#[cfg(feature = "di")]
mod di_builder_tests {
    use spikard_core::di::ValueDependency;
    use spikard_http::ServerConfig;
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let config = ServerConfig::builder().port(3000).host("0.0.0.0").build();

        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    fn test_provide_value() {
        let config = ServerConfig::builder()
            .provide_value("test", "value".to_string())
            .build();

        assert!(config.di_container.is_some());
        let container = config.di_container.unwrap();

        assert!(Arc::strong_count(&container) >= 1);
    }

    #[test]
    fn test_provide_multiple_values() {
        let config = ServerConfig::builder()
            .provide_value("val1", 1)
            .provide_value("val2", 2)
            .provide_value("val3", "three".to_string())
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_provide_factory() {
        let config = ServerConfig::builder()
            .provide_factory("counter", |_resolved| async { Ok(42) })
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_provide_value_and_factory() {
        let config = ServerConfig::builder()
            .provide_value("base_value", 10)
            .provide_factory("computed", |_resolved| async { Ok(100) })
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_provide_custom_dependency() {
        let dep = ValueDependency::new("custom", "value".to_string());

        let config = ServerConfig::builder().provide(Arc::new(dep)).build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_builder_chaining() {
        let config = ServerConfig::builder()
            .port(8080)
            .host("localhost")
            .enable_request_id(false)
            .provide_value("app_name", "TestApp".to_string())
            .provide_value("version", "1.0.0".to_string())
            .build();

        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "localhost");
        assert!(!config.enable_request_id);
        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_builder_with_all_options() {
        use spikard_http::{BackgroundTaskConfig, CompressionConfig, RateLimitConfig};

        let config = ServerConfig::builder()
            .port(9000)
            .host("127.0.0.1")
            .workers(4)
            .enable_request_id(true)
            .max_body_size(Some(5 * 1024 * 1024))
            .request_timeout(Some(60))
            .compression(Some(CompressionConfig::default()))
            .rate_limit(Some(RateLimitConfig::default()))
            .graceful_shutdown(true)
            .shutdown_timeout(30)
            .background_tasks(BackgroundTaskConfig::default())
            .provide_value("config_value", "test".to_string())
            .build();

        assert_eq!(config.port, 9000);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.workers, 4);
        assert!(config.enable_request_id);
        assert_eq!(config.max_body_size, Some(5 * 1024 * 1024));
        assert_eq!(config.request_timeout, Some(60));
        assert!(config.compression.is_some());
        assert!(config.rate_limit.is_some());
        assert!(config.graceful_shutdown);
        assert_eq!(config.shutdown_timeout, 30);
        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_factory_with_dependencies() {
        let config = ServerConfig::builder()
            .provide_value("multiplier", 2)
            .provide_factory("result", |resolved| {
                let multiplier = resolved.get::<i32>("multiplier").map(|v| *v);
                async move {
                    let mult = multiplier.ok_or("multiplier not found")?;
                    Ok(mult * 21)
                }
            })
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_builder_without_di() {
        let config = ServerConfig::builder().port(3000).host("localhost").build();

        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "localhost");
        assert!(config.di_container.is_none());
    }

    #[test]
    fn test_multiple_factories() {
        let config = ServerConfig::builder()
            .provide_factory("factory1", |_resolved| async { Ok(1) })
            .provide_factory("factory2", |_resolved| async { Ok(2) })
            .provide_factory("factory3", |_resolved| async { Ok(3) })
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_mixed_dependencies() {
        let config = ServerConfig::builder()
            .provide_value("static1", "value1".to_string())
            .provide_factory("dynamic1", |_resolved| async { Ok("computed1".to_string()) })
            .provide_value("static2", 42)
            .provide_factory("dynamic2", |_resolved| async { Ok(100) })
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_type_inference() {
        let config = ServerConfig::builder()
            .provide_value("string", "test")
            .provide_value("number", 42)
            .provide_value("bool", true)
            .provide_value("vec", vec![1, 2, 3])
            .build();

        assert!(config.di_container.is_some());
    }

    #[test]
    fn test_builder_default() {
        let builder = ServerConfig::builder();
        let config = builder.build();

        assert_eq!(config.port, 8000);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.workers, 1);
        assert!(!config.enable_request_id);
        assert_eq!(config.max_body_size, Some(10 * 1024 * 1024));
        assert_eq!(config.request_timeout, None);
        assert!(config.graceful_shutdown);
        assert_eq!(config.shutdown_timeout, 30);
    }

    #[test]
    fn test_builder_override_defaults() {
        let config = ServerConfig::builder()
            .port(9999)
            .enable_request_id(false)
            .max_body_size(None)
            .build();

        assert_eq!(config.port, 9999);
        assert!(!config.enable_request_id);
        assert_eq!(config.max_body_size, None);
    }

    #[test]
    fn test_provide_with_arc() {
        let shared_value = Arc::new("shared".to_string());

        let config = ServerConfig::builder()
            .provide_value("shared", shared_value.clone())
            .build();

        assert!(config.di_container.is_some());
        assert_eq!(*shared_value, "shared");
    }
}

#[cfg(not(feature = "di"))]
mod no_di_tests {
    use spikard_http::ServerConfig;

    #[test]
    fn test_builder_without_di_feature() {
        let config = ServerConfig::builder().port(3000).host("localhost").build();

        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "localhost");
    }
}

/// Integration tests for common test handlers
///
/// These tests verify that the mock handlers in common module work correctly
/// and can be used for testing HTTP server components.
mod common_handler_tests {
    use crate::common::handlers::{EchoHandler, ErrorHandler, JsonHandler, SuccessHandler};
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::json;
    use spikard_http::{Handler, RequestData};
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
    async fn test_success_handler_integration() {
        let handler = SuccessHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok(), "SuccessHandler should return Ok response");
    }

    #[tokio::test]
    async fn test_error_handler_integration() {
        let handler = ErrorHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_err(), "ErrorHandler should return Err response");
    }

    #[tokio::test]
    async fn test_echo_handler_integration() {
        let handler = EchoHandler;
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok(), "EchoHandler should return Ok response");
    }

    #[tokio::test]
    async fn test_json_handler_ok_integration() {
        let body = json!({"status": "ok", "data": [1, 2, 3]});
        let handler = JsonHandler::ok(body);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok(), "JsonHandler::ok should return Ok response");
    }

    #[tokio::test]
    async fn test_json_handler_created_integration() {
        let body = json!({"id": 123, "created": true});
        let handler = JsonHandler::created(body);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_test_request_data();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok(), "JsonHandler::created should return Ok response");
    }
}
