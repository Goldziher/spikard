//! Value dependency implementation
//!
//! This module provides `ValueDependency<T>`, a simple dependency that wraps
//! a static value and returns it whenever resolved.

use super::dependency::Dependency;
use super::error::DependencyError;
use super::resolved::ResolvedDependencies;
use crate::request_data::RequestData;
use http::Request;
use std::any::Any;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

/// A dependency that wraps a static value
///
/// This is the simplest form of dependency - it just returns a pre-configured
/// value whenever resolved. Useful for configuration values, constants, or
/// pre-built objects.
///
/// # Type Parameters
///
/// * `T` - The type of value to provide. Must be `Clone + Send + Sync + 'static`.
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::{Dependency, ValueDependency};
/// use http::Request;
/// use crate::request_data::RequestData;
/// use std::collections::HashMap;
/// use std::sync::Arc;
///
/// # tokio_test::block_on(async {
/// // Create a value dependency with a configuration string
/// let config = ValueDependency::new("database_url", "postgresql://localhost/mydb");
///
/// // Resolve it (returns the same value every time)
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
/// let resolved = spikard_core::di::ResolvedDependencies::new();
///
/// let result = config.resolve(&request, &request_data, &resolved).await.unwrap();
/// let value: Arc<String> = result.downcast().unwrap();
/// assert_eq!(*value, "postgresql://localhost/mydb");
/// # });
/// ```
pub struct ValueDependency<T: Clone + Send + Sync + 'static> {
    key: String,
    value: Arc<T>,
    _phantom: PhantomData<T>,
}

impl<T: Clone + Send + Sync + 'static> ValueDependency<T> {
    /// Create a new value dependency
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    /// * `value` - The value to provide when resolved
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ValueDependency;
    ///
    /// // Simple value
    /// let port = ValueDependency::new("port", 8080u16);
    ///
    /// // Complex value
    /// #[derive(Clone)]
    /// struct Config {
    ///     debug: bool,
    ///     timeout: u64,
    /// }
    ///
    /// let config = ValueDependency::new("config", Config {
    ///     debug: true,
    ///     timeout: 30,
    /// });
    /// ```
    pub fn new(key: impl Into<String>, value: T) -> Self {
        Self {
            key: key.into(),
            value: Arc::new(value),
            _phantom: PhantomData,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Dependency for ValueDependency<T> {
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>> {
        let value = Arc::clone(&self.value);
        Box::pin(async move { Ok(value as Arc<dyn Any + Send + Sync>) })
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![]
    }

    fn cacheable(&self) -> bool {
        true
    }

    fn singleton(&self) -> bool {
        true
    }
}

impl<T: Clone + Send + Sync + 'static> std::fmt::Debug for ValueDependency<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueDependency")
            .field("key", &self.key)
            .field("value_type", &std::any::type_name::<T>())
            .field("value", &"<T>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        let dep = ValueDependency::new("test", 42i32);
        assert_eq!(dep.key(), "test");
    }

    #[test]
    fn test_key() {
        let dep = ValueDependency::new("my_key", "value");
        assert_eq!(dep.key(), "my_key");
    }

    #[test]
    fn test_depends_on() {
        let dep = ValueDependency::new("test", 42i32);
        assert_eq!(dep.depends_on(), Vec::<String>::new());
    }

    #[test]
    fn test_cacheable() {
        let dep = ValueDependency::new("test", 42i32);
        assert!(dep.cacheable());
    }

    #[test]
    fn test_singleton() {
        let dep = ValueDependency::new("test", 42i32);
        assert!(dep.singleton());
    }

    #[tokio::test]
    async fn test_resolve_simple() {
        let dep = ValueDependency::new("answer", 42i32);
        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();
        let resolved = ResolvedDependencies::new();

        let result = dep.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<i32> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, 42);
    }

    #[tokio::test]
    async fn test_resolve_string() {
        let dep = ValueDependency::new("message", "Hello, World!".to_string());
        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();
        let resolved = ResolvedDependencies::new();

        let result = dep.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<String> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, "Hello, World!");
    }

    #[tokio::test]
    async fn test_resolve_concurrent() {
        let dep = Arc::new(ValueDependency::new("shared", 100i32));
        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let dep = Arc::clone(&dep);
                let req = request.clone();
                let data = request_data.clone();
                tokio::spawn(async move {
                    let resolved = ResolvedDependencies::new();
                    let result = dep.resolve(&req, &data, &resolved).await.unwrap();
                    let value: Arc<i32> = result.downcast().unwrap();
                    *value
                })
            })
            .collect();

        for handle in handles {
            let value = handle.await.unwrap();
            assert_eq!(value, 100);
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct ComplexValue {
        name: String,
        count: i32,
        tags: Vec<String>,
    }

    #[tokio::test]
    async fn test_resolve_complex_type() {
        let complex = ComplexValue {
            name: "test".to_string(),
            count: 42,
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        let dep = ValueDependency::new("complex", complex.clone());
        let request = Request::builder().body(()).unwrap();
        let request_data = make_request_data();
        let resolved = ResolvedDependencies::new();

        let result = dep.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value: Arc<ComplexValue> = result.unwrap().downcast().unwrap();
        assert_eq!(*value, complex);
    }

    #[test]
    fn test_debug() {
        let dep = ValueDependency::new("test", 42i32);
        let debug_str = format!("{dep:?}");
        assert!(debug_str.contains("ValueDependency"));
        assert!(debug_str.contains("test"));
    }
}
