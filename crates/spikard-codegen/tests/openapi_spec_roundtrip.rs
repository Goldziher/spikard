use indexmap::IndexMap;
use spikard_codegen::openapi::spec::{
    Components, Example, Header, Info, MediaType, OpenApiSpec, Operation, Parameter, PathItem, RequestBody, Response,
    Schema, SchemaObject, Server, Tag,
};

#[test]
fn openapi_spec_roundtrips_with_optional_fields() {
    let mut paths = IndexMap::new();
    paths.insert(
        "/widgets".to_string(),
        PathItem {
            get: Some(Operation {
                summary: Some("List widgets".to_string()),
                description: None,
                operation_id: Some("listWidgets".to_string()),
                parameters: Some(vec![Parameter {
                    name: "limit".to_string(),
                    location: "query".to_string(),
                    description: Some("Max widgets".to_string()),
                    required: Some(false),
                    schema: Some(Schema::Object(Box::new(SchemaObject {
                        schema_type: "integer".to_string(),
                        properties: None,
                        required: None,
                        format: Some("int32".to_string()),
                        items: None,
                        minimum: Some(1.0),
                        maximum: Some(100.0),
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        description: None,
                    }))),
                }]),
                request_body: Some(RequestBody {
                    description: Some("Optional request body".to_string()),
                    content: {
                        let mut content = IndexMap::new();
                        content.insert(
                            "application/json".to_string(),
                            MediaType {
                                schema: Some(Schema::Ref {
                                    reference: "#/components/schemas/Widget".to_string(),
                                }),
                                example: Some(serde_json::json!({"name": "demo"})),
                                examples: Some({
                                    let mut examples = IndexMap::new();
                                    examples.insert(
                                        "demo".to_string(),
                                        Example {
                                            summary: Some("example".to_string()),
                                            value: Some(serde_json::json!({"name": "demo"})),
                                        },
                                    );
                                    examples
                                }),
                            },
                        );
                        content
                    },
                    required: Some(false),
                }),
                responses: {
                    let mut responses = IndexMap::new();
                    responses.insert(
                        "200".to_string(),
                        Response {
                            description: "ok".to_string(),
                            content: Some({
                                let mut content = IndexMap::new();
                                content.insert(
                                    "application/json".to_string(),
                                    MediaType {
                                        schema: Some(Schema::Ref {
                                            reference: "#/components/schemas/Widget".to_string(),
                                        }),
                                        example: None,
                                        examples: None,
                                    },
                                );
                                content
                            }),
                            headers: Some({
                                let mut headers = IndexMap::new();
                                headers.insert(
                                    "x-request-id".to_string(),
                                    Header {
                                        description: Some("request id".to_string()),
                                        schema: Some(Schema::Object(Box::new(SchemaObject {
                                            schema_type: "string".to_string(),
                                            properties: None,
                                            required: None,
                                            format: None,
                                            items: None,
                                            minimum: None,
                                            maximum: None,
                                            min_length: None,
                                            max_length: None,
                                            pattern: None,
                                            description: None,
                                        }))),
                                    },
                                );
                                headers
                            }),
                        },
                    );
                    responses
                },
                tags: Some(vec!["widgets".to_string()]),
            }),
            post: None,
            put: None,
            patch: None,
            delete: None,
            parameters: None,
        },
    );

    let spec = OpenApiSpec {
        openapi: "3.1.0".to_string(),
        info: Info {
            title: "Widget API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test spec".to_string()),
        },
        servers: Some(vec![Server {
            url: "https://example.test".to_string(),
            description: Some("test".to_string()),
        }]),
        paths,
        components: Some(Components {
            schemas: Some({
                let mut schemas = IndexMap::new();
                schemas.insert(
                    "Widget".to_string(),
                    Schema::Object(Box::new(SchemaObject {
                        schema_type: "object".to_string(),
                        properties: Some({
                            let mut properties = IndexMap::new();
                            properties.insert(
                                "name".to_string(),
                                Box::new(Schema::Object(Box::new(SchemaObject {
                                    schema_type: "string".to_string(),
                                    properties: None,
                                    required: None,
                                    format: None,
                                    items: None,
                                    minimum: None,
                                    maximum: None,
                                    min_length: Some(1),
                                    max_length: Some(100),
                                    pattern: None,
                                    description: Some("Name".to_string()),
                                }))),
                            );
                            properties
                        }),
                        required: Some(vec!["name".to_string()]),
                        format: None,
                        items: None,
                        minimum: None,
                        maximum: None,
                        min_length: None,
                        max_length: None,
                        pattern: None,
                        description: None,
                    })),
                );
                schemas
            }),
            responses: None,
            parameters: None,
        }),
        tags: Some(vec![Tag {
            name: "widgets".to_string(),
            description: Some("widget operations".to_string()),
        }]),
    };

    let json = serde_json::to_string(&spec).expect("serialize OpenApiSpec");
    let decoded: OpenApiSpec = serde_json::from_str(&json).expect("deserialize OpenApiSpec");

    assert_eq!(decoded.openapi, "3.1.0");
    assert_eq!(decoded.info.title, "Widget API");
    assert_eq!(decoded.paths.len(), 1);
    assert!(decoded.components.is_some());
    assert_eq!(decoded.tags.as_ref().unwrap().first().unwrap().name, "widgets");
}
