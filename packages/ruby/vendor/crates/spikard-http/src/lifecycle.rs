use axum::{
    body::Body,
    http::{Request, Response},
};
use std::sync::Arc;

pub mod adapter;

pub use spikard_core::lifecycle::{HookResult, LifecycleHook};

pub type LifecycleHooks = spikard_core::lifecycle::LifecycleHooks<Request<Body>, Response<Body>>;
pub type LifecycleHooksBuilder = spikard_core::lifecycle::LifecycleHooksBuilder<Request<Body>, Response<Body>>;

/// Create a request hook for the current target.
#[cfg(not(target_arch = "wasm32"))]
pub fn request_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>
where
    F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'static,
{
    spikard_core::lifecycle::request_hook::<Request<Body>, Response<Body>, _, _>(name, func)
}

/// Create a request hook for wasm targets (no Send on futures).
#[cfg(target_arch = "wasm32")]
pub fn request_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>
where
    F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + 'static,
{
    spikard_core::lifecycle::request_hook::<Request<Body>, Response<Body>, _, _>(name, func)
}

/// Create a response hook for the current target.
#[cfg(not(target_arch = "wasm32"))]
pub fn response_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>
where
    F: Fn(Response<Body>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'static,
{
    spikard_core::lifecycle::response_hook::<Request<Body>, Response<Body>, _, _>(name, func)
}

/// Create a response hook for wasm targets (no Send on futures).
#[cfg(target_arch = "wasm32")]
pub fn response_hook<F, Fut>(name: impl Into<String>, func: F) -> Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>
where
    F: Fn(Response<Body>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + 'static,
{
    spikard_core::lifecycle::response_hook::<Request<Body>, Response<Body>, _, _>(name, func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Response, StatusCode};
    use std::future::Future;
    use std::pin::Pin;

    /// Test hook that always continues
    struct ContinueHook {
        name: String,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for ContinueHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(req)) })
        }

        fn execute_response<'a>(
            &'a self,
            resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(resp)) })
        }
    }

    /// Test hook that short-circuits with a 401 response
    struct ShortCircuitHook {
        name: String,
    }

    impl LifecycleHook<Request<Body>, Response<Body>> for ShortCircuitHook {
        fn name(&self) -> &str {
            &self.name
        }

        fn execute_request<'a>(
            &'a self,
            _req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            })
        }

        fn execute_response<'a>(
            &'a self,
            _resp: Response<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            })
        }
    }

    #[tokio::test]
    async fn test_empty_hooks_fast_path() {
        let hooks = LifecycleHooks::new();
        assert!(hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_on_request_continue() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "test".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_on_request_short_circuit() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_request(Arc::new(ShortCircuitHook {
            name: "auth_check".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit, got Continue"),
        }
    }

    #[tokio::test]
    async fn test_multiple_hooks_in_order() {
        let mut hooks = LifecycleHooks::new();

        hooks.add_on_request(Arc::new(ContinueHook {
            name: "first".to_string(),
        }));
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "second".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_short_circuit_stops_execution() {
        let mut hooks = LifecycleHooks::new();

        hooks.add_on_request(Arc::new(ShortCircuitHook {
            name: "short_circuit".to_string(),
        }));
        hooks.add_on_request(Arc::new(ContinueHook {
            name: "never_executed".to_string(),
        }));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(_) => {}
            HookResult::Continue(_) => panic!("Expected ShortCircuit, got Continue"),
        }
    }

    #[tokio::test]
    async fn test_on_response_hooks() {
        let mut hooks = LifecycleHooks::new();
        hooks.add_on_response(Arc::new(ContinueHook {
            name: "response_hook".to_string(),
        }));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_request_hook_builder() {
        let hook = request_hook("test", |req| async move { Ok(HookResult::Continue(req)) });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_request_hook_with_modification() {
        let hook = request_hook("add_header", |mut req| async move {
            req.headers_mut()
                .insert("X-Custom-Header", axum::http::HeaderValue::from_static("test-value"));
            Ok(HookResult::Continue(req))
        });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Custom-Header").unwrap(), "test-value");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_request_hook_short_circuit() {
        let hook = request_hook("auth", |_req| async move {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        });

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hook.execute_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_response_hook_builder() {
        let hook = response_hook("security", |mut resp| async move {
            resp.headers_mut()
                .insert("X-Frame-Options", axum::http::HeaderValue::from_static("DENY"));
            Ok(HookResult::Continue(resp))
        });

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hook.execute_response(resp).await.unwrap();

        match result {
            HookResult::Continue(resp) => {
                assert_eq!(resp.headers().get("X-Frame-Options").unwrap(), "DENY");
                assert_eq!(resp.status(), StatusCode::OK);
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook(
                "logger",
                |req| async move { Ok(HookResult::Continue(req)) },
            ))
            .pre_handler(request_hook("auth", |req| async move { Ok(HookResult::Continue(req)) }))
            .on_response(response_hook("security", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .build();

        assert!(!hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }

    #[tokio::test]
    async fn test_builder_with_multiple_hooks() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("first", |mut req| async move {
                req.headers_mut()
                    .insert("X-First", axum::http::HeaderValue::from_static("1"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("second", |mut req| async move {
                req.headers_mut()
                    .insert("X-Second", axum::http::HeaderValue::from_static("2"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-First").unwrap(), "1");
                assert_eq!(req.headers().get("X-Second").unwrap(), "2");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_builder_short_circuit_stops_chain() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook(
                "first",
                |req| async move { Ok(HookResult::Continue(req)) },
            ))
            .on_request(request_hook("short_circuit", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from("Blocked"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .on_request(request_hook("never_called", |mut req| async move {
                req.headers_mut()
                    .insert("X-Should-Not-Exist", axum::http::HeaderValue::from_static("value"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::FORBIDDEN);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_all_hook_types() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("on_request", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .pre_validation(request_hook("pre_validation", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .pre_handler(request_hook("pre_handler", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .on_response(response_hook("on_response", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .on_error(response_hook("on_error", |resp| async move {
                Ok(HookResult::Continue(resp))
            }))
            .build();

        assert!(!hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_on_request(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_pre_validation(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let req = Request::builder().body(Body::empty()).unwrap();
        assert!(matches!(
            hooks.execute_pre_handler(req).await.unwrap(),
            HookResult::Continue(_)
        ));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let result = hooks.execute_on_response(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);

        let resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();
        let result = hooks.execute_on_error(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_empty_builder() {
        let hooks = LifecycleHooks::builder().build();
        assert!(hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));
    }
}
