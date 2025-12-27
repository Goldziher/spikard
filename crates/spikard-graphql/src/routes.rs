//! GraphQL route configuration and integration helpers
//!
//! This module provides utilities to integrate GraphQL handlers with the Spikard HTTP server
//! and tower-http middleware stack. It exposes route builders and handler wrappers that allow
//! GraphQL to be used as a first-class citizen alongside other HTTP routes.
//!
//! # Integration Overview
//!
//! The GraphQL handler integrates seamlessly with Spikard's HTTP runtime:
//!
//! 1. **`GraphQLHandler` implements the Handler trait** - Enabling it to work with the HTTP server
//! 2. **Full middleware support** - Works with tower-http middleware (compression, rate limiting, etc.)
//! 3. **Request/response handling** - Proper conversion between HTTP and GraphQL formats
//! 4. **Error handling** - Structured error responses following GraphQL spec
//! 5. **Zero-copy serialization** - Efficient conversion using serde and msgspec
//!
//! # Example: Basic Setup
//!
//! ```ignore
//! use spikard_graphql::{GraphQLHandler, GraphQLExecutor, QueryOnlyConfig, SchemaBuilder};
//! use spikard_http::{Handler, Route, Router, Server, ServerConfig};
//! use std::sync::Arc;
//!
//! // 1. Create GraphQL schema
//! let schema = SchemaBuilder::new()
//!     .query_only(QueryOnlyConfig::builder().build())
//!     .build();
//!
//! // 2. Create executor and handler
//! let executor = Arc::new(GraphQLExecutor::<Query, (), ()>::new(schema));
//! let handler = Arc::new(GraphQLHandler::new(executor));
//!
//! // 3. Register route with Spikard
//! let route = Route::new(
//!     "/graphql".to_string(),
//!     spikard_http::Method::Post,
//!     handler as Arc<dyn Handler>,
//! );
//!
//! let mut router = Router::new();
//! router.register_route(route);
//!
//! // 4. Start server
//! let config = ServerConfig::builder()
//!     .port(8000)
//!     .build();
//! let server = Server::new(config, router);
//! server.run().await?;
//! ```
//!
//! # Example: With Route Configuration
//!
//! ```ignore
//! use spikard_graphql::routes::GraphQLRouteConfig;
//!
//! // Use GraphQLRouteConfig for declarative configuration
//! let config = GraphQLRouteConfig::new()
//!     .path("/api/graphql")
//!     .method("POST")
//!     .enable_playground(false)
//!     .description("Main GraphQL endpoint");
//!
//! assert_eq!(config.get_path(), "/api/graphql");
//! assert!(config.get_description().is_some());
//! ```
//!
//! # Middleware Integration
//!
//! The GraphQL handler works seamlessly with tower-http middleware:
//!
//! ```ignore
//! let config = ServerConfig::builder()
//!     .compression(Some(CompressionConfig {
//!         enabled: true,
//!         ..Default::default()
//!     }))
//!     .rate_limit(Some(RateLimitConfig {
//!         requests_per_second: 100,
//!         ..Default::default()
//!     }))
//!     .build();
//! ```
//!
//! All middleware is applied transparently to GraphQL requests.
//!
//! # Request/Response Flow
//!
//! ```text
//! HTTP Request (POST /graphql)
//!     |
//!     v
//! Tower-HTTP Middleware (compression, rate limiting, etc.)
//!     |
//!     v
//! GraphQLHandler::call() (implements Handler trait)
//!     |
//!     v
//! Parse GraphQL request JSON
//!     |
//!     v
//! Extract query, variables, operation name
//!     |
//!     v
//! GraphQLExecutor::execute()
//!     |
//!     v
//! Format GraphQL response
//!     |
//!     v
//! HTTP Response (200 OK with JSON)
//! ```
//!
//! # Error Handling
//!
//! GraphQL errors are properly formatted and returned with appropriate HTTP status codes:
//!
//! - **Parse errors** (400 Bad Request) - Invalid JSON or GraphQL syntax
//! - **Execution errors** (200 OK) - Errors in query execution (following GraphQL spec)
//! - **Internal errors** (500 Internal Server Error) - Unexpected server errors
//!
//! # See Also
//!
//! - [`GraphQLHandler`](crate::handler::GraphQLHandler) - The Handler implementation
//! - [`GraphQLExecutor`](crate::executor::GraphQLExecutor) - Query execution
//! - [spikard-http Route documentation](https://docs.rs/spikard-http)

