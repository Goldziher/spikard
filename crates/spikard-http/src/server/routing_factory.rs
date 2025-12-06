//! Method routing factory for consolidating HTTP method handler creation.
//!
//! This module provides a factory pattern to eliminate duplication in the
//! create_method_router function. It handles:
//! - Different HTTP methods (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, TRACE)
//! - Both path variants (with and without path parameters)
//! - Handler wrapping and request data extraction
//! - Lifecycle hook integration

use crate::handler_trait::Handler;
use axum::body::Body;
use axum::extract::{Path, Request as AxumRequest};
use axum::http::Request;
use axum::routing::MethodRouter;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;

use super::lifecycle_execution;
use super::request_extraction;

/// HTTP method type enumeration for routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Trace,
    Options,
}

impl HttpMethod {
    /// Parse from string representation (e.g., "GET", "POST")
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PUT" => Some(HttpMethod::Put),
            "PATCH" => Some(HttpMethod::Patch),
            "DELETE" => Some(HttpMethod::Delete),
            "HEAD" => Some(HttpMethod::Head),
            "TRACE" => Some(HttpMethod::Trace),
            "OPTIONS" => Some(HttpMethod::Options),
            _ => None,
        }
    }

    /// Check if this method typically has a request body
    pub fn expects_body(&self) -> bool {
        matches!(self, HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch)
    }
}

/// Factory for creating method routers
pub struct MethodRouterFactory;

impl MethodRouterFactory {
    /// Create a method router for the given HTTP method
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method string (e.g., "GET", "POST")
    /// * `has_path_params` - Whether the route has path parameters
    /// * `handler` - The request handler
    /// * `hooks` - Optional lifecycle hooks
    ///
    /// # Returns
    ///
    /// A configured MethodRouter or an error if the method is unsupported
    pub fn create(
        method: &str,
        has_path_params: bool,
        handler: Arc<dyn Handler>,
        hooks: Option<Arc<crate::LifecycleHooks>>,
    ) -> Result<MethodRouter, String> {
        let http_method = HttpMethod::from_str(method)
            .ok_or_else(|| format!("[spikard-router] unsupported HTTP method: {}", method))?;

        Ok(if has_path_params {
            Self::create_with_path_params(http_method, handler, hooks)
        } else {
            Self::create_without_path_params(http_method, handler, hooks)
        })
    }

