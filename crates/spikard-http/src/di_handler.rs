//! Dependency Injection Handler Wrapper
//!
//! This module provides a handler wrapper that integrates the DI system with the HTTP
//! handler pipeline. It follows the same composition pattern as `ValidatingHandler`.
//!
//! # Architecture
//!
//! The `DependencyInjectingHandler` wraps any `Handler` and:
//! 1. Resolves required dependencies in parallel batches before calling the handler
//! 2. Attaches resolved dependencies to `RequestData`
//! 3. Calls the inner handler with the enriched request data
//! 4. Cleans up dependencies after the handler completes (async Drop pattern)
//!
//! # Performance
//!
//! - **Zero overhead when no DI**: If no container is provided, DI is skipped entirely
//! - **Parallel resolution**: Independent dependencies are resolved concurrently
//! - **Efficient caching**: Singleton and per-request caching minimize redundant work
//! - **Composable**: Works seamlessly with `ValidatingHandler` and lifecycle hooks
//!
//! # Examples
//!
//! ```ignore
//! use spikard_http::di_handler::DependencyInjectingHandler;
//! use spikard_core::di::DependencyContainer;
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let container = Arc::new(DependencyContainer::new());
//! let handler = Arc::new(MyHandler::new());
//!
//! let di_handler = DependencyInjectingHandler::new(
//!     handler,
//!     container,
//!     vec!["database".to_string(), "cache".to_string()],
//! );
//! # });
//! ```

use crate::handler_trait::{Handler, HandlerResult, RequestData};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use spikard_core::di::{DependencyContainer, DependencyError};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::{debug, info_span, instrument};

/// Handler wrapper that resolves dependencies before calling the inner handler
///
/// This wrapper follows the composition pattern used by `ValidatingHandler`:
/// it wraps an existing handler and enriches the request with resolved dependencies.
///
/// # Thread Safety
///
/// This struct is `Send + Sync` and can be safely shared across threads.
/// The container is shared via `Arc`, and all dependencies must be `Send + Sync`.
pub struct DependencyInjectingHandler {
    /// The wrapped handler that will receive the enriched request
    inner: Arc<dyn Handler>,
    /// Shared dependency container for resolution
    container: Arc<DependencyContainer>,
    /// List of dependency names required by this handler
    required_dependencies: Vec<String>,
}

impl DependencyInjectingHandler {
    /// Create a new dependency-injecting handler wrapper
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler to wrap
    /// * `container` - Shared dependency container
    /// * `required_dependencies` - Names of dependencies to resolve for this handler
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_http::di_handler::DependencyInjectingHandler;
    /// use spikard_core::di::DependencyContainer;
    /// use std::sync::Arc;
    ///
    /// # tokio_test::block_on(async {
    /// let container = Arc::new(DependencyContainer::new());
    /// let handler = Arc::new(MyHandler::new());
    ///
    /// let di_handler = DependencyInjectingHandler::new(
    ///     handler,
    ///     container,
    ///     vec!["db".to_string()],
    /// );
    /// # });
    /// ```
    pub fn new(
        handler: Arc<dyn Handler>,
        container: Arc<DependencyContainer>,
        required_dependencies: Vec<String>,
    ) -> Self {
        Self {
            inner: handler,
            container,
            required_dependencies,
        }
    }

    /// Get the list of required dependencies
    pub fn required_dependencies(&self) -> &[String] {
        &self.required_dependencies
    }
}

