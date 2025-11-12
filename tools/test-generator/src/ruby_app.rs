//! Ruby test app generator
//!
//! Generates Ruby Spikard applications based on fixtures.

use anyhow::{Context, Result};
use serde_json::{Map, Value};
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use crate::ruby_utils::{
    build_handler_name, build_method_name, bytes_to_ruby_string, string_literal, string_map_to_ruby, value_to_ruby,
};
use crate::streaming::chunk_bytes;

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
            code.push_str(&build_fixture_function(category, index, fixture)?);
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

fn build_fixture_function(category: &str, index: usize, fixture: &Fixture) -> Result<String> {
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

    // Generate lifecycle hooks if present
    let hooks = fixture
        .handler
        .as_ref()
        .and_then(|h| h.middleware.as_ref())
        .and_then(|m| m.get("lifecycle_hooks"));

    if let Some(hooks) = hooks {
        let hook_procs = generate_lifecycle_hooks_ruby(hooks, fixture);
        for hook_code in hook_procs {
            function.push_str(&format!("{}\n", hook_code));
        }
        // TODO: Register hooks with app when Ruby API is implemented
        // function.push_str("    app.on_request(on_request_proc)\n");
        // etc.
    }

    let args_joined = args.join(", ");
    function.push_str(&format!(
        "    app.{}({}, {}) do |_request|
",
        method,
        string_literal(route),
        args_joined
    ));

    if fixture.streaming.is_some() {
        function.push_str(&build_streaming_response_block(fixture)?);
    } else {
        let response_expr = build_response_expression(&fixture.expected_response);
        function.push_str(&format!("      {}\n", response_expr));
    }
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

    Ok(function)
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

    // Always include required field, even if empty, for consistency
    schema.insert("required".to_string(), Value::Array(required_fields));

    Some(Value::Object(schema))
}

fn extract_file_params(params: &Value) -> Option<Value> {
    params.as_object().and_then(|obj| obj.get("files")).cloned()
}

fn build_response_expression(expected: &FixtureExpectedResponse) -> String {
    let status = expected.status_code;

    // If validation_errors is present, build a validation error response
    let body_code = if let Some(ref validation_errors) = expected.validation_errors {
        let count = validation_errors.len();
        let detail = if count == 1 {
            "1 validation error in request"
        } else {
            &format!("{} validation errors in request", count)
        };

        // Build errors array
        let mut errors = Vec::new();
        for err in validation_errors {
            let mut error_map = Map::new();
            error_map.insert("type".to_string(), Value::String(err.error_type.clone()));
            error_map.insert(
                "loc".to_string(),
                Value::Array(err.loc.iter().map(|s| Value::String(s.clone())).collect()),
            );
            error_map.insert("msg".to_string(), Value::String(err.msg.clone()));
            errors.push(Value::Object(error_map));
        }

        let mut response_map = Map::new();
        response_map.insert("detail".to_string(), Value::String(detail.to_string()));
        response_map.insert("errors".to_string(), Value::Array(errors));
        response_map.insert("status".to_string(), Value::Number(status.into()));
        response_map.insert(
            "title".to_string(),
            Value::String("Request Validation Failed".to_string()),
        );
        response_map.insert(
            "type".to_string(),
            Value::String("https://spikard.dev/errors/validation-error".to_string()),
        );

        value_to_ruby(&Value::Object(response_map))
    } else {
        expected
            .body
            .as_ref()
            .map(value_to_ruby)
            .unwrap_or_else(|| "nil".to_string())
    };

    "build_response(content: ".to_string() + &body_code + &format!(", status: {}, headers: nil)", status)
}

fn build_streaming_response_block(fixture: &Fixture) -> Result<String> {
    let streaming = fixture
        .streaming
        .as_ref()
        .expect("streaming metadata should exist when building streaming block");

    let mut block = String::new();
    block.push_str("      stream = Enumerator.new do |yielder|\n");
    for chunk in &streaming.chunks {
        match chunk {
            spikard_codegen::openapi::FixtureStreamChunk::Text { value } => {
                block.push_str(&format!("        yielder << {}\n", string_literal(value)));
            }
            spikard_codegen::openapi::FixtureStreamChunk::Bytes { .. } => {
                let bytes = chunk_bytes(chunk)?;
                block.push_str(&format!("        yielder << {}\n", bytes_to_ruby_string(&bytes)));
            }
        }
    }
    block.push_str("      end\n\n");

    let headers_literal = build_streaming_headers_ruby(fixture);
    block.push_str(&format!(
        "      Spikard::StreamingResponse.new(\n        stream,\n        status_code: {},\n        headers: {}\n      )\n",
        fixture.expected_response.status_code,
        headers_literal
    ));

    Ok(block)
}

