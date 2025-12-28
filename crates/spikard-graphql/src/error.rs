//! GraphQL error types and handling
//!
//! Provides error types compatible with async-graphql and HTTP response conversion.
//! All errors follow the GraphQL specification error format with extensions for
//! HTTP integration.

use serde_json::{Value, json};
use thiserror::Error;

/// Result type alias for GraphQL operations
pub type Result<T> = std::result::Result<T, GraphQLError>;

/// Errors that can occur during GraphQL operations
///
/// These errors are compatible with async-graphql error handling and can be
/// converted to structured HTTP responses matching the project's error fixtures.
#[derive(Error, Debug)]
pub enum GraphQLError {
    /// Error during schema execution
    ///
    /// Occurs when the GraphQL executor encounters a runtime error during query execution.
    #[error("execution error: {0}")]
    ExecutionError(String),

    /// Error during schema building
    ///
    /// Occurs when schema construction fails due to invalid definitions or conflicts.
    #[error("schema build error: {0}")]
    SchemaBuildError(String),

    /// Error during request handling
    ///
    /// Occurs when the HTTP request cannot be properly handled or parsed.
    #[error("request handling error: {0}")]
    RequestHandlingError(String),

    /// Serialization error
    ///
    /// Occurs during JSON serialization/deserialization of GraphQL values.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// JSON parsing error
    ///
    /// Occurs when JSON input cannot be parsed.
    #[error("JSON error: {0}")]
    JsonError(String),

    /// GraphQL validation error
    ///
    /// Occurs when a GraphQL query fails schema validation.
    #[error("GraphQL validation error: {0}")]
    ValidationError(String),

    /// GraphQL parse error
    ///
    /// Occurs when the GraphQL query string cannot be parsed.
    #[error("GraphQL parse error: {0}")]
    ParseError(String),

    /// Authentication error
    ///
    /// Occurs when request authentication fails.
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    /// Authorization error
    ///
    /// Occurs when user lacks required permissions.
    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    /// Not found error
    ///
    /// Occurs when a requested resource is not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Rate limit error
    ///
    /// Occurs when rate limit is exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Invalid input error with validation details
    ///
    /// Occurs during input validation with detailed error information.
    #[error("Invalid input: {message}")]
    InvalidInput {
        /// Error message
        message: String,
        /// Validation error details
        #[source]
        details: Option<Box<Self>>,
    },

    /// Query complexity limit exceeded
    ///
    /// Occurs when a GraphQL query exceeds the configured complexity limit.
    #[error("Query complexity limit exceeded")]
    ComplexityLimitExceeded,

    /// Query depth limit exceeded
    ///
    /// Occurs when a GraphQL query exceeds the configured depth limit.
    #[error("Query depth limit exceeded")]
    DepthLimitExceeded,

