#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive tests for lifecycle hooks execution
//!
//! Tests the execute_with_lifecycle_hooks function covering:
//! - No hooks and empty hooks fast path
//! - Hook execution order and continuation
//! - Short-circuit behavior
//! - Error handling and onError transformation
//! - Success path with onResponse
//! - Response modifications

mod common;

use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use serde_json::json;
use spikard_http::lifecycle::HookResult;
use spikard_http::server::lifecycle_execution::execute_with_lifecycle_hooks;
use spikard_http::{Handler, HandlerResult, RequestData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Test request data factory
fn test_request_data() -> RequestData {
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

/// Handler that returns 200 OK with custom body
struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("OK"))
                .unwrap();
            Ok(response)
        })
    }
}

/// Handler that returns 400 Bad Request error
struct BadRequestHandler;

impl Handler for BadRequestHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move { Err((StatusCode::BAD_REQUEST, "{\"error\": \"Bad Request\"}".to_string())) })
    }
}

#[tokio::test]
async fn test_no_hooks_none_variant() {
    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, None).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_empty_hooks_list() {
    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();
    let hooks = Arc::new(spikard_http::lifecycle::LifecycleHooks::new());

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(hooks)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_pre_validation_hook_continues() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "continue_hook",
        |req| async move { Ok(HookResult::Continue(req)) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_pre_validation_hook_short_circuits() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "short_circuit",
        |_req| async move {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("{\"error\": \"Unauthorized\"}"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_pre_validation_hook_error_calls_on_error() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "failing_hook",
        |_req| async move { Err("Validation failed".to_string()) },
    ));

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "error_handler",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Error-Handled", axum::http::HeaderValue::from_static("true"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        response.headers().get("X-Error-Handled").map(|v| v.to_str().unwrap()),
        Some("true")
    );
}

#[tokio::test]
async fn test_pre_handler_hook_continues() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook(
        "continue_hook",
        |req| async move { Ok(HookResult::Continue(req)) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_pre_handler_hook_short_circuits() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook(
        "short_circuit",
        |_req| async move {
            let response = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("{\"error\": \"Forbidden\"}"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_pre_handler_hook_error_calls_on_error() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook(
        "failing_hook",
        |_req| async move { Err("Handler preparation failed".to_string()) },
    ));

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "error_handler",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-PreHandler-Error", axum::http::HeaderValue::from_static("caught"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        response
            .headers()
            .get("X-PreHandler-Error")
            .map(|v| v.to_str().unwrap()),
        Some("caught")
    );
}

#[tokio::test]
async fn test_handler_error_calls_on_error() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "error_logger",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Handler-Error", axum::http::HeaderValue::from_static("logged"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(BadRequestHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.headers().get("X-Handler-Error").map(|v| v.to_str().unwrap()),
        Some("logged")
    );
}

#[tokio::test]
async fn test_success_handler_calls_on_response() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "response_modifier",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Response-Modified", axum::http::HeaderValue::from_static("yes"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("X-Response-Modified")
            .map(|v| v.to_str().unwrap()),
        Some("yes")
    );
}

#[tokio::test]
async fn test_on_response_hook_error_returns_500() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "failing_response_hook",
        |_resp| async move { Err("Response processing failed".to_string()) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_on_error_hook_fails_fallback_500() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "failing_validation",
        |_req| async move { Err("Validation error".to_string()) },
    ));

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "failing_error_hook",
        |_resp| async move { Err("Error hook failed".to_string()) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_hook_execution_order_preserved() {
    use std::sync::Mutex;

    let execution_log = Arc::new(Mutex::new(Vec::new()));
    let execution_log_clone1 = execution_log.clone();
    let execution_log_clone2 = execution_log.clone();
    let execution_log_clone3 = execution_log.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook("pre_validation", move |req| {
        let log = execution_log_clone1.clone();
        async move {
            log.lock().unwrap().push("pre_validation");
            Ok(HookResult::Continue(req))
        }
    }));

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook("pre_handler", move |req| {
        let log = execution_log_clone2.clone();
        async move {
            log.lock().unwrap().push("pre_handler");
            Ok(HookResult::Continue(req))
        }
    }));

    hooks.add_on_response(spikard_http::lifecycle::response_hook("on_response", move |resp| {
        let log = execution_log_clone3.clone();
        async move {
            log.lock().unwrap().push("on_response");
            Ok(HookResult::Continue(resp))
        }
    }));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());

    let log = execution_log.lock().unwrap();
    assert_eq!(log.as_slice(), &["pre_validation", "pre_handler", "on_response"]);
}

