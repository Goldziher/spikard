//! Ruby dependency injection implementations
//!
//! This module provides Ruby-specific implementations of the Dependency trait,
//! bridging Ruby values and Procs to the Rust DI system.

use http::Request;
use magnus::prelude::*;
use magnus::value::{InnerValue, Opaque};
use magnus::{Error, RHash, Ruby, TryConvert, Value};
use serde_json::Value as JsonValue;
use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};
use spikard_core::request_data::RequestData;
use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

/// Ruby value dependency
///
/// Wraps a Ruby object as a static dependency value
pub struct RubyValueDependency {
    key: String,
    value: Opaque<Value>,
}

impl RubyValueDependency {
    pub fn new(key: String, value: Value) -> Self {
        Self {
            key,
            value: Opaque::from(value),
        }
    }
}

impl Dependency for RubyValueDependency {
    fn key(&self) -> &str {
        &self.key
    }

    fn depends_on(&self) -> Vec<String> {
        Vec::new() // Value dependencies have no dependencies
    }

    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>>
    {
        Box::pin(async move {
            // Get the Ruby value
            let ruby = Ruby::get().map_err(|e| DependencyError::ResolutionFailed { message: e.to_string() })?;

            let value = self.value.get_inner_with(&ruby);

            // Convert to JSON and back to make it Send + Sync
            let json_value = ruby_value_to_json(&ruby, value)
                .map_err(|e| DependencyError::ResolutionFailed { message: e.to_string() })?;

            Ok(Arc::new(json_value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn singleton(&self) -> bool {
        true // Value dependencies are always singletons
    }

    fn cacheable(&self) -> bool {
        true
    }
}

/// Ruby factory dependency
///
/// Wraps a Ruby Proc as a factory dependency
pub struct RubyFactoryDependency {
    key: String,
    factory: Opaque<Value>,
    depends_on: Vec<String>,
    singleton: bool,
    cacheable: bool,
}

impl RubyFactoryDependency {
    pub fn new(key: String, factory: Value, depends_on: Vec<String>, singleton: bool, cacheable: bool) -> Self {
        Self {
            key,
            factory: Opaque::from(factory),
            depends_on,
            singleton,
            cacheable,
        }
    }
}

impl Dependency for RubyFactoryDependency {
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
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>>
    {
        // Clone data needed in async block
        let factory = self.factory.clone();
        let depends_on = self.depends_on.clone();
        let key = self.key.clone();
        let is_singleton = self.singleton;
        let resolved_clone = resolved.clone();

        // Extract resolved dependencies now (before async)
        // Need to handle both JsonValue and RubyValueWrapper types
        let resolved_deps: Vec<(String, JsonValue)> = depends_on
            .iter()
            .filter_map(|dep_key| {
                // Try JsonValue first
                if let Some(json_value) = resolved.get::<JsonValue>(dep_key) {
                    return Some((dep_key.clone(), (*json_value).clone()));
                }
                // Try RubyValueWrapper (for singletons)
                if let Some(wrapper) = resolved.get::<RubyValueWrapper>(dep_key) {
                    // Convert wrapper to JSON synchronously
                    if let Ok(ruby) = Ruby::get() {
                        if let Ok(json) = wrapper.to_json(&ruby) {
                            return Some((dep_key.clone(), json));
                        }
                    }
                }
                None
            })
            .collect();

        Box::pin(async move {
            let ruby = Ruby::get().map_err(|e| DependencyError::ResolutionFailed { message: e.to_string() })?;

            // Build positional arguments array from resolved dependencies
            // Dependencies must be passed in the order specified by depends_on
            // Important: preserve the order from depends_on, not from resolved_deps iteration
            let args: Result<Vec<Value>, DependencyError> = depends_on
                .iter()
                .filter_map(|dep_key| {
                    // Find this dependency in resolved_deps
                    resolved_deps.iter().find(|(k, _)| k == dep_key).map(|(_, v)| v)
                })
                .map(|dep_value| {
                    json_to_ruby(&ruby, dep_value).map_err(|e| DependencyError::ResolutionFailed {
                        message: format!("Failed to convert dependency value: {}", e),
                    })
                })
                .collect();
            let args = args?;

            // Call the factory Proc with positional arguments
            let factory_value = factory.get_inner_with(&ruby);

            // Check if factory responds to call
            if !factory_value
                .respond_to("call", false)
                .map_err(|e| DependencyError::ResolutionFailed { message: e.to_string() })?
            {
                return Err(DependencyError::ResolutionFailed {
                    message: format!("Dependency factory for '{}' is not callable", key),
                });
            }

            // Call factory with positional arguments
            // Use a Ruby helper to call with splatted arguments
            let result: Value = if !args.is_empty() {
                // Create a Ruby array of arguments
                let args_array = ruby.ary_new();
                for arg in &args {
                    args_array.push(*arg).map_err(|e| DependencyError::ResolutionFailed {
                        message: format!("Failed to push arg to array: {}", e),
                    })?;
                }

                // Use Ruby's send with * to splat arguments
                // Equivalent to: factory_value.call(*args_array)
                let splat_lambda = ruby
                    .eval::<Value>("lambda { |proc, args| proc.call(*args) }")
                    .map_err(|e| DependencyError::ResolutionFailed {
                        message: format!("Failed to create splat lambda: {}", e),
                    })?;

                splat_lambda.funcall("call", (factory_value, args_array))
            } else {
                factory_value.funcall("call", ())
            }
            .map_err(|e| DependencyError::ResolutionFailed {
                message: format!("Failed to call factory for '{}': {}", key, e),
            })?;

            // Check if result is an array with cleanup callback (Ruby pattern: [resource, cleanup_proc])
            let (value_to_convert, cleanup_callback) = if result.is_kind_of(ruby.class_array()) {
                let array = magnus::RArray::from_value(result).ok_or_else(|| DependencyError::ResolutionFailed {
                    message: format!("Failed to convert result to array for '{}'", key),
                })?;

                let len = array.len();
                if len == 2 {
                    // Extract the resource (first element)
                    let resource: Value = array.entry(0).map_err(|e| DependencyError::ResolutionFailed {
                        message: format!("Failed to extract resource from array for '{}': {}", key, e),
                    })?;

                    // Extract cleanup callback (second element)
                    let cleanup: Value = array.entry(1).map_err(|e| DependencyError::ResolutionFailed {
                        message: format!("Failed to extract cleanup callback for '{}': {}", key, e),
                    })?;

                    (resource, Some(cleanup))
                } else {
                    // Not a cleanup pattern, use the array as-is
                    (result, None)
                }
            } else {
                // Not an array, use the value as-is
                (result, None)
            };

            // Register cleanup callback if present
            if let Some(cleanup_proc) = cleanup_callback {
                let cleanup_opaque = Opaque::from(cleanup_proc);

                resolved_clone.add_cleanup_task(Box::new(move || {
                    Box::pin(async move {
                        // Get Ruby runtime and call cleanup proc
                        if let Ok(ruby) = Ruby::get() {
                            let proc = cleanup_opaque.get_inner_with(&ruby);
                            // Call the cleanup proc - ignore errors during cleanup
                            let _ = proc.funcall::<_, _, Value>("call", ());
                        }
                    })
                }));
            }

            // For singleton dependencies, store Ruby value wrapper to preserve mutations
            // For non-singleton, convert to JSON immediately (no need to preserve mutations)
            if is_singleton {
                let wrapper = RubyValueWrapper::new(value_to_convert);
                return Ok(Arc::new(wrapper) as Arc<dyn Any + Send + Sync>);
            }

            // Convert result to JSON for non-singleton dependencies
            let json_value = ruby_value_to_json(&ruby, value_to_convert)
                .map_err(|e| DependencyError::ResolutionFailed { message: e.to_string() })?;

            Ok(Arc::new(json_value) as Arc<dyn Any + Send + Sync>)
        })
    }

    fn singleton(&self) -> bool {
        self.singleton
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }
}

/// Wrapper around a Ruby Value that preserves object identity for singleton mutations
///
/// This stores the Ruby object itself rather than a JSON snapshot, allowing
/// singleton dependencies to maintain mutable state across requests.
#[derive(Clone)]
pub struct RubyValueWrapper {
    /// Thread-safe wrapper around Ruby Value
    /// Opaque<Value> is Send + Sync per magnus design
    value: Opaque<Value>,
}

impl RubyValueWrapper {
    /// Create a new wrapper around a Ruby value
    pub fn new(value: Value) -> Self {
        Self {
            value: Opaque::from(value),
        }
    }

    /// Get the raw Ruby value directly
    ///
    /// This preserves object identity for singletons, allowing mutations
    /// to persist across requests.
    pub fn get_value(&self, ruby: &Ruby) -> Value {
        self.value.get_inner_with(ruby)
    }

    /// Convert the wrapped Ruby value to JSON
    ///
    /// This is called fresh each time to capture any mutations to the object.
    /// For singletons, this means we see updated counter values, etc.
    pub fn to_json(&self, ruby: &Ruby) -> Result<JsonValue, Error> {
        let value = self.value.get_inner_with(ruby);
        ruby_value_to_json(ruby, value)
    }
}

// Safety: Opaque<Value> is designed to be Send + Sync by magnus
// It holds a stable pointer that's safe to share across threads
unsafe impl Send for RubyValueWrapper {}
unsafe impl Sync for RubyValueWrapper {}

/// Convert Ruby Value to serde_json::Value
fn ruby_value_to_json(ruby: &Ruby, value: Value) -> Result<JsonValue, Error> {
    if value.is_nil() {
        return Ok(JsonValue::Null);
    }

    let json_module: Value = ruby
        .class_object()
        .const_get("JSON")
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

    let json_string: String = json_module.funcall("generate", (value,))?;
    serde_json::from_str(&json_string).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to convert Ruby value to JSON: {err}"),
        )
    })
}

/// Convert serde_json::Value to Ruby Value
pub fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(ruby.qnil().as_value()),
        JsonValue::Bool(b) => Ok(if *b {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }),
        JsonValue::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(ruby.integer_from_i64(i).as_value())
            } else if let Some(f) = num.as_f64() {
                Ok(ruby.float_from_f64(f).as_value())
            } else {
                Ok(ruby.qnil().as_value())
            }
        }
        JsonValue::String(str_val) => Ok(ruby.str_new(str_val).as_value()),
        JsonValue::Array(items) => {
            let array = ruby.ary_new();
            for item in items {
                array.push(json_to_ruby(ruby, item)?)?;
            }
            Ok(array.as_value())
        }
        JsonValue::Object(map) => {
            let hash = ruby.hash_new();
            for (key, item) in map {
                hash.aset(ruby.str_new(key), json_to_ruby(ruby, item)?)?;
            }
            Ok(hash.as_value())
        }
    }
}

