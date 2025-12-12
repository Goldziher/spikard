//! Ruby test app generator
//!
//! Generates Ruby Spikard applications based on fixtures.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::{BackgroundFixtureData, background_data};
use crate::dependencies::{Dependency, DependencyConfig, has_cleanup};
use crate::middleware::{MiddlewareMetadata, parse_middleware, write_static_assets};
use anyhow::{Context, Result};
use serde_json::{Map, Value};
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use crate::ruby_utils::{
    build_handler_name, build_method_name, bytes_to_ruby_string, sanitize_identifier, string_literal,
    string_map_to_ruby, value_to_ruby,
};
use crate::streaming::chunk_bytes;

pub fn generate_ruby_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create Ruby app directory")?;
    let static_root = app_dir.join("static_assets");
    if static_root.exists() {
        fs::remove_dir_all(&static_root).with_context(|| format!("Failed to clear {}", static_root.display()))?;
    }

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;
    let mut needs_background = false;
    let mut needs_cleanup_state = false;
    'outer: for fixtures in fixtures_by_category.values() {
        for fixture in fixtures {
            if background_data(fixture)?.is_some() {
                needs_background = true;
                if needs_cleanup_state {
                    break 'outer;
                }
            }
            if let Some(di_config) = DependencyConfig::from_fixture(fixture)? {
                if has_cleanup(&di_config) {
                    needs_cleanup_state = true;
                    if needs_background {
                        break 'outer;
                    }
                }
            }
        }
    }

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
    if needs_background {
        code.push_str("BACKGROUND_STATE = Hash.new { |hash, key| hash[key] = [] }\n\n");
    }
    if needs_cleanup_state {
        code.push_str("CLEANUP_STATE = Hash.new { |hash, key| hash[key] = [] }\n\n");
    }
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

    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;

    for (category, fixtures) in fixtures_by_category.iter() {
        for (index, fixture) in fixtures.iter().enumerate() {
            let metadata = parse_middleware(fixture)?;
            let fixture_dir = format!(
                "{}_{}",
                sanitize_identifier(category),
                sanitize_identifier(&fixture.name)
            );
            if !metadata.static_dirs.is_empty() {
                write_static_assets(&app_dir, &fixture_dir, &metadata.static_dirs)?;
            }
            let background_info = background_data(fixture)?;
            let di_config = DependencyConfig::from_fixture(fixture)?;

            if let Some(ref di_cfg) = di_config {
                let factories = generate_all_dependency_factories_ruby(di_cfg, &fixture_dir)?;
                if !factories.is_empty() {
                    code.push_str(&factories);
                }
            }

            code.push_str(&build_fixture_function(
                category,
                index,
                fixture,
                background_info,
                &metadata,
                &fixture_dir,
                di_config.as_ref(),
            )?);
        }
    }
    append_sse_factories(&mut code, &sse_fixtures)?;
    append_websocket_factories(&mut code, &websocket_fixtures)?;

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

