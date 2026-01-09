//! Dependency injection container
//!
//! This module provides the main `DependencyContainer` which manages dependency
//! registration, resolution, and caching.

use super::dependency::Dependency;
use super::error::DependencyError;
use super::graph::DependencyGraph;
use super::resolved::ResolvedDependencies;
use crate::request_data::RequestData;
use http::Request;
use indexmap::IndexMap;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main dependency injection container
///
/// The container manages:
/// - Registration of dependencies with cycle detection
/// - Batched parallel resolution using topological sorting
/// - Singleton caching (global across all requests)
/// - Request-scoped caching (within a single request)
///
/// # Thread Safety
///
/// The container is thread-safe and can be shared across multiple threads
/// using `Arc<DependencyContainer>`.
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::{DependencyContainer, ValueDependency};
/// use std::sync::Arc;
///
/// # tokio_test::block_on(async {
/// let mut container = DependencyContainer::new();
///
/// // Register a simple value dependency
/// let config = ValueDependency::new("port", 8080u16);
/// container.register("port".to_string(), Arc::new(config)).unwrap();
///
/// // Resolve dependencies for a handler
/// use http::Request;
/// use crate::request_data::RequestData;
/// use std::collections::HashMap;
///
/// let request = Request::builder().body(()).unwrap();
/// let request_data = RequestData {
///     path_params: Arc::new(HashMap::new()),
///     query_params: serde_json::Value::Null,
///     validated_params: None,
///     raw_query_params: Arc::new(HashMap::new()),
///     body: serde_json::Value::Null,
///     raw_body: None,
///     headers: Arc::new(HashMap::new()),
///     cookies: Arc::new(HashMap::new()),
///     method: "GET".to_string(),
///     path: "/".to_string(),
/// };
///
/// let resolved = container
///     .resolve_for_handler(&["port".to_string()], &request, &request_data)
///     .await
///     .unwrap();
///
/// let port: Option<Arc<u16>> = resolved.get("port");
/// assert_eq!(port.map(|p| *p), Some(8080));
/// # });
/// ```
pub struct DependencyContainer {
    /// Registered dependencies by key (preserves insertion order)
    dependencies: IndexMap<String, Arc<dyn Dependency>>,
    /// Dependency graph for topological sorting and cycle detection
    dependency_graph: DependencyGraph,
    /// Global singleton cache
    singleton_cache: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}

impl DependencyContainer {
    /// Create a new empty dependency container
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyContainer;
    ///
    /// let container = DependencyContainer::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            dependencies: IndexMap::new(),
            dependency_graph: DependencyGraph::new(),
            singleton_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a dependency in the container
    ///
    /// This will validate that:
    /// - The key is not already registered
    /// - Adding this dependency won't create a circular dependency
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    /// * `dep` - The dependency implementation
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` for method chaining.
    ///
    /// # Errors
    ///
    /// - `DependencyError::DuplicateKey` if a dependency with this key exists
    /// - `DependencyError::CircularDependency` if this would create a cycle
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::{DependencyContainer, ValueDependency};
    /// use std::sync::Arc;
    ///
    /// let mut container = DependencyContainer::new();
    ///
    /// let config = ValueDependency::new("config", "production".to_string());
    /// container.register("config".to_string(), Arc::new(config)).unwrap();
    /// ```
    pub fn register(&mut self, key: String, dep: Arc<dyn Dependency>) -> Result<&mut Self, DependencyError> {
        self.dependency_graph.add_dependency(&key, dep.depends_on())?;

        self.dependencies.insert(key, dep);

        Ok(self)
    }

