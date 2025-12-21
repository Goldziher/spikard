//! Dependency injection container building from Ruby objects.
//!
//! This module handles constructing DependencyContainer instances from Ruby
//! hashes and factories.

use magnus::prelude::*;
use magnus::{Error, RHash, Ruby, TryConvert, Value, r_hash::ForEach};
use spikard_core::di::DependencyContainer;
use std::sync::Arc;

/// Build a DependencyContainer from Ruby dependency definitions
pub fn build_dependency_container(ruby: &Ruby, dependencies: Value) -> Result<DependencyContainer, Error> {
    if dependencies.is_nil() {
        return Ok(DependencyContainer::new());
    }

    let mut container = DependencyContainer::new();
    let deps_hash = RHash::try_convert(dependencies)?;

    deps_hash.foreach(|key: String, value: Value| -> Result<ForEach, Error> {
        if let Ok(dep_hash) = RHash::try_convert(value) {
            let dep_type: Option<String> = get_kw(ruby, dep_hash, "type").and_then(|v| {
                if let Ok(sym) = magnus::Symbol::try_convert(v) {
                    Some(sym.name().ok()?.to_string())
                } else {
                    String::try_convert(v).ok()
                }
            });

            match dep_type.as_deref() {
                Some("factory") => {
                    let factory = get_kw(ruby, dep_hash, "factory")
                        .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Factory missing 'factory' key"))?;

                    let depends_on: Vec<String> = get_kw(ruby, dep_hash, "depends_on")
                        .and_then(|v| Vec::<String>::try_convert(v).ok())
                        .unwrap_or_default();

                    let singleton: bool = get_kw(ruby, dep_hash, "singleton")
                        .and_then(|v| bool::try_convert(v).ok())
                        .unwrap_or(false);

                    let cacheable: bool = get_kw(ruby, dep_hash, "cacheable")
                        .and_then(|v| bool::try_convert(v).ok())
                        .unwrap_or(true);

                    let factory_dep =
                        crate::di::RubyFactoryDependency::new(key.clone(), factory, depends_on, singleton, cacheable);

                    container.register(key.clone(), Arc::new(factory_dep)).map_err(|e| {
                        Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to register factory '{}': {}", key, e),
                        )
                    })?;
                }
                Some("value") => {
                    let value_data = get_kw(ruby, dep_hash, "value").ok_or_else(|| {
                        Error::new(ruby.exception_runtime_error(), "Value dependency missing 'value' key")
                    })?;

                    let value_dep = crate::di::RubyValueDependency::new(key.clone(), value_data);

                    container.register(key.clone(), Arc::new(value_dep)).map_err(|e| {
                        Error::new(
                            ruby.exception_runtime_error(),
                            format!("Failed to register value '{}': {}", key, e),
                        )
                    })?;
                }
                _ => {
                    return Err(Error::new(
                        ruby.exception_runtime_error(),
                        format!("Invalid dependency type for '{}'", key),
                    ));
                }
            }
        } else {
            let value_dep = crate::di::RubyValueDependency::new(key.clone(), value);
            container.register(key.clone(), Arc::new(value_dep)).map_err(|e| {
                Error::new(
                    ruby.exception_runtime_error(),
                    format!("Failed to register value '{}': {}", key, e),
                )
            })?;
        }

        Ok(ForEach::Continue)
    })?;

    Ok(container)
}

/// Get a keyword argument from a Ruby hash (returns None if not present or nil)
fn get_kw(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    match hash.get(ruby.to_symbol(name)) {
        Some(v) if !v.is_nil() => Some(v),
        _ => None,
    }
}
