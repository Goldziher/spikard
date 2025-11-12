use anyhow::{Context, Result};
use heck::ToSnakeCase;
use spikard_cli::codegen::{
    Protocol, detect_primary_protocol, generate_fixtures, generate_nodejs_test_app, generate_python_test_app,
    generate_ruby_test_app, parse_asyncapi_schema,
};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Generate AsyncAPI-driven SSE/WebSocket tests for supported languages.
pub fn generate_asyncapi_tests(lang: &str, output_dir: &Path) -> Result<()> {
    let specs_dir = Path::new("examples/asyncapi");
    if !specs_dir.exists() {
        return Ok(());
    }

    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory {}", output_dir.display()))?;

    for entry in fs::read_dir(specs_dir).context("Failed to read AsyncAPI examples directory")? {
        let entry = entry?;
        let path = entry.path();
        if !is_schema_file(&path) {
            continue;
        }

        let spec = parse_asyncapi_schema(&path)
            .with_context(|| format!("Failed to parse AsyncAPI schema {}", path.display()))?;
        let protocol = detect_primary_protocol(&spec)
            .with_context(|| format!("Failed to detect protocol for {}", path.display()))?;

        // Only generate tests for the protocols we currently support.
        if !matches!(protocol, Protocol::Sse | Protocol::WebSocket) {
            continue;
        }

        // Ensure fixtures for this spec are up to date.
        generate_fixtures(&spec, Path::new("testing_data"), protocol)
            .with_context(|| format!("Failed to generate fixtures for {}", path.display()))?;

        let slug = spec_slug(&path);
        let filename = match lang {
            "ruby" => format!("{slug}_test.rb"),
            "python" => format!("{slug}_test.py"),
            "node" => format!("{slug}_test.ts"),
            _ => continue,
        };

        let output_file = output_dir.join(filename);

        let code = match lang {
            "ruby" => generate_ruby_test_app(&spec, protocol)?,
            "python" => generate_python_test_app(&spec, protocol)?,
            "node" => generate_nodejs_test_app(&spec, protocol)?,
            _ => continue,
        };

        fs::write(&output_file, code)
            .with_context(|| format!("Failed to write AsyncAPI test app {}", output_file.display()))?;
    }

    Ok(())
}

fn is_schema_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(OsStr::to_str),
        Some("yaml") | Some("yml") | Some("json")
    )
}

fn spec_slug(path: &Path) -> String {
    path.file_stem()
        .and_then(OsStr::to_str)
        .map(|s| s.to_snake_case())
        .unwrap_or_else(|| "asyncapi".to_string())
}
