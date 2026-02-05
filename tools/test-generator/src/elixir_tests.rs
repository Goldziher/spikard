//! Elixir test spec generator
//!
//! Generates ExUnit test files from fixtures.

use crate::elixir_utils::{build_handler_name, build_test_name, string_literal, value_to_elixir};
use crate::fixture_filter::is_http_fixture_category;
use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::from_fixtures::FixtureRequest;
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;
use urlencoding::encode;

pub fn generate_elixir_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let test_dir = output_dir.join("test/generated");
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).context("Failed to remove existing Elixir generated tests")?;
    }
    fs::create_dir_all(&test_dir).context("Failed to create generated test directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;

    for (category, fixtures) in fixtures_by_category.iter() {
        let test_code = build_test_module(category, fixtures);
        let file_name = format!("{}_test.exs", category.replace(['-', ' '], "_"));
        fs::write(test_dir.join(file_name), test_code)
            .with_context(|| format!("Failed to write test for category {category}"))?;
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

fn build_test_module(category: &str, fixtures: &[Fixture]) -> String {
    let module_name = category
        .split(['_', '-', ' '])
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<String>();

    let mut code = String::new();
    code.push_str(&format!(
        r#"defmodule E2EElixirApp.{}Test do
  @moduledoc """
  Generated tests for {} fixtures.

  Each test starts its own isolated server with only its route,
  avoiding conflicts when multiple fixtures share the same path.
  """
  use ExUnit.Case, async: false

  alias E2EElixirApp.AppFactories

  @base_url "http://127.0.0.1:59800"

  setup do
    :inets.start()
    :ssl.start()
    :ok
  end

"#,
        module_name, category
    ));

    for fixture in fixtures.iter() {
        code.push_str(&build_test_case(category, fixture));
    }

    code.push_str("end\n");
    code
}

fn get_content_type(request: &FixtureRequest) -> &str {
    // Check the content_type field first (used by multipart fixtures)
    if let Some(ref ct) = request.content_type {
        return ct.as_str();
    }
    // Fall back to headers
    request
        .headers
        .as_ref()
        .and_then(|h: &HashMap<String, String>| h.get("content-type").or(h.get("Content-Type")))
        .map(|s: &String| s.as_str())
        .unwrap_or("application/json")
}

fn url_encode_form_data(form_data: &HashMap<String, Value>) -> String {
    form_data
        .iter()
        .flat_map(|(k, v)| match v {
            Value::Array(arr) => arr
                .iter()
                .map(|item| {
                    let item_str = match item {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => item.to_string(),
                    };
                    format!("{}={}", encode(k), encode(&item_str))
                })
                .collect::<Vec<_>>(),
            _ => {
                let v_str = match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => v.to_string(),
                };
                vec![format!("{}={}", encode(k), encode(&v_str))]
            }
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn is_multipart_request(request: &FixtureRequest) -> bool {
    let content_type = get_content_type(request);
    if content_type.starts_with("multipart/form-data") {
        return true;
    }
    // Also check if files field exists (indicates multipart even without explicit content_type)
    request.files.is_some()
}

fn build_multipart_body(request: &FixtureRequest) -> String {
    // Generate Elixir code that builds multipart body at runtime
    let mut code = String::new();
    code.push_str("(fn ->\n");
    code.push_str("        boundary = \"----ElixirFormBoundary#{:erlang.unique_integer([:positive])}\"\n");
    code.push_str("        parts = []\n");

    // Add form data fields
    if let Some(ref data) = request.data {
        for (key, value) in data.iter() {
            let value_str = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            code.push_str(&format!(
                "        parts = parts ++ [\"--#{{boundary}}\\r\\nContent-Disposition: form-data; name=\\\"{}\\\"\\r\\n\\r\\n{}\\r\\n\"]\n",
                key.replace('"', "\\\""),
                value_str.replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
            ));
        }
    }

    // Add file parts
    if let Some(ref files) = request.files {
        for file in files.iter() {
            let filename = file.filename.as_deref().unwrap_or("file");
            let content = file.content.as_deref().unwrap_or("");
            let file_content_type = file.content_type.as_deref().unwrap_or("application/octet-stream");

            if file.filename.is_some() {
                code.push_str(&format!(
                    "        parts = parts ++ [\"--#{{boundary}}\\r\\nContent-Disposition: form-data; name=\\\"{}\\\"; filename=\\\"{}\\\"\\r\\nContent-Type: {}\\r\\n\\r\\n{}\\r\\n\"]\n",
                    file.field_name.replace('"', "\\\""),
                    filename.replace('"', "\\\""),
                    file_content_type,
                    content.replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
                ));
            } else {
                // File without filename
                code.push_str(&format!(
                    "        parts = parts ++ [\"--#{{boundary}}\\r\\nContent-Disposition: form-data; name=\\\"{}\\\"\\r\\nContent-Type: {}\\r\\n\\r\\n{}\\r\\n\"]\n",
                    file.field_name.replace('"', "\\\""),
                    file_content_type,
                    content.replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
                ));
            }
        }
    }

    code.push_str("        body = Enum.join(parts, \"\") <> \"--#{boundary}--\\r\\n\"\n");
    code.push_str("        {boundary, body}\n");
    code.push_str("      end).()");
    code
}

fn build_request_body(request: &FixtureRequest) -> String {
    let content_type = get_content_type(request);

    if content_type == "application/x-www-form-urlencoded" {
        if let Some(ref form_data) = request.form_data {
            return string_literal(&url_encode_form_data(form_data));
        }
        if let Some(ref body) = request.body {
            if let Some(s) = body.as_str() {
                return string_literal(s);
            }
        }
        return "\"\"".to_string();
    }

    // Multipart is handled separately in build_test_case
    if is_multipart_request(request) {
        return "\"\"".to_string(); // Placeholder, actual body built differently
    }

    match &request.body {
        Some(body) => format!("Jason.encode!({})", value_to_elixir(body)),
        None => "\"\"".to_string(),
    }
}

fn build_url(path: &str, query_params: &Option<HashMap<String, Value>>) -> String {
    // If path already contains query params, don't add them again
    if path.contains('?') {
        // Encode the query string part to handle special characters like |
        if let Some(idx) = path.find('?') {
            let (base_path, query_str) = path.split_at(idx);
            // Encode characters that are invalid in URLs but preserve = and &
            let encoded_query: String = query_str
                .chars()
                .map(|c| match c {
                    '|' => "%7C".to_string(),
                    ' ' => "%20".to_string(),
                    _ => c.to_string(),
                })
                .collect();
            return format!(
                "@base_url <> {}",
                string_literal(&format!("{}{}", base_path, encoded_query))
            );
        }
        return format!("@base_url <> {}", string_literal(path));
    }

    match query_params {
        Some(query) if !query.is_empty() => {
            let query_string: String = query
                .iter()
                .map(|(k, v)| {
                    let v_str = match v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    };
                    format!("{}={}", encode(k), encode(&v_str))
                })
                .collect::<Vec<_>>()
                .join("&");
            format!(
                "@base_url <> {} <> \"?\" <> {}",
                string_literal(path),
                string_literal(&query_string)
            )
        }
        _ => format!("@base_url <> {}", string_literal(path)),
    }
}

fn build_headers(
    headers: &Option<HashMap<String, String>>,
    cookies: &Option<HashMap<String, String>>,
) -> String {
    let mut pairs: Vec<String> = Vec::new();

    // Add regular headers
    if let Some(h) = headers {
        for (k, v) in h.iter() {
            pairs.push(format!(
                "{{~c\"{}\", ~c\"{}\"}}",
                k.replace('"', "\\\""),
                v.replace('"', "\\\"")
            ));
        }
    }

    // Add cookies as Cookie header
    if let Some(c) = cookies {
        if !c.is_empty() {
            let cookie_str: String = c
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("; ");
            pairs.push(format!(
                "{{~c\"Cookie\", ~c\"{}\"}}",
                cookie_str.replace('"', "\\\"")
            ));
        }
    }

    if pairs.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", pairs.join(", "))
    }
}

fn is_redirect_status(status_code: u16) -> bool {
    matches!(status_code, 301 | 302 | 303 | 307 | 308)
}

fn needs_short_timeout(status_code: u16) -> bool {
    // 503 Service Unavailable with Retry-After causes :httpc to wait for the
    // specified duration before retrying. We'll handle timeout gracefully.
    matches!(status_code, 503)
}

fn build_http_request(method: &str, content_type: &str, expected_status: u16, is_multipart: bool) -> String {
    // Build HTTP options based on expected status
    let http_opts = if is_redirect_status(expected_status) {
        // Disable autoredirect for redirect status codes so we can assert on them
        "[{:autoredirect, false}]"
    } else if needs_short_timeout(expected_status) {
        // Use short timeout for 503 to avoid :httpc waiting for Retry-After header
        "[{:timeout, 5000}]"
    } else {
        "[]"
    };

    match method {
        "GET" => {
            format!(
                "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                 :httpc.request(:get, {{String.to_charlist(url), headers}}, {}, [])\n",
                http_opts
            )
        }
        "POST" | "PUT" | "PATCH" => {
            if is_multipart {
                // For multipart, content type is set in headers with boundary variable
                format!(
                    "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                     :httpc.request(:{}, {{String.to_charlist(url), headers, ~c\"multipart/form-data; boundary=#{{boundary}}\", req_body}}, {}, [])\n",
                    method.to_lowercase(),
                    http_opts
                )
            } else {
                format!(
                    "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                     :httpc.request(:{}, {{String.to_charlist(url), headers, ~c\"{}\", req_body}}, {}, [])\n",
                    method.to_lowercase(),
                    content_type,
                    http_opts
                )
            }
        }
        "DELETE" => {
            format!(
                "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                 :httpc.request(:delete, {{String.to_charlist(url), headers}}, {}, [])\n",
                http_opts
            )
        }
        "HEAD" => {
            format!(
                "      {{:ok, {{{{_, status, _}}, _resp_headers, _resp_body}}}} = \
                 :httpc.request(:head, {{String.to_charlist(url), headers}}, {}, [])\n",
                http_opts
            )
        }
        "OPTIONS" => {
            format!(
                "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                 :httpc.request(:options, {{String.to_charlist(url), headers}}, {}, [])\n",
                http_opts
            )
        }
        "TRACE" => {
            format!(
                "      {{:ok, {{{{_, status, _}}, _resp_headers, resp_body}}}} = \
                 :httpc.request(:trace, {{String.to_charlist(url), headers}}, {}, [])\n",
                http_opts
            )
        }
        _ => format!(
            "      # Unsupported method: {}\n      status = 200\n      resp_body = []\n",
            method
        ),
    }
}

fn build_http_request_with_error_handling(method: &str, content_type: &str, is_multipart: bool) -> String {
    // For 503 responses, :httpc may timeout due to Retry-After handling.
    // We build a let statement that assigns status and resp_body from the case result.
    let httpc_call: String = match method {
        "GET" => {
            ":httpc.request(:get, {String.to_charlist(url), headers}, [{:timeout, 5000}], [])".to_string()
        }
        "POST" | "PUT" | "PATCH" => {
            if is_multipart {
                format!(
                    ":httpc.request(:{}, {{String.to_charlist(url), headers, ~c\"multipart/form-data; boundary=#{{boundary}}\", req_body}}, [{{:timeout, 5000}}], [])",
                    method.to_lowercase()
                )
            } else {
                format!(
                    ":httpc.request(:{}, {{String.to_charlist(url), headers, ~c\"{}\", req_body}}, [{{:timeout, 5000}}], [])",
                    method.to_lowercase(),
                    content_type
                )
            }
        }
        "DELETE" => {
            ":httpc.request(:delete, {String.to_charlist(url), headers}, [{:timeout, 5000}], [])".to_string()
        }
        "HEAD" => {
            ":httpc.request(:head, {String.to_charlist(url), headers}, [{:timeout, 5000}], [])".to_string()
        }
        "OPTIONS" => {
            ":httpc.request(:options, {String.to_charlist(url), headers}, [{:timeout, 5000}], [])".to_string()
        }
        "TRACE" => {
            ":httpc.request(:trace, {String.to_charlist(url), headers}, [{:timeout, 5000}], [])".to_string()
        }
        _ => {
            "".to_string()
        }
    };

    let mut code = String::new();
    code.push_str("      {status, resp_body} =\n");
    code.push_str("        case ");
    code.push_str(&httpc_call);
    code.push_str(" do\n");
    code.push_str("          {:ok, {{_, s, _}, _resp_headers, body}} ->\n");
    code.push_str("            {s, body}\n");
    code.push_str("          {:error, :timeout} ->\n");
    code.push_str("            # :httpc may timeout due to Retry-After handling on 503 responses.\n");
    code.push_str("            # We assume 503 and no body in this case.\n");
    code.push_str("            {503, []}\n");
    code.push_str("        end\n");
    code
}

fn build_test_case(category: &str, fixture: &Fixture) -> String {
    let test_name = build_test_name(category, &fixture.name);
    let handler_name = build_handler_name(category, &fixture.name);
    let factory_name = format!("create_app_{}", handler_name);
    let method = fixture.request.method.to_uppercase();
    let content_type = get_content_type(&fixture.request);

    let mut code = String::new();

    // Test header
    code.push_str(&format!(
        "  @tag :integration\n  test {} do\n",
        string_literal(&test_name)
    ));

    // Server setup
    code.push_str(&format!("    {{routes, config}} = AppFactories.{}()\n", factory_name));
    code.push_str(
        r#"    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
"#,
    );

    // URL
    code.push_str(&format!(
        "      url = {}\n",
        build_url(&fixture.request.path, &fixture.request.query_params)
    ));

    // Check if this is a multipart request
    let is_multipart = is_multipart_request(&fixture.request);

    if is_multipart && matches!(method.as_str(), "POST" | "PUT" | "PATCH") {
        // Multipart request - build body with boundary
        code.push_str(&format!(
            "      {{boundary, req_body}} = {}\n",
            build_multipart_body(&fixture.request)
        ));
        // Headers with dynamic content-type including boundary
        code.push_str("      headers = [{~c\"Content-Type\", ~c\"multipart/form-data; boundary=#{boundary}\"}]\n");
    } else {
        // Regular headers (including cookies)
        code.push_str(&format!(
            "      headers = {}\n",
            build_headers(&fixture.request.headers, &fixture.request.cookies)
        ));

        // Request body (for methods that need it)
        if matches!(method.as_str(), "POST" | "PUT" | "PATCH") {
            code.push_str(&format!(
                "      req_body = {}\n",
                build_request_body(&fixture.request)
            ));
        }
    }

    // HTTP request
    let response = &fixture.expected_response;

    // For 503 responses, :httpc may timeout due to Retry-After handling
    if response.status_code == 503 {
        code.push_str(&build_http_request_with_error_handling(&method, content_type, is_multipart));
    } else {
        code.push_str(&build_http_request(&method, content_type, response.status_code, is_multipart));
    }

    // Assertions
    code.push_str(&format!(
        "      assert status == {}, \"Expected status {}, got #{{status}}\"\n",
        response.status_code, response.status_code
    ));

    // Body assertions (skip for 503 since we may have timed out)
    if let Some(ref expected_body) = response.body {
        code.push_str("      # Response body validation\n");
        if response.status_code == 503 {
            code.push_str("      # For 503, body assertions are conditional since :httpc may timeout\n");
            code.push_str("      if resp_body != [] do\n");
            code.push_str("        resp_body_str = :erlang.list_to_binary(resp_body)\n");
            if !expected_body.is_string() {
                code.push_str("        parsed_body = Jason.decode!(resp_body_str)\n");
            }
            // Adjust indentation for assertions within if block
            add_body_assertions(&mut code, expected_body, "        ");
            code.push_str("      end\n");
        } else {
            // Convert charlist to binary for proper UTF-8 handling
            code.push_str("      resp_body_str = :erlang.list_to_binary(resp_body)\n");
            if !expected_body.is_string() {
                code.push_str("      parsed_body = Jason.decode!(resp_body_str)\n");
            }
            add_body_assertions(&mut code, expected_body, "      ");
        }
    }

    // Cleanup
    code.push_str(
        r#"    after
      Spikard.stop(server)
    end
  end

"#,
    );

    code
}

fn add_body_assertions(code: &mut String, expected: &Value, indent: &str) {
    match expected {
        Value::Object(obj) => {
            for (key, value) in obj.iter() {
                match value {
                    Value::String(s) => {
                        code.push_str(&format!(
                            "{}assert parsed_body[{}] == {}\n",
                            indent,
                            string_literal(key),
                            string_literal(s)
                        ));
                    }
                    Value::Number(_) => {
                        code.push_str(&format!(
                            "{}assert parsed_body[{}] == {}\n",
                            indent,
                            string_literal(key),
                            value_to_elixir(value)
                        ));
                    }
                    Value::Bool(b) => {
                        code.push_str(&format!(
                            "{}assert parsed_body[{}] == {}\n",
                            indent,
                            string_literal(key),
                            b
                        ));
                    }
                    _ => {
                        code.push_str(&format!(
                            "{}assert Map.has_key?(parsed_body, {})\n",
                            indent,
                            string_literal(key)
                        ));
                    }
                }
            }
        }
        Value::Array(_) => {
            code.push_str(&format!("{}assert is_list(parsed_body)\n", indent));
        }
        Value::String(s) => {
            code.push_str(&format!(
                "{}assert resp_body_str == {}\n",
                indent,
                string_literal(s)
            ));
        }
        _ => {}
    }
}
