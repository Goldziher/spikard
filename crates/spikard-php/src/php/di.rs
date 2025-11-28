//! Dependency Injection for PHP bindings.
//!
//! Provides DI container support matching Python/Node/Ruby patterns:
//! - Value dependencies (singletons)
//! - Factory dependencies (callables)
//! - Scoped dependencies (per-request)
//!
//! ## Thread Safety Architecture
//!
//! PHP Zvals contain raw pointers and cannot be Send+Sync. This module uses
//! thread-local storage to safely handle Zvals across async boundaries:
//!
//! 1. All Zvals are stored in thread_local! registries
//! 2. Dependencies hold only numeric IDs (which ARE Send+Sync)
//! 3. When resolving, IDs are used to look up Zvals from thread-local storage
//! 4. Resolved values are wrapped in ZvalHandle (Send+Sync wrapper with ID)

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use http::Request;
use spikard_core::RequestData;
use spikard_core::di::{Dependency, DependencyContainer, DependencyError, ResolvedDependencies};
use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

thread_local! {
    /// Registry of PHP value dependencies (singletons).
    /// Maps value_id -> Zval. Thread-local to avoid Send+Sync issues.
    static PHP_VALUE_REGISTRY: RefCell<Vec<Zval>> = RefCell::new(Vec::new());

    /// Registry of PHP factory callables.
    /// Maps factory_id -> Zval callable. Thread-local to avoid Send+Sync issues.
    static PHP_FACTORY_REGISTRY: RefCell<Vec<Zval>> = RefCell::new(Vec::new());
}

/// Send+Sync wrapper around a Zval ID.
///
/// This type is safe to send across threads because it only holds a numeric ID.
/// The actual Zval is stored in thread-local storage and retrieved when needed.
#[derive(Debug, Clone)]
pub struct ZvalHandle {
    /// Index into PHP_VALUE_REGISTRY
    value_id: usize,
}

impl ZvalHandle {
    /// Create a new handle from a value ID
    fn new(value_id: usize) -> Self {
        Self { value_id }
    }

    /// Get the Zval from thread-local storage
    pub fn get(&self) -> Result<Zval, DependencyError> {
        PHP_VALUE_REGISTRY.with(|registry| {
            let reg = registry.borrow();
            reg.get(self.value_id)
                .map(|z| z.shallow_clone())
                .ok_or_else(|| DependencyError::ResolutionFailed {
                    message: format!("Zval handle {} not found in registry", self.value_id),
                })
        })
    }
}

// SAFETY: ZvalHandle only contains a numeric ID, not raw pointers
// The actual Zval is stored in thread-local storage
unsafe impl Send for ZvalHandle {}
unsafe impl Sync for ZvalHandle {}

/// PHP value dependency (singleton instance).
///
/// Stores a Zval in thread-local storage and returns a Send+Sync handle.
/// The actual Zval is registered once and accessed via ID.
#[derive(Clone)]
pub struct PhpValueDependency {
    key: String,
    /// Index into PHP_VALUE_REGISTRY (Send+Sync safe)
    value_id: usize,
}

impl PhpValueDependency {
    /// Register a PHP value and return a dependency that references it.
    ///
    /// # Arguments
    /// * `key` - Unique dependency key
    /// * `value` - PHP Zval to store (will be moved to thread-local storage)
    ///
    /// # Returns
    /// PhpValueDependency with registered value_id
    pub fn new(key: String, value: Zval) -> Self {
        let value_id = PHP_VALUE_REGISTRY.with(|registry| {
            let mut reg = registry.borrow_mut();
            let id = reg.len();
            reg.push(value);
            id
        });

        Self { key, value_id }
    }
}

impl Dependency for PhpValueDependency {
    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![] // Value dependencies don't depend on other dependencies
    }

    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> BoxFuture<'_, Result<Arc<dyn Any + Send + Sync>, DependencyError>> {
        // Create a Send+Sync handle that references the value
        let handle = ZvalHandle::new(self.value_id);

        // Wrap the handle (which IS Send+Sync) in Arc<Any>
        let boxed: Arc<dyn Any + Send + Sync> = Arc::new(handle);

        Box::pin(async move { Ok(boxed) })
    }
}

