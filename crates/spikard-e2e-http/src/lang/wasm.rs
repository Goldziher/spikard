//! WebAssembly HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern files for spikard's WASM e2e suite:
//! - `app_harness.mjs` — spawns the SUT as an HTTP server via the wasm-bindgen binding
//! - `globalSetup.ts` — the server-pattern variant that spawns the app harness subprocess
//!
//! The shared client-pattern files (package.json, vitest.config.ts, setup.ts, test files)
//! stay generic in alef. Only the server-spawn slice lives here.
//!
//! Sources (alef `src/e2e/codegen/wasm.rs`):
//! - `render_wasm_app_harness` (line ~774)
//! - pkg_name derivation and `app_class_excluded` gate (lines ~60-180, ~234-239)

use alef::GeneratedFile;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use anyhow::Result;
use minijinja::{Environment, context};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Template environment
// ---------------------------------------------------------------------------

/// Build the private template environment holding the TypeScript/WASM HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "typescript/app_harness.mjs.jinja".to_owned(),
        include_str!("../../templates/typescript/app_harness.mjs.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "typescript/globalSetup_server.ts.jinja".to_owned(),
        include_str!("../../templates/typescript/globalSetup_server.ts.jinja").to_owned(),
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
// App harness renderer (ported from alef `wasm.rs::render_wasm_app_harness`).
// ---------------------------------------------------------------------------

/// Render the server-pattern `app_harness.mjs` that spawns the SUT HTTP server.
/// Uses the wasm-bindgen nodejs package instead of the node NAPI-RS package.
///
/// Ported verbatim from alef's `wasm.rs::render_wasm_app_harness`.
#[must_use]
fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup], wasm_pkg_name: &str) -> String {
    // Collect all HTTP fixtures from all groups.
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            let http_data = &fixture.http.as_ref().unwrap();
            let mut handler_obj = serde_json::json!({
                "route": &http_data.handler.route,
                "method": &http_data.handler.method,
                "body_schema": http_data.handler.body_schema.clone(),
            });
            // Include middleware if present for CORS preflight registration
            if let Some(middleware) = &http_data.handler.middleware
                && let Ok(middleware_json) = serde_json::to_value(middleware)
                && let serde_json::Value::Object(ref obj) = handler_obj
            {
                let mut handler_map = obj.clone();
                handler_map.insert("middleware".to_string(), middleware_json);
                handler_obj = serde_json::Value::Object(handler_map);
            }
            let mut request_obj = serde_json::json!({
                "path": &http_data.request.path,
            });
            // Include content_type if present so the harness can detect
            // multipart/form-encoded bodies and synthesize them correctly.
            if let Some(ct) = &http_data.request.content_type
                && let serde_json::Value::Object(ref obj) = request_obj
            {
                let mut request_map = obj.clone();
                request_map.insert("content_type".to_string(), serde_json::Value::String(ct.clone()));
                request_obj = serde_json::Value::Object(request_map);
            }
            let fixture_json = serde_json::json!({
                "http": {
                    "handler": handler_obj,
                    "request": request_obj,
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
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;
    let header = hash::header(CommentStyle::DoubleSlash);

    let app_class = e2e_config.harness.app_class_for_lang("wasm");
    let method_enum = &e2e_config.harness.method_enum;
    let run_method = e2e_config.harness.run_method_for_lang("wasm");
    // wasm-bindgen emits JS-idiomatic camelCase method names, so the harness
    // must call `registerRoute` (or whatever override is configured), not the
    // canonical snake_case identifier from the config.
    let register_method = e2e_config
        .harness
        .register_method_idiomatic("wasm")
        .unwrap_or_else(|| "registerRoute".to_string());
    let route_builder_class = e2e_config.harness.route_builder.as_deref().unwrap_or("RouteBuilder");

    // wasm-bindgen exposes ServerConfig as a class with a default constructor,
    // so the harness must use `new WasmServerConfig()` and include
    // `WasmServerConfig` in the destructure import.
    let server_config_factory = e2e_config.harness.server_config_factory_for_lang("wasm");
    let server_config_factory_import = e2e_config
        .harness
        .server_config_factory_import_for_lang("wasm")
        .unwrap_or_else(|| "ServerConfig".to_string());
    let import_style = e2e_config.harness.import_style_for_lang("wasm");

    let env = make_env();
    render(
        &env,
        "typescript/app_harness.mjs.jinja",
        context! {
            header => header,
            host => host,
            port => port,
            response_body_field => e2e_config.harness.response_body_field.as_str(),
            fixtures_json => fixtures_json,
            imports => vec![wasm_pkg_name.to_string()],
            app_class => app_class.as_deref().unwrap_or("App"),
            method_enum => method_enum.as_deref().unwrap_or("Method"),
            route_builder_class => route_builder_class,
            run_method => run_method.as_deref().unwrap_or("run"),
            register_route_method => register_method.as_str(),
            constructor_method => ".new()",
            server_config_factory => server_config_factory,
            server_config_factory_import => server_config_factory_import,
            import_style => import_style,
        },
    )
}

/// Render the server-pattern `globalSetup.ts` that spawns the app harness subprocess.
///
/// Ported verbatim from alef's `typescript/config.rs::render_global_setup(true)`.
#[must_use]
fn render_global_setup_server() -> String {
    let header = hash::header(CommentStyle::DoubleSlash);
    let env = make_env();
    render(
        &env,
        "typescript/globalSetup_server.ts.jinja",
        context! { header => header },
    )
}

// ---------------------------------------------------------------------------
// Public emit entrypoint (called by the orchestrator).
// ---------------------------------------------------------------------------

/// Emit the WASM server-pattern `GeneratedFile`s.
///
/// Returns `app_harness.mjs` and `globalSetup.ts` under `e2e/wasm/` when:
/// - at least one fixture has an `http` field, AND
/// - `e2e_config.harness.imports` is non-empty (indicating a binding is wired), AND
/// - the `App` class is NOT in the `[crates.wasm].exclude_types` list
///   (i.e., the App service API is included in the WASM binding).
///
/// Returns an empty `Vec` when the gate condition is not met, matching alef's
/// server-pattern gate at `wasm.rs:234-239`.
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &alef::ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    // Gate 1: Check for HTTP fixtures (uses `needs_mock_server` like alef does)
    let has_http_fixtures = groups
        .iter()
        .flat_map(|g| g.fixtures.iter())
        .any(|f| f.needs_mock_server());

    // Gate 2: Check if harness imports are configured
    let has_harness_imports = !e2e_config.harness.imports.is_empty();

    // Gate 3: Check if App class is excluded from the WASM binding
    let app_class_excluded = config
        .wasm
        .as_ref()
        .map(|w| w.exclude_types.iter().any(|t| t == "App"))
        .unwrap_or(false);

    let use_server_pattern = has_http_fixtures && has_harness_imports && !app_class_excluded;

    if !use_server_pattern {
        return Ok(Vec::new());
    }

    // Derive pkg_name exactly as alef does at `wasm.rs:74-78`:
    // - Prefer name from `[crates.e2e.packages.wasm].name`
    // - Fall back to `config.wasm_package_name()`
    let wasm_pkg = e2e_config.resolve_package("wasm");
    let pkg_name = wasm_pkg
        .as_ref()
        .and_then(|p| p.name.as_ref())
        .cloned()
        .unwrap_or_else(|| config.wasm_package_name());

    let output_base = PathBuf::from(e2e_config.effective_output()).join("wasm");

    Ok(vec![
        GeneratedFile {
            path: output_base.join("app_harness.mjs"),
            content: render_app_harness(e2e_config, groups, &pkg_name),
            generated_header: true,
        },
        GeneratedFile {
            path: output_base.join("globalSetup.ts"),
            content: render_global_setup_server(),
            generated_header: true,
        },
    ])
}