    /// Resolve dependencies for a handler
    ///
    /// This method:
    /// 1. Calculates the optimal batched resolution order using topological sorting
    /// 2. Resolves dependencies in batches (dependencies in the same batch run in parallel)
    /// 3. Caches singleton dependencies globally
    /// 4. Caches per-request dependencies within the returned `ResolvedDependencies`
    ///
    /// # Arguments
    ///
    /// * `deps` - The dependency keys needed by the handler
    /// * `req` - The HTTP request being handled
    /// * `data` - Extracted request data
    ///
    /// # Returns
    ///
    /// A `ResolvedDependencies` instance containing all resolved dependencies.
    ///
    /// # Errors
    ///
    /// - `DependencyError::NotFound` if a required dependency is not registered
    /// - `DependencyError::CircularDependency` if there's a cycle in dependencies
    /// - `DependencyError::ResolutionFailed` if a dependency fails to resolve
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::{DependencyContainer, ValueDependency, FactoryDependency};
    /// use std::sync::Arc;
    ///
    /// # tokio_test::block_on(async {
    /// let mut container = DependencyContainer::new();
    ///
    /// // Register dependencies
    /// let config = ValueDependency::new("config", "production".to_string());
    /// container.register("config".to_string(), Arc::new(config)).unwrap();
    ///
    /// let db = FactoryDependency::builder("database")
    ///     .depends_on(vec!["config".to_string()])
    ///     .factory(|_req, _data, resolved| {
    ///         Box::pin(async move {
    ///             let config: Arc<String> = resolved.get("config").unwrap();
    ///             let db = format!("DB connected to {}", *config);
    ///             Ok(Arc::new(db) as Arc<dyn std::any::Any + Send + Sync>)
    ///         })
    ///     })
    ///     .build();
    /// container.register("database".to_string(), Arc::new(db)).unwrap();
    ///
    /// // Resolve for handler
    /// use http::Request;
    /// use crate::request_data::RequestData;
    /// use std::collections::HashMap;
    ///
    /// let request = Request::builder().body(()).unwrap();
    /// let request_data = RequestData {
    ///     path_params: Arc::new(HashMap::new()),
    ///     query_params: serde_json::Value::Null,
    ///     validated_params: None,
    ///     raw_query_params: Arc::new(HashMap::new()),
    ///     body: serde_json::Value::Null,
    ///     raw_body: None,
    ///     headers: Arc::new(HashMap::new()),
    ///     cookies: Arc::new(HashMap::new()),
    ///     method: "GET".to_string(),
    ///     path: "/".to_string(),
    /// };
    ///
    /// let resolved = container
    ///     .resolve_for_handler(&["database".to_string()], &request, &request_data)
    ///     .await
    ///     .unwrap();
    ///
    /// let db: Option<Arc<String>> = resolved.get("database");
    /// assert!(db.is_some());
    /// # });
    /// ```
    pub async fn resolve_for_handler(
        &self,
        deps: &[String],
        req: &Request<()>,
        data: &RequestData,
    ) -> Result<ResolvedDependencies, DependencyError> {
        for key in deps {
            if !self.dependencies.contains_key(key) {
                return Err(DependencyError::NotFound { key: key.clone() });
            }
        }

        let batches = self.dependency_graph.calculate_batches(deps)?;

        let mut resolved = ResolvedDependencies::new();
        let mut request_cache: HashMap<String, Arc<dyn Any + Send + Sync>> = HashMap::new();

        for batch in batches {
            // NOTE: We resolve sequentially within each batch to ensure cleanup tasks
            let mut sorted_keys: Vec<_> = batch.iter().collect();

            sorted_keys.sort_by_key(|key| self.dependencies.get_index_of(*key).unwrap_or(usize::MAX));

            for key in sorted_keys {
                let dep = self
                    .dependencies
                    .get(key)
                    .ok_or_else(|| DependencyError::NotFound { key: key.clone() })?;

                if dep.singleton() {
                    let cache = self.singleton_cache.read().await;
                    if let Some(cached) = cache.get(key) {
                        resolved.insert(key.clone(), Arc::clone(cached));
                        continue;
                    }
                }

                if dep.cacheable()
                    && let Some(cached) = request_cache.get(key)
                {
                    resolved.insert(key.clone(), Arc::clone(cached));
                    continue;
                }

                let result = dep.resolve(req, data, &resolved).await?;

                if dep.singleton() {
                    let mut cache = self.singleton_cache.write().await;
                    cache.insert(key.clone(), Arc::clone(&result));
                } else if dep.cacheable() {
                    request_cache.insert(key.clone(), Arc::clone(&result));
                }

                resolved.insert(key.clone(), result);
            }
        }

        Ok(resolved)
    }

    /// Get the number of registered dependencies
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::{DependencyContainer, ValueDependency};
    /// use std::sync::Arc;
    ///
    /// let mut container = DependencyContainer::new();
    /// assert_eq!(container.len(), 0);
    ///
    /// let dep = ValueDependency::new("test", 42);
    /// container.register("test".to_string(), Arc::new(dep)).unwrap();
    /// assert_eq!(container.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Check if the container is empty
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyContainer;
    ///
    /// let container = DependencyContainer::new();
    /// assert!(container.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    /// Check if a dependency is registered
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::{DependencyContainer, ValueDependency};
    /// use std::sync::Arc;
    ///
    /// let mut container = DependencyContainer::new();
    /// assert!(!container.contains("config"));
    ///
    /// let dep = ValueDependency::new("config", "value");
    /// container.register("config".to_string(), Arc::new(dep)).unwrap();
    /// assert!(container.contains("config"));
    /// ```
    #[must_use]
    pub fn contains(&self, key: &str) -> bool {
        self.dependencies.contains_key(key)
    }