    /// Create a method router for a route with path parameters
    fn create_with_path_params(
        method: HttpMethod,
        handler: Arc<dyn Handler>,
        hooks: Option<Arc<crate::LifecycleHooks>>,
    ) -> MethodRouter {
        let handler_clone = handler.clone();
        let hooks_clone = hooks.clone();

        if method.expects_body() {
            match method {
                HttpMethod::Post => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::post(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let (parts, body) = req.into_parts();
                                let request_data =
                                    request_extraction::create_request_data_with_body(&parts, path_params.0, body)
                                        .await?;
                                let mut req = Request::from_parts(
                                    parts,
                                    Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                                );
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Put => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::put(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let (parts, body) = req.into_parts();
                                let request_data =
                                    request_extraction::create_request_data_with_body(&parts, path_params.0, body)
                                        .await?;
                                let mut req = Request::from_parts(
                                    parts,
                                    Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                                );
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Patch => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::patch(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let (parts, body) = req.into_parts();
                                let request_data =
                                    request_extraction::create_request_data_with_body(&parts, path_params.0, body)
                                        .await?;
                                let mut req = Request::from_parts(
                                    parts,
                                    Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                                );
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                _ => MethodRouter::new(),
            }
        } else {
            match method {
                HttpMethod::Get => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::get(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let request_data = request_extraction::create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                let mut req = req;
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Delete => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::delete(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let request_data = request_extraction::create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                let mut req = req;
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Head => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::head(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let request_data = request_extraction::create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                let mut req = req;
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Trace => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::trace(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let request_data = request_extraction::create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                let mut req = req;
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                HttpMethod::Options => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::options(
                        move |path_params: Path<HashMap<String, String>>, req: AxumRequest| {
                            let handler = handler_clone.clone();
                            let hooks = hooks_clone.clone();
                            async move {
                                let request_data = request_extraction::create_request_data_without_body(
                                    req.uri(),
                                    req.method(),
                                    req.headers(),
                                    path_params.0,
                                );
                                let mut req = req;
                                req.extensions_mut().insert(Arc::new(request_data.clone()));
                                lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                    .await
                            }
                        },
                    )
                }
                _ => MethodRouter::new(),
            }
        }
    }

    /// Create a method router for a route without path parameters
    fn create_without_path_params(
        method: HttpMethod,
        handler: Arc<dyn Handler>,
        hooks: Option<Arc<crate::LifecycleHooks>>,
    ) -> MethodRouter {
        let handler_clone = handler.clone();
        let hooks_clone = hooks.clone();

        if method.expects_body() {
            match method {
                HttpMethod::Post => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::post(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let (parts, body) = req.into_parts();
                            let request_data =
                                request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let mut req = Request::from_parts(
                                parts,
                                Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                            );
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Put => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::put(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let (parts, body) = req.into_parts();
                            let request_data =
                                request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let mut req = Request::from_parts(
                                parts,
                                Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                            );
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Patch => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::patch(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let (parts, body) = req.into_parts();
                            let request_data =
                                request_extraction::create_request_data_with_body(&parts, HashMap::new(), body).await?;
                            let mut req = Request::from_parts(
                                parts,
                                Body::from(request_data.raw_body.clone().unwrap_or_else(Bytes::new)),
                            );
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                _ => MethodRouter::new(),
            }
        } else {
            match method {
                HttpMethod::Get => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::get(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let request_data = request_extraction::create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            let mut req = req;
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Delete => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::delete(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let request_data = request_extraction::create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            let mut req = req;
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Head => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::head(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let request_data = request_extraction::create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            let mut req = req;
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Trace => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::trace(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let request_data = request_extraction::create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            let mut req = req;
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                HttpMethod::Options => {
                    let handler_clone = handler_clone.clone();
                    let hooks_clone = hooks_clone.clone();
                    axum::routing::options(move |req: AxumRequest| {
                        let handler = handler_clone.clone();
                        let hooks = hooks_clone.clone();
                        async move {
                            let request_data = request_extraction::create_request_data_without_body(
                                req.uri(),
                                req.method(),
                                req.headers(),
                                HashMap::new(),
                            );
                            let mut req = req;
                            req.extensions_mut().insert(Arc::new(request_data.clone()));
                            lifecycle_execution::execute_with_lifecycle_hooks(req, request_data, handler, hooks)
                                .await
                        }
                    })
                }
                _ => MethodRouter::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_from_str() {
        assert_eq!(HttpMethod::from_str("GET"), Some(HttpMethod::Get));
        assert_eq!(HttpMethod::from_str("POST"), Some(HttpMethod::Post));
        assert_eq!(HttpMethod::from_str("PUT"), Some(HttpMethod::Put));
        assert_eq!(HttpMethod::from_str("PATCH"), Some(HttpMethod::Patch));
        assert_eq!(HttpMethod::from_str("DELETE"), Some(HttpMethod::Delete));
        assert_eq!(HttpMethod::from_str("HEAD"), Some(HttpMethod::Head));
        assert_eq!(HttpMethod::from_str("TRACE"), Some(HttpMethod::Trace));
        assert_eq!(HttpMethod::from_str("OPTIONS"), Some(HttpMethod::Options));
        assert_eq!(HttpMethod::from_str("INVALID"), None);
    }

    #[test]
    fn test_http_method_expects_body() {
        assert!(!HttpMethod::Get.expects_body());
        assert!(HttpMethod::Post.expects_body());
        assert!(HttpMethod::Put.expects_body());
        assert!(HttpMethod::Patch.expects_body());
        assert!(!HttpMethod::Delete.expects_body());
        assert!(!HttpMethod::Head.expects_body());
        assert!(!HttpMethod::Trace.expects_body());
        assert!(!HttpMethod::Options.expects_body());
    }
}
