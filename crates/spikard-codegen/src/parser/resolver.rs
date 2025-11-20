//! Schema reference resolution ($ref, includes)

use crate::error::{CodegenError, Result};
use serde_json::Value;
use std::collections::HashMap;

/// Resolve $ref references in schemas
pub fn resolve_refs(schema: &Value, definitions: &HashMap<String, Value>) -> Result<Value> {
    match schema {
        Value::Object(obj) => {
            if let Some(Value::String(ref_path)) = obj.get("$ref") {
                return resolve_ref_path(ref_path, definitions);
            }

            let mut resolved = serde_json::Map::new();
            for (key, value) in obj {
                resolved.insert(key.clone(), resolve_refs(value, definitions)?);
            }
            Ok(Value::Object(resolved))
        }
        Value::Array(arr) => {
            let resolved: Result<Vec<Value>> = arr.iter().map(|item| resolve_refs(item, definitions)).collect();
            Ok(Value::Array(resolved?))
        }
        _ => Ok(schema.clone()),
    }
}

fn resolve_ref_path(ref_path: &str, definitions: &HashMap<String, Value>) -> Result<Value> {
    if let Some(schema_name) = ref_path.strip_prefix("#/schemas/") {
        definitions
            .get(schema_name)
            .map(|schema| resolve_refs(schema, definitions))
            .unwrap_or_else(|| Err(CodegenError::SchemaRefNotFound(ref_path.to_string())))
    } else {
        Err(CodegenError::SchemaRefNotFound(ref_path.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_resolve_simple_ref() {
        let mut definitions = HashMap::new();
        definitions.insert(
            "User".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                }
            }),
        );

        let schema = json!({"$ref": "#/schemas/User"});
        let resolved = resolve_refs(&schema, &definitions).unwrap();

        assert_eq!(
            resolved,
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                }
            })
        );
    }

    #[test]
    fn test_resolve_nested_refs() {
        let mut definitions = HashMap::new();
        definitions.insert("Name".to_string(), json!({"type": "string"}));
        definitions.insert(
            "User".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "name": {"$ref": "#/schemas/Name"}
                }
            }),
        );

        let schema = json!({"$ref": "#/schemas/User"});
        let resolved = resolve_refs(&schema, &definitions).unwrap();

        assert_eq!(
            resolved,
            json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"}
                }
            })
        );
    }
}
