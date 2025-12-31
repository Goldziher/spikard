use super::asyncapi::{Protocol, parse_asyncapi_schema};
use super::asyncapi::{
    generate_nodejs_handler_app, generate_nodejs_test_app, generate_php_handler_app, generate_python_handler_app,
    generate_python_test_app, generate_ruby_handler_app, generate_ruby_test_app, generate_rust_handler_app,
};
use super::openrpc::{
    generate_php_handler_app as generate_openrpc_php_handler,
    generate_python_handler_app as generate_openrpc_python_handler,
    generate_ruby_handler_app as generate_openrpc_ruby_handler,
    generate_typescript_handler_app as generate_openrpc_typescript_handler, parse_openrpc_schema,
};
use super::{DtoConfig, TargetLanguage, detect_primary_protocol, generate_fixtures};
use crate::codegen::generate_from_openapi;
use anyhow::{Context, Result, bail};
use asyncapiv3::spec::AsyncApiV3Spec;
use heck::ToKebabCase;
use std::fs;
use std::path::{Path, PathBuf};

/// Code generation schema families supported by the CLI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaKind {
    OpenApi,
    AsyncApi,
    OpenRpc,
    GraphQL,
    Protobuf,
}

/// Type of artifact to generate for a schema
#[derive(Clone)]
pub enum CodegenTargetKind {
    /// Generate server handler code for a language (OpenAPI today)
    Server {
        language: TargetLanguage,
        output: Option<PathBuf>,
    },
    /// Generate AsyncAPI fixtures (SSE/WebSocket)
    AsyncFixtures { output: PathBuf },
    /// Generate AsyncAPI test application for a language
    AsyncTestApp { language: TargetLanguage, output: PathBuf },
    /// Generate AsyncAPI handler scaffolding for a language
    AsyncHandlers { language: TargetLanguage, output: PathBuf },
    /// Generate fixtures + test applications for all AsyncAPI languages
    AsyncAll { output: PathBuf },
    /// Generate JSON-RPC handler scaffolding for a language
    JsonRpcHandlers { language: TargetLanguage, output: PathBuf },
    /// Generate GraphQL types, resolvers, or schema for a language
    GraphQL {
        language: TargetLanguage,
        output: PathBuf,
        target: String,
    },
    /// Generate Protobuf messages and gRPC services
    Protobuf {
        language: TargetLanguage,
        output: PathBuf,
        target: String,
    },
}

/// Request executed by the code generation engine
#[derive(Debug, Clone)]
pub struct CodegenRequest {
    pub schema_path: PathBuf,
    pub schema_kind: SchemaKind,
    pub target: CodegenTargetKind,
    pub dto: Option<DtoConfig>,
}

/// Represents an asset emitted by the code generation engine
#[derive(Debug, Clone)]
pub struct GeneratedAsset {
    pub path: PathBuf,
    pub description: String,
}

/// Output of the engine run
#[derive(Debug, Clone)]
pub enum CodegenOutcome {
    /// Generated code that should be printed to stdout (no file requested)
    InMemory(String),
    /// Files that were written to disk
    Files(Vec<GeneratedAsset>),
}

/// Code generation runtime orchestrating schema parsing and artifact generation
pub struct CodegenEngine;

