//! `PyO3` wrappers for GraphQL schema building
//!
//! Provides Python-friendly interfaces for constructing and configuring GraphQL schemas.

use pyo3::prelude::*;
use std::fmt;

/// Python wrapper for GraphQL schema configuration
///
/// Encapsulates all schema-level configuration options including
/// introspection control, complexity limits, and depth limits.
#[pyclass(name = "GraphQLSchemaConfig")]
#[derive(Clone, Debug)]
pub struct PySchemaConfig {
    /// Enable introspection queries
    #[pyo3(get, set)]
    pub introspection_enabled: bool,

    /// Maximum query complexity (`None` = unlimited)
    #[pyo3(get, set)]
    pub complexity_limit: Option<usize>,

    /// Maximum query depth (`None` = unlimited)
    #[pyo3(get, set)]
    pub depth_limit: Option<usize>,
}

#[pymethods]
impl PySchemaConfig {
    /// Create a new GraphQL schema configuration with default settings
    ///
    /// Defaults:
    /// - `introspection_enabled`: `True`
    /// - `complexity_limit`: `None` (unlimited)
    /// - `depth_limit`: `None` (unlimited)
    #[new]
    fn new() -> Self {
        Self {
            introspection_enabled: true,
            complexity_limit: None,
            depth_limit: None,
        }
    }

    /// Set the maximum complexity allowed for queries
    ///
    /// The complexity is calculated based on the query structure and field costs.
    /// Queries exceeding this limit will be rejected. Use 0 for unlimited.
    ///
    /// Args:
    ///     limit (int): The maximum complexity allowed (0 = unlimited)
    fn set_complexity(&mut self, limit: usize) {
        self.complexity_limit = if limit > 0 { Some(limit) } else { None };
    }

    /// Set the maximum depth allowed for queries
    ///
    /// The depth is the maximum nesting level of selections.
    /// Queries exceeding this limit will be rejected. Use 0 for unlimited.
    ///
    /// Args:
    ///     limit (int): The maximum depth allowed (0 = unlimited)
    fn set_depth(&mut self, limit: usize) {
        self.depth_limit = if limit > 0 { Some(limit) } else { None };
    }

    /// Check if introspection is enabled
    ///
    /// Returns:
    ///     bool: Whether introspection is enabled
    fn is_introspection_enabled(&self) -> bool {
        self.introspection_enabled
    }

    /// Get the complexity limit
    ///
    /// Returns:
    ///     int or `None`: The complexity limit, or `None` if unlimited
    fn get_complexity_limit(&self) -> Option<usize> {
        self.complexity_limit
    }

    /// Get the depth limit
    ///
    /// Returns:
    ///     int or `None`: The depth limit, or `None` if unlimited
    fn get_depth_limit(&self) -> Option<usize> {
        self.depth_limit
    }

    /// Validate the configuration
    ///
    /// Returns:
    ///     bool: `True` if configuration is valid
    ///
    /// Raises:
    ///     `ValueError`: If configuration is invalid
    fn validate(&self) -> PyResult<bool> {
        // Configuration is valid if introspection and limits are set
        // Add specific validation rules as needed
        Ok(true)
    }

    /// Get a string representation of the configuration
    fn __repr__(&self) -> String {
        format!(
            "GraphQLSchemaConfig(introspection_enabled={}, complexity_limit={}, depth_limit={})",
            self.introspection_enabled,
            self.complexity_limit
                .map_or_else(|| "None".to_string(), |l| l.to_string()),
            self.depth_limit.map_or_else(|| "None".to_string(), |l| l.to_string()),
        )
    }

    /// Get a string representation of the configuration
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl fmt::Display for PySchemaConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GraphQLSchemaConfig(introspection_enabled={}, complexity_limit={:?}, depth_limit={:?})",
            self.introspection_enabled, self.complexity_limit, self.depth_limit
        )
    }
}

/// Builder for constructing GraphQL schemas with fluent API
///
/// Provides a fluent interface for building schemas with Query, Mutation, and Subscription types.
/// Supports optional features like introspection, complexity limits, and depth limits.
///
/// # Example
///
/// ```python
/// from `spikard` import GraphQLSchemaBuilder
///
/// config = (GraphQLSchemaBuilder()
///     .enable_introspection(`True`)
///     .`complexity_limit`(5000)
///     .`depth_limit`(50)
///     .build())
/// ```
#[pyclass(name = "GraphQLSchemaBuilder")]
#[derive(Clone, Debug)]
pub struct PySchemaBuilder {
    config: PySchemaConfig,
}

#[pymethods]
impl PySchemaBuilder {
    /// Create a new schema builder with default configuration
    #[new]
    fn new() -> Self {
        Self {
            config: PySchemaConfig::new(),
        }
    }

    /// Enable or disable introspection
    ///
    /// Introspection is enabled by default. Disabling it prevents clients from
    /// querying the schema structure via introspection queries.
    ///
    /// Args:
    ///     enable (bool): Whether to enable introspection
    ///
    /// Returns:
    ///     self: The builder instance for method chaining
    fn enable_introspection(mut slf: PyRefMut<Self>, enable: bool) -> PyRefMut<Self> {
        slf.config.introspection_enabled = enable;
        slf
    }

    /// Set the maximum complexity allowed for queries
    ///
    /// The complexity is calculated based on the query structure and field costs.
    /// Queries exceeding this limit will be rejected. Use 0 for unlimited.
    ///
    /// Args:
    ///     limit (int): The maximum complexity allowed (0 = unlimited)
    ///
    /// Returns:
    ///     self: The builder instance for method chaining
    fn complexity_limit(mut slf: PyRefMut<Self>, limit: usize) -> PyRefMut<Self> {
        slf.config.complexity_limit = if limit > 0 { Some(limit) } else { None };
        slf
    }

