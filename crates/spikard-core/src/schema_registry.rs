//! Schema registry for deduplication and `OpenAPI` generation
//!
//! This module provides a global registry that compiles JSON schemas once at application
//! startup and reuses them across all routes. This enables:
//! - Schema deduplication (same schema used by multiple routes)
//! - `OpenAPI` spec generation (access to all schemas)
//! - Memory efficiency (one compiled validator per unique schema)

use crate::validation::SchemaValidator;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Global schema registry for compiled validators
///
/// Thread-safe registry that ensures each unique schema is compiled exactly once.
/// Uses `RwLock` for concurrent read access with occasional writes during startup.
pub struct SchemaRegistry {
    /// Map from schema JSON string to compiled validator
    schemas: RwLock<HashMap<String, Arc<SchemaValidator>>>,
}

impl SchemaRegistry {
    /// Create a new empty schema registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            schemas: RwLock::new(HashMap::new()),
        }
    }

    /// Get or compile a schema, returning `Arc` to the compiled validator
    ///
    /// This method is thread-safe and uses a double-check pattern:
    /// 1. Fast path: Read lock to check if schema exists
    /// 2. Slow path: Write lock to compile and store new schema
    ///
    /// # Arguments
    /// * `schema` - The JSON schema to compile
    ///
    /// # Returns
    /// `Arc`-wrapped compiled validator that can be cheaply cloned
    ///
    /// # Errors
    /// Returns an error if schema serialization or compilation fails.
    ///
    /// # Panics
    /// Panics if the read or write lock is poisoned.
    pub fn get_or_compile(&self, schema: &Value) -> Result<Arc<SchemaValidator>, String> {
        let key = serde_json::to_string(schema).map_err(|e| format!("Failed to serialize schema: {e}"))?;

        {
            let schemas = self.schemas.read().unwrap();
            if let Some(validator) = schemas.get(&key) {
                return Ok(Arc::clone(validator));
            }
        }

        let validator = Arc::new(SchemaValidator::new(schema.clone())?);

        {
            let mut schemas = self.schemas.write().unwrap();
            if let Some(existing) = schemas.get(&key) {
                return Ok(Arc::clone(existing));
            }
            schemas.insert(key, Arc::clone(&validator));
        }

        Ok(validator)
    }

    /// Get all registered schemas (for `OpenAPI` generation)
    ///
    /// Returns a snapshot of all compiled validators.
    /// Useful for generating `OpenAPI` specifications from runtime schema information.
    ///
    /// # Panics
    /// Panics if the read lock is poisoned.
    #[must_use]
    pub fn all_schemas(&self) -> Vec<Arc<SchemaValidator>> {
        let schemas = self.schemas.read().unwrap();
        schemas.values().cloned().collect()
    }

    /// Get the number of unique schemas registered
    ///
    /// Useful for diagnostics and understanding schema deduplication effectiveness.
    ///
    /// # Panics
    /// Panics if the read lock is poisoned.
    #[must_use]
    pub fn schema_count(&self) -> usize {
        let schemas = self.schemas.read().unwrap();
        schemas.len()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_deduplication() {
        let registry = SchemaRegistry::new();

        let schema1 = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let schema2 = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let validator1 = registry.get_or_compile(&schema1).unwrap();
        let validator2 = registry.get_or_compile(&schema2).unwrap();

        assert!(Arc::ptr_eq(&validator1, &validator2));

        assert_eq!(registry.schema_count(), 1);
    }

    #[test]
    fn test_different_schemas() {
        let registry = SchemaRegistry::new();

        let schema1 = json!({
            "type": "string"
        });

        let schema2 = json!({
            "type": "integer"
        });

        let validator1 = registry.get_or_compile(&schema1).unwrap();
        let validator2 = registry.get_or_compile(&schema2).unwrap();

        assert!(!Arc::ptr_eq(&validator1, &validator2));

        assert_eq!(registry.schema_count(), 2);
    }

    #[test]
    fn test_all_schemas() {
        let registry = SchemaRegistry::new();

        let schema1 = json!({"type": "string"});
        let schema2 = json!({"type": "integer"});

        registry.get_or_compile(&schema1).unwrap();
        registry.get_or_compile(&schema2).unwrap();

        let all = registry.all_schemas();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc as StdArc;
        use std::thread;

        let registry = StdArc::new(SchemaRegistry::new());
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "integer"}
            }
        });

        let validators: Vec<_> = (0..10)
            .map(|_| {
                let registry = StdArc::clone(&registry);
                let schema = schema.clone();
                thread::spawn(move || registry.get_or_compile(&schema).unwrap())
            })
            .map(|h| h.join().unwrap())
            .collect();

        for i in 1..validators.len() {
            assert!(Arc::ptr_eq(&validators[0], &validators[i]));
        }

        assert_eq!(registry.schema_count(), 1);
    }
}
