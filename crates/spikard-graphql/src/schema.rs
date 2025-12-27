//! GraphQL schema builder with support for Query, Mutation, and Subscription types.
//!
//! Provides a builder pattern for constructing async-graphql schemas with optional
//! features like introspection, complexity limits, depth limits, and federation support.
//!
//! # Examples
//!
//! ```ignore
//! use spikard_graphql::SchemaBuilder;
//!
//! let schema = SchemaBuilder::new(query, mutation, subscription)
//!     .enable_introspection(true)
//!     .complexity_limit(5000)
//!     .depth_limit(50)
//!     .finish();
//! ```

use std::fmt;
use thiserror::Error;

/// Error type for schema building operations
#[derive(Debug, Error)]
pub enum SchemaError {
    /// Generic schema building error
    #[error("Schema building failed: {0}")]
    BuildingFailed(String),

    /// Configuration validation error
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),

    /// Complexity limit exceeded
    #[error("Query complexity limit exceeded: limit={limit}, actual={actual}")]
    ComplexityLimitExceeded {
        /// The maximum allowed complexity
        limit: usize,
        /// The actual query complexity
        actual: usize,
    },

    /// Depth limit exceeded
    #[error("Query depth limit exceeded: limit={limit}, actual={actual}")]
    DepthLimitExceeded {
        /// The maximum allowed depth
        limit: usize,
        /// The actual query depth
        actual: usize,
    },
}

/// Result type for schema operations
pub type SchemaResult<T> = Result<T, SchemaError>;

/// Configuration for GraphQL schema building.
///
/// Encapsulates all schema-level configuration options including
/// introspection control, complexity limits, and depth limits.
#[derive(Debug, Clone)]
pub struct SchemaConfig {
    /// Enable introspection queries
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    pub complexity_limit: Option<usize>,
    /// Maximum query depth (None = unlimited)
    pub depth_limit: Option<usize>,
}

impl Default for SchemaConfig {
    fn default() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

impl SchemaConfig {
    /// Create a new default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable introspection
    pub const fn set_introspection_enabled(&mut self, enabled: bool) -> &mut Self {
        self.introspection_enabled = enabled;
        self
    }

    /// Set the complexity limit (0 means unlimited)
    pub const fn set_complexity_limit(&mut self, limit: usize) -> &mut Self {
        self.complexity_limit = if limit > 0 { Some(limit) } else { None };
        self
    }

    /// Set the depth limit (0 means unlimited)
    pub const fn set_depth_limit(&mut self, limit: usize) -> &mut Self {
        self.depth_limit = if limit > 0 { Some(limit) } else { None };
        self
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid (currently all configurations are valid)
    pub const fn validate(&self) -> SchemaResult<()> {
        // Configuration is valid if introspection and limits are set
        // Add specific validation rules as needed
        Ok(())
    }
}

/// Builder for constructing GraphQL schemas with async-graphql.
///
/// Provides a fluent interface for building schemas with Query, Mutation, and Subscription types.
/// Supports optional features like introspection, complexity limits, and depth limits.
///
/// This is a generic schema builder that can be used with any async-graphql compatible types.
///
/// # Type Parameters
///
/// * `Query` - The GraphQL Query root type
/// * `Mutation` - The GraphQL Mutation root type
/// * `Subscription` - The GraphQL Subscription root type
///
/// # Examples
///
/// ```ignore
/// let schema = SchemaBuilder::new(query, mutation, subscription)
///     .enable_introspection(true)
///     .complexity_limit(5000)
///     .depth_limit(50)
///     .finish();
/// ```
pub struct SchemaBuilder<Query, Mutation, Subscription> {
    config: SchemaConfig,
    _query: std::marker::PhantomData<Query>,
    _mutation: std::marker::PhantomData<Mutation>,
    _subscription: std::marker::PhantomData<Subscription>,
}

impl<Query, Mutation, Subscription> SchemaBuilder<Query, Mutation, Subscription> {
    /// Create a new schema builder with the specified root types.
    ///
    /// # Arguments
    ///
    /// * `_query` - The Query root type (for type tracking)
    /// * `_mutation` - The Mutation root type (for type tracking)
    /// * `_subscription` - The Subscription root type (for type tracking)
    ///
    /// # Returns
    ///
    /// A new `SchemaBuilder` instance
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let builder = SchemaBuilder::new(query, mutation, subscription);
    /// ```
    #[must_use]
    pub fn new(_query: Query, _mutation: Mutation, _subscription: Subscription) -> Self {
        Self {
            config: SchemaConfig::default(),
            _query: std::marker::PhantomData,
            _mutation: std::marker::PhantomData,
            _subscription: std::marker::PhantomData,
        }
    }

