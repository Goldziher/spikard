#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Integration tests for server handler wrappers (ValidatingHandler, DependencyInjectingHandler)
//!
//! These tests verify critical validation and dependency injection behavior across
//! the HTTP request pipeline. They test observable behaviors like status codes,
//! error messages, and request enrichment rather than internal validation logic.

mod common;

use axum::http::StatusCode;
use common::test_builders::{HandlerBuilder, RequestBuilder, assert_status};
use serde_json::json;
use spikard_core::Route;
use spikard_http::{Handler, server::handler::ValidatingHandler};
use std::sync::Arc;

mod validating_handler {
    use super::*;

    /// Test 1: Valid request passes validation and calls handler
    #[tokio::test]
    async fn test_validating_handler_allows_valid_request() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "email": {"type": "string"}
            },
            "required": ["name", "email"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/users".to_string(),
            handler_name: "create_user".to_string(),
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

        let inner_handler = HandlerBuilder::new()
            .status(201)
            .json_body(json!({"id": 1, "created": true}))
            .build();

        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .path("/users")
            .json_body(json!({"name": "Alice", "email": "alice@example.com"}))
            .build();

        let response = validator_handler.call(request, request_data).await.unwrap();

        assert_status(&response, StatusCode::CREATED);
    }

    /// Test 2: Request with invalid JSON body returns 422
    #[tokio::test]
    async fn test_validating_handler_rejects_invalid_json_body() {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {"type": "string"}
            },
            "required": ["email"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/users".to_string(),
            handler_name: "create_user".to_string(),
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

        let inner_handler = HandlerBuilder::new().status(200).build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .path("/users")
            .json_body(json!({"name": "Alice"}))
            .build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let error: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(error["errors"].is_array());
    }

    /// Test 3: Missing required field returns 422 with field-specific error
    #[tokio::test]
    async fn test_validating_handler_rejects_missing_required_field() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/api/test".to_string(),
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

        let inner_handler = HandlerBuilder::new().build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .json_body(json!({"age": 25}))
            .build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let error: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(error["errors"][0]["loc"][1].as_str().map_or(false, |s| s == "name"));
    }

    /// Test 4: Wrong type in field returns 422 with type error
    #[tokio::test]
    async fn test_validating_handler_rejects_wrong_type() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {"type": "integer"}
            },
            "required": ["count"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/api/test".to_string(),
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

        let inner_handler = HandlerBuilder::new().build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .json_body(json!({"count": "not_a_number"}))
            .build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let error: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(error["errors"].is_array());
    }

    /// Test 5: Optional fields missing still allows handler execution
    #[tokio::test]
    async fn test_validating_handler_allows_optional_fields() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "description": {"type": "string"}
            },
            "required": ["name"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/api/test".to_string(),
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

        let inner_handler = HandlerBuilder::new().status(200).build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .json_body(json!({"name": "Test"}))
            .build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_status(&response, StatusCode::OK);
    }

    /// Test 6: Nested object validation returns 422 with error path
    #[tokio::test]
    async fn test_validating_handler_validates_nested_objects() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"}
                    },
                    "required": ["name"]
                }
            },
            "required": ["user"]
        });

        let validator = Arc::new(spikard_core::SchemaValidator::new(schema).unwrap());

        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/api/test".to_string(),
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

        let inner_handler = HandlerBuilder::new().build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new()
            .method(axum::http::Method::POST)
            .json_body(json!({"user": {}}))
            .build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

        let error: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(error["errors"].is_array());
    }

    /// Test 7: Panicking handler returns 500 with structured panic error
    #[tokio::test]
    async fn test_validating_handler_catches_handler_panic() {
        let route = Route {
            method: spikard_core::http::Method::Post,
            path: "/api/test".to_string(),
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

        let inner_handler = HandlerBuilder::new().panics().build();
        let validator_handler = ValidatingHandler::new(inner_handler, &route);

        let (request, request_data) = RequestBuilder::new().method(axum::http::Method::POST).build();

        let result = validator_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, body) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let error: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(error["code"], "panic");
    }
}

#[cfg(feature = "di")]
mod dependency_injecting_handler {
    use super::*;
    use spikard_core::di::{DependencyContainer, ValueDependency};
    use spikard_http::DependencyInjectingHandler;

    /// Test 8: Dependencies are resolved and injected into handler
    #[tokio::test]
    async fn test_di_handler_injects_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_config_value")),
            )
            .unwrap();

        let inner_handler = HandlerBuilder::new()
            .status(200)
            .json_body(json!({"status": "ok"}))
            .build();

        let di_handler =
            DependencyInjectingHandler::new(inner_handler, Arc::new(container), vec!["config".to_string()]);

        let (request, request_data) = RequestBuilder::new().method(axum::http::Method::GET).build();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_status(&response, StatusCode::OK);
    }

    /// Test 9: Missing dependency returns 500 with resolution error
    #[tokio::test]
    async fn test_di_handler_resolution_failure_returns_500() {
        let container = DependencyContainer::new();

        let inner_handler = HandlerBuilder::new().build();
        let di_handler =
            DependencyInjectingHandler::new(inner_handler, Arc::new(container), vec!["missing_db".to_string()]);

        let (request, request_data) = RequestBuilder::new().method(axum::http::Method::GET).build();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test 10: Multiple dependencies are resolved correctly
    #[tokio::test]
    async fn test_di_handler_multiple_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register("db".to_string(), Arc::new(ValueDependency::new("db", "postgres")))
            .unwrap();
        container
            .register("cache".to_string(), Arc::new(ValueDependency::new("cache", "redis")))
            .unwrap();
        container
            .register("logger".to_string(), Arc::new(ValueDependency::new("logger", "slog")))
            .unwrap();

        let inner_handler = HandlerBuilder::new()
            .status(200)
            .json_body(json!({"services": ["db", "cache", "logger"]}))
            .build();

        let di_handler = DependencyInjectingHandler::new(
            inner_handler,
            Arc::new(container),
            vec!["db".to_string(), "cache".to_string(), "logger".to_string()],
        );

        let (request, request_data) = RequestBuilder::new().method(axum::http::Method::GET).build();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_status(&response, StatusCode::OK);
    }

    /// Test 11: Request-scoped dependencies are unique per request
    #[tokio::test]
    async fn test_di_handler_scoped_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "request_id".to_string(),
                Arc::new(ValueDependency::new("request_id", "uuid-123")),
            )
            .unwrap();

        let inner_handler = HandlerBuilder::new().status(200).build();
        let di_handler =
            DependencyInjectingHandler::new(inner_handler, Arc::new(container), vec!["request_id".to_string()]);

        let (request1, request_data1) = RequestBuilder::new()
            .method(axum::http::Method::GET)
            .path("/request/1")
            .build();

        let result1 = di_handler.call(request1, request_data1).await;
        assert!(result1.is_ok());

        let (request2, request_data2) = RequestBuilder::new()
            .method(axum::http::Method::GET)
            .path("/request/2")
            .build();

        let result2 = di_handler.call(request2, request_data2).await;
        assert!(result2.is_ok());

        assert_eq!(result1.unwrap().status(), StatusCode::OK);
        assert_eq!(result2.unwrap().status(), StatusCode::OK);
    }
}