impl CodegenEngine {
    pub fn execute(request: CodegenRequest) -> Result<CodegenOutcome> {
        match (&request.schema_kind, &request.target) {
            (SchemaKind::OpenApi, CodegenTargetKind::Server { language, output }) => {
                let dto = request.dto.clone().unwrap_or_default();
                let code = generate_from_openapi(&request.schema_path, *language, &dto, output.as_deref())?;

                if let Some(path) = output {
                    Ok(CodegenOutcome::Files(vec![GeneratedAsset {
                        path: path.clone(),
                        description: format!("{} server handlers", language_name(*language)),
                    }]))
                } else {
                    Ok(CodegenOutcome::InMemory(code))
                }
            }
            (SchemaKind::AsyncApi, CodegenTargetKind::AsyncFixtures { output }) => {
                let spec = parse_asyncapi_schema(&request.schema_path)
                    .context("Failed to parse AsyncAPI schema for fixture generation")?;
                let protocol = detect_primary_protocol(&spec)?;
                let paths = Self::generate_asyncapi_fixtures(&spec, protocol, output)?;
                Ok(CodegenOutcome::Files(paths))
            }
            (SchemaKind::AsyncApi, CodegenTargetKind::AsyncTestApp { language, output }) => {
                let spec = parse_asyncapi_schema(&request.schema_path)
                    .context("Failed to parse AsyncAPI schema for test app generation")?;
                let protocol = detect_primary_protocol(&spec)?;
                let asset = Self::generate_asyncapi_app(&spec, protocol, *language, output)?;
                Ok(CodegenOutcome::Files(vec![asset]))
            }
            (SchemaKind::AsyncApi, CodegenTargetKind::AsyncHandlers { language, output }) => {
                let spec = parse_asyncapi_schema(&request.schema_path)
                    .context("Failed to parse AsyncAPI schema for handler generation")?;
                let protocol = detect_primary_protocol(&spec)?;
                let asset = Self::generate_asyncapi_handler(&spec, protocol, *language, output)?;
                Ok(CodegenOutcome::Files(vec![asset]))
            }
            (SchemaKind::AsyncApi, CodegenTargetKind::AsyncAll { output }) => {
                let spec = parse_asyncapi_schema(&request.schema_path)
                    .context("Failed to parse AsyncAPI schema for all-assets generation")?;
                let protocol = detect_primary_protocol(&spec)?;
                let assets = Self::generate_asyncapi_bundle(&spec, protocol, output)?;
                Ok(CodegenOutcome::Files(assets))
            }
            (SchemaKind::OpenRpc, CodegenTargetKind::JsonRpcHandlers { language, output }) => {
                let spec = parse_openrpc_schema(&request.schema_path)
                    .context("Failed to parse OpenRPC schema for handler generation")?;
                let asset = Self::generate_openrpc_handler(&spec, *language, output)?;
                Ok(CodegenOutcome::Files(vec![asset]))
            }
            (
                SchemaKind::GraphQL,
                CodegenTargetKind::GraphQL {
                    language,
                    output,
                    target,
                },
            ) => {
                let assets = Self::generate_graphql_code(&request.schema_path, *language, output, target)
                    .context("Failed to generate code from GraphQL schema")?;
                Ok(CodegenOutcome::Files(assets))
            }
            (SchemaKind::Protobuf, CodegenTargetKind::Protobuf { language, output, target }) => {
                let schema = super::protobuf::parse_proto_schema(&request.schema_path)?;

                // Parse target string to ProtobufTarget enum
                let proto_target = match target.as_str() {
                    "all" => super::protobuf::generators::ProtobufTarget::All,
                    "messages" => super::protobuf::generators::ProtobufTarget::Messages,
                    "services" => super::protobuf::generators::ProtobufTarget::Services,
                    _ => bail!("Invalid protobuf target: {}. Use 'all', 'messages', or 'services'", target),
                };

                let code = match language {
                    TargetLanguage::Python => super::protobuf::generate_python_protobuf(&schema, &proto_target)?,
                    TargetLanguage::TypeScript => super::protobuf::generate_typescript_protobuf(&schema, &proto_target)?,
                    TargetLanguage::Ruby => super::protobuf::generate_ruby_protobuf(&schema, &proto_target)?,
                    TargetLanguage::Php => super::protobuf::generate_php_protobuf(&schema, &proto_target)?,
                    TargetLanguage::Rust => bail!("Rust protobuf generation not yet implemented"),
                };

                Ok(CodegenOutcome::Files(vec![Self::write_asset(
                    output,
                    format!("{} Protobuf code", language_name(*language)),
                    &code,
                )?]))
            }
            _ => bail!(
                "Unsupported schema/target combination: {:?} -> {:?}",
                request.schema_kind,
                request.target
            ),
        }
    }

