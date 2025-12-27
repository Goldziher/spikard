//! GraphQL HTTP handler integration with Spikard
//!
//! This module provides HTTP request handling for GraphQL queries and mutations,
//! implementing the `Handler` trait for integration with Spikard's HTTP server
//! and tower-http middleware stack.

use crate::error::GraphQLError;
use crate::GraphQLExecutor;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use spikard_http::{Handler, HandlerResult, RequestData};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// GraphQL request payload
///
/// Represents a standard GraphQL HTTP request body as defined by the GraphQL
/// specification for application/json requests.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GraphQLRequestPayload {
    /// The GraphQL query string
    pub query: String,

    /// Optional query variables as a JSON object
    #[serde(default)]
    pub variables: Option<Value>,

    /// Optional operation name for selecting which operation to execute
    /// when the query contains multiple operations
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

/// GraphQL response payload
///
/// Represents a standard GraphQL HTTP response body containing data
/// and optional errors.
#[derive(Debug, Serialize, Clone)]
pub struct GraphQLResponsePayload {
    /// Execution result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,

    /// Execution errors (null if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLErrorResponse>>,

    /// Extensions (optional, for tracing, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

/// GraphQL error response format
#[derive(Debug, Serialize, Clone)]
pub struct GraphQLErrorResponse {
    /// Error message
    pub message: String,

    /// Optional locations in the query where the error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Value>>,

    /// Optional path to the field that caused the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Value>>,

