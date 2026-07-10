//! Built-in GraphQL test schemas used by [`App::register_graphql_route`](crate::App::register_graphql_route).
//!
//! These concrete `async-graphql` root types exist purely to give bindings and the
//! generated e2e suite a ready-to-serve GraphQL endpoint without requiring every
//! host language to define its own schema DSL. They are intentionally simple and
//! mirror the fixtures in `fixtures/graphql_schema.json`.
//!
//! Do not extend these schemas with product behavior — they are test fixtures, not
//! part of Spikard's business logic surface.

use async_graphql::{Object, SimpleObject};

/// A user record served by the built-in test schemas.
#[derive(Debug, Clone, SimpleObject)]
pub struct TestUser {
    id: String,
    name: String,
    email: String,
    posts: Vec<TestPost>,
}

/// A post authored by a [`TestUser`].
#[derive(Debug, Clone, SimpleObject)]
pub struct TestPost {
    id: String,
    title: String,
    comments: Vec<TestComment>,
}

/// A comment on a [`TestPost`].
#[derive(Debug, Clone, SimpleObject)]
pub struct TestComment {
    id: String,
    text: String,
}

fn seed_user() -> TestUser {
    TestUser {
        id: "1".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        posts: vec![TestPost {
            id: "1".to_string(),
            title: "First Post".to_string(),
            comments: vec![TestComment {
                id: "1".to_string(),
                text: "Great post!".to_string(),
            }],
        }],
    }
}

/// Query root for the `query_only` built-in test schema.
///
/// Backs `fixtures/graphql_schema.json` cases tagged `query_only` as well as the
/// introspection/complexity/depth-limit cases, which all default to this schema.
#[derive(Debug, Default)]
pub struct QueryOnlyRoot;

#[Object(name = "Query")]
impl QueryOnlyRoot {
    /// Fetch the seeded test user. Returns `None` when `id` is provided and does
    /// not match the seeded user's id, matching the `not_found` fixture case.
    async fn user(&self, id: Option<String>) -> Option<TestUser> {
        let user = seed_user();
        match id {
            Some(requested) if requested != user.id => None,
            _ => Some(user),
        }
    }
}

/// Mutation root for the `query_mutation` built-in test schema.
#[derive(Debug, Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a user record. Returns a validation error for negative ages,
    /// matching the `invalid_input` fixture case.
    #[expect(clippy::unused_async, reason = "async-graphql field resolvers must be async")]
    async fn create_user(&self, name: String, age: Option<i32>) -> async_graphql::Result<TestUser> {
        if let Some(age) = age
            && age < 0
        {
            return Err(async_graphql::Error::new("age must not be negative"));
        }
        Ok(TestUser {
            id: "2".to_string(),
            name,
            email: String::new(),
            posts: Vec::new(),
        })
    }
}

/// Query root paired with [`MutationRoot`] for the `query_mutation` built-in test schema.
#[derive(Debug, Default)]
pub struct QueryMutationQueryRoot;

#[Object(name = "Query")]
impl QueryMutationQueryRoot {
    /// Fetch the seeded test user.
    async fn user(&self, id: Option<String>) -> Option<TestUser> {
        let user = seed_user();
        match id {
            Some(requested) if requested != user.id => None,
            _ => Some(user),
        }
    }
}

/// Query root for the `full` built-in test schema (query + mutation + subscription).
#[derive(Debug, Default)]
pub struct FullQueryRoot;

#[Object(name = "Query")]
impl FullQueryRoot {
    /// Fetch the seeded test user.
    async fn user(&self, id: Option<String>) -> Option<TestUser> {
        let user = seed_user();
        match id {
            Some(requested) if requested != user.id => None,
            _ => Some(user),
        }
    }
}

/// Selects which built-in async-graphql schema a `/graphql` route should serve.
///
/// This mirrors the `schema_type` field on the `graphql` route middleware in
/// `fixtures/graphql_schema.json` (`"query_only" | "query_mutation" | "full"`).
/// Any other value is rejected by [`crate::App::register_graphql_route`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinGraphQLSchema {
    /// Query-only schema (see [`QueryOnlyRoot`]).
    QueryOnly,
    /// Query + mutation schema (see [`QueryMutationQueryRoot`] and [`MutationRoot`]).
    QueryMutation,
    /// Query + mutation + subscription schema (see [`FullQueryRoot`]).
    Full,
}

impl BuiltinGraphQLSchema {
    /// Parse a `schema_type` string from route/middleware configuration.
    pub(crate) fn parse(schema_type: &str) -> Option<Self> {
        match schema_type {
            "query_only" => Some(Self::QueryOnly),
            "query_mutation" => Some(Self::QueryMutation),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
}

/// Apply shared [`spikard_graphql::SchemaConfig`] knobs (introspection/complexity/depth)
/// to an async-graphql `SchemaBuilder`.
pub fn apply_schema_config<Q, M, S>(
    builder: async_graphql::SchemaBuilder<Q, M, S>,
    config: &spikard_graphql::SchemaConfig,
) -> async_graphql::SchemaBuilder<Q, M, S>
where
    Q: async_graphql::ObjectType + 'static,
    M: async_graphql::ObjectType + 'static,
    S: async_graphql::SubscriptionType + 'static,
{
    let mut builder = builder.disable_introspection_only_when(!config.introspection_enabled);
    if let Some(limit) = config.complexity_limit {
        builder = builder.limit_complexity(limit);
    }
    if let Some(limit) = config.depth_limit {
        builder = builder.limit_depth(limit);
    }
    builder
}

/// Small extension trait to make introspection toggling read naturally at the call site.
trait DisableIntrospectionOnlyWhen {
    #[must_use]
    fn disable_introspection_only_when(self, disable: bool) -> Self;
}

impl<Q, M, S> DisableIntrospectionOnlyWhen for async_graphql::SchemaBuilder<Q, M, S>
where
    Q: async_graphql::ObjectType + 'static,
    M: async_graphql::ObjectType + 'static,
    S: async_graphql::SubscriptionType + 'static,
{
    fn disable_introspection_only_when(self, disable: bool) -> Self {
        if disable { self.disable_introspection() } else { self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_only() {
        assert_eq!(
            BuiltinGraphQLSchema::parse("query_only"),
            Some(BuiltinGraphQLSchema::QueryOnly)
        );
    }

    #[test]
    fn test_parse_query_mutation() {
        assert_eq!(
            BuiltinGraphQLSchema::parse("query_mutation"),
            Some(BuiltinGraphQLSchema::QueryMutation)
        );
    }

    #[test]
    fn test_parse_full() {
        assert_eq!(BuiltinGraphQLSchema::parse("full"), Some(BuiltinGraphQLSchema::Full));
    }

    #[test]
    fn test_parse_invalid() {
        assert_eq!(BuiltinGraphQLSchema::parse("invalid"), None);
    }
}
