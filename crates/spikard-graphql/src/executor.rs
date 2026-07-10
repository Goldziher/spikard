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
    /// Whether introspection queries (`__schema`, `__type`) are permitted.
    ///
    /// `async-graphql`'s own `disable_introspection()` silently returns `null`
    /// for introspection fields rather than raising an error. Spikard instead
    /// rejects introspection queries outright when disabled, matching the
    /// project's GraphQL error-response fixtures.
    introspection_enabled: bool,
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
    /// Create a new GraphQL executor from an `async-graphql` schema, with introspection permitted.
    #[must_use]
    pub fn new(schema: Schema<Query, Mutation, Subscription>) -> Self {
        Self {
            schema: Arc::new(schema),
            introspection_enabled: true,
        }
    }

    /// Create a new GraphQL executor with explicit control over introspection.
    ///
    /// When `introspection_enabled` is `false`, queries selecting `__schema` or
    /// `__type` are rejected with `GraphQLError::ValidationError` instead of the
    /// underlying `async-graphql` behavior of silently returning `null`.
    #[must_use]
    pub fn with_introspection(schema: Schema<Query, Mutation, Subscription>, introspection_enabled: bool) -> Self {
        Self {
            schema: Arc::new(schema),
            introspection_enabled,
        }
    }

    /// Execute a GraphQL query or mutation and return GraphQL-spec JSON.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `GraphQLError::ValidationError` for empty query strings
    /// - `GraphQLError::IntrospectionDisabled` for introspection queries
    ///   (`__schema`/`__type`) when introspection has been disabled
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

        if !self.introspection_enabled && is_introspection_query(query) {
            return Err(GraphQLError::IntrospectionDisabled);
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

/// Heuristically detect whether a GraphQL query document selects an introspection
/// field (`__schema` or `__type`) at any point in the document.
///
/// This is a lightweight lexical scan rather than a full GraphQL parse: it looks
/// for the `__schema`/`__type` tokens bounded by non-identifier characters, which
/// is sufficient to distinguish introspection field selections from unrelated
/// identifiers containing the same substrings (e.g. a field named `mySchemaType`).
fn is_introspection_query(query: &str) -> bool {
    for needle in ["__schema", "__type"] {
        let mut search_from = 0;
        while let Some(relative_index) = query[search_from..].find(needle) {
            let start = search_from + relative_index;
            let end = start + needle.len();
            let preceded_by_identifier_char = start > 0 && is_identifier_byte(query.as_bytes()[start - 1]);
            let followed_by_identifier_char = query.as_bytes().get(end).is_some_and(|&b| is_identifier_byte(b));
            if !preceded_by_identifier_char && !followed_by_identifier_char {
                return true;
            }
            search_from = end;
        }
    }
    false
}

/// Whether a byte can continue a GraphQL identifier (alphanumeric or underscore).
///
/// Small helper to bound token matches without pulling in a regex crate.
const fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn map_query_limit_error(response: &Response) -> Option<GraphQLError> {
    for error in &response.errors {
        let message = error.message.to_ascii_lowercase();
        if message.contains("complex") {
            return Some(GraphQLError::ComplexityLimitExceeded);
        }
        if message.contains("depth") || message.contains("too deep") {
            return Some(GraphQLError::DepthLimitExceeded);
        }

        if let Some(extensions) = &error.extensions {
            let extension_text = format!("{extensions:?}").to_ascii_lowercase();
            if extension_text.contains("complex") {
                return Some(GraphQLError::ComplexityLimitExceeded);
            }
            if extension_text.contains("depth") || extension_text.contains("too deep") {
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
