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
        eprintln!(
            "[spikard-di] entering DI handler, required_deps={:?}",
            self.required_dependencies
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
            // Span for dependency resolution timing
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

            // Convert RequestData to spikard_core::RequestData for DI
            let core_request_data = spikard_core::RequestData {
                path_params: Arc::clone(&request_data.path_params),
                query_params: request_data.query_params.clone(),
                raw_query_params: Arc::clone(&request_data.raw_query_params),
                body: request_data.body.clone(),
                raw_body: request_data.raw_body.clone(),
                headers: Arc::clone(&request_data.headers),
                cookies: Arc::clone(&request_data.cookies),
                method: request_data.method.clone(),
                path: request_data.path.clone(),
                #[cfg(feature = "di")]
                dependencies: None,
            };

            // Convert Request<Body> to Request<()> for DI (body not needed for resolution)
            let (parts, _body) = request.into_parts();
            let core_request = Request::from_parts(parts.clone(), ());

            // Restore original request for handler
            let request = Request::from_parts(parts, axum::body::Body::default());

            // Resolve dependencies in parallel batches
            let resolved = match container
                .resolve_for_handler(&required_dependencies, &core_request, &core_request_data)
                .await
            {
                Ok(resolved) => resolved,
                Err(e) => {
                    debug!("DI error: {}", e);

                    // Convert DI errors to proper JSON HTTP responses
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

                    // Return JSON error response
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

            // Attach resolved dependencies to request_data
            request_data.dependencies = Some(Arc::new(resolved));

            // Call the inner handler with enriched request data
            let result = inner.call(request, request_data.clone()).await;

            // Cleanup: Execute cleanup tasks after handler completes
            // This implements the async Drop pattern for generator-style dependencies
            if let Some(deps) = request_data.dependencies.take() {
                // Try to get exclusive ownership for cleanup
                if let Ok(deps) = Arc::try_unwrap(deps) {
                    let cleanup_span = info_span!("cleanup_dependencies");
                    let _enter = cleanup_span.enter();

                    debug!("Running dependency cleanup tasks");
                    deps.cleanup().await;
                } else {
                    // Dependencies are still shared (shouldn't happen in normal flow)
                    debug!("Skipping cleanup: dependencies still shared");
                }
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
                // Verify dependencies are present
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

    #[tokio::test]
    async fn test_di_handler_resolves_dependencies() {
        // Setup
        let mut container = DependencyContainer::new();
        container
            .register(
                "config".to_string(),
                Arc::new(ValueDependency::new("config", "test_value")),
            )
            .unwrap();

        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["config".to_string()]);

        // Execute
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: serde_json::Value::Null,
            raw_query_params: Arc::new(HashMap::new()),
            body: serde_json::Value::Null,
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = di_handler.call(request, request_data).await;

        // Verify
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_di_handler_error_on_missing_dependency() {
        // Setup: empty container, but handler requires "database"
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(handler, Arc::new(container), vec!["database".to_string()]);

        // Execute
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: serde_json::Value::Null,
            raw_query_params: Arc::new(HashMap::new()),
            body: serde_json::Value::Null,
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = di_handler.call(request, request_data).await;

        // Verify: should return structured error response
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_di_handler_empty_dependencies() {
        // Setup: no dependencies required
        let container = DependencyContainer::new();
        let handler = Arc::new(TestHandler);
        let di_handler = DependencyInjectingHandler::new(
            handler,
            Arc::new(container),
            vec![], // No dependencies
        );

        // Execute
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: serde_json::Value::Null,
            raw_query_params: Arc::new(HashMap::new()),
            body: serde_json::Value::Null,
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let result = di_handler.call(request, request_data).await;

        // Verify: should succeed even with empty dependencies
        assert!(result.is_ok());
    }
}