    /// Enable or disable introspection.
    ///
    /// Introspection is enabled by default. Disabling it prevents clients from
    /// querying the schema structure via introspection queries.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable introspection
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[must_use]
    pub const fn enable_introspection(mut self, enable: bool) -> Self {
        self.config.introspection_enabled = enable;
        self
    }

    /// Set the maximum complexity allowed for queries.
    ///
    /// The complexity is calculated based on the query structure and field costs.
    /// Queries exceeding this limit will be rejected.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum complexity allowed (0 means unlimited)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[must_use]
    pub const fn complexity_limit(mut self, limit: usize) -> Self {
        self.config.set_complexity_limit(limit);
        self
    }

    /// Set the maximum depth allowed for queries.
    ///
    /// The depth is the maximum nesting level of selections.
    /// Queries exceeding this limit will be rejected.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum depth allowed (0 means unlimited)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    #[must_use]
    pub const fn depth_limit(mut self, limit: usize) -> Self {
        self.config.set_depth_limit(limit);
        self
    }

    /// Get the current introspection setting.
    ///
    /// # Returns
    ///
    /// Whether introspection is enabled
    #[must_use]
    pub const fn is_introspection_enabled(&self) -> bool {
        self.config.introspection_enabled
    }

    /// Get the current complexity limit if set.
    ///
    /// # Returns
    ///
    /// The complexity limit, or None if unlimited
    #[must_use]
    pub const fn get_complexity_limit(&self) -> Option<usize> {
        self.config.complexity_limit
    }

    /// Get the current depth limit if set.
    ///
    /// # Returns
    ///
    /// The depth limit, or None if unlimited
    #[must_use]
    pub const fn get_depth_limit(&self) -> Option<usize> {
        self.config.depth_limit
    }

    /// Get the underlying configuration
    ///
    /// # Returns
    ///
    /// A reference to the `SchemaConfig`
    #[must_use]
    pub const fn config(&self) -> &SchemaConfig {
        &self.config
    }

    /// Build the schema configuration (does not construct async-graphql Schema directly).
    ///
    /// Returns the configuration that should be applied to an async-graphql `SchemaBuilder`.
    /// The actual Schema construction must be done by the caller using the async-graphql API.
    ///
    /// # Returns
    ///
    /// The `SchemaConfig` instance
    #[must_use]
    pub const fn finish(self) -> SchemaConfig {
        self.config
    }
}

impl<Query, Mutation, Subscription> fmt::Debug for SchemaBuilder<Query, Mutation, Subscription> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SchemaBuilder")
            .field("config", &self.config)
            .finish()
    }
}

/// Configuration for schemas with only Query type
#[derive(Debug, Clone)]
pub struct QueryOnlyConfig {
    /// Enable introspection queries
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    pub complexity_limit: Option<usize>,
    /// Maximum query depth (None = unlimited)
    pub depth_limit: Option<usize>,
}

