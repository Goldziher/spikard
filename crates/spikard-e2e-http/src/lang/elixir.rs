//! Elixir HTTP e2e test generation (server-pattern slice from alef core).
//!
//! This module owns the server-pattern files for spikard's Elixir e2e suite:
//! - `app_harness.exs` — standalone Elixir script that loads the SUT binding,
//!   registers one handler per fixture, and serves on the configured port.
//! - `test/test_helper.exs` (server variant) — spawns `app_harness.exs` as a
//!   `Port`, polls TCP, and sets `SUT_URL` before `ExUnit.start()`.
//!
//! The shared client-pattern `test_helper.exs` (mock-server and NIF-only variants)
//! and all project scaffolding (`mix.exs`, `lib/`, test files) stay generic in alef.
//! Only the server-spawn slice lives here.
//!
//! Sources (alef `src/e2e/codegen/elixir/`):
//! - `project.rs::render_app_harness` (3-arg: `e2e_config`, groups, `crate_config`)
//! - `project.rs::render_test_helper` (`uses_harness` branch only)
//! - `project.rs::render_env_setup_block`
#![allow(dead_code)]

use alef::GeneratedFile;
use alef::ResolvedCrateConfig;
use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::fixture::FixtureGroup;
use minijinja::{Environment, context};

// ---------------------------------------------------------------------------
// Template environment
// ---------------------------------------------------------------------------

/// Build the private template environment holding the Elixir HTTP templates.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "elixir/app_harness.exs.jinja".to_owned(),
        include_str!("../../templates/elixir/app_harness.exs.jinja").to_owned(),
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
// Helpers
// ---------------------------------------------------------------------------

/// Convert a `snake_case` or hyphenated module name to Elixir's `PascalCase`.
///
/// Ported from `alef::e2e::codegen::elixir::values::elixir_module_name`
/// (not in alef's public API surface).
fn elixir_module_name(name: &str) -> String {
    name.split(['.', '_', '-'])
        .filter(|s| !s.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}

/// Emit an Elixir snippet that sets every `[e2e.env]` entry into the environment
/// using `System.get_env` guards. Returns empty string when no env vars are configured.
///
/// Ported from `alef::e2e::codegen::elixir::project::render_env_setup_block`.
fn render_env_setup_block(e2e_config: &E2eConfig) -> String {
    if e2e_config.env.is_empty() {
        return String::new();
    }
    let mut keys: Vec<&String> = e2e_config.env.keys().collect();
    keys.sort();
    let mut out = String::new();
    for k in keys {
        let v = &e2e_config.env[k];
        out.push_str(&format!(
            "unless System.get_env(\"{k}\") do\n  System.put_env(\"{k}\", \"{v}\")\nend\n"
        ));
    }
    out.push('\n');
    out
}

// ---------------------------------------------------------------------------
// App harness renderer
// (from alef `elixir/project.rs::render_app_harness`)
// ---------------------------------------------------------------------------

/// Render the server-pattern `app_harness.exs` that serves SUT fixtures.
///
/// Ported verbatim from alef's `elixir/project.rs::render_app_harness`.
///
/// # Panics
///
/// Panics if the built-in Jinja template fails to parse (indicates a compile-time
/// template authoring error, not a runtime condition).
#[must_use]
pub fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup], config: &ResolvedCrateConfig) -> String {
    // Collect all HTTP fixtures from all groups.
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

    let fixtures_json_str = serde_json::to_string(&fixtures_map).unwrap_or_default();
    // Escape backslashes and quotes for Elixir string literal.
    let fixtures_json = fixtures_json_str.replace('\\', "\\\\").replace('"', "\\\"");
    let fixtures_json = format!("\"{fixtures_json}\"");

    // Apply per-language harness overrides on top of the top-level harness config.
    let harness_override = e2e_config.harness.overrides.get("elixir");
    let imports_override = harness_override.and_then(|o| o.imports.as_ref());
    let imports: &[String] = imports_override.unwrap_or(&e2e_config.harness.imports);
    let app_class: Option<&str> = harness_override
        .and_then(|o| o.app_class.as_deref())
        .or(e2e_config.harness.app_class.as_deref());
    let register_route_method: Option<&str> = harness_override
        .and_then(|o| o.register_method.as_deref())
        .or(e2e_config.harness.register_method.as_deref());
    let body_schema_setter: Option<&str> = harness_override
        .and_then(|o| o.body_schema_setter.as_deref())
        .or(e2e_config.harness.body_schema_setter.as_deref());
    let run_method: Option<&str> = harness_override
        .and_then(|o| o.run_method.as_deref())
        .or(e2e_config.harness.run_method.as_deref());
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;

    let header = hash::header(CommentStyle::Hash);

    let binding_path = if e2e_config.dep_mode == alef::e2e::config::DependencyMode::Local {
        "../../packages/elixir"
    } else {
        "."
    };

    // Build module paths for RouteBuilder, Method, and App using the binding name from imports[0].
    let module_prefix = if imports.is_empty() {
        String::new()
    } else {
        format!("{}.", elixir_module_name(&imports[0]))
    };
    let route_builder_class = format!("{module_prefix}RouteBuilder");
    let method_enum_class = format!("{module_prefix}Method");
    let server_config_class = format!("{module_prefix}ServerConfig");
    let unqualified_app_class = app_class.unwrap_or("App");
    let app_class_name = format!("{module_prefix}{unqualified_app_class}");

    // Check if App.config is excluded from bindings.
    let config_method_key = format!("{app_class_name}.config");
    let skip_app_config = config.exclude.methods.iter().any(|m| m == &config_method_key);

    let env = make_env();
    render(
        &env,
        "elixir/app_harness.exs.jinja",
        context! {
            header => header,
            app_class => app_class_name,
            route_builder_class => &route_builder_class,
            route_builder_schema_setter => body_schema_setter.unwrap_or("request_schema_json"),
            method_enum_class => &method_enum_class,
            register_route_method => register_route_method.unwrap_or("route"),
            run_method => run_method.unwrap_or("run"),
            server_config_class => &server_config_class,
            host => host,
            port => port,
            binding_path => binding_path,
            fixtures_json => fixtures_json,
            skip_app_config => skip_app_config,
        },
    )
}