fn build_streaming_headers_ruby(fixture: &Fixture) -> String {
    let mut headers: HashMap<String, String> = fixture.expected_response.headers.clone().unwrap_or_default();

    if let Some(content_type) = fixture
        .streaming
        .as_ref()
        .and_then(|streaming| streaming.content_type.as_ref())
    {
        headers.insert("content-type".to_string(), content_type.clone());
    }

    if !headers.contains_key("content-type") {
        headers.insert("content-type".to_string(), "application/octet-stream".to_string());
    }

    if headers.is_empty() {
        "nil".to_string()
    } else {
        string_map_to_ruby(&headers)
    }
}

/// Generate Ruby lifecycle hook procs
fn generate_lifecycle_hooks_ruby(hooks: &Value, fixture: &Fixture) -> Vec<String> {
    let mut hook_code = Vec::new();

    // Process on_request hooks
    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for hook in on_request {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");

            hook_code.push(format!(
                r#"    # onRequest hook: {}
    on_request_proc = lambda do |request|
      # Mock implementation
      request
    end"#,
                hook_name
            ));
        }
    }

    // Process pre_validation hooks
    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for hook in pre_validation {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");

            let should_short_circuit = hook_name.contains("rate_limit") && fixture.expected_response.status_code == 429;

            if should_short_circuit {
                hook_code.push(format!(
                    r#"    # preValidation hook: {} - Short circuits with 429
    pre_validation_proc = lambda do |_request|
      build_response(
        content: {{ error: "Rate limit exceeded", message: "Too many requests, please try again later" }},
        status: 429,
        headers: {{ "Retry-After" => "60" }}
      )
    end"#,
                    hook_name
                ));
            } else {
                hook_code.push(format!(
                    r#"    # preValidation hook: {}
    pre_validation_proc = lambda do |request|
      # Mock implementation
      request
    end"#,
                    hook_name
                ));
            }
        }
    }

    // Process pre_handler hooks
    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for hook in pre_handler {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");

            let auth_fails = hook_name.contains("auth")
                && (fixture.expected_response.status_code == 401 || fixture.expected_response.status_code == 403);

            if auth_fails {
                let (status_code, error_msg, detail_msg) = if fixture.expected_response.status_code == 401 {
                    (401, "Unauthorized", "Invalid or expired authentication token")
                } else {
                    (403, "Forbidden", "Admin role required for this endpoint")
                };

                hook_code.push(format!(
                    r#"    # preHandler hook: {} - Short circuits with {}
    pre_handler_proc = lambda do |_request|
      build_response(
        content: {{ error: "{}", message: "{}" }},
        status: {}
      )
    end"#,
                    hook_name, status_code, error_msg, detail_msg, status_code
                ));
            } else {
                hook_code.push(format!(
                    r#"    # preHandler hook: {}
    pre_handler_proc = lambda do |request|
      # Mock implementation
      request
    end"#,
                    hook_name
                ));
            }
        }
    }

    // Process on_response hooks
    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for hook in on_response {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");

            if hook_name.contains("security") {
                hook_code.push(format!(
                    r#"    # onResponse hook: {} - Adds security headers
    on_response_proc = lambda do |response|
      response.headers["X-Content-Type-Options"] = "nosniff"
      response.headers["X-Frame-Options"] = "DENY"
      response.headers["X-XSS-Protection"] = "1; mode=block"
      response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"
      response
    end"#,
                    hook_name
                ));
            } else if hook_name.contains("timing") || hook_name.contains("timer") {
                hook_code.push(format!(
                    r#"    # onResponse hook: {} - Adds timing header
    on_response_proc = lambda do |response|
      response.headers["X-Response-Time"] = "0ms"
      response
    end"#,
                    hook_name
                ));
            } else {
                hook_code.push(format!(
                    r#"    # onResponse hook: {}
    on_response_proc = lambda do |response|
      # Mock implementation
      response
    end"#,
                    hook_name
                ));
            }
        }
    }

    // Process on_error hooks
    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for hook in on_error {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");

            hook_code.push(format!(
                r#"    # onError hook: {}
    on_error_proc = lambda do |response|
      response.headers["Content-Type"] = "application/json"
      response
    end"#,
                hook_name
            ));
        }
    }

    hook_code
}
