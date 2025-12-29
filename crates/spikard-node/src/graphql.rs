//! GraphQL bindings for Node.js using napi-rs
//!
//! This module provides napi-rs bindings for the spikard-graphql crate,
//! allowing Node.js/TypeScript to build and configure GraphQL schemas
//! with the same high-performance characteristics as the Rust core.
//!
//! # Features
//!
//! - Schema builder pattern matching Rust API
//! - Introspection control
//! - Complexity and depth limit configuration
//! - Zero-copy schema configuration serialization
//!
//! # Example (TypeScript)
//!
//! ```typescript
//! import { GraphQL } from 'spikard';
//!
//! const config = GraphQL.schemaConfig()
//!     .enableIntrospection(true)
//!     .complexityLimit(5000)
//!     .depthLimit(50)
//!     .finish();
//! ```

use napi_derive::napi;
use serde_json::{Value, json};
use std::fmt;

/// Configuration for GraphQL schema building
///
/// Encapsulates all schema-level configuration options including
/// introspection control, complexity limits, and depth limits.
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct SchemaConfig {
    /// Enable introspection queries
    #[napi(js_name = "introspectionEnabled")]
    pub introspection_enabled: Option<bool>,

    /// Maximum query complexity (0 = unlimited)
    #[napi(js_name = "complexityLimit")]
    pub complexity_limit: Option<u32>,

    /// Maximum query depth (0 = unlimited)
    #[napi(js_name = "depthLimit")]
    pub depth_limit: Option<u32>,
}

impl fmt::Display for SchemaConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SchemaConfig {{")?;
        write!(f, "introspection_enabled: {:?}, ", self.introspection_enabled)?;
        write!(f, "complexity_limit: {:?}, ", self.complexity_limit)?;
        write!(f, "depth_limit: {:?}", self.depth_limit)?;
        write!(f, "}}")
    }
}

/// Builder for GraphQL schema configuration
///
/// Provides a fluent interface for configuring GraphQL schemas.
/// This builder follows the same pattern as the Rust SchemaBuilder.
#[napi]
pub struct GraphQLSchemaBuilder { // codeql[rust/access-invalid-pointer] false positive: napi-rs wraps this in a managed JS object.
    introspection_enabled: bool,
    complexity_limit: Option<u32>,
    depth_limit: Option<u32>,
}

#[napi]
impl GraphQLSchemaBuilder {
    /// Create a new GraphQL schema builder with default settings
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Enable or disable GraphQL introspection
    ///
    /// Introspection is enabled by default. Disabling it prevents clients
    /// from querying the schema structure via introspection queries.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable introspection
    #[napi(js_name = "enableIntrospection")]
    pub fn enable_introspection(&mut self, enabled: bool) {
        self.introspection_enabled = enabled;
    }

    /// Set the maximum complexity allowed for queries
    ///
    /// The complexity is calculated based on the query structure and field costs.
    /// Queries exceeding this limit will be rejected.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum complexity allowed (0 = unlimited)
    #[napi(js_name = "complexityLimit")]
    pub fn complexity_limit(&mut self, limit: u32) {
        self.complexity_limit = if limit > 0 { Some(limit) } else { None };
    }

    /// Set the maximum depth allowed for queries
    ///
    /// The depth is the maximum nesting level of selections.
    /// Queries exceeding this limit will be rejected.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum depth allowed (0 = unlimited)
    #[napi(js_name = "depthLimit")]
    pub fn depth_limit(&mut self, limit: u32) {
        self.depth_limit = if limit > 0 { Some(limit) } else { None };
    }

    /// Get the current introspection setting
    #[napi(js_name = "isIntrospectionEnabled")]
    pub fn is_introspection_enabled(&self) -> bool {
        self.introspection_enabled
    }

    /// Get the current complexity limit if set
    #[napi(js_name = "getComplexityLimit")]
    pub fn get_complexity_limit(&self) -> Option<u32> {
        self.complexity_limit
    }

    /// Get the current depth limit if set
    #[napi(js_name = "getDepthLimit")]
    pub fn get_depth_limit(&self) -> Option<u32> {
        self.depth_limit
    }

    /// Build and return the schema configuration
    ///
    /// Returns a SchemaConfig object that can be serialized and passed
    /// to the GraphQL execution engine.
    ///
    /// # Returns
    ///
    /// A SchemaConfig instance with the configured settings
    pub fn finish(self) -> SchemaConfig {
        SchemaConfig {
            introspection_enabled: Some(self.introspection_enabled),
            complexity_limit: self.complexity_limit,
            depth_limit: self.depth_limit,
        }
    }

    /// Convert the schema configuration to a JSON object
    ///
    /// This method serializes the configuration to a JSON Value
    /// for passing to the Rust execution engine.
    ///
    /// # Returns
    ///
    /// A JSON representation of the schema configuration
    pub fn to_json(&self) -> Value {
        json!({
            "introspection_enabled": self.introspection_enabled,
            "complexity_limit": self.complexity_limit,
            "depth_limit": self.depth_limit,
        })
    }
}

