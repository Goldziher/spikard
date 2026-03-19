#![allow(
    clippy::needless_raw_string_hashes,
    clippy::too_many_arguments,
    clippy::similar_names,
    clippy::doc_markdown,
    clippy::uninlined_format_args,
    clippy::redundant_clone,
    reason = "Test file with many GraphQL schemas and test parameters"
)]
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use spikard_cli::codegen::{
    CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, NodeDtoStyle, PythonDtoStyle,
    RubyDtoStyle, SchemaKind, TargetLanguage, generate_from_openapi, quality::QualityValidator,
};
use tempfile::tempdir;

const SIMPLE_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Example API
  version: "1.0.0"
paths:
  /hello:
    get:
      operationId: helloWorld
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HelloResponse"
components:
  schemas:
    HelloResponse:
      type: object
      description: Example greeting payload
      properties:
        message:
          type: string
        count:
          type: integer
          nullable: true
      required:
        - message
"##;

const ALLOF_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Errors API
  version: "1.0.0"
paths: {}
components:
  schemas:
    BaseError:
      type: object
      properties:
        title:
          type: string
        status:
          type: integer
      required:
        - title
        - status
    AuthError:
      allOf:
        - $ref: "#/components/schemas/BaseError"
        - type: object
          properties:
            detail:
              type: string
          required:
            - detail
"##;

const SIMPLE_ASYNCAPI: &str = r##"
asyncapi: "3.0.0"
info:
  title: Chat API
  version: "1.0.0"
servers:
  primary:
    host: ws.example.com
    protocol: ws
channels:
  /chat:
    messages:
      chatEvent:
        payload:
          type: object
          properties:
            type:
              const: chatEvent
            body:
              type: string
          required:
            - type
            - body
"##;

const INLINE_OBJECT_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Inline API
  version: "1.0.0"
paths:
  /widgets:
    post:
      operationId: createWidget
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                name:
                  type: string
                size:
                  type: integer
                metadata:
                  type: object
                  properties:
                    enabled:
                      type: boolean
                  required:
                    - enabled
              required:
                - name
      responses:
        "201":
          description: Created
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                  status:
                    type: string
                required:
                  - id
                  - status
"##;

const TYPED_PARAMETERS_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Typed Parameters API
  version: "1.0.0"
paths:
  /projects/{projectId}/tasks:
    get:
      operationId: listTasks
      parameters:
        - name: projectId
          in: path
          required: true
          schema:
            type: integer
        - name: status
          in: query
          schema:
            type: string
            enum:
              - todo
              - in_progress
              - done
        - name: limit
          in: query
          required: true
          schema:
            type: integer
        - name: includeArchived
          in: query
          schema:
            type: boolean
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                type: object
                properties:
                  ok:
                    type: boolean
                required:
                  - ok
"##;

const STRING_FORMATS_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: String Formats API
  version: "1.0.0"
paths:
  /events/{eventId}:
    get:
      operationId: getEvent
      parameters:
        - name: eventId
          in: path
          required: true
          schema:
            type: string
            format: uuid
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                    format: uuid
                  startsAt:
                    type: string
                    format: date-time
                  eventDate:
                    type: string
                    format: date
                  rawPayload:
                    type: string
                    format: byte
                required:
                  - id
                  - startsAt
                  - eventDate
                  - rawPayload
"##;

const ROUTE_SEMANTICS_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Route Semantics API
  version: "1.0.0"
paths:
  /events/{eventId}:
    get:
      operationId: getEvent
      parameters:
        - name: eventId
          in: path
          required: true
          schema:
            type: string
            format: uuid
        - name: status
          in: query
          required: false
          schema:
            type: string
            enum:
              - scheduled
              - cancelled
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                type: object
                properties:
                  ok:
                    type: boolean
                required:
                  - ok
"##;

fn write_temp_file(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, contents).expect("failed to write test fixture");
    path
}

#[test]
fn python_dataclass_generation_emits_dataclasses() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(code.contains("@dataclass"), "expected dataclass annotation");
    assert!(code.contains("class HelloResponse"), "expected response class");
    assert!(code.contains("slots=True"), "dataclasses should enable slots");
    assert_python_class_executes("HelloResponse", &code)?;
    Ok(())
}

