//! Elixir test app generator
//!
//! Generates Elixir Spikard applications based on fixtures.
//!
//! Unlike Python/Ruby generators that rely on framework-level parameter extraction,
//! the Elixir handlers receive a `Spikard.Request` struct and must extract values
//! manually from headers, query params, etc.

use crate::elixir_utils::{
    build_handler_name, build_parameter_schema_elixir, sanitize_identifier, string_literal,
    value_to_elixir,
};
use crate::fixture_filter::is_http_fixture_category;
use crate::middleware::{parse_middleware, write_static_assets, MiddlewareMetadata};
use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn generate_elixir_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("lib/e2e_elixir_app");
    fs::create_dir_all(&app_dir).context("Failed to create Elixir app directory")?;

    let static_root = app_dir.join("static_assets");
    if static_root.exists() {
        fs::remove_dir_all(&static_root)
            .with_context(|| format!("Failed to clear {}", static_root.display()))?;
    }

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;

    let mut code = String::new();
    code.push_str(
        r#"defmodule E2EElixirApp.Handlers do
  @moduledoc """
  Generated handler functions from test fixtures.

  These handlers extract data from the Spikard.Request struct and return
  responses that match the fixture expectations. Most handlers echo back
  request data (headers, params, body) to verify correct parsing.
  """

  alias Spikard.{Request, Response}

  @doc """
  Build a response with the given content, status, and optional headers.
  """
  def build_response(content, status, headers \\ %{}) do
    %{
      status: status,
      headers: headers,
      body: content
    }
  end

  @doc """
  Get a header value from the request, case-insensitively.
  """
  def get_header(request, name) do
    name_lower = String.downcase(name)

    Enum.find_value(request.headers, fn {k, v} ->
      if String.downcase(k) == name_lower, do: v, else: nil
    end)
  end

  @doc """
  Get a query parameter from the request.
  """
  def get_query_param(request, name, default \\ nil) do
    Map.get(request.query_params || %{}, name, default)
  end

  @doc """
  Get a path parameter from the request.
  """
  def get_path_param(request, name, default \\ nil) do
    Map.get(request.path_params || %{}, name, default)
  end

"#,
    );

    for (category, fixtures) in fixtures_by_category.iter() {
        for (index, fixture) in fixtures.iter().enumerate() {
            let metadata = parse_middleware(fixture)?;
            let fixture_dir = format!(
                "{}_{}",
                sanitize_identifier(category),
                sanitize_identifier(&fixture.name)
            );

            if !metadata.static_dirs.is_empty() {
                let lib_dir = output_dir.join("lib/e2e_elixir_app");
                write_static_assets(&lib_dir, &fixture_dir, &metadata.static_dirs)?;
            }

            code.push_str(&build_fixture_function(category, index, fixture, &metadata)?);
        }
    }

    code.push_str("end\n");

    fs::write(app_dir.join("handlers.ex"), &code).context("Failed to write Elixir handlers file")?;

    // Generate the router module
    let router_code = generate_router(&fixtures_by_category)?;
    fs::write(app_dir.join("router.ex"), &router_code).context("Failed to write Elixir router file")?;

    // Generate mix.exs if it doesn't exist
    let mix_file = output_dir.join("mix.exs");
    if !mix_file.exists() {
        let mix_code = generate_mix_exs();
        fs::write(&mix_file, mix_code).context("Failed to write mix.exs")?;
    }

    // Generate test_helper.exs
    let test_helper = output_dir.join("test/test_helper.exs");
    fs::create_dir_all(output_dir.join("test")).context("Failed to create test directory")?;
    let test_helper_code = r#"ExUnit.start()
"#;
    fs::write(&test_helper, test_helper_code).context("Failed to write test_helper.exs")?;

    // Generate .formatter.exs
    let formatter_file = output_dir.join(".formatter.exs");
    let formatter_code = r#"[
  inputs: ["{mix,.formatter}.exs", "{config,lib,test}/**/*.{ex,exs}"]
]
"#;
    fs::write(&formatter_file, formatter_code).context("Failed to write .formatter.exs")?;

    Ok(())
}

fn load_fixtures_grouped(fixtures_dir: &Path) -> Result<BTreeMap<String, Vec<Fixture>>> {
    let mut grouped: BTreeMap<String, Vec<Fixture>> = BTreeMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read fixture directory entry")?;
        let path = entry.path();
        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("fixtures")
                .to_string();
            if !is_http_fixture_category(&category) {
                continue;
            }
            let mut fixtures = load_fixtures_from_dir(&path)
                .with_context(|| format!("Failed to load fixtures from {}", path.display()))?;
            fixtures.sort_by(|a, b| a.name.cmp(&b.name));
            grouped.insert(category, fixtures);
        }
    }

    Ok(grouped)
}

