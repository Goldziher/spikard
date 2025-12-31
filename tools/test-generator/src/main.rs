//! Test Generator
//!
//! Internal tool for generating test infrastructure from fixtures.
//! Generates test applications and test suites for Rust, Python, and TypeScript.

#![allow(clippy::all, dead_code, unused_variables)]

mod asyncapi;
mod background;
mod codegen_utils;
mod dependencies;
mod graphql;
mod graphql_tests;
mod grpc;
mod jsonrpc;
mod middleware;
mod node_app;
mod node_tests;
mod php_app;
mod php_tests;
mod python_app;
mod python_tests;
mod ruby_app;
mod ruby_tests;
mod ruby_utils;
mod rust_app;
mod rust_tests;
mod streaming;
mod ts_target;

use anyhow::{Context, Result};
use clap::Parser;
use spikard_codegen::openapi::{OpenApiOptions, fixtures_to_openapi, load_fixtures_from_dir};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate test infrastructure from fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Generate OpenAPI spec from fixtures
    Openapi {
        /// Fixtures directory
        #[arg(long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output file
        #[arg(long, short = 'o')]
        output: PathBuf,

        /// API title
        #[arg(long, default_value = "Test API")]
        title: String,

        /// API version
        #[arg(long, default_value = "1.0.0")]
        version: String,
    },

    /// Generate test suite for a language
    Tests {
        /// Target language
        #[arg(long, value_parser = ["rust", "python", "typescript", "node", "ruby", "wasm", "php", "deno", "cloudflare"])]
        lang: String,

        /// Fixtures directory
        #[arg(long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output directory
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Openapi {
            fixtures,
            output,
            title,
            version,
        } => generate_openapi(fixtures, output, title, version)?,
        Commands::Tests { lang, fixtures, output } => generate_tests(&lang, fixtures, output)?,
    }

    Ok(())
}

fn generate_openapi(fixtures_dir: PathBuf, output: PathBuf, title: String, version: String) -> Result<()> {
    println!("Loading fixtures from {}...", fixtures_dir.display());
    let fixtures = load_fixtures_from_dir(&fixtures_dir).context("Failed to load fixtures")?;

    println!("Found {} fixtures", fixtures.len());

    let options = OpenApiOptions {
        title,
        version,
        description: Some("Generated from test fixtures".to_string()),
    };

    let spec = fixtures_to_openapi(fixtures, options).context("Failed to generate OpenAPI spec")?;

    let yaml = serde_saphyr::to_string(&spec).context("Failed to serialize OpenAPI spec")?;

    std::fs::write(&output, yaml).context("Failed to write OpenAPI spec")?;

    println!("✓ OpenAPI spec generated: {}", output.display());
    Ok(())
}

fn generate_tests(lang: &str, fixtures: PathBuf, output: PathBuf) -> Result<()> {
    println!("Generating {} tests to {}...", lang, output.display());

    match lang {
        "rust" => {
            rust_app::generate_rust_app(&fixtures, &output)?;
            rust_tests::generate_rust_tests(&fixtures, &output)?;
        }
        "python" => {
            python_app::generate_python_app(&fixtures, &output)?;
            python_tests::generate_python_tests(&fixtures, &output)?;

            println!("Running ruff fix on generated Python code...");
            let ruff_fix_status = std::process::Command::new("uv")
                .args(["run", "ruff", "check", "--fix", "--unsafe-fixes"])
                .arg(&output)
                .status()
                .context("Failed to run ruff fix")?;

            if !ruff_fix_status.success() {
                eprintln!(
                    "Warning: ruff fix had issues (exit code {})",
                    ruff_fix_status.code().unwrap_or(-1)
                );
            }

            println!("Running ruff format on generated Python code...");
            let ruff_format_status = std::process::Command::new("uv")
                .args(["run", "ruff", "format"])
                .arg(&output)
                .status()
                .context("Failed to run ruff format")?;

            if !ruff_format_status.success() {
                eprintln!(
                    "Warning: ruff format had issues (exit code {})",
                    ruff_format_status.code().unwrap_or(-1)
                );
            }
        }
        "typescript" => {
            println!("TODO: Generate TypeScript/Vitest test suite");
        }
        "node" => {
            node_app::generate_node_app(&fixtures, &output, &ts_target::NODE_TARGET)?;
            node_tests::generate_node_tests(&fixtures, &output, &ts_target::NODE_TARGET)?;
            graphql_tests::generate_graphql_tests(&fixtures, &output, &ts_target::NODE_TARGET)?;
        }
        "wasm" => {
            node_app::generate_node_app(&fixtures, &output, &ts_target::WASM_TARGET)?;
            node_tests::generate_node_tests(&fixtures, &output, &ts_target::WASM_TARGET)?;
            graphql_tests::generate_graphql_tests(&fixtures, &output, &ts_target::WASM_TARGET)?;
        }
        "deno" => {
            node_app::generate_node_app(&fixtures, &output, &ts_target::DENO_TARGET)?;
            node_tests::generate_node_tests(&fixtures, &output, &ts_target::DENO_TARGET)?;
            graphql_tests::generate_graphql_tests(&fixtures, &output, &ts_target::DENO_TARGET)?;
        }
        "cloudflare" => {
            node_app::generate_node_app(&fixtures, &output, &ts_target::CLOUDFLARE_TARGET)?;
            node_tests::generate_node_tests(&fixtures, &output, &ts_target::CLOUDFLARE_TARGET)?;
            graphql_tests::generate_graphql_tests(&fixtures, &output, &ts_target::CLOUDFLARE_TARGET)?;
        }
        "ruby" => {
            ruby_app::generate_ruby_app(&fixtures, &output)?;
            ruby_tests::generate_ruby_tests(&fixtures, &output)?;
        }
        "php" => {
            php_app::generate_php_app(&fixtures, &output)?;
            php_tests::generate_php_tests(&fixtures, &output)?;
        }
        _ => unreachable!("Invalid language"),
    }

    println!("✓ {} tests generated successfully", lang);
    Ok(())
}
