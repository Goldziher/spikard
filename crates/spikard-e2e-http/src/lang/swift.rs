//! Swift HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern file for spikard's Swift e2e suite:
//! - `app_harness.swift` (or `main.swift` in Harness target) — spawns the SUT as an HTTP server
//!
//! The shared e2e files (Package.swift, TestHelpers.swift, per-category test files) stay
//! generic in alef. Only the server-spawn harness executable lives here.
//!
//! Sources (alef `src/e2e/codegen/swift/`):
//! - `project.rs::render_app_harness`
#![allow(dead_code)]

use alef::GeneratedFile;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use heck::ToUpperCamelCase;
use minijinja::{Environment, context};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Template environment
// ---------------------------------------------------------------------------

/// Build the private template environment holding the Swift HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "swift/app_harness.swift.jinja".to_owned(),
        include_str!("../../templates/swift/app_harness.swift.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

/// Render a named template from the local environment.
fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// App harness renderer (ported from alef `swift/project.rs::render_app_harness`).
// ---------------------------------------------------------------------------

/// Split a string into UTF-8-safe chunks of max ~30000 bytes.
/// Each chunk is wrapped as a Swift raw string literal with a safe delimiter level.
fn chunk_fixtures_for_swift(json: &str) -> Vec<String> {
    const CHUNK_SIZE: usize = 30000;

    // Find the longest consecutive run of `#` characters in the JSON.
    // Swift raw string delimiters use `#"..."#`, `##"..."##`, etc., where
    // the number of `#` on both sides must match and exceed any consecutive `#`
    // run inside the string content.
    let mut max_hash_run = 0;
    let mut current_run = 0;
    for c in json.chars() {
        if c == '#' {
            current_run += 1;
            max_hash_run = max_hash_run.max(current_run);
        } else {
            current_run = 0;
        }
    }
    // Use one more `#` than the longest run to guarantee no collision.
    let delimiter_level = max_hash_run + 1;
    let delimiter = "#".repeat(delimiter_level);

    let mut chunks = Vec::new();

    // Split at UTF-8 char boundaries (not byte boundaries) to avoid breaking
    // multi-byte UTF-8 sequences.
    let mut current_chunk = String::new();
    for c in json.chars() {
        // If adding this char would exceed CHUNK_SIZE, save current chunk and start new one.
        if !current_chunk.is_empty() && current_chunk.len() + c.len_utf8() > CHUNK_SIZE {
            // Wrap the chunk in a raw string literal.
            chunks.push(format!("{0}\"{1}\"{0}", delimiter, current_chunk));
            current_chunk.clear();
        }
        current_chunk.push(c);
    }

    // Don't forget the last chunk.
    if !current_chunk.is_empty() {
        chunks.push(format!("{0}\"{1}\"{0}", delimiter, current_chunk));
    }

    chunks
}

/// Render the Swift app harness that runs the SUT server for HTTP e2e tests.
/// Collects all HTTP fixtures, builds a JSON payload, and generates Swift source
/// that registers handlers per fixture and starts the app on the configured port.
fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup], module_name: &str) -> String {
    let env = make_env();

    // Collect all HTTP fixtures from all groups.
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            let http_data = &fixture.http.as_ref().unwrap();
            let fixture_json = serde_json::json!({
                "http": {
                    "handler": {
                        "route": &http_data.handler.route,
                        "method": &http_data.handler.method,
                        "body_schema": http_data.handler.body_schema.clone(),
                    },
                    "request": {
                        "path": &http_data.request.path,
                    },
                    "expected_response": {
                        "status_code": http_data.expected_response.status_code,
                        "body": &http_data.expected_response.body,
                        "headers": &http_data.expected_response.headers,
                    }
                }
            });
            fixtures_map.insert(fixture.id.clone(), fixture_json);
        }
    }

    let fixtures_json = serde_json::to_string(&fixtures_map).unwrap_or_default();
    let fixtures_json_chunks = chunk_fixtures_for_swift(&fixtures_json);

    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;
    let app_class = e2e_config.harness.app_class_for_lang("swift");
    // Swift methods are camelCase per Swift API design guidelines.
    let register_route_method = e2e_config
        .harness
        .register_method_idiomatic("swift")
        .unwrap_or_else(|| "registerRoute".to_string());
    let body_schema_setter = &e2e_config.harness.body_schema_setter;
    let method_enum = &e2e_config.harness.method_enum;
    let run_method = e2e_config.harness.run_method_for_lang("swift");

    // Build imports: include harness.imports config plus the binding module_name.
    // Get language-specific imports for swift with fallback to global imports.
    let mut imports = e2e_config.harness.imports_for_lang("swift");
    // Prepend the binding module_name if not already present (case-insensitive check).
    if !imports.iter().any(|i| i.to_lowercase() == module_name.to_lowercase()) {
        imports.insert(0, module_name.to_string());
    }
    let imports_str = imports
        .iter()
        .map(|m| format!("import {}", m))
        .collect::<Vec<_>>()
        .join("\n");

    let ctx = context! {
        imports => imports_str,
        app_class => app_class.as_deref().unwrap_or("App"),
        route_builder_constructor => "RouteBuilder",
        route_builder_schema_setter => body_schema_setter.as_deref().unwrap_or("requestSchemaJson"),
        method_enum_class => method_enum.as_deref().unwrap_or("Method"),
        register_route_method => register_route_method.as_str(),
        run_method => run_method.as_deref().unwrap_or("run"),
        response_body_field => e2e_config.harness.response_body_field.as_str(),
        host => host,
        port => port,
        fixtures_json_chunks => fixtures_json_chunks,
    };

    render(&env, "swift/app_harness.swift.jinja", ctx)
}

// ---------------------------------------------------------------------------
// Public emit entry point
// ---------------------------------------------------------------------------

/// Emit Swift's server-pattern file: `app_harness.swift` (the executable harness).
///
/// Returns the server-pattern `GeneratedFile`s at `e2e/swift_e2e/Sources/Harness/main.swift`,
/// gated identically to alef's prior emission: HTTP fixtures present and a harness import
/// configured.
///
/// Files produced:
/// - `e2e/swift_e2e/Sources/Harness/main.swift` — the app harness executable
///
/// Note: The test files (XCTest classes in `Tests/<Module>E2ETests/`) and Package.swift
/// are emitted by alef's shared e2e generation; only the HTTP server harness is owned here.
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &alef::ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    // Derive module_name from the package config, mirroring alef swift.rs lines 76-96.
    let swift_pkg = e2e_config.resolve_package("swift");
    let pkg_name = swift_pkg
        .as_ref()
        .and_then(|p| p.name.as_ref())
        .cloned()
        .unwrap_or_else(|| config.name.to_upper_camel_case());
    let module_name_str = pkg_name.to_upper_camel_case();
    let module_name = module_name_str.as_str();

    // The harness lives under e2e/swift_e2e/ (not e2e/swift/) to avoid SwiftPM
    // package identity collision with the binding dependency at packages/swift/.
    // See alef swift.rs lines 50-57 for the rationale.
    let output_base = PathBuf::from(e2e_config.effective_output()).join("swift_e2e");

    let app_harness_body = render_app_harness(e2e_config, groups, module_name);
    // Prepend the generated header (double-slash comments for Swift).
    // Set generated_header: false because the header is already baked into content.
    let app_harness_content = format!("{}{}", hash::header(CommentStyle::DoubleSlash), app_harness_body);

    Ok(vec![GeneratedFile {
        path: output_base.join("Sources").join("Harness").join("main.swift"),
        content: app_harness_content,
        generated_header: false,
    }])
}
