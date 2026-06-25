//! Dart HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern file for spikard's Dart e2e suite:
//! - `app_harness.dart` — spawns the SUT as an HTTP server via the FFI binding
//!
//! The shared client-pattern files (`pubspec.yaml`, `dart_test.yaml`, test files)
//! stay generic in alef. Only the server-spawn app harness lives here.
//!
//! Sources (alef `src/e2e/codegen/dart/project.rs`):
//! - `render_app_harness` (3-arg: groups, e2e_config, pkg_name)

use alef::GeneratedFile;
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use minijinja::{Environment, context};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Template environment
// ---------------------------------------------------------------------------

/// Build the private template environment holding the Dart HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "dart/app_harness.dart.jinja".to_owned(),
        include_str!("../../templates/dart/app_harness.dart.jinja").to_owned(),
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
// App harness renderer (ported from alef `dart/project.rs::render_app_harness`).
// ---------------------------------------------------------------------------

/// Render the server-pattern `app_harness.dart` that spawns the SUT HTTP server.
///
/// Ported verbatim from alef's `dart/project.rs::render_app_harness`.
#[must_use]
fn render_app_harness(groups: &[FixtureGroup], e2e_config: &E2eConfig, pkg_name: &str) -> String {
    // Collect all HTTP fixtures from all groups.
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if let Some(http) = &fixture.http {
                let mut fixture_obj = serde_json::Map::new();

                let mut http_obj = serde_json::Map::new();

                // handler: route, method, body_schema
                let mut handler_obj = serde_json::Map::new();
                handler_obj.insert("route".to_string(), serde_json::json!(http.handler.route));
                handler_obj.insert("method".to_string(), serde_json::json!(http.handler.method.as_str()));
                if let Some(body_schema) = &http.handler.body_schema {
                    handler_obj.insert("body_schema".to_string(), body_schema.clone());
                } else {
                    handler_obj.insert("body_schema".to_string(), serde_json::Value::Null);
                }
                http_obj.insert("handler".to_string(), serde_json::Value::Object(handler_obj));

                // expected_response: status_code, body, headers
                let mut response_obj = serde_json::Map::new();
                response_obj.insert(
                    "status_code".to_string(),
                    serde_json::json!(http.expected_response.status_code),
                );
                if let Some(body) = &http.expected_response.body {
                    response_obj.insert("body".to_string(), body.clone());
                } else {
                    response_obj.insert("body".to_string(), serde_json::Value::Null);
                }

                let headers: serde_json::Map<String, serde_json::Value> = http
                    .expected_response
                    .headers
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::json!(v)))
                    .collect();
                response_obj.insert("headers".to_string(), serde_json::Value::Object(headers));

                http_obj.insert("expected_response".to_string(), serde_json::Value::Object(response_obj));

                fixture_obj.insert("http".to_string(), serde_json::Value::Object(http_obj));
                fixtures_map.insert(fixture.id.clone(), serde_json::Value::Object(fixture_obj));
            }
        }
    }

    let fixtures_json = serde_json::to_string(&fixtures_map).unwrap_or_else(|_| "{}".to_string());

    // Derive the bridge module name from the package name:
    // e.g. "my_pkg" → "my_pkg_bridge_generated"
    let bridge_module = format!("{pkg_name}_bridge_generated");

    // Render using the Jinja template.
    let ctx = context! {
        fixtures_json => fixtures_json,
        pkg_name => pkg_name,
        bridge_module => bridge_module,
        host => &e2e_config.harness.host,
        port => e2e_config.harness.port,
    };
    render(&make_env(), "dart/app_harness.dart.jinja", ctx)
}

// ---------------------------------------------------------------------------
// Public emit entrypoint (called by the orchestrator).
// ---------------------------------------------------------------------------

/// Emit the Dart server-pattern `GeneratedFile`s.
///
/// Returns `app_harness.dart` under `e2e/dart/` when:
/// - at least one fixture has an `http` field, AND
/// - `e2e_config.harness.imports` is non-empty (indicating a binding is wired).
///
/// Returns an empty `Vec` when the gate condition is not met, matching alef's
/// `has_http_fixtures` guard at `dart.rs:75-84`.
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &alef::ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    let has_http_fixtures = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http_fixtures || e2e_config.harness.imports.is_empty() {
        return Ok(Vec::new());
    }

    // Resolve package config, mirroring alef's `dart.rs:34-51`.
    let dart_pkg = e2e_config.resolve_package("dart");
    let pkg_name = dart_pkg
        .as_ref()
        .and_then(|p| p.name.as_ref())
        .cloned()
        .unwrap_or_else(|| config.dart_pubspec_name());

    let output_base = PathBuf::from(e2e_config.effective_output()).join("dart");

    Ok(vec![GeneratedFile {
        path: output_base.join("app_harness.dart"),
        content: render_app_harness(groups, e2e_config, &pkg_name),
        generated_header: true,
    }])
}