fn build_fixture_function(
    category: &str,
    index: usize,
    fixture: &Fixture,
    background: Option<BackgroundFixtureData>,
    metadata: &MiddlewareMetadata,
    fixture_dir: &str,
    di_config: Option<&DependencyConfig>,
) -> Result<String> {
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

    let sanitized_headers = sanitized_expected_headers(&fixture.expected_response);
    let handler_status = if metadata.rate_limit.is_some() {
        200
    } else {
        fixture.expected_response.status_code
    };

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
    let raw_middleware = fixture.handler.as_ref().and_then(|handler| handler.middleware.as_ref());
    if let Some(config_literal) = build_server_config_literal(metadata, fixture_dir, raw_middleware)? {
        function.push_str(&format!("    config = {}\n", config_literal));
        function.push_str("    app = Spikard::App.new\n");
        function.push_str("    app.instance_variable_set(:@__spikard_test_config, config)\n");
    } else {
        function.push_str("    app = Spikard::App.new\n");
    }

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
    }

    let args_joined = args.join(", ");
    let skip_route_registration = !metadata.static_dirs.is_empty();

    let mut handler_params = Vec::new();
    handler_params.push("_request".to_string());
    if let Some(di_cfg) = di_config {
        let mut seen = std::collections::HashSet::new();
        for dep_key in &di_cfg.handler_dependencies {
            if seen.insert(dep_key.clone()) {
                handler_params.push(format!("{}:", dep_key));
            }
        }
    }
    let handler_params_str = handler_params.join(", ");

    if !skip_route_registration {
        let request_var = if background.is_some() { "request" } else { "_request" };
        function.push_str(&format!(
            "    app.{}({}, {}) do |{}|
",
            method,
            string_literal(route),
            args_joined,
            if di_config
                .as_ref()
                .map_or(false, |cfg| !cfg.handler_dependencies.is_empty())
            {
                &handler_params_str
            } else {
                request_var
            }
        ));

        if let Some(stmt) = request_timeout_sleep_statement(metadata) {
            function.push_str(&stmt);
        }

        if let Some(bg) = &background {
            function.push_str(&build_background_handler_block(
                &handler_name,
                request_var,
                fixture,
                bg,
                handler_status,
                &sanitized_headers,
            ));
        } else if fixture.streaming.is_some() {
            function.push_str(&build_streaming_response_block(
                fixture,
                handler_status,
                &sanitized_headers,
            )?);
        } else {
            let response_expr =
                build_response_expression(&fixture.expected_response, handler_status, &sanitized_headers);
            function.push_str(&format!("      {}\n", response_expr));
        }
        function.push_str(
            "    end
",
        );
    } else {
        function.push_str("    # Static files served via ServerConfig\n");
    }
    if let Some(bg) = &background {
        function.push_str(&format!(
            "    app.get({}, handler_name: {}) do |_req|
",
            string_literal(&bg.state_path),
            string_literal(&format!("{}_background_state", handler_name))
        ));
        function.push_str(&format!(
            "      build_response(content: {{ {} => BACKGROUND_STATE[{}] }}, status: 200)
",
            string_literal(&bg.state_key),
            string_literal(&handler_name)
        ));
        function.push_str(
            "    end
",
        );
    }

    if let Some(di_cfg) = di_config {
        let di_registration = generate_dependency_registration_ruby(di_cfg, fixture_dir)?;
        if !di_registration.is_empty() {
            function.push_str(&di_registration);
        }

        if has_cleanup(di_cfg) {
            function.push_str(&format!(
                "    app.get('/api/cleanup-state', handler_name: {}) do |_req|\n",
                string_literal(&format!("{}_cleanup_state", handler_name))
            ));
            function.push_str(&format!(
                "      build_response(content: {{ cleanup_events: CLEANUP_STATE[:{}] }}, status: 200)\n",
                fixture_dir
            ));
            function.push_str("    end\n");
        }
    }

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

fn append_sse_factories(code: &mut String, fixtures: &[AsyncFixture]) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
    }

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let app_fn_name = format!("create_app_sse_{}", slug);
        let handler_name = format!("sse_{}", slug);
        let events_literal = build_sse_events_literal(&channel_fixtures)?;

        code.push_str(&format!(
            "  def {app_fn_name}\n    app = Spikard::App.new\n    events = {events_literal}\n\n    app.get(\"{path}\", handler_name: \"{handler_name}\") do |_request|\n      stream = Enumerator.new do |yielder|\n        events.each do |payload|\n          yielder << \"data: #{{payload}}\\n\\n\"\n        end\n      end\n\n      Spikard::StreamingResponse.new(\n        stream,\n        status_code: 200,\n        headers: {{ \"content-type\" => \"text/event-stream\", \"cache-control\" => \"no-cache\" }}\n      )\n    end\n\n    app\n  end\n\n",
            app_fn_name = app_fn_name,
            events_literal = events_literal,
            path = channel_path,
            handler_name = handler_name
        ));
    }

    Ok(())
}

