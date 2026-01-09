//! Factory dependency implementation
//!
//! This module provides `FactoryDependency`, a dependency that uses a factory
//! function to create values dynamically based on request context.

use super::dependency::Dependency;
use super::error::DependencyError;
use super::resolved::ResolvedDependencies;
use crate::request_data::RequestData;
use http::Request;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Factory function type for creating dependencies
///
/// The factory receives:
/// - The HTTP request
/// - Extracted request data
/// - Already-resolved dependencies
///
/// And returns a future that resolves to the dependency value or an error.
pub type FactoryFn = dyn Fn(
        &Request<()>,
        &RequestData,
        &ResolvedDependencies,
    ) -> BoxFuture<'static, Result<Arc<dyn Any + Send + Sync>, DependencyError>>
    + Send
    + Sync;

/// A dependency that uses a factory function to create values
///
/// Factory dependencies are more flexible than value dependencies - they can:
/// - Access request data (headers, query params, etc.)
/// - Depend on other dependencies
/// - Perform async operations (database queries, HTTP requests)
/// - Return different values based on context
///
/// # Caching Strategies
///
/// - **Singleton**: Factory runs once globally, result cached forever
/// - **Cacheable**: Factory runs once per request, result cached for that request
/// - **Non-cacheable**: Factory runs every time the dependency is requested
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::{FactoryDependency, Dependency, ResolvedDependencies};
/// use http::Request;
/// use crate::request_data::RequestData;
/// use std::sync::Arc;
///
/// # tokio_test::block_on(async {
/// // Simple factory that returns a constant
/// let factory = FactoryDependency::builder("counter")
///     .factory(|_req, _data, _resolved| {
///         Box::pin(async {
///             Ok(Arc::new(42i32) as Arc<dyn std::any::Any + Send + Sync>)
///         })
///     })
///     .build();
///
/// assert_eq!(factory.key(), "counter");
/// # });
/// ```
pub struct FactoryDependency {
    key: String,
    factory: Arc<FactoryFn>,
    dependencies: Vec<String>,
    cacheable: bool,
    singleton: bool,
}

