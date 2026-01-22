#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Integration tests for Dependency Injection system
//!
//! These tests verify that the DI system integrates correctly with the HTTP handler pipeline.
//! More complex DI features (factories, dependencies, cleanup) are tested in unit tests.

#![cfg(feature = "di")]

mod common;

use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use spikard_core::di::{DependencyContainer, ValueDependency};
use spikard_http::{DependencyInjectingHandler, Handler, HandlerResult, RequestData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Test handler that accesses injected dependencies
struct DependencyAccessHandler {
    dependency_name: String,
}

impl Handler for DependencyAccessHandler {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let dependency_name = self.dependency_name.clone();

        Box::pin(async move {
            if let Some(deps) = &request_data.dependencies {
                if let Some(value) = deps.get::<String>(&dependency_name) {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(format!("Dependency value: {}", *value)))
                        .unwrap();
                    Ok(response)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Dependency '{}' not found", dependency_name),
                    ))
                }
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No dependencies attached".to_string(),
                ))
            }
        })
    }
}

#[tokio::test]
async fn test_di_value_injection() {
    let mut container = DependencyContainer::new();
    container
        .register(
            "config".to_string(),
            Arc::new(ValueDependency::new("config", "test_config_value".to_string())),
        )
        .unwrap();

    let handler = Arc::new(DependencyAccessHandler {
        dependency_name: "config".to_string(),
    });

    let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let result = di_handler.call(request, request_data).await;

    let response = match result {
        Ok(r) => r,
        Err((status, msg)) => {
            panic!("Expected Ok, got Err({:?}, {})", status, msg);
        }
    };
    assert_eq!(response.status(), StatusCode::OK);

    use http_body_util::BodyExt;
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(body_str, "Dependency value: test_config_value");
}

#[tokio::test]
async fn test_di_missing_dependency_error() {
    let container = DependencyContainer::new();

    let handler = Arc::new(DependencyAccessHandler {
        dependency_name: "database".to_string(),
    });

    let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["database".to_string()]);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let result = di_handler.call(request, request_data).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_di_multiple_value_dependencies() {
    let mut container = DependencyContainer::new();

    container
        .register(
            "config".to_string(),
            Arc::new(ValueDependency::new("config", "config_value".to_string())),
        )
        .unwrap();

    container
        .register(
            "cache_url".to_string(),
            Arc::new(ValueDependency::new("cache_url", "redis://localhost".to_string())),
        )
        .unwrap();

    let handler = Arc::new(DependencyAccessHandler {
        dependency_name: "cache_url".to_string(),
    });

    let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["cache_url".to_string()]);

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let result = di_handler.call(request, request_data).await;

    let response = match result {
        Ok(r) => r,
        Err((status, msg)) => {
            panic!("Expected Ok, got Err({:?}, {})", status, msg);
        }
    };
    assert_eq!(response.status(), StatusCode::OK);

    use http_body_util::BodyExt;
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(body_str, "Dependency value: redis://localhost");
}