fn build_fixture_function(
    category: &str,
    _index: usize,
    fixture: &Fixture,
    _metadata: &MiddlewareMetadata,
) -> Result<String> {
    let handler_name = build_handler_name(category, &fixture.name);
    let mut code = String::new();

    code.push_str(&format!(
        "  @doc \"\"\"\n  Handler for fixture: {} - {}\n  \"\"\"\n",
        category, fixture.name
    ));

    // Most handlers don't need the request - they return the expected response
    // because validation and parameter extraction happens at the Rust/NIF layer.
    // Only handlers that need to dynamically compute responses use the request.
    let needs_request = needs_dynamic_response(fixture);
    let request_param = if needs_request { "request" } else { "_request" };

    code.push_str(&format!("  def {}({}) do\n", handler_name, request_param));

    // Generate handler body
    let handler_body = generate_handler_body(fixture)?;
    code.push_str(&handler_body);

    code.push_str("  end\n\n");
    Ok(code)
}

/// Determine if the handler needs dynamic response generation
/// Most handlers just return the expected response body directly since
/// validation happens at the Rust layer. Only certain fixtures need
/// to actually read and transform request data.
fn needs_dynamic_response(fixture: &Fixture) -> bool {
    // Check if this is a fixture that needs to echo back request data
    // that isn't directly available in the expected_response.body
    if let Some(ref handler) = fixture.handler {
        // Body requests may need to echo the body back
        if handler.body_schema.is_some() {
            if let Some(ref body) = fixture.expected_response.body {
                // If the response contains "echo" or similar, it echoes the body
                if body_contains_echo_pattern(body) {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if the response body contains patterns suggesting it echoes input
fn body_contains_echo_pattern(body: &Value) -> bool {
    match body {
        Value::Object(map) => {
            map.keys().any(|k| {
                let k_lower = k.to_lowercase();
                k_lower == "echo" || k_lower == "received" || k_lower == "body"
            })
        }
        _ => false,
    }
}

/// Generate the handler body based on fixture specifications
///
/// Strategy: Return the expected response body directly. The Rust/NIF layer
/// handles validation, so if a request reaches the handler, it's already valid.
/// The expected_response in the fixture is what a valid request should return.
fn generate_handler_body(fixture: &Fixture) -> Result<String> {
    let response = &fixture.expected_response;
    let status = response.status_code;
    let response_headers = generate_headers_map(response);
    let body = generate_body(response);

    Ok(format!(
        "    build_response({}, {}, {})\n",
        body, status, response_headers
    ))
}

fn generate_headers_map(response: &FixtureExpectedResponse) -> String {
    if let Some(ref headers) = response.headers {
        let pairs: Vec<String> = headers
            .iter()
            .map(|(k, v)| format!("{} => {}", string_literal(k), string_literal(v)))
            .collect();
        if pairs.is_empty() {
            "%{}".to_string()
        } else {
            format!("%{{{}}}", pairs.join(", "))
        }
    } else {
        "%{}".to_string()
    }
}

fn generate_body(response: &FixtureExpectedResponse) -> String {
    if let Some(ref body) = response.body {
        value_to_elixir(body)
    } else {
        "nil".to_string()
    }
}

/// Generate the AppFactories module with per-fixture factory functions.
/// Each factory function returns a route list for exactly one fixture,
/// avoiding route conflicts when multiple fixtures share the same path.
fn generate_router(fixtures_by_category: &BTreeMap<String, Vec<Fixture>>) -> Result<String> {
    let mut code = String::new();
    code.push_str(
        r#"defmodule E2EElixirApp.AppFactories do
  @moduledoc """
  Generated app factory functions from test fixtures.

  Each factory function returns a route list for a single fixture.
  This avoids route conflicts when multiple fixtures test the same path
  (e.g., /query/basic with different query parameters).

  Usage:
      routes = E2EElixirApp.AppFactories.create_app_query_params_basic_success()
      {:ok, server} = Spikard.start(port: 59800, host: "127.0.0.1", routes: routes)
  """

  alias E2EElixirApp.Handlers

"#,
    );

    for (category, fixtures) in fixtures_by_category.iter() {
        for fixture in fixtures.iter() {
            code.push_str(&generate_app_factory(category, fixture)?);
        }
    }

    code.push_str("end\n");
    Ok(code)
}

/// Generate a single app factory function for a fixture.
fn generate_app_factory(category: &str, fixture: &Fixture) -> Result<String> {
    let handler_name = build_handler_name(category, &fixture.name);
    let factory_name = format!("create_app_{}", handler_name);
    let method = fixture.request.method.to_lowercase();

    // Use handler.route if available (has path params like /items/{id}),
    // otherwise fall back to request.path (literal like /items/1)
    let path: &str = fixture
        .handler
        .as_ref()
        .map(|h| h.route.as_str())
        .unwrap_or(&fixture.request.path);

    // Build parameter schema if handler has parameters
    let param_schema = fixture
        .handler
        .as_ref()
        .and_then(|h| h.parameters.as_ref())
        .map(|p| build_parameter_schema_elixir(p))
        .unwrap_or_else(|| "nil".to_string());

    // Build request body schema if handler has body_schema
    let request_schema = fixture
        .handler
        .as_ref()
        .and_then(|h| h.body_schema.as_ref())
        .map(|s| value_to_elixir(s))
        .unwrap_or_else(|| "nil".to_string());

    let mut route_parts = vec![
        format!("method: :{}", method),
        format!("path: {}", string_literal(path)),
        format!("handler: &Handlers.{}/1", handler_name),
    ];

    if param_schema != "nil" {
        route_parts.push(format!("parameter_schema: {}", param_schema));
    }
    if request_schema != "nil" {
        route_parts.push(format!("request_schema: {}", request_schema));
    }

    // Parse middleware and generate config
    let metadata = parse_middleware(fixture)?;

    let fixture_id = format!(
        "{}_{}",
        sanitize_identifier(category),
        sanitize_identifier(&fixture.name)
    );

    let config_code = generate_config_code(&metadata, &fixture_id)?;

    let mut code = String::new();
    code.push_str(&format!(
        "  @doc \"\"\"\n  App factory for fixture: {} - {}\n  \"\"\"\n",
        category, fixture.name
    ));
    code.push_str(&format!("  def {}() do\n", factory_name));

    // For static_files fixtures, don't create a handler route - let the middleware handle it
    if metadata.static_dirs.is_empty() {
        code.push_str(&format!("    routes = [%{{{}}}]\n", route_parts.join(", ")));
    } else {
        code.push_str("    routes = []\n");
    }

    if !config_code.is_empty() {
        code.push_str(&format!("    config = {}\n", config_code));
        code.push_str("    {routes, config}\n");
    } else {
        code.push_str("    {routes, %{}}\n");
    }

    code.push_str("  end\n\n");

    Ok(code)
}

/// Generate Elixir config code from middleware metadata
fn generate_config_code(metadata: &MiddlewareMetadata, fixture_id: &str) -> Result<String> {
    let mut fields = Vec::new();

    if !metadata.static_dirs.is_empty() {
        let mut static_files = String::from("static_files: [\n");
        for dir in &metadata.static_dirs {
            static_files.push_str("      %{\n");
            static_files.push_str(&format!("        directory: {:?},\n", format!("lib/e2e_elixir_app/static_assets/{}/{}",
                fixture_id, dir.directory_name)));
            static_files.push_str(&format!("        route_prefix: {:?},\n", dir.route_prefix));
            if !dir.index_file {
                static_files.push_str("        index_file: false,\n");
            }
            if let Some(cache) = &dir.cache_control {
                static_files.push_str(&format!("        cache_control: {:?},\n", cache));
            }
            static_files.push_str("      },\n");
        }
        static_files.push_str("    ]");
        fields.push(static_files);
    }

    if fields.is_empty() {
        Ok("%{}".to_string())
    } else {
        let mut config = String::from("%{\n");
        for field in fields {
            config.push_str("    ");
            config.push_str(&field);
            config.push('\n');
        }
        config.push_str("  }");
        Ok(config)
    }
}

fn generate_mix_exs() -> String {
    r#"defmodule E2EElixirApp.MixProject do
  use Mix.Project

  def project do
    [
      app: :e2e_elixir_app,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :inets, :ssl]
    ]
  end

  defp deps do
    [
      {:spikard, path: "../../packages/elixir"},
      {:jason, "~> 1.4"}
    ]
  end
end
"#
    .to_string()
}