    /// Set the maximum depth allowed for queries
    ///
    /// The depth is the maximum nesting level of selections.
    /// Queries exceeding this limit will be rejected. Use 0 for unlimited.
    ///
    /// Args:
    ///     limit (int): The maximum depth allowed (0 = unlimited)
    ///
    /// Returns:
    ///     self: The builder instance for method chaining
    fn depth_limit(mut slf: PyRefMut<Self>, limit: usize) -> PyRefMut<Self> {
        slf.config.depth_limit = if limit > 0 { Some(limit) } else { None };
        slf
    }

    /// Check if introspection is enabled
    ///
    /// Returns:
    ///     bool: Whether introspection is enabled
    fn is_introspection_enabled(&self) -> bool {
        self.config.is_introspection_enabled()
    }

    /// Get the complexity limit
    ///
    /// Returns:
    ///     int or `None`: The complexity limit, or `None` if unlimited
    fn get_complexity_limit(&self) -> Option<usize> {
        self.config.get_complexity_limit()
    }

    /// Get the depth limit
    ///
    /// Returns:
    ///     int or `None`: The depth limit, or `None` if unlimited
    fn get_depth_limit(&self) -> Option<usize> {
        self.config.get_depth_limit()
    }

    /// Get the underlying configuration
    ///
    /// Returns:
    ///     GraphQLSchemaConfig: The configuration object
    fn config(&self) -> PySchemaConfig {
        self.config.clone()
    }

    /// Build and return the schema configuration
    ///
    /// Returns:
    ///     GraphQLSchemaConfig: The finalized configuration
    fn build(&self) -> PySchemaConfig {
        self.config.clone()
    }

    /// Get a string representation of the builder
    fn __repr__(&self) -> String {
        format!("GraphQLSchemaBuilder({})", self.config)
    }
}

// Regular Rust methods for use in tests (not Python methods)
impl PySchemaBuilder {
    /// Enable or disable introspection (Rust version)
    pub fn enable_introspection_rs(&mut self, enable: bool) -> &mut Self {
        self.config.introspection_enabled = enable;
        self
    }

    /// Set the maximum complexity allowed for queries (Rust version)
    pub fn complexity_limit_rs(&mut self, limit: usize) -> &mut Self {
        self.config.complexity_limit = if limit > 0 { Some(limit) } else { None };
        self
    }

    /// Set the maximum depth allowed for queries (Rust version)
    pub fn depth_limit_rs(&mut self, limit: usize) -> &mut Self {
        self.config.depth_limit = if limit > 0 { Some(limit) } else { None };
        self
    }

    /// Get a string representation of the builder
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl fmt::Display for PySchemaBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GraphQLSchemaBuilder({})", self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_schema_config_new() {
        let config = PySchemaConfig::new();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, None);
        assert_eq!(config.depth_limit, None);
    }

    #[test]
    fn test_py_schema_config_display() {
        let config = PySchemaConfig::new();
        let s = format!("{}", config);
        assert!(s.contains("GraphQLSchemaConfig"));
        assert!(s.contains("introspection_enabled=true"));
    }

    #[test]
    fn test_py_schema_builder_new() {
        let builder = PySchemaBuilder::new();
        assert!(builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), None);
        assert_eq!(builder.get_depth_limit(), None);
    }

    #[test]
    fn test_py_schema_builder_enable_introspection() {
        let mut builder = PySchemaBuilder::new();
        builder.enable_introspection_rs(false);
        assert!(!builder.is_introspection_enabled());
    }

    #[test]
    fn test_py_schema_builder_complexity_limit() {
        let mut builder = PySchemaBuilder::new();
        builder.complexity_limit_rs(5000);
        assert_eq!(builder.get_complexity_limit(), Some(5000));
    }

    #[test]
    fn test_py_schema_builder_depth_limit() {
        let mut builder = PySchemaBuilder::new();
        builder.depth_limit_rs(50);
        assert_eq!(builder.get_depth_limit(), Some(50));
    }

    #[test]
    fn test_py_schema_builder_chaining() {
        let mut builder = PySchemaBuilder::new();
        builder.enable_introspection_rs(false);
        builder.complexity_limit_rs(3000);
        builder.depth_limit_rs(100);

        assert!(!builder.is_introspection_enabled());
        assert_eq!(builder.get_complexity_limit(), Some(3000));
        assert_eq!(builder.get_depth_limit(), Some(100));
    }

    #[test]
    fn test_py_schema_builder_build() {
        let mut builder = PySchemaBuilder::new();
        builder.complexity_limit_rs(5000);
        builder.depth_limit_rs(50);

        let config = builder.build();
        assert!(config.introspection_enabled);
        assert_eq!(config.complexity_limit, Some(5000));
        assert_eq!(config.depth_limit, Some(50));
    }

    #[test]
    fn test_py_schema_builder_display() {
        let builder = PySchemaBuilder::new();
        let s = format!("{}", builder);
        assert!(s.contains("GraphQLSchemaBuilder"));
    }

    #[test]
    fn test_py_schema_config_zero_limits() {
        let mut config = PySchemaConfig::new();
        config.set_complexity(0);
        config.set_depth(0);
        assert_eq!(config.complexity_limit, None);
        assert_eq!(config.depth_limit, None);
    }
}
