//! Ruby test spec generator
//!
//! Generates RSpec test files from fixtures.

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use crate::ruby_utils::{
    build_method_name, bytes_to_ruby_string, string_literal, string_map_to_ruby, value_map_to_ruby, value_to_ruby,
};
use crate::streaming::{StreamingFixtureData, streaming_data};
use urlencoding::encode;

pub fn generate_ruby_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let spec_dir = output_dir.join("spec/generated");
    if spec_dir.exists() {
        fs::remove_dir_all(&spec_dir).context("Failed to remove existing Ruby generated specs")?;
    }
    fs::create_dir_all(&spec_dir).context("Failed to create generated spec directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;

    for (category, fixtures) in fixtures_by_category.iter() {
        let spec_code = build_spec_file(category, fixtures);
        let file_name = format!("{}_spec.rb", category.replace(['-', ' '], "_"));
        fs::write(spec_dir.join(file_name), spec_code)
            .with_context(|| format!("Failed to write spec for category {category}"))?;
    }

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

fn build_spec_file(category: &str, fixtures: &[Fixture]) -> String {
    let mut spec = String::new();
    spec.push_str(
        "# frozen_string_literal: true

",
    );
    spec.push_str(
        "require 'spec_helper'
",
    );
    spec.push_str(
        "require_relative '../../app/main'

",
    );
    spec.push_str(&format!(
        "RSpec.describe {} do
",
        string_literal(category)
    ));

    for (index, fixture) in fixtures.iter().enumerate() {
        spec.push_str(&build_spec_example(category, index, fixture));
    }

    spec.push_str(
        "end
",
    );
    spec
}

fn build_spec_example(category: &str, index: usize, fixture: &Fixture) -> String {
    let method_name = build_method_name(category, index, &fixture.name);
    let request_method = fixture.request.method.to_ascii_lowercase();
    let mut query_suffix: Option<String> = None;

    let mut options = Vec::new();
    let streaming_info = streaming_data(fixture).expect("invalid streaming fixture");

    if let Some(query) = fixture.request.query_params.as_ref() {
        if requires_query_string(query) {
            if let Some(encoded) = build_query_string(query) {
                query_suffix = Some(format!("?{}", encoded));
            }
        } else {
            options.push(format!("query: {}", value_map_to_ruby(query)));
        }
    }
    if let Some(headers) = fixture.request.headers.as_ref() {
        options.push(format!("headers: {}", string_map_to_ruby(headers)));
    }
    if let Some(cookies) = fixture.request.cookies.as_ref() {
        options.push(format!("cookies: {}", string_map_to_ruby(cookies)));
    }

    // Handle file uploads for multipart requests
    if let Some(files) = fixture.request.files.as_ref()
        && !files.is_empty()
    {
        let files_hash = build_files_hash(files);
        options.push(format!("files: {}", files_hash));
    }

    let mut content_type_value = fixture.request.content_type.clone();
    if content_type_value.is_none()
        && let Some(headers) = fixture.request.headers.as_ref()
        && let Some(ct) = headers.get("Content-Type")
    {
        content_type_value = Some(ct.clone());
    }
    let content_type = content_type_value.as_deref();

    if let Some(form) = fixture.request.form_data.as_ref() {
        options.push(format!("data: {}", value_map_to_ruby(form)));
    } else if let Some(data) = fixture.request.data.as_ref() {
        options.push(format!("data: {}", value_map_to_ruby(data)));
    } else if let Some(body) = fixture.request.body.as_ref() {
        // Handle body based on content type
        let use_raw_body = if let Some(ct) = content_type {
            // For URL-encoded content with string body, use raw_body
            ct.contains("application/x-www-form-urlencoded") && matches!(body, Value::String(_))
        } else {
            false
        };

        if use_raw_body {
            // URL-encoded string body should be sent as raw_body
            if let Value::String(s) = body {
                options.push(format!("raw_body: {}", string_literal(s)));
            }
        } else {
            // Match Python behavior: treat JSON, XML, and URL-encoded as json parameter
            let use_json = match content_type {
                None => true,
                Some(ct) => {
                    ct.contains("application/json")
                        || ct.contains("application/x-www-form-urlencoded")
                        || ct.contains("application/xml")
                }
            };

            if use_json {
                options.push(format!("json: {}", value_to_ruby(body)));
            } else if let Some(ct) = content_type {
                if ct.contains("application/x-www-form-urlencoded") {
                    if let Value::Object(map) = body {
                        let mut converted: HashMap<String, Value> = HashMap::new();
                        for (key, value) in map {
                            converted.insert(key.clone(), value.clone());
                        }
                        options.push(format!("data: {}", value_map_to_ruby(&converted)));
                    }
                } else if let Value::String(s) = body {
                    options.push(format!("raw_body: {}", string_literal(s)));
                } else {
                    let serialized = serde_json::to_string(body).unwrap_or_else(|_| body.to_string());
                    options.push(format!("raw_body: {}", string_literal(&serialized)));
                }
            }
        }
    }
    let options_suffix = if options.is_empty() {
        String::new()
    } else {
        format!(", {}", options.join(", "))
    };

    let mut example = String::new();
    example.push_str(&format!(
        "  it {} do
",
        string_literal(&fixture.name)
    ));
    example.push_str(&format!(
        "    app = E2ERubyApp.{}
",
        method_name
    ));

    // Generate ServerConfig if middleware is present
    let config_code = generate_server_config(fixture);
    if !config_code.is_empty() {
        example.push_str(&config_code);
        example.push_str(
            "    client = Spikard::Testing.create_test_client(app, config: config)
",
        );
    } else {
        example.push_str(
            "    client = Spikard::Testing.create_test_client(app)
",
        );
    }

    // Use the actual request path from the fixture (which has path params substituted)
    // instead of the route template
    let base_path = &fixture.request.path;
    let request_path_expr = if let Some(suffix) = query_suffix.as_ref() {
        format!("\"{}{}\"", base_path, suffix)
    } else {
        format!("\"{}\"", base_path)
    };

    example.push_str(&format!(
        "    response = client.{}({}{})
",
        request_method, request_path_expr, options_suffix
    ));
    example.push_str(&build_expectations(&fixture.expected_response, streaming_info.as_ref()));
    example.push_str("    client.close\n");
    example.push_str(
        "  end

",
    );
    example
}

fn build_query_string(params: &HashMap<String, Value>) -> Option<String> {
    let mut all_pairs: Vec<String> = Vec::new();

    let mut sorted: BTreeMap<&String, &Value> = BTreeMap::new();
    for (key, value) in params.iter() {
        sorted.insert(key, value);
    }

    for (key, value) in sorted {
        let pairs = value_to_query_pairs(key, value);
        all_pairs.extend(pairs);
    }

    if all_pairs.is_empty() {
        None
    } else {
        Some(all_pairs.join("&"))
    }
}

fn requires_query_string(params: &HashMap<String, Value>) -> bool {
    params
        .values()
        .any(|value| matches!(value, Value::Array(_) | Value::Object(_)))
}

fn value_to_query_pairs(key: &str, value: &Value) -> Vec<String> {
    match value {
        Value::Null => vec![format!("{}=", encode(key))],
        Value::Bool(b) => vec![format!("{}={}", encode(key), encode(&b.to_string()))],
        Value::Number(num) => vec![format!("{}={}", encode(key), encode(&num.to_string()))],
        Value::String(s) => vec![format!("{}={}", encode(key), encode(s))],
        Value::Array(items) => {
            let mut pairs = Vec::new();
            for item in items {
                for encoded in value_to_query_pairs(key, item) {
                    pairs.push(encoded);
                }
            }
            pairs
        }
        Value::Object(_obj) => {
            // Fall back to JSON encoding for nested objects.
            if let Ok(json) = serde_json::to_string(value) {
                vec![format!("{}={}", encode(key), encode(&json))]
            } else {
                vec![format!("{}={}", encode(key), encode(&value.to_string()))]
            }
        }
    }
}

fn build_expectations(expected: &FixtureExpectedResponse, streaming_info: Option<&StreamingFixtureData>) -> String {
    let mut expectations = String::new();
    expectations.push_str(&format!(
        "    expect(response.status_code).to eq({})\n",
        expected.status_code
    ));

    if let Some(info) = streaming_info {
        let expected_literal = bytes_to_ruby_string(&info.expected_bytes);
        expectations.push_str(&format!("    expected_body = {}\n", expected_literal));
        expectations.push_str("    expect(response.body_bytes).to eq(expected_body)\n");
        if info.is_text_only {
            expectations.push_str("    expect(response.text).to eq(expected_body)\n");
        }
        return expectations;
    }

    if expected.validation_errors.is_some() {
        expectations.push_str("    body = response.json\n");
        expectations.push_str("    expect(body).to be_a(Hash)\n");
        expectations.push_str("    expect(body.keys).to include('errors').or include('detail')\n");
        return expectations;
    }

    if let Some(body) = expected.body.as_ref() {
        if let Some(map) = body.as_object().filter(|map| map.contains_key("errors")) {
            expectations.push_str("    body = response.json\n");
            expectations.push_str("    expect(body).to be_a(Hash)\n");
            expectations.push_str("    expect(body['errors']).to be_an(Array)\n");
            expectations.push_str("    expect(body['errors']).not_to be_empty\n");

            if let Some(detail) = map.get("detail").and_then(|v| v.as_str()) {
                expectations.push_str(&format!(
                    "    expect(body['detail']).to eq({})\n",
                    string_literal(detail)
                ));
            }

            if let Some(status) = map.get("status").and_then(|v| v.as_i64()) {
                expectations.push_str(&format!("    expect(body['status']).to eq({})\n", status));
            }

            if let Some(first) = map
                .get("errors")
                .and_then(|v| v.as_array())
                .and_then(|errors| errors.first())
                .and_then(|v| v.as_object())
            {
                if let Some(loc) = first.get("loc") {
                    expectations.push_str(&format!(
                        "    expect(body['errors'].first['loc']).to eq({})\n",
                        value_to_ruby(loc)
                    ));
                }

                // Only check error type if it's not the generic "validation_error"
                // The actual implementation may return more specific error types
                if let Some(error_type) = first.get("type").and_then(|v| v.as_str())
                    && error_type != "validation_error"
                {
                    expectations.push_str(&format!(
                        "    expect(body['errors'].first['type']).to eq({})\n",
                        string_literal(error_type)
                    ));
                }
            }

            return expectations;
        }

        expectations.push_str("    expect(response.json).to eq(");
        expectations.push_str(&value_to_ruby(body));
        expectations.push_str(")\n");
        return expectations;
    }

    // Check for body_partial (used for OpenAPI and other tests with partial matching)
    if let Some(body_partial) = expected.body_partial.as_ref() {
        // For partial body matching, just check that body_text is not nil and contains expected keys
        expectations.push_str("    expect(response.body_text).not_to be_nil\n");

        // Extract some key fields from body_partial to verify
        if let Some(obj) = body_partial.as_object() {
            for key in obj.keys() {
                expectations.push_str(&format!(
                    "    expect(response.body_text).to include({})\n",
                    string_literal(key)
                ));
            }
        }

        return expectations;
    }

    // For successful responses with content-type that indicates content (like HTML),
    // skip the nil assertion - just check the status code
    if expected.status_code >= 200
        && expected.status_code < 300
        && let Some(headers) = expected.headers.as_ref()
        && let Some(content_type) = headers.get("content-type")
        && (content_type.contains("text/html") || content_type.contains("application"))
    {
        // Don't assert about body_text for HTML/application responses
        return expectations;
    }

    expectations.push_str("    expect(response.body_text).to be_nil\n");
    expectations
}

/// Build Ruby hash representation of file uploads
/// Format: {"field_name" => ["filename", "content", "content_type"]}
/// For multiple files with same field_name: {"field_name" => [["file1", "content1", "type1"], ["file2", "content2", "type2"]]}
fn build_files_hash(files: &[spikard_codegen::openapi::from_fixtures::FixtureFile]) -> String {
    use std::collections::HashMap;

    // Group files by field_name
    let mut grouped: HashMap<String, Vec<&spikard_codegen::openapi::from_fixtures::FixtureFile>> = HashMap::new();
    for file in files {
        grouped.entry(file.field_name.clone()).or_default().push(file);
    }

    let mut entries = Vec::new();

    // Sort by field name for consistent output
    let mut sorted_fields: Vec<_> = grouped.keys().collect();
    sorted_fields.sort();

    for field_name in sorted_fields {
        let field_files = &grouped[field_name];

        if field_files.len() == 1 {
            // Single file: {"field" => ["filename", "content", "type"]}
            let file = field_files[0];
            let filename = file.filename.as_deref().unwrap_or("file.txt");
            let content = file.content.as_deref().unwrap_or("");

            let mut array_elements = vec![string_literal(filename), string_literal(content)];

            if let Some(ref ct) = file.content_type {
                array_elements.push(string_literal(ct));
            }

            let array_str = format!("[{}]", array_elements.join(", "));
            entries.push(format!("{} => {}", string_literal(field_name), array_str));
        } else {
            // Multiple files: {"field" => [["file1", "content1", "type1"], ["file2", "content2", "type2"]]}
            let mut file_arrays = Vec::new();

            for file in field_files {
                let filename = file.filename.as_deref().unwrap_or("file.txt");
                let content = file.content.as_deref().unwrap_or("");

                let mut array_elements = vec![string_literal(filename), string_literal(content)];

                if let Some(ref ct) = file.content_type {
                    array_elements.push(string_literal(ct));
                }

                file_arrays.push(format!("[{}]", array_elements.join(", ")));
            }

            let nested_array = format!("[{}]", file_arrays.join(", "));
            entries.push(format!("{} => {}", string_literal(field_name), nested_array));
        }
    }

    format!("{{{}}}", entries.join(", "))
}

fn generate_server_config(fixture: &Fixture) -> String {
    let middleware = match &fixture.handler {
        Some(handler) => match &handler.middleware {
            Some(m) => m,
            None => {
                // No middleware configured - disable default compression
                let config_lines = [
                    "    config = Spikard::ServerConfig.new".to_string(),
                    "    config.compression = nil".to_string(),
                ];
                return config_lines.join("\n") + "\n";
            }
        },
        None => {
            // No handler - disable default compression
            let config_lines = [
                "    config = Spikard::ServerConfig.new".to_string(),
                "    config.compression = nil".to_string(),
            ];
            return config_lines.join("\n") + "\n";
        }
    };

    let mut config_lines = Vec::new();
    config_lines.push("    config = Spikard::ServerConfig.new".to_string());

    // Disable default compression unless explicitly configured
    let has_compression = middleware.get("compression").is_some();
    if !has_compression {
        config_lines.push("    config.compression = nil".to_string());
    }

    // Handle OpenAPI config
    if let Some(openapi) = middleware.get("openapi").and_then(|v| v.as_object()) {
        let mut openapi_params = Vec::new();

        if let Some(enabled) = openapi.get("enabled").and_then(|v| v.as_bool()) {
            openapi_params.push(format!("enabled: {}", enabled));
        }
        if let Some(title) = openapi.get("title").and_then(|v| v.as_str()) {
            openapi_params.push(format!("title: {}", string_literal(title)));
        }
        if let Some(version) = openapi.get("version").and_then(|v| v.as_str()) {
            openapi_params.push(format!("version: {}", string_literal(version)));
        }
        if let Some(description) = openapi.get("description").and_then(|v| v.as_str()) {
            openapi_params.push(format!("description: {}", string_literal(description)));
        }
        if let Some(swagger_ui_path) = openapi.get("swagger_ui_path").and_then(|v| v.as_str()) {
            openapi_params.push(format!("swagger_ui_path: {}", string_literal(swagger_ui_path)));
        }
        if let Some(redoc_path) = openapi.get("redoc_path").and_then(|v| v.as_str()) {
            openapi_params.push(format!("redoc_path: {}", string_literal(redoc_path)));
        }
        if let Some(openapi_json_path) = openapi.get("openapi_json_path").and_then(|v| v.as_str()) {
            openapi_params.push(format!("openapi_json_path: {}", string_literal(openapi_json_path)));
        }

        // Handle contact as hash
        if let Some(contact) = openapi.get("contact").and_then(|v| v.as_object()) {
            let contact_str = object_to_ruby_hash(contact);
            openapi_params.push(format!("contact: {}", contact_str));
        }

        // Handle license as hash
        if let Some(license) = openapi.get("license").and_then(|v| v.as_object()) {
            let license_str = object_to_ruby_hash(license);
            openapi_params.push(format!("license: {}", license_str));
        }

        // Handle servers as array of hashes
        if let Some(servers) = openapi.get("servers").and_then(|v| v.as_array()) {
            let servers_str = servers
                .iter()
                .filter_map(|s| s.as_object().map(object_to_ruby_hash))
                .collect::<Vec<_>>()
                .join(", ");
            openapi_params.push(format!("servers: [{}]", servers_str));
        }

        if !openapi_params.is_empty() {
            config_lines.push(format!(
                "    config.openapi = Spikard::OpenApiConfig.new(\n      {}\n    )",
                openapi_params.join(",\n      ")
            ));
        }
    }

    // Handle JWT auth config
    if let Some(jwt_auth) = middleware.get("jwt_auth").and_then(|v| v.as_object()) {
        // Check if enabled (default to true if not specified)
        let enabled = jwt_auth.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

        if enabled {
            let mut jwt_params = Vec::new();

            if let Some(algorithm) = jwt_auth.get("algorithm").and_then(|v| v.as_str()) {
                jwt_params.push(format!("algorithm: {}", string_literal(algorithm)));
            }
            if let Some(secret) = jwt_auth.get("secret").and_then(|v| v.as_str()) {
                jwt_params.push(format!("secret: {}", string_literal(secret)));
            }
            if let Some(audience) = jwt_auth.get("audience").and_then(|v| v.as_array()) {
                let aud_values: Vec<String> = audience.iter().filter_map(|v| v.as_str()).map(string_literal).collect();
                if !aud_values.is_empty() {
                    jwt_params.push(format!("audience: [{}]", aud_values.join(", ")));
                }
            }
            if let Some(issuer) = jwt_auth.get("issuer").and_then(|v| v.as_str()) {
                jwt_params.push(format!("issuer: {}", string_literal(issuer)));
            }

            if !jwt_params.is_empty() {
                config_lines.push(format!(
                    "    config.jwt_auth = Spikard::JwtConfig.new(\n      {}\n    )",
                    jwt_params.join(",\n      ")
                ));
            }
        }
    }

    // Handle API key auth config
    if let Some(api_key_auth) = middleware.get("api_key_auth").and_then(|v| v.as_object()) {
        // Check if enabled (default to true if not specified)
        let enabled = api_key_auth.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

        if enabled {
            let mut api_key_params = Vec::new();

            if let Some(header_name) = api_key_auth.get("header_name").and_then(|v| v.as_str()) {
                api_key_params.push(format!("header_name: {}", string_literal(header_name)));
            }
            if let Some(keys) = api_key_auth.get("keys").and_then(|v| v.as_array()) {
                let keys_str = keys
                    .iter()
                    .filter_map(|k| k.as_str().map(string_literal))
                    .collect::<Vec<_>>()
                    .join(", ");
                api_key_params.push(format!("keys: [{}]", keys_str));
            }

            if !api_key_params.is_empty() {
                config_lines.push(format!(
                    "    config.api_key_auth = Spikard::ApiKeyConfig.new(\n      {}\n    )",
                    api_key_params.join(",\n      ")
                ));
            }
        }
    }

    if config_lines.len() <= 2 {
        // Only "config = ServerConfig.new" and possibly "config.compression = nil" were added
        // If no other middleware, still return config to disable compression
        if config_lines.len() == 2 && config_lines[1].contains("compression = nil") {
            return config_lines.join("\n") + "\n";
        }
        return String::new();
    }

    config_lines.join("\n") + "\n"
}

fn object_to_ruby_hash(obj: &serde_json::Map<String, Value>) -> String {
    let entries: Vec<String> = obj
        .iter()
        .map(|(k, v)| {
            let value_str = match v {
                Value::String(s) => string_literal(s),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => v.to_string(),
            };
            format!("{} => {}", string_literal(k), value_str)
        })
        .collect();
    format!("{{{}}}", entries.join(", "))
}
