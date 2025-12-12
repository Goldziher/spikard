//! Ruby test spec generator
//!
//! Generates RSpec test files from fixtures.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::background_data;
use crate::dependencies::{DependencyConfig, has_cleanup, requires_multi_request_test};
use crate::middleware::parse_middleware;
use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use crate::ruby_utils::{
    build_method_name, bytes_to_ruby_string, sanitize_identifier, string_literal, string_map_to_ruby,
    value_map_to_ruby, value_to_ruby,
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

    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    if !sse_fixtures.is_empty() {
        let sse_spec = build_sse_spec(&sse_fixtures)?;
        fs::write(spec_dir.join("asyncapi_sse_spec.rb"), sse_spec).context("Failed to write asyncapi_sse_spec.rb")?;
    }

    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;
    if !websocket_fixtures.is_empty() {
        let websocket_spec = build_websocket_spec(&websocket_fixtures)?;
        fs::write(spec_dir.join("asyncapi_websocket_spec.rb"), websocket_spec)
            .context("Failed to write asyncapi_websocket_spec.rb")?;
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

fn build_sse_spec(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut spec = String::new();
    spec.push_str(
        "# frozen_string_literal: true\n\nrequire 'spec_helper'\nrequire 'json'\n\nRSpec.describe \"asyncapi_sse\" do\n",
    );

    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let factory_name = format!(
            "create_app_sse_{}",
            sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"))
        );
        let expected_literal = build_sse_expected_literal(channel_fixtures)?;

        spec.push_str(&format!(
            "  it \"streams events for {path}\" do\n    app = E2ERubyApp.{factory}\n    client = Spikard::Testing.create_test_client(app)\n\n    response = client.get(\"{path}\")\n    expect(response.status_code).to eq(200)\n    body = response.body_text\n    events = body.gsub(\"\\r\\n\", \"\\n\")\n                 .split(\"\\n\\n\")\n                 .select {{ |chunk| chunk.start_with?(\"data:\") }}\n                 .map {{ |chunk| chunk.sub(/^data:\\s*/, \"\").strip }}\n\n    expected = {expected_literal}\n    expect(events.length).to eq(expected.length)\n    events.zip(expected).each do |payload, expected_json|\n      expect(JSON.parse(payload)).to eq(JSON.parse(expected_json))\n    end\n\n    client.close\n  end\n\n",
            path = channel_path,
            factory = factory_name,
            expected_literal = expected_literal
        ));
    }

    spec.push_str("end\n");
    Ok(spec)
}

fn build_spec_example(category: &str, index: usize, fixture: &Fixture) -> String {
    let method_name = build_method_name(category, index, &fixture.name);
    if fixture_should_skip(category, fixture) {
        return format!(
            "  xit \"{}\" do\n    skip \"Not supported by the Ruby in-memory client\"\n  end\n\n",
            fixture.name
        );
    }
    let background_info = background_data(fixture).expect("invalid background fixture");
    let request_method = fixture.request.method.to_ascii_lowercase();
    let mut query_suffix: Option<String> = None;

    let mut options = Vec::new();
    let streaming_info = streaming_data(fixture).expect("invalid streaming fixture");
    let middleware = parse_middleware(fixture).expect("failed to parse middleware metadata");
    let sanitized_headers = sanitized_expected_headers(&fixture.expected_response);

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
        let use_raw_body = if let Some(ct) = content_type {
            ct.contains("application/x-www-form-urlencoded") && matches!(body, Value::String(_))
        } else {
            false
        };

        if use_raw_body {
            if let Value::String(s) = body {
                options.push(format!("raw_body: {}", string_literal(s)));
            }
        } else {
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

    let base_path = &fixture.request.path;
    let request_path_expr = if let Some(suffix) = query_suffix.as_ref() {
        format!("\"{}{}\"", base_path, suffix)
    } else {
        format!("\"{}\"", base_path)
    };

    let request_call = format!("client.{}({}{})", request_method, request_path_expr, options_suffix);

    if let Some(rate_limit) = middleware.rate_limit.as_ref()
        && rate_limit.warmup_requests > 0
    {
        example.push_str(&format!("    {}.times do\n", rate_limit.warmup_requests));
        example.push_str(&format!("      warmup_response = {}\n", request_call));
        let warmup_status = rate_limit.warmup_expect_status.unwrap_or(200);
        example.push_str(&format!(
            "      expect(warmup_response.status_code).to eq({})\n",
            warmup_status
        ));
        if let Some(delay) = rate_limit.sleep_ms_between {
            example.push_str(&format!("      sleep {}\n", format_sleep_seconds(delay)));
        }
        example.push_str("    end\n\n");
    }

    example.push_str(&format!(
        "    response = {}
",
        request_call
    ));

    if let Some(di_config) = DependencyConfig::from_fixture(fixture).expect("invalid DI config") {
        if requires_multi_request_test(&di_config) {
            example.push_str("\n");
            example.push_str("    # Second request to verify singleton caching\n");
            example.push_str(&format!("    response2 = {}\n", request_call));
            example.push_str("    expect(response.status_code).to eq(200)\n");
            example.push_str("    expect(response2.status_code).to eq(200)\n");
            example.push_str("    data1 = response.json\n");
            example.push_str("    data2 = response2.json\n");
            example.push_str("\n");
            example.push_str("    # Singleton should have same ID but incremented count\n");
            example.push_str("    expect(data1).to have_key('id')\n");
            example.push_str("    expect(data2).to have_key('id')\n");
            example.push_str("    expect(data1['id']).to eq(data2['id'])  # Same singleton instance\n");
            example.push_str("    if data1.key?('count') && data2.key?('count')\n");
            example.push_str("      expect(data2['count']).to be > data1['count']  # Count incremented\n");
            example.push_str("    end\n");
            example.push_str("    client.close\n");
            example.push_str(
                "  end

",
            );
            return example;
        }

        if has_cleanup(&di_config) {
            example.push_str("    expect(response.status_code).to eq(200)\n");
            example.push_str("\n");
            example.push_str("    # Allow async cleanup to complete\n");
            example.push_str("    sleep 0.1\n");
            example.push_str("\n");
            example.push_str("    # Verify cleanup was called\n");
            example.push_str("    cleanup_response = client.get('/api/cleanup-state')\n");
            example.push_str("    expect(cleanup_response.status_code).to eq(200)\n");
            example.push_str("    cleanup_state = cleanup_response.json\n");
            example.push_str("    expect(cleanup_state).to have_key('cleanup_events')\n");
            example.push_str("    events = cleanup_state['cleanup_events']\n");
            example.push_str("    expect(events).to include('session_opened')\n");
            example.push_str("    expect(events).to include('session_closed')\n");
            example.push_str("    client.close\n");
            example.push_str(
                "  end

",
            );
            return example;
        }
    }

    example.push_str(&build_expectations(
        &fixture.expected_response,
        streaming_info.as_ref(),
        &sanitized_headers,
    ));
    if let Some(bg) = background_info {
        let expected_json =
            serde_json::json!({ bg.state_key.clone(): serde_json::Value::Array(bg.expected_state.clone()) });
        example.push_str(&format!("    expected_state = {}\n", value_to_ruby(&expected_json)));
        example.push_str("    attempts = 0\n");
        example.push_str("    actual_state = nil\n");
        example.push_str("    begin\n");
        example.push_str(&format!(
            "      state_response = client.get({})\n",
            string_literal(&bg.state_path)
        ));
        example.push_str("      expect(state_response.status_code).to eq(200)\n");
        example.push_str("      actual_state = state_response.json\n");
        example.push_str("      break if actual_state == expected_state\n");
        example.push_str("      attempts += 1\n");
        example.push_str("      sleep 0.02\n");
        example.push_str("    end while attempts < 5\n");
        example.push_str("    expect(actual_state).to eq(expected_state)\n");
    }
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
            if let Ok(json) = serde_json::to_string(value) {
                vec![format!("{}={}", encode(key), encode(&json))]
            } else {
                vec![format!("{}={}", encode(key), encode(&value.to_string()))]
            }
        }
    }
}

