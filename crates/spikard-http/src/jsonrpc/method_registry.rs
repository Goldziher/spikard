//! JSON-RPC method registry for handler registration and lookup
//!
//! This module provides thread-safe registration and retrieval of JSON-RPC methods
//! with their associated handlers and metadata.

use crate::handler_trait::Handler;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Example for a JSON-RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodExample {
    /// Name of the example
    pub name: String,
    /// Optional description of the example
    pub description: Option<String>,
    /// Example parameters
    pub params: Value,
    /// Example result
    pub result: Value,
}

/// Metadata for a JSON-RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodMetadata {
    /// Method name
    pub name: String,
    /// Optional description of what the method does
    pub description: Option<String>,
    /// JSON Schema for method parameters
    pub params_schema: Option<Value>,
    /// JSON Schema for method result
    pub result_schema: Option<Value>,
    /// JSON Schema for method errors
    pub error_schema: Option<Value>,
    /// Examples for this method
    pub examples: Vec<MethodExample>,
    /// Whether this method is deprecated
    pub deprecated: bool,
    /// Tags for organizing/categorizing methods
    pub tags: Vec<String>,
}

impl MethodMetadata {
    /// Create a new MethodMetadata with minimal required fields
    ///
    /// # Arguments
    ///
    /// * `name` - The method name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            params_schema: None,
            result_schema: None,
            error_schema: None,
            examples: Vec::new(),
            deprecated: false,
            tags: Vec::new(),
        }
    }

    /// Set the description for this method
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the parameters schema for this method
    pub fn with_params_schema(mut self, schema: Value) -> Self {
        self.params_schema = Some(schema);
        self
    }

    /// Set the result schema for this method
    pub fn with_result_schema(mut self, schema: Value) -> Self {
        self.result_schema = Some(schema);
        self
    }

    /// Set the error schema for this method
    pub fn with_error_schema(mut self, schema: Value) -> Self {
        self.error_schema = Some(schema);
        self
    }

    /// Add an example to this method's examples
    pub fn with_example(mut self, example: MethodExample) -> Self {
        self.examples.push(example);
        self
    }

    /// Mark this method as deprecated
    pub fn mark_deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }

    /// Add a tag to this method
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// Thread-safe registry for JSON-RPC methods
///
/// Stores handlers along with their metadata. The registry uses `Arc<RwLock>` for
/// thread-safe concurrent access with low contention for reads.
///
/// # Example
///
/// ```ignore
/// use spikard_http::jsonrpc::method_registry::{JsonRpcMethodRegistry, MethodMetadata};
/// use std::sync::Arc;
///
/// let registry = JsonRpcMethodRegistry::new();
///
/// // Register a method (handler implementation omitted for brevity)
/// registry.register(
///     "add",
///     Arc::new(add_handler),
///     MethodMetadata::new("add").with_description("Add two numbers"),
/// );
///
/// // Lookup a method
/// if let Some(handler) = registry.get("add") {
///     // Use handler
/// }
/// ```
pub struct JsonRpcMethodRegistry {
    /// Internal storage: method name -> (handler, metadata)
    methods: Arc<RwLock<HashMap<String, (Arc<dyn Handler>, MethodMetadata)>>>,
}

impl JsonRpcMethodRegistry {
    /// Create a new empty method registry
    pub fn new() -> Self {
        Self {
            methods: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a method with its handler and metadata
    ///
    /// If a method with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name (e.g., "add", "multiply")
    /// * `handler` - The handler that processes this method
    /// * `metadata` - Metadata describing the method
    ///
    /// # Panics
    ///
    /// This method will panic if the write lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn register(&self, name: impl Into<String>, handler: Arc<dyn Handler>, metadata: MethodMetadata) {
        let name = name.into();
        let mut methods = self
            .methods
            .write()
            .expect("Failed to acquire write lock on methods registry");
        methods.insert(name, (handler, metadata));
    }

    /// Get a handler by method name
    ///
    /// Returns `None` if the method is not registered.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name to look up
    ///
    /// # Returns
    ///
    /// An `Option<Arc<dyn Handler>>` containing the handler if found, or `None` otherwise.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Handler>> {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods.get(name).map(|(handler, _)| Arc::clone(handler))
    }

