//! Base trait for OpenAPI code generators
//!
//! Provides a language-neutral abstraction for code generation from OpenAPI specs,
//! eliminating duplication across Python, TypeScript, Ruby, and PHP generators.

use crate::codegen::SchemaRegistry;
use anyhow::Result;
use openapiv3::{OpenAPI, Operation, ReferenceOr, Schema};

/// Base trait for OpenAPI code generators to eliminate duplication across languages.
///
/// Implementors should override language-specific methods while leveraging shared
/// default implementations for common patterns.
pub trait OpenApiGenerator {
    /// Get the OpenAPI specification
    fn spec(&self) -> &OpenAPI;

    /// Get the schema registry for reference resolution
    fn registry(&self) -> &SchemaRegistry;

    /// Generate the file header (imports, module declaration, etc.)
    fn generate_header(&self) -> String;

    /// Generate data models/DTOs from OpenAPI components
    fn generate_models(&self) -> Result<String>;

    /// Generate route handlers from OpenAPI paths
    fn generate_routes(&self) -> Result<String>;

    /// Generate file footer (bootstrap, exports, etc.)
    fn generate_footer(&self) -> String {
        String::new()
    }

    /// Orchestrate the full code generation pipeline
    fn generate(&self) -> Result<String> {
        let mut output = String::new();

        output.push_str(&self.generate_header());
        output.push_str(&self.generate_models()?);
        output.push_str(&self.generate_routes()?);

        let footer = self.generate_footer();
        if !footer.is_empty() {
            output.push_str(&footer);
        }

        Ok(output)
    }

    /// Iterate over all paths in the spec and apply a function to each operation
    fn iter_paths<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&str, &str, &Operation) -> Result<()>,
    {
        for (path, path_item_ref) in &self.spec().paths.paths {
            let path_item = match path_item_ref {
                ReferenceOr::Item(item) => item,
                ReferenceOr::Reference { .. } => continue,
            };

            if let Some(op) = &path_item.get {
                f(path, "get", op)?;
            }
            if let Some(op) = &path_item.post {
                f(path, "post", op)?;
            }
            if let Some(op) = &path_item.put {
                f(path, "put", op)?;
            }
            if let Some(op) = &path_item.delete {
                f(path, "delete", op)?;
            }
            if let Some(op) = &path_item.patch {
                f(path, "patch", op)?;
            }
        }
        Ok(())
    }

    /// Iterate over all component schemas and apply a function to each
    fn iter_schemas<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&str, &Schema) -> Result<()>,
    {
        if let Some(components) = &self.spec().components {
            for (name, schema_ref) in &components.schemas {
                match schema_ref {
                    ReferenceOr::Item(schema) => {
                        f(name, schema)?;
                    }
                    ReferenceOr::Reference { .. } => continue,
                }
            }
        }
        Ok(())
    }

    /// Extract request body type from operation (looks for application/json)
    fn extract_request_body_type(&self, operation: &Operation) -> Option<String> {
        operation.request_body.as_ref().and_then(|body_ref| match body_ref {
            ReferenceOr::Item(request_body) => request_body.content.get("application/json").and_then(|media_type| {
                media_type
                    .schema
                    .as_ref()
                    .map(|schema_ref| self.extract_type_from_schema_ref(schema_ref))
            }),
            ReferenceOr::Reference { reference } => {
                let ref_name = reference.split('/').next_back().unwrap();
                Some(self.format_type_name(ref_name))
            }
        })
    }

    /// Extract response type from operation (looks for 200/201 responses)
    fn extract_response_type(&self, operation: &Operation) -> String {
        use openapiv3::StatusCode;

        let response = operation
            .responses
            .responses
            .get(&StatusCode::Code(200))
            .or_else(|| operation.responses.responses.get(&StatusCode::Code(201)))
            .or_else(|| operation.responses.responses.get(&StatusCode::Range(2)));

        if let Some(response_ref) = response {
            match response_ref {
                ReferenceOr::Item(response) => {
                    if let Some(content) = response.content.get("application/json")
                        && let Some(schema_ref) = &content.schema
                    {
                        return self.extract_type_from_schema_ref(schema_ref);
                    }
                }
                ReferenceOr::Reference { reference } => {
                    let ref_name = reference.split('/').next_back().unwrap();
                    return self.format_type_name(ref_name);
                }
            }
        }

        self.default_response_type()
    }

    /// Extract type name from a schema reference or inline schema
    fn extract_type_from_schema_ref(&self, schema_ref: &ReferenceOr<Schema>) -> String {
        match schema_ref {
            ReferenceOr::Reference { reference } => {
                let ref_name = reference.split('/').next_back().unwrap();
                self.format_type_name(ref_name)
            }
            ReferenceOr::Item(_schema) => self.default_response_type(),
        }
    }

    /// Format a type name according to language conventions (PascalCase by default)
    fn format_type_name(&self, name: &str) -> String {
        heck::ToPascalCase::to_pascal_case(name)
    }

    /// Return the language's default response type (e.g., "dict[str, Any]", "Record<string, unknown>")
    fn default_response_type(&self) -> String {
        "unknown".to_string()
    }

    /// Generate operation ID (function/method name) from operation and path
    fn generate_operation_id(&self, path: &str, method: &str, operation: &Operation) -> String {
        operation
            .operation_id
            .as_ref()
            .map(|id| heck::ToSnakeCase::to_snake_case(id.as_str()))
            .unwrap_or_else(|| {
                format!(
                    "{}_{}",
                    method,
                    path.replace('/', "_").replace(['{', '}'], "").trim_matches('_')
                )
            })
    }
}