    /// Optional additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,
}

/// Parse GraphQL request from HTTP request body
///
/// # Arguments
///
/// * `raw_body` - Raw request body bytes
///
/// # Returns
///
/// Parsed GraphQL request payload or error
fn parse_graphql_request(raw_body: &[u8]) -> Result<GraphQLRequestPayload, GraphQLError> {
    serde_json::from_slice(raw_body).map_err(|e| {
        GraphQLError::RequestHandlingError(format!("Failed to parse GraphQL request: {e}"))
    })
}

/// GraphQL HTTP handler
///
/// Integrates GraphQL execution with Spikard's HTTP server and tower-http
/// middleware stack. Handles parsing GraphQL requests, executing them via
/// the GraphQL executor, and returning properly formatted responses.
///
/// # Type Parameters
///
/// * `Query` - The root GraphQL Query type
/// * `Mutation` - The root GraphQL Mutation type
/// * `Subscription` - The root GraphQL Subscription type
///
/// # Example
///
/// ```ignore
/// use spikard_graphql::{GraphQLHandler, GraphQLExecutor};
///
/// // With concrete types:
/// // let executor = GraphQLExecutor::<Query, Mutation, Subscription>::new(schema);
/// // let handler = GraphQLHandler::new(Arc::new(executor));
/// ```
#[derive(Debug)]
pub struct GraphQLHandler<Query, Mutation, Subscription> {
    executor: Arc<GraphQLExecutor<Query, Mutation, Subscription>>,
}

impl<Query, Mutation, Subscription> GraphQLHandler<Query, Mutation, Subscription>
where
    Query: Send + Sync + 'static,
    Mutation: Send + Sync + 'static,
    Subscription: Send + Sync + 'static,
{
    /// Create a new GraphQL handler
    ///
    /// # Arguments
    ///
    /// * `executor` - Arc-wrapped GraphQL executor instance
    ///
    /// # Returns
    ///
    /// A new `GraphQLHandler` instance
    #[must_use] 
    pub const fn new(executor: Arc<GraphQLExecutor<Query, Mutation, Subscription>>) -> Self {
        Self { executor }
    }

    /// Handle a GraphQL request
    ///
    /// # Arguments
    ///
    /// * `request_data` - The incoming request data
    ///
    /// # Returns
    ///
    /// Handler result with GraphQL response
    ///
    /// # Errors
    ///
    /// Returns `GraphQLError` if the request body cannot be parsed or execution fails.
    pub fn handle_graphql(&self, request_data: &RequestData) -> Result<Value, GraphQLError> {
        // Extract raw body
        let body_bytes = request_data
            .raw_body
            .as_ref()
            .map_or_else(
                || serde_json::to_vec(&request_data.body).unwrap_or_default(),
                |raw_body| raw_body.to_vec(),
            );

        // Parse the request payload
        let payload = parse_graphql_request(&body_bytes)?;

        // Execute the GraphQL query
        self.executor.execute(
            &payload.query,
            payload.variables.as_ref(),
            payload.operation_name.as_deref(),
        )
    }

    /// Create an HTTP response from a GraphQL result
    fn response_from_result(result: Result<Value, GraphQLError>) -> Response<Body> {
        match result {
            Ok(data) => {
                let response_payload = GraphQLResponsePayload {
                    data: Some(data),
                    errors: None,
                    extensions: None,
                };

                // Optimize serialization: avoid double JSON encoding (Value -> String -> Bytes)
                let body = serde_json::to_vec(&response_payload).unwrap_or_else(|_| {
                    serde_json::to_vec(&json!({"errors": [{"message": "Failed to serialize response"}]}))
                        .unwrap_or_else(|_| b"Internal server error".to_vec())
                });

                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap_or_else(|_| {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Internal server error"))
                            .unwrap()
                    })
            }
            Err(e) => {
                let error_response = GraphQLResponsePayload {
                    data: None,
                    errors: Some(vec![GraphQLErrorResponse {
                        message: e.to_string(),
                        locations: None,
                        path: None,
                        extensions: None,
                    }]),
                    extensions: None,
                };

                // Optimize serialization: avoid double JSON encoding (Value -> String -> Bytes)
                let body = serde_json::to_vec(&error_response).unwrap_or_else(|_| {
                    serde_json::to_vec(&json!({"errors": [{"message": "Internal server error"}]}))
                        .unwrap_or_else(|_| b"Internal server error".to_vec())
                });

                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap_or_else(|_| {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Internal server error"))
                            .unwrap()
                    })
            }
        }
    }
}

impl<Query, Mutation, Subscription> Clone for GraphQLHandler<Query, Mutation, Subscription> {
    fn clone(&self) -> Self {
        Self {
            executor: Arc::clone(&self.executor),
        }
    }
}

impl<Query, Mutation, Subscription> Handler for GraphQLHandler<Query, Mutation, Subscription>
where
    Query: Send + Sync + 'static,
    Mutation: Send + Sync + 'static,
    Subscription: Send + Sync + 'static,
{
    fn call(
        &self,
        _request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let result = self.handle_graphql(&request_data);
            Ok(Self::response_from_result(result))
        })
    }

    fn prefers_raw_json_body(&self) -> bool {
        true
    }

    fn wants_headers(&self) -> bool {
        false
    }

    fn wants_cookies(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphql_request_payload_parsing_minimal() {
        let json = r#"{"query":"{ hello }"}"#;
        let payload: GraphQLRequestPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.query, "{ hello }");
        assert!(payload.variables.is_none());
        assert!(payload.operation_name.is_none());
    }

    #[test]
    fn test_graphql_request_payload_parsing_with_variables() {
        let json = r#"{"query":"query GetUser($id: ID!) { user(id: $id) { name } }","variables":{"id":"123"}}"#;
        let payload: GraphQLRequestPayload = serde_json::from_str(json).unwrap();

        assert_eq!(
            payload.query,
            "query GetUser($id: ID!) { user(id: $id) { name } }"
        );
        assert!(payload.variables.is_some());
        assert_eq!(payload.variables.as_ref().unwrap()["id"], "123");
    }

    #[test]
    fn test_graphql_request_payload_parsing_with_operation_name() {
        let json = r#"{"query":"query GetUser { user { name } } query ListUsers { users { id } }","operationName":"GetUser"}"#;
        let payload: GraphQLRequestPayload = serde_json::from_str(json).unwrap();

        assert_eq!(payload.operation_name, Some("GetUser".to_string()));
    }

    #[test]
    fn test_graphql_request_payload_serialization() {
        let payload = GraphQLRequestPayload {
            query: "{ hello }".to_string(),
            variables: Some(json!({"test": "value"})),
            operation_name: Some("TestOp".to_string()),
        };

        let json = serde_json::to_string(&payload).unwrap();
        let parsed: GraphQLRequestPayload = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.query, payload.query);
        assert_eq!(parsed.variables, payload.variables);
        assert_eq!(parsed.operation_name, payload.operation_name);
    }

    #[test]
    fn test_graphql_response_payload_success() {
        let payload = GraphQLResponsePayload {
            data: Some(json!({"hello": "world"})),
            errors: None,
            extensions: None,
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"data\""));
        assert!(!json.contains("\"errors\""));
    }

    #[test]
    fn test_graphql_response_payload_with_errors() {
        let error = GraphQLErrorResponse {
            message: "Test error".to_string(),
            locations: None,
            path: None,
            extensions: None,
        };

        let payload = GraphQLResponsePayload {
            data: None,
            errors: Some(vec![error]),
            extensions: None,
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"errors\""));
        assert!(json.contains("Test error"));
    }

    #[test]
    fn test_parse_request_invalid_json() {
        let result = parse_graphql_request(b"invalid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_request_missing_query() {
        let result = parse_graphql_request(b"{}");
        assert!(result.is_err());
    }

    #[test]
    fn test_handler_prefers_raw_json_body() {
        let executor = Arc::new(GraphQLExecutor::<(), (), ()>::new(()));
        let handler = GraphQLHandler::new(executor);
        assert!(handler.prefers_raw_json_body());
    }

    #[test]
    fn test_handler_does_not_want_headers() {
        let executor = Arc::new(GraphQLExecutor::<(), (), ()>::new(()));
        let handler = GraphQLHandler::new(executor);
        assert!(!handler.wants_headers());
    }

    #[test]
    fn test_handler_does_not_want_cookies() {
        let executor = Arc::new(GraphQLExecutor::<(), (), ()>::new(()));
        let handler = GraphQLHandler::new(executor);
        assert!(!handler.wants_cookies());
    }

    #[test]
    fn test_handler_clone() {
        let executor = Arc::new(GraphQLExecutor::<(), (), ()>::new(()));
        let handler1 = GraphQLHandler::new(executor);
        let handler2 = handler1.clone();

        // Verify Arc is shared
        assert!(Arc::ptr_eq(&handler1.executor, &handler2.executor));
    }
}
