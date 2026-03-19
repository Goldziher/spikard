use openapiv3::OpenAPI;
use spikard_cli::codegen::{
    DtoConfig, PhpDtoStyle, PhpGenerator, TargetLanguage, generate_from_openapi, quality::QualityValidator,
};
use std::fs;
use std::path::Path;

// Basic model generation tests

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

#[test]
fn php_generator_has_valid_php_header() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": "My API", "version": "2.0.0" },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.starts_with("<?php"));
    assert!(output.contains("declare(strict_types=1);"));
    assert!(output.contains("namespace SpikardGenerated;"));
    assert!(output.contains("My API"));
    assert!(output.contains("3.1.0"));
}

#[test]
fn php_generator_handles_multiple_properties_in_model() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Product": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "name": { "type": "string" },
                        "price": { "type": "number" },
                        "inStock": { "type": "boolean" },
                        "quantity": { "type": "integer" }
                    },
                    "required": ["id", "name", "price"]
                }
            }
        },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("readonly class Product"));
    assert!(output.contains("public string $id,"));
    assert!(output.contains("public string $name,"));
    assert!(output.contains("public float $price,"));
    assert!(output.contains("public ?bool $inStock = null"));
    assert!(output.contains("public ?int $quantity = null"));
}

#[test]
fn php_generator_handles_nullable_properties() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Post": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "description": { "type": "string" },
                        "author": { "type": "string" }
                    },
                    "required": ["title"]
                }
            }
        },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("public string $title,"));
    assert!(output.contains("public ?string $description = null"));
    assert!(output.contains("public ?string $author = null"));
}

#[test]
fn php_generator_handles_empty_models() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Empty": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("readonly class Empty"));
    assert!(output.contains("// Empty schema"));
}

// Controller and route generation tests

#[test]
fn php_generator_creates_controllers_from_paths() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "paths": {
            "/products": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": { "application/json": { "schema": { "type": "array" } } }
                        }
                    }
                }
            },
            "/products/{id}": {
                "get": {
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": { "application/json": { "schema": { "type": "object" } } }
                        }
                    }
                }
            }
        }
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("class ProductsController"));
    assert!(output.contains("#[Route('/products'"));
    assert!(output.contains("#[Route('/products/{id}'"));
}

#[test]
fn php_generator_promotes_inline_object_payloads_to_named_models() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": "Inline Shapes API", "version": "1.0.0" },
        "paths": {
            "/widgets": {
                "post": {
                    "operationId": "createWidget",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "name": { "type": "string" },
                                        "size": { "type": "integer" },
                                        "metadata": {
                                            "type": "object",
                                            "properties": {
                                                "enabled": { "type": "boolean" }
                                            },
                                            "required": ["enabled"]
                                        }
                                    },
                                    "required": ["name"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Created",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "id": { "type": "string" },
                                            "status": { "type": "string" }
                                        },
                                        "required": ["id", "status"]
                                    }
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

    assert!(
        output.contains("readonly class CreateWidgetRequestBody"),
        "inline request bodies should get named model classes"
    );
    assert!(
        output.contains("readonly class CreateWidgetRequestBodyMetadata"),
        "nested inline objects should also get named models"
    );
    assert!(
        output.contains("@param CreateWidgetRequestBody $body"),
        "controller docs should reference the generated request body model"
    );
    assert!(
        output.contains("public function createWidget(CreateWidgetRequestBody $body): CreateWidgetResponseBody"),
        "controller signature should use generated inline request and response models"
    );
    assert!(
        output.contains("readonly class CreateWidgetResponseBody"),
        "inline responses should get named model classes"
    );
}

#[test]
fn php_generator_promotes_component_nested_objects_without_duplicate_parent_aliases() {
    let schema_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testing_data/openapi_schemas/complex_nested.json");
    let spec: OpenAPI = serde_json::from_str(&fs::read_to_string(schema_path).expect("fixture")).expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("readonly class OrganizationSettingsIntegrations"));
    assert!(output.contains("public ?OrganizationSettingsIntegrations $integrations = null"));
    assert!(!output.contains("readonly class OrganizationIntegrations"));
    assert!(!output.contains("readonly class OrganizationNotifications"));
}

#[test]
fn php_openapi_generated_code_validates() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": "Validation API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Widget": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "name": { "type": "string" }
                    },
                    "required": ["id", "name"]
                }
            }
        },
        "paths": {
            "/widgets": {
                "post": {
                    "operationId": "createWidget",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/Widget" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Created",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/Widget" }
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
    let report = QualityValidator::new(TargetLanguage::Php)
        .validate_all(&output)
        .expect("php quality validation should run");

    assert!(
        report.is_valid(),
        "generated PHP OpenAPI code should validate cleanly: {report}"
    );
}

#[test]
fn php_openapi_auth_service_example_validates() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");
    let output = generate_from_openapi(&schema_path, TargetLanguage::Php, &DtoConfig::default()).expect("generate");
    let report = QualityValidator::new(TargetLanguage::Php)
        .validate_all(&output)
        .expect("php quality validation should run");

    assert!(
        report.is_valid(),
        "generated PHP auth-service OpenAPI code should validate cleanly: {report}"
    );
    assert!(!output.contains("StringBackedEnum"));
}

#[test]
fn php_generator_handles_multiple_http_methods() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "paths": {
            "/items": {
                "get": {
                    "operationId": "listItems",
                    "responses": { "200": { "description": "Success", "content": { "application/json": { "schema": { "type": "array" } } } } }
                },
                "post": {
                    "operationId": "createItem",
                    "responses": { "201": { "description": "Created", "content": { "application/json": { "schema": { "type": "object" } } } } }
                },
                "put": {
                    "operationId": "updateItem",
                    "responses": { "200": { "description": "Success", "content": { "application/json": { "schema": { "type": "object" } } } } }
                },
                "delete": {
                    "operationId": "deleteItem",
                    "responses": { "204": { "description": "Deleted" } }
                }
            }
        }
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("public function listItems"));
    assert!(output.contains("public function createItem"));
    assert!(output.contains("public function updateItem"));
    assert!(output.contains("public function deleteItem"));
    assert!(output.contains("methods: ['GET']"));
    assert!(output.contains("methods: ['POST']"));
    assert!(output.contains("methods: ['PUT']"));
    assert!(output.contains("methods: ['DELETE']"));
}