impl Default for GraphQLSchemaBuilder {
    fn default() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

/// GraphQL utilities and factory functions
///
/// Provides factory methods for creating schema builders and configurations.
#[napi]
pub struct GraphQL; // codeql[rust/access-invalid-pointer] false positive: napi-rs wraps this in a managed JS object.

#[napi]
impl GraphQL {
    /// Create a new GraphQL schema builder
    ///
    /// Returns a builder instance that can be configured with various settings.
    ///
    /// # Example
    ///
    /// ```typescript
    /// const builder = GraphQL.schemaBuilder()
    ///     .enableIntrospection(true)
    ///     .complexityLimit(5000)
    ///     .depthLimit(50);
    /// ```
    #[napi(js_name = "schemaBuilder")]
    pub fn schema_builder() -> GraphQLSchemaBuilder {
        GraphQLSchemaBuilder {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Create a default schema configuration
    ///
    /// Returns a configuration with default settings:
    /// - Introspection enabled
    /// - No complexity limit
    /// - No depth limit
    ///
    /// # Returns
    ///
    /// A SchemaConfig with default settings
    #[napi(js_name = "defaultSchemaConfig")]
    pub fn default_schema_config() -> SchemaConfig {
        SchemaConfig {
            introspection_enabled: Some(true),
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Create a schema configuration for query-only schemas
    ///
    /// Returns a configuration suitable for schemas without mutations or subscriptions.
    ///
    /// # Returns
    ///
    /// A SchemaConfig optimized for query-only schemas
    #[napi(js_name = "queryOnlyConfig")]
    pub fn query_only_config() -> SchemaConfig {
        SchemaConfig {
            introspection_enabled: Some(true),
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Create a schema configuration for query and mutation schemas
    ///
    /// Returns a configuration suitable for schemas with queries and mutations
    /// but no subscriptions.
    ///
    /// # Returns
    ///
    /// A SchemaConfig optimized for query and mutation schemas
    #[napi(js_name = "queryMutationConfig")]
    pub fn query_mutation_config() -> SchemaConfig {
        SchemaConfig {
            introspection_enabled: Some(true),
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Create a full schema configuration
    ///
    /// Returns a configuration suitable for schemas with queries, mutations,
    /// and subscriptions.
    ///
    /// # Returns
    ///
    /// A SchemaConfig optimized for full-featured schemas
    #[napi(js_name = "fullSchemaConfig")]
    pub fn full_schema_config() -> SchemaConfig {
        SchemaConfig {
            introspection_enabled: Some(true),
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_builder_new() {
        let builder = GraphQLSchemaBuilder::new();
        assert!(builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), None);
        assert_eq!(builder.get_depth_limit(), None);
    }

    #[test]
    fn test_schema_builder_enable_introspection() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.enable_introspection(false);
        assert!(!builder.is_introspection_enabled());
    }

    #[test]
    fn test_schema_builder_complexity_limit() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.complexity_limit(5000);
        assert_eq!(builder.get_complexity_limit(), Some(5000));
    }

    #[test]
    fn test_schema_builder_depth_limit() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.depth_limit(50);
        assert_eq!(builder.get_depth_limit(), Some(50));
    }

    #[test]
    fn test_schema_builder_chaining() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.enable_introspection(false);
        builder.complexity_limit(3000);
        builder.depth_limit(100);

        assert!(!builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), Some(3000));
        assert_eq!(builder.get_depth_limit(), Some(100));
    }

    #[test]
    fn test_schema_builder_zero_limits() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.complexity_limit(0);
        builder.depth_limit(0);

        assert_eq!(builder.get_complexity_limit(), None);
        assert_eq!(builder.get_depth_limit(), None);
    }

    #[test]
    fn test_schema_builder_finish() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.enable_introspection(true);
        builder.complexity_limit(5000);
        builder.depth_limit(50);

        let config = builder.finish();
        assert_eq!(config.introspection_enabled, Some(true));
        assert_eq!(config.complexity_limit, Some(5000));
        assert_eq!(config.depth_limit, Some(50));
    }

    #[test]
    fn test_schema_config_to_json() {
        let mut builder = GraphQLSchemaBuilder::new();
        builder.complexity_limit(5000);
        builder.depth_limit(50);

        let json = builder.to_json();
        assert!(json.get("introspection_enabled").is_some());
        assert!(json.get("complexity_limit").is_some());
        assert!(json.get("depth_limit").is_some());
    }

    #[test]
    fn test_graphql_factory_methods() {
        let builder = GraphQL::schema_builder();
        assert!(builder.is_introspection_enabled());

        let config = GraphQL::default_schema_config();
        assert_eq!(config.introspection_enabled, Some(true));

        let query_config = GraphQL::query_only_config();
        assert_eq!(query_config.introspection_enabled, Some(true));

        let query_mutation = GraphQL::query_mutation_config();
        assert_eq!(query_mutation.introspection_enabled, Some(true));

        let full_config = GraphQL::full_schema_config();
        assert_eq!(full_config.introspection_enabled, Some(true));
    }

    #[test]
    fn test_schema_config_default() {
        let config = SchemaConfig::default();
        assert_eq!(config.introspection_enabled, None);
        assert_eq!(config.complexity_limit, None);
        assert_eq!(config.depth_limit, None);
    }

    #[test]
    fn test_schema_config_display() {
        let config = SchemaConfig {
            introspection_enabled: Some(true),
            complexity_limit: Some(5000),
            depth_limit: Some(50),
        };
        let display_str = format!("{}", config);
        assert!(display_str.contains("SchemaConfig"));
        assert!(display_str.contains("true"));
    }
}