#[tokio::test]
async fn test_pre_validation_short_circuit_skips_handler_and_on_response() {
    use std::sync::Mutex;

    let handler_called = Arc::new(Mutex::new(false));
    let response_hook_called = Arc::new(Mutex::new(false));

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "short_circuit",
        |_req| async move {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        },
    ));

    let response_hook_called_clone = response_hook_called.clone();
    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "response_tracker",
        move |resp| {
            let flag = response_hook_called_clone.clone();
            async move {
                *flag.lock().unwrap() = true;
                Ok(HookResult::Continue(resp))
            }
        },
    ));

    struct TrackingHandler {
        called: Arc<Mutex<bool>>,
    }

    impl Handler for TrackingHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            let called = self.called.clone();
            Box::pin(async move {
                *called.lock().unwrap() = true;
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from("OK"))
                    .unwrap();
                Ok(response)
            })
        }
    }

    let handler = Arc::new(TrackingHandler {
        called: handler_called.clone(),
    });

    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    assert!(!*handler_called.lock().unwrap(), "Handler should not be called");
    assert!(
        !*response_hook_called.lock().unwrap(),
        "on_response should not be called"
    );
}

#[tokio::test]
async fn test_multiple_hooks_same_phase_executed_in_order() {
    use std::sync::Mutex;

    let execution_log = Arc::new(Mutex::new(Vec::new()));
    let log1 = execution_log.clone();
    let log2 = execution_log.clone();
    let log3 = execution_log.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook("first_hook", move |resp| {
        let log = log1.clone();
        async move {
            log.lock().unwrap().push("first");
            Ok(HookResult::Continue(resp))
        }
    }));

    hooks.add_on_response(spikard_http::lifecycle::response_hook("second_hook", move |resp| {
        let log = log2.clone();
        async move {
            log.lock().unwrap().push("second");
            Ok(HookResult::Continue(resp))
        }
    }));

    hooks.add_on_response(spikard_http::lifecycle::response_hook("third_hook", move |resp| {
        let log = log3.clone();
        async move {
            log.lock().unwrap().push("third");
            Ok(HookResult::Continue(resp))
        }
    }));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());

    let log = execution_log.lock().unwrap();
    assert_eq!(log.as_slice(), &["first", "second", "third"]);
}

#[tokio::test]
async fn test_hook_can_modify_request() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook(
        "add_header",
        |mut req| async move {
            req.headers_mut()
                .insert("X-Added-By-Hook", axum::http::HeaderValue::from_static("hook-value"));
            Ok(HookResult::Continue(req))
        },
    ));

    struct HeaderCheckHandler;

    impl Handler for HeaderCheckHandler {
        fn call(
            &self,
            request: Request<Body>,
            _request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move {
                let has_header = request.headers().contains_key("X-Added-By-Hook");
                let body = if has_header { "Header found" } else { "Header not found" };

                let response = Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(body))
                    .unwrap();
                Ok(response)
            })
        }
    }

    let handler = Arc::new(HeaderCheckHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_error_response_contains_hook_error_message() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "failing_hook",
        |_req| async move { Err("Custom validation error".to_string()) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_str.contains("Custom validation error"));
}

#[tokio::test]
async fn test_chained_hook_modifications() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "add_header_1",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Header-1", axum::http::HeaderValue::from_static("value1"));
            Ok(HookResult::Continue(resp))
        },
    ));

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "add_header_2",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Header-2", axum::http::HeaderValue::from_static("value2"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(
        response.headers().get("X-Header-1").map(|v| v.to_str().unwrap()),
        Some("value1")
    );
    assert_eq!(
        response.headers().get("X-Header-2").map(|v| v.to_str().unwrap()),
        Some("value2")
    );
}