    fn generate_asyncapi_fixtures(
        spec: &AsyncApiV3Spec,
        protocol: Protocol,
        output: &Path,
    ) -> Result<Vec<GeneratedAsset>> {
        let fixture_paths = generate_fixtures(spec, output, protocol)?;

        Ok(fixture_paths
            .into_iter()
            .map(|path| GeneratedAsset {
                description: format!("{} fixture", protocol.as_str()),
                path,
            })
            .collect())
    }

    fn generate_asyncapi_app(
        spec: &AsyncApiV3Spec,
        protocol: Protocol,
        language: TargetLanguage,
        output: &Path,
    ) -> Result<GeneratedAsset> {
        let code = match language {
            TargetLanguage::Python => generate_python_test_app(spec, protocol)?,
            TargetLanguage::TypeScript => generate_nodejs_test_app(spec, protocol)?,
            TargetLanguage::Ruby => generate_ruby_test_app(spec, protocol)?,
            other => {
                bail!("{:?} is not supported for AsyncAPI test apps", other);
            }
        };

        Self::write_asset(output, format!("{} AsyncAPI test app", language_name(language)), code)
    }

    fn generate_asyncapi_handler(
        spec: &AsyncApiV3Spec,
        protocol: Protocol,
        language: TargetLanguage,
        output: &Path,
    ) -> Result<GeneratedAsset> {
        let code = match language {
            TargetLanguage::Python => generate_python_handler_app(spec, protocol)?,
            TargetLanguage::TypeScript => generate_nodejs_handler_app(spec, protocol)?,
            TargetLanguage::Ruby => generate_ruby_handler_app(spec, protocol)?,
            TargetLanguage::Rust => generate_rust_handler_app(spec, protocol)?,
            TargetLanguage::Php => generate_php_handler_app(spec, protocol)?,
        };

        Self::write_asset(output, format!("{} AsyncAPI handler", language_name(language)), code)
    }

    fn generate_asyncapi_bundle(
        spec: &AsyncApiV3Spec,
        protocol: Protocol,
        output: &Path,
    ) -> Result<Vec<GeneratedAsset>> {
        let mut assets = Vec::new();

        let fixtures_dir = output.join("testing_data");
        assets.extend(Self::generate_asyncapi_fixtures(spec, protocol, &fixtures_dir)?);

        let app_dir = output.join("apps");
        fs::create_dir_all(&app_dir).with_context(|| format!("Failed to create {}", app_dir.display()))?;
        let base_name = spec.info.title.to_kebab_case();

        let python_asset = Self::generate_asyncapi_app(
            spec,
            protocol,
            TargetLanguage::Python,
            &app_dir.join(format!("{base_name}-asyncapi.py")),
        )?;
        assets.push(python_asset);

        let node_asset = Self::generate_asyncapi_app(
            spec,
            protocol,
            TargetLanguage::TypeScript,
            &app_dir.join(format!("{base_name}-asyncapi.ts")),
        )?;
        assets.push(node_asset);

        let ruby_asset = Self::generate_asyncapi_app(
            spec,
            protocol,
            TargetLanguage::Ruby,
            &app_dir.join(format!("{base_name}-asyncapi.rb")),
        )?;
        assets.push(ruby_asset);

        Ok(assets)
    }

    fn generate_openrpc_handler(
        spec: &super::openrpc::spec_parser::OpenRpcSpec,
        language: TargetLanguage,
        output: &Path,
    ) -> Result<GeneratedAsset> {
        let code = match language {
            TargetLanguage::Python => generate_openrpc_python_handler(spec)?,
            TargetLanguage::TypeScript => generate_openrpc_typescript_handler(spec)?,
            TargetLanguage::Ruby => generate_openrpc_ruby_handler(spec)?,
            TargetLanguage::Php => generate_openrpc_php_handler(spec)?,
            other => {
                bail!("{:?} is not supported for OpenRPC handler generation", other);
            }
        };

        Self::write_asset(output, format!("{} JSON-RPC handlers", language_name(language)), code)
    }