/// PHP factory dependency (callable).
///
/// Stores an index into PHP_FACTORY_REGISTRY pointing to a PHP callable.
/// When resolved, invokes the PHP callable with resolved dependencies.
#[derive(Clone)]
pub struct PhpFactoryDependency {
    key: String,
    factory_id: usize,
    dependencies: Vec<String>,
}

impl PhpFactoryDependency {
    /// Register a PHP factory callable and return a PhpFactoryDependency.
    ///
    /// # Arguments
    /// * `key` - Unique key for this dependency
    /// * `callable` - PHP callable (Zval) to invoke for dependency creation
    /// * `depends_on` - List of dependency names this factory requires
    ///
    /// # Returns
    /// PhpFactoryDependency with registered factory_id
    pub fn register(key: String, callable: Zval, depends_on: Vec<String>) -> Self {
        let factory_id = PHP_FACTORY_REGISTRY.with(|registry| {
            let mut reg = registry.borrow_mut();
            let id = reg.len();
            reg.push(callable);
            id
        });

        Self {
            key,
            factory_id,
            dependencies: depends_on,
        }
    }

    /// Invoke the PHP factory with resolved dependencies.
    ///
    /// # Arguments
    /// * `resolved` - ResolvedDependencies containing ZvalHandles for dependencies
    ///
    /// # Returns
    /// Result containing the created instance value ID
    fn invoke_factory(&self, resolved: &ResolvedDependencies) -> Result<usize, DependencyError> {
        // Collect resolved Zvals from handles
        let mut zvals = Vec::new();
        for dep_name in &self.dependencies {
            let resolved_value = match resolved.get_arc(dep_name) {
                Some(v) => v,
                None => {
                    return Err(DependencyError::ResolutionFailed {
                        message: format!("Dependency '{}' not resolved", dep_name),
                    });
                }
            };

            // Downcast to ZvalHandle
            let handle =
                resolved_value
                    .downcast_ref::<ZvalHandle>()
                    .ok_or_else(|| DependencyError::ResolutionFailed {
                        message: format!("Dependency '{}' is not a ZvalHandle", dep_name),
                    })?;

            // Get the actual Zval from thread-local storage
            let zval = handle.get()?;
            zvals.push(zval);
        }

        // Invoke factory callable with the resolved Zvals
        let result_zval = PHP_FACTORY_REGISTRY.with(|registry| {
            let reg = registry.borrow();
            let callable_zval = reg
                .get(self.factory_id)
                .ok_or_else(|| DependencyError::ResolutionFailed {
                    message: format!("Factory {} not found in registry", self.factory_id),
                })?;

            // Build argument references for try_call
            let args: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = zvals
                .iter()
                .map(|v| v as &dyn ext_php_rs::convert::IntoZvalDyn)
                .collect();

            // Invoke PHP callable
            let callable = ZendCallable::new(callable_zval).map_err(|e| DependencyError::ResolutionFailed {
                message: format!("Failed to create callable: {:?}", e),
            })?;
            let result = callable.try_call(args).map_err(|e| DependencyError::ResolutionFailed {
                message: format!("Factory invocation failed: {:?}", e),
            })?;

            Ok(result)
        })?;

        // Store the result in the value registry and return its ID
        let value_id = PHP_VALUE_REGISTRY.with(|registry| {
            let mut reg = registry.borrow_mut();
            let id = reg.len();
            reg.push(result_zval);
            id
        });

        Ok(value_id)
    }
}

impl Dependency for PhpFactoryDependency {
    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> BoxFuture<'_, Result<Arc<dyn Any + Send + Sync>, DependencyError>> {
        // Invoke factory synchronously (PHP has no async)
        // The DI container has already resolved all dependencies we declared in depends_on()
        // Returns a value_id, which we wrap in a ZvalHandle
        let value_id = match self.invoke_factory(resolved) {
            Ok(id) => id,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        // Create a Send+Sync handle that references the factory result
        let handle = ZvalHandle::new(value_id);
        let boxed: Arc<dyn Any + Send + Sync> = Arc::new(handle);

        Box::pin(async move { Ok(boxed) })
    }
}

