//! gRPC fixture utilities
//!
//! This module provides data structures and utilities for working with gRPC test fixtures.
//! Fixtures are defined in JSON format and include protobuf definitions, service handlers,
//! requests, and expected responses for gRPC testing scenarios.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a complete gRPC test fixture with protobuf definition and test case data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrpcFixture {
    /// Descriptive name of the test case
    pub name: String,
    /// Detailed description of what this fixture tests
    #[serde(default)]
    pub description: Option<String>,
    /// Protobuf schema definition embedded in the fixture
    pub protobuf: ProtobufDef,
    /// Service handler specification
    pub handler: HandlerDef,
    /// gRPC request definition
    pub request: RequestDef,
    /// Expected response definition
    pub expected_response: ResponseDef,
    /// Optional tags for categorizing fixtures
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Protobuf schema definition for a gRPC service
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProtobufDef {
    /// Package name (e.g., "example.v1")
    pub package: String,
    /// Message definitions used by the service
    pub messages: Vec<MessageDef>,
    /// Service definitions
    pub services: Vec<ServiceDef>,
}

/// Protobuf message type definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageDef {
    /// Message type name
    pub name: String,
    /// Fields in this message
    pub fields: Vec<FieldDef>,
}

/// Protobuf message field definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldDef {
    /// Field name (snake_case by convention)
    pub name: String,
    /// Field type (scalar type or message name)
    #[serde(rename = "type")]
    pub field_type: String,
    /// Field number (used in wire format encoding)
    pub number: u32,
    /// Field label: "optional", "required", or "repeated"
    pub label: String,
}

/// Protobuf service definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceDef {
    /// Service name
    pub name: String,
    /// RPC methods in this service
    pub methods: Vec<MethodDef>,
}

/// Protobuf RPC method definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodDef {
    /// Method name
    pub name: String,
    /// Input message type name
    pub input_type: String,
    /// Output message type name
    pub output_type: String,
    /// Whether the client streams multiple requests
    #[serde(default)]
    pub client_streaming: bool,
    /// Whether the server streams multiple responses
    #[serde(default)]
    pub server_streaming: bool,
}

/// Specification of which service method is being tested
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HandlerDef {
    /// Fully qualified service name (e.g., "example.v1.UserService")
    pub service: String,
    /// Method name within the service
    pub method: String,
}

/// gRPC request definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestDef {
    /// Optional gRPC metadata/headers
    #[serde(default)]
    pub metadata: Option<HashMap<String, String>>,
    /// Single request message payload as JSON (for unary and server streaming)
    #[serde(default)]
    pub message: Option<Value>,
    /// Multiple request messages (for client streaming and bidirectional streaming)
    #[serde(default)]
    pub stream: Option<Vec<Value>>,
}

/// gRPC response definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseDef {
    /// gRPC status code (e.g., "OK", "NOT_FOUND", "INVALID_ARGUMENT")
    pub status_code: String,
    /// Single response message (for unary and server streaming)
    #[serde(default)]
    pub message: Option<Value>,
    /// Multiple response messages (for streaming responses)
    #[serde(default)]
    pub stream: Option<Vec<Value>>,
}

fn try_load_grpc_fixture(path: &Path) -> Option<GrpcFixture> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Skipping gRPC fixture {}: {}", path.display(), err);
            return None;
        }
    };

    match parse_grpc_fixture(&content) {
        Ok(fixture) => Some(fixture),
        Err(err) => {
            eprintln!("Skipping gRPC fixture {}: {}", path.display(), err);
            None
        }
    }
}

fn collect_grpc_fixtures(dir: &Path, fixtures: &mut Vec<GrpcFixture>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            collect_grpc_fixtures(&path, fixtures)?;
            continue;
        }

        if path.is_file()
            && path.extension().is_some_and(|e| e == "json")
            && path.file_name().is_some_and(|name| name != "schema.json")
        {
            if let Some(fixture) = try_load_grpc_fixture(&path) {
                fixtures.push(fixture);
            }
        }
    }

    Ok(())
}

/// Load all gRPC fixtures from a directory
///
/// This function recursively searches the protobuf/gRPC fixtures directory and loads
/// all JSON fixture files. It expects fixtures to be organized in a directory
/// structure like: `fixtures_dir/protobuf/*.json` or `fixtures_dir/grpc/*.json`
///
/// # Arguments
///
/// * `fixtures_dir` - Path to the root fixtures directory
///
/// # Returns
///
/// A vector of loaded `GrpcFixture` instances, or an error if loading fails
pub fn load_grpc_fixtures(fixtures_dir: &Path) -> Result<Vec<GrpcFixture>> {
    // Try multiple directory names for compatibility
    let grpc_dir = fixtures_dir.join("protobuf");
    let grpc_dir_alt = fixtures_dir.join("grpc");

    let target_dir = if grpc_dir.exists() {
        grpc_dir
    } else if grpc_dir_alt.exists() {
        grpc_dir_alt
    } else {
        return Ok(Vec::new());
    };

    let mut fixtures = Vec::new();
    collect_grpc_fixtures(&target_dir, &mut fixtures)?;

    Ok(fixtures)
}

