//! Go HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern file for spikard's Go e2e suite:
//! - `cmd/harness/main.go` — spawns the SUT as an HTTP server serving fixture responses
//!
//! The shared client-pattern files (go.mod, helpers_test.go, main_test.go, *_test.go
//! test bodies) stay generic in alef. Only the server-spawn harness lives here.
//!
//! Sources (alef `src/e2e/codegen/go.rs`):
//! - `render_harness_main` — renders `cmd/harness/main.go` via Jinja template

use alef::GeneratedFile;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use minijinja::{Environment, context};
use std::path::PathBuf;

/// Build the private template environment holding the Go HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "go/harness_main.go.jinja".to_owned(),
        include_str!("../../templates/go/harness_main.go.jinja").to_owned(),
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

/// Render the server-pattern `cmd/harness/main.go` that spawns the SUT HTTP server.
///
/// Ported verbatim from alef's `go.rs::render_harness_main`.
#[must_use]
fn render_harness_main(_e2e_config: &E2eConfig, groups: &[FixtureGroup], go_module_path: &str) -> String {
    let mut fixtures_map = serde_json::Map::new();
    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            let http_data = fixture.http.as_ref().unwrap();
            let fixture_json = serde_json::json!({
                "http": {
                    "handler": {
                        "route": &http_data.handler.route,
                        "method": &http_data.handler.method,
                        "body_schema": http_data.handler.body_schema.clone(),
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
    let fixtures_json_obj = serde_json::Value::Object(fixtures_map);
    let fixtures_json_str = serde_json::to_string(&fixtures_json_obj).unwrap_or_default();
    let fixtures_json = fixtures_json_str.replace('\\', "\\\\").replace('"', "\\\"");

    let mut env = Environment::new();
    let harness_template = include_str!("../../templates/go/harness_main.go.jinja");
    env.add_template("harness", harness_template).ok();

    let import_alias = go_module_path.rsplit('/').next().unwrap_or("pkg").to_string();

    let template = env.get_template("harness").unwrap();
    let output = template
        .render(context! {
            imports => vec![go_module_path],
            import_alias => import_alias,
            register_route_method => "RegisterRoute",
            run_method => "Run",
            start_background_method => "StartBackground",
            port => 8012,
            fixtures_json => fixtures_json,
        })
        .unwrap_or_default();

    let mut out = hash::header(CommentStyle::DoubleSlash);
    out.push_str(&output);
    out
}

/// Emit the Go server-pattern `GeneratedFile`s.
///
/// Returns `cmd/harness/main.go` under `e2e/go/` when:
/// - at least one fixture has an `http` field, AND
/// - `e2e_config.harness.imports` is non-empty (indicating a binding is wired).
///
/// Returns an empty `Vec` when the gate condition is not met, matching alef's
/// HTTP-only server-harness gate.
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &alef::ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let call = &e2e_config.call;
    let overrides = call.overrides.get("go");
    let configured_go_module_path = config.go.as_ref().and_then(|go| go.module.as_ref()).cloned();
    let module_path = overrides
        .and_then(|o| o.module.as_ref())
        .cloned()
        .or_else(|| configured_go_module_path.clone())
        .unwrap_or_else(|| call.module.clone());

    let output_base = PathBuf::from(e2e_config.effective_output()).join("go");

    Ok(vec![GeneratedFile {
        path: output_base.join("cmd").join("harness").join("main.go"),
        content: render_harness_main(e2e_config, groups, &module_path),
        generated_header: true,
    }])
}
