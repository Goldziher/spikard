use openapiv3::{OpenAPI, ReferenceOr, Schema};
use std::collections::HashMap;

/// Simple registry that lets generators resolve `$ref` identifiers to concrete schemas.
#[derive(Debug, Default, Clone)]
pub struct SchemaRegistry {
    schemas: HashMap<String, Schema>,
}

impl SchemaRegistry {
    /// Build a registry from the OpenAPI spec's component schemas.
    pub fn from_spec(spec: &OpenAPI) -> Self {
        let mut registry = SchemaRegistry::default();
        if let Some(components) = &spec.components {
            for (name, schema_ref) in &components.schemas {
                if let ReferenceOr::Item(schema) = schema_ref {
                    registry.schemas.insert(name.clone(), schema.clone());
                }
            }
        }
        registry
    }

    /// Lookup the schema identified by a JSON pointer reference (e.g. `#/components/schemas/User`).
    pub fn resolve_reference(&self, reference: &str) -> Option<&Schema> {
        reference.rsplit('/').next().and_then(|name| self.schemas.get(name))
    }

    /// Resolve either inline schemas or `$ref` entries to a `Schema`.
    pub fn resolve<'a>(&'a self, schema_ref: &'a ReferenceOr<Schema>) -> Option<&'a Schema> {
        match schema_ref {
            ReferenceOr::Item(schema) => Some(schema),
            ReferenceOr::Reference { reference } => self.resolve_reference(reference),
        }
    }
}