fn fixture_should_skip(category: &str, fixture: &Fixture) -> bool {
    category == "content_types" && fixture.name == "20_content_length_mismatch"
}

fn build_expectations(
    expected: &FixtureExpectedResponse,
    streaming_info: Option<&StreamingFixtureData>,
    sanitized_headers: &HashMap<String, String>,
) -> String {
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
        let mut headers = sanitized_headers.clone();
        if let Some(content_type) = info.streaming.content_type.as_ref() {
            headers.insert("content-type".to_string(), content_type.clone());
        } else if !headers.contains_key("content-type") {
            headers.insert("content-type".to_string(), "application/octet-stream".to_string());
        }
        append_header_expectations(&mut expectations, &headers);
        return expectations;
    }

    if expected.validation_errors.is_some() {
        expectations.push_str("    body = response.json\n");
        expectations.push_str("    expect(body).to be_a(Hash)\n");
        expectations.push_str("    expect(body.keys).to include('errors').or include('detail')\n");
        append_header_expectations(&mut expectations, sanitized_headers);
        return expectations;
    }

    if let Some(body) = expected.body.as_ref() {
        if let Some(text) = body.as_str() {
            expectations.push_str(&format!(
                "    expect(response.body_text).to eq({})\n",
                string_literal(text)
            ));
            append_header_expectations(&mut expectations, sanitized_headers);
            return expectations;
        }

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

                if let Some(error_type) = first.get("type").and_then(|v| v.as_str())
                    && error_type != "validation_error"
                {
                    expectations.push_str(&format!(
                        "    expect(body['errors'].first['type']).to eq({})\n",
                        string_literal(error_type)
                    ));
                }
            }

            append_header_expectations(&mut expectations, sanitized_headers);
            return expectations;
        }

        expectations.push_str("    expect(response.json).to eq(");
        expectations.push_str(&value_to_ruby(body));
        expectations.push_str(")\n");
        append_header_expectations(&mut expectations, sanitized_headers);
        return expectations;
    }

    if let Some(body_partial) = expected.body_partial.as_ref() {
        expectations.push_str("    expect(response.body_text).not_to be_nil\n");

        if let Some(obj) = body_partial.as_object() {
            for key in obj.keys() {
                expectations.push_str(&format!(
                    "    expect(response.body_text).to include({})\n",
                    string_literal(key)
                ));
            }
        }

        append_header_expectations(&mut expectations, sanitized_headers);
        return expectations;
    }

    if expected.status_code >= 200
        && expected.status_code < 300
        && let Some(content_type) = sanitized_headers.get("content-type")
        && (content_type.contains("text/html") || content_type.contains("application"))
    {
        append_header_expectations(&mut expectations, sanitized_headers);
        return expectations;
    }

    if expected.status_code >= 400 {
        append_header_expectations(&mut expectations, sanitized_headers);
        return expectations;
    }

    expectations.push_str("    expect(response.body_text).to be_nil\n");
    append_header_expectations(&mut expectations, sanitized_headers);
    expectations
}

