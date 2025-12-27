//! Spikard GraphQL - GraphQL support for Spikard with async-graphql integration.
//!
//! This crate provides a high-level, type-safe GraphQL implementation built on
//! async-graphql, with support for:
//!
//! - Query, Mutation, and Subscription types
//! - Builder pattern for schema construction
//! - Introspection control
//! - Complexity and depth limits
//! - Federation support (via feature flag)
//! - Integration with Spikard's HTTP runtime
//!
//! # Features
//!
//! - `federation` - Enable Apollo Federation support

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    clippy::all
)]

pub mod error;
pub mod executor;
pub mod handler;
pub mod routes;
pub mod schema;

pub use error::{GraphQLError, Result as GraphQLResult};
pub use executor::GraphQLExecutor;
pub use handler::GraphQLHandler;
pub use routes::GraphQLRouteConfig;
pub use schema::{
    schema_full, schema_query_mutation, schema_query_only, FullSchemaConfig, QueryMutationConfig,
    QueryOnlyConfig, SchemaBuilder, SchemaConfig, SchemaError, SchemaResult,
};
