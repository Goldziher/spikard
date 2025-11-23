//! Node.js dependency injection implementations
//!
//! This module provides Node.js-specific implementations of the Dependency trait,
//! bridging JavaScript values and factories to the Rust DI system.

use http::Request;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use spikard_core::di::{Dependency, ResolvedDependencies};
use spikard_core::request_data::RequestData;
use std::any::Any;
use std::sync::Arc;

/// Node.js value dependency
///
/// Wraps a JavaScript value as a static dependency.
/// The value is stored as JSON string to avoid GC issues across threads.
pub struct NodeValueDependency {
    key: String,
    value_json: String,
}

impl NodeValueDependency {
    /// Create a new Node value dependency from a JSON string
    pub fn new(key: String, value_json: String) -> Self {
        Self { key, value_json }
    }
}

impl Dependency for NodeValueDependency {
    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        vec![] // Value dependencies have no dependencies
    }

    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<Arc<dyn Any + Send + Sync>, spikard_core::di::DependencyError>,
                > + Send
                + '_,
        >,
    > {
        let value = self.value_json.clone();
        Box::pin(async move {
            // Store as JSON string to pass across threads
            Ok(Arc::new(value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn singleton(&self) -> bool {
        true // Value dependencies are always singletons
    }

    fn cacheable(&self) -> bool {
        true
    }
}

/// Node.js factory dependency
///
/// Wraps a JavaScript callable as a factory dependency.
/// Uses ThreadsafeFunction to call JS from Rust async context.
pub struct NodeFactoryDependency {
    key: String,
    factory_fn: Arc<ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>>,
    depends_on: Vec<String>,
    singleton: bool,
    cacheable: bool,
}

unsafe impl Send for NodeFactoryDependency {}
unsafe impl Sync for NodeFactoryDependency {}

impl NodeFactoryDependency {
    /// Create a new Node factory dependency
    pub fn new(
        key: String,
        factory_fn: ThreadsafeFunction<String, Promise<String>, Vec<String>, napi::Status, false>,
        depends_on: Vec<String>,
        singleton: bool,
        cacheable: bool,
    ) -> Self {
        Self {
            key,
            factory_fn: Arc::new(factory_fn),
            depends_on,
            singleton,
            cacheable,
        }
    }
}

impl Dependency for NodeFactoryDependency {
    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        self.depends_on.clone()
    }

    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        resolved: &ResolvedDependencies,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = std::result::Result<Arc<dyn Any + Send + Sync>, spikard_core::di::DependencyError>,
                > + Send
                + '_,
        >,
    > {
        // Clone what we need before async
        let factory_fn = Arc::clone(&self.factory_fn);
        let depends_on = self.depends_on.clone();

        Box::pin(async move {
            // Build dependencies object as JSON
            let mut deps_map = std::collections::HashMap::new();
            for dep_key in &depends_on {
                if let Some(dep_value) = resolved.get::<String>(dep_key) {
                    // Dependencies are stored as JSON strings
                    let parsed: serde_json::Value = serde_json::from_str(&dep_value).map_err(|e| {
                        spikard_core::di::DependencyError::ResolutionFailed {
                            message: format!("Failed to parse dependency {}: {}", dep_key, e),
                        }
                    })?;
                    deps_map.insert(dep_key.clone(), parsed);
                }
            }

            let deps_json =
                serde_json::to_string(&deps_map).map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                    message: format!("Failed to serialize dependencies: {}", e),
                })?;

            // Call the factory function
            let result = factory_fn
                .call_async::<Promise<String>>(deps_json.clone())
                .await
                .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                    message: format!("Failed to call factory: {}", e),
                })?
                .await
                .map_err(|e| spikard_core::di::DependencyError::ResolutionFailed {
                    message: format!("Factory promise failed: {}", e),
                })?;

            // Store result as JSON string
            Ok(Arc::new(result) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn singleton(&self) -> bool {
        self.singleton
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }
}

/// Extract dependency container from Node.js app object
///
/// Builds a DependencyContainer from the dependencies registered on the app.
/// Returns None if no dependencies are registered.
pub fn extract_dependency_container(app: &Object) -> Result<Option<Arc<spikard_core::di::DependencyContainer>>> {
    // Try to get dependencies object
    let dependencies_opt: Option<Object> = app.get("dependencies")?;

    if dependencies_opt.is_none() {
        return Ok(None);
    }

    let dependencies = dependencies_opt.unwrap();
    let dep_keys: Vec<String> = dependencies.get_property_names()?.into_iter().collect();

    if dep_keys.is_empty() {
        return Ok(None);
    }

    let mut container = spikard_core::di::DependencyContainer::new();

    for key in dep_keys {
        let dep_obj: Object = dependencies
            .get(&key)?
            .ok_or_else(|| Error::from_reason(format!("Dependency {} not found", key)))?;

        // Check if it's a factory or a value
        let is_factory: bool = dep_obj.get("isFactory")?.unwrap_or(false);

        if is_factory {
            // Extract factory configuration
            let factory_fn: Function<String, Promise<String>> = dep_obj
                .get("factory")?
                .ok_or_else(|| Error::from_reason(format!("Factory function not found for {}", key)))?;

            let depends_on: Vec<String> = dep_obj.get("dependsOn")?.unwrap_or_default();

            let singleton: bool = dep_obj.get("singleton")?.unwrap_or(false);
            let cacheable: bool = dep_obj.get("cacheable")?.unwrap_or(false);

            // Build ThreadsafeFunction
            let tsfn = factory_fn
                .build_threadsafe_function()
                .build_callback(|ctx| Ok(vec![ctx.value]))
                .map_err(|e| {
                    Error::from_reason(format!(
                        "Failed to build ThreadsafeFunction for factory '{}': {}",
                        key, e
                    ))
                })?;

            let factory_dep = NodeFactoryDependency::new(key.clone(), tsfn, depends_on, singleton, cacheable);

            container
                .register(Arc::new(factory_dep))
                .map_err(|e| Error::from_reason(format!("Failed to register factory {}: {}", key, e)))?;
        } else {
            // Value dependency - serialize to JSON
            let value: Unknown = dep_obj
                .get("value")?
                .ok_or_else(|| Error::from_reason(format!("Value not found for dependency {}", key)))?;

            // Convert to JSON string
            let env = dep_obj.env;
            let global = env.get_global()?;
            let json_obj: Object = global.get_named_property("JSON")?;
            let stringify: Function<Unknown, String> = json_obj.get_named_property("stringify")?;

            let value_json: String = stringify.call(None, &[value])?;

            let value_dep = NodeValueDependency::new(key.clone(), value_json);

            container
                .register(key.clone(), Arc::new(value_dep))
                .map_err(|e| Error::from_reason(format!("Failed to register value {}: {}", key, e)))?;
        }
    }

    Ok(Some(Arc::new(container)))
}

/// Convert resolved dependencies to JavaScript object
///
/// Takes the resolved dependencies (stored as JSON strings) and converts them
/// back to JavaScript values for handler consumption.
pub fn resolved_to_js_object<'a>(
    env: Env<'a>,
    resolved: &ResolvedDependencies,
    keys: &[String],
) -> napi::Result<Object> {
    let obj = env.create_object()?;

    let global = env.get_global()?;
    let json_obj: Object = global.get_named_property("JSON")?;
    let parse: Function<String, Unknown> = json_obj.get_named_property("parse")?;

    for key in keys {
        if let Some(value_json) = resolved.get::<String>(key) {
            // Parse JSON string back to JS value
            let js_value = parse.call(None, &[env.create_string(&value_json)?])?;
            obj.set(key.as_str(), js_value)?;
        }
    }

    Ok(obj)
}
