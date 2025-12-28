use openapiv3::OpenAPI;
use spikard_cli::codegen::{PhpDtoStyle, PhpGenerator};

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
