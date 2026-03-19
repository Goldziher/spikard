//! Shared application layer for CLI and MCP operations.

use crate::codegen::{
    CodegenEngine, CodegenOutcome, CodegenRequest, GeneratedAsset, PhpDtoGenerator, detect_primary_protocol,
    parse_asyncapi_schema,
};
use crate::init::{InitEngine, InitRequest, InitResponse};
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Result of AsyncAPI validation exposed to both CLI and MCP surfaces.
#[derive(Debug, Clone, Serialize)]
pub struct AsyncApiValidationSummary {
    pub spec_version: String,
    pub title: String,
    pub api_version: String,
    pub primary_protocol: String,
    pub channel_count: usize,
}

/// Human-readable framework capability summary for agents and CLI users.
#[derive(Debug, Clone, Serialize)]
pub struct FeatureSummary {
    pub rust_core: bool,
    pub language_bindings: Vec<LanguageBinding>,
    pub documentation_url: String,
}

/// Install and usage hint for a supported binding.
#[derive(Debug, Clone, Serialize)]
pub struct LanguageBinding {
    pub name: String,
    pub install_hint: String,
    pub usage_hint: String,
}

/// Execute a project initialization request.
pub fn init_project(request: InitRequest) -> Result<InitResponse> {
    InitEngine::execute(request)
}

/// Execute validated code generation.
pub fn execute_codegen(request: CodegenRequest) -> Result<CodegenOutcome> {
    CodegenEngine::execute_validated(request)
}

/// Execute code generation without validator passes.
pub fn execute_codegen_unvalidated(request: CodegenRequest) -> Result<CodegenOutcome> {
    CodegenEngine::execute(request)
}

/// Generate the PHP DTO helper classes.
pub fn generate_php_dto(output: &Path) -> Result<Vec<GeneratedAsset>> {
    let generator = PhpDtoGenerator::new();
    let generated = generator.generate_all().context("Failed to generate PHP DTOs")?;

    fs::create_dir_all(output).with_context(|| format!("Failed to create output directory: {}", output.display()))?;

    let mut assets = Vec::with_capacity(generated.len());
    for (filename, code) in generated {
        let file_path = output.join(&filename);
        fs::write(&file_path, code).with_context(|| format!("Failed to write DTO file: {}", file_path.display()))?;
        assets.push(GeneratedAsset {
            path: file_path,
            description: format!("PHP DTO class {}", filename),
        });
    }

    Ok(assets)
}

/// Validate an AsyncAPI schema and return the structured summary.
pub fn validate_asyncapi_schema(schema: &Path) -> Result<AsyncApiValidationSummary> {
    let spec = parse_asyncapi_schema(schema).context("Failed to parse AsyncAPI schema")?;
    let protocol = detect_primary_protocol(&spec)?;

    Ok(AsyncApiValidationSummary {
        spec_version: "3.0.0".to_string(),
        title: spec.info.title,
        api_version: spec.info.version,
        primary_protocol: format!("{protocol:?}"),
        channel_count: spec.channels.len(),
    })
}

/// Return the current feature summary shown by the CLI and exposed via MCP.
#[must_use]
pub fn feature_summary() -> FeatureSummary {
    FeatureSummary {
        rust_core: true,
        language_bindings: vec![
            LanguageBinding {
                name: "Rust".to_string(),
                install_hint: "cargo add spikard".to_string(),
                usage_hint: "cargo run".to_string(),
            },
            LanguageBinding {
                name: "Python".to_string(),
                install_hint: "pip install spikard".to_string(),
                usage_hint: "python server.py".to_string(),
            },
            LanguageBinding {
                name: "TypeScript".to_string(),
                install_hint: "npm install spikard".to_string(),
                usage_hint: "node server.js".to_string(),
            },
            LanguageBinding {
                name: "Ruby".to_string(),
                install_hint: "gem install spikard".to_string(),
                usage_hint: "ruby app.rb".to_string(),
            },
            LanguageBinding {
                name: "PHP".to_string(),
                install_hint: "composer require spikard/spikard".to_string(),
                usage_hint: "php src/App.php".to_string(),
            },
            LanguageBinding {
                name: "Elixir".to_string(),
                install_hint: "mix deps.get".to_string(),
                usage_hint: "iex -S mix".to_string(),
            },
        ],
        documentation_url: "https://spikard.dev".to_string(),
    }
}
