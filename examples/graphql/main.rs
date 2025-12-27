//! GraphQL integration example for Spikard HTTP server
//!
//! This example demonstrates how to integrate a GraphQL server with Spikard's
//! HTTP runtime and tower-http middleware stack.
//!
//! # Running the example
//!
//! ```bash
//! cargo run --example graphql --all-features
//! ```
//!
//! # Making requests
//!
//! ```bash
//! curl -X POST http://localhost:8000/graphql \
//!   -H "Content-Type: application/json" \
//!   -d '{"query":"{ hello }"}'
//! ```

use spikard_graphql::{
    GraphQLExecutor, GraphQLHandler, GraphQLRouteConfig, QueryOnlyConfig, SchemaBuilder,
};
use spikard_http::{Handler, Route, Router, Server, ServerConfig};
use std::sync::Arc;

/// Simple GraphQL Query type
#[derive(Debug, Clone)]
pub struct Query;

impl Query {
    /// Returns a greeting
    pub fn hello(&self) -> String {
        "Hello from Spikard GraphQL!".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a simple GraphQL schema with only a query root
    let schema_config = QueryOnlyConfig::builder()
        .introspection_enabled(true)
        .build();

    let schema = SchemaBuilder::new()
        .query_only(schema_config)
        .build();

    // Create the GraphQL executor with the schema
    let executor = Arc::new(GraphQLExecutor::<Query, (), ()>::new(schema));

    // Create the GraphQL handler - it implements the Handler trait
    let graphql_handler = GraphQLHandler::new(executor);

    // Create a route for the GraphQL endpoint
    // The handler is boxed as Arc<dyn Handler> for route registration
    let graphql_route = Route::new(
        "/graphql".to_string(),
        spikard_http::Method::Post,
        Arc::new(graphql_handler) as Arc<dyn Handler>,
    );

    // Register the route with the router
    let mut router = Router::new();
    router.register_route(graphql_route);

    // Create server configuration
    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(8000)
        .enable_http_trace(true)
        .build();

    // Create and start the HTTP server
    let server = Server::new(config, router);

    println!("GraphQL server starting on http://127.0.0.1:8000/graphql");
    println!("Try: curl -X POST http://127.0.0.1:8000/graphql \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"query\":\"{{ hello }}\"}}' ");

    server.run().await?;

    Ok(())
}