#[test]
fn python_msgspec_generation_emits_structs() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Msgspec,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("class HelloResponse(msgspec.Struct)"),
        "msgspec struct not generated"
    );
    assert!(code.contains("import msgspec"), "msgspec import missing");
    assert_python_class_executes("HelloResponse", &code)?;
    Ok(())
}

#[test]
fn python_nullable_properties_emit_optional_union() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("count: int | None = None"),
        "expected nullable optional dataclass field"
    );
    Ok(())
}

#[test]
fn python_openapi_generation_uses_server_config_startup() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("from spikard.config import ServerConfig"),
        "expected ServerConfig import in generated startup"
    );
    assert!(
        code.contains("app.run(config=ServerConfig(host=\"0.0.0.0\", port=8000))"),
        "expected ServerConfig-based startup in generated python app"
    );
    Ok(())
}

#[test]
fn python_openapi_all_of_models_merge_object_fields() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "allof_openapi.yaml", ALLOF_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(code.contains("class AuthError"), "expected composed model class");
    assert!(code.contains("title: str"), "expected inherited required field");
    assert!(code.contains("status: int"), "expected inherited integer field");
    assert!(code.contains("detail: str"), "expected composed field");

    Ok(())
}

#[test]
fn python_openapi_generated_code_validates() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    let report = QualityValidator::new(TargetLanguage::Python)
        .validate_all(&code)
        .expect("python openapi validation should run");

    assert!(
        report.is_valid(),
        "generated Python OpenAPI code should validate cleanly: {report}"
    );

    Ok(())
}

#[test]
fn python_openapi_inline_objects_generate_named_models() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "inline_openapi.yaml", INLINE_OBJECT_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("class CreateWidgetRequestBody:"),
        "expected named request body model"
    );
    assert!(
        code.contains("class CreateWidgetRequestBodyMetadata:"),
        "expected nested named model for shaped inline object"
    );
    assert!(
        code.contains("body: Body[CreateWidgetRequestBody]"),
        "route signature should use generated request body model"
    );
    assert!(
        code.contains(") -> CreateWidgetResponseBody:"),
        "route return type should use generated response body model"
    );

    Ok(())
}

#[test]
fn python_openapi_component_inline_objects_generate_named_models() -> Result<()> {
    let schema_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testing_data/openapi_schemas/complex_nested.json");

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("class OrganizationSettingsIntegrations:"),
        "expected named model for nested component object"
    );
    assert!(
        code.contains("integrations: OrganizationSettingsIntegrations | None = None"),
        "component field should use generated nested model"
    );
    assert!(
        code.contains("class OrganizationSettingsNotifications:"),
        "expected named model for additional nested component object"
    );
    assert!(
        code.contains("notifications: OrganizationSettingsNotifications | None = None"),
        "nested notifications object should use generated model"
    );
    assert!(
        code.contains("metadata: ProjectMetadata | None = None"),
        "shaped metadata should use a generated nested model"
    );
    assert!(
        code.contains("custom_fields: dict[str, object] | None = None"),
        "free-form nested leaves should remain dicts"
    );

    Ok(())
}

#[test]
fn python_openapi_parameters_preserve_schema_types() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "typed_parameters_openapi.yaml", TYPED_PARAMETERS_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("from typing import Literal"),
        "expected Literal import when enum-backed parameter types are generated"
    );
    assert!(
        code.contains("project_id: Path[int]"),
        "path parameters should preserve integer typing"
    );
    assert!(
        code.contains("status: Query[Literal[\"todo\", \"in_progress\", \"done\"] | None] = Query(default=None)"),
        "optional enum query parameters should preserve literal typing"
    );
    assert!(
        code.contains("limit: Query[int]"),
        "required integer query parameters should preserve integer typing"
    );
    assert!(
        code.contains("include_archived: Query[bool | None] = Query(default=None)"),
        "optional boolean query parameters should preserve boolean typing"
    );

    Ok(())
}