// ---------------------------------------------------------------------------
// Server-pattern test_helper renderer
// (from alef `elixir/project.rs::render_test_helper` uses_harness branch)
// ---------------------------------------------------------------------------

/// Render the server-pattern `test/test_helper.exs` that spawns `app_harness.exs`.
///
/// This is the `uses_harness` branch of alef's
/// `elixir/project.rs::render_test_helper`, extracted verbatim so the extension
/// can emit it instead of alef. The env-setup block (env vars with
/// `System.get_env` guards), the Finch start, and the `Port.open` spawn logic
/// are all ported exactly.
#[must_use]
pub fn render_test_helper_server(e2e_config: &E2eConfig) -> String {
    let env_setup = render_env_setup_block(e2e_config);
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;
    format!(
        r#"{env_setup}# Start a named Finch pool before ExUnit configured to use HTTP/1 only.
# Tests pass `finch: AlefE2EFinch` on every Req call; the pool's protocol
# selection (via `pools.default.protocols: [:http1]`) is the canonical place
# to pin the wire protocol since Req rejects per-call `:connect_options` when
# `:finch` is set.
{{:ok, _}} = Finch.start_link(name: AlefE2EFinch, pools: %{{:default => [protocols: [:http1]]}})

ExUnit.start()

# Spawn app_harness subprocess and set SUT_URL
# If SUT_URL is already set, a parent process started a shared harness.
# Use it as-is and do NOT spawn our own.

unless System.get_env("SUT_URL") do
  app_harness_bin = Path.expand("../app_harness.exs", __DIR__)
  project_root = Path.expand("..", __DIR__)

  # Build the list of ebin directories from _build/dev/lib so the harness can access compiled dependencies
  build_lib_dir = Path.join(project_root, "_build/dev/lib")
  lib_paths = if File.dir?(build_lib_dir) do
    File.ls!(build_lib_dir)
    |> Enum.map(&Path.join(build_lib_dir, &1))
    |> Enum.filter(&File.dir?/1)
    |> Enum.flat_map(fn lib_path ->
      ebin_path = Path.join(lib_path, "ebin")
      if File.dir?(ebin_path), do: ["-pa", ebin_path], else: []
    end)
  else
    []
  end

  # Use `elixir` to execute the harness script with proper code paths
  port = Port.open({{:spawn_executable, System.find_executable("elixir")}}, [
    :binary,
    {{:line, 65_536}},
    args: lib_paths ++ [app_harness_bin]
  ])

  url = "http://{host}:{port}"

  # Poll until the harness accepts TCP connections
  deadline = :erlang.monotonic_time(:millisecond) + 15_000
  ready = false

  {{ready, url}} =
    Enum.reduce_while(1..150, {{false, url}}, fn _, {{_, url_acc}} ->
      now = :erlang.monotonic_time(:millisecond)
      if now > deadline do
        {{:halt, {{false, url_acc}}}}
      else
        case :gen_tcp.connect(String.to_charlist("{host}"), {port}, [], 500) do
          {{:ok, socket}} ->
            :gen_tcp.close(socket)
            {{:halt, {{true, url_acc}}}}
          {{:error, _}} ->
            Process.sleep(100)
            {{:cont, {{false, url_acc}}}}
        end
      end
    end)

  unless ready do
    Port.close(port)
    raise "App harness did not become reachable on {host}:{port} within 15s"
  end

  System.put_env("SUT_URL", url)
end
"#
    )
}

// ---------------------------------------------------------------------------
// Top-level emit function (extension dispatch target)
// ---------------------------------------------------------------------------

/// Emit Elixir's server-pattern e2e files.
///
/// Returns `app_harness.exs` and the server-variant `test/test_helper.exs` gated
/// on the same condition alef used: HTTP fixtures present **and** harness imports
/// configured. Returns an empty `Vec` otherwise (alef emits the client-pattern
/// files for all other cases).
///
/// # Errors
///
/// Currently infallible; returns `Err` only if an internal template render
/// produces an unrecoverable error (which would indicate a compile-time template
/// authoring bug, not a runtime condition).
pub fn emit(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &ResolvedCrateConfig,
) -> anyhow::Result<Vec<GeneratedFile>> {
    use std::path::PathBuf;

    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let base = PathBuf::from(e2e_config.effective_output()).join("elixir");
    Ok(vec![
        GeneratedFile {
            path: base.join("app_harness.exs"),
            content: render_app_harness(e2e_config, groups, config),
            generated_header: true,
        },
        GeneratedFile {
            path: base.join("test").join("test_helper.exs"),
            content: render_test_helper_server(e2e_config),
            generated_header: false,
        },
    ])
}
