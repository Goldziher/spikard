//! Storage for resolved dependencies
//!
//! This module provides the `ResolvedDependencies` type which holds all dependencies
//! resolved for a particular request, with type-safe access and cleanup support.

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type CleanupTask = Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>;

/// Storage for resolved dependencies with type-safe access
///
/// This type stores all dependencies that have been resolved for a request,
/// allowing type-safe retrieval and supporting cleanup tasks for generator-pattern
/// dependencies.
///
/// Uses Arc<Mutex<>> internally for thread-safe shared access.
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::ResolvedDependencies;
/// use std::sync::Arc;
///
/// # tokio_test::block_on(async {
/// let mut resolved = ResolvedDependencies::new();
///
/// // Insert a dependency
/// let value = Arc::new(42i32);
/// resolved.insert("answer".to_string(), value);
///
/// // Retrieve with type safety
/// let retrieved: Option<Arc<i32>> = resolved.get("answer");
/// assert_eq!(retrieved.map(|v| *v), Some(42));
///
/// // Type mismatch returns None
/// let wrong_type: Option<Arc<String>> = resolved.get("answer");
/// assert!(wrong_type.is_none());
///
/// // Cleanup
/// resolved.cleanup().await;
/// # });
/// ```
#[derive(Default, Clone)]
pub struct ResolvedDependencies {
    /// Map of dependency keys to type-erased values
    dependencies: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    /// Cleanup tasks to run when dependencies are dropped
    cleanup_tasks: Arc<Mutex<Vec<CleanupTask>>>,
}

impl ResolvedDependencies {
    /// Create a new empty resolved dependencies storage
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    ///
    /// let resolved = ResolvedDependencies::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            dependencies: Arc::new(Mutex::new(HashMap::new())),
            cleanup_tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Insert a dependency into the storage
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    /// * `value` - The dependency value wrapped in Arc<dyn Any + Send + Sync>
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::Arc;
    ///
    /// let mut resolved = ResolvedDependencies::new();
    /// let config = Arc::new("production".to_string());
    /// resolved.insert("config".to_string(), config);
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    pub fn insert(&mut self, key: String, value: Arc<dyn Any + Send + Sync>) {
        self.dependencies.lock().unwrap().insert(key, value);
    }

    /// Get a dependency with type-safe downcasting
    ///
    /// Returns `Some(Arc<T>)` if the dependency exists and is of type `T`,
    /// or `None` if it doesn't exist or has a different type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected type of the dependency (must be Send + Sync)
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the dependency to retrieve
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::Arc;
    ///
    /// let mut resolved = ResolvedDependencies::new();
    /// resolved.insert("count".to_string(), Arc::new(100i32));
    ///
    /// // Correct type
    /// let count: Option<Arc<i32>> = resolved.get("count");
    /// assert_eq!(count.map(|v| *v), Some(100));
    ///
    /// // Wrong type
    /// let wrong: Option<Arc<String>> = resolved.get("count");
    /// assert!(wrong.is_none());
    ///
    /// // Missing key
    /// let missing: Option<Arc<i32>> = resolved.get("missing");
    /// assert!(missing.is_none());
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    #[must_use]
    pub fn get<T: Send + Sync + 'static>(&self, key: &str) -> Option<Arc<T>> {
        self.dependencies
            .lock()
            .unwrap()
            .get(key)
            .and_then(|value| Arc::clone(value).downcast::<T>().ok())
    }

    /// Get a dependency as Arc<dyn Any> without type checking
    ///
    /// This is useful when you need to pass dependencies around without
    /// knowing their concrete type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the dependency to retrieve
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::Arc;
    ///
    /// let mut resolved = ResolvedDependencies::new();
    /// resolved.insert("data".to_string(), Arc::new(vec![1, 2, 3]));
    ///
    /// let any_ref = resolved.get_arc("data");
    /// assert!(any_ref.is_some());
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    #[must_use]
    pub fn get_arc(&self, key: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        self.dependencies.lock().unwrap().get(key).cloned()
    }

    /// Check if a dependency exists
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::Arc;
    ///
    /// let mut resolved = ResolvedDependencies::new();
    /// resolved.insert("exists".to_string(), Arc::new(true));
    ///
    /// assert!(resolved.contains("exists"));
    /// assert!(!resolved.contains("missing"));
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    #[must_use]
    pub fn contains(&self, key: &str) -> bool {
        self.dependencies.lock().unwrap().contains_key(key)
    }

    /// Get all dependency keys
    ///
    /// Returns a vector of all keys currently stored in this resolved dependencies.
    /// Useful for iterating over all dependencies when you need to extract them.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::Arc;
    ///
    /// let mut resolved = ResolvedDependencies::new();
    /// resolved.insert("config".to_string(), Arc::new("prod".to_string()));
    /// resolved.insert("db".to_string(), Arc::new(42i32));
    ///
    /// let keys = resolved.keys();
    /// assert_eq!(keys.len(), 2);
    /// assert!(keys.contains(&"config".to_string()));
    /// assert!(keys.contains(&"db".to_string()));
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    #[must_use]
    pub fn keys(&self) -> Vec<String> {
        self.dependencies.lock().unwrap().keys().cloned().collect()
    }

    /// Add a cleanup task to be run when dependencies are cleaned up
    ///
    /// Cleanup tasks are useful for generator-pattern dependencies that need
    /// to perform cleanup (e.g., closing database connections, releasing locks).
    ///
    /// # Arguments
    ///
    /// * `task` - A function that returns a future performing cleanup
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    ///
    /// # tokio_test::block_on(async {
    /// let mut resolved = ResolvedDependencies::new();
    ///
    /// resolved.add_cleanup_task(Box::new(|| {
    ///     Box::pin(async {
    ///         println!("Cleaning up resources");
    ///     })
    /// }));
    ///
    /// resolved.cleanup().await;
    /// # });
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    pub fn add_cleanup_task(&self, task: CleanupTask) {
        self.cleanup_tasks.lock().unwrap().push(task);
    }

    /// Run all cleanup tasks in reverse order
    ///
    /// Cleanup tasks are executed in LIFO order (last added, first executed)
    /// to properly handle nested resource dependencies.
    ///
    /// This consumes self to ensure cleanup is only run once.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::ResolvedDependencies;
    /// use std::sync::{Arc, Mutex};
    ///
    /// # tokio_test::block_on(async {
    /// let order = Arc::new(Mutex::new(Vec::new()));
    ///
    /// let mut resolved = ResolvedDependencies::new();
    ///
    /// let order1 = order.clone();
    /// resolved.add_cleanup_task(Box::new(move || {
    ///     Box::pin(async move {
    ///         order1.lock().unwrap().push(1);
    ///     })
    /// }));
    ///
    /// let order2 = order.clone();
    /// resolved.add_cleanup_task(Box::new(move || {
    ///     Box::pin(async move {
    ///         order2.lock().unwrap().push(2);
    ///     })
    /// }));
    ///
    /// resolved.cleanup().await;
    ///
    /// // Tasks run in reverse order (LIFO)
    /// assert_eq!(*order.lock().unwrap(), vec![2, 1]);
    /// # });
    /// ```
    ///
    /// # Panics
    /// Panics if the lock is poisoned.
    pub async fn cleanup(self) {
        let tasks = {
            let mut cleanup_tasks = self.cleanup_tasks.lock().unwrap();
            std::mem::take(&mut *cleanup_tasks)
        };

        for task in tasks.into_iter().rev() {
            task().await;
        }
    }
}