#[test]
fn python_openapi_string_formats_use_semantic_types() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "string_formats_openapi.yaml", STRING_FORMATS_OPENAPI);

    let dto = DtoConfig {
        python: PythonDtoStyle::Dataclass,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto)?;
    assert!(
        code.contains("from datetime import date, datetime"),
        "expected datetime imports for date/date-time schemas"
    );
    assert!(
        code.contains("from uuid import UUID"),
        "expected UUID import for uuid-formatted schemas"
    );
    assert!(
        code.contains("event_id: Path[UUID]"),
        "uuid path parameters should use UUID typing"
    );
    assert!(code.contains("id: UUID"), "uuid schema fields should use UUID typing");
    assert!(
        code.contains("starts_at: datetime"),
        "date-time schema fields should use datetime typing"
    );
    assert!(
        code.contains("event_date: date"),
        "date schema fields should use date typing"
    );
    assert!(
        code.contains("raw_payload: bytes"),
        "byte-formatted string fields should use bytes typing"
    );

    Ok(())
}

#[test]
fn node_generation_uses_zod_schemas() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        node: NodeDtoStyle::Zod,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto)?;
    assert!(
        code.contains("import { z } from \"zod\""),
        "expected Zod import in generated code"
    );
    assert!(
        code.contains("export const HelloResponseSchema = z.object"),
        "expected inferred schema"
    );
    Ok(())
}

#[test]
fn typescript_nullable_properties_emit_nullable_optional_schemas() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        node: NodeDtoStyle::Zod,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto)?;
    assert!(
        code.contains("\tcount: z.number().int().nullable().optional(),"),
        "expected nullable + optional zod chain"
    );
    Ok(())
}

#[test]
fn typescript_route_parameters_preserve_schema_types() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "typed_parameters.yaml", TYPED_PARAMETERS_OPENAPI);

    let dto = DtoConfig {
        node: NodeDtoStyle::Zod,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto)?;
    assert!(
        code.contains("_project_id: Path<number>"),
        "integer path parameters should preserve number typing"
    );
    assert!(
        code.contains("_status: Query<\"todo\" | \"in_progress\" | \"done\" | undefined>"),
        "enum query parameters should preserve literal union typing"
    );
    assert!(
        code.contains("_include_archived: Query<boolean | undefined>"),
        "boolean query parameters should preserve boolean typing"
    );

    Ok(())
}

#[test]
fn typescript_openapi_auth_service_example_validates() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let dto = DtoConfig {
        node: NodeDtoStyle::Zod,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto)?;
    let report = QualityValidator::new(TargetLanguage::TypeScript)
        .validate_all(&code)
        .expect("typescript openapi validation should run");

    assert!(
        report.is_valid(),
        "generated TypeScript auth-service OpenAPI code should validate cleanly: {report}"
    );
    assert!(
        !code.contains("Body<z."),
        "route body generics should use TypeScript types, not zod expressions"
    );

    Ok(())
}

#[test]
fn typescript_openapi_auth_service_preserves_semantic_zod_and_union_bodies() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let dto = DtoConfig {
        node: NodeDtoStyle::Zod,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto)?;

    assert!(
        code.contains("\tcreated_at: z.string().datetime(),"),
        "date-time fields should emit semantic zod validators"
    );
    assert!(
        code.contains("keys: z.array(z.object({"),
        "shaped array item objects should remain shaped in zod schemas"
    );
    assert!(
        code.contains("pagination: z.object({"),
        "nested object properties should remain shaped in zod schemas"
    );
    assert!(
        code.contains(
            "Body<PasswordGrantRequest | ClientCredentialsGrantRequest | RefreshTokenGrantRequest>"
        ),
        "oneOf request bodies should preserve union types"
    );

    Ok(())
}

#[test]
fn ruby_generation_uses_dry_structs() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("class HelloResponse < Dry::Struct"),
        "expected Dry::Struct model"
    );
    Ok(())
}

#[test]
fn ruby_openapi_inline_objects_generate_named_models() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "inline_openapi.yaml", INLINE_OBJECT_OPENAPI);

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("class CreateWidgetRequestBody < Dry::Struct"),
        "expected named dry-struct request body model"
    );
    assert!(
        code.contains("class CreateWidgetResponseBody < Dry::Struct"),
        "expected named dry-struct response body model"
    );
    assert!(
        code.contains("class CreateWidgetRequestBodyMetadata < Dry::Struct"),
        "nested inline objects should get named dry-struct models"
    );
    assert!(
        code.contains("attribute :metadata, Types.Instance(CreateWidgetRequestBodyMetadata).optional"),
        "parent model should reference the nested generated model"
    );
    assert!(
        code.contains("# @param body [CreateWidgetRequestBody] Request body"),
        "route docs should use generated request model type"
    );
    assert!(
        code.contains("# @return [CreateWidgetResponseBody] Response body"),
        "route docs should use generated response model type"
    );

    Ok(())
}