    /// Internal server error
    ///
    /// Occurs when an unexpected internal error happens.
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl GraphQLError {
    /// Convert error to HTTP status code
    ///
    /// Maps GraphQL error types to appropriate HTTP status codes:
    /// - 400: Bad Request for parse/validation errors
    /// - 401: Unauthorized for authentication errors
    /// - 403: Forbidden for authorization errors
    /// - 404: Not Found for resource not found
    /// - 422: Unprocessable Entity for validation failures
    /// - 429: Too Many Requests for rate limit errors
    /// - 500: Internal Server Error for execution/internal errors
    /// - 200: OK for GraphQL errors (per GraphQL spec)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_graphql::error::GraphQLError;
    ///
    /// let error = GraphQLError::AuthenticationError("Invalid token".to_string());
    /// assert_eq!(error.status_code(), 401);
    ///
    /// let error = GraphQLError::ExecutionError("Query failed".to_string());
    /// assert_eq!(error.status_code(), 200); // GraphQL spec: errors return 200 with errors in body
    /// ```
    #[must_use]
    pub const fn status_code(&self) -> u16 {
        match self {
            Self::ParseError(_) | Self::JsonError(_) => 400,
            Self::ValidationError(_)
            | Self::InvalidInput { .. }
            | Self::ComplexityLimitExceeded
            | Self::DepthLimitExceeded => 422,
            Self::AuthenticationError(_) => 401,
            Self::AuthorizationError(_) => 403,
            Self::NotFound(_) => 404,
            Self::RateLimitExceeded(_) => 429,
            Self::ExecutionError(_)
            | Self::SchemaBuildError(_)
            | Self::RequestHandlingError(_)
            | Self::SerializationError(_)
            | Self::InternalError(_) => 200, // GraphQL errors return 200 with errors in body
        }
    }

    /// Convert error to GraphQL error response JSON
    ///
    /// Returns a JSON object matching the GraphQL spec error format with
    /// structured extensions for HTTP integration.
    ///
    /// # Format
    ///
    /// ```json
    /// {
    ///   "errors": [
    ///     {
    ///       "message": "error message",
    ///       "extensions": {
    ///         "code": "ERROR_CODE",
    ///         "status": 400,
    ///         "type": "https://spikard.dev/errors/..."
    ///       }
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_graphql::error::GraphQLError;
    ///
    /// let error = GraphQLError::ValidationError("Invalid query".to_string());
    /// let json = error.to_graphql_response();
    /// assert!(json["errors"].is_array());
    /// ```
    #[must_use]
    pub fn to_graphql_response(&self) -> Value {
        json!({
            "errors": [{
                "message": self.to_string(),
                "extensions": {
                    "code": self.error_code(),
                    "status": self.status_code(),
                    "type": self.error_type_uri()
                }
            }]
        })
    }

    /// Convert error to structured HTTP error response
    ///
    /// Returns a JSON object matching the project's error fixture format,
    /// suitable for direct HTTP response conversion.
    ///
    /// # Format
    ///
    /// ```json
    /// {
    ///   "type": "https://spikard.dev/errors/...",
    ///   "title": "Error Title",
    ///   "status": 422,
    ///   "detail": "error message",
    ///   "errors": [
    ///     {
    ///       "type": "error_code",
    ///       "message": "error message"
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_graphql::error::GraphQLError;
    ///
    /// let error = GraphQLError::ValidationError("Invalid query".to_string());
    /// let json = error.to_http_response();
    /// assert_eq!(json["status"], 422);
    /// ```
    #[must_use]
    pub fn to_http_response(&self) -> Value {
        let status = self.status_code();
        let title = match self {
            Self::ParseError(_) | Self::JsonError(_) => "Bad Request",
            Self::ValidationError(_)
            | Self::InvalidInput { .. }
            | Self::ComplexityLimitExceeded
            | Self::DepthLimitExceeded => "Validation Failed",
            Self::AuthenticationError(_) => "Unauthorized",
            Self::AuthorizationError(_) => "Forbidden",
            Self::NotFound(_) => "Not Found",
            Self::RateLimitExceeded(_) => "Too Many Requests",
            Self::ExecutionError(_) | Self::InternalError(_) => "Internal Server Error",
            Self::SchemaBuildError(_) | Self::RequestHandlingError(_) | Self::SerializationError(_) => "Server Error",
        };

        json!({
            "type": self.error_type_uri(),
            "title": title,
            "status": status,
            "detail": self.to_string(),
            "errors": [{
                "type": self.error_code(),
                "message": self.to_string()
            }]
        })
    }

    /// Get the error code suitable for machine parsing
    ///
    /// Returns a screaming `SNAKE_CASE` error code that identifies the error type.
    const fn error_code(&self) -> &'static str {
        match self {
            Self::ParseError(_) => "GRAPHQL_PARSE_ERROR",
            Self::JsonError(_) => "JSON_ERROR",
            Self::ValidationError(_) => "GRAPHQL_VALIDATION_FAILED",
            Self::ExecutionError(_) => "GRAPHQL_EXECUTION_ERROR",
            Self::SchemaBuildError(_) => "GRAPHQL_SCHEMA_BUILD_ERROR",
            Self::RequestHandlingError(_) => "REQUEST_HANDLING_ERROR",
            Self::SerializationError(_) => "SERIALIZATION_ERROR",
            Self::AuthenticationError(_) => "AUTHENTICATION_FAILED",
            Self::AuthorizationError(_) => "AUTHORIZATION_FAILED",
            Self::NotFound(_) => "NOT_FOUND",
            Self::RateLimitExceeded(_) => "RATE_LIMIT_EXCEEDED",
            Self::InvalidInput { .. } => "VALIDATION_ERROR",
            Self::ComplexityLimitExceeded => "GRAPHQL_COMPLEXITY_LIMIT_EXCEEDED",
            Self::DepthLimitExceeded => "GRAPHQL_DEPTH_LIMIT_EXCEEDED",
            Self::InternalError(_) => "INTERNAL_SERVER_ERROR",
        }
    }