/// Extract DependencyContainer from PHP DependencyContainer object.
///
/// # Arguments
/// * `container_zval` - Zval containing PHP DependencyContainer instance
///
/// # Returns
/// Result containing populated DependencyContainer for Rust
pub fn extract_di_container_from_php(container_zval: Option<&Zval>) -> Result<Option<DependencyContainer>, String> {
    let container_zval = match container_zval {
        Some(z) if !z.is_null() => z,
        _ => return Ok(None),
    };

    // Get dependencies array from PHP container
    // Assumes: $container->dependencies is a public property or has getDependencies()
    let deps_array = if let Some(obj) = container_zval.object() {
        // Try to get 'dependencies' property
        obj.get_property::<&Zval>("dependencies")
            .map_err(|_| "DependencyContainer must have 'dependencies' property".to_string())?
    } else {
        return Err("DI container must be an object".to_string());
    };

    let deps_map = deps_array
        .array()
        .ok_or_else(|| "dependencies must be an array".to_string())?;

    let mut container = DependencyContainer::new();

    for (key, dep_val) in deps_map.iter() {
        let dep_name = match key {
            ext_php_rs::types::ArrayKey::String(s) => s.to_string(),
            ext_php_rs::types::ArrayKey::Str(s) => s.to_string(),
            ext_php_rs::types::ArrayKey::Long(l) => l.to_string(),
        };

        // Check if it's a factory or value dependency
        if let Some(dep_obj) = dep_val.object() {
            // Check if it's a Provide instance (factory)
            if let Ok(class_name) = dep_obj.get_class_name() {
                if class_name.contains("Provide") {
                    // Extract factory callable and dependencies
                    let factory_callable = dep_obj
                        .get_property::<&Zval>("factory")
                        .map_err(|_| format!("Provide instance '{}' missing 'factory' property", dep_name))?;

                    let depends_on_zval = dep_obj
                        .get_property::<&Zval>("dependsOn")
                        .map_err(|_| format!("Provide instance '{}' missing 'dependsOn' property", dep_name))?;

                    let depends_on = if let Some(arr) = depends_on_zval.array() {
                        arr.values().filter_map(|v| v.string()).map(|s| s.to_string()).collect()
                    } else {
                        Vec::new()
                    };

                    let factory =
                        PhpFactoryDependency::register(dep_name.clone(), factory_callable.shallow_clone(), depends_on);
                    container
                        .register(dep_name, Arc::new(factory))
                        .map_err(|e| format!("Failed to register factory: {:?}", e))?;
                } else {
                    // Plain object value dependency
                    let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
                    container
                        .register(dep_name, Arc::new(value))
                        .map_err(|e| format!("Failed to register value: {:?}", e))?;
                }
            } else {
                // Object without class name - treat as value
                let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
                container
                    .register(dep_name, Arc::new(value))
                    .map_err(|e| format!("Failed to register value: {:?}", e))?;
            }
        } else {
            // Scalar value dependency
            let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
            container
                .register(dep_name, Arc::new(value))
                .map_err(|e| format!("Failed to register value: {:?}", e))?;
        }
    }

    Ok(Some(container))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_dependency() {
        // Test that PhpValueDependency can be created
        let zval = Zval::new(); // Create a null Zval for testing
        let dep = PhpValueDependency::new(zval);
        assert!(Arc::strong_count(&dep.value) == 1);
    }

    #[test]
    fn test_factory_registry() {
        // Test that factory registry works
        PHP_FACTORY_REGISTRY.with(|registry| {
            registry.borrow_mut().clear();
            let initial_len = registry.borrow().len();
            assert_eq!(initial_len, 0);

            let callable = Zval::new();
            let _factory = PhpFactoryDependency::register(callable, vec![]);

            let final_len = registry.borrow().len();
            assert_eq!(final_len, 1);
        });
    }
}
