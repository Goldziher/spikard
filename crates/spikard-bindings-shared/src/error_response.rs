//! Shared error response formatting
//!
//! This module consolidates error response building across all language bindings,
//! eliminating duplicate `structured_error()` functions in Node, Ruby, and PHP bindings.

use axum::http::StatusCode;
use serde_json::Value;
use spikard_core::errors::StructuredError;
use spikard_core::problem::ProblemDetails;
use spikard_core::validation::ValidationError;

/// Builder for creating standardized error responses across all bindings
pub struct ErrorResponseBuilder;

impl ErrorResponseBuilder {
    /// Create a structured error response with status code and error details
    ///
    /// Returns a tuple of (`StatusCode`, JSON body as String)
    ///
    /// # Arguments
    /// * `status` - HTTP status code
    /// * `code` - Machine-readable error code
    /// * `message` - Human-readable error message
    ///
    /// # Example
    /// ```
    /// use axum::http::StatusCode;
    /// use spikard_bindings_shared::ErrorResponseBuilder;
    ///
    /// let (status, body) = ErrorResponseBuilder::structured_error(
    ///     StatusCode::BAD_REQUEST,
    ///     "invalid_input",
    ///     "Missing required field",
    /// );
    /// ```
    pub fn structured_error(status: StatusCode, code: &str, message: impl Into<String>) -> (StatusCode, String) {
        let payload = StructuredError::simple(code.to_string(), message.into());
        let body = serde_json::to_string(&payload)
            .unwrap_or_else(|_| r#"{"error":"serialization_failed","code":"internal_error","details":{}}"#.to_string());
        (status, body)
    }

    /// Create an error response with additional details
    ///
    /// Returns a tuple of (`StatusCode`, JSON body as String)
    ///
    /// # Arguments
    /// * `status` - HTTP status code
    /// * `code` - Machine-readable error code
    /// * `message` - Human-readable error message
    /// * `details` - Structured details about the error
    ///
    /// # Example
    /// ```
    /// use axum::http::StatusCode;
    /// use serde_json::json;
    /// use spikard_bindings_shared::ErrorResponseBuilder;
    ///
    /// let details = json!({
    ///     "field": "email",
    ///     "reason": "invalid_format"
    /// });
    /// let (status, body) = ErrorResponseBuilder::with_details(
    ///     StatusCode::BAD_REQUEST,
    ///     "validation_error",
    ///     "Invalid email format",
    ///     details,
    /// );
    /// ```
    pub fn with_details(
        status: StatusCode,
        code: &str,
        message: impl Into<String>,
        details: Value,
    ) -> (StatusCode, String) {
        let payload = StructuredError::new(code.to_string(), message.into(), details);
        let body = serde_json::to_string(&payload)
            .unwrap_or_else(|_| r#"{"error":"serialization_failed","code":"internal_error","details":{}}"#.to_string());
        (status, body)
    }

    /// Create an error response from a `StructuredError`
    ///
    /// Returns a tuple of (`StatusCode`, JSON body as String)
    ///
    /// # Arguments
    /// * `error` - The structured error
    ///
    /// # Note
    /// Uses `INTERNAL_SERVER_ERROR` as the default status code. Override with
    /// `structured_error()` or `with_details()` for specific status codes.
    #[must_use]
    pub fn from_structured_error(error: &StructuredError) -> (StatusCode, String) {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let body = serde_json::to_string(&error)
            .unwrap_or_else(|_| r#"{"error":"serialization_failed","code":"internal_error","details":{}}"#.to_string());
        (status, body)
    }

    /// Create a validation error response
    ///
    /// Converts `ValidationError` to RFC 9457 Problem Details format.
    /// Returns (`StatusCode::UNPROCESSABLE_ENTITY`, JSON body)
    ///
    /// # Arguments
    /// * `validation_error` - The validation error containing one or more details
    ///
    /// # Example
    /// ```
    /// use spikard_core::validation::{ValidationError, ValidationErrorDetail};
    /// use serde_json::Value;
    /// use spikard_bindings_shared::ErrorResponseBuilder;
    ///
    /// let validation_error = ValidationError {
    ///     errors: vec![
    ///         ValidationErrorDetail {
    ///             error_type: "missing".to_string(),
    ///             loc: vec!["body".to_string(), "username".to_string()],
    ///             msg: "Field required".to_string(),
    ///             input: Value::String("".to_string()),
    ///             ctx: None,
    ///         },
    ///     ],
    /// };
    ///
    /// let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
    /// ```
    #[must_use]
    pub fn validation_error(validation_error: &ValidationError) -> (StatusCode, String) {
        let problem = ProblemDetails::from_validation_error(validation_error);
        let status = problem.status_code();
        let body = serde_json::to_string(&problem).unwrap_or_else(|_| {
            r#"{"title":"Validation Failed","type":"https://spikard.dev/errors/validation-error","status":422}"#
                .to_string()
        });
        (status, body)
    }

    /// Create an RFC 9457 Problem Details response
    ///
    /// Returns a tuple of (`StatusCode`, JSON body as String)
    ///
    /// # Arguments
    /// * `problem` - The Problem Details object
    ///
    /// # Example
    /// ```
    /// use axum::http::StatusCode;
    /// use spikard_core::problem::ProblemDetails;
    /// use spikard_bindings_shared::ErrorResponseBuilder;
    ///
    /// let problem = ProblemDetails::not_found("User with id 123 not found");
    /// let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
    /// ```
    #[must_use]
    pub fn problem_details_response(problem: &ProblemDetails) -> (StatusCode, String) {
        let status = problem.status_code();
        let body = serde_json::to_string(problem).unwrap_or_else(|_| {
            r#"{"title":"Internal Server Error","type":"https://spikard.dev/errors/internal-server-error","status":500}"#
                .to_string()
        });
        (status, body)
    }

    /// Create a generic bad request error
    ///
    /// Returns (`StatusCode::BAD_REQUEST`, JSON body)
    pub fn bad_request(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    /// Create a generic internal server error
    ///
    /// Returns (`StatusCode::INTERNAL_SERVER_ERROR`, JSON body)
    pub fn internal_error(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    /// Create an unauthorized error
    ///
    /// Returns (`StatusCode::UNAUTHORIZED`, JSON body)
    pub fn unauthorized(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    /// Create a forbidden error
    ///
    /// Returns (`StatusCode::FORBIDDEN`, JSON body)
    pub fn forbidden(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::FORBIDDEN, "forbidden", message)
    }

    /// Create a not found error
    ///
    /// Returns (`StatusCode::NOT_FOUND`, JSON body)
    pub fn not_found(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// Create a method not allowed error
    ///
    /// Returns (`StatusCode::METHOD_NOT_ALLOWED`, JSON body)
    pub fn method_not_allowed(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed", message)
    }

    /// Create an unprocessable entity error (validation failed)
    ///
    /// Returns (`StatusCode::UNPROCESSABLE_ENTITY`, JSON body)
    pub fn unprocessable_entity(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::UNPROCESSABLE_ENTITY, "unprocessable_entity", message)
    }

    /// Create a conflict error
    ///
    /// Returns (`StatusCode::CONFLICT`, JSON body)
    pub fn conflict(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::CONFLICT, "conflict", message)
    }

    /// Create a service unavailable error
    ///
    /// Returns (`StatusCode::SERVICE_UNAVAILABLE`, JSON body)
    pub fn service_unavailable(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::SERVICE_UNAVAILABLE, "service_unavailable", message)
    }

    /// Create a request timeout error
    ///
    /// Returns (`StatusCode::REQUEST_TIMEOUT`, JSON body)
    pub fn request_timeout(message: impl Into<String>) -> (StatusCode, String) {
        Self::structured_error(StatusCode::REQUEST_TIMEOUT, "request_timeout", message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spikard_core::validation::ValidationErrorDetail;

    #[test]
    fn test_structured_error() {
        let (status, body) =
            ErrorResponseBuilder::structured_error(StatusCode::BAD_REQUEST, "invalid_input", "Missing required field");
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["error"], "Missing required field");
        assert_eq!(parsed["code"], "invalid_input");
        assert!(parsed["details"].is_object());
    }

    #[test]
    fn test_with_details() {
        let details = json!({
            "field": "email",
            "reason": "invalid_format"
        });
        let (status, body) = ErrorResponseBuilder::with_details(
            StatusCode::BAD_REQUEST,
            "validation_error",
            "Invalid email format",
            details,
        );
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "validation_error");
        assert_eq!(parsed["error"], "Invalid email format");
        assert_eq!(parsed["details"]["field"], "email");
        assert_eq!(parsed["details"]["reason"], "invalid_format");
    }

    #[test]
    fn test_from_structured_error() {
        let error = StructuredError::simple("test_error", "Something went wrong");
        let (status, body) = ErrorResponseBuilder::from_structured_error(&error);
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "test_error");
        assert_eq!(parsed["error"], "Something went wrong");
    }

    #[test]
    fn test_validation_error() {
        let validation_error = ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "missing".to_string(),
                loc: vec!["body".to_string(), "username".to_string()],
                msg: "Field required".to_string(),
                input: Value::String(String::new()),
                ctx: None,
            }],
        };