#[test]
fn ruby_openapi_component_inline_objects_generate_named_models() -> Result<()> {
    let schema_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testing_data/openapi_schemas/complex_nested.json");

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("class OrganizationSettingsIntegrations < Dry::Struct"),
        "expected named model for nested component object"
    );
    assert!(
        code.contains("attribute :integrations, Types.Instance(OrganizationSettingsIntegrations).optional"),
        "component field should reference generated nested model"
    );
    assert!(
        code.contains("class OrganizationSettingsNotifications < Dry::Struct"),
        "expected named model for additional nested component object"
    );
    assert!(
        code.contains("attribute :notifications, Types.Instance(OrganizationSettingsNotifications).optional"),
        "nested notifications object should reference generated model"
    );
    assert!(
        code.contains("attribute :custom_fields, Types::Strict::Hash.optional"),
        "free-form nested leaves should remain hashes"
    );

    Ok(())
}

#[test]
fn ruby_openapi_string_formats_and_enums_use_semantic_types() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "string_formats_openapi.yaml", STRING_FORMATS_OPENAPI);

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("require 'date'"),
        "expected date support import for semantic date/date-time types"
    );
    assert!(
        code.contains("UUID = Types::Strict::String"),
        "expected UUID helper type in generated Types module"
    );
    assert!(
        code.contains("attribute :id, Types::UUID"),
        "uuid schema fields should use UUID helper type"
    );
    assert!(
        code.contains("attribute :starts_at, Types::ISODateTime"),
        "date-time schema fields should use semantic datetime type"
    );
    assert!(
        code.contains("attribute :event_date, Types::ISODate"),
        "date schema fields should use semantic date type"
    );

    Ok(())
}

#[test]
fn ruby_openapi_routes_coerce_and_validate_semantic_parameters() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "route_semantics_openapi.yaml", ROUTE_SEMANTICS_OPENAPI);

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("def coerce_uuid_param!(value, name)"),
        "expected UUID route coercion helper"
    );
    assert!(
        code.contains("def coerce_enum_param!(value, name, allowed)"),
        "expected enum route coercion helper"
    );
    assert!(
        code.contains("# @param event_id [String] Path parameter (UUID)"),
        "path parameter docs should preserve UUID semantics"
    );
    assert!(
        code.contains("# @param status [String, nil] Query parameter (enum: scheduled, cancelled; optional)"),
        "query parameter docs should preserve enum semantics and optionality"
    );
    assert!(
        code.contains("_event_id = coerce_uuid_param!(params.fetch('eventId'), 'eventId')"),
        "route should coerce path UUID parameters"
    );
    assert!(
        code.contains("_status = params.key?('status') ? coerce_enum_param!(params['status'], 'status', ['scheduled', 'cancelled']) : nil"),
        "route should coerce and validate optional enum query parameters"
    );

    Ok(())
}

#[test]
fn ruby_openapi_string_enums_use_dry_enum_types() -> Result<()> {
    let schema_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testing_data/openapi_schemas/complex_nested.json");

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    assert!(
        code.contains("attribute :type, Types::Strict::String.enum('startup', 'enterprise', 'nonprofit', 'educational', 'personal')"),
        "string enums should use dry-types enum constraints"
    );
    assert!(
        code.contains("attribute :priority, Types::Strict::String.enum('low', 'medium', 'high', 'critical')"),
        "additional enums should use dry-types enum constraints"
    );

    Ok(())
}

#[test]
fn ruby_openapi_generated_code_validates() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    let report = QualityValidator::new(TargetLanguage::Ruby)
        .validate_all(&code)
        .expect("ruby openapi validation should run");

    assert!(
        report.is_valid(),
        "generated Ruby OpenAPI code should validate cleanly: {report}"
    );

    Ok(())
}

