//! Ruby test app generator
//!
//! Generates Ruby Spikard applications based on fixtures.

use anyhow::{Context, Result};
use serde_json::{Map, Value};
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::ruby_utils::{build_handler_name, build_method_name, string_literal, value_to_ruby};

pub fn generate_ruby_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create Ruby app directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;

    let mut code = String::new();
    code.push_str(
        "# frozen_string_literal: true

",
    );
    code.push_str(
        "require 'json'
",
    );
    code.push_str(
        "require 'spikard'

",
    );
    code.push_str(
        "module E2ERubyApp
",
    );
    code.push_str(
        "  module_function

",
    );
    code.push_str(
        "  def build_response(content:, status:, headers: nil)
",
    );
    code.push_str(
        "    headers ||= {}
",
    );
    code.push_str(
        "    Spikard::Response.new(content: content, status_code: status, headers: headers)
",
    );
    code.push_str(
        "  end

",
    );

    for (category, fixtures) in fixtures_by_category.iter() {
        for (index, fixture) in fixtures.iter().enumerate() {
            code.push_str(&build_fixture_function(category, index, fixture));
        }
    }

    code.push_str(
        "end
",
    );

    fs::write(app_dir.join("main.rb"), code).context("Failed to write Ruby app file")?;
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
            let mut fixtures = load_fixtures_from_dir(&path)
                .with_context(|| format!("Failed to load fixtures from {}", path.display()))?;
            fixtures.sort_by(|a, b| a.name.cmp(&b.name));
            grouped.insert(category, fixtures);
        }
    }

    Ok(grouped)
}

fn build_fixture_function(category: &str, index: usize, fixture: &Fixture) -> String {
    let method_name = build_method_name(category, index, &fixture.name);
    let handler_name = build_handler_name(category, index, &fixture.name);
    let route = fixture
        .handler
        .as_ref()
        .map(|h| h.route.as_str())
        .unwrap_or_else(|| fixture.request.path.as_str());
    let method = fixture
        .handler
        .as_ref()
        .map(|h| h.method.as_str())
        .unwrap_or_else(|| fixture.request.method.as_str());
    let method = method.to_ascii_lowercase();

    let mut args = Vec::new();
    args.push(format!("handler_name: {}", string_literal(&handler_name)));

    if let Some(handler) = fixture.handler.as_ref() {
        if let Some(parameters) = handler.parameters.as_ref() {
            if let Some(param_schema) = build_parameter_schema(parameters) {
                args.push(format!("parameter_schema: {}", value_to_ruby(&param_schema)));
            }
            if let Some(file_params) = extract_file_params(parameters) {
                args.push(format!("file_params: {}", value_to_ruby(&file_params)));
            }
        }
        if let Some(body_schema) = handler.body_schema.as_ref() {
            args.push(format!("request_schema: {}", value_to_ruby(body_schema)));
        }
        if let Some(cors) = handler.cors.as_ref() {
            args.push(format!("cors: {}", value_to_ruby(cors)));
        }
    }

    let mut function = String::new();
    function.push_str(&format!(
        "  def {}
",
        method_name
    ));
    function.push_str(
        "    app = Spikard::App.new
",
    );

    if args.len() == 1 {
        function.push_str(&format!(
            "    app.{}({}) do |_request|
",
            method,
            string_literal(route)
        ));
    } else {
        let args_joined = args.join(", ");
        function.push_str(&format!(
            "    app.{}({}, {}) do |_request|
",
            method,
            string_literal(route),
            args_joined
        ));
    }

    let response_expr = build_response_expression(&fixture.expected_response);
    function.push_str(&format!(
        "      {}
",
        response_expr
    ));
    function.push_str(
        "    end
",
    );
    function.push_str(
        "    app
",
    );
    function.push_str(
        "  end

",
    );

    function
}

fn build_parameter_schema(params: &Value) -> Option<Value> {
    let obj = params.as_object()?;
    let mut properties = Map::new();
    let mut required_fields: Vec<Value> = Vec::new();

    for (section_key, source) in [
        ("path", "path"),
        ("query", "query"),
        ("headers", "header"),
        ("cookies", "cookie"),
    ] {
        if let Some(section) = obj.get(section_key).and_then(|v| v.as_object()) {
            for (name, schema_value) in section {
                let mut schema_obj = match schema_value {
                    Value::Object(map) => map.clone(),
                    Value::String(s) => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), Value::String(s.clone()));
                        map
                    }
                    Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Null => {
                        let mut map = Map::new();
                        map.insert("type".to_string(), schema_value.clone());
                        map
                    }
                };

                schema_obj
                    .entry("type".to_string())
                    .or_insert_with(|| Value::String("string".to_string()));
                schema_obj.insert("source".to_string(), Value::String(source.to_string()));

                let is_optional = schema_obj.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required_flag = schema_obj
                    .get("required")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(!is_optional);

                schema_obj.remove("optional");
                schema_obj.remove("required");

                properties.insert(name.clone(), Value::Object(schema_obj));

                let should_require = if source == "path" {
                    true
                } else {
                    is_required_flag && !is_optional
                };

                if should_require {
                    required_fields.push(Value::String(name.clone()));
                }
            }
        }
    }

    if properties.is_empty() {
        return None;
    }

    let mut schema = Map::new();
    schema.insert("type".to_string(), Value::String("object".to_string()));
    schema.insert("properties".to_string(), Value::Object(properties));

    if !required_fields.is_empty() {
        schema.insert("required".to_string(), Value::Array(required_fields));
    }

    Some(Value::Object(schema))
}

fn extract_file_params(params: &Value) -> Option<Value> {
    params.as_object().and_then(|obj| obj.get("files")).cloned()
}

fn build_response_expression(expected: &FixtureExpectedResponse) -> String {
    let status = expected.status_code;
    let body_code = expected
        .body
        .as_ref()
        .map(value_to_ruby)
        .unwrap_or_else(|| "nil".to_string());
    "build_response(content: ".to_string() + &body_code + &format!(", status: {}, headers: nil)", status)
}