    fn generate_graphql_code(
        schema_path: &Path,
        language: TargetLanguage,
        output: &Path,
        target: &str,
    ) -> Result<Vec<GeneratedAsset>> {
        // Read the GraphQL schema file
        let schema_content = fs::read_to_string(schema_path)
            .with_context(|| format!("Failed to read GraphQL schema: {}", schema_path.display()))?;

        // Generate code based on language
        let code = match language {
            TargetLanguage::Python => super::graphql::generate_python_graphql(&schema_content, target)?,
            TargetLanguage::TypeScript => super::graphql::generate_typescript_graphql(&schema_content, target)?,
            TargetLanguage::Rust => super::graphql::generate_rust_graphql(&schema_content, target)?,
            TargetLanguage::Ruby => super::graphql::generate_ruby_graphql(&schema_content, target)?,
            TargetLanguage::Php => super::graphql::generate_php_graphql(&schema_content, target)?,
        };

        // For Ruby, also generate RBS type signatures when appropriate
        let mut assets = vec![Self::write_asset(
            output,
            format!("{} GraphQL code", language_name(language)),
            &code,
        )?];

        if language == TargetLanguage::Ruby && (target == "all" || target == "types" || target == "schema") {
            use super::graphql::generators::GraphQLGenerator;
            use super::graphql::generators::ruby::RubyGenerator;
            use super::graphql::parse_graphql_sdl_string;

            let parsed_schema = parse_graphql_sdl_string(&schema_content)?;
            let generator = RubyGenerator;
            let rbs_code = generator.generate_type_signatures(&parsed_schema)?;

            // Determine RBS output path (replace .rb extension with .rbs)
            let rbs_output = output.with_extension("rbs");

            assets.push(Self::write_asset(
                &rbs_output,
                format!("{} GraphQL RBS types", language_name(language)),
                &rbs_code,
            )?);
        }

        Ok(assets)
    }

    fn write_asset(path: &Path, description: impl Into<String>, content: impl AsRef<[u8]>) -> Result<GeneratedAsset> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;
        }

        fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;

        Ok(GeneratedAsset {
            path: path.to_path_buf(),
            description: description.into(),
        })
    }
}

fn language_name(language: TargetLanguage) -> &'static str {
    match language {
        TargetLanguage::Python => "Python",
        TargetLanguage::TypeScript => "Node.js",
        TargetLanguage::Rust => "Rust",
        TargetLanguage::Ruby => "Ruby",
        TargetLanguage::Php => "PHP",
    }
}