fn build_sse_events_literal(fixtures: &[&AsyncFixture]) -> Result<String> {
    let mut entries = Vec::new();
    for fixture in fixtures {
        for example in &fixture.examples {
            let json = serde_json::to_string(example)?;
            entries.push(string_literal(&json));
        }
    }
    if entries.is_empty() {
        entries.push("\"{}\"".to_string());
    }
    Ok(format!("[{}]", entries.join(", ")))
}

fn append_websocket_factories(code: &mut String, fixtures: &[AsyncFixture]) -> Result<()> {
    use std::collections::BTreeMap;

    if fixtures.is_empty() {
        return Ok(());
    }

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    for (channel, _channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let app_fn_name = format!("create_app_websocket_{}", slug);
        let handler_name = format!("websocket_{}", slug);

        code.push_str(&format!(
            r#"  def {app_fn_name}
    app = Spikard::App.new
    app.websocket("{path}", handler_name: "{handler_name}") do
      # Create handler object with handle_message method (message-based pattern)
      handler = Object.new
      def handler.handle_message(message)
        message['validated'] = true
        message
      end
      handler
    end
    app
  end

"#,
            app_fn_name = app_fn_name,
            path = channel_path,
            handler_name = handler_name
        ));
    }

    Ok(())
}

fn build_background_handler_block(
    handler_name: &str,
    request_var: &str,
    fixture: &Fixture,
    background: &BackgroundFixtureData,
    handler_status: u16,
    sanitized_headers: &HashMap<String, String>,
) -> String {
    let response_expr = build_response_expression(&fixture.expected_response, handler_status, sanitized_headers);
    format!(
        "      body = {request_var}[:body]
      raise ArgumentError, 'background handler requires JSON body' unless body.is_a?(Hash)
      value = body[{value_field}]
      raise ArgumentError, 'background handler missing value' if value.nil?
      Spikard::Background.run do
        BACKGROUND_STATE[{handler_key}] << value
      end
      {response_expr}
",
        request_var = request_var,
        value_field = string_literal(&background.value_field),
        handler_key = string_literal(handler_name),
        response_expr = response_expr
    )
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

    schema.insert("required".to_string(), Value::Array(required_fields));

    Some(Value::Object(schema))
}

fn extract_file_params(params: &Value) -> Option<Value> {
    params.as_object().and_then(|obj| obj.get("files")).cloned()
}