/// Configuration for GraphQL routes
///
/// Provides a builder pattern for configuring GraphQL route parameters
/// while maintaining compatibility with the Spikard HTTP server's routing system.
///
/// # Example
///
/// ```
/// use spikard_graphql::routes::GraphQLRouteConfig;
///
/// let config = GraphQLRouteConfig::new()
///     .path("/graphql")
///     .method("POST")
///     .enable_playground(true);
///
/// assert_eq!(config.get_path(), "/graphql");
/// assert_eq!(config.get_method(), "POST");
/// ```
#[derive(Debug, Clone)]
pub struct GraphQLRouteConfig {
    /// HTTP path for the GraphQL endpoint
    path: String,

    /// HTTP method (typically "POST")
    method: String,

    /// Enable GraphQL Playground (if supported by executor)
    enable_playground: bool,

    /// Custom description for documentation
    description: Option<String>,
}

impl GraphQLRouteConfig {
    /// Create a new GraphQL route configuration with defaults
    ///
    /// Default values:
    /// - path: "/graphql"
    /// - method: "POST"
    /// - `enable_playground`: false
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: "/graphql".to_string(),
            method: "POST".to_string(),
            enable_playground: false,
            description: None,
        }
    }

    /// Set the HTTP path for the GraphQL endpoint
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path (e.g., "/graphql", "/api/graphql")
    #[must_use]
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Set the HTTP method for the GraphQL endpoint
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (typically "POST")
    #[must_use]
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    /// Enable or disable the GraphQL Playground UI
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable playground
    #[must_use]
    pub const fn enable_playground(mut self, enable: bool) -> Self {
        self.enable_playground = enable;
        self
    }

    /// Set a custom description for documentation
    ///
    /// # Arguments
    ///
    /// * `description` - Documentation string
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get the configured path
    #[must_use] 
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Get the configured method
    #[must_use] 
    pub fn get_method(&self) -> &str {
        &self.method
    }

    /// Check if playground is enabled
    #[must_use] 
    pub const fn is_playground_enabled(&self) -> bool {
        self.enable_playground
    }

    /// Get the description if set
    #[must_use] 
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl Default for GraphQLRouteConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphql_route_config_defaults() {
        let config = GraphQLRouteConfig::new();
        assert_eq!(config.get_path(), "/graphql");
        assert_eq!(config.get_method(), "POST");
        assert!(!config.is_playground_enabled());
        assert!(config.get_description().is_none());
    }

    #[test]
    fn test_graphql_route_config_builder() {
        let config = GraphQLRouteConfig::new()
            .path("/api/graphql")
            .method("POST")
            .enable_playground(true)
            .description("Main GraphQL API");

        assert_eq!(config.get_path(), "/api/graphql");
        assert_eq!(config.get_method(), "POST");
        assert!(config.is_playground_enabled());
        assert_eq!(config.get_description(), Some("Main GraphQL API"));
    }

    #[test]
    fn test_graphql_route_config_default_impl() {
        let config = GraphQLRouteConfig::default();
        assert_eq!(config.get_path(), "/graphql");
        assert_eq!(config.get_method(), "POST");
    }

    #[test]
    fn test_graphql_route_config_clone() {
        let config1 = GraphQLRouteConfig::new()
            .path("/graphql")
            .description("Test API");
        let config2 = config1.clone();

        assert_eq!(config1.get_path(), config2.get_path());
        assert_eq!(config1.get_description(), config2.get_description());
    }
}