    /// Get the error type URI for structured error responses
    ///
    /// Returns a URI identifying the error type, following RFC 7231 conventions.
    const fn error_type_uri(&self) -> &'static str {
        match self {
            Self::ParseError(_) => "https://spikard.dev/errors/graphql-parse-error",
            Self::JsonError(_) => "https://spikard.dev/errors/json-error",
            Self::ValidationError(_) => "https://spikard.dev/errors/graphql-validation-error",
            Self::ExecutionError(_) => "https://spikard.dev/errors/graphql-execution-error",
            Self::SchemaBuildError(_) => "https://spikard.dev/errors/schema-build-error",
            Self::RequestHandlingError(_) => "https://spikard.dev/errors/request-handling-error",
            Self::SerializationError(_) => "https://spikard.dev/errors/serialization-error",
            Self::AuthenticationError(_) => "https://spikard.dev/errors/authentication-error",
            Self::AuthorizationError(_) => "https://spikard.dev/errors/authorization-error",
            Self::NotFound(_) => "https://spikard.dev/errors/not-found",
            Self::RateLimitExceeded(_) => "https://spikard.dev/errors/rate-limit-exceeded",
            Self::InvalidInput { .. } => "https://spikard.dev/errors/validation-error",
            Self::ComplexityLimitExceeded => "https://spikard.dev/errors/complexity-limit-exceeded",
            Self::DepthLimitExceeded => "https://spikard.dev/errors/depth-limit-exceeded",
            Self::InternalError(_) => "https://spikard.dev/errors/internal-server-error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_parse_error() {
        let error = GraphQLError::ParseError("Invalid syntax".to_string());
        assert_eq!(error.status_code(), 400);
    }

    #[test]
    fn test_status_code_validation_error() {
        let error = GraphQLError::ValidationError("Invalid query".to_string());
        assert_eq!(error.status_code(), 422);
    }

    #[test]
    fn test_status_code_authentication_error() {
        let error = GraphQLError::AuthenticationError("Invalid token".to_string());
        assert_eq!(error.status_code(), 401);
    }

    #[test]
    fn test_status_code_authorization_error() {
        let error = GraphQLError::AuthorizationError("Forbidden".to_string());
        assert_eq!(error.status_code(), 403);
    }

    #[test]
    fn test_status_code_not_found() {
        let error = GraphQLError::NotFound("User not found".to_string());
        assert_eq!(error.status_code(), 404);
    }

    #[test]
    fn test_status_code_rate_limit() {
        let error = GraphQLError::RateLimitExceeded("Too many requests".to_string());
        assert_eq!(error.status_code(), 429);
    }

    #[test]
    fn test_status_code_execution_error() {
        let error = GraphQLError::ExecutionError("Query execution failed".to_string());
        assert_eq!(error.status_code(), 200); // GraphQL spec
    }

    #[test]
    fn test_to_graphql_response_structure() {
        let error = GraphQLError::ValidationError("Invalid query".to_string());
        let response = error.to_graphql_response();

        assert!(response["errors"].is_array());
        assert_eq!(response["errors"].as_array().unwrap().len(), 1);
        assert!(response["errors"][0]["message"].is_string());
        assert!(response["errors"][0]["extensions"]["code"].is_string());
        assert_eq!(response["errors"][0]["extensions"]["code"], "GRAPHQL_VALIDATION_FAILED");
        assert_eq!(response["errors"][0]["extensions"]["status"], 422);
    }

    #[test]
    fn test_to_http_response_structure() {
        let error = GraphQLError::AuthenticationError("Invalid token".to_string());
        let response = error.to_http_response();

        assert_eq!(response["status"], 401);
        assert_eq!(response["title"], "Unauthorized");
        assert!(response["type"].is_string());
        assert!(response["errors"].is_array());
        assert_eq!(response["errors"][0]["type"], "AUTHENTICATION_FAILED");
    }

    #[test]
    fn test_error_code_serialization() {
        let error = GraphQLError::InvalidInput {
            message: "Field required".to_string(),
            details: None,
        };
        assert_eq!(error.error_code(), "VALIDATION_ERROR");
    }

    #[test]
    fn test_error_type_uri_parse_error() {
        let error = GraphQLError::ParseError("Invalid".to_string());
        assert_eq!(error.error_type_uri(), "https://spikard.dev/errors/graphql-parse-error");
    }

    #[test]
    fn test_json_error_creation() {
        let json_error = GraphQLError::JsonError("Invalid JSON".to_string());
        assert_eq!(json_error.error_code(), "JSON_ERROR");
        assert_eq!(json_error.status_code(), 400);
    }

    #[test]
    fn test_error_message_display() {
        let error = GraphQLError::ExecutionError("Query failed".to_string());
        assert_eq!(error.to_string(), "execution error: Query failed");
    }

