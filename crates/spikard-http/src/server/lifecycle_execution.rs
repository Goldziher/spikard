//! Lifecycle hooks execution logic

use crate::handler_trait::Handler;
use axum::body::Body;
use axum::http::StatusCode;
use std::sync::Arc;

/// Execute a handler with lifecycle hooks
///
/// This wraps the handler execution with lifecycle hooks at appropriate points:
/// 1. preValidation hooks (before handler, which does validation)
/// 2. preHandler hooks (after validation, before handler)
/// 3. Handler execution
/// 4. onResponse hooks (after successful handler execution)
/// 5. onError hooks (if handler or any hook fails)
pub async fn execute_with_lifecycle_hooks(
    req: axum::http::Request<Body>,
    request_data: crate::handler_trait::RequestData,
    handler: Arc<dyn Handler>,
    hooks: Option<Arc<crate::LifecycleHooks>>,
) -> Result<axum::http::Response<Body>, (axum::http::StatusCode, String)> {
    use crate::lifecycle::HookResult;

    let Some(hooks) = hooks else {
        return handler.call(req, request_data).await;
    };

    if hooks.is_empty() {
        return handler.call(req, request_data).await;
    }

    let req = match hooks.execute_on_request(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return Ok(response),
        Err(e) => {
            let error_response = axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("{{\"error\":\"onRequest hook failed: {}\"}}", e)))
                .unwrap();

            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(_) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("{\"error\":\"Hook execution failed\"}"))
                    .unwrap()),
            };
        }
    };

    let req = match hooks.execute_pre_validation(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return Ok(response),
        Err(e) => {
            let error_response = axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "{{\"error\":\"preValidation hook failed: {}\"}}",
                    e
                )))
                .unwrap();

            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(_) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("{\"error\":\"Hook execution failed\"}"))
                    .unwrap()),
            };
        }
    };

    let req = match hooks.execute_pre_handler(req).await {
        Ok(HookResult::Continue(r)) => r,
        Ok(HookResult::ShortCircuit(response)) => return Ok(response),
        Err(e) => {
            let error_response = axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("{{\"error\":\"preHandler hook failed: {}\"}}", e)))
                .unwrap();

            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(_) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("{\"error\":\"Hook execution failed\"}"))
                    .unwrap()),
            };
        }
    };

    let response = match handler.call(req, request_data).await {
        Ok(resp) => resp,
        Err((status, message)) => {
            let error_response = axum::http::Response::builder()
                .status(status)
                .body(Body::from(message))
                .unwrap();

            return match hooks.execute_on_error(error_response).await {
                Ok(resp) => Ok(resp),
                Err(e) => Ok(axum::http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("{{\"error\":\"onError hook failed: {}\"}}", e)))
                    .unwrap()),
            };
        }
    };

    match hooks.execute_on_response(response).await {
        Ok(resp) => Ok(resp),
        Err(e) => Ok(axum::http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{{\"error\":\"onResponse hook failed: {}\"}}", e)))
            .unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lifecycle::{HookResult, request_hook, response_hook};
    use axum::http::{Request, Response, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::json;
    use std::collections::HashMap;

    struct OkHandler;

    impl Handler for OkHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: crate::handler_trait::RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::handler_trait::HandlerResult> + Send + '_>>
        {
            Box::pin(async move {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from("ok"))
                    .unwrap())
            })
        }
    }

    struct ErrHandler;

    impl Handler for ErrHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: crate::handler_trait::RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::handler_trait::HandlerResult> + Send + '_>>
        {
            Box::pin(async move { Err((StatusCode::BAD_REQUEST, "bad".to_string())) })
        }
    }

    fn empty_request_data() -> crate::handler_trait::RequestData {
        crate::handler_trait::RequestData {
            path_params: std::sync::Arc::new(HashMap::new()),
            query_params: std::sync::Arc::new(json!({})),
            validated_params: None,
            raw_query_params: std::sync::Arc::new(HashMap::new()),
            body: std::sync::Arc::new(json!(null)),
            raw_body: None,
            headers: std::sync::Arc::new(HashMap::new()),
            cookies: std::sync::Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    #[tokio::test]
    async fn pre_validation_error_with_failing_on_error_hook_returns_fallback() {
        let mut hooks = crate::LifecycleHooks::new();
        hooks.add_pre_validation(request_hook("boom", |_req| async move { Err("boom".to_string()) }));
        hooks.add_on_error(response_hook("fail-on-error", |_resp| async move {
            Err("fail".to_string())
        }));

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp = execute_with_lifecycle_hooks(req, empty_request_data(), Arc::new(OkHandler), Some(Arc::new(hooks)))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"{\"error\":\"Hook execution failed\"}");
    }

    #[tokio::test]
    async fn handler_error_with_failing_on_error_hook_returns_on_error_hook_failed_response() {
        let mut hooks = crate::LifecycleHooks::new();
        hooks.add_on_error(response_hook("fail-on-error", |_resp| async move {
            Err("boom".to_string())
        }));

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp = execute_with_lifecycle_hooks(req, empty_request_data(), Arc::new(ErrHandler), Some(Arc::new(hooks)))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(body.as_ref()).unwrap();
        assert!(body_str.contains("\"error\":\"onError hook failed:"));
        assert!(body_str.contains("boom"));
    }

    #[tokio::test]
    async fn on_response_hook_error_returns_on_response_hook_failed_response() {
        let mut hooks = crate::LifecycleHooks::new();
        hooks.add_on_response(response_hook("fail-on-response", |_resp| async move {
            Err("boom".to_string())
        }));

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp = execute_with_lifecycle_hooks(req, empty_request_data(), Arc::new(OkHandler), Some(Arc::new(hooks)))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let body_str = std::str::from_utf8(body.as_ref()).unwrap();
        assert!(body_str.contains("\"error\":\"onResponse hook failed:"));
        assert!(body_str.contains("boom"));
    }

    #[tokio::test]
    async fn pre_validation_short_circuit_skips_handler_and_returns_response() {
        let mut hooks = crate::LifecycleHooks::new();
        hooks.add_pre_validation(request_hook("short-circuit", |_req| async move {
            Ok(HookResult::ShortCircuit(
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("nope"))
                    .unwrap(),
            ))
        }));

        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let resp = execute_with_lifecycle_hooks(req, empty_request_data(), Arc::new(ErrHandler), Some(Arc::new(hooks)))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"nope");
    }
}