impl std::fmt::Debug for CodegenTargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenTargetKind::Server { language, .. } => f
                .debug_struct("Server")
                .field("language", language)
                .finish_non_exhaustive(),
            CodegenTargetKind::AsyncFixtures { output } => {
                f.debug_struct("AsyncFixtures").field("output", output).finish()
            }
            CodegenTargetKind::AsyncTestApp { language, output } => f
                .debug_struct("AsyncTestApp")
                .field("language", language)
                .field("output", output)
                .finish(),
            CodegenTargetKind::AsyncHandlers { language, output } => f
                .debug_struct("AsyncHandlers")
                .field("language", language)
                .field("output", output)
                .finish(),
            CodegenTargetKind::AsyncAll { output } => f.debug_struct("AsyncAll").field("output", output).finish(),
            CodegenTargetKind::JsonRpcHandlers { language, output } => f
                .debug_struct("JsonRpcHandlers")
                .field("language", language)
                .field("output", output)
                .finish(),
            CodegenTargetKind::GraphQL {
                language,
                output,
                target,
            } => f
                .debug_struct("GraphQL")
                .field("language", language)
                .field("output", output)
                .field("target", target)
                .finish(),
            CodegenTargetKind::Protobuf {
                language,
                output,
                target,
            } => f
                .debug_struct("Protobuf")
                .field("language", language)
                .field("output", output)
                .field("target", target)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn write_minimal_openapi_schema(path: &Path) {
        let spec = r#"
{
  "openapi": "3.0.3",
  "info": { "title": "Demo", "version": "1.0.0" },
  "paths": {
    "/ping": {
      "get": {
        "operationId": "ping",
        "responses": {
          "200": {
            "description": "ok",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": { "message": { "type": "string" } },
                  "required": ["message"]
                }
              }
            }
          }
        }
      }
    }
  }
}
"#;
        fs::write(path, spec).unwrap();
    }

    #[test]
    fn generates_openapi_code_in_memory_when_no_output_path() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("openapi.json");
        write_minimal_openapi_schema(&schema_path);

        let outcome = CodegenEngine::execute(CodegenRequest {
            schema_path,
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::Server {
                language: TargetLanguage::Python,
                output: None,
            },
            dto: None,
        })
        .unwrap();

        match outcome {
            CodegenOutcome::InMemory(code) => {
                assert!(code.contains("Generated by Spikard OpenAPI code generator"));
                assert!(code.contains("ping"));
            }
            other => panic!("expected in-memory output, got {other:?}"),
        }
    }

    #[test]
    fn generates_openapi_code_to_file_when_output_path_provided() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("openapi.json");
        write_minimal_openapi_schema(&schema_path);

        let output_path = dir.path().join("generated.py");
        let outcome = CodegenEngine::execute(CodegenRequest {
            schema_path,
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::Server {
                language: TargetLanguage::Python,
                output: Some(output_path.clone()),
            },
            dto: None,
        })
        .unwrap();

        match outcome {
            CodegenOutcome::Files(assets) => {
                assert_eq!(assets.len(), 1);
                assert_eq!(assets[0].path, output_path);
                assert!(assets[0].description.contains("Python"));
                assert!(
                    fs::read_to_string(&assets[0].path)
                        .unwrap()
                        .contains("Generated by Spikard OpenAPI code generator")
                );
            }
            other => panic!("expected file output, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unsupported_schema_target_combinations() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("openapi.json");
        write_minimal_openapi_schema(&schema_path);

        let err = CodegenEngine::execute(CodegenRequest {
            schema_path,
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::AsyncFixtures {
                output: dir.path().join("out"),
            },
            dto: None,
        })
        .unwrap_err();

        assert!(err.to_string().contains("Unsupported schema/target combination"));
    }

    #[test]
    fn generates_openrpc_handlers_to_file() {
        let dir = tempdir().unwrap();
        let schema_path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/user-api.openrpc.json");

        let output_path = dir.path().join("handlers.ts");
        let outcome = CodegenEngine::execute(CodegenRequest {
            schema_path,
            schema_kind: SchemaKind::OpenRpc,
            target: CodegenTargetKind::JsonRpcHandlers {
                language: TargetLanguage::TypeScript,
                output: output_path.clone(),
            },
            dto: None,
        })
        .unwrap();

        match outcome {
            CodegenOutcome::Files(assets) => {
                assert_eq!(assets.len(), 1);
                assert_eq!(assets[0].path, output_path);
                let contents = fs::read_to_string(&assets[0].path).unwrap();
                assert!(contents.contains("handleJsonRpcCall"));
            }
            other => panic!("expected file output, got {other:?}"),
        }
    }

    #[test]
    fn generates_protobuf_python_code_to_file() {
        let dir = tempdir().unwrap();
        let schema_path = dir.path().join("test.proto");

        // Write a minimal proto3 schema
        let proto_schema = r#"syntax = "proto3";

package test;

message TestMessage {
  string id = 1;
  string name = 2;
}
"#;
        fs::write(&schema_path, proto_schema).unwrap();

        let output_path = dir.path().join("test_pb.py");
        let outcome = CodegenEngine::execute(CodegenRequest {
            schema_path,
            schema_kind: SchemaKind::Protobuf,
            target: CodegenTargetKind::Protobuf {
                language: TargetLanguage::Python,
                output: output_path.clone(),
                target: "all".to_string(),
            },
            dto: None,
        })
        .unwrap();

        match outcome {
            CodegenOutcome::Files(assets) => {
                assert_eq!(assets.len(), 1);
                assert_eq!(assets[0].path, output_path);
                let contents = fs::read_to_string(&assets[0].path).unwrap();
                assert!(contents.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
                assert!(contents.contains("from google.protobuf import message"));
                assert!(contents.contains("Package: test"));
            }
            other => panic!("expected file output, got {other:?}"),
        }
    }
}
