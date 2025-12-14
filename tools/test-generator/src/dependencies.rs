//! Dependency Injection support for test generator
//!
//! Parses DI configuration from fixtures and provides helpers for generating
//! dependency registration and injection code across all target languages.

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::Fixture;
use std::collections::{HashMap, HashSet};

/// Type of dependency
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyType {
    /// Static value (config, constants)
    Value,
    /// Synchronous factory function
    Factory,
    /// Asynchronous factory function
    AsyncFactory,
}

impl DependencyType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "value" => Some(DependencyType::Value),
            "factory" => Some(DependencyType::Factory),
            "async_factory" => Some(DependencyType::AsyncFactory),
            _ => None,
        }
    }
}

/// Dependency definition from fixture
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Dependency key/name
    pub key: String,
    /// Type of dependency
    pub dep_type: DependencyType,
    /// Static value (for Value type)
    pub value: Option<Value>,
    /// Value type hint
    pub value_type: Option<String>,
    /// Factory function name
    pub factory: Option<String>,
    /// Dependencies this depends on
    pub depends_on: Vec<String>,
    /// Singleton (shared across all requests)
    pub singleton: bool,
    /// Cacheable (per-request caching)
    pub cacheable: bool,
    /// Has cleanup handler
    pub cleanup: bool,
    /// Scope (app or route)
    pub scope: String,
    /// Python type annotation
    pub python_type: Option<String>,
}

impl Dependency {
    /// Parse dependency from JSON value
    pub fn from_json(key: String, value: &Value) -> Result<Self> {
        let dep_type_str = value
            .get("type")
            .and_then(|v| v.as_str())
            .context("Dependency missing 'type' field")?;

        let dep_type = DependencyType::from_str(dep_type_str)
            .with_context(|| format!("Invalid dependency type: {}", dep_type_str))?;

        Ok(Dependency {
            key,
            dep_type,
            value: value.get("value").cloned(),
            value_type: value.get("value_type").and_then(|v| v.as_str()).map(String::from),
            factory: value.get("factory").and_then(|v| v.as_str()).map(String::from),
            depends_on: value
                .get("depends_on")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
                .unwrap_or_default(),
            singleton: value.get("singleton").and_then(|v| v.as_bool()).unwrap_or(false),
            cacheable: value.get("cacheable").and_then(|v| v.as_bool()).unwrap_or(false),
            cleanup: value.get("cleanup").and_then(|v| v.as_bool()).unwrap_or(false),
            scope: value.get("scope").and_then(|v| v.as_str()).unwrap_or("app").to_string(),
            python_type: value.get("python_type").and_then(|v| v.as_str()).map(String::from),
        })
    }

    /// Check if this is a value dependency
    pub fn is_value(&self) -> bool {
        self.dep_type == DependencyType::Value
    }

    /// Check if this is a factory dependency
    pub fn is_factory(&self) -> bool {
        self.dep_type == DependencyType::Factory
    }

    /// Check if this is an async factory dependency
    pub fn is_async_factory(&self) -> bool {
        self.dep_type == DependencyType::AsyncFactory
    }
}

/// DI configuration extracted from fixture
#[derive(Debug, Clone)]
pub struct DependencyConfig {
    /// App-level dependencies
    pub dependencies: HashMap<String, Dependency>,
    /// Route-level dependency overrides
    pub route_overrides: HashMap<String, Dependency>,
    /// Dependencies required by handler
    pub handler_dependencies: Vec<String>,
    /// Injection strategy (name, type, destructure, keyword_args)
    pub injection_strategy: Option<String>,
    /// Expected dependency resolution order (batches)
    pub resolution_order: Option<Vec<Vec<String>>>,
}

impl DependencyConfig {
    /// Parse DI configuration from fixture
    ///
    /// Since the spikard_codegen Fixture struct doesn't have DI fields yet,
    /// we need to deserialize the fixture as a raw JSON Value to access DI fields
    pub fn from_fixture(fixture: &Fixture) -> Result<Option<Self>> {
        let fixture_json = serde_json::to_value(fixture)?;

        let handler = match fixture_json.get("handler") {
            Some(h) if h.is_object() => h,
            _ => return Ok(None),
        };

        let has_dependencies = handler.get("dependencies").is_some()
            || handler.get("handler_dependencies").is_some()
            || handler.get("route_overrides").is_some();

        if !has_dependencies {
            return Ok(None);
        }

        let mut dependencies = HashMap::new();
        if let Some(deps) = handler.get("dependencies").and_then(|v| v.as_object()) {
            for (key, value) in deps.iter() {
                let dep = Dependency::from_json(key.clone(), value)?;
                dependencies.insert(key.clone(), dep);
            }
        }

        let mut route_overrides = HashMap::new();
        if let Some(overrides) = handler.get("route_overrides").and_then(|v| v.as_object()) {
            for (key, value) in overrides.iter() {
                let dep = Dependency::from_json(key.clone(), value)?;
                route_overrides.insert(key.clone(), dep);
            }
        }

        let handler_dependencies = handler
            .get("handler_dependencies")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default();

        let injection_strategy = handler
            .get("injection_strategy")
            .and_then(|v| v.as_str())
            .map(String::from);

        let resolution_order = None;

        Ok(Some(DependencyConfig {
            dependencies,
            route_overrides,
            handler_dependencies,
            injection_strategy,
            resolution_order,
        }))
    }

