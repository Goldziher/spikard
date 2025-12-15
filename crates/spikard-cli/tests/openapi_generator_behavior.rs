use anyhow::Result;
use openapiv3::{MediaType, OpenAPI, Operation, PathItem, ReferenceOr, RequestBody, Response, Responses, Schema};
use spikard_cli::codegen::{OpenApiGenerator, SchemaRegistry};

struct DummyGenerator {
    spec: OpenAPI,
    registry: SchemaRegistry,
}

impl DummyGenerator {
    fn new(spec: OpenAPI) -> Self {
        let registry = SchemaRegistry::from_spec(&spec);
        Self { spec, registry }
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
        "// header\n".to_string()
    }

    fn generate_models(&self) -> Result<String> {
        Ok("// models\n".to_string())
    }

    fn generate_routes(&self) -> Result<String> {
        Ok("// routes\n".to_string())
    }
}

fn json_schema_ref(schema: Schema) -> ReferenceOr<Schema> {
    ReferenceOr::Item(schema)
}

fn empty_object_schema() -> Schema {
    Schema {
        schema_data: Default::default(),
        schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Object(Default::default())),
    }
}

#[test]
fn iter_paths_visits_all_http_methods_and_operation_ids() {
    let mut spec = OpenAPI::default();
    let mut path_item = PathItem::default();

    let get = Operation {
        operation_id: Some("getThing".to_string()),
        ..Default::default()
    };
    path_item.get = Some(get);

    let post = Operation::default();
    path_item.post = Some(post);

    let put = Operation::default();
    path_item.put = Some(put);

    let delete = Operation::default();
    path_item.delete = Some(delete);

    let patch = Operation::default();
    path_item.patch = Some(patch);

    spec.paths
        .paths
        .insert("/things/{id}".to_string(), ReferenceOr::Item(path_item));

    let generator = DummyGenerator::new(spec);
    let mut seen = Vec::new();
    generator
        .iter_paths(|path, method, op| {
            seen.push((
                path.to_string(),
                method.to_string(),
                generator.generate_operation_id(path, method, op),
            ));
            Ok(())
        })
        .unwrap();

    assert!(seen.iter().any(|(_, m, id)| m == "get" && id == "get_thing"));
    assert!(seen.iter().any(|(_, m, _)| m == "post"));
    assert!(seen.iter().any(|(_, m, _)| m == "put"));
    assert!(seen.iter().any(|(_, m, _)| m == "delete"));
    assert!(seen.iter().any(|(_, m, _)| m == "patch"));
}

#[test]
fn extract_request_and_response_types_handle_refs_and_ranges() {
    let mut spec = OpenAPI::default();

    // components.schemas["Widget"]
    let widget_schema = empty_object_schema();
    spec.components
        .get_or_insert_with(Default::default)
        .schemas
        .insert("Widget".to_string(), ReferenceOr::Item(widget_schema));

    // request body uses $ref (should format type name from ref name)
    let request_body = RequestBody {
        content: {
            let mut content: openapiv3::Content = Default::default();
            content.insert(
                "application/json".to_string(),
                MediaType {
                    schema: Some(ReferenceOr::Reference {
                        reference: "#/components/schemas/Widget".to_string(),
                    }),
                    ..Default::default()
                },
            );
            content
        },
        ..Default::default()
    };

    // 2XX range response uses inline schema (should return default response type)
    let response = Response {
        content: {
            let mut content: openapiv3::Content = Default::default();
            content.insert(
                "application/json".to_string(),
                MediaType {
                    schema: Some(json_schema_ref(empty_object_schema())),
                    ..Default::default()
                },
            );
            content
        },
        ..Default::default()
    };

    let mut responses = Responses::default();
    responses
        .responses
        .insert(openapiv3::StatusCode::Range(2), ReferenceOr::Item(response));
    let op = Operation {
        request_body: Some(ReferenceOr::Item(request_body)),
        responses,
        ..Default::default()
    };

    let generator = DummyGenerator::new(spec);

    assert_eq!(generator.extract_request_body_type(&op).as_deref(), Some("Widget"));
    assert_eq!(generator.extract_response_type(&op), "unknown");
}

#[test]
fn iter_schemas_calls_callback_for_inline_schemas_only() {
    let mut spec = OpenAPI::default();
    let components = spec.components.get_or_insert_with(Default::default);
    components
        .schemas
        .insert("Inline".to_string(), ReferenceOr::Item(empty_object_schema()));
    components.schemas.insert(
        "Ref".to_string(),
        ReferenceOr::Reference {
            reference: "#/components/schemas/Inline".to_string(),
        },
    );

    let generator = DummyGenerator::new(spec);
    let mut names = Vec::new();
    generator
        .iter_schemas(|name, _schema| {
            names.push(name.to_string());
            Ok(())
        })
        .unwrap();

    assert_eq!(names, vec!["Inline".to_string()]);
}

#[test]
fn extract_response_type_formats_reference_names() {
    let spec = OpenAPI::default();
    let generator = DummyGenerator::new(spec);

    let response_ref = ReferenceOr::Reference {
        reference: "#/components/responses/MyResponse".to_string(),
    };

    let mut op = Operation::default();
    let mut responses = Responses::default();
    responses
        .responses
        .insert(openapiv3::StatusCode::Code(200), response_ref);
    op.responses = responses;

    assert_eq!(generator.extract_response_type(&op), "MyResponse");
}
