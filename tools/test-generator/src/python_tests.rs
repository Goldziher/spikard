//! Python test generator
//!
//! Generates pytest test suites from fixtures for e2e testing.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
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
    r#""""Pytest configuration and fixtures for e2e tests."""

import pytest
from spikard.testing import TestClient

from app.main import app


@pytest.fixture(scope="session")
def client():
    """Provide a test client."""
    return TestClient(app)
"#
    .to_string()
}

/// Generate test file for a category
fn generate_test_file(category: &str, fixtures: &[Fixture]) -> Result<String> {
    let mut code = String::new();

    // File header
    code.push_str(&format!("\"\"\"E2E tests for {}.\"\"\"\n\n", category));
    code.push_str("import pytest\n");
    code.push_str("from typing import Any\n\n");

    // Generate test for each fixture
    for fixture in fixtures {
        let test_function = generate_test_function(fixture)?;
        code.push_str(&test_function);
        code.push_str("\n\n");
    }

    Ok(code)
}

/// Generate a single test function
fn generate_test_function(fixture: &Fixture) -> Result<String> {
    let test_name = sanitize_test_name(&fixture.name);
    let mut code = String::new();

    code.push_str(&format!("async def test_{}(client: Any) -> None:\n", test_name));
    code.push_str(&format!("    \"\"\"{}.\"\"\"\n", fixture.description));

    // Build request
    let method = fixture.request.method.to_lowercase();
    let path = &fixture.request.path;

    // Prepare request kwargs
    let mut request_kwargs = Vec::new();

    // Add query params
    if let Some(ref query_params) = fixture.request.query_params {
        if !query_params.is_empty() {
            code.push_str("    params = {\n");
            for (key, value) in query_params {
                code.push_str(&format!("        \"{}\": {},\n", key, json_to_python(value)));
            }
            code.push_str("    }\n");
            request_kwargs.push("query_params=params");
        }
    }

    // Add headers
    if let Some(ref headers) = fixture.request.headers {
        if !headers.is_empty() {
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
    }

    // Add cookies
    if let Some(ref cookies) = fixture.request.cookies {
        if !cookies.is_empty() {
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
    }

    // Add body
    if let Some(ref body) = fixture.request.body {
        code.push_str(&format!("    json_data = {}\n", json_to_python(body)));
        request_kwargs.push("json=json_data");
    }

    // Add form data (for URL-encoded forms)
    if let Some(ref form_data) = fixture.request.form_data {
        code.push_str(&format!("    json_data = {}\n", hashmap_to_python(form_data)));
        request_kwargs.push("json=json_data");
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

    // Assert response body
    if let Some(ref body) = fixture.expected_response.body {
        code.push_str("    response_data = response.json()\n");

        // Generate assertions for body
        generate_body_assertions(&mut code, body, "response_data");
    }

    // Assert validation errors if present
    if let Some(ref errors) = fixture.expected_response.validation_errors {
        code.push_str("    response_data = response.json()\n");
        code.push_str("    assert \"detail\" in response_data\n");
        code.push_str(&format!(
            "    assert len(response_data[\"detail\"]) == {}\n",
            errors.len()
        ));

        for (idx, error) in errors.iter().enumerate() {
            code.push_str(&format!("    error_{} = response_data[\"detail\"][{}]\n", idx, idx));
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

/// Generate assertions for response body
fn generate_body_assertions(code: &mut String, body: &serde_json::Value, path: &str) {
    match body {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("    assert \"{}\" in {}\n", key, path));

                match value {
                    serde_json::Value::Object(_) => {
                        generate_body_assertions(code, value, &new_path);
                    }
                    serde_json::Value::Array(_) => {
                        generate_body_assertions(code, value, &new_path);
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
    let items: Vec<String> = map
        .iter()
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
    name.to_lowercase()
        .replace(
            [
                ' ', '-', '/', '.', '(', ')', '=', ',', ':', '+', '<', '>', '[', ']', '\'', '"',
            ],
            "_",
        )
        .replace("__", "_")
        .trim_matches('_')
        .to_string()
}