    /// Get the keys of all registered dependencies
    #[must_use]
    pub fn keys(&self) -> Vec<String> {
        self.dependencies.keys().cloned().collect()
    }

    /// Clear the singleton cache
    ///
    /// This is useful for testing or when you need to force re-resolution
    /// of singleton dependencies.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyContainer;
    ///
    /// # tokio_test::block_on(async {
    /// let container = DependencyContainer::new();
    /// container.clear_singleton_cache().await;
    /// # });
    /// ```
    pub async fn clear_singleton_cache(&self) {
        let mut cache = self.singleton_cache.write().await;
        cache.clear();
    }
}

impl Default for DependencyContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for DependencyContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DependencyContainer")
            .field("dependencies", &self.dependencies.keys())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::di::{FactoryDependency, ValueDependency};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn make_request() -> Request<()> {
        Request::builder().body(()).unwrap()
    }

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
    fn test_new() {
        let container = DependencyContainer::new();
        assert!(container.is_empty());
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn test_register_simple() {
        let mut container = DependencyContainer::new();
        let dep = ValueDependency::new("test", 42i32);

        assert!(container.register("test".to_string(), Arc::new(dep)).is_ok());
        assert_eq!(container.len(), 1);
        assert!(container.contains("test"));
    }

    #[test]
    fn test_register_duplicate() {
        let mut container = DependencyContainer::new();
        let dep1 = ValueDependency::new("test", 42i32);
        let dep2 = ValueDependency::new("test", 100i32);

        container.register("test".to_string(), Arc::new(dep1)).unwrap();
        let result = container.register("test".to_string(), Arc::new(dep2));

        assert!(matches!(result, Err(DependencyError::DuplicateKey { .. })));
    }

    #[tokio::test]
    async fn test_register_circular() {
        let mut container = DependencyContainer::new();

        let dep_a = FactoryDependency::builder("a")
            .depends_on(vec!["b".to_string()])
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(1i32) as Arc<dyn Any + Send + Sync>) }))
            .build();

        let dep_b = FactoryDependency::builder("b")
            .depends_on(vec!["a".to_string()])
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(2i32) as Arc<dyn Any + Send + Sync>) }))
            .build();

        container.register("a".to_string(), Arc::new(dep_a)).unwrap();
        container.register("b".to_string(), Arc::new(dep_b)).unwrap();

        let request = make_request();
        let request_data = make_request_data();
        let result = container
            .resolve_for_handler(&["a".to_string()], &request, &request_data)
            .await;

        assert!(matches!(result, Err(DependencyError::CircularDependency { .. })));
    }

    #[tokio::test]
    async fn test_resolve_value() {
        let mut container = DependencyContainer::new();
        let dep = ValueDependency::new("answer", 42i32);
        container.register("answer".to_string(), Arc::new(dep)).unwrap();

        let request = make_request();
        let request_data = make_request_data();

        let resolved = container
            .resolve_for_handler(&["answer".to_string()], &request, &request_data)
            .await
            .unwrap();

        let value: Option<Arc<i32>> = resolved.get("answer");
        assert_eq!(value.map(|v| *v), Some(42));
    }

    #[tokio::test]
    async fn test_resolve_factory() {
        let mut container = DependencyContainer::new();

        let factory = FactoryDependency::builder("computed")
            .factory(|_req, _data, _resolved| Box::pin(async { Ok(Arc::new(100i32) as Arc<dyn Any + Send + Sync>) }))
            .build();

        container.register("computed".to_string(), Arc::new(factory)).unwrap();

        let request = make_request();
        let request_data = make_request_data();

        let resolved = container
            .resolve_for_handler(&["computed".to_string()], &request, &request_data)
            .await
            .unwrap();

        let value: Option<Arc<i32>> = resolved.get("computed");
        assert_eq!(value.map(|v| *v), Some(100));
    }

    #[tokio::test]
    async fn test_resolve_nested() {
        let mut container = DependencyContainer::new();

        let config = ValueDependency::new("config", "production".to_string());
        container.register("config".to_string(), Arc::new(config)).unwrap();

        let database = FactoryDependency::builder("database")
            .depends_on(vec!["config".to_string()])
            .factory(|_req, _data, resolved| {
                let resolved = resolved.clone();
                Box::pin(async move {
                    let config: Arc<String> = resolved.get("config").unwrap();
                    let db = format!("DB:{}", *config);
                    Ok(Arc::new(db) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();
        container.register("database".to_string(), Arc::new(database)).unwrap();

        let request = make_request();
        let request_data = make_request_data();

        let resolved = container
            .resolve_for_handler(&["database".to_string()], &request, &request_data)
            .await
            .unwrap();

        let db: Option<Arc<String>> = resolved.get("database");
        assert_eq!(db.as_ref().map(|v| v.as_str()), Some("DB:production"));
    }

    #[tokio::test]
    async fn test_resolve_batched() {
        let mut container = DependencyContainer::new();

        let counter = Arc::new(AtomicU32::new(0));

        let counter1 = Arc::clone(&counter);
        let config = FactoryDependency::builder("config")
            .factory(move |_req, _data, _resolved| {
                let c = Arc::clone(&counter1);
                Box::pin(async move {
                    let order = c.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(order) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();
        container.register("config".to_string(), Arc::new(config)).unwrap();

        let counter2 = Arc::clone(&counter);
        let database = FactoryDependency::builder("database")
            .depends_on(vec!["config".to_string()])
            .factory(move |_req, _data, _resolved| {
                let c = Arc::clone(&counter2);
                Box::pin(async move {
                    let order = c.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(order) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();
        container.register("database".to_string(), Arc::new(database)).unwrap();

        let counter3 = Arc::clone(&counter);
        let cache = FactoryDependency::builder("cache")
            .depends_on(vec!["config".to_string()])
            .factory(move |_req, _data, _resolved| {
                let c = Arc::clone(&counter3);
                Box::pin(async move {
                    let order = c.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(order) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();
        container.register("cache".to_string(), Arc::new(cache)).unwrap();

        let request = make_request();
        let request_data = make_request_data();

        let resolved = container
            .resolve_for_handler(&["database".to_string(), "cache".to_string()], &request, &request_data)
            .await
            .unwrap();

        let config_order: Arc<u32> = resolved.get("config").unwrap();
        assert_eq!(*config_order, 0);

        let db_order: Arc<u32> = resolved.get("database").unwrap();
        let cache_order: Arc<u32> = resolved.get("cache").unwrap();
        assert!(*db_order >= 1);
        assert!(*cache_order >= 1);
    }

    #[tokio::test]
    async fn test_singleton_cache() {
        let mut container = DependencyContainer::new();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let singleton = FactoryDependency::builder("singleton")
            .singleton(true)
            .factory(move |_req, _data, _resolved| {
                let c = Arc::clone(&counter_clone);
                Box::pin(async move {
                    let value = c.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();

        container
            .register("singleton".to_string(), Arc::new(singleton))
            .unwrap();

        let request = make_request();
        let request_data = make_request_data();

        for _ in 0..3 {
            let resolved = container
                .resolve_for_handler(&["singleton".to_string()], &request, &request_data)
                .await
                .unwrap();

            let value: Arc<u32> = resolved.get("singleton").unwrap();
            assert_eq!(*value, 0);
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_clear_singleton_cache() {
        let mut container = DependencyContainer::new();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&counter);

        let singleton = FactoryDependency::builder("singleton")
            .singleton(true)
            .factory(move |_req, _data, _resolved| {
                let c = Arc::clone(&counter_clone);
                Box::pin(async move {
                    let value = c.fetch_add(1, Ordering::SeqCst);
                    Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
                })
            })
            .build();

        container
            .register("singleton".to_string(), Arc::new(singleton))
            .unwrap();

        let request = make_request();
        let request_data = make_request_data();

        let resolved1 = container
            .resolve_for_handler(&["singleton".to_string()], &request, &request_data)
            .await
            .unwrap();
        let value1: Arc<u32> = resolved1.get("singleton").unwrap();
        assert_eq!(*value1, 0);

        container.clear_singleton_cache().await;

        let resolved2 = container
            .resolve_for_handler(&["singleton".to_string()], &request, &request_data)
            .await
            .unwrap();
        let value2: Arc<u32> = resolved2.get("singleton").unwrap();
        assert_eq!(*value2, 1);
    }

    #[tokio::test]
    async fn test_resolve_not_found() {
        let container = DependencyContainer::new();
        let request = make_request();
        let request_data = make_request_data();

        let result = container
            .resolve_for_handler(&["missing".to_string()], &request, &request_data)
            .await;

        assert!(matches!(result, Err(DependencyError::NotFound { .. })));
    }

    #[test]
    fn test_contains() {
        let mut container = DependencyContainer::new();
        assert!(!container.contains("test"));

        let dep = ValueDependency::new("test", 42i32);
        container.register("test".to_string(), Arc::new(dep)).unwrap();

        assert!(container.contains("test"));
        assert!(!container.contains("other"));
    }

    #[test]
    fn test_debug() {
        let mut container = DependencyContainer::new();
        let dep = ValueDependency::new("test", 42i32);
        container.register("test".to_string(), Arc::new(dep)).unwrap();

        let debug_str = format!("{container:?}");
        assert!(debug_str.contains("DependencyContainer"));
    }
}