#[tokio::test]
async fn test_on_response_not_called_when_handler_fails() {
    use std::sync::Mutex;

    let response_hook_called = Arc::new(Mutex::new(false));
    let response_hook_called_clone = response_hook_called.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "response_tracker",
        move |resp| {
            let flag = response_hook_called_clone.clone();
            async move {
                *flag.lock().unwrap() = true;
                Ok(HookResult::Continue(resp))
            }
        },
    ));

    let handler = Arc::new(BadRequestHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    assert!(
        !*response_hook_called.lock().unwrap(),
        "on_response should NOT be called when handler fails"
    );
}

#[tokio::test]
async fn test_on_error_called_when_handler_fails() {
    use std::sync::Mutex;

    let error_hook_called = Arc::new(Mutex::new(false));
    let error_hook_called_clone = error_hook_called.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_error(spikard_http::lifecycle::response_hook("error_tracker", move |resp| {
        let flag = error_hook_called_clone.clone();
        async move {
            *flag.lock().unwrap() = true;
            Ok(HookResult::Continue(resp))
        }
    }));

    let handler = Arc::new(BadRequestHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    assert!(
        *error_hook_called.lock().unwrap(),
        "on_error should be called when handler fails"
    );
}

#[tokio::test]
async fn test_on_error_hook_preserves_original_handler_error_status() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "error_checker",
        |resp| async move {
            if resp.status() == StatusCode::BAD_REQUEST {
                Ok(HookResult::Continue(resp))
            } else {
                Err("Expected 400 status".to_string())
            }
        },
    ));

    let handler = Arc::new(BadRequestHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_all_three_error_paths_call_on_error() {
    use std::sync::Mutex;

    let error_hook_count = Arc::new(Mutex::new(0));

    {
        let error_hook_count_clone = error_hook_count.clone();
        let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

        hooks.add_pre_validation(spikard_http::lifecycle::request_hook("failing", |_req| async move {
            Err("preValidation error".to_string())
        }));

        hooks.add_on_error(spikard_http::lifecycle::response_hook("counter", move |resp| {
            let count = error_hook_count_clone.clone();
            async move {
                *count.lock().unwrap() += 1;
                Ok(HookResult::Continue(resp))
            }
        }));

        let handler = Arc::new(OkHandler);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = test_request_data();

        let _result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;
    }

    assert_eq!(
        *error_hook_count.lock().unwrap(),
        1,
        "on_error called for preValidation error"
    );

    *error_hook_count.lock().unwrap() = 0;
    {
        let error_hook_count_clone = error_hook_count.clone();
        let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

        hooks.add_pre_handler(spikard_http::lifecycle::request_hook("failing", |_req| async move {
            Err("preHandler error".to_string())
        }));

        hooks.add_on_error(spikard_http::lifecycle::response_hook("counter", move |resp| {
            let count = error_hook_count_clone.clone();
            async move {
                *count.lock().unwrap() += 1;
                Ok(HookResult::Continue(resp))
            }
        }));

        let handler = Arc::new(OkHandler);
        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = test_request_data();

        let _result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;
    }

    assert_eq!(
        *error_hook_count.lock().unwrap(),
        1,
        "on_error called for preHandler error"
    );
}

#[tokio::test]
async fn test_on_error_fallback_response_when_error_hook_fails() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook(
        "failing_validation",
        |_req| async move { Err("Validation error".to_string()) },
    ));

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "failing_error_hook",
        |_resp| async move { Err("on_error itself failed".to_string()) },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_str.contains("Hook execution failed"));
}

#[tokio::test]
async fn test_full_hook_chain_with_success() {
    use std::sync::Mutex;

    let execution_log = Arc::new(Mutex::new(Vec::new()));
    let log1 = execution_log.clone();
    let log2 = execution_log.clone();
    let log3 = execution_log.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook("pv", move |req| {
        let log = log1.clone();
        async move {
            log.lock().unwrap().push("preValidation");
            Ok(HookResult::Continue(req))
        }
    }));

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook("ph", move |req| {
        let log = log2.clone();
        async move {
            log.lock().unwrap().push("preHandler");
            Ok(HookResult::Continue(req))
        }
    }));

    hooks.add_on_response(spikard_http::lifecycle::response_hook("or", move |resp| {
        let log = log3.clone();
        async move {
            log.lock().unwrap().push("onResponse");
            Ok(HookResult::Continue(resp))
        }
    }));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let log = execution_log.lock().unwrap();
    assert_eq!(log.as_slice(), &["preValidation", "preHandler", "onResponse"]);
}

