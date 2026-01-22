//! Dependency injection base traits
//!
//! This module provides language-agnostic DI abstractions that eliminate
//! duplicate value/factory patterns across Node, Ruby, and PHP bindings.

use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::Request;
use spikard_core::RequestData;
use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};

/// Type alias for the common dependency resolution future type
type DependencyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + 'a>>;

/// Adapter trait for value dependencies across language bindings
///
/// Language bindings should implement this trait to wrap their
/// language-specific value storage (e.g., `Py<PyAny>`, `Opaque<Value>`, etc.)
pub trait ValueDependencyAdapter: Send + Sync {
    /// Get the dependency key
    fn key(&self) -> &str;

    /// Resolve the stored value
    ///
    /// Returns an `Arc<dyn Any>` that can be downcast to the concrete type
    fn resolve_value(&self) -> DependencyFuture<'_>;
}

/// Adapter trait for factory dependencies across language bindings
///
/// Language bindings should implement this trait to wrap their
/// language-specific callable storage (e.g., `Py<PyAny>`, `ThreadsafeFunction`, etc.)
pub trait FactoryDependencyAdapter: Send + Sync {
    /// Get the dependency key
    fn key(&self) -> &str;

    /// Invoke the factory with resolved dependencies
    ///
    /// The factory receives already-resolved dependencies and returns
    /// a new instance wrapped in `Arc<dyn Any>`
    fn invoke_factory(
        &self,
        request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> DependencyFuture<'_>;
}

/// Bridge between language-specific adapters and core DI system
///
/// This struct implements the core `Dependency` trait while delegating
/// to the language-specific adapter implementation
pub struct ValueDependencyBridge<T: ValueDependencyAdapter> {
    adapter: Arc<T>,
}

impl<T: ValueDependencyAdapter + 'static> ValueDependencyBridge<T> {
    /// Create a new value dependency bridge
    pub fn new(adapter: T) -> Self {
        Self {
            adapter: Arc::new(adapter),
        }
    }
}

impl<T: ValueDependencyAdapter + 'static> Dependency for ValueDependencyBridge<T> {
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
        self.adapter.resolve_value()
    }

    fn key(&self) -> &str {
        self.adapter.key()
    }

    // PERFORMANCE: Value dependencies have no sub-dependencies.
    // Returning an empty Vec is unavoidable if the trait requires it, but
    // consider optimizing the DI system to use Option<&[String]> or a default method.
    fn depends_on(&self) -> Vec<String> {
        vec![]
    }
}

/// Bridge between language-specific factory adapters and core DI system
pub struct FactoryDependencyBridge<T: FactoryDependencyAdapter> {
    adapter: Arc<T>,
}

impl<T: FactoryDependencyAdapter + 'static> FactoryDependencyBridge<T> {
    /// Create a new factory dependency bridge
    pub fn new(adapter: T) -> Self {
        Self {
            adapter: Arc::new(adapter),
        }
    }
}

impl<T: FactoryDependencyAdapter + 'static> Dependency for FactoryDependencyBridge<T> {
    fn resolve(
        &self,
        request: &Request<()>,
        request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
        self.adapter.invoke_factory(request, request_data, resolved)
    }

    fn key(&self) -> &str {
        self.adapter.key()
    }

    // PERFORMANCE: Factory dependencies may or may not have sub-dependencies.
    // Language bindings should override this if they track dependencies.
    // The default empty Vec is acceptable for language bindings that don't track explicit dependency graphs.
    fn depends_on(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockValueAdapter {
        key: String,
        value: i32,
    }

    impl ValueDependencyAdapter for MockValueAdapter {
        fn key(&self) -> &str {
            &self.key
        }

        fn resolve_value(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
            let value = self.value;
            Box::pin(async move { Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>) })
        }
    }

    struct MockFactoryAdapter {
        key: String,
        call_count: Arc<AtomicUsize>,
    }

    impl FactoryDependencyAdapter for MockFactoryAdapter {
        fn key(&self) -> &str {
            &self.key
        }

        fn invoke_factory(
            &self,
            _request: &Request<()>,
            _request_data: &RequestData,
            _resolved: &ResolvedDependencies,
        ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
            let count = self.call_count.clone();
            count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                let value = count.load(Ordering::SeqCst);
                Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
            })
        }
    }

    #[tokio::test]
    async fn test_value_dependency_bridge() {
        let adapter = MockValueAdapter {
            key: "test_key".to_string(),
            value: 42,
        };
        let bridge = ValueDependencyBridge::new(adapter);

        assert_eq!(bridge.key(), "test_key");
        assert_eq!(bridge.depends_on(), Vec::<String>::new());

        let request = Request::builder().body(()).unwrap();
        let request_data = RequestData {
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
            dependencies: None,
        };
        let resolved = ResolvedDependencies::new();

        let result = bridge.resolve(&request, &request_data, &resolved).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        let downcast = value.downcast_ref::<i32>();
        assert_eq!(*downcast.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_factory_dependency_bridge() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let adapter = MockFactoryAdapter {
            key: "factory_key".to_string(),
            call_count: call_count.clone(),
        };
        let bridge = FactoryDependencyBridge::new(adapter);

        assert_eq!(bridge.key(), "factory_key");
        assert_eq!(bridge.depends_on(), Vec::<String>::new());

        let request = Request::builder().body(()).unwrap();
        let request_data = RequestData {
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
            dependencies: None,
        };
        let resolved = ResolvedDependencies::new();

        let result1 = bridge.resolve(&request, &request_data, &resolved).await;
        assert!(result1.is_ok());
        let value1 = result1.unwrap();
        let count1 = *value1.downcast_ref::<usize>().unwrap();
        assert_eq!(count1, 1);

        let result2 = bridge.resolve(&request, &request_data, &resolved).await;
        assert!(result2.is_ok());
        let value2 = result2.unwrap();
        let count2 = *value2.downcast_ref::<usize>().unwrap();
        assert_eq!(count2, 2);
    }
}
