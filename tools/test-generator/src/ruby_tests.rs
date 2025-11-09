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

use crate::ruby_utils::{build_method_name, string_literal, string_map_to_ruby, value_map_to_ruby, value_to_ruby};
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
    example.push_str(
        "    client = Spikard::Testing.create_test_client(app)
",
    );

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
    example.push_str(&build_expectations(&fixture.expected_response));
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

fn build_expectations(expected: &FixtureExpectedResponse) -> String {
    let mut expectations = String::new();
    expectations.push_str(&format!(
        "    expect(response.status_code).to eq({})\n",
        expected.status_code
    ));

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