    #[test]
    fn test_invalid_input_error_with_details() {
        let detail_error = Box::new(GraphQLError::ValidationError("Field required".to_string()));
        let error = GraphQLError::InvalidInput {
            message: "Invalid input provided".to_string(),
            details: Some(detail_error),
        };

        let response = error.to_http_response();
        assert_eq!(response["status"], 422);
        assert_eq!(response["title"], "Validation Failed");
    }

    #[test]
    fn test_rate_limit_error_status() {
        let error = GraphQLError::RateLimitExceeded("Limit: 100 requests/min".to_string());
        let response = error.to_http_response();
        assert_eq!(response["status"], 429);
        assert_eq!(response["title"], "Too Many Requests");
    }

    #[test]
    fn test_not_found_error_conversion() {
        let error = GraphQLError::NotFound("Product ID 123 not found".to_string());
        let response = error.to_http_response();
        assert_eq!(response["status"], 404);
        assert_eq!(response["title"], "Not Found");
        assert_eq!(response["errors"][0]["type"], "NOT_FOUND");
    }

    #[test]
    fn test_schema_build_error() {
        let error = GraphQLError::SchemaBuildError("Duplicate type definition".to_string());
        assert_eq!(error.status_code(), 200);
        let response = error.to_graphql_response();
        assert_eq!(
            response["errors"][0]["extensions"]["code"],
            "GRAPHQL_SCHEMA_BUILD_ERROR"
        );
    }

    #[test]
    fn test_complexity_limit_exceeded_status_code() {
        let error = GraphQLError::ComplexityLimitExceeded;
        assert_eq!(error.status_code(), 422);
        assert_eq!(error.error_code(), "GRAPHQL_COMPLEXITY_LIMIT_EXCEEDED");
    }

    #[test]
    fn test_depth_limit_exceeded_status_code() {
        let error = GraphQLError::DepthLimitExceeded;
        assert_eq!(error.status_code(), 422);
        assert_eq!(error.error_code(), "GRAPHQL_DEPTH_LIMIT_EXCEEDED");
    }

    #[test]
    fn test_complexity_limit_exceeded_response() {
        let error = GraphQLError::ComplexityLimitExceeded;
        let response = error.to_graphql_response();
        assert_eq!(
            response["errors"][0]["extensions"]["code"],
            "GRAPHQL_COMPLEXITY_LIMIT_EXCEEDED"
        );
        assert_eq!(response["errors"][0]["extensions"]["status"], 422);
    }

    #[test]
    fn test_depth_limit_exceeded_response() {
        let error = GraphQLError::DepthLimitExceeded;
        let response = error.to_graphql_response();
        assert_eq!(
            response["errors"][0]["extensions"]["code"],
            "GRAPHQL_DEPTH_LIMIT_EXCEEDED"
        );
        assert_eq!(response["errors"][0]["extensions"]["status"], 422);
    }

    #[test]
    fn test_complexity_limit_exceeded_error_type_uri() {
        let error = GraphQLError::ComplexityLimitExceeded;
        assert_eq!(
            error.error_type_uri(),
            "https://spikard.dev/errors/complexity-limit-exceeded"
        );
    }

    #[test]
    fn test_depth_limit_exceeded_error_type_uri() {
        let error = GraphQLError::DepthLimitExceeded;
        assert_eq!(
            error.error_type_uri(),
            "https://spikard.dev/errors/depth-limit-exceeded"
        );
    }

    #[test]
    fn test_all_error_codes_are_static() {
        let errors = vec![
            GraphQLError::ParseError(String::new()),
            GraphQLError::JsonError(String::new()),
            GraphQLError::ValidationError(String::new()),
            GraphQLError::ExecutionError(String::new()),
            GraphQLError::SchemaBuildError(String::new()),
            GraphQLError::RequestHandlingError(String::new()),
            GraphQLError::SerializationError(String::new()),
            GraphQLError::AuthenticationError(String::new()),
            GraphQLError::AuthorizationError(String::new()),
            GraphQLError::NotFound(String::new()),
            GraphQLError::RateLimitExceeded(String::new()),
            GraphQLError::InvalidInput {
                message: String::new(),
                details: None,
            },
            GraphQLError::InternalError(String::new()),
        ];

        for error in errors {
            let code = error.error_code();
            let response = error.to_graphql_response();
            assert_eq!(response["errors"][0]["extensions"]["code"].as_str(), Some(code));
        }
    }
}
