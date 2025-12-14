use openapiv3::OpenAPI;
use spikard_cli::codegen::{PhpDtoStyle, PhpGenerator};

#[test]
fn php_generator_emits_models_and_controller_skeletons() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "User": {
                    "type": "object",
                    "description": "A user record",
                    "properties": {
                        "id": { "type": "string" },
                        "age": { "type": "integer" },
                        "tags": { "type": "array", "items": { "type": "string" } }
                    },
                    "required": ["id"]
                }
            }
        },
        "paths": {
            "/users": {
                "post": {
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/User" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "created",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/User" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("declare(strict_types=1);"));
    assert!(output.contains("namespace SpikardGenerated;"));
    assert!(output.contains("readonly class User"));
    assert!(output.contains("public string $id"));
    assert!(output.contains("public ?int $age"));
    assert!(output.contains("array"));
    assert!(output.contains("class UsersController") || output.contains("Controller"));
}