    /// Check if fixture has any DI configuration
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty() || !self.route_overrides.is_empty()
    }

    /// Get all dependencies (app + route overrides)
    pub fn all_dependencies(&self) -> HashMap<String, &Dependency> {
        let mut all = HashMap::new();

        for (key, dep) in &self.dependencies {
            all.insert(key.clone(), dep);
        }

        for (key, dep) in &self.route_overrides {
            all.insert(key.clone(), dep);
        }

        all
    }

    /// Compute dependency resolution order using topological sort
    /// Returns batches of dependencies that can be resolved in parallel
    pub fn compute_resolution_order(&self) -> Result<Vec<Vec<String>>> {
        let all_deps = self.all_dependencies();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_keys: HashSet<String> = HashSet::new();

        for (key, dep) in &all_deps {
            all_keys.insert(key.clone());
            in_degree.entry(key.clone()).or_insert(0);
            graph.entry(key.clone()).or_insert_with(Vec::new);

            for dependency in &dep.depends_on {
                all_keys.insert(dependency.clone());
                graph
                    .entry(dependency.clone())
                    .or_insert_with(Vec::new)
                    .push(key.clone());
                *in_degree.entry(key.clone()).or_insert(0) += 1;
            }
        }

        let mut batches = Vec::new();
        let mut processed = HashSet::new();

        while processed.len() < all_keys.len() {
            let mut batch: Vec<String> = in_degree
                .iter()
                .filter(|&(key, degree)| *degree == 0 && !processed.contains(key))
                .map(|(key, _)| key.clone())
                .collect();

            if batch.is_empty() {
                let remaining: Vec<String> = all_keys.iter().filter(|k| !processed.contains(*k)).cloned().collect();
                anyhow::bail!("Circular dependency detected in: {:?}", remaining);
            }

            batch.sort();
            batches.push(batch.clone());

            for key in &batch {
                processed.insert(key.clone());
                if let Some(neighbors) = graph.get(key) {
                    for neighbor in neighbors {
                        if let Some(degree) = in_degree.get_mut(neighbor) {
                            *degree = degree.saturating_sub(1);
                        }
                    }
                }
            }
        }

        Ok(batches)
    }
}

/// Check if fixture requires multi-request testing (singleton caching)
pub fn requires_multi_request_test(di_config: &DependencyConfig) -> bool {
    di_config.dependencies.values().any(|dep| dep.singleton)
}

/// Check if fixture has cleanup dependencies
pub fn has_cleanup(di_config: &DependencyConfig) -> bool {
    di_config.all_dependencies().values().any(|dep| dep.cleanup)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort_simple() {
        let mut deps = HashMap::new();
        deps.insert(
            "config".to_string(),
            Dependency {
                key: "config".to_string(),
                dep_type: DependencyType::Value,
                value: None,
                value_type: None,
                factory: None,
                depends_on: vec![],
                singleton: false,
                cacheable: false,
                cleanup: false,
                scope: "app".to_string(),
                python_type: None,
            },
        );
        deps.insert(
            "db_pool".to_string(),
            Dependency {
                key: "db_pool".to_string(),
                dep_type: DependencyType::AsyncFactory,
                value: None,
                value_type: None,
                factory: Some("create_db_pool".to_string()),
                depends_on: vec!["config".to_string()],
                singleton: false,
                cacheable: true,
                cleanup: false,
                scope: "app".to_string(),
                python_type: None,
            },
        );

        let config = DependencyConfig {
            dependencies: deps,
            route_overrides: HashMap::new(),
            handler_dependencies: vec!["db_pool".to_string()],
            injection_strategy: None,
            resolution_order: None,
        };

        let order = config.compute_resolution_order().unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0], vec!["config"]);
        assert_eq!(order[1], vec!["db_pool"]);
    }

    #[test]
    fn test_topological_sort_parallel() {
        let mut deps = HashMap::new();
        deps.insert(
            "config".to_string(),
            Dependency {
                key: "config".to_string(),
                dep_type: DependencyType::Value,
                value: None,
                value_type: None,
                factory: None,
                depends_on: vec![],
                singleton: false,
                cacheable: false,
                cleanup: false,
                scope: "app".to_string(),
                python_type: None,
            },
        );
        deps.insert(
            "db_pool".to_string(),
            Dependency {
                key: "db_pool".to_string(),
                dep_type: DependencyType::AsyncFactory,
                value: None,
                value_type: None,
                factory: Some("create_db_pool".to_string()),
                depends_on: vec!["config".to_string()],
                singleton: false,
                cacheable: true,
                cleanup: false,
                scope: "app".to_string(),
                python_type: None,
            },
        );
        deps.insert(
            "cache".to_string(),
            Dependency {
                key: "cache".to_string(),
                dep_type: DependencyType::AsyncFactory,
                value: None,
                value_type: None,
                factory: Some("create_cache".to_string()),
                depends_on: vec!["config".to_string()],
                singleton: false,
                cacheable: true,
                cleanup: false,
                scope: "app".to_string(),
                python_type: None,
            },
        );

        let config = DependencyConfig {
            dependencies: deps,
            route_overrides: HashMap::new(),
            handler_dependencies: vec![],
            injection_strategy: None,
            resolution_order: None,
        };

        let order = config.compute_resolution_order().unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0], vec!["config"]);
        assert_eq!(order[1].len(), 2);
        assert!(order[1].contains(&"cache".to_string()));
        assert!(order[1].contains(&"db_pool".to_string()));
    }
}