impl std::fmt::Debug for ResolvedDependencies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deps = self.dependencies.lock().unwrap();
        let tasks = self.cleanup_tasks.lock().unwrap();
        f.debug_struct("ResolvedDependencies")
            .field("dependencies", &deps.keys())
            .field("cleanup_tasks_count", &tasks.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let resolved = ResolvedDependencies::new();
        assert!(!resolved.contains("anything"));
    }

    #[test]
    fn test_insert_and_get() {
        let mut resolved = ResolvedDependencies::new();
        let value = Arc::new(42i32);
        resolved.insert("answer".to_string(), value);

        let retrieved: Option<Arc<i32>> = resolved.get("answer");
        assert_eq!(retrieved.map(|v| *v), Some(42));
    }

    #[test]
    fn test_get_type_mismatch() {
        let mut resolved = ResolvedDependencies::new();
        resolved.insert("number".to_string(), Arc::new(42i32));

        let wrong: Option<Arc<String>> = resolved.get("number");
        assert!(wrong.is_none());
    }

    #[test]
    fn test_get_missing() {
        let resolved = ResolvedDependencies::new();
        let missing: Option<Arc<i32>> = resolved.get("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_get_arc() {
        let mut resolved = ResolvedDependencies::new();
        resolved.insert("data".to_string(), Arc::new(vec![1, 2, 3]));

        let any_ref = resolved.get_arc("data");
        assert!(any_ref.is_some());

        let vec_ref = any_ref.unwrap().downcast::<Vec<i32>>().ok();
        assert!(vec_ref.is_some());
    }

    #[test]
    fn test_contains() {
        let mut resolved = ResolvedDependencies::new();
        resolved.insert("exists".to_string(), Arc::new(true));

        assert!(resolved.contains("exists"));
        assert!(!resolved.contains("missing"));
    }

    #[tokio::test]
    async fn test_cleanup_order() {
        let order = Arc::new(Mutex::new(Vec::new()));

        let resolved = ResolvedDependencies::new();

        let order1 = order.clone();
        resolved.add_cleanup_task(Box::new(move || {
            Box::pin(async move {
                order1.lock().unwrap().push(1);
            })
        }));

        let order2 = order.clone();
        resolved.add_cleanup_task(Box::new(move || {
            Box::pin(async move {
                order2.lock().unwrap().push(2);
            })
        }));

        let order3 = order.clone();
        resolved.add_cleanup_task(Box::new(move || {
            Box::pin(async move {
                order3.lock().unwrap().push(3);
            })
        }));

        resolved.cleanup().await;

        assert_eq!(*order.lock().unwrap(), vec![3, 2, 1]);
    }

    #[tokio::test]
    async fn test_cleanup_empty() {
        let resolved = ResolvedDependencies::new();
        resolved.cleanup().await;
    }

    #[test]
    fn test_clone() {
        let mut resolved1 = ResolvedDependencies::new();
        resolved1.insert("key".to_string(), Arc::new(42i32));

        let resolved2 = resolved1.clone();
        let value: Option<Arc<i32>> = resolved2.get("key");
        assert_eq!(value.map(|v| *v), Some(42));
    }
}