    /// Get metadata for a method by name
    ///
    /// Returns `None` if the method is not registered.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name to look up
    ///
    /// # Returns
    ///
    /// An `Option<MethodMetadata>` containing the metadata if found, or `None` otherwise.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn get_metadata(&self, name: &str) -> Option<MethodMetadata> {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods.get(name).map(|(_, metadata)| metadata.clone())
    }

    /// Get both handler and metadata for a method
    ///
    /// Returns `None` if the method is not registered.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name to look up
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple of `(Arc<dyn Handler>, MethodMetadata)` if found.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn get_with_metadata(&self, name: &str) -> Option<(Arc<dyn Handler>, MethodMetadata)> {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods
            .get(name)
            .map(|(handler, metadata)| (Arc::clone(handler), metadata.clone()))
    }

    /// List all registered method names
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all registered method names, sorted lexicographically.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn list_methods(&self) -> Vec<String> {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        let mut names: Vec<String> = methods.keys().cloned().collect();
        names.sort();
        names
    }

    /// Check if a method is registered
    ///
    /// # Arguments
    ///
    /// * `name` - The method name to check
    ///
    /// # Returns
    ///
    /// `true` if the method is registered, `false` otherwise.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn contains(&self, name: &str) -> bool {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods.contains_key(name)
    }

    /// Get the number of registered methods
    ///
    /// # Returns
    ///
    /// The count of registered methods.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn len(&self) -> usize {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods.len()
    }

    /// Check if the registry is empty
    ///
    /// # Returns
    ///
    /// `true` if no methods are registered, `false` otherwise.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn is_empty(&self) -> bool {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods.is_empty()
    }

    /// Remove a method from the registry
    ///
    /// Returns `true` if the method was removed, `false` if it didn't exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name to remove
    ///
    /// # Panics
    ///
    /// This method will panic if the write lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn remove(&self, name: &str) -> bool {
        let mut methods = self
            .methods
            .write()
            .expect("Failed to acquire write lock on methods registry");
        methods.remove(name).is_some()
    }

    /// Clear all methods from the registry
    ///
    /// # Panics
    ///
    /// This method will panic if the write lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn clear(&self) {
        let mut methods = self
            .methods
            .write()
            .expect("Failed to acquire write lock on methods registry");
        methods.clear();
    }

    /// Get all methods with their metadata
    ///
    /// # Returns
    ///
    /// A `Vec` containing tuples of method name, handler, and metadata.
    ///
    /// # Panics
    ///
    /// This method will panic if the read lock cannot be acquired. This is
    /// extremely unlikely in normal operation.
    pub fn list_all(&self) -> Vec<(String, Arc<dyn Handler>, MethodMetadata)> {
        let methods = self
            .methods
            .read()
            .expect("Failed to acquire read lock on methods registry");
        methods
            .iter()
            .map(|(name, (handler, metadata))| (name.clone(), Arc::clone(handler), metadata.clone()))
            .collect()
    }
}

