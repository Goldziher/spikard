use spikard_cli::codegen::ts_schema::generate_typescript_dto;

#[test]
fn typescript_dto_generation_handles_object_required_and_enum() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "kind": { "enum": ["a", "b"] }
        },
        "required": ["id"]
    });

    let dto = generate_typescript_dto("test_event", &schema).expect("dto");

    assert!(dto.schema_declaration.contains("const"));
    assert!(dto.schema_declaration.contains("z.object"));
    assert!(dto.type_declaration.contains("type"));
    assert!(dto.type_declaration.contains("id: string"));
    assert!(dto.type_declaration.contains("kind:"));
    assert!(dto.type_declaration.contains("\"a\""));
}

#[test]
fn typescript_dto_generation_supports_additional_properties_schema() {
    let schema = serde_json::json!({
        "type": "object",
        "additionalProperties": { "type": "string" }
    });

    let dto = generate_typescript_dto("kv", &schema).expect("dto");
    assert!(dto.type_declaration.contains("Record<string, string>"));
}