#[test]
fn ruby_openapi_auth_service_example_validates() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let dto = DtoConfig {
        ruby: RubyDtoStyle::DrySchema,
        ..Default::default()
    };

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto)?;
    let report = QualityValidator::new(TargetLanguage::Ruby)
        .validate_all(&code)
        .expect("ruby openapi validation should run");

    assert!(
        report.is_valid(),
        "generated Ruby auth-service OpenAPI code should validate cleanly: {report}"
    );
    assert!(
        code.contains("attribute :keys, Types::Strict::Array.of(Types.Instance(ApiKeyListResponseKeysItem))"),
        "array-of-object properties should use generated named item models"
    );

    Ok(())
}

#[test]
fn elixir_openapi_generation_uses_spikard_router() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "inline_openapi.yaml", INLINE_OBJECT_OPENAPI);

    let code = generate_from_openapi(&schema_path, TargetLanguage::Elixir, &DtoConfig::default())?;

    assert!(
        code.contains("use Spikard.Router"),
        "expected Spikard.Router in generated output"
    );
    assert!(
        code.contains("post(\"/widgets\""),
        "expected Elixir route macro for POST endpoint"
    );
    assert!(
        code.contains("request_schema: @create_widget_request_schema"),
        "expected request schema metadata on generated route"
    );
    assert!(
        code.contains("response_schema: @create_widget_response_schema"),
        "expected response schema metadata on generated route"
    );

    Ok(())
}

#[test]
fn elixir_openapi_path_params_use_spikard_router_syntax() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(
        dir.path(),
        "openapi.yaml",
        r##"
openapi: 3.1.0
info:
  title: Users API
  version: "1.0.0"
paths:
  /users/{id}:
    get:
      operationId: getUser
      parameters:
        - in: path
          name: id
          required: true
          schema:
            type: string
      responses:
        "200":
          description: OK
          content:
            application/json:
              schema:
                type: object
                properties:
                  id:
                    type: string
                required:
                  - id
"##,
    );

    let code = generate_from_openapi(&schema_path, TargetLanguage::Elixir, &DtoConfig::default())?;
    assert!(code.contains("get(\"/users/:id\""), "expected :param route syntax");
    assert!(
        code.contains("_id = coerce_uuid_param!(Spikard.Request.get_path_param(request, \"id\"), \"id\")")
            || code.contains("_id = Spikard.Request.get_path_param(request, \"id\")"),
        "expected handler to bind path params through generated request access"
    );

    Ok(())
}

#[test]
fn elixir_openapi_handlers_coerce_semantic_parameters() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "route_semantics_openapi.yaml", ROUTE_SEMANTICS_OPENAPI);

    let code = generate_from_openapi(&schema_path, TargetLanguage::Elixir, &DtoConfig::default())?;
    assert!(
        code.contains("defp coerce_uuid_param!(value, name) do"),
        "expected UUID coercion helper in generated handler module"
    );
    assert!(
        code.contains("defp coerce_enum_param!(value, name, allowed) do"),
        "expected enum coercion helper in generated handler module"
    );
    assert!(
        code.contains(
            "_event_id = coerce_uuid_param!(Spikard.Request.get_path_param(request, \"eventId\"), \"eventId\")"
        ),
        "expected path params to be coerced through semantic helper"
    );
    assert!(
        code.contains(
            "_status =\n      coerce_enum_param!(Spikard.Request.get_query_param(request, \"status\"), \"status\", ["
        ),
        "expected query params to be coerced through semantic helper"
    );
    assert!(
        code.contains("\"scheduled\"") && code.contains("\"cancelled\""),
        "expected generated enum coercion to preserve allowed values"
    );

    Ok(())
}

#[test]
fn elixir_openapi_generated_code_validates() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let code = generate_from_openapi(&schema_path, TargetLanguage::Elixir, &DtoConfig::default())?;
    let report = QualityValidator::new(TargetLanguage::Elixir)
        .validate_all(&code)
        .expect("elixir openapi validation should run");

    assert!(
        report.is_valid(),
        "generated Elixir OpenAPI code should validate cleanly: {report}"
    );

    Ok(())
}

#[test]
fn rust_generation_uses_spikard_app() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let dto = DtoConfig::default();
    let code = generate_from_openapi(&schema_path, TargetLanguage::Rust, &dto)?;
    assert!(
        code.contains("use spikard::{App, AppError"),
        "expected Spikard App import in Rust output"
    );
    assert!(
        code.contains("app.route(get(\"/hello\")"),
        "expected route registration using Spikard builder"
    );
    Ok(())
}