impl Default for JsonRpcMethodRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for JsonRpcMethodRegistry {
    fn clone(&self) -> Self {
        Self {
            methods: Arc::clone(&self.methods),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler_trait::{HandlerResult, RequestData};
    use axum::body::Body;
    use axum::http::Request;

    /// Mock handler for testing
    struct MockHandler;

    impl Handler for MockHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async { Err((axum::http::StatusCode::OK, "mock".to_string())) })
        }
    }

    fn create_test_registry() -> JsonRpcMethodRegistry {
        JsonRpcMethodRegistry::new()
    }

    fn create_mock_handler() -> Arc<dyn Handler> {
        Arc::new(MockHandler)
    }

    #[test]
    fn test_new_registry_is_empty() {
        let registry = create_test_registry();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(registry.list_methods().is_empty());
    }

    #[test]
    fn test_register_and_get_method() {
        let registry = create_test_registry();
        let handler = create_mock_handler();
        let metadata = MethodMetadata::new("test_method");

        registry.register("test_method", handler.clone(), metadata.clone());

        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("test_method"));
        assert!(registry.get("test_method").is_some());
        assert_eq!(registry.get_metadata("test_method").unwrap().name, "test_method");
    }

    #[test]
    fn test_get_nonexistent_method() {
        let registry = create_test_registry();
        assert!(registry.get("nonexistent").is_none());
        assert!(registry.get_metadata("nonexistent").is_none());
    }

    #[test]
    fn test_list_methods_returns_sorted() {
        let registry = create_test_registry();
        let handler = create_mock_handler();

        registry.register("zebra", handler.clone(), MethodMetadata::new("zebra"));
        registry.register("apple", handler.clone(), MethodMetadata::new("apple"));
        registry.register("banana", handler.clone(), MethodMetadata::new("banana"));

        let methods = registry.list_methods();
        assert_eq!(methods, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn test_register_overwrites_existing() {
        let registry = create_test_registry();
        let handler1 = create_mock_handler();
        let handler2 = create_mock_handler();

        registry.register("method", handler1, MethodMetadata::new("method"));
        assert_eq!(registry.len(), 1);

        registry.register("method", handler2, MethodMetadata::new("method"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_remove_method() {
        let registry = create_test_registry();
        let handler = create_mock_handler();

        registry.register("method", handler, MethodMetadata::new("method"));
        assert_eq!(registry.len(), 1);

        let removed = registry.remove("method");
        assert!(removed);
        assert_eq!(registry.len(), 0);
        assert!(registry.get("method").is_none());
    }

    #[test]
    fn test_remove_nonexistent_method() {
        let registry = create_test_registry();
        let removed = registry.remove("nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_clear_registry() {
        let registry = create_test_registry();
        let handler = create_mock_handler();

        registry.register("method1", handler.clone(), MethodMetadata::new("method1"));
        registry.register("method2", handler.clone(), MethodMetadata::new("method2"));
        registry.register("method3", handler.clone(), MethodMetadata::new("method3"));

        assert_eq!(registry.len(), 3);
        registry.clear();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_get_with_metadata() {
        let registry = create_test_registry();
        let handler = create_mock_handler();
        let metadata = MethodMetadata::new("method").with_description("Test method");

        registry.register("method", handler.clone(), metadata);

        let result = registry.get_with_metadata("method");
        assert!(result.is_some());

        let (_retrieved_handler, retrieved_metadata) = result.unwrap();
        assert_eq!(retrieved_metadata.name, "method");
        assert_eq!(retrieved_metadata.description, Some("Test method".to_string()));
    }

    #[test]
    fn test_list_all() {
        let registry = create_test_registry();
        let handler = create_mock_handler();

        registry.register("add", handler.clone(), MethodMetadata::new("add"));
        registry.register("subtract", handler.clone(), MethodMetadata::new("subtract"));

        let all = registry.list_all();
        assert_eq!(all.len(), 2);

        let names: Vec<String> = all.iter().map(|(name, _, _)| name.clone()).collect();
        assert!(names.contains(&"add".to_string()));
        assert!(names.contains(&"subtract".to_string()));
    }

    #[test]
    fn test_clone_shares_registry() {
        let registry1 = create_test_registry();
        let handler = create_mock_handler();

        registry1.register("method", handler, MethodMetadata::new("method"));

        let registry2 = registry1.clone();
        assert_eq!(registry2.len(), 1);
        assert!(registry2.contains("method"));
    }

    #[test]
    fn test_metadata_builder_pattern() {
        let metadata = MethodMetadata::new("test")
            .with_description("Test description")
            .with_tag("math")
            .with_tag("utility")
            .mark_deprecated();

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.description, Some("Test description".to_string()));
        assert_eq!(metadata.tags, vec!["math", "utility"]);
        assert!(metadata.deprecated);
    }

    #[test]
    fn test_metadata_with_schemas() {
        let params_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "x": { "type": "number" },
                "y": { "type": "number" }
            }
        });

        let result_schema = serde_json::json!({
            "type": "number"
        });

        let metadata = MethodMetadata::new("add")
            .with_params_schema(params_schema.clone())
            .with_result_schema(result_schema.clone());

        assert_eq!(metadata.params_schema, Some(params_schema));
        assert_eq!(metadata.result_schema, Some(result_schema));
    }

    #[test]
    fn test_metadata_with_examples() {
        let example = MethodExample {
            name: "example1".to_string(),
            description: Some("Test example".to_string()),
            params: serde_json::json!({"x": 1, "y": 2}),
            result: serde_json::json!(3),
        };

        let metadata = MethodMetadata::new("add").with_example(example.clone());

        assert_eq!(metadata.examples.len(), 1);
        assert_eq!(metadata.examples[0].name, "example1");
        assert_eq!(metadata.examples[0].description, Some("Test example".to_string()));
    }
}
