//! Node.js test generator
//!
//! Generates vitest test suites from fixtures for e2e testing.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate Node.js test suite from fixtures
pub fn generate_node_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Node.js tests...");

    // Create tests directory
    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

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
        let test_file = tests_dir.join(format!("{}.test.ts", category));
        fs::write(&test_file, test_content).with_context(|| format!("Failed to write test file for {}", category))?;
        println!("  ✓ Generated tests/{}.test.ts ({} tests)", category, fixtures.len());
    }

    Ok(())
}

/// Generate test file for a category
fn generate_test_file(category: &str, fixtures: &[Fixture]) -> Result<String> {
    let mut code = String::new();

    // File header
    code.push_str(&format!("/**\n * E2E tests for {}\n * @generated\n */\n\n", category));
    code.push_str("import { describe, test, expect } from \"vitest\";\n");
    code.push_str("import { TestClient } from \"@spikard/node\";\n");

    // Import all app factories for this category
    let mut app_factories = Vec::new();
    for fixture in fixtures {
        let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
        let app_factory_name = format!("createApp{}", to_pascal_case(&fixture_id));
        app_factories.push(app_factory_name.clone());
    }

    code.push_str(&format!(
        "import {{ {} }} from \"../app/main.js\";\n\n",
        app_factories.join(", ")
    ));

    // Generate test suite
    code.push_str(&format!("describe(\"{}\", () => {{\n", category));

    // Generate test for each fixture
    for fixture in fixtures {
        let test_function = generate_test_function(category, fixture)?;
        code.push_str(&test_function);
        code.push('\n');
    }

    code.push_str("});\n");

    Ok(code)
}

/// Generate a single test function
fn generate_test_function(category: &str, fixture: &Fixture) -> Result<String> {
    let test_name = sanitize_test_name(&fixture.name);
    let mut code = String::new();

    // Test function header
    code.push_str(&format!("\ttest(\"{}\", async () => {{\n", test_name));

    // Import and create client from per-fixture app factory
    let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
    let app_factory_name = format!("createApp{}", to_pascal_case(&fixture_id));
    code.push_str(&format!("\t\tconst app = {}();\n", app_factory_name));
    code.push_str("\t\tconst client = new TestClient(app);\n\n");

    // Build request
    let method = fixture.request.method.to_lowercase();
    let path = &fixture.request.path;

    // Prepare request options
    let mut has_options = false;
    let mut options_parts = Vec::new();

    // Add headers
    if let Some(ref headers) = fixture.request.headers {
        if !headers.is_empty() {
            has_options = true;
            code.push_str("\t\tconst headers = {\n");
            for (key, value) in headers {
                let escaped_value = escape_string(value);
                code.push_str(&format!("\t\t\t\"{}\": \"{}\",\n", key, escaped_value));
            }
            code.push_str("\t\t};\n");
            options_parts.push("headers");
        }
    }

    // Add body/json
    if let Some(ref body) = fixture.request.body {
        has_options = true;
        let json_str = serde_json::to_string(body)?;
        code.push_str(&format!("\t\tconst json = {};\n", json_str));
        options_parts.push("json");
    }

    // Add form data (for URL-encoded forms)
    if let Some(ref form_data) = fixture.request.form_data {
        has_options = true;
        code.push_str("\t\tconst json = {\n");
        for (key, value) in form_data {
            code.push_str(&format!("\t\t\t\"{}\": {},\n", key, json_to_typescript(value)));
        }
        code.push_str("\t\t};\n");
        options_parts.push("json");
    }

    // Build query params string if present
    let path_with_query = if let Some(ref query_params) = fixture.request.query_params {
        if !query_params.is_empty() {
            let query_string: Vec<String> = query_params
                .iter()
                .map(|(k, v)| {
                    let value_str = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    format!("{}={}", k, urlencoding::encode(&value_str))
                })
                .collect();
            format!("{}?{}", path, query_string.join("&"))
        } else {
            path.clone()
        }
    } else {
        path.clone()
    };

    if method == "get" || method == "delete" {
        if has_options {
            code.push_str(&format!(
                "\t\tconst response = await client.{}(\"{}\", headers);\n\n",
                method, path_with_query
            ));
        } else {
            code.push_str(&format!(
                "\t\tconst response = await client.{}(\"{}\");\n\n",
                method, path_with_query
            ));
        }
    } else {
        code.push_str(&format!(
            "\t\tconst response = await client.{}(\"{}\", {{{}}});\n\n",
            method,
            path_with_query,
            options_parts.join(", ")
        ));
    }

    // Assert status code
    code.push_str(&format!(
        "\t\texpect(response.statusCode).toBe({});\n",
        fixture.expected_response.status_code
    ));

    let status_code = fixture.expected_response.status_code;

    // Different assertion strategies based on status code
    if status_code == 200 {
        // Success case - verify response matches expected
        let should_parse_json =
            fixture.request.method.to_uppercase() != "HEAD" || fixture.expected_response.body.is_some();

        if should_parse_json {
            code.push_str("\t\tconst responseData = response.json();\n");
        }

        // If fixture has expected response body, assert against that
        if let Some(ref expected_body) = fixture.expected_response.body {
            generate_body_assertions(&mut code, expected_body, "responseData", 2);
        } else if should_parse_json {
            // Fallback: verify echoed parameters match what we sent
            if let Some(ref body) = fixture.request.body {
                generate_echo_assertions(&mut code, body, "responseData", 2);
            }

            if let Some(ref form_data) = fixture.request.form_data {
                for (key, value) in form_data {
                    code.push_str(&format!(
                        "\t\texpect(responseData[\"{}\"]).toBe({});\n",
                        key,
                        json_to_typescript(value)
                    ));
                }
            }

            if let Some(ref query_params) = fixture.request.query_params {
                for (key, value) in query_params {
                    code.push_str(&format!(
                        "\t\texpect(responseData[\"{}\"]).toBe({});\n",
                        key,
                        json_to_typescript(value)
                    ));
                }
            }
        }
    } else if status_code == 422 {
        // Validation error - framework should reject before handler
        code.push_str("\t\tconst responseData = response.json();\n");
        code.push_str("\t\t// Validation should be done by framework, not handler\n");
        code.push_str("\t\texpect(responseData).toHaveProperty(\"errors\");\n");
    } else {
        // Other status codes - assert expected response body
        if let Some(ref body) = fixture.expected_response.body {
            code.push_str("\t\tconst responseData = response.json();\n");
            generate_body_assertions(&mut code, body, "responseData", 2);
        }
    }

    code.push_str("\t});\n");

    Ok(code)
}