fn build_response_expression(
    expected: &FixtureExpectedResponse,
    handler_status: u16,
    sanitized_headers: &HashMap<String, String>,
) -> String {
    let expected_status = expected.status_code;
    let headers_literal = if sanitized_headers.is_empty() {
        "nil".to_string()
    } else {
        string_map_to_ruby(sanitized_headers)
    };

    let body_code = if let Some(ref validation_errors) = expected.validation_errors {
        let count = validation_errors.len();
        let detail = if count == 1 {
            "1 validation error in request"
        } else {
            &format!("{} validation errors in request", count)
        };

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
        response_map.insert("status".to_string(), Value::Number(expected_status.into()));
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

    format!(
        "build_response(content: {body}, status: {status}, headers: {headers})",
        body = body_code,
        status = handler_status,
        headers = headers_literal
    )
}

fn build_streaming_response_block(
    fixture: &Fixture,
    handler_status: u16,
    sanitized_headers: &HashMap<String, String>,
) -> Result<String> {
    let streaming = fixture.streaming.as_ref().ok_or_else(|| {
        anyhow::anyhow!("Fixture must have streaming metadata when building streaming response block")
    })?;

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

    let headers_literal = build_streaming_headers_ruby(fixture, sanitized_headers);
    block.push_str(&format!(
        "      Spikard::StreamingResponse.new(\n        stream,\n        status_code: {},\n        headers: {}\n      )\n",
        handler_status,
        headers_literal
    ));

    Ok(block)
}

fn build_streaming_headers_ruby(fixture: &Fixture, sanitized_headers: &HashMap<String, String>) -> String {
    let mut headers = sanitized_headers.clone();

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

    headers_ruby_literal(&headers)
}

/// Generate Ruby lifecycle hook procs
fn generate_lifecycle_hooks_ruby(hooks: &Value, fixture: &Fixture) -> Vec<String> {
    let mut hook_code = Vec::new();

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

fn sanitized_expected_headers(expected: &FixtureExpectedResponse) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(expected_headers) = expected.headers.as_ref() {
        for (key, value) in expected_headers {
            if key.eq_ignore_ascii_case("content-encoding") {
                continue;
            }
            if let Some(converted) = normalize_expected_header_value(value) {
                headers.insert(key.clone(), converted);
            }
        }
    }
    headers
}

fn normalize_expected_header_value(value: &str) -> Option<String> {
    match value {
        "<<absent>>" => None,
        "<<present>>" => Some("spikard-test-value".to_string()),
        "<<uuid>>" => Some("00000000-0000-4000-8000-000000000000".to_string()),
        _ => Some(value.to_string()),
    }
}

fn headers_ruby_literal(headers: &HashMap<String, String>) -> String {
    if headers.is_empty() {
        "nil".to_string()
    } else {
        string_map_to_ruby(headers)
    }
}

fn build_server_config_literal(
    metadata: &MiddlewareMetadata,
    fixture_dir: &str,
    raw_middleware: Option<&Value>,
) -> Result<Option<String>> {
    let mut args = Vec::new();

    if let Some(compression) = &metadata.compression {
        let mut parts = Vec::new();
        if let Some(gzip) = compression.gzip {
            parts.push(format!("gzip: {}", if gzip { "true" } else { "false" }));
        }
        if let Some(brotli) = compression.brotli {
            parts.push(format!("brotli: {}", if brotli { "true" } else { "false" }));
        }
        if let Some(min_size) = compression.min_size {
            parts.push(format!("min_size: {}", min_size));
        }
        if let Some(quality) = compression.quality {
            parts.push(format!("quality: {}", quality));
        }
        let literal = if parts.is_empty() {
            "Spikard::CompressionConfig.new".to_string()
        } else {
            format!("Spikard::CompressionConfig.new({})", parts.join(", "))
        };
        args.push(format!("      compression: {}", literal));
    }

    if let Some(rate_limit) = &metadata.rate_limit {
        let mut parts = vec![
            format!("per_second: {}", rate_limit.per_second),
            format!("burst: {}", rate_limit.burst),
        ];
        if let Some(ip_based) = rate_limit.ip_based {
            parts.push(format!("ip_based: {}", if ip_based { "true" } else { "false" }));
        }
        args.push(format!(
            "      rate_limit: Spikard::RateLimitConfig.new({})",
            parts.join(", ")
        ));
    }

    if let Some(timeout) = &metadata.request_timeout {
        args.push(format!("      request_timeout: {}", timeout.seconds));
    }

    if let Some(request_id) = &metadata.request_id
        && let Some(enabled) = request_id.enabled
    {
        args.push(format!(
            "      enable_request_id: {}",
            if enabled { "true" } else { "false" }
        ));
    }

    if let Some(body_limit) = &metadata.body_limit {
        match body_limit.max_bytes {
            Some(bytes) => args.push(format!("      max_body_size: {}", bytes)),
            None => args.push("      max_body_size: nil".to_string()),
        }
    }

    if !metadata.static_dirs.is_empty() {
        let mut entries = Vec::new();
        for dir in &metadata.static_dirs {
            let directory_literal = format!(
                "File.expand_path(File.join(__dir__, \"static_assets\", \"{}\", \"{}\"))",
                fixture_dir, dir.directory_name
            );
            let mut parts = vec![
                format!("directory: {}", directory_literal),
                format!("route_prefix: {}", string_literal(&dir.route_prefix)),
            ];
            if !dir.index_file {
                parts.push("index_file: false".to_string());
            }
            if let Some(cache) = &dir.cache_control {
                parts.push(format!("cache_control: {}", string_literal(cache)));
            }
            entries.push(format!("        Spikard::StaticFilesConfig.new({})", parts.join(", ")));
        }
        if !entries.is_empty() {
            let block = format!("      static_files: [\n{}\n      ]", entries.join(",\n"));
            args.push(block);
        }
    }

    if let Some(middleware) = raw_middleware {
        if let Some(openapi) = middleware.get("openapi").and_then(|v| v.as_object())
            && let Some(block) = build_openapi_config_literal(openapi)
        {
            args.push(block);
        }
        if let Some(jwt) = middleware.get("jwt_auth").and_then(|v| v.as_object())
            && let Some(block) = build_jwt_config_literal(jwt)
        {
            args.push(block);
        }
        if let Some(api_key) = middleware.get("api_key_auth").and_then(|v| v.as_object())
            && let Some(block) = build_api_key_config_literal(api_key)
        {
            args.push(block);
        }
    }

    if args.is_empty() {
        return Ok(None);
    }

    let mut literal = String::from("Spikard::ServerConfig.new(\n");
    literal.push_str(&args.join(",\n"));
    literal.push('\n');
    literal.push_str("    )");
    Ok(Some(literal))
}

fn build_openapi_config_literal(openapi: &serde_json::Map<String, Value>) -> Option<String> {
    let mut parts = Vec::new();

    if let Some(enabled) = openapi.get("enabled").and_then(|v| v.as_bool()) {
        parts.push(format!("enabled: {}", if enabled { "true" } else { "false" }));
    }
    if let Some(title) = openapi.get("title").and_then(|v| v.as_str()) {
        parts.push(format!("title: {}", string_literal(title)));
    }
    if let Some(version) = openapi.get("version").and_then(|v| v.as_str()) {
        parts.push(format!("version: {}", string_literal(version)));
    }
    if let Some(description) = openapi.get("description").and_then(|v| v.as_str()) {
        parts.push(format!("description: {}", string_literal(description)));
    }
    if let Some(swagger_ui) = openapi.get("swagger_ui_path").and_then(|v| v.as_str()) {
        parts.push(format!("swagger_ui_path: {}", string_literal(swagger_ui)));
    }
    if let Some(redoc_path) = openapi.get("redoc_path").and_then(|v| v.as_str()) {
        parts.push(format!("redoc_path: {}", string_literal(redoc_path)));
    }
    if let Some(openapi_json_path) = openapi.get("openapi_json_path").and_then(|v| v.as_str()) {
        parts.push(format!("openapi_json_path: {}", string_literal(openapi_json_path)));
    }

    if let Some(contact) = openapi.get("contact").and_then(|v| v.as_object()) {
        parts.push(format!(
            "contact: Spikard::ContactInfo.new({})",
            build_keyword_args(contact)
        ));
    }

    if let Some(license) = openapi.get("license").and_then(|v| v.as_object()) {
        parts.push(format!(
            "license: Spikard::LicenseInfo.new({})",
            build_keyword_args(license)
        ));
    }

    if let Some(servers) = openapi.get("servers").and_then(|v| v.as_array()) {
        let mut entries = Vec::new();
        for server in servers {
            if let Some(obj) = server.as_object() {
                entries.push(format!("Spikard::ServerInfo.new({})", build_keyword_args(obj)));
            }
        }
        if !entries.is_empty() {
            parts.push(format!("servers: [{}]", entries.join(", ")));
        }
    }

    if parts.is_empty() {
        return None;
    }

    Some(format!(
        "      openapi: Spikard::OpenApiConfig.new(\n        {}\n      )",
        parts.join(",\n        ")
    ))
}

fn build_jwt_config_literal(jwt: &serde_json::Map<String, Value>) -> Option<String> {
    let enabled = jwt.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    if !enabled {
        return None;
    }
    let mut parts = Vec::new();
    if let Some(secret) = jwt.get("secret").and_then(|v| v.as_str()) {
        parts.push(format!("secret: {}", string_literal(secret)));
    }
    if let Some(algorithm) = jwt.get("algorithm").and_then(|v| v.as_str()) {
        parts.push(format!("algorithm: {}", string_literal(algorithm)));
    }
    if let Some(audience) = jwt.get("audience") {
        parts.push(format!("audience: {}", value_to_ruby(audience)));
    }
    if let Some(issuer) = jwt.get("issuer").and_then(|v| v.as_str()) {
        parts.push(format!("issuer: {}", string_literal(issuer)));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!(
        "      jwt_auth: Spikard::JwtConfig.new(\n        {}\n      )",
        parts.join(",\n        ")
    ))
}

fn build_api_key_config_literal(api_key: &serde_json::Map<String, Value>) -> Option<String> {
    let enabled = api_key.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    if !enabled {
        return None;
    }
    let mut parts = Vec::new();
    if let Some(header) = api_key.get("header_name").and_then(|v| v.as_str()) {
        parts.push(format!("header_name: {}", string_literal(header)));
    }
    if let Some(keys) = api_key.get("keys") {
        parts.push(format!("keys: {}", value_to_ruby(keys)));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!(
        "      api_key_auth: Spikard::ApiKeyConfig.new(\n        {}\n      )",
        parts.join(",\n        ")
    ))
}

fn build_keyword_args(obj: &serde_json::Map<String, Value>) -> String {
    let mut parts = Vec::new();
    for (key, value) in obj {
        parts.push(format!("{}: {}", key, value_to_ruby(value)));
    }
    parts.join(", ")
}

fn request_timeout_sleep_statement(metadata: &MiddlewareMetadata) -> Option<String> {
    let sleep_ms = metadata.request_timeout.as_ref()?.sleep_ms?;
    Some(format!("      sleep({})\n", format_sleep_seconds(sleep_ms)))
}

#[allow(clippy::manual_is_multiple_of)]
fn format_sleep_seconds(ms: u64) -> String {
    if ms % 1000 == 0 {
        return format!("{}", ms / 1000);
    }
    let secs = (ms as f64) / 1000.0;
    let mut literal = format!("{:.3}", secs);
    while literal.contains('.') && literal.ends_with('0') {
        literal.pop();
    }
    if literal.ends_with('.') {
        literal.push('0');
    }
    literal
}

/// Generate Ruby factory function for a dependency
fn generate_dependency_factory_ruby(dep: &Dependency, fixture_id: &str) -> Result<String> {
    let factory_name = dep.factory.as_ref().unwrap_or(&dep.key);
    let is_cleanup = dep.cleanup;

    let mut code = String::new();

    if is_cleanup {
        code.push_str(&format!("  def {}(", factory_name));
        for (i, depend_key) in dep.depends_on.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&format!("{}:", depend_key));
        }
        code.push_str(")\n");
        code.push_str("    # Create resource\n");
        code.push_str(&format!("    CLEANUP_STATE[:{}] << 'session_opened'\n", fixture_id));
        code.push_str(&format!(
            "    resource = {{ id: '{:012x}', active: true }}\n",
            fixture_id.len()
        ));
        code.push_str("    \n");
        code.push_str("    # Return resource and cleanup proc\n");
        code.push_str("    cleanup_proc = -> do\n");
        code.push_str(&format!("      CLEANUP_STATE[:{}] << 'session_closed'\n", fixture_id));
        code.push_str("    end\n");
        code.push_str("    \n");
        code.push_str("    [resource, cleanup_proc]\n");
        code.push_str("  end\n\n");
    } else if dep.singleton {
        code.push_str(&format!("  def {}(", factory_name));
        for (i, depend_key) in dep.depends_on.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&format!("{}:", depend_key));
        }
        code.push_str(")\n");
        code.push_str("    # Singleton with counter\n");
        code.push_str(&format!("    singleton_key = 'singleton_{}'\n", dep.key));
        code.push_str("    BACKGROUND_STATE[singleton_key] ||= {\n");
        code.push_str("      id: '00000000-0000-0000-0000-000000000063',\n");
        code.push_str("      count: 0\n");
        code.push_str("    }\n");
        code.push_str("    BACKGROUND_STATE[singleton_key][:count] += 1\n");
        code.push_str("    BACKGROUND_STATE[singleton_key]\n");
        code.push_str("  end\n\n");
    } else {
        code.push_str(&format!("  def {}(", factory_name));
        for (i, depend_key) in dep.depends_on.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&format!("{}:", depend_key));
        }
        code.push_str(")\n");

        if dep.key.contains("auth") {
            code.push_str("    # Create auth service\n");
            if !dep.depends_on.is_empty() {
                code.push_str(&format!(
                    "    {{ {}_enabled: true, has_db: !{}.nil?, has_cache: !{}.nil? }}\n",
                    dep.key,
                    dep.depends_on.get(0).unwrap_or(&"nil".to_string()),
                    dep.depends_on.get(1).unwrap_or(&"nil".to_string())
                ));
            } else {
                code.push_str(&format!("    {{ {}_enabled: true, enabled: true }}\n", dep.key));
            }
        } else {
            code.push_str(&format!(
                "    {{ id: '{:012x}', type: '{}', timestamp: Time.now.to_s }}\n",
                dep.key.len(),
                dep.key
            ));
        }

        code.push_str("  end\n\n");
    }

    Ok(code)
}