#[tokio::test]
async fn test_pre_handler_short_circuit_skips_on_response() {
    use std::sync::Mutex;

    let response_hook_called = Arc::new(Mutex::new(false));
    let response_hook_called_clone = response_hook_called.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook(
        "short_circuit",
        |_req| async move {
            let response = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("Forbidden"))
                .unwrap();
            Ok(HookResult::ShortCircuit(response))
        },
    ));

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "response_tracker",
        move |resp| {
            let flag = response_hook_called_clone.clone();
            async move {
                *flag.lock().unwrap() = true;
                Ok(HookResult::Continue(resp))
            }
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert!(
        !*response_hook_called.lock().unwrap(),
        "on_response should not be called"
    );
}

#[tokio::test]
async fn test_on_response_hook_can_transform_success_response() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "transform",
        |_old_resp| async move {
            let new_response = Response::builder()
                .status(StatusCode::CREATED)
                .header("X-Transformed", "yes")
                .body(Body::from("Transformed"))
                .unwrap();
            Ok(HookResult::Continue(new_response))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        response.headers().get("X-Transformed").map(|v| v.to_str().unwrap()),
        Some("yes")
    );
}

#[tokio::test]
async fn test_error_response_body_contains_correct_hook_context() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_handler(spikard_http::lifecycle::request_hook("failing", |_req| async move {
        Err("Specific handler setup error".to_string())
    }));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_str.contains("preHandler"));
    assert!(body_str.contains("Specific handler setup error"));
}

#[tokio::test]
async fn test_multiple_error_hooks_execute_in_sequence() {
    use std::sync::Mutex;

    let execution_log = Arc::new(Mutex::new(Vec::new()));
    let log1 = execution_log.clone();
    let log2 = execution_log.clone();

    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_pre_validation(spikard_http::lifecycle::request_hook("failing", |_req| async move {
        Err("Validation failed".to_string())
    }));

    hooks.add_on_error(spikard_http::lifecycle::response_hook("error_handler_1", move |resp| {
        let log = log1.clone();
        async move {
            log.lock().unwrap().push("error_handler_1");
            Ok(HookResult::Continue(resp))
        }
    }));

    hooks.add_on_error(spikard_http::lifecycle::response_hook("error_handler_2", move |resp| {
        let log = log2.clone();
        async move {
            log.lock().unwrap().push("error_handler_2");
            Ok(HookResult::Continue(resp))
        }
    }));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let log = execution_log.lock().unwrap();
    assert_eq!(log.as_slice(), &["error_handler_1", "error_handler_2"]);
}

#[tokio::test]
async fn test_handler_result_error_becomes_500_with_on_error_handling() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_error(spikard_http::lifecycle::response_hook(
        "error_modifier",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Error-Detected", axum::http::HeaderValue::from_static("true"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(BadRequestHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.headers().get("X-Error-Detected").map(|v| v.to_str().unwrap()),
        Some("true")
    );
}

#[tokio::test]
async fn test_on_response_with_multiple_sequential_transforms() {
    let mut hooks = spikard_http::lifecycle::LifecycleHooks::new();

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "add_header_1",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Step-1", axum::http::HeaderValue::from_static("done"));
            Ok(HookResult::Continue(resp))
        },
    ));

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "add_header_2",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Step-2", axum::http::HeaderValue::from_static("done"));
            Ok(HookResult::Continue(resp))
        },
    ));

    hooks.add_on_response(spikard_http::lifecycle::response_hook(
        "add_header_3",
        |mut resp| async move {
            resp.headers_mut()
                .insert("X-Step-3", axum::http::HeaderValue::from_static("done"));
            Ok(HookResult::Continue(resp))
        },
    ));

    let handler = Arc::new(OkHandler);
    let request = Request::builder().body(Body::empty()).unwrap();
    let request_data = test_request_data();

    let result = execute_with_lifecycle_hooks(request, request_data, handler, Some(Arc::new(hooks))).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(
        response.headers().get("X-Step-1").map(|v| v.to_str().unwrap()),
        Some("done")
    );
    assert_eq!(
        response.headers().get("X-Step-2").map(|v| v.to_str().unwrap()),
        Some("done")
    );
    assert_eq!(
        response.headers().get("X-Step-3").map(|v| v.to_str().unwrap()),
        Some("done")
    );
}
