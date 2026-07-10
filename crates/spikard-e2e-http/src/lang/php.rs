//! PHP HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern file for spikard's PHP e2e suite:
//! - `app_harness.php` — spawns the SUT as an HTTP server via the ext-php-rs binding
//!
//! The shared client-pattern files (composer.json, phpunit.xml, bootstrap.php,
//! `run_tests.php`, test files) stay generic in alef. Only the server-spawn slice
//! lives here.
//!
//! Sources (alef `src/e2e/codegen/php/project.rs`):
//! - `render_app_harness` (3-arg: `e2e_config`, groups, `pkg_path`)
#![allow(dead_code)]

use alef::GeneratedFile;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use minijinja::{Environment, context};
use std::path::PathBuf;

/// Build the private template environment holding the PHP HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "php/app_harness.php.jinja".to_owned(),
        include_str!("../../templates/php/app_harness.php.jinja").to_owned(),
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

/// Render the server-pattern `app_harness.php` that spawns the SUT HTTP server.
///
/// Ported verbatim from alef's `php/project.rs::render_app_harness`.
#[must_use]
fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup], pkg_path: &str) -> String {
    use serde_json::json;

    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            let http_data = fixture.http.as_ref().unwrap();
            let fixture_json = json!({
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

    let imports = &e2e_config.harness.imports;
    let app_class = e2e_config.harness.app_class_for_lang("php");
    let register_route_method = e2e_config
        .harness
        .register_method_idiomatic("php")
        .unwrap_or_else(|| "route".to_string());
    let body_schema_setter = &e2e_config.harness.body_schema_setter;
    let method_enum = &e2e_config.harness.method_enum;
    let run_method = e2e_config.harness.run_method_for_lang("php");
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;

    let header = hash::header(CommentStyle::DoubleSlash);

    let route_builder_import = if imports.is_empty() {
        "App\\Php".to_string()
    } else {
        let module_name = &imports[0];
        module_name
            .split('_')
            .map(|p| {
                let mut chars = p.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join("\\")
            + "\\Php"
    };
    let method_enum_import = route_builder_import.clone();

    let ctx = context! {
        header => header,
        imports => imports,
        app_class => app_class.as_deref().unwrap_or("App"),
        route_builder_import => route_builder_import,
        route_builder_class => "RouteBuilder",
        register_route_method => register_route_method.as_str(),
        route_builder_schema_setter => body_schema_setter.as_deref().unwrap_or("request_schema_json"),
        method_enum_import => method_enum_import,
        method_enum_class => method_enum.as_deref().unwrap_or("Method"),
        run_method => run_method.as_deref().unwrap_or("run"),
        response_body_field => e2e_config.harness.response_body_field.as_str(),
        host => host,
        port => port,
        pkg_path => pkg_path,
        fixtures_json => fixtures_json,
    };

    let env = make_env();
    render(&env, "php/app_harness.php.jinja", ctx)
}

/// Emit the PHP server-pattern `GeneratedFile`s.
///
/// Returns `app_harness.php` under `e2e/php/` when:
/// - at least one fixture has an `http` field, AND
/// - `e2e_config.harness.imports` is non-empty (indicating a binding is wired).
///
/// Returns an empty `Vec` when the gate condition is not met, matching alef's
/// `uses_server_harness` guard at `php.rs:156-157`.
pub fn emit(
    groups: &[alef::FixtureGroup],
    e2e_config: &alef::E2eConfig,
    _config: &alef::ResolvedCrateConfig,
) -> Result<Vec<alef::GeneratedFile>> {
    let has_http_server_fixtures = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    let uses_server_harness = has_http_server_fixtures && !e2e_config.harness.imports.is_empty();

    if !uses_server_harness {
        return Ok(Vec::new());
    }

    let php_pkg = e2e_config.resolve_package("php");
    let pkg_path = php_pkg
        .as_ref()
        .and_then(|p| p.path.as_ref())
        .cloned()
        .unwrap_or_else(|| "../../packages/php".to_string());

    let output_base = PathBuf::from(e2e_config.effective_output()).join("php");

    Ok(vec![GeneratedFile {
        path: output_base.join("app_harness.php"),
        content: render_app_harness(e2e_config, groups, &pkg_path),
        generated_header: true,
    }])
}
