//! Dependency Injection system for Spikard
//!
//! This module provides a comprehensive dependency injection system with:
//!
//! - **Type-safe dependency resolution**: Dependencies are stored as `Arc<dyn Any>` but
//!   can be retrieved with type safety using `ResolvedDependencies::get<T>()`
//! - **Async resolution**: All dependencies can perform async operations during resolution
//! - **Batched parallel resolution**: Dependencies with no interdependencies are resolved
//!   in parallel using topological sorting
//! - **Multiple caching strategies**:
//!   - Singleton: Resolved once globally, cached forever
//!   - Per-request cacheable: Resolved once per request
//!   - Non-cacheable: Resolved every time
//! - **Cycle detection**: Circular dependencies are detected at registration time
//! - **Cleanup support**: Generator-pattern dependencies can register cleanup tasks
//!
//! # Architecture
//!
//! The DI system is built on several core components:
//!
//! - [`Dependency`] trait: The core abstraction that all dependencies implement
//! - [`DependencyContainer`]: Manages registration and resolution
//! - [`ResolvedDependencies`]: Stores resolved dependencies with type-safe access
//! - [`DependencyGraph`]: Handles topological sorting and cycle detection
//! - [`ValueDependency<T>`]: Simple static value dependencies
//! - [`FactoryDependency`]: Dynamic factory-based dependencies
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```ignore
//! use spikard_core::di::{DependencyContainer, ValueDependency, FactoryDependency};
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let mut container = DependencyContainer::new();
//!
//! // Register a simple value dependency
//! let config = ValueDependency::new("database_url", "postgresql://localhost/mydb");
//! container.register("database_url".to_string(), Arc::new(config)).unwrap();
//!
//! // Register a factory dependency that depends on the config
//! let pool = FactoryDependency::builder("db_pool")
//!     .depends_on(vec!["database_url".to_string()])
//!     .factory(|_req, _data, resolved| {
//!         Box::pin(async move {
//!             let url: Arc<String> = resolved.get("database_url").unwrap();
//!             let pool = format!("Pool connected to {}", *url);
//!             Ok(Arc::new(pool) as Arc<dyn std::any::Any + Send + Sync>)
//!         })
//!     })
//!     .singleton(true)  // Share across all requests
//!     .build();
//! container.register("db_pool".to_string(), Arc::new(pool)).unwrap();
//!
//! // Resolve for a handler
//! use http::Request;
//! use crate::request_data::RequestData;
//! use std::collections::HashMap;
//!
//! let request = Request::builder().body(()).unwrap();
//! let request_data = RequestData {
//!     path_params: Arc::new(HashMap::new()),
//!     query_params: serde_json::Value::Null,
//!     validated_params: None,
//!     raw_query_params: Arc::new(HashMap::new()),
//!     body: serde_json::Value::Null,
//!     raw_body: None,
//!     headers: Arc::new(HashMap::new()),
//!     cookies: Arc::new(HashMap::new()),
//!     method: "GET".to_string(),
//!     path: "/".to_string(),
//! };
//!
//! let resolved = container
//!     .resolve_for_handler(&["db_pool".to_string()], &request, &request_data)
//!     .await
//!     .unwrap();
//!
//! let pool: Option<Arc<String>> = resolved.get("db_pool");
//! assert!(pool.is_some());
//! # });
//! ```
//!
//! ## Request-Scoped Dependencies
//!
//! ```ignore
//! use spikard_core::di::{DependencyContainer, FactoryDependency};
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let mut container = DependencyContainer::new();
//!
//! // Create a request-scoped dependency (e.g., request ID)
//! let request_id = FactoryDependency::builder("request_id")
//!     .factory(|_req, _data, _resolved| {
//!         Box::pin(async {
//!             let id = uuid::Uuid::new_v4().to_string();
//!             Ok(Arc::new(id) as Arc<dyn std::any::Any + Send + Sync>)
//!         })
//!     })
//!     .cacheable(true)  // Same ID throughout the request
//!     .build();
//!
//! container.register("request_id".to_string(), Arc::new(request_id)).unwrap();
//! # });
//! ```
//!
//! ## Accessing Request Data
//!
//! ```ignore
//! use spikard_core::di::{DependencyContainer, FactoryDependency};
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let mut container = DependencyContainer::new();
//!
//! // Access headers, query params, etc.
//! let user_agent = FactoryDependency::builder("user_agent")
//!     .factory(|_req, request_data, _resolved| {
//!         let ua = request_data.headers
//!             .get("user-agent")
//!             .cloned()
//!             .unwrap_or_else(|| "unknown".to_string());
//!
//!         Box::pin(async move {
//!             Ok(Arc::new(ua) as Arc<dyn std::any::Any + Send + Sync>)
//!         })
//!     })
//!     .build();
//!
//! container.register("user_agent".to_string(), Arc::new(user_agent)).unwrap();
//! # });
//! ```
//!
//! ## Cleanup Tasks
//!
//! ```ignore
//! use spikard_core::di::ResolvedDependencies;
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let mut resolved = ResolvedDependencies::new();
//!
//! // Add a dependency with cleanup
//! resolved.insert("connection".to_string(), Arc::new("DB Connection"));
//!
//! // Register cleanup task
//! resolved.add_cleanup_task(Box::new(|| {
//!     Box::pin(async {
//!         println!("Closing database connection");
//!     })
//! }));
//!
//! // Cleanup runs when resolved is dropped (or explicitly)
//! resolved.cleanup().await;
//! # });
//! ```
//!
//! # Performance
//!
//! The DI system is designed for high performance:
//!
//! - **Parallel resolution**: Independent dependencies are resolved concurrently
//! - **Efficient caching**: Singleton and per-request caching minimize redundant work
//! - **Arc-based sharing**: Values are reference-counted, not cloned
//! - **Zero-cost abstractions**: Type erasure has minimal overhead
//!
//! # Thread Safety
//!
//! All components are thread-safe:
//!
//! - `DependencyContainer` can be shared with `Arc<DependencyContainer>`
//! - Singleton cache uses `RwLock` for concurrent access
//! - All dependencies must be `Send + Sync`

mod container;
mod dependency;
mod error;
mod factory;
mod graph;
mod resolved;
mod value;

pub use container::DependencyContainer;
pub use dependency::Dependency;
pub use error::DependencyError;
pub use factory::{FactoryDependency, FactoryDependencyBuilder, FactoryFn};
pub use graph::DependencyGraph;
pub use resolved::ResolvedDependencies;
pub use value::ValueDependency;
