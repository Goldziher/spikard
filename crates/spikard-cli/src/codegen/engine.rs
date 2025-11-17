use super::asyncapi::{Protocol, parse_asyncapi_schema};
use super::asyncapi::{
    generate_nodejs_handler_app, generate_nodejs_test_app, generate_python_handler_app, generate_python_test_app,
    generate_ruby_handler_app, generate_ruby_test_app,
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
            _ => anyhow::bail!(
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
                anyhow::bail!("{:?} is not supported for AsyncAPI test apps", other);
            }
        };

        if let Some(parent) = output.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;
        }

        fs::write(output, &code).with_context(|| format!("Failed to write {} test app", language_name(language)))?;

        Ok(GeneratedAsset {
            path: output.to_path_buf(),
            description: format!("{} AsyncAPI test app", language_name(language)),
        })
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
            other => bail!("{:?} is not supported for AsyncAPI handler generation", other),
        };

        if let Some(parent) = output.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;
        }

        fs::write(output, &code).with_context(|| format!("Failed to write handler file {}", output.display()))?;

        Ok(GeneratedAsset {
            path: output.to_path_buf(),
            description: format!("{} AsyncAPI handler", language_name(language)),
        })
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
        }
    }
}