fn append_header_expectations(expectations: &mut String, headers: &HashMap<String, String>) {
    if headers.is_empty() {
        return;
    }
    expectations.push_str("    response_headers = response.headers.transform_keys { |key| key.downcase }\n");
    let mut sorted: BTreeMap<&String, &String> = BTreeMap::new();
    for (key, value) in headers {
        sorted.insert(key, value);
    }
    for (key, value) in sorted {
        let normalized_key = key.to_ascii_lowercase();
        expectations.push_str(&format!(
            "    expect(response_headers[{}]).to eq({})\n",
            string_literal(&normalized_key),
            string_literal(value)
        ));
    }
}

fn build_sse_expected_literal(fixtures: Vec<&AsyncFixture>) -> Result<String> {
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

fn build_websocket_spec(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut spec = String::new();
    spec.push_str(
        "# frozen_string_literal: true\n\nrequire 'spec_helper'\nrequire 'json'\n\nRSpec.describe \"asyncapi_websocket\" do\n",
    );

    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let factory_name = format!(
            "create_app_websocket_{}",
            sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"))
        );

        for (example_idx, fixture) in channel_fixtures.iter().enumerate() {
            for (msg_idx, example) in fixture.examples.iter().enumerate() {
                let test_name = if channel_fixtures.len() == 1 && fixture.examples.len() == 1 {
                    format!("echoes WebSocket messages on {}", channel_path)
                } else {
                    format!(
                        "echoes WebSocket message {} on {}",
                        example_idx * fixture.examples.len() + msg_idx + 1,
                        channel_path
                    )
                };

                let example_json = serde_json::to_string(example)?;

                spec.push_str(&format!(
                    r#"  it "{}" do
    app = E2ERubyApp.{}
    client = Spikard::Testing.create_test_client(app)
    ws = client.websocket("{}")

    message = JSON.parse({})
    ws.send_json(message)
    response = ws.receive_json

    expect(response['validated']).to eq(true)
    message.each do |key, value|
      expect(response[key]).to eq(value)
    end

    ws.close
    client.close
  end

"#,
                    test_name,
                    factory_name,
                    channel_path,
                    string_literal(&example_json)
                ));
            }
        }
    }

    spec.push_str("end\n");
    Ok(spec)
}

/// Build Ruby hash representation of file uploads
/// Format: {"field_name" => ["filename", "content", "content_type"]}
/// For multiple files with same field_name: {"field_name" => [["file1", "content1", "type1"], ["file2", "content2", "type2"]]}
fn build_files_hash(files: &[spikard_codegen::openapi::from_fixtures::FixtureFile]) -> String {
    use std::collections::HashMap;

    let mut grouped: HashMap<String, Vec<&spikard_codegen::openapi::from_fixtures::FixtureFile>> = HashMap::new();
    for file in files {
        grouped.entry(file.field_name.clone()).or_default().push(file);
    }

    let mut entries = Vec::new();

    let mut sorted_fields: Vec<_> = grouped.keys().collect();
    sorted_fields.sort();

    for field_name in sorted_fields {
        let field_files = &grouped[field_name];

        if field_files.len() == 1 {
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

fn sanitized_expected_headers(expected: &FixtureExpectedResponse) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(map) = expected.headers.as_ref() {
        for (key, value) in map {
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

fn normalize_expected_header_value(raw: &str) -> Option<String> {
    match raw {
        "<<absent>>" => None,
        "<<present>>" => Some("spikard-test-value".to_string()),
        "<<uuid>>" => Some("00000000-0000-4000-8000-000000000000".to_string()),
        _ => Some(raw.to_string()),
    }
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