        let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["title"], "Request Validation Failed");
        assert_eq!(parsed["status"], 422);
        assert!(parsed["errors"].is_array());
    }

    #[test]
    fn test_problem_details_response() {
        let problem = ProblemDetails::not_found("User with id 123 not found");
        let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
        assert_eq!(status, StatusCode::NOT_FOUND);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["type"], "https://spikard.dev/errors/not-found");
        assert_eq!(parsed["title"], "Resource Not Found");
        assert_eq!(parsed["status"], 404);
    }

    #[test]
    fn test_bad_request() {
        let (status, body) = ErrorResponseBuilder::bad_request("Invalid data");
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "bad_request");
        assert_eq!(parsed["error"], "Invalid data");
    }

    #[test]
    fn test_internal_error() {
        let (status, body) = ErrorResponseBuilder::internal_error("Something went wrong");
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "internal_error");
        assert_eq!(parsed["error"], "Something went wrong");
    }

    #[test]
    fn test_unauthorized() {
        let (status, body) = ErrorResponseBuilder::unauthorized("Authentication required");
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "unauthorized");
    }

    #[test]
    fn test_forbidden() {
        let (status, body) = ErrorResponseBuilder::forbidden("Access denied");
        assert_eq!(status, StatusCode::FORBIDDEN);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "forbidden");
    }

    #[test]
    fn test_not_found() {
        let (status, body) = ErrorResponseBuilder::not_found("Resource not found");
        assert_eq!(status, StatusCode::NOT_FOUND);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "not_found");
    }

    #[test]
    fn test_method_not_allowed() {
        let (status, body) = ErrorResponseBuilder::method_not_allowed("Method POST not allowed");
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "method_not_allowed");
    }

    #[test]
    fn test_unprocessable_entity() {
        let (status, body) = ErrorResponseBuilder::unprocessable_entity("Validation failed");
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "unprocessable_entity");
    }

    #[test]
    fn test_conflict() {
        let (status, body) = ErrorResponseBuilder::conflict("Resource already exists");
        assert_eq!(status, StatusCode::CONFLICT);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "conflict");
    }

    #[test]
    fn test_service_unavailable() {
        let (status, body) = ErrorResponseBuilder::service_unavailable("Service temporarily unavailable");
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "service_unavailable");
    }

    #[test]
    fn test_request_timeout() {
        let (status, body) = ErrorResponseBuilder::request_timeout("Request timed out");
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["code"], "request_timeout");
    }

    #[test]
    fn test_serialization_fallback() {
        let details = serde_json::Map::new();
        let (_status, body) =
            ErrorResponseBuilder::with_details(StatusCode::BAD_REQUEST, "test", "Test error", Value::Object(details));

        assert!(serde_json::from_str::<Value>(&body).is_ok());
    }
}
