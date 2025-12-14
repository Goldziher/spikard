use anyhow::Result;
use openapiv3::{OpenAPI, ReferenceOr};
use spikard_cli::codegen::{OpenApiGenerator, SchemaRegistry};

struct DummyGenerator {
    spec: OpenAPI,
    registry: SchemaRegistry,
}

impl DummyGenerator {
    fn new(spec: OpenAPI) -> Self {
        Self {
            registry: SchemaRegistry::from_spec(&spec),
            spec,
        }
    }
}

impl OpenApiGenerator for DummyGenerator {
    fn spec(&self) -> &OpenAPI {
        &self.spec
    }

    fn registry(&self) -> &SchemaRegistry {
        &self.registry
    }

    fn generate_header(&self) -> String {
        "HEADER\n".to_string()
    }

    fn generate_models(&self) -> Result<String> {
        let mut names = Vec::new();
        self.iter_schemas(|name, _schema| {
            names.push(name.to_string());
            Ok(())
        })?;
        Ok(format!("MODELS:{}\n", names.join(",")))
    }

    fn generate_routes(&self) -> Result<String> {
        let mut ops = Vec::new();
        self.iter_paths(|path, method, operation| {
            ops.push(format!(
                "{}:{}:{}",
                method,
                path,
                self.generate_operation_id(path, method, operation)
            ));
            Ok(())
        })?;
        Ok(format!("ROUTES:{}\n", ops.join(",")))
    }

    fn default_response_type(&self) -> String {
        "unknown".to_string()
    }
}

#[test]
fn openapi_generator_trait_iterates_schemas_and_paths() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test", "version": "1.0.0" },
        "components": {
            "schemas": {
                "User": { "type": "object", "properties": { "id": { "type": "string" } } }
            }
        },
        "paths": {
            "/users": {
                "get": { "operationId": "listUsers", "responses": { "200": { "description": "ok" } } }
            }
        }
    }))
    .expect("spec");

    assert!(matches!(
        spec.components.as_ref().unwrap().schemas["User"],
        ReferenceOr::Item(_)
    ));

    let generator = DummyGenerator::new(spec);
    let output = generator.generate().expect("generate");
    assert!(output.contains("HEADER"));
    assert!(output.contains("MODELS:User"));
    assert!(output.contains("get:/users:list_users"));
}
