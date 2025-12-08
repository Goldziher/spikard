//! ValidatingHandler wrapper that executes request/parameter validation before handler

use crate::ProblemDetails;
use crate::handler_trait::{Handler, HandlerResult, RequestData};
use crate::parameters::ParameterValidator;
use crate::validation::SchemaValidator;
use axum::body::Body;
use futures::FutureExt;
use serde_json::Value;
use spikard_core::errors::StructuredError;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::Arc;

/// Wrapper that runs request/parameter validation before calling the user handler.
pub struct ValidatingHandler {
    inner: Arc<dyn Handler>,
    request_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
}

impl ValidatingHandler {
    /// Create a new validating handler wrapping the inner handler with schema validators
    pub fn new(inner: Arc<dyn Handler>, route: &crate::Route) -> Self {
        Self {
            inner,
            request_validator: route.request_validator.clone(),
            parameter_validator: route.parameter_validator.clone(),
        }
    }
}

impl Handler for ValidatingHandler {
    fn call(
        &self,
        req: axum::http::Request<Body>,
        mut request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        let inner = self.inner.clone();
        let request_validator = self.request_validator.clone();
        let parameter_validator = self.parameter_validator.clone();

        Box::pin(async move {
            if let Some(validator) = request_validator {
                if request_data.body.is_null() && request_data.raw_body.is_some() {
                    let raw_bytes = request_data.raw_body.as_ref().unwrap();
                    request_data.body = serde_json::from_slice::<Value>(raw_bytes)
                        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?;
                }

                if let Err(errors) = validator.validate(&request_data.body) {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((problem.status_code(), body));
                }
            }

            if let Some(validator) = parameter_validator
                && let Err(errors) = validator.validate_and_extract(
                    &request_data.query_params,
                    &request_data.raw_query_params,
                    &request_data.path_params,
                    &request_data.headers,
                    &request_data.cookies,
                )
            {
                let problem = ProblemDetails::from_validation_error(&errors);
                let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                return Err((problem.status_code(), body));
            }

            match AssertUnwindSafe(async { inner.call(req, request_data).await })
                .catch_unwind()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    let panic_payload = StructuredError::simple("panic", "Unexpected panic in handler");
                    let body = serde_json::to_string(&panic_payload)
                        .unwrap_or_else(|_| r#"{"error":"panic","code":"panic","details":{}}"#.to_string());
                    Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, body))
                }
            }
        })
    }
}