impl FactoryDependency {
    /// Create a new builder for constructing a factory dependency
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("my_dep")
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             Ok(Arc::new(100i32) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .build();
    /// ```
    pub fn builder(key: impl Into<String>) -> FactoryDependencyBuilder {
        FactoryDependencyBuilder::new(key)
    }
}

impl Dependency for FactoryDependency {
    fn resolve(
        &self,
        request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>> {
        (self.factory)(request, request_data, resolved)
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn singleton(&self) -> bool {
        self.singleton
    }
}

impl std::fmt::Debug for FactoryDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FactoryDependency")
            .field("key", &self.key)
            .field("dependencies", &self.dependencies)
            .field("cacheable", &self.cacheable)
            .field("singleton", &self.singleton)
            .finish_non_exhaustive()
    }
}

/// Builder for constructing factory dependencies
///
/// Provides a fluent API for configuring factory dependencies with optional
/// settings like dependencies, caching, and singleton behavior.
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::FactoryDependency;
/// use std::sync::Arc;
///
/// // Factory with dependencies
/// let factory = FactoryDependency::builder("service")
///     .depends_on(vec!["database".to_string(), "cache".to_string()])
///     .factory(|_req, _data, resolved| {
///         Box::pin(async move {
///             // Access other dependencies
///             let _db: Option<Arc<String>> = resolved.get("database");
///             let _cache: Option<Arc<String>> = resolved.get("cache");
///
///             Ok(Arc::new("service".to_string()) as Arc<dyn std::any::Any + Send + Sync>)
///         })
///     })
///     .cacheable(true)
///     .build();
/// ```
pub struct FactoryDependencyBuilder {
    key: String,
    factory: Option<Arc<FactoryFn>>,
    dependencies: Vec<String>,
    cacheable: bool,
    singleton: bool,
}

impl FactoryDependencyBuilder {
    /// Create a new builder
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            factory: None,
            dependencies: Vec::new(),
            cacheable: false,
            singleton: false,
        }
    }

    /// Set the factory function
    ///
    /// # Arguments
    ///
    /// * `factory` - A function that creates the dependency value
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("timestamp")
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             let now = std::time::SystemTime::now();
    ///             Ok(Arc::new(now) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .build();
    /// ```
    #[must_use]
    pub fn factory<F>(mut self, factory: F) -> Self
    where
        F: Fn(
                &Request<()>,
                &RequestData,
                &ResolvedDependencies,
            ) -> BoxFuture<'static, Result<Arc<dyn Any + Send + Sync>, DependencyError>>
            + Send
            + Sync
            + 'static,
    {
        self.factory = Some(Arc::new(factory));
        self
    }

    /// Set the dependencies that this factory requires
    ///
    /// # Arguments
    ///
    /// * `dependencies` - List of dependency keys that must be resolved first
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("service")
    ///     .depends_on(vec!["database".to_string()])
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             Ok(Arc::new("service") as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .build();
    /// ```
    #[must_use]
    pub fn depends_on(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// Set whether this dependency should be cached within a request
    ///
    /// # Arguments
    ///
    /// * `cacheable` - If true, resolves once per request
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("request_id")
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             let id = uuid::Uuid::new_v4().to_string();
    ///             Ok(Arc::new(id) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .cacheable(true)  // Same ID for all uses in one request
    ///     .build();
    /// ```
    #[must_use]
    pub const fn cacheable(mut self, cacheable: bool) -> Self {
        self.cacheable = cacheable;
        self
    }

    /// Set whether this dependency is a singleton (cached globally)
    ///
    /// # Arguments
    ///
    /// * `singleton` - If true, resolves once globally across all requests
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("database_pool")
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             // Expensive initialization
    ///             let pool = "DatabasePool::new()".to_string();
    ///             Ok(Arc::new(pool) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .singleton(true)  // Share across all requests
    ///     .build();
    /// ```
    #[must_use]
    pub const fn singleton(mut self, singleton: bool) -> Self {
        self.singleton = singleton;
        self
    }

    /// Build the factory dependency
    ///
    /// # Panics
    ///
    /// Panics if the factory function was not set.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::FactoryDependency;
    /// use std::sync::Arc;
    ///
    /// let factory = FactoryDependency::builder("my_dep")
    ///     .factory(|_req, _data, _resolved| {
    ///         Box::pin(async {
    ///             Ok(Arc::new(42i32) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> FactoryDependency {
        FactoryDependency {
            key: self.key.clone(),
            factory: self
                .factory
                .unwrap_or_else(|| panic!("Factory function must be set for dependency '{}'", self.key)),
            dependencies: self.dependencies,
            cacheable: self.cacheable,
            singleton: self.singleton,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn make_request_data() -> RequestData {
        RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: serde_json::Value::Null,
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: serde_json::Value::Null,
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    #[test]
    fn test_builder_key() {
        let factory = FactoryDependency::builder("test")
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(42i32) as Arc<dyn Any + Send + Sync>) }))
            .build();

        assert_eq!(factory.key(), "test");
    }

    #[test]
    fn test_builder_depends_on() {
        let factory = FactoryDependency::builder("test")
            .depends_on(vec!["dep1".to_string(), "dep2".to_string()])
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(42i32) as Arc<dyn Any + Send + Sync>) }))
            .build();

        assert_eq!(factory.depends_on(), vec!["dep1", "dep2"]);
    }

    #[test]
    fn test_builder_cacheable() {
        let factory = FactoryDependency::builder("test")
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(42i32) as Arc<dyn Any + Send + Sync>) }))
            .cacheable(true)
            .build();

        assert!(factory.cacheable());
    }

    #[test]
    fn test_builder_singleton() {
        let factory = FactoryDependency::builder("test")
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(42i32) as Arc<dyn Any + Send + Sync>) }))
            .singleton(true)
            .build();

        assert!(factory.singleton());
    }

    #[tokio::test]
    async fn test_factory_async() {
        let factory = FactoryDependency::builder("async_value")
            .factory(|_req, _data, _resolved| {
                Box::pin(async {
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    Ok(Arc::new(100i32) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();

        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();
        let resolved = ResolvedDependencies::new();

        let result = factory.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<i32> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, 100);
    }

    #[tokio::test]
    async fn test_factory_depends_on() {
        let mut resolved = ResolvedDependencies::new();
        resolved.insert("config".to_string(), Arc::new("test_config".to_string()));

        let factory = FactoryDependency::builder("service")
            .depends_on(vec!["config".to_string()])
            .factory(|_req, _data, resolved| {
                let resolved = resolved.clone();
                Box::pin(async move {
                    let config: Option<Arc<String>> = resolved.get("config");
                    let config_value = config.map(|c| (*c).clone()).unwrap_or_default();

                    Ok(Arc::new(format!("Service using {config_value}")) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();

        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();

        let result = factory.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<String> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, "Service using test_config");
    }

    #[tokio::test]
    async fn test_factory_request_data() {
        let factory = FactoryDependency::builder("user_agent")
            .factory(|_req, request_data, _resolved| {
                let ua = request_data
                    .headers
                    .get("user-agent")
                    .cloned()
                    .unwrap_or_else(|| "unknown".to_string());

                Box::pin(async move { Ok(Arc::new(ua) as Arc<dyn Any + Send + Sync>) })
            })
            .build();

        let mut headers = HashMap::new();
        headers.insert("user-agent".to_string(), "test-agent/1.0".to_string());

        let request_data = RequestData {
            headers: Arc::new(headers),
            ..make_request_data()
        };

        let request = Request::builder().body(()).unwrap();
        let resolved = ResolvedDependencies::new();

        let result = factory.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<String> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, "test-agent/1.0");
    }

    #[tokio::test]
    async fn test_factory_call_count() {
        let call_count = Arc::new(AtomicU32::new(0));

        let call_count_clone = Arc::clone(&call_count);
        let factory = FactoryDependency::builder("counter")
            .factory(move |_req, _data, _resolved| {
                let count = Arc::clone(&call_count_clone);
                Box::pin(async move {
                    let current = count.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(current) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();

        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();
        let resolved = ResolvedDependencies::new();

        for i in 0..3 {
            let result = factory.resolve(&request, &request_data, &resolved).await;
            let value: Arc<u32> = result.unwrap().downcast().unwrap();
            assert_eq!(*value, i);
        }

        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_debug() {
        let factory = FactoryDependency::builder("test")
            .depends_on(vec!["dep1".to_string()])
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(42i32) as Arc<dyn Any + Send + Sync>) }))
            .cacheable(true)
            .singleton(false)
            .build();

        let debug_str = format!("{factory:?}");
        assert!(debug_str.contains("FactoryDependency"));
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("dep1"));
    }

    #[test]
    #[should_panic(expected = "Factory function must be set")]
    fn test_builder_without_factory() {
        let _factory = FactoryDependency::builder("test").build();
    }
}