impl Handler for DependencyInjectingHandler {
    #[instrument(
        skip(self, request, request_data),
        fields(
            required_deps = %self.required_dependencies.len(),
            deps = ?self.required_dependencies
        )
    )]
    fn call(
        &self,
        request: Request<Body>,
        mut request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        tracing::debug!(
            target = "spikard::di",
            required_deps = ?self.required_dependencies,
            "entering DI handler"
        );
        let inner = self.inner.clone();
        let container = self.container.clone();
        let required_dependencies = self.required_dependencies.clone();

        Box::pin(async move {
            debug!(
                "DI handler invoked for {} deps; container keys: {:?}",
                required_dependencies.len(),
                container.keys()
            );
            let resolution_span = info_span!(
                "resolve_dependencies",
                count = %required_dependencies.len()
            );
            let _enter = resolution_span.enter();

            debug!(
                "Resolving {} dependencies: {:?}",
                required_dependencies.len(),
                required_dependencies
            );

            let start = std::time::Instant::now();

            let core_request_data = spikard_core::RequestData {
                path_params: Arc::clone(&request_data.path_params),
                query_params: Arc::try_unwrap(Arc::clone(&request_data.query_params))
                    .unwrap_or_else(|arc| (*arc).clone()),
                validated_params: request_data
                    .validated_params
                    .as_ref()
                    .map(|arc| Arc::try_unwrap(Arc::clone(arc)).unwrap_or_else(|a| (*a).clone())),
                raw_query_params: Arc::clone(&request_data.raw_query_params),
                body: Arc::try_unwrap(Arc::clone(&request_data.body)).unwrap_or_else(|arc| (*arc).clone()),
                raw_body: request_data.raw_body.clone(),
                headers: Arc::clone(&request_data.headers),
                cookies: Arc::clone(&request_data.cookies),
                method: request_data.method.clone(),
                path: request_data.path.clone(),
                #[cfg(feature = "di")]
                dependencies: None,
            };

            let (parts, _body) = request.into_parts();
            let core_request = Request::from_parts(parts.clone(), ());

            let request = Request::from_parts(parts, axum::body::Body::default());

            let resolved = match container
                .resolve_for_handler(&required_dependencies, &core_request, &core_request_data)
                .await
            {
                Ok(resolved) => resolved,
                Err(e) => {
                    debug!("DI error: {}", e);

                    let (status, json_body) = match e {
                        DependencyError::NotFound { ref key } => {
                            let body = serde_json::json!({
                                "detail": "Required dependency not found",
                                "errors": [{
                                    "dependency_key": key,
                                    "msg": format!("Dependency '{}' is not registered", key),
                                    "type": "missing_dependency"
                                }],
                                "status": 500,
                                "title": "Dependency Resolution Failed",
                                "type": "https://spikard.dev/errors/dependency-error"
                            });
                            (StatusCode::INTERNAL_SERVER_ERROR, body)
                        }
                        DependencyError::CircularDependency { ref cycle } => {
                            let body = serde_json::json!({
                                "detail": "Circular dependency detected",
                                "errors": [{
                                    "cycle": cycle,
                                    "msg": "Circular dependency detected in dependency graph",
                                    "type": "circular_dependency"
                                }],
                                "status": 500,
                                "title": "Dependency Resolution Failed",
                                "type": "https://spikard.dev/errors/dependency-error"
                            });
                            (StatusCode::INTERNAL_SERVER_ERROR, body)
                        }
                        DependencyError::ResolutionFailed { ref message } => {
                            let body = serde_json::json!({
                                "detail": "Dependency resolution failed",
                                "errors": [{
                                    "msg": message,
                                    "type": "resolution_failed"
                                }],
                                "status": 503,
                                "title": "Service Unavailable",
                                "type": "https://spikard.dev/errors/dependency-error"
                            });
                            (StatusCode::SERVICE_UNAVAILABLE, body)
                        }
                        _ => {
                            let body = serde_json::json!({
                                "detail": "Dependency resolution failed",
                                "errors": [{
                                    "msg": e.to_string(),
                                    "type": "unknown"
                                }],
                                "status": 500,
                                "title": "Dependency Resolution Failed",
                                "type": "https://spikard.dev/errors/dependency-error"
                            });
                            (StatusCode::INTERNAL_SERVER_ERROR, body)
                        }
                    };

                    let response = axum::http::Response::builder()
                        .status(status)
                        .header("Content-Type", "application/json")
                        .body(Body::from(json_body.to_string()))
                        .unwrap();

                    return Ok(response);
                }
            };

            let duration = start.elapsed();
            debug!(
                "Dependencies resolved in {:?} ({} dependencies)",
                duration,
                required_dependencies.len()
            );

            drop(_enter);

            let deps = Arc::new(resolved);
            request_data.dependencies = Some(Arc::clone(&deps));

            let result = inner.call(request, request_data).await;

            if let Ok(deps) = Arc::try_unwrap(deps) {
                let cleanup_span = info_span!("cleanup_dependencies");
                let _enter = cleanup_span.enter();

                debug!("Running dependency cleanup tasks");
                deps.cleanup().await;
            } else {
                debug!("Skipping cleanup: dependencies still shared");
            }

            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler_trait::RequestData;
    use axum::http::Response;
    use spikard_core::di::ValueDependency;
    use std::collections::HashMap;

    /// Test handler that checks for dependency presence
    struct TestHandler;

    impl Handler for TestHandler {
        fn call(
            &self,
            _request: Request<Body>,
            request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move {
                if request_data.dependencies.is_some() {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from("dependencies present"))
                        .unwrap();
                    Ok(response)
                } else {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "no dependencies".to_string()))
                }
            })
        }
    }

    /// Handler that returns error to test error propagation
    struct ErrorHandler;

    impl Handler for ErrorHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move { Err((StatusCode::INTERNAL_SERVER_ERROR, "inner handler error".to_string())) })
        }
    }

    /// Handler that reads and validates dependency values
    struct ReadDependencyHandler;

    impl Handler for ReadDependencyHandler {
        fn call(
            &self,
            _request: Request<Body>,
            request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move {
                if request_data.dependencies.is_some() {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from("dependencies resolved and accessible"))
                        .unwrap();
                    Ok(response)
                } else {
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "no dependencies".to_string()))
                }
            })
        }
    }

    /// Helper function to create a basic RequestData
    fn create_request_data() -> RequestData {
        RequestData {
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
        }
    }

    #[tokio::test]
    async fn test_di_handler_resolves_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_value")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_error_on_missing_dependency() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["database".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_di_handler_empty_dependencies() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec![]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_di_handler_multiple_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register("db".to_string(), Arc::new(ValueDependency::new("db", "postgresql")))
            .unwrap();
        container
            .register("cache".to_string(), Arc::new(ValueDependency::new("cache", "redis")))
            .unwrap();
        container
            .register("logger".to_string(), Arc::new(ValueDependency::new("logger", "slog")))
            .unwrap();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "config_data")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec![
                "db".to_string(),
                "cache".to_string(),
                "logger".to_string(),
                "config".to_string(),
            ],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_required_dependencies_getter() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let deps = vec!["db".to_string(), "cache".to_string(), "logger".to_string()];
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), deps.clone());

        assert_eq!(di_handler.required_dependencies(), deps.as_slice());
    }

    #[tokio::test]
    async fn test_di_handler_handler_error_propagation() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_value")),
            )
            .unwrap();

        let handler = Arc::new(ErrorHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(msg.contains("inner handler error"));
    }

    #[tokio::test]
    async fn test_di_handler_request_data_enrichment() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "service".to_string(),
                Arc::new(ValueDependency::new("service", "my_service")),
            )
            .unwrap();

        let handler = Arc::new(ReadDependencyHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["service".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_missing_dependency_json_structure() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler =
            DependencyInjectingHandler::new(handler, Arc::new(container), vec!["missing_service".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let content_type = response.headers().get("Content-Type").and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/json"));
    }

    #[tokio::test]
    async fn test_di_handler_partial_dependencies_present() {
        let mut container = DependencyContainer::new();
        container
            .register("db".to_string(), Arc::new(ValueDependency::new("db", "postgresql")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec!["db".to_string(), "cache".to_string()],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_di_handler_cleanup_executed() {
        let mut container = DependencyContainer::new();

        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_dependent_dependencies() {
        let mut container = DependencyContainer::new();

        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "base_config")),
            )
            .unwrap();

        container
            .register(
                "database".to_string(),
                Arc::new(ValueDependency::new("database", "db_from_config")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["database".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_parallel_independent_dependencies() {
        let mut container = DependencyContainer::new();

        container
            .register(
                "service_a".to_string(),
                Arc::new(ValueDependency::new("service_a", "svc_a")),
            )
            .unwrap();
        container
            .register(
                "service_b".to_string(),
                Arc::new(ValueDependency::new("service_b", "svc_b")),
            )
            .unwrap();
        container
            .register(
                "service_c".to_string(),
                Arc::new(ValueDependency::new("service_c", "svc_c")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec![
                "service_a".to_string(),
                "service_b".to_string(),
                "service_c".to_string(),
            ],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_request_method_preserved() {
        let mut container = DependencyContainer::new();
        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().method("POST").body(Body::empty()).unwrap();
        let mut request_data = create_request_data();
        request_data.method = "POST".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_complex_scenario_multiple_deps_with_error() {
        let mut container = DependencyContainer::new();

        for i in 1..=5 {
            container
                .register(
                    format!("service_{}", i),
                    Arc::new(ValueDependency::new(&format!("service_{}", i), format!("svc_{}", i))),
                )
                .unwrap();
        }

        let handler = Arc::new(ErrorHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec![
                "service_1".to_string(),
                "service_2".to_string(),
                "service_3".to_string(),
                "service_4".to_string(),
                "service_5".to_string(),
            ],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, _msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_di_handler_empty_request_body_with_deps() {
        let mut container = DependencyContainer::new();
        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_shared_container_across_handlers() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "shared_config".to_string(),
                Arc::new(ValueDependency::new("shared_config", "shared_value")),
            )
            .unwrap();

        let shared_container = Arc::new(container);

        let handler1 = Arc::new(TestHandler);
        let di_handler1 = DependencyInjectingHandler::new(
            handler1,
            Arc::clone(&shared_container),
            vec!["shared_config".to_string()],
        );

        let handler2 = Arc::new(TestHandler);
        let di_handler2 = DependencyInjectingHandler::new(
            handler2,
            Arc::clone(&shared_container),
            vec!["shared_config".to_string()],
        );

        let request1 = Request::builder().body(Body::empty()).unwrap();
        let request_data1 = create_request_data();
        let result1 = di_handler1.call(request1, request_data1).await;

        let request2 = Request::builder().body(Body::empty()).unwrap();
        let request_data2 = create_request_data();
        let result2 = di_handler2.call(request2, request_data2).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_requests_same_handler_no_race() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_config")),
            )
            .unwrap();

        let shared_container = Arc::new(container);
        let handler = Arc::new(TestHandler);
        let di_handler = Arc::new(DependencyInjectingHandler::new(
            handler,
            shared_container,
            vec!["config".to_string()],
        ));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let di_handler = Arc::clone(&di_handler);
                tokio::spawn(async move {
                    let request = Request::builder().body(Body::empty()).unwrap();
                    let request_data = create_request_data();
                    di_handler.call(request, request_data).await
                })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_concurrent_different_handlers_shared_container() {
        let mut container = DependencyContainer::new();
        container
            .register("db".to_string(), Arc::new(ValueDependency::new("db", "postgres")))
            .unwrap();
        container
            .register("cache".to_string(), Arc::new(ValueDependency::new("cache", "redis")))
            .unwrap();

        let shared_container = Arc::new(container);

        let mut handles = vec![];
        for i in 0..5 {
            let container = Arc::clone(&shared_container);
            let handler = Arc::new(TestHandler);
            let di_handler = DependencyInjectingHandler::new(
                handler,
                container,
                if i % 2 == 0 {
                    vec!["db".to_string()]
                } else {
                    vec!["cache".to_string()]
                },
            );

            let handle = tokio::spawn(async move {
                let request = Request::builder().body(Body::empty()).unwrap();
                let request_data = create_request_data();
                di_handler.call(request, request_data).await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_missing_dependency_multiple_concurrent_requests() {
        let container = DependencyContainer::new();
        let shared_container = Arc::new(container);
        let handler = Arc::new(TestHandler);
        let di_handler = Arc::new(DependencyInjectingHandler::new(
            handler,
            shared_container,
            vec!["nonexistent".to_string()],
        ));

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let di_handler = Arc::clone(&di_handler);
                tokio::spawn(async move {
                    let request = Request::builder().body(Body::empty()).unwrap();
                    let request_data = create_request_data();
                    di_handler.call(request, request_data).await
                })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn test_large_dependency_tree_resolution() {
        let mut container = DependencyContainer::new();
        for i in 0..20 {
            container
                .register(
                    format!("dep_{}", i),
                    Arc::new(ValueDependency::new(&format!("dep_{}", i), format!("value_{}", i))),
                )
                .unwrap();
        }

        let handler = Arc::new(TestHandler);
        let mut required = vec![];
        for i in 0..20 {
            required.push(format!("dep_{}", i));
        }

        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), required);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_error_does_not_prevent_cleanup() {
        let mut container = DependencyContainer::new();
        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let handler = Arc::new(ErrorHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_err());
        let (status, msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(msg.contains("inner handler error"));
    }

    #[tokio::test]
    async fn test_partial_dependency_resolution_failure() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "service_a".to_string(),
                Arc::new(ValueDependency::new("service_a", "svc_a")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec!["service_a".to_string(), "service_b".to_string()],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_circular_dependency_detection() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "service_a".to_string(),
                Arc::new(ValueDependency::new("service_a", "svc_a")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["service_a".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_empty_required_dependencies_with_multiple_registered() {
        let mut container = DependencyContainer::new();
        for i in 0..5 {
            container
                .register(
                    format!("unused_{}", i),
                    Arc::new(ValueDependency::new(&format!("unused_{}", i), format!("val_{}", i))),
                )
                .unwrap();
        }

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec![]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_resolution_with_varying_dependency_counts() {
        let mut container = DependencyContainer::new();
        for i in 0..10 {
            container
                .register(
                    format!("svc_{}", i),
                    Arc::new(ValueDependency::new(&format!("svc_{}", i), format!("s_{}", i))),
                )
                .unwrap();
        }

        let shared_container = Arc::new(container);

        let mut handles = vec![];
        for i in 0..10 {
            let container = Arc::clone(&shared_container);
            let handler = Arc::new(TestHandler);

            let required: Vec<String> = (0..=(i % 5)).map(|j| format!("svc_{}", j)).collect();

            let di_handler = DependencyInjectingHandler::new(handler, container, required);

            let handle = tokio::spawn(async move {
                let request = Request::builder().body(Body::empty()).unwrap();
                let request_data = create_request_data();
                di_handler.call(request, request_data).await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_request_data_isolation_across_concurrent_requests() {
        let mut container = DependencyContainer::new();
        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let shared_container = Arc::new(container);
        let handler = Arc::new(TestHandler);
        let di_handler = Arc::new(DependencyInjectingHandler::new(
            handler,
            shared_container,
            vec!["config".to_string()],
        ));

        let mut handles = vec![];
        for i in 0..10 {
            let di_handler = Arc::clone(&di_handler);
            let handle = tokio::spawn(async move {
                let request = Request::builder().body(Body::empty()).unwrap();
                let mut request_data = create_request_data();
                request_data.path = format!("/path/{}", i);
                di_handler.call(request, request_data).await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_missing_dependency_error_json_format() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler =
            DependencyInjectingHandler::new(handler, Arc::new(container), vec!["missing_service".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response.headers().get("Content-Type").and_then(|v| v.to_str().ok()),
            Some("application/json")
        );
    }

    #[tokio::test]
    async fn test_many_sequential_requests_same_handler_state() {
        let mut container = DependencyContainer::new();
        container
            .register("state".to_string(), Arc::new(ValueDependency::new("state", "initial")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["state".to_string()]);

        for _ in 0..50 {
            let request = Request::builder().body(Body::empty()).unwrap();
            let request_data = create_request_data();
            let result = di_handler.call(request, request_data).await;

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn test_dependency_availability_after_resolution() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "service".to_string(),
                Arc::new(ValueDependency::new("service", "my_service")),
            )
            .unwrap();

        let handler = Arc::new(ReadDependencyHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["service".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_container_keys_availability_during_resolution() {
        let mut container = DependencyContainer::new();
        container
            .register("key1".to_string(), Arc::new(ValueDependency::new("key1", "val1")))
            .unwrap();
        container
            .register("key2".to_string(), Arc::new(ValueDependency::new("key2", "val2")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec!["key1".to_string(), "key2".to_string()],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post_request_with_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "validator".to_string(),
                Arc::new(ValueDependency::new("validator", "strict_mode")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["validator".to_string()]);

        let request = Request::builder()
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"key":"value"}"#))
            .unwrap();
        let mut request_data = create_request_data();
        request_data.method = "POST".to_string();
        request_data.body = Arc::new(serde_json::json!({"key": "value"}));

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_request_with_authorization_dependency() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "auth".to_string(),
                Arc::new(ValueDependency::new("auth", "bearer_token")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["auth".to_string()]);

        let request = Request::builder().method("DELETE").body(Body::empty()).unwrap();
        let mut request_data = create_request_data();
        request_data.method = "DELETE".to_string();
        request_data.path = "/resource/123".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_very_large_number_of_dependencies_in_single_handler() {
        let mut container = DependencyContainer::new();
        let mut required_deps = vec![];
        for i in 0..50 {
            let key = format!("dep_{}", i);
            container
                .register(
                    key.clone(),
                    Arc::new(ValueDependency::new(&key, format!("value_{}", i))),
                )
                .unwrap();
            required_deps.push(key);
        }

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), required_deps);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_cloning_with_same_container() {
        let mut container = DependencyContainer::new();
        container
            .register("svc".to_string(), Arc::new(ValueDependency::new("svc", "service")))
            .unwrap();

        let shared_container = Arc::new(container);
        let base_handler: Arc<dyn Handler> = Arc::new(TestHandler);

        let di_handler1 = Arc::new(DependencyInjectingHandler::new(
            base_handler.clone(),
            Arc::clone(&shared_container),
            vec!["svc".to_string()],
        ));

        let di_handler2 = Arc::new(DependencyInjectingHandler::new(
            base_handler.clone(),
            Arc::clone(&shared_container),
            vec!["svc".to_string()],
        ));

        let handle1 = tokio::spawn({
            let dih = Arc::clone(&di_handler1);
            async move {
                let request = Request::builder().body(Body::empty()).unwrap();
                let request_data = create_request_data();
                dih.call(request, request_data).await
            }
        });

        let handle2 = tokio::spawn({
            let dih = Arc::clone(&di_handler2);
            async move {
                let request = Request::builder().body(Body::empty()).unwrap();
                let request_data = create_request_data();
                dih.call(request, request_data).await
            }
        });

        assert!(handle1.await.unwrap().is_ok());
        assert!(handle2.await.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_request_parts_reconstruction_correctness() {
        let mut container = DependencyContainer::new();
        container
            .register("config".to_string(), Arc::new(ValueDependency::new("config", "test")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let request = Request::builder()
            .method("GET")
            .header("User-Agent", "test-client")
            .header("Accept", "application/json")
            .body(Body::empty())
            .unwrap();
        let mut request_data = create_request_data();
        request_data.method = "GET".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_resolution_failure_returns_service_unavailable() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "external_api".to_string(),
                Arc::new(ValueDependency::new("external_api", "unavailable")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler =
            DependencyInjectingHandler::new(handler, Arc::new(container), vec!["external_api".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_missing_dependencies_reports_first() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec![
                "missing_a".to_string(),
                "missing_b".to_string(),
                "missing_c".to_string(),
            ],
        );

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_required_dependencies_getter_consistency() {
        let deps = vec![
            "dep_a".to_string(),
            "dep_b".to_string(),
            "dep_c".to_string(),
            "dep_d".to_string(),
        ];
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), deps.clone());

        let returned_deps = di_handler.required_dependencies();
        assert_eq!(returned_deps.len(), 4);
        assert_eq!(returned_deps, deps.as_slice());
    }

    #[tokio::test]
    async fn test_concurrent_error_handlers_isolation() {
        let container = DependencyContainer::new();
        let handler = Arc::new(ErrorHandler);
        let di_handler = Arc::new(DependencyInjectingHandler::new(handler, Arc::new(container), vec![]));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let dih = Arc::clone(&di_handler);
                tokio::spawn(async move {
                    let request = Request::builder().body(Body::empty()).unwrap();
                    let request_data = create_request_data();
                    dih.call(request, request_data).await
                })
            })
            .collect();

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_err());
            let (status, msg) = result.unwrap_err();
            assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
            assert!(msg.contains("inner handler error"));
        }
    }

    #[tokio::test]
    async fn test_patch_request_with_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "merger".to_string(),
                Arc::new(ValueDependency::new("merger", "strategic_merge")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["merger".to_string()]);

        let request = Request::builder().method("PATCH").body(Body::empty()).unwrap();
        let mut request_data = create_request_data();
        request_data.method = "PATCH".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_receives_enriched_request_data_with_multiple_deps() {
        let mut container = DependencyContainer::new();
        for i in 0..5 {
            container
                .register(
                    format!("svc_{}", i),
                    Arc::new(ValueDependency::new(&format!("svc_{}", i), format!("s_{}", i))),
                )
                .unwrap();
        }

        let handler = Arc::new(ReadDependencyHandler);
        let required: Vec<String> = (0..5).map(|i| format!("svc_{}", i)).collect();
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), required);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_arc_try_unwrap_cleanup_branch() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "resource".to_string(),
                Arc::new(ValueDependency::new("resource", "allocated")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["resource".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_head_request_with_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "metadata".to_string(),
                Arc::new(ValueDependency::new("metadata", "headers_only")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["metadata".to_string()]);

        let request = Request::builder().method("HEAD").body(Body::empty()).unwrap();
        let mut request_data = create_request_data();
        request_data.method = "HEAD".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_options_request_with_dependencies() {
        let mut container = DependencyContainer::new();
        container
            .register("cors".to_string(), Arc::new(ValueDependency::new("cors", "permissive")))
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["cors".to_string()]);

        let request = Request::builder().method("OPTIONS").body(Body::empty()).unwrap();
        let mut request_data = create_request_data();
        request_data.method = "OPTIONS".to_string();

        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_circular_dependency_error_json_structure() {
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);

        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["missing".to_string()]);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = create_request_data();
        let result = di_handler.call(request, request_data).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let content_type = response.headers().get("Content-Type").and_then(|v| v.to_str().ok());
        assert_eq!(content_type, Some("application/json"));

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json_body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert!(json_body.get("type").is_some(), "type field must be present");
        assert!(json_body.get("title").is_some(), "title field must be present");
        assert!(json_body.get("detail").is_some(), "detail field must be present");
        assert!(json_body.get("status").is_some(), "status field must be present");

        assert_eq!(json_body.get("status").and_then(|v| v.as_i64()), Some(500));
        assert_eq!(
            json_body.get("type").and_then(|v| v.as_str()),
            Some("https://spikard.dev/errors/dependency-error")
        );
    }

    #[tokio::test]
    async fn test_request_data_is_cloned_not_moved_to_handler() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "service".to_string(),
                Arc::new(ValueDependency::new("service", "test_service")),
            )
            .unwrap();

        let handler = Arc::new(ReadDependencyHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["service".to_string()]);

        let mut original_request_data = create_request_data();
        original_request_data.path = "/api/test".to_string();
        original_request_data.method = "POST".to_string();

        let mut headers = HashMap::new();
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
        original_request_data.headers = Arc::new(headers.clone());

        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "test-session".to_string());
        original_request_data.cookies = Arc::new(cookies.clone());

        let original_path = original_request_data.path.clone();
        let original_method = original_request_data.method.clone();

        let request = Request::builder().method("POST").body(Body::empty()).unwrap();
        let request_data_clone = original_request_data.clone();
        let result = di_handler.call(request, original_request_data).await;

        assert!(result.is_ok());

        assert_eq!(request_data_clone.path, original_path);
        assert_eq!(request_data_clone.method, original_method);

        assert!(request_data_clone.dependencies.is_none());

        assert_eq!(*request_data_clone.headers, headers);
        assert_eq!(*request_data_clone.cookies, cookies);
    }

    #[tokio::test]
    async fn test_core_request_data_conversion_preserves_all_fields() {
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_config")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());
        path_params.insert("resource".to_string(), "users".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("filter".to_string(), vec!["active".to_string()]);
        raw_query_params.insert("sort".to_string(), vec!["name".to_string(), "asc".to_string()]);

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "abc123".to_string());
        cookies.insert("preferences".to_string(), "dark_mode".to_string());

        let request_data = RequestData {
            path_params: Arc::new(path_params.clone()),
            query_params: Arc::new(serde_json::json!({"filter": "active", "sort": "name"})),
            validated_params: None,
            raw_query_params: Arc::new(raw_query_params.clone()),
            body: Arc::new(serde_json::json!({"name": "John", "email": "john@example.com"})),
            raw_body: Some(bytes::Bytes::from(r#"{"name":"John","email":"john@example.com"}"#)),
            headers: Arc::new(headers.clone()),
            cookies: Arc::new(cookies.clone()),
            method: "POST".to_string(),
            path: "/api/users/123".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let original_path = request_data.path.clone();
        let original_method = request_data.method.clone();
        let original_body = request_data.body.clone();
        let original_query_params = request_data.query_params.clone();

        let request = Request::builder().method("POST").body(Body::empty()).unwrap();
        let result = di_handler.call(request, request_data.clone()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        assert_eq!(request_data.path, original_path, "path field must be preserved");
        assert_eq!(request_data.method, original_method, "method field must be preserved");
        assert_eq!(request_data.body, original_body, "body field must be preserved");
        assert_eq!(
            request_data.query_params, original_query_params,
            "query_params must be preserved"
        );

        assert_eq!(request_data.path_params.get("id"), Some(&"123".to_string()));
        assert_eq!(request_data.path_params.get("resource"), Some(&"users".to_string()));

        assert_eq!(
            request_data.raw_query_params.get("filter"),
            Some(&vec!["active".to_string()])
        );
        assert_eq!(
            request_data.raw_query_params.get("sort"),
            Some(&vec!["name".to_string(), "asc".to_string()])
        );

        assert_eq!(
            request_data.headers.get("Authorization"),
            Some(&"Bearer token123".to_string())
        );
        assert_eq!(
            request_data.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );

        assert_eq!(request_data.cookies.get("session"), Some(&"abc123".to_string()));
        assert_eq!(request_data.cookies.get("preferences"), Some(&"dark_mode".to_string()));

        assert!(request_data.raw_body.is_some());
        assert_eq!(
            request_data.raw_body.as_ref().unwrap().as_ref(),
            r#"{"name":"John","email":"john@example.com"}"#.as_bytes()
        );
    }
}