/// Load gRPC fixtures from a specific category subdirectory
///
/// This function loads all fixtures from a category subdirectory within
/// the gRPC fixtures directory. Useful for organizing fixtures by type
/// or test scenario.
///
/// # Arguments
///
/// * `fixtures_dir` - Path to the root fixtures directory
/// * `category` - Category subdirectory name
///
/// # Returns
///
/// A vector of loaded `GrpcFixture` instances from that category
#[allow(dead_code)]
pub fn load_grpc_fixtures_by_category(fixtures_dir: &Path, category: &str) -> Result<Vec<GrpcFixture>> {
    let category_dir = fixtures_dir.join("grpc").join(category);

    if !category_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();
    collect_grpc_fixtures(&category_dir, &mut fixtures)?;

    Ok(fixtures)
}

/// Parse a gRPC fixture from JSON string
///
/// Deserializes a JSON string into a `GrpcFixture` struct. The JSON should
/// conform to the fixture schema defined by the GrpcFixture struct.
///
/// # Arguments
///
/// * `json` - JSON string containing the fixture definition
///
/// # Returns
///
/// Parsed `GrpcFixture` or an error if deserialization fails
pub fn parse_grpc_fixture(json: &str) -> Result<GrpcFixture> {
    serde_json::from_str(json).context("Failed to deserialize gRPC fixture JSON")
}

