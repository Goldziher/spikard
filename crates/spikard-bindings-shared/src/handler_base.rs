//! Base handler traits and execution infrastructure
//!
//! This module provides language-agnostic handler execution patterns that
//! eliminate duplicate code across Node, Ruby, PHP, and WASM bindings.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, Response};
use spikard_core::validation::{SchemaValidator, ValidationError};
use spikard_http::Handler;
use spikard_http::handler_trait::{HandlerResult, RequestData};

use crate::error_response::ErrorResponseBuilder;

/// Error type for handler operations
#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Handler execution error: {0}")]
    Execution(String),

    #[error("Response conversion error: {0}")]
    ResponseConversion(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<ValidationError> for HandlerError {
    fn from(err: ValidationError) -> Self {
        // PERFORMANCE: Avoid format! allocation for debug representation.
        // Most callers just need the error type, not the full debug output.
        Self::Validation(err.to_string())
    }
}

/// Language-specific handler implementation
///
/// This trait defines the three key operations that every language binding must implement:
/// 1. Prepare request data for the language runtime
/// 2. Invoke the handler in the language runtime
/// 3. Interpret the response from the language runtime
///
/// The `HandlerExecutor` handles all common logic (validation, error formatting)
/// while delegating language-specific operations to implementations of this trait.
pub trait LanguageHandler: Send + Sync {
    /// Input type passed to the language handler
    type Input: Send;

    /// Output type returned from the language handler
    type Output: Send;

    /// Prepare request data for passing to the language handler
    ///
    /// # Errors
    ///
    /// Returns an error if request preparation fails.
    fn prepare_request(&self, request_data: &RequestData) -> Result<Self::Input, HandlerError>;

    /// Invoke the language-specific handler with the prepared input
    fn invoke_handler(
        &self,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Output, HandlerError>> + Send + '_>>;

    /// Interpret the handler's output and convert it to an HTTP response
    ///
    /// # Errors
    ///
    /// Returns an error if response interpretation fails.
    fn interpret_response(&self, output: Self::Output) -> Result<Response<Body>, HandlerError>;
}

/// Universal handler executor that works with any language binding
///
/// This struct consolidates the common handler execution flow:
/// - Request body validation
/// - Handler invocation
/// - Error handling and formatting
///
/// Language bindings provide thin wrappers that implement `LanguageHandler`.
pub struct HandlerExecutor<L: LanguageHandler> {
    language_handler: Arc<L>,
    request_validator: Option<Arc<SchemaValidator>>,
}

impl<L: LanguageHandler> HandlerExecutor<L> {
    /// Create a new handler executor
    pub const fn new(language_handler: Arc<L>, request_validator: Option<Arc<SchemaValidator>>) -> Self {
        Self {
            language_handler,
            request_validator,
        }
    }

    /// Create a handler executor with only a language handler
    pub const fn with_handler(language_handler: Arc<L>) -> Self {
        Self {
            language_handler,
            request_validator: None,
        }
    }

    /// Add request validation to this executor
    #[must_use]
    pub fn with_request_validator(mut self, validator: Arc<SchemaValidator>) -> Self {
        self.request_validator = Some(validator);
        self
    }
}

impl<L: LanguageHandler + 'static> Handler for HandlerExecutor<L> {
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            if let Some(validator) = &self.request_validator
                && let Err(validation_err) = validator.validate(&request_data.body)
            {
                return Err(ErrorResponseBuilder::validation_error(&validation_err));
            }

            // PERFORMANCE: Avoid format! allocations in the hot path. ErrorResponseBuilder
            // can accept &dyn Display or construct error messages directly, reducing
            // string allocation overhead in typical error handling paths.
            let input = self
                .language_handler
                .prepare_request(&request_data)
                .map_err(|e| ErrorResponseBuilder::internal_error(format!("Failed to prepare request: {e}")))?;

            let output = self
                .language_handler
                .invoke_handler(input)
                .await
                .map_err(|e| ErrorResponseBuilder::internal_error(format!("Handler execution failed: {e}")))?;

            let response = self
                .language_handler
                .interpret_response(output)
                .map_err(|e| ErrorResponseBuilder::internal_error(format!("Failed to interpret response: {e}")))?;

            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockLanguageHandler;

    impl LanguageHandler for MockLanguageHandler {
        type Input = String;
        type Output = String;

        fn prepare_request(&self, _data: &RequestData) -> Result<Self::Input, HandlerError> {
            Ok("test_input".to_string())
        }

        fn invoke_handler(
            &self,
            _input: Self::Input,
        ) -> Pin<Box<dyn Future<Output = Result<Self::Output, HandlerError>> + Send + '_>> {
            Box::pin(async { Ok("test_output".to_string()) })
        }

        fn interpret_response(&self, output: Self::Output) -> Result<Response<Body>, HandlerError> {
            Ok(Response::builder().status(200).body(Body::from(output)).unwrap())
        }
    }

    #[tokio::test]
    async fn test_handler_executor_basic() {
        let mock_handler = Arc::new(MockLanguageHandler);
        let executor = HandlerExecutor::new(mock_handler, None);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(std::collections::HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(std::collections::HashMap::new()),
            body: Arc::new(json!({})),
            raw_body: None,
            headers: Arc::new(std::collections::HashMap::new()),
            cookies: Arc::new(std::collections::HashMap::new()),
            method: "GET".to_string(),
            path: "/test".to_string(),
            dependencies: None,
        };

        let result = executor.call(request, request_data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handler_executor_with_handler_only() {
        let mock_handler = Arc::new(MockLanguageHandler);
        let executor = HandlerExecutor::with_handler(mock_handler);

        let request = Request::builder().body(Body::empty()).unwrap();
        let request_data = RequestData {
            path_params: Arc::new(std::collections::HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(std::collections::HashMap::new()),
            body: Arc::new(json!({})),
            raw_body: None,
            headers: Arc::new(std::collections::HashMap::new()),
            cookies: Arc::new(std::collections::HashMap::new()),
            method: "GET".to_string(),
            path: "/test".to_string(),
            dependencies: None,
        };

        let result = executor.call(request, request_data).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_handler_error_validation() {
        let err = HandlerError::Validation("test error".to_string());
        assert_eq!(err.to_string(), "Validation error: test error");
    }

    #[test]
    fn test_handler_error_execution() {
        let err = HandlerError::Execution("test error".to_string());
        assert_eq!(err.to_string(), "Handler execution error: test error");
    }

    #[test]
    fn test_handler_error_response_conversion() {
        let err = HandlerError::ResponseConversion("test error".to_string());
        assert_eq!(err.to_string(), "Response conversion error: test error");
    }

    #[test]
    fn test_handler_error_internal() {
        let err = HandlerError::Internal("test error".to_string());
        assert_eq!(err.to_string(), "Internal error: test error");
    }
}