#[test]
fn rust_openapi_auth_service_preserves_named_inline_and_union_request_bodies() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let code = generate_from_openapi(&schema_path, TargetLanguage::Rust, &DtoConfig::default())?;

    assert!(
        code.contains("pub struct GenerateApiKeyRequestBody"),
        "inline object request bodies should generate named Rust structs"
    );
    assert!(
        code.contains(".request_body::<GenerateApiKeyRequestBody>()"),
        "route metadata should reference the generated inline request body struct"
    );
    assert!(
        code.contains("pub enum IssueTokenRequestBody"),
        "oneOf request bodies should generate a named untagged enum"
    );
    assert!(
        code.contains("PasswordGrantRequest(PasswordGrantRequest)")
            && code.contains("ClientCredentialsGrantRequest(ClientCredentialsGrantRequest)")
            && code.contains("RefreshTokenGrantRequest(RefreshTokenGrantRequest)"),
        "union body enum should preserve component-backed variants"
    );
    assert!(
        code.contains(".request_body::<IssueTokenRequestBody>()"),
        "route metadata should reference the generated union request body enum"
    );

    Ok(())
}

#[test]
fn rust_openapi_auth_service_generates_named_nested_component_models() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let code = generate_from_openapi(&schema_path, TargetLanguage::Rust, &DtoConfig::default())?;

    assert!(
        code.contains("pub struct ApiKeyListResponseKeysItem"),
        "array item objects should generate named nested Rust structs"
    );
    assert!(
        code.contains("pub struct ApiKeyListResponsePagination"),
        "nested object properties should generate named Rust structs"
    );
    assert!(
        code.contains("pub keys: Option<Vec<ApiKeyListResponseKeysItem>>"),
        "parent model should reference generated array item structs"
    );
    assert!(
        code.contains("pub pagination: Option<ApiKeyListResponsePagination>"),
        "parent model should reference generated nested object structs"
    );

    Ok(())
}

#[test]
fn rust_openapi_auth_service_example_validates() -> Result<()> {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/auth-service.openapi.yaml");

    let code = generate_from_openapi(&schema_path, TargetLanguage::Rust, &DtoConfig::default())?;
    let report = QualityValidator::new(TargetLanguage::Rust)
        .validate_all(&code)
        .expect("rust openapi validation should run");

    assert!(
        report.is_valid(),
        "generated Rust auth-service OpenAPI code should validate cleanly: {report}"
    );

    Ok(())
}

#[test]
fn rust_openapi_string_formats_use_semantic_types() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "string_formats_openapi.yaml", STRING_FORMATS_OPENAPI);

    let code = generate_from_openapi(&schema_path, TargetLanguage::Rust, &DtoConfig::default())?;

    assert!(
        code.contains("pub id: uuid::Uuid"),
        "uuid schema fields should use uuid::Uuid"
    );
    assert!(
        code.contains("pub starts_at: chrono::DateTime<chrono::Utc>"),
        "date-time schema fields should use chrono::DateTime<Utc>"
    );
    assert!(
        code.contains("pub event_date: chrono::NaiveDate"),
        "date schema fields should use chrono::NaiveDate"
    );

    Ok(())
}

#[test]
fn asyncapi_fixture_generation_creates_files() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let fixtures_dir = dir.path().join("fixtures");

    let request = CodegenRequest {
        schema_path: schema_path.clone(),
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncFixtures {
            output: fixtures_dir.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("fixture generation should emit files"),
    };

    assert!(!files.is_empty(), "no fixtures generated");
    for asset in files {
        assert!(asset.path.exists(), "missing fixture {}", asset.path.display());
    }

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_python_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.py");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::Python,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("async def handle_websocket"),
        "expected websocket handler in generated app"
    );
    compile_python_file(&contents)?;

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_node_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.ts");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::TypeScript,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "Node test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("async function handleWebSocket"),
        "expected websocket handler in Node app"
    );

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_ruby_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.rb");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::Ruby,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "Ruby test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("def handle_websocket"),
        "expected websocket handler in Ruby app"
    );
    assert!(contents.contains("Faye::WebSocket"), "expected Faye WebSocket usage");

    Ok(())
}