impl Default for QueryOnlyConfig {
    fn default() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

/// Create a simple schema configuration with only Query type.
///
/// This is a convenience function for schemas that only have queries.
///
/// # Returns
///
/// A `QueryOnlyConfig` with default settings
#[must_use]
pub fn schema_query_only() -> QueryOnlyConfig {
    QueryOnlyConfig::default()
}

/// Configuration for schemas with Query and Mutation types
#[derive(Debug, Clone)]
pub struct QueryMutationConfig {
    /// Enable introspection queries
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    pub complexity_limit: Option<usize>,
    /// Maximum query depth (None = unlimited)
    pub depth_limit: Option<usize>,
}

impl Default for QueryMutationConfig {
    fn default() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

/// Create a schema configuration with Query and Mutation types.
///
/// This is a convenience function for schemas with queries and mutations but no subscriptions.
///
/// # Returns
///
/// A `QueryMutationConfig` with default settings
#[must_use]
pub fn schema_query_mutation() -> QueryMutationConfig {
    QueryMutationConfig::default()
}

/// Configuration for fully-featured schemas with Query, Mutation, and Subscription types
#[derive(Debug, Clone)]
pub struct FullSchemaConfig {
    /// Enable introspection queries
    pub introspection_enabled: bool,
    /// Maximum query complexity (None = unlimited)
    pub complexity_limit: Option<usize>,
    /// Maximum query depth (None = unlimited)
    pub depth_limit: Option<usize>,
}

impl Default for FullSchemaConfig {
    fn default() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }
}

/// Create a schema configuration with all three root types.
///
/// This is a convenience function for fully-featured schemas.
///
/// # Returns
///
/// A `FullSchemaConfig` with default settings
#[must_use]
pub fn schema_full() -> FullSchemaConfig {
    FullSchemaConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_config_creation() {
        let config = SchemaConfig::new();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, None);
        assert_eq!(config.depth_limit, None);
    }

    #[test]
    fn test_schema_config_validation() {
        let config = SchemaConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_schema_builder_creation() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription);
        assert!(builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), None);
        assert_eq!(builder.get_depth_limit(), None);
    }

    #[test]
    fn test_schema_builder_disable_introspection() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .enable_introspection(false);
        assert!(!builder.is_introspection_enabled());
    }

    #[test]
    fn test_schema_builder_complexity_limit() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .complexity_limit(5000);
        assert_eq!(builder.get_complexity_limit(), Some(5000));
    }

    #[test]
    fn test_schema_builder_depth_limit() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .depth_limit(50);
        assert_eq!(builder.get_depth_limit(), Some(50));
    }

    #[test]
    fn test_schema_builder_chaining() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .enable_introspection(false)
            .complexity_limit(3000)
            .depth_limit(100);

        assert!(!builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), Some(3000));
        assert_eq!(builder.get_depth_limit(), Some(100));
    }

    #[test]
    fn test_schema_builder_zero_limits_are_ignored() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .complexity_limit(0)
            .depth_limit(0);

        assert_eq!(builder.get_complexity_limit(), None);
        assert_eq!(builder.get_depth_limit(), None);
    }

    #[test]
    fn test_schema_builder_debug() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let builder = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .complexity_limit(5000)
            .depth_limit(50);

        let debug_str = format!("{builder:?}");
        assert!(debug_str.contains("SchemaBuilder"));
        assert!(debug_str.contains("config"));
    }

    #[test]
    fn test_schema_builder_finish() {
        struct DummyQuery;
        struct DummyMutation;
        struct DummySubscription;

        let config = SchemaBuilder::new(DummyQuery, DummyMutation, DummySubscription)
            .complexity_limit(5000)
            .depth_limit(50)
            .finish();

        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, Some(5000));
        assert_eq!(config.depth_limit, Some(50));
    }

    #[test]
    fn test_query_only_config() {
        let config = schema_query_only();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, None);
    }

    #[test]
    fn test_query_mutation_config() {
        let config = schema_query_mutation();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, None);
    }

    #[test]
    fn test_full_schema_config() {
        let config = schema_full();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, None);
    }

    #[test]
    fn test_schema_error_display() {
        let err = SchemaError::ComplexityLimitExceeded {
            limit: 5000,
            actual: 6000,
        };
        let msg = err.to_string();
        assert!(msg.contains("5000"));
        assert!(msg.contains("6000"));
    }
}