/// Generate assertions for echoed parameters (success cases)
fn generate_echo_assertions(code: &mut String, sent_value: &serde_json::Value, path: &str, indent_level: usize) {
    let indent = "\t".repeat(indent_level);

    match sent_value {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("{}expect({}).toHaveProperty(\"{}\");\n", indent, path, key));

                match value {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        generate_echo_assertions(code, value, &new_path, indent_level);
                    }
                    _ => {
                        code.push_str(&format!(
                            "{}expect({}).toBe({});\n",
                            indent,
                            new_path,
                            json_to_typescript(value)
                        ));
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("{}expect({}.length).toBe({});\n", indent, path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_echo_assertions(code, item, &new_path, indent_level);
            }
        }
        _ => {
            code.push_str(&format!(
                "{}expect({}).toBe({});\n",
                indent,
                path,
                json_to_typescript(sent_value)
            ));
        }
    }
}

/// Generate assertions for response body
fn generate_body_assertions(code: &mut String, body: &serde_json::Value, path: &str, indent_level: usize) {
    let indent = "\t".repeat(indent_level);

    match body {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("{}expect({}).toHaveProperty(\"{}\");\n", indent, path, key));

                match value {
                    serde_json::Value::Object(_) => {
                        // Skip "ctx" objects in validation errors
                        let skip_ctx = key == "ctx" && path.contains("[\"errors\"]");
                        if !skip_ctx {
                            generate_body_assertions(code, value, &new_path, indent_level);
                        }
                    }
                    serde_json::Value::Array(_) => {
                        generate_body_assertions(code, value, &new_path, indent_level);
                    }
                    _ => {
                        // Skip certain fields inside validation errors
                        let in_errors = path.contains("[\"errors\"]");
                        let skip_assertion = in_errors && (key == "input" || key == "msg" || key == "type");

                        if !skip_assertion {
                            code.push_str(&format!(
                                "{}expect({}).toBe({});\n",
                                indent,
                                new_path,
                                json_to_typescript(value)
                            ));
                        }
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("{}expect({}.length).toBe({});\n", indent, path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_body_assertions(code, item, &new_path, indent_level);
            }
        }
        _ => {
            code.push_str(&format!(
                "{}expect({}).toBe({});\n",
                indent,
                path,
                json_to_typescript(body)
            ));
        }
    }
}

/// Convert JSON value to TypeScript literal
fn json_to_typescript(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            let escaped = escape_string(s);
            format!("\"{}\"", escaped)
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_typescript).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_typescript(v)))
                .collect();
            format!("{{ {} }}", items.join(", "))
        }
    }
}

/// Escape special characters in strings
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Sanitize fixture name for test function
fn sanitize_test_name(name: &str) -> String {
    let mut result = name.replace(
        [
            ' ', '/', '.', '(', ')', '=', ',', ':', '+', '<', '>', '[', ']', '\'', '"',
        ],
        " ",
    );

    // Collapse multiple consecutive spaces
    while result.contains("  ") {
        result = result.replace("  ", " ");
    }

    result.trim().to_string()
}

/// Sanitize a string to be a valid identifier (lowercase snake_case)
fn sanitize_identifier(s: &str) -> String {
    let mut result = s
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

    // Collapse multiple consecutive underscores
    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

/// Convert to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-'][..])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
