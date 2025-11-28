//! Dependency Injection for PHP bindings.
//!
//! Provides DI container support matching Python/Node/Ruby patterns:
//! - Value dependencies (singletons)
//! - Factory dependencies (callables)
//! - Scoped dependencies (per-request)
//!
//! Uses thread-local storage for PHP Zvals (non-Send/Sync).

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use spikard_core::di::{Dependency, DependencyContainer, DependencyError, ResolvedDependencies};
use spikard_http::RequestData;
use http::Request;
use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

thread_local! {
    /// Registry of PHP factory callables.
    /// Stores Zval references to PHP closures/callables for factory dependencies.
    /// Indexed by factory_id from PhpFactoryDependency.
    static PHP_FACTORY_REGISTRY: RefCell<Vec<Zval>> = RefCell::new(Vec::new());
}

/// PHP value dependency (singleton instance).
///
/// Wraps a Zval containing a PHP object/value that's reused across requests.
/// The Zval is cloned (shallow) when resolving the dependency.
#[derive(Clone)]
pub struct PhpValueDependency {
    key: String,
    value: Arc<Zval>,
}

impl PhpValueDependency {
    pub fn new(key: String, value: Zval) -> Self {
        Self {
            key,
            value: Arc::new(value),
        }
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
        // Clone the Zval (shallow copy)
        let value_clone = self.value.shallow_clone();

        // Wrap in Arc<Any> for the DI system
        // Note: Zval is not Send+Sync, but we handle it carefully in thread-local context
        let boxed: Arc<dyn Any + Send + Sync> =
            Arc::new(value_clone) as Arc<dyn Any + Send + Sync>;

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
    /// * `resolved` - ResolvedDependencies containing already-resolved dependencies
    ///
    /// # Returns
    /// Result containing the created instance as Zval
    fn invoke_factory(&self, resolved: &ResolvedDependencies) -> Result<Zval, DependencyError> {
        PHP_FACTORY_REGISTRY.with(|registry| {
            let reg = registry.borrow();
            let callable_zval = reg
                .get(self.factory_id)
                .ok_or_else(|| DependencyError::ResolutionFailed(format!("Factory {} not found in registry", self.factory_id)))?;

            // Build argument array from dependencies
            let mut args = Vec::new();
            for dep_name in &self.dependencies {
                // Get resolved dependency using the new API
                let resolved_value = resolved
                    .get(dep_name)
                    .ok_or_else(|| DependencyError::ResolutionFailed(format!("Dependency '{}' not resolved", dep_name)))?;

                // Downcast Arc<Any> back to Zval
                let zval_ref = resolved_value
                    .downcast_ref::<Zval>()
                    .ok_or_else(|| DependencyError::ResolutionFailed(format!("Dependency '{}' is not a Zval", dep_name)))?;

                args.push(zval_ref);
            }

            // Invoke PHP callable
            let callable = ZendCallable::new(callable_zval.clone(), None);
            let result = callable
                .try_call(args)
                .map_err(|e| DependencyError::ResolutionFailed(format!("Factory invocation failed: {:?}", e)))?;

            Ok(result)
        })
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
        let result = match self.invoke_factory(resolved) {
            Ok(zval) => Arc::new(zval) as Arc<dyn Any + Send + Sync>,
            Err(e) => return Box::pin(async move { Err(e) }),
        };

        Box::pin(async move { Ok(result) })
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
        obj.get_property("dependencies")
            .ok_or_else(|| "DependencyContainer must have 'dependencies' property".to_string())?
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
            if let Some(class_name) = dep_obj.get_class_name() {
                if class_name.contains("Provide") {
                    // Extract factory callable and dependencies
                    let factory_callable = dep_obj
                        .get_property("factory")
                        .ok_or_else(|| format!("Provide instance '{}' missing 'factory' property", dep_name))?;

                    let depends_on_zval = dep_obj
                        .get_property("dependsOn")
                        .ok_or_else(|| format!("Provide instance '{}' missing 'dependsOn' property", dep_name))?;

                    let depends_on = if let Some(arr) = depends_on_zval.array() {
                        arr.values()
                            .filter_map(|v| v.string())
                            .map(|s| s.to_string())
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let factory = PhpFactoryDependency::register(dep_name.clone(), factory_callable, depends_on);
                    container.register(dep_name, Arc::new(factory))
                        .map_err(|e| format!("Failed to register factory: {:?}", e))?;
                } else {
                    // Plain object value dependency
                    let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
                    container.register(dep_name, Arc::new(value))
                        .map_err(|e| format!("Failed to register value: {:?}", e))?;
                }
            } else {
                // Object without class name - treat as value
                let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
                container.register(dep_name, Arc::new(value))
                    .map_err(|e| format!("Failed to register value: {:?}", e))?;
            }
        } else {
            // Scalar value dependency
            let value = PhpValueDependency::new(dep_name.clone(), dep_val.shallow_clone());
            container.register(dep_name, Arc::new(value))
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
