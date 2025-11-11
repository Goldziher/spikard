//! Python test generator
//!
//! Generates pytest test suites from fixtures for e2e testing.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate Python test suite from fixtures
pub fn generate_python_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Python tests...");

    // Create tests directory
    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    // Generate conftest.py
    let conftest_content = generate_conftest();
    fs::write(tests_dir.join("conftest.py"), conftest_content).context("Failed to write conftest.py")?;

    // Generate __init__.py
    fs::write(tests_dir.join("__init__.py"), "\"\"\"E2E tests.\"\"\"\n").context("Failed to write __init__.py")?;

    // Load fixtures by category
    let mut fixtures_by_category: HashMap<String, Vec<Fixture>> = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path.file_name().unwrap().to_str().unwrap().to_string();
            let fixtures = load_fixtures_from_dir(&path)?;

            if !fixtures.is_empty() {
                fixtures_by_category.insert(category, fixtures);
            }
        }
    }

    // Generate test file for each category
    for (category, fixtures) in fixtures_by_category.iter() {
        let test_content = generate_test_file(category, fixtures)?;
        let test_file = tests_dir.join(format!("test_{}.py", category));
        fs::write(&test_file, test_content).with_context(|| format!("Failed to write test file for {}", category))?;
        println!("  ✓ Generated tests/test_{}.py ({} tests)", category, fixtures.len());
    }

    Ok(())
}