/// Helper to extract keyword arguments from Ruby options hash
pub fn extract_di_options(ruby: &Ruby, options: Value) -> Result<(Vec<String>, bool, bool), Error> {
    if options.is_nil() {
        return Ok((Vec::new(), false, true));
    }

    let hash = RHash::try_convert(options)?;

    // Extract depends_on
    let depends_on = if let Some(deps_value) = get_kw(ruby, hash, "depends_on") {
        if deps_value.is_nil() {
            Vec::new()
        } else {
            Vec::<String>::try_convert(deps_value)?
        }
    } else {
        Vec::new()
    };

    // Extract singleton (default false)
    let singleton = if let Some(singleton_value) = get_kw(ruby, hash, "singleton") {
        bool::try_convert(singleton_value).unwrap_or(false)
    } else {
        false
    };

    // Extract cacheable (default true)
    let cacheable = if let Some(cacheable_value) = get_kw(ruby, hash, "cacheable") {
        bool::try_convert(cacheable_value).unwrap_or(true)
    } else {
        true
    };

    Ok((depends_on, singleton, cacheable))
}

/// Get keyword argument from Ruby hash (tries both symbol and string keys)
fn get_kw(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    let sym = ruby.intern(name);
    hash.get(sym).or_else(|| hash.get(name))
}
