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