/// Generate conftest.py with shared fixtures
fn generate_conftest() -> String {
    r#""""Pytest configuration for e2e tests.

Each test creates its own isolated app and client from per-fixture app factories.
This ensures complete test isolation and allows multiple tests for the same route.
"""
"#
    .to_string()
}

/// Generate test file for a category
fn generate_test_file(category: &str, fixtures: &[Fixture]) -> Result<String> {
    let mut code = String::new();

    // Collect app factory imports so we can emit them once at the top
    let mut app_factories: Vec<String> = fixtures
        .iter()
        .map(|fixture| {
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            format!("create_app_{}", fixture_id)
        })
        .collect();
    app_factories.sort();
    app_factories.dedup();

    // File header and imports
    code.push_str(&format!("\"\"\"E2E tests for {}.\"\"\"\n\n", category));
    code.push_str("from spikard.testing import TestClient\n");
    if !app_factories.is_empty() {
        code.push_str("from app.main import (\n");
        for factory in &app_factories {
            code.push_str(&format!("    {},\n", factory));
        }
        code.push_str(")\n");
    }
    code.push('\n');

    // Generate test for each fixture
    for fixture in fixtures {
        let test_function = generate_test_function(category, fixture)?;
        code.push_str(&test_function);
        code.push_str("\n\n");
    }

    Ok(code)
}

/// Generate a single test function
fn generate_test_function(category: &str, fixture: &Fixture) -> Result<String> {
    let test_name = sanitize_test_name(&fixture.name);
    let mut code = String::new();

    // No client parameter - create per-test client from app factory
    code.push_str(&format!("async def test_{}() -> None:\n", test_name));
    code.push_str(&format!("    \"\"\"{}.\"\"\"\n", fixture.description));
    code.push('\n');

    // Import and create client from per-fixture app factory
    // The app factory name matches the one generated in python_app.rs:
    // sanitize_identifier(&format!("{}_{}", category, &fixture.name))
    let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
    let app_factory_name = format!("create_app_{}", fixture_id);
    code.push_str(&format!("    app = {}()\n", app_factory_name));
    code.push_str("    client = TestClient(app)\n\n");

    // Build request
    let method = fixture.request.method.to_lowercase();
    let path = &fixture.request.path;

    // Prepare request kwargs
    let mut request_kwargs = Vec::new();

    // Add query params
    if let Some(ref query_params) = fixture.request.query_params
        && !query_params.is_empty()
    {
        code.push_str("    params = {\n");
        for (key, value) in query_params {
            code.push_str(&format!("        \"{}\": {},\n", key, json_to_python(value)));
        }
        code.push_str("    }\n");
        request_kwargs.push("query_params=params");
    }

    // Add headers
    if let Some(ref headers) = fixture.request.headers
        && !headers.is_empty()
    {
        code.push_str("    headers = {\n");
        for (key, value) in headers {
            // Escape special characters in header values
            let escaped_value = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            code.push_str(&format!("        \"{}\": \"{}\",\n", key, escaped_value));
        }
        code.push_str("    }\n");
        request_kwargs.push("headers=headers");
    }

    // Add cookies
    if let Some(ref cookies) = fixture.request.cookies
        && !cookies.is_empty()
    {
        code.push_str("    cookies = {\n");
        for (key, value) in cookies {
            // Escape special characters in cookie values
            let escaped_value = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            code.push_str(&format!("        \"{}\": \"{}\",\n", key, escaped_value));
        }
        code.push_str("    }\n");
        request_kwargs.push("cookies=cookies");
    }

    // Add body
    if let Some(ref body) = fixture.request.body {
        let content_type = fixture
            .request
            .headers
            .as_ref()
            .and_then(|headers| headers.get("Content-Type"))
            .map(|value| value.to_ascii_lowercase());

        let treat_as_json = content_type
            .as_deref()
            .map(|ct| {
                ct.contains("application/json")
                    || ct.contains("application/x-www-form-urlencoded")
                    || ct.contains("application/xml")
            })
            .unwrap_or(true);

        if treat_as_json {
            code.push_str(&format!("    json_data = {}\n", json_to_python(body)));
            request_kwargs.push("json=json_data");
        } else {
            code.push_str(&format!("    raw_body = {}\n", json_to_python(body)));
            request_kwargs.push("data=raw_body");
        }
    }

    // Add form data (for URL-encoded forms)
    if let Some(ref form_data) = fixture.request.form_data {
        code.push_str(&format!("    json_data = {}\n", hashmap_to_python(form_data)));
        request_kwargs.push("json=json_data");
    }

    // Add form data (for multipart form data without files)
    if let Some(ref data) = fixture.request.data {
        code.push_str(&format!("    data = {}\n", hashmap_to_python(data)));
        request_kwargs.push("data=data");
    }

    // Add files (for multipart form data)
    if let Some(ref files) = fixture.request.files
        && !files.is_empty()
    {
        // Group files by field_name to handle multiple files with same name
        use std::collections::HashMap;
        let mut files_by_name: HashMap<&str, Vec<String>> = HashMap::new();

        for file in files {
            let field_name = file.field_name.as_str();
            let filename = file.filename.as_deref().unwrap_or("file.txt");

            // Handle content - either direct content or magic_bytes
            let file_content = if let Some(ref content) = file.content {
                // Text or binary content
                let escaped = content
                    .replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\r", "\\r")
                    .replace("\t", "\\t");
                format!("b\"{}\"", escaped)
            } else if let Some(ref magic_bytes) = file.magic_bytes {
                // Hex-encoded binary data
                format!("bytes.fromhex(\"{}\")", magic_bytes)
            } else {
                // Empty file
                "b\"\"".to_string()
            };

            // Include content_type if specified (TestClient supports 3-tuple format)
            let file_tuple = if let Some(ref content_type) = file.content_type {
                format!("(\"{}\", {}, \"{}\")", filename, file_content, content_type)
            } else {
                format!("(\"{}\", {})", filename, file_content)
            };

            files_by_name.entry(field_name).or_default().push(file_tuple);
        }

        // Generate files dict
        code.push_str("    files = {\n");
        for (field_name, file_tuples) in files_by_name.iter() {
            if file_tuples.len() == 1 {
                // Single file for this field name
                code.push_str(&format!("        \"{}\": {},\n", field_name, file_tuples[0]));
            } else {
                // Multiple files for this field name - use list
                code.push_str(&format!("        \"{}\": [{}],\n", field_name, file_tuples.join(", ")));
            }
        }
        code.push_str("    }\n");
        request_kwargs.push("files=files");
    }

    // Make request
    let kwargs_str = if request_kwargs.is_empty() {
        String::new()
    } else {
        format!(", {}", request_kwargs.join(", "))
    };

    code.push_str(&format!(
        "    response = await client.{}(\"{}\"{})\n\n",
        method, path, kwargs_str
    ));

    // Assert status code
    code.push_str(&format!(
        "    assert response.status_code == {}\n",
        fixture.expected_response.status_code
    ));

    let status_code = fixture.expected_response.status_code;
    let method = fixture.request.method.to_uppercase();

    // Different assertion strategies based on what we're testing:
    // - 200 success: Verify echoed parameters match sent values
    // - 422 validation errors: Verify error structure (handler should not be reached)
    // - Other: Verify expected response body (business logic)

    if status_code == 200 {
        // Success case - verify response matches expected
        // Check if response is text (HTML, plain text, CSV, etc.) - don't parse as JSON
        let is_text_response = fixture
            .expected_response
            .headers
            .as_ref()
            .and_then(|h| h.get("content-type"))
            .map(|ct| ct.starts_with("text/"))
            .unwrap_or(false);

        // Skip parsing JSON for HEAD requests without expected body (HEAD has no response body)
        // Also skip for text responses
        let should_parse_json = !is_text_response && (method != "HEAD" || fixture.expected_response.body.is_some());

        if should_parse_json {
            code.push_str("    response_data = response.json()\n");
        }

        // If fixture has expected response body, assert against that (handles type conversion)
        if let Some(ref expected_body) = fixture.expected_response.body {
            // For text responses (HTML, plain text, CSV, etc.), assert against response.text() directly
            if is_text_response && expected_body.is_string() {
                code.push_str(&format!(
                    "    assert response.text() == {}\n",
                    json_to_python(expected_body)
                ));
            } else {
                generate_body_assertions(&mut code, expected_body, "response_data");
            }
        } else if should_parse_json {
            // Fallback: verify echoed parameters match what we sent
            // (This path is for fixtures without expected_response.body)

            // Verify body parameters
            if let Some(ref body) = fixture.request.body {
                generate_echo_assertions(&mut code, body, "response_data");
            }

            // Verify form data parameters
            if let Some(ref form_data) = fixture.request.form_data {
                for (key, value) in form_data {
                    code.push_str(&format!(
                        "    assert response_data[\"{}\"] == {}\n",
                        key,
                        json_to_python(value)
                    ));
                }
            }

            // Verify query parameters
            if let Some(ref query_params) = fixture.request.query_params {
                for (key, value) in query_params {
                    code.push_str(&format!(
                        "    assert response_data[\"{}\"] == {}\n",
                        key,
                        json_to_python(value)
                    ));
                }
            }
        }
    } else if status_code == 422 {
        // Validation error - framework should reject before handler
        code.push_str("    response_data = response.json()\n");
        code.push_str("    # Validation should be done by framework, not handler\n");
        code.push_str("    assert \"errors\" in response_data or \"detail\" in response_data\n");
        // Don't assert specific error structure - that varies by validator
    } else {
        // Other status codes - assert expected response body
        if let Some(ref body) = fixture.expected_response.body {
            code.push_str("    response_data = response.json()\n");
            generate_body_assertions(&mut code, body, "response_data");
        }
    }

    // Legacy validation_errors field (deprecated in favor of status code checking)
    if let Some(ref errors) = fixture.expected_response.validation_errors {
        code.push_str("    response_data = response.json()\n");
        // RFC 9457 format uses "errors" array, not "detail"
        code.push_str("    assert \"errors\" in response_data\n");
        code.push_str(&format!(
            "    assert len(response_data[\"errors\"]) == {}\n",
            errors.len()
        ));

        for (idx, error) in errors.iter().enumerate() {
            code.push_str(&format!("    error_{} = response_data[\"errors\"][{}]\n", idx, idx));
            code.push_str(&format!(
                "    assert error_{}[\"type\"] == \"{}\"\n",
                idx, error.error_type
            ));
            code.push_str(&format!(
                "    assert error_{}[\"loc\"] == [{}]\n",
                idx,
                error
                    .loc
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            code.push_str(&format!("    assert error_{}[\"msg\"] == \"{}\"\n", idx, error.msg));
        }
    }

    Ok(code)
}

/// Generate assertions for echoed parameters (success cases)
/// Verifies that the response contains the same values that were sent
fn generate_echo_assertions(code: &mut String, sent_value: &serde_json::Value, path: &str) {
    match sent_value {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("    assert \"{}\" in {}\n", key, path));

                match value {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        generate_echo_assertions(code, value, &new_path);
                    }
                    _ => {
                        code.push_str(&format!("    assert {} == {}\n", new_path, json_to_python(value)));
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("    assert len({}) == {}\n", path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_echo_assertions(code, item, &new_path);
            }
        }
        _ => {
            code.push_str(&format!("    assert {} == {}\n", path, json_to_python(sent_value)));
        }
    }
}

/// Generate assertions for response body
fn generate_body_assertions(code: &mut String, body: &serde_json::Value, path: &str) {
    match body {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("    assert \"{}\" in {}\n", key, path));

                match value {
                    serde_json::Value::Object(_) => {
                        // Skip "ctx" objects in validation errors (contents vary by validator)
                        let skip_ctx = key == "ctx" && path.contains("[\"errors\"]");
                        if !skip_ctx {
                            generate_body_assertions(code, value, &new_path);
                        }
                    }
                    serde_json::Value::Array(_) => {
                        generate_body_assertions(code, value, &new_path);
                    }
                    _ => {
                        // Skip asserting on certain fields inside validation errors
                        // because they are implementation details that vary by validator
                        let in_errors = path.contains("[\"errors\"]");
                        let skip_assertion = in_errors
                            && (
                                // Skip input field entirely (content varies by validator)
                                key == "input"
                            // Skip error messages (wording varies by validator)
                            || key == "msg"
                            // Skip error type names (naming varies by validator)
                            || key == "type"
                            );

                        if !skip_assertion {
                            code.push_str(&format!("    assert {} == {}\n", new_path, json_to_python(value)));
                        }
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("    assert len({}) == {}\n", path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_body_assertions(code, item, &new_path);
            }
        }
        _ => {
            code.push_str(&format!("    assert {} == {}\n", path, json_to_python(body)));
        }
    }
}

/// Convert HashMap to Python dict literal
fn hashmap_to_python(map: &HashMap<String, serde_json::Value>) -> String {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|(ak, _), (bk, _)| ak.cmp(bk));
    let items: Vec<String> = entries
        .into_iter()
        .map(|(k, v)| format!("\"{}\": {}", k, json_to_python(v)))
        .collect();
    format!("{{{}}}", items.join(", "))
}

/// Convert JSON value to Python literal
fn json_to_python(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "None".to_string(),
        serde_json::Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            // Escape special characters
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            format!("\"{}\"", escaped)
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_python).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_python(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

/// Sanitize fixture name for test function
fn sanitize_test_name(name: &str) -> String {
    let mut result = name.to_lowercase().replace(
        [
            ' ', '-', '/', '.', '(', ')', '=', ',', ':', '+', '<', '>', '[', ']', '\'', '"',
        ],
        "_",
    );

    // Collapse multiple consecutive underscores to single underscore
    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

/// Sanitize a string to be a valid Python identifier (lowercase snake_case, matches python_app.rs)
fn sanitize_identifier(s: &str) -> String {
    let mut result = s
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

    // Collapse multiple consecutive underscores to single underscore
    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}