#[test]
fn asyncapi_handler_generation_writes_rust_scaffold() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("handlers.rs");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncHandlers {
            language: TargetLanguage::Rust,
            output: output.clone(),
        },
        dto: None,
    };

    match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(_) => {
            let contents = fs::read_to_string(&output)?;
            assert!(
                contents.contains("use spikard::{App, WebSocketHandler};"),
                "expected WebSocket handler import in Rust scaffold"
            );
            assert!(
                contents.contains("app.websocket(\"/chat\", ChatWebSocketHandler);"),
                "expected websocket registration"
            );
        }
        CodegenOutcome::InMemory(_) => panic!("Rust handler generation should emit files"),
    }

    Ok(())
}

#[test]
fn asyncapi_handler_generation_writes_php_scaffold() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("handlers.php");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncHandlers {
            language: TargetLanguage::Php,
            output: output.clone(),
        },
        dto: None,
    };

    match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(_) => {
            let contents = fs::read_to_string(&output)?;
            assert!(contents.contains("<?php"), "expected PHP file header");
        }
        CodegenOutcome::InMemory(_) => panic!("PHP handler generation should emit files"),
    }

    Ok(())
}

fn assert_python_class_executes(class_name: &str, code: &str) -> Result<()> {
    let dir = tempdir()?;
    let stub_dir = create_python_stub_dir(dir.path())?;
    let module_path = dir.path().join("generated_app.py");
    fs::write(&module_path, code)?;

    let script = format!(
        r#"
import importlib.util
import sys
spec = importlib.util.spec_from_file_location("generated_app", r"{path}")
module = importlib.util.module_from_spec(spec)
sys.modules["generated_app"] = module
spec.loader.exec_module(module)
instance = getattr(module, "{class_name}")(message="hello")
assert instance.message == "hello"
"#,
        path = module_path.display(),
        class_name = class_name
    );

    let pythonpath = pythonpath_value(&stub_dir);
    let status = Command::new("uv")
        .args(["run", "python"])
        .env("PYTHONPATH", pythonpath)
        .arg("-c")
        .arg(script)
        .status()
        .expect("failed to run python");

    assert!(status.success(), "python execution failed");
    Ok(())
}

fn compile_python_file(code: &str) -> Result<()> {
    let dir = tempdir()?;
    let stub_dir = create_python_stub_dir(dir.path())?;
    let module_path = dir.path().join("async_app.py");
    fs::write(&module_path, code)?;
    let pythonpath = pythonpath_value(&stub_dir);
    let status = Command::new("uv")
        .args(["run", "python"])
        .env("PYTHONPATH", pythonpath)
        .arg("-m")
        .arg("py_compile")
        .arg(&module_path)
        .status()
        .expect("failed to run python");
    assert!(status.success(), "python compilation failed");
    Ok(())
}

fn pythonpath_value(stub_dir: &Path) -> OsString {
    let package_path = pythonpath_env();
    env::join_paths([stub_dir, package_path.as_path()]).expect("failed to build PYTHONPATH")
}

fn pythonpath_env() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/python")
        .canonicalize()
        .expect("failed to resolve python package path")
}

fn create_python_stub_dir(base: &Path) -> Result<PathBuf> {
    let root = base.join("py_stubs");
    let spikard_dir = root.join("spikard");
    fs::create_dir_all(&spikard_dir)?;
    fs::write(
        spikard_dir.join("__init__.py"),
        r#"
class _Param:
    def __class_getitem__(cls, item):
        return cls

Body = Path = Query = _Param

class Request:
    ...

class Spikard:
    def __call__(self, *args, **kwargs):
        pass

    def route(self, *args, **kwargs):
        def decorator(fn):
            return fn
        return decorator

def route(*args, **kwargs):
    def decorator(fn):
        return fn
    return decorator
"#,
    )?;

    fs::write(
        root.join("_spikard.py"),
        r#"
class Response:
    ...

        class StreamingResponse:
            ...
"#,
    )?;

    fs::write(
        root.join("msgspec.py"),
        r#"
class Struct:
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)

    def __class_getitem__(cls, item):
        return cls
"#,
    )?;

    Ok(root)
}