#[test]
fn php_generator_handles_path_and_query_parameters() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "paths": {
            "/users/{userId}/posts/{postId}": {
                "get": {
                    "operationId": "getPost",
                    "parameters": [
                        {
                            "name": "userId",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        },
                        {
                            "name": "postId",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string" }
                        },
                        {
                            "name": "includeComments",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string" }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": { "application/json": { "schema": { "type": "object" } } }
                        }
                    }
                }
            }
        }
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(
        output.contains("public function getPost(string $userId, string $postId, ?string $includeComments = null)")
    );
}

#[test]
fn php_generator_preserves_parameter_types_and_orders_required_params_before_optional_ones() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": "Typed Params API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "CreateTaskInput": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "priority": { "type": "string", "enum": ["low", "high"] }
                    },
                    "required": ["title", "priority"]
                }
            }
        },
        "paths": {
            "/projects/{projectId}/tasks": {
                "post": {
                    "operationId": "createTask",
                    "parameters": [
                        {
                            "name": "projectId",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer" }
                        },
                        {
                            "name": "limit",
                            "in": "query",
                            "required": true,
                            "schema": { "type": "integer" }
                        },
                        {
                            "name": "dryRun",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "boolean" }
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/CreateTaskInput" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Created",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/CreateTaskInput" }
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

    assert!(output.contains("enum CreateTaskInputPriority: string"));
    assert!(output.contains("public CreateTaskInputPriority $priority"));
    assert!(output.contains("@param CreateTaskInputPriority $priority"));
    assert!(output.contains("@param int $projectId"));
    assert!(output.contains("@param int $limit"));
    assert!(output.contains("@param bool|null $dryRun"));
    assert!(output.contains(
        "public function createTask(int $projectId, int $limit, CreateTaskInput $body, ?bool $dryRun = null)"
    ));
}

#[test]
fn php_generator_uses_semantic_types_for_string_formats_and_enums() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0",
        "info": { "title": "Event API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Event": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "startsAt": { "type": "string", "format": "date-time" },
                        "eventDate": { "type": "string", "format": "date" },
                        "status": { "type": "string", "enum": ["scheduled", "cancelled"] }
                    },
                    "required": ["id", "startsAt", "eventDate", "status"]
                }
            }
        },
        "paths": {
            "/events/{eventId}": {
                "get": {
                    "operationId": "getEvent",
                    "parameters": [
                        {
                            "name": "eventId",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "string", "format": "uuid" }
                        },
                        {
                            "name": "status",
                            "in": "query",
                            "required": false,
                            "schema": { "type": "string", "enum": ["scheduled", "cancelled"] }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/Event" }
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

    assert!(output.contains("readonly class UuidValue"));
    assert!(output.contains("enum EventStatus: string"));
    assert!(output.contains("case Scheduled = 'scheduled';"));
    assert!(output.contains("case Cancelled = 'cancelled';"));
    assert!(output.contains("public UuidValue $id"));
    assert!(output.contains("public \\DateTimeImmutable $startsAt"));
    assert!(output.contains("public \\DateTimeImmutable $eventDate"));
    assert!(output.contains("public EventStatus $status"));
    assert!(output.contains("enum GetEventStatus: string"));
    assert!(output.contains("@param UuidValue $eventId"));
    assert!(output.contains("@param GetEventStatus|null $status"));
    assert!(output.contains("public function getEvent(UuidValue $eventId, ?GetEventStatus $status = null): Event"));
}

#[test]
fn php_generator_escapes_special_characters_in_strings() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "API with 'quotes' and \"special\" chars",
            "version": "1.0.0"
        },
        "components": {
            "schemas": {
                "Test": {
                    "type": "object",
                    "description": "Test with\nnewlines\tand\rcarriage",
                    "properties": {
                        "field": { "type": "string" }
                    }
                }
            }
        },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    // Should contain the API title (escaped)
    assert!(output.contains("Test with") || output.contains("quotes") || output.contains("special"));
}

#[test]
fn php_generator_handles_reference_types() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "components": {
            "schemas": {
                "Address": {
                    "type": "object",
                    "properties": {
                        "street": { "type": "string" },
                        "city": { "type": "string" }
                    },
                    "required": ["street", "city"]
                },
                "User": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "address": { "$ref": "#/components/schemas/Address" }
                    },
                    "required": ["name", "address"]
                }
            }
        },
        "paths": {}
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("readonly class Address"));
    assert!(output.contains("readonly class User"));
}

#[test]
fn php_generator_includes_phpdoc_comments() {
    let spec: OpenAPI = serde_json::from_value(serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "paths": {
            "/test": {
                "get": {
                    "summary": "Test endpoint",
                    "description": "This endpoint tests something important",
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": { "application/json": { "schema": { "type": "object" } } }
                        }
                    }
                }
            }
        }
    }))
    .expect("spec");

    let generator = PhpGenerator::new(spec, PhpDtoStyle::ReadonlyClass);
    let output = generator.generate().expect("generate");

    assert!(output.contains("/**"));
    assert!(output.contains("Test endpoint"));
    assert!(output.contains("This endpoint tests something important"));
    assert!(output.contains("@return array"));
}
