//! Python bindings for GraphQL schema building and execution
//!
//! This module provides `PyO3` wrappers for spikard-graphql functionality,
//! enabling Python code to build and work with GraphQL schemas.
//!
//! # Features
//!
//! This module is only available when the `graphql` feature is enabled.

#[cfg(feature = "graphql")]
pub mod schema;

#[cfg(feature = "graphql")]
pub use schema::{PySchemaBuilder, PySchemaConfig};
