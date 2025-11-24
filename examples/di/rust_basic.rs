//! Basic Dependency Injection Example
//!
//! This example demonstrates how to use Spikard's dependency injection system
//! to register and inject dependencies into handlers.
//!
//! Run with: cargo run --example rust_basic --features di

use axum::body::Body;
use axum::http::Request;
use spikard_http::{Handler, HandlerResult, RequestData, ServerConfig};
use std::sync::Arc;

/// Example configuration struct
#[derive(Debug, Clone)]
struct AppConfig {
    app_name: String,
    version: String,
    max_connections: usize,
}

/// Example database pool (simulated)
#[derive(Debug, Clone)]
struct DatabasePool {
    connection_string: String,
    max_connections: usize,
}

impl DatabasePool {
    async fn connect(url: &str, max_connections: usize) -> Result<Self, String> {
        println!("Connecting to database: {}", url);
        Ok(Self {
            connection_string: url.to_string(),
            max_connections,
        })
    }

    async fn query(&self, sql: &str) -> Result<Vec<String>, String> {
        println!("Executing query: {}", sql);
        Ok(vec!["user1".to_string(), "user2".to_string()])
    }
}

/// Example handler that uses injected dependencies
struct ExampleHandler;

impl Handler for ExampleHandler {
    fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            #[cfg(feature = "di")]
            {
                // Access injected dependencies
                if let Some(ref dependencies) = request_data.dependencies {
                    // Get app_name
                    let app_name: Option<Arc<String>> = dependencies.get("app_name");
                    if let Some(name) = app_name {
                        println!("App name from DI: {}", name);
                    }

                    // Get database pool
                    let db: Option<Arc<DatabasePool>> = dependencies.get("db");
                    if let Some(pool) = db {
                        let users = pool
                            .query("SELECT * FROM users")
                            .await
                            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

                        let response_body = format!("Users: {:?}", users);
                        return Ok(axum::http::Response::builder()
                            .status(axum::http::StatusCode::OK)
                            .body(Body::from(response_body))
                            .unwrap());
                    }
                }
            }

            Ok(axum::http::Response::builder()
                .status(axum::http::StatusCode::OK)
                .body(Body::from("No dependencies injected"))
                .unwrap())
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Spikard Dependency Injection Example ===\n");

    #[cfg(not(feature = "di"))]
    {
        eprintln!("Error: This example requires the 'di' feature to be enabled.");
        eprintln!("Run with: cargo run --example rust_basic --features di");
        std::process::exit(1);
    }

    #[cfg(feature = "di")]
    {
        // Example 1: Simple value dependencies
        println!("1. Simple Value Dependencies");
        println!("------------------------------");

        let config1 = ServerConfig::builder()
            .port(3000)
            .provide_value("app_name", "MyApp".to_string())
            .provide_value("version", "1.0.0".to_string())
            .provide_value("max_connections", 100)
            .build();

        println!("✓ Registered value dependencies: app_name, version, max_connections");
        println!("  Container exists: {}\n", config1.di_container.is_some());

        // Example 2: Factory dependencies
        println!("2. Factory Dependencies");
        println!("------------------------");

        let config2 = ServerConfig::builder()
            .port(3001)
            .provide_value("db_url", "postgresql://localhost/mydb".to_string())
            .provide_value("max_connections", 100)
            .provide_factory("db", |resolved| async move {
                // Access other dependencies
                let db_url = resolved
                    .get::<String>("db_url")
                    .ok_or("db_url not found")?;

                let max_conn = resolved
                    .get::<usize>("max_connections")
                    .ok_or("max_connections not found")?;

                println!("  Factory: Creating database pool...");
                println!("  - URL: {}", db_url);
                println!("  - Max connections: {}", max_conn);

                // Simulate async database connection
                let pool = DatabasePool::connect(&db_url, *max_conn)
                    .await
                    .map_err(|e| format!("Failed to connect to database: {}", e))?;

                Ok(pool)
            })
            .build();

        println!("✓ Registered factory dependency: db");
        println!("  Container exists: {}\n", config2.di_container.is_some());

        // Example 3: Multiple dependencies with relationships
        println!("3. Multiple Dependencies");
        println!("-------------------------");

        let config3 = ServerConfig::builder()
            .port(3002)
            // Base configuration
            .provide_value("app_name", "ProductionApp".to_string())
            .provide_value("environment", "production".to_string())
            // Database configuration
            .provide_value("db_url", "postgresql://prod-db/myapp".to_string())
            .provide_value("db_max_connections", 200)
            // Database pool factory
            .provide_factory("db_pool", |resolved| async move {
                let url = resolved.get::<String>("db_url").ok_or("Missing db_url")?;
                let max_conn = resolved
                    .get::<usize>("db_max_connections")
                    .ok_or("Missing db_max_connections")?;

                DatabasePool::connect(&url, *max_conn).await
            })
            .build();

        println!("✓ Registered multiple dependencies with relationships");
        println!("  Container exists: {}\n", config3.di_container.is_some());

        // Example 4: Advanced - custom dependency
        println!("4. Advanced: Custom Dependency");
        println!("--------------------------------");

        use spikard_core::di::ValueDependency;

        let custom_dep = ValueDependency::new("custom_config", AppConfig {
            app_name: "CustomApp".to_string(),
            version: "2.0.0".to_string(),
            max_connections: 500,
        });

        let config4 = ServerConfig::builder()
            .port(3003)
            .provide(Arc::new(custom_dep))
            .build();

        println!("✓ Registered custom dependency using provide()");
        println!("  Container exists: {}\n", config4.di_container.is_some());

        // Summary
        println!("=== Summary ===");
        println!("✓ All examples completed successfully");
        println!("\nKey Concepts Demonstrated:");
        println!("  • Value dependencies for static configuration");
        println!("  • Factory dependencies for dynamic resource creation");
        println!("  • Dependency relationships (factories depending on values)");
        println!("  • Custom dependencies using the advanced API");
        println!("\nNext Steps:");
        println!("  • See handler integration examples for actual HTTP usage");
        println!("  • Explore async cleanup with generator patterns");
        println!("  • Learn about singleton and per-request caching strategies");
    }

    Ok(())
}
