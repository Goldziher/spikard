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

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use serde_json::json;
use spikard_graphql::{GraphQLExecutor, GraphQLHandler};
use spikard_http::{Handler, Route, RouteMetadata, SchemaRegistry, Server, ServerConfig};
use std::sync::Arc;

/// Simple GraphQL Query type
#[derive(Debug, Clone, Default)]
pub struct Query;

#[Object]
impl Query {
    /// Returns a greeting
    async fn hello(&self) -> String {
        "Hello from Spikard GraphQL!".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a simple GraphQL schema with only a query root.
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish();

    // Create the GraphQL executor with the schema
    let executor = Arc::new(GraphQLExecutor::<Query, EmptyMutation, EmptySubscription>::new(schema));

    // Create the GraphQL handler - it implements the Handler trait
    let graphql_handler = GraphQLHandler::new(executor);

    let route_metadata: RouteMetadata = serde_json::from_value(json!({
        "method": "POST",
        "path": "/graphql",
        "handler_name": "graphql_handler",
        "is_async": true
    }))?;
    let registry = SchemaRegistry::new();
    let graphql_route = Route::from_metadata(route_metadata.clone(), &registry).map_err(std::io::Error::other)?;
    let graphql_handler = Arc::new(graphql_handler) as Arc<dyn Handler>;

    // Create server configuration
    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(8000)
        .enable_http_trace(true)
        .build();

    let app = Server::with_handlers_and_metadata(
        config.clone(),
        vec![(graphql_route, graphql_handler)],
        vec![route_metadata],
    )
    .map_err(std::io::Error::other)?;

    println!("GraphQL server starting on http://127.0.0.1:8000/graphql");
    println!("Try: curl -X POST http://127.0.0.1:8000/graphql \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"query\":\"{{ hello }}\"}}' ");

    Server::run_with_config(app, config).await?;

    Ok(())
}