/// Generate Ruby code to register dependencies in app
fn generate_dependency_registration_ruby(di_config: &DependencyConfig, _fixture_id: &str) -> Result<String> {
    if !di_config.has_dependencies() {
        return Ok(String::new());
    }

    let mut code = String::new();
    code.push_str("\n    # Register dependencies\n");

    let all_deps = di_config.all_dependencies();

    for (key, dep) in all_deps.iter() {
        if dep.is_value() {
            let value = ruby_dependency_value(dep)?;
            code.push_str(&format!("    app.provide({}, {})\n", string_literal(key), value));
        } else {
            let factory_name = dep.factory.as_ref().unwrap_or(key);
            let mut provide_args = Vec::new();
            provide_args.push(format!("method({})", string_literal(factory_name)));

            if !dep.depends_on.is_empty() {
                let deps_array = dep
                    .depends_on
                    .iter()
                    .map(|d| string_literal(d))
                    .collect::<Vec<_>>()
                    .join(", ");
                provide_args.push(format!("depends_on: [{}]", deps_array));
            }
            if dep.singleton {
                provide_args.push("singleton: true".to_string());
            }
            if dep.cacheable {
                provide_args.push("cacheable: true".to_string());
            }

            code.push_str(&format!(
                "    app.provide({}, Spikard::Provide.new({}))\n",
                string_literal(key),
                provide_args.join(", ")
            ));
        }
    }

    Ok(code)
}

/// Generate factory functions for all dependencies
fn generate_all_dependency_factories_ruby(di_config: &DependencyConfig, fixture_id: &str) -> Result<String> {
    let mut code = String::new();

    let all_deps = di_config.all_dependencies();

    for dep in all_deps.values() {
        if !dep.is_value() {
            code.push_str(&generate_dependency_factory_ruby(dep, fixture_id)?);
        }
    }

    Ok(code)
}

/// Convert dependency value to Ruby literal
fn ruby_dependency_value(dep: &Dependency) -> Result<String> {
    if let Some(ref value) = dep.value {
        Ok(value_to_ruby(value))
    } else {
        Ok("nil".to_string())
    }
}
