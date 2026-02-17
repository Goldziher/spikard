//! GraphQL executor for executing queries and mutations.
//!
//! This module provides a thin wrapper around `async-graphql` execution that
//! produces JSON payloads suitable for Spikard's HTTP handlers.

use async_graphql::{ObjectType, Request, Response, Schema, SubscriptionType, Variables};
use serde_json::Value;
use std::sync::Arc;

use crate::error::GraphQLError;

/// Generic GraphQL executor that wraps an `async-graphql` schema.
///
/// # Type Parameters
///
/// - `Query`: The root Query type
/// - `Mutation`: The root Mutation type
/// - `Subscription`: The root Subscription type
pub struct GraphQLExecutor<Query, Mutation, Subscription> {
    /// The underlying GraphQL schema, wrapped in Arc for thread safety.
    schema: Arc<Schema<Query, Mutation, Subscription>>,
}

impl<Query, Mutation, Subscription> std::fmt::Debug for GraphQLExecutor<Query, Mutation, Subscription> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphQLExecutor")
            .field("schema", &"<async_graphql::Schema>")
            .finish()
    }
}

impl<Query, Mutation, Subscription> GraphQLExecutor<Query, Mutation, Subscription>
where
    Query: ObjectType + Send + Sync + 'static,
    Mutation: ObjectType + Send + Sync + 'static,
    Subscription: SubscriptionType + Send + Sync + 'static,
{
    /// Create a new GraphQL executor from an `async-graphql` schema.
    #[must_use]
    pub fn new(schema: Schema<Query, Mutation, Subscription>) -> Self {
        Self {
            schema: Arc::new(schema),
        }
    }

    /// Execute a GraphQL query or mutation and return GraphQL-spec JSON.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `GraphQLError::ValidationError` for empty query strings
    /// - `GraphQLError::ComplexityLimitExceeded` when limits are hit
    /// - `GraphQLError::DepthLimitExceeded` when limits are hit
    /// - `GraphQLError::SerializationError` when response JSON conversion fails
    pub async fn execute(
        &self,
        query: &str,
        variables: Option<&Value>,
        operation_name: Option<&str>,
    ) -> Result<Value, GraphQLError> {
        if query.trim().is_empty() {
            return Err(GraphQLError::ValidationError(
                "Query string cannot be empty".to_string(),
            ));
        }

        let mut request = Request::new(query);
        if let Some(vars) = variables {
            request = request.variables(Variables::from_json(vars.clone()));
        }
        if let Some(name) = operation_name {
            request = request.operation_name(name);
        }

        let response = self.schema.execute(request).await;

        if let Some(limit_error) = map_query_limit_error(&response) {
            return Err(limit_error);
        }

        serde_json::to_value(response)
            .map_err(|error| GraphQLError::SerializationError(format!("Failed to serialize GraphQL response: {error}")))
    }

    /// Get a reference to the underlying schema.
    #[must_use]
    pub const fn schema_ref(&self) -> &Arc<Schema<Query, Mutation, Subscription>> {
        &self.schema
    }

    /// Clone the Arc to the schema for sharing across async tasks.
    #[must_use]
    pub fn clone_schema(&self) -> Arc<Schema<Query, Mutation, Subscription>> {
        Arc::clone(&self.schema)
    }
}

fn map_query_limit_error(response: &Response) -> Option<GraphQLError> {
    for error in &response.errors {
        let message = error.message.to_ascii_lowercase();
        if message.contains("complexity") {
            return Some(GraphQLError::ComplexityLimitExceeded);
        }
        if message.contains("depth") {
            return Some(GraphQLError::DepthLimitExceeded);
        }

        if let Some(extensions) = &error.extensions {
            let extension_text = format!("{extensions:?}").to_ascii_lowercase();
            if extension_text.contains("complexity") {
                return Some(GraphQLError::ComplexityLimitExceeded);
            }
            if extension_text.contains("depth") {
                return Some(GraphQLError::DepthLimitExceeded);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{EmptyMutation, EmptySubscription, Object};

    #[derive(Default)]
    struct TestQuery;

    #[Object]
    impl TestQuery {
        async fn hello(&self) -> &'static str {
            "world"
        }

        async fn greet(&self, name: String) -> String {
            format!("Hello, {name}")
        }
    }

    fn make_executor() -> GraphQLExecutor<TestQuery, EmptyMutation, EmptySubscription> {
        let schema = Schema::build(TestQuery, EmptyMutation, EmptySubscription).finish();
        GraphQLExecutor::new(schema)
    }

    #[tokio::test]
    async fn test_executor_creation() {
        let executor = make_executor();
        let _ = executor.schema_ref();
        let _ = executor.clone_schema();
    }

    #[tokio::test]
    async fn test_execute_with_empty_query_fails() {
        let executor = make_executor();
        let result = executor.execute("", None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_with_variables() {
        let executor = make_executor();
        let variables = serde_json::json!({ "name": "Alice" });
        let result = executor
            .execute("query($name: String!) { greet(name: $name) }", Some(&variables), None)
            .await
            .expect("query should execute");

        assert_eq!(result["data"]["greet"], "Hello, Alice");
    }

    #[tokio::test]
    async fn test_execute_with_operation_name() {
        let executor = make_executor();
        let result = executor
            .execute(
                "query First { hello } query Second { greet(name: \"Bob\") }",
                None,
                Some("First"),
            )
            .await
            .expect("query with operation name should execute");

        assert_eq!(result["data"]["hello"], "world");
    }
}
