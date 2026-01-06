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
            &self,
            req: Request<Body>,
        ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>>
        {
            Box::pin(async move { Ok(HookResult::Continue(req)) })
        }

        fn execute_response<'a>(
            &self,
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
            &self,
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
            &self,
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

    #[tokio::test]
    async fn test_hook_chaining_modifies_request_sequentially() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("add_header_1", |mut req| async move {
                req.headers_mut()
                    .insert("X-Chain-1", axum::http::HeaderValue::from_static("first"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("add_header_2", |mut req| async move {
                req.headers_mut()
                    .insert("X-Chain-2", axum::http::HeaderValue::from_static("second"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("add_header_3", |mut req| async move {
                req.headers_mut()
                    .insert("X-Chain-3", axum::http::HeaderValue::from_static("third"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Chain-1").unwrap(), "first");
                assert_eq!(req.headers().get("X-Chain-2").unwrap(), "second");
                assert_eq!(req.headers().get("X-Chain-3").unwrap(), "third");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_response_hook_chaining_modifies_status_and_headers() {
        let hooks = LifecycleHooks::builder()
            .on_response(response_hook("add_security_header", |mut resp| async move {
                resp.headers_mut().insert(
                    "X-Content-Type-Options",
                    axum::http::HeaderValue::from_static("nosniff"),
                );
                Ok(HookResult::Continue(resp))
            }))
            .on_response(response_hook("add_cache_header", |mut resp| async move {
                resp.headers_mut()
                    .insert("Cache-Control", axum::http::HeaderValue::from_static("no-cache"));
                Ok(HookResult::Continue(resp))
            }))
            .on_response(response_hook("add_custom_header", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Custom", axum::http::HeaderValue::from_static("value"));
                Ok(HookResult::Continue(resp))
            }))
            .build();

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await.unwrap();

        assert_eq!(result.status(), StatusCode::OK);
        assert_eq!(result.headers().get("X-Content-Type-Options").unwrap(), "nosniff");
        assert_eq!(result.headers().get("Cache-Control").unwrap(), "no-cache");
        assert_eq!(result.headers().get("X-Custom").unwrap(), "value");
    }

    #[tokio::test]
    async fn test_pre_validation_and_pre_handler_chaining() {
        let hooks = LifecycleHooks::builder()
            .pre_validation(request_hook("validate_auth", |mut req| async move {
                req.headers_mut()
                    .insert("X-Validated", axum::http::HeaderValue::from_static("true"));
                Ok(HookResult::Continue(req))
            }))
            .pre_handler(request_hook("prepare_handler", |mut req| async move {
                req.headers_mut()
                    .insert("X-Prepared", axum::http::HeaderValue::from_static("true"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Validated").unwrap(), "true");
                assert!(!req.headers().contains_key("X-Prepared"));
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }

        let req = Request::builder()
            .header("X-Validated", "true")
            .body(Body::empty())
            .unwrap();
        let result = hooks.execute_pre_handler(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Prepared").unwrap(), "true");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_hook_chain_with_state_passing() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("add_user_id", |mut req| async move {
                req.headers_mut()
                    .insert("X-User-ID", axum::http::HeaderValue::from_static("123"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("add_session_id", |mut req| async move {
                if let Some(user_id) = req.headers().get("X-User-ID") {
                    if user_id == "123" {
                        req.headers_mut()
                            .insert("X-Session-ID", axum::http::HeaderValue::from_static("session_abc"));
                    }
                }
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-User-ID").unwrap(), "123");
                assert_eq!(req.headers().get("X-Session-ID").unwrap(), "session_abc");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_pre_validation_short_circuit_stops_subsequent_hooks() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("on_request", |req| async move {
                println!("on_request executed");
                Ok(HookResult::Continue(req))
            }))
            .pre_validation(request_hook("pre_validation_abort", |_req| async move {
                println!("pre_validation executed - short circuiting");
                let response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Validation failed"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .pre_handler(request_hook("pre_handler", |req| async move {
                println!("pre_handler executed - should NOT happen");
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_pre_handler_short_circuit_returns_early_response() {
        let hooks = LifecycleHooks::builder()
            .pre_validation(request_hook("pre_validation", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .pre_handler(request_hook("rate_limit_check", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(Body::from("Rate limit exceeded"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_handler(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_short_circuit_in_middle_of_chain() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("hook_1", |mut req| async move {
                req.headers_mut()
                    .insert("X-Executed-1", axum::http::HeaderValue::from_static("yes"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("hook_2_abort", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from("Access denied"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .on_request(request_hook("hook_3", |mut req| async move {
                req.headers_mut()
                    .insert("X-Executed-3", axum::http::HeaderValue::from_static("yes"));
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
    async fn test_short_circuit_with_custom_response_headers() {
        let hooks = LifecycleHooks::builder()
            .pre_validation(request_hook("auth_check", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("WWW-Authenticate", "Bearer realm=\"api\"")
                    .body(Body::from("Authorization required"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
                assert_eq!(resp.headers().get("WWW-Authenticate").unwrap(), "Bearer realm=\"api\"");
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit"),
        }
    }

    #[tokio::test]
    async fn test_hook_error_propagates_through_chain() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("good_hook", |mut req| async move {
                req.headers_mut()
                    .insert("X-Good", axum::http::HeaderValue::from_static("yes"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("bad_hook", |_req| async move {
                Err("Something went wrong in hook".to_string())
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Something went wrong in hook");
    }

    #[tokio::test]
    async fn test_error_in_pre_validation_stops_chain() {
        let hooks = LifecycleHooks::builder()
            .pre_validation(request_hook("validation_hook", |_req| async move {
                Err("Validation error: invalid input".to_string())
            }))
            .pre_handler(request_hook("handler_prep", |req| async move {
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Validation error"));
    }

    #[tokio::test]
    async fn test_on_error_hook_transforms_response() {
        let hooks = LifecycleHooks::builder()
            .on_error(response_hook("transform_error", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Error-Handled", axum::http::HeaderValue::from_static("true"));

                let _status = resp.status();
                Ok(HookResult::Continue(resp))
            }))
            .build();

        let resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();

        let result = hooks.execute_on_error(resp).await.unwrap();

        assert_eq!(result.headers().get("X-Error-Handled").unwrap(), "true");
    }

    #[tokio::test]
    async fn test_response_hook_error_propagates() {
        let hooks = LifecycleHooks::builder()
            .on_response(response_hook("good_response_hook", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Processed", axum::http::HeaderValue::from_static("yes"));
                Ok(HookResult::Continue(resp))
            }))
            .on_response(response_hook("bad_response_hook", |_resp| async move {
                Err("Error processing response".to_string())
            }))
            .build();

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error processing response");
    }

    #[tokio::test]
    async fn test_error_hook_error_propagates() {
        let hooks = LifecycleHooks::builder()
            .on_error(response_hook("error_hook_1", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Error-Processed", axum::http::HeaderValue::from_static("1"));
                Ok(HookResult::Continue(resp))
            }))
            .on_error(response_hook("error_hook_2_fails", |_resp| async move {
                Err("Error in error hook".to_string())
            }))
            .build();

        let resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();

        let result = hooks.execute_on_error(resp).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error in error hook");
    }

    #[tokio::test]
    async fn test_on_request_adds_multiple_headers() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("add_request_headers", |mut req| async move {
                req.headers_mut()
                    .insert("X-Request-ID", axum::http::HeaderValue::from_static("req_123"));
                req.headers_mut()
                    .insert("X-Timestamp", axum::http::HeaderValue::from_static("2025-01-01"));
                req.headers_mut()
                    .insert("X-Processed", axum::http::HeaderValue::from_static("true"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Request-ID").unwrap(), "req_123");
                assert_eq!(req.headers().get("X-Timestamp").unwrap(), "2025-01-01");
                assert_eq!(req.headers().get("X-Processed").unwrap(), "true");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_on_response_adds_security_headers() {
        let hooks = LifecycleHooks::builder()
            .on_response(response_hook("add_security_headers", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Frame-Options", axum::http::HeaderValue::from_static("DENY"));
                resp.headers_mut().insert(
                    "X-Content-Type-Options",
                    axum::http::HeaderValue::from_static("nosniff"),
                );
                resp.headers_mut().insert(
                    "Strict-Transport-Security",
                    axum::http::HeaderValue::from_static("max-age=31536000"),
                );
                Ok(HookResult::Continue(resp))
            }))
            .build();

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await.unwrap();

        assert_eq!(result.headers().get("X-Frame-Options").unwrap(), "DENY");
        assert_eq!(result.headers().get("X-Content-Type-Options").unwrap(), "nosniff");
        assert_eq!(
            result.headers().get("Strict-Transport-Security").unwrap(),
            "max-age=31536000"
        );
    }

    #[tokio::test]
    async fn test_pre_handler_modifies_request_before_execution() {
        let hooks = LifecycleHooks::builder()
            .pre_handler(request_hook("inject_context", |mut req| async move {
                req.headers_mut().insert(
                    "X-Handler-Context",
                    axum::http::HeaderValue::from_static("context_data"),
                );
                req.headers_mut()
                    .insert("X-Injected", axum::http::HeaderValue::from_static("true"));
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_handler(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Handler-Context").unwrap(), "context_data");
                assert_eq!(req.headers().get("X-Injected").unwrap(), "true");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_register_multiple_hooks_different_types() {
        let mut hooks = LifecycleHooks::new();

        hooks.add_on_request(request_hook("on_request_1", |req| async move {
            Ok(HookResult::Continue(req))
        }));

        hooks.add_pre_validation(request_hook("pre_validation_1", |req| async move {
            Ok(HookResult::Continue(req))
        }));

        hooks.add_pre_handler(request_hook("pre_handler_1", |req| async move {
            Ok(HookResult::Continue(req))
        }));

        hooks.add_on_response(response_hook("on_response_1", |resp| async move {
            Ok(HookResult::Continue(resp))
        }));

        hooks.add_on_error(response_hook("on_error_1", |resp| async move {
            Ok(HookResult::Continue(resp))
        }));

        assert!(!hooks.is_empty());
    }

    #[tokio::test]
    async fn test_builder_composition_with_request_and_response_hooks() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("req_1", |mut req| async move {
                req.headers_mut()
                    .insert("X-R1", axum::http::HeaderValue::from_static("1"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("req_2", |mut req| async move {
                req.headers_mut()
                    .insert("X-R2", axum::http::HeaderValue::from_static("2"));
                Ok(HookResult::Continue(req))
            }))
            .on_response(response_hook("resp_1", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Resp1", axum::http::HeaderValue::from_static("resp1"));
                Ok(HookResult::Continue(resp))
            }))
            .on_response(response_hook("resp_2", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Resp2", axum::http::HeaderValue::from_static("resp2"));
                Ok(HookResult::Continue(resp))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let req_result = hooks.execute_on_request(req).await.unwrap();

        match req_result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-R1").unwrap(), "1");
                assert_eq!(req.headers().get("X-R2").unwrap(), "2");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let resp_result = hooks.execute_on_response(resp).await.unwrap();

        assert_eq!(resp_result.headers().get("X-Resp1").unwrap(), "resp1");
        assert_eq!(resp_result.headers().get("X-Resp2").unwrap(), "resp2");
    }

    #[tokio::test]
    async fn test_multiple_hooks_accumulate_state() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("init_counter", |mut req| async move {
                req.headers_mut()
                    .insert("X-Count", axum::http::HeaderValue::from_static("0"));
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("increment_1", |mut req| async move {
                if let Some(count_header) = req.headers().get("X-Count") {
                    if count_header == "0" {
                        req.headers_mut()
                            .insert("X-Count", axum::http::HeaderValue::from_static("1"));
                    }
                }
                Ok(HookResult::Continue(req))
            }))
            .on_request(request_hook("increment_2", |mut req| async move {
                if let Some(count_header) = req.headers().get("X-Count") {
                    if count_header == "1" {
                        req.headers_mut()
                            .insert("X-Count", axum::http::HeaderValue::from_static("2"));
                    }
                }
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();

        match result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Count").unwrap(), "2");
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue"),
        }
    }

    #[tokio::test]
    async fn test_first_hook_short_circuits_second_continues() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("early_exit", |_req| async move {
                let response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Body::from("Early exit"))
                    .unwrap();
                Ok(HookResult::ShortCircuit(response))
            }))
            .on_request(request_hook("never_runs", |req| async move {
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
    async fn test_all_hook_phases_in_sequence() {
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

        let req = Request::builder().body(Body::empty()).unwrap();
        let _ = hooks.execute_on_request(req).await;

        let req = Request::builder().body(Body::empty()).unwrap();
        let _ = hooks.execute_pre_validation(req).await;

        let req = Request::builder().body(Body::empty()).unwrap();
        let _ = hooks.execute_pre_handler(req).await;

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let _ = hooks.execute_on_response(resp).await;

        let resp = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap();
        let _ = hooks.execute_on_error(resp).await;
    }

    #[tokio::test]
    async fn test_hook_with_complex_header_manipulation() {
        let hooks = LifecycleHooks::builder()
            .on_request(request_hook("parse_auth", |mut req| async move {
                let has_auth = req.headers().contains_key("Authorization");
                let auth_status = if has_auth { "authenticated" } else { "anonymous" };
                req.headers_mut()
                    .insert("X-Auth-Status", axum::http::HeaderValue::from_static(auth_status));
                Ok(HookResult::Continue(req))
            }))
            .pre_validation(request_hook("validate_auth", |req| async move {
                if let Some(auth_header) = req.headers().get("X-Auth-Status") {
                    if auth_header == "anonymous" {
                        let response = Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::from("Authentication required"))
                            .unwrap();
                        return Ok(HookResult::ShortCircuit(response));
                    }
                }
                Ok(HookResult::Continue(req))
            }))
            .build();

        let auth_req = Request::builder()
            .header("Authorization", "Bearer token123")
            .body(Body::empty())
            .unwrap();

        let result = hooks.execute_on_request(auth_req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));

        let anon_req = Request::builder().body(Body::empty()).unwrap();
        let on_req_result = hooks.execute_on_request(anon_req).await.unwrap();

        match on_req_result {
            HookResult::Continue(req) => {
                assert_eq!(req.headers().get("X-Auth-Status").unwrap(), "anonymous");

                let val_result = hooks.execute_pre_validation(req).await.unwrap();
                assert!(matches!(val_result, HookResult::ShortCircuit(_)));
            }
            HookResult::ShortCircuit(_) => panic!("Expected Continue from on_request"),
        }
    }

    #[tokio::test]
    async fn test_empty_hooks_no_overhead() {
        let hooks = LifecycleHooks::new();
        assert!(hooks.is_empty());

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_on_request(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();
        assert!(matches!(result, HookResult::Continue(_)));

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();
        let result = hooks.execute_on_response(resp).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_response_hook_short_circuit_treated_as_continue() {
        let hooks = LifecycleHooks::builder()
            .on_response(response_hook("hook_with_short_circuit", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Processed", axum::http::HeaderValue::from_static("yes"));
                Ok(HookResult::ShortCircuit(resp))
            }))
            .on_response(response_hook("second_hook", |mut resp| async move {
                resp.headers_mut()
                    .insert("X-Second", axum::http::HeaderValue::from_static("yes"));
                Ok(HookResult::Continue(resp))
            }))
            .build();

        let resp = Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap();

        let result = hooks.execute_on_response(resp).await.unwrap();

        assert_eq!(result.headers().get("X-Processed").unwrap(), "yes");
        assert_eq!(result.headers().get("X-Second").unwrap(), "yes");
    }

    #[tokio::test]
    async fn test_complex_pre_validation_flow_with_auth_and_content_check() {
        let hooks = LifecycleHooks::builder()
            .pre_validation(request_hook("check_auth", |req| async move {
                if !req.headers().contains_key("Authorization") {
                    return Ok(HookResult::ShortCircuit(
                        Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(Body::from("Missing auth"))
                            .unwrap(),
                    ));
                }
                Ok(HookResult::Continue(req))
            }))
            .pre_validation(request_hook("check_content_type", |req| async move {
                if req.method() == axum::http::Method::POST {
                    if !req.headers().contains_key("Content-Type") {
                        return Ok(HookResult::ShortCircuit(
                            Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from("Missing Content-Type"))
                                .unwrap(),
                        ));
                    }
                }
                Ok(HookResult::Continue(req))
            }))
            .build();

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit for missing auth"),
        }

        let req = Request::builder()
            .method(axum::http::Method::POST)
            .header("Authorization", "Bearer token")
            .body(Body::empty())
            .unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        match result {
            HookResult::ShortCircuit(resp) => {
                assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
            }
            HookResult::Continue(_) => panic!("Expected ShortCircuit for missing content type"),
        }

        let req = Request::builder()
            .method(axum::http::Method::POST)
            .header("Authorization", "Bearer token")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap();
        let result = hooks.execute_pre_validation(req).await.unwrap();

        assert!(matches!(result, HookResult::Continue(_)));
    }
}