/// Generate a .proto file from a fixture's protobuf definition
///
/// Creates a Protocol Buffer 3 (.proto) file content from the embedded
/// protobuf definition in a fixture. The generated proto file includes:
/// - Syntax declaration (proto3)
/// - Package declaration
/// - All message type definitions with fields
/// - All service definitions with RPC methods
///
/// # Arguments
///
/// * `protobuf` - Protobuf definition from a fixture
///
/// # Returns
///
/// Generated .proto file content as a string
pub fn generate_proto_file(protobuf: &ProtobufDef) -> Result<String> {
    let mut proto = String::new();

    // Proto3 syntax declaration
    proto.push_str("syntax = \"proto3\";\n\n");

    // Package declaration
    proto.push_str(&format!("package {};\n\n", protobuf.package));

    // Message definitions
    for message in &protobuf.messages {
        proto.push_str(&format!("message {} {{\n", message.name));

        for field in &message.fields {
            let label = if field.label == "optional" || field.label == "required" {
                String::new()
            } else if field.label == "repeated" {
                "repeated ".to_string()
            } else {
                String::new()
            };

            proto.push_str(&format!(
                "  {}{} {} = {};\n",
                label, field.field_type, field.name, field.number
            ));
        }

        proto.push_str("}\n\n");
    }

    // Service definitions
    for service in &protobuf.services {
        proto.push_str(&format!("service {} {{\n", service.name));

        for method in &service.methods {
            let method_def = if method.client_streaming && method.server_streaming {
                // Bidirectional streaming
                format!(
                    "  rpc {}(stream {}) returns (stream {});\n",
                    method.name, method.input_type, method.output_type
                )
            } else if method.client_streaming {
                // Client streaming
                format!(
                    "  rpc {}(stream {}) returns ({});\n",
                    method.name, method.input_type, method.output_type
                )
            } else if method.server_streaming {
                // Server streaming
                format!(
                    "  rpc {}({}) returns (stream {});\n",
                    method.name, method.input_type, method.output_type
                )
            } else {
                // Unary
                format!(
                    "  rpc {}({}) returns ({});\n",
                    method.name, method.input_type, method.output_type
                )
            };
            proto.push_str(&method_def);
        }

        proto.push_str("}\n\n");
    }

    Ok(proto)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_grpc_fixture() {
        let json = r#"{
            "name": "Simple unary RPC - GetUser",
            "description": "Tests basic unary gRPC call",
            "protobuf": {
                "package": "example.v1",
                "messages": [
                    {
                        "name": "GetUserRequest",
                        "fields": [
                            {"name": "user_id", "type": "int32", "number": 1, "label": "required"}
                        ]
                    },
                    {
                        "name": "User",
                        "fields": [
                            {"name": "id", "type": "int32", "number": 1, "label": "required"},
                            {"name": "name", "type": "string", "number": 2, "label": "required"}
                        ]
                    }
                ],
                "services": [
                    {
                        "name": "UserService",
                        "methods": [
                            {
                                "name": "GetUser",
                                "input_type": "GetUserRequest",
                                "output_type": "User",
                                "client_streaming": false,
                                "server_streaming": false
                            }
                        ]
                    }
                ]
            },
            "handler": {
                "service": "example.v1.UserService",
                "method": "GetUser"
            },
            "request": {
                "metadata": {"authorization": "Bearer test-token"},
                "message": {"user_id": 123}
            },
            "expected_response": {
                "status_code": "OK",
                "message": {"id": 123, "name": "Alice"}
            }
        }"#;

        let fixture = parse_grpc_fixture(json).expect("Failed to parse fixture");
        assert_eq!(fixture.name, "Simple unary RPC - GetUser");
        assert_eq!(fixture.protobuf.package, "example.v1");
        assert_eq!(fixture.protobuf.messages.len(), 2);
        assert_eq!(fixture.protobuf.services.len(), 1);
        assert_eq!(fixture.handler.service, "example.v1.UserService");
        assert_eq!(fixture.handler.method, "GetUser");
    }

    #[test]
    fn test_parse_streaming_fixture() {
        let json = r#"{
            "name": "Server streaming RPC",
            "description": "Tests server streaming",
            "protobuf": {
                "package": "example.v1",
                "messages": [
                    {
                        "name": "ListRequest",
                        "fields": [
                            {"name": "limit", "type": "int32", "number": 1, "label": "optional"}
                        ]
                    },
                    {
                        "name": "Item",
                        "fields": [
                            {"name": "id", "type": "int32", "number": 1, "label": "required"},
                            {"name": "name", "type": "string", "number": 2, "label": "required"}
                        ]
                    }
                ],
                "services": [
                    {
                        "name": "ItemService",
                        "methods": [
                            {
                                "name": "ListItems",
                                "input_type": "ListRequest",
                                "output_type": "Item",
                                "client_streaming": false,
                                "server_streaming": true
                            }
                        ]
                    }
                ]
            },
            "handler": {
                "service": "example.v1.ItemService",
                "method": "ListItems"
            },
            "request": {
                "message": {"limit": 10}
            },
            "expected_response": {
                "status_code": "OK",
                "stream": [
                    {"id": 1, "name": "Item 1"},
                    {"id": 2, "name": "Item 2"}
                ]
            }
        }"#;

        let fixture = parse_grpc_fixture(json).expect("Failed to parse fixture");
        assert_eq!(fixture.name, "Server streaming RPC");
        assert!(fixture.protobuf.services[0].methods[0].server_streaming);
        assert!(!fixture.protobuf.services[0].methods[0].client_streaming);
        assert!(fixture.expected_response.stream.is_some());
        assert_eq!(fixture.expected_response.stream.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_generate_proto_file_unary() {
        let protobuf = ProtobufDef {
            package: "example.v1".to_string(),
            messages: vec![
                MessageDef {
                    name: "GetUserRequest".to_string(),
                    fields: vec![FieldDef {
                        name: "user_id".to_string(),
                        field_type: "int32".to_string(),
                        number: 1,
                        label: "required".to_string(),
                    }],
                },
                MessageDef {
                    name: "User".to_string(),
                    fields: vec![
                        FieldDef {
                            name: "id".to_string(),
                            field_type: "int32".to_string(),
                            number: 1,
                            label: "required".to_string(),
                        },
                        FieldDef {
                            name: "name".to_string(),
                            field_type: "string".to_string(),
                            number: 2,
                            label: "required".to_string(),
                        },
                    ],
                },
            ],
            services: vec![ServiceDef {
                name: "UserService".to_string(),
                methods: vec![MethodDef {
                    name: "GetUser".to_string(),
                    input_type: "GetUserRequest".to_string(),
                    output_type: "User".to_string(),
                    client_streaming: false,
                    server_streaming: false,
                }],
            }],
        };

        let proto = generate_proto_file(&protobuf).expect("Failed to generate proto");

        assert!(proto.contains("syntax = \"proto3\";"));
        assert!(proto.contains("package example.v1;"));
        assert!(proto.contains("message GetUserRequest"));
        assert!(proto.contains("message User"));
        assert!(proto.contains("service UserService"));
        // Verify complete method signature
        assert!(proto.contains("rpc GetUser(GetUserRequest) returns (User);"));
    }

    #[test]
    fn test_generate_proto_file_streaming() {
        let protobuf = ProtobufDef {
            package: "example.v1".to_string(),
            messages: vec![
                MessageDef {
                    name: "ListRequest".to_string(),
                    fields: vec![FieldDef {
                        name: "limit".to_string(),
                        field_type: "int32".to_string(),
                        number: 1,
                        label: "optional".to_string(),
                    }],
                },
                MessageDef {
                    name: "Item".to_string(),
                    fields: vec![FieldDef {
                        name: "id".to_string(),
                        field_type: "int32".to_string(),
                        number: 1,
                        label: "required".to_string(),
                    }],
                },
            ],
            services: vec![ServiceDef {
                name: "ItemService".to_string(),
                methods: vec![MethodDef {
                    name: "ListItems".to_string(),
                    input_type: "ListRequest".to_string(),
                    output_type: "Item".to_string(),
                    client_streaming: false,
                    server_streaming: true,
                }],
            }],
        };

        let proto = generate_proto_file(&protobuf).expect("Failed to generate proto");

        // Verify complete method signature with stream keyword
        assert!(proto.contains("rpc ListItems(ListRequest) returns (stream Item);"));
    }

    #[test]
    fn test_repeated_field_generation() {
        let protobuf = ProtobufDef {
            package: "test".to_string(),
            messages: vec![MessageDef {
                name: "Message".to_string(),
                fields: vec![FieldDef {
                    name: "items".to_string(),
                    field_type: "string".to_string(),
                    number: 1,
                    label: "repeated".to_string(),
                }],
            }],
            services: vec![],
        };

        let proto = generate_proto_file(&protobuf).expect("Failed to generate proto");
        assert!(proto.contains("repeated string items"));
    }

    #[test]
    fn test_load_grpc_fixtures_integration() {
        let fixtures_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testing_data");

        let fixtures = load_grpc_fixtures(&fixtures_path).expect("Failed to load fixtures");
        assert!(
            !fixtures.is_empty(),
            "Should load at least some gRPC fixtures from testing_data"
        );

        // Validate structure of first fixture
        let first = &fixtures[0];
        assert!(!first.name.is_empty(), "Fixture should have a name");
        assert!(!first.protobuf.package.is_empty(), "Fixture should have a package");
        assert!(
            !first.protobuf.messages.is_empty(),
            "Fixture should have at least one message"
        );
        assert!(
            !first.protobuf.services.is_empty(),
            "Fixture should have at least one service"
        );
    }

    #[test]
    fn test_handler_parsing() {
        let json = r#"{
            "name": "Test",
            "protobuf": {
                "package": "pkg",
                "messages": [],
                "services": []
            },
            "handler": {
                "service": "my.package.MyService",
                "method": "MyMethod"
            },
            "request": {"message": {}},
            "expected_response": {"status_code": "OK"}
        }"#;

        let fixture = parse_grpc_fixture(json).expect("Failed to parse");
        assert_eq!(fixture.handler.service, "my.package.MyService");
        assert_eq!(fixture.handler.method, "MyMethod");
    }

    #[test]
    fn test_metadata_parsing() {
        let json = r#"{
            "name": "Test",
            "protobuf": {
                "package": "pkg",
                "messages": [],
                "services": []
            },
            "handler": {
                "service": "Service",
                "method": "Method"
            },
            "request": {
                "metadata": {
                    "authorization": "Bearer token",
                    "custom-header": "value"
                },
                "message": {}
            },
            "expected_response": {"status_code": "OK"}
        }"#;

        let fixture = parse_grpc_fixture(json).expect("Failed to parse");
        assert!(fixture.request.metadata.is_some());
        let metadata = fixture.request.metadata.unwrap();
        assert_eq!(metadata.get("authorization").map(|s| s.as_str()), Some("Bearer token"));
        assert_eq!(metadata.get("custom-header").map(|s| s.as_str()), Some("value"));
    }

    #[test]
    fn test_client_streaming_flag() {
        let json = r#"{
            "name": "Test",
            "protobuf": {
                "package": "pkg",
                "messages": [],
                "services": [
                    {
                        "name": "Service",
                        "methods": [
                            {
                                "name": "Stream",
                                "input_type": "In",
                                "output_type": "Out",
                                "client_streaming": true,
                                "server_streaming": false
                            }
                        ]
                    }
                ]
            },
            "handler": {
                "service": "Service",
                "method": "Stream"
            },
            "request": {"message": {}},
            "expected_response": {"status_code": "OK"}
        }"#;

        let fixture = parse_grpc_fixture(json).expect("Failed to parse");
        let method = &fixture.protobuf.services[0].methods[0];
        assert!(method.client_streaming);
        assert!(!method.server_streaming);
    }
}
