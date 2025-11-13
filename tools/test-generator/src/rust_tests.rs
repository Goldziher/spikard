//! Rust test generation

use crate::background::{BackgroundFixtureData, background_data};
use crate::streaming::streaming_data;
use anyhow::{Context, Result};
use spikard_codegen::openapi::Fixture;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

pub fn generate_rust_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Loading fixtures from {}...", fixtures_dir.display());

    // Load fixtures from all subdirectories
    let categories = discover_fixture_categories(fixtures_dir)?;

    println!("Found {} fixture categories", categories.len());

    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    // Generate test file for each category
    for (category, fixtures) in categories {
        let test_file = format!("{}_tests.rs", category);
        let content = generate_category_test_file(&category, &fixtures)?;

        fs::write(tests_dir.join(&test_file), content)
            .with_context(|| format!("Failed to write test file: {}", test_file))?;

        println!("  ✓ Generated {}", test_file);
    }

    // Generate common module
    let common_content = generate_common_module();
    let common_dir = tests_dir.join("common");
    fs::create_dir_all(&common_dir).context("Failed to create common directory")?;
    fs::write(common_dir.join("mod.rs"), common_content).context("Failed to write common/mod.rs")?;

    println!("  ✓ Generated common/mod.rs");

    Ok(())
}

/// Sanitize fixture name to valid Rust identifier (must match rust_app.rs logic)
fn sanitize_fixture_name(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        .trim_matches('_')
        .to_string()
}

fn discover_fixture_categories(fixtures_dir: &Path) -> Result<BTreeMap<String, Vec<(Fixture, String)>>> {
    let mut categories = BTreeMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|n| n.to_str())
                .context("Invalid directory name")?
                .to_string();

            let mut fixtures_with_files = Vec::new();

            // Load fixtures manually to track filenames
            for file_entry in fs::read_dir(&path).context("Failed to read category directory")? {
                let file_entry = file_entry.context("Failed to read file entry")?;
                let file_path = file_entry.path();

                if file_path.extension().is_some_and(|e| e == "json") {
                    let filename = file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .context("Invalid filename")?
                        .to_string();

                    if filename.starts_with("00-") || filename == "schema.json" {
                        continue;
                    }

                    let content = fs::read_to_string(&file_path)?;
                    match serde_json::from_str::<Fixture>(&content) {
                        Ok(fixture) => fixtures_with_files.push((fixture, filename)),
                        Err(e) => {
                            eprintln!("Warning: Skipping {}: {}", file_path.display(), e);
                        }
                    }
                }
            }

            if !fixtures_with_files.is_empty() {
                fixtures_with_files.sort_by(|(fix_a, file_a), (fix_b, file_b)| {
                    fix_a.name.cmp(&fix_b.name).then_with(|| file_a.cmp(file_b))
                });
                categories.insert(category, fixtures_with_files);
            }
        }
    }

    Ok(categories)
}

fn generate_category_test_file(category: &str, fixtures: &[(Fixture, String)]) -> Result<String> {
    let test_name = category.replace('-', "_");

    let mut test_cases = Vec::new();

    for (fixture, filename) in fixtures {
        let mut case_name = fixture
            .name
            .replace(['-', ' ', '/'], "_")
            .replace(['(', ')'], "")
            .replace(':', "_")
            .replace('+', "_plus_")
            .replace('=', "_eq_")
            .replace(['\'', '"'], "")
            .replace(['.', ','], "_")
            .to_lowercase();

        // Replace multiple consecutive underscores with single underscore
        while case_name.contains("__") {
            case_name = case_name.replace("__", "_");
        }

        // Generate the fixture-specific app function name (must match rust_app.rs logic)
        let fixture_id = format!("{}_{}", category, sanitize_fixture_name(&fixture.name));
        let app_fn_name = format!("create_app_{}", fixture_id);

        let fixture_path = format!("../../testing_data/{}/{}", category, filename);
        let streaming_info = streaming_data(fixture)?;
        let background_info = background_data(fixture)?;
        let method = &fixture.request.method;
        // Extract path without query string (before '?')
        let path = fixture.request.path.split('?').next().unwrap_or(&fixture.request.path);
        let expected_status = fixture.expected_response.status_code;

        let test_case = format!(
            r#"
#[tokio::test]
async fn test_{category}_{case_name}() {{
    // Fixture: {fixture_name}
    // Description: {description}
    // Expected status: {expected_status}

    // Load fixture
    let fixture_json = std::fs::read_to_string("{fixture_path}")
        .expect("Failed to read fixture file");
    let fixture: Value = serde_json::from_str(&fixture_json)
        .expect("Failed to parse fixture JSON");

    // Create app for this specific fixture
    let app = spikard_e2e_app::{app_fn_name}();

    // Build request
    let mut uri = "{path}".to_string();

    // Use query_string if provided (for exact encoding control), otherwise build from query_params
    if let Some(query_string) = fixture["request"]["query_string"].as_str() {{
        if !query_string.is_empty() {{
            uri.push_str("?");
            uri.push_str(query_string);
        }}
    }} else if let Some(query_params) = fixture["request"]["query_params"].as_object() {{
        use percent_encoding::{{percent_encode, AsciiSet, CONTROLS}};

        // Define the query component encoding set (RFC 3986)
        // Encode: space, ", #, <, >, %, {{, }}, |, \\, ^, `, and control characters
        // Plus query-specific: &, =, +
        const QUERY: &AsciiSet = &CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'#')
            .add(b'<')
            .add(b'>')
            .add(b'%')
            .add(b'{{')
            .add(b'}}')
            .add(b'|')
            .add(b'\\')
            .add(b'^')
            .add(b'`')
            .add(b'&')
            .add(b'=')
            .add(b'+');

        let query_string = query_params
            .iter()
            .flat_map(|(k, v)| {{
                let key = percent_encode(k.as_bytes(), QUERY).to_string();
                match v {{
                    Value::String(s) => {{
                        let value = percent_encode(s.as_bytes(), QUERY).to_string();
                        vec![format!("{{}}={{}}", key, value)]
                    }},
                    Value::Number(n) => vec![format!("{{}}={{}}", key, n)],
                    Value::Bool(b) => vec![format!("{{}}={{}}", key, b)],
                    Value::Array(arr) => {{
                        // For arrays, repeat the key for each value
                        arr.iter()
                            .filter_map(|item| {{
                                match item {{
                                    Value::String(s) => Some(format!("{{}}={{}}", key, percent_encode(s.as_bytes(), QUERY))),
                                    Value::Number(n) => Some(format!("{{}}={{}}", key, n)),
                                    _ => None,
                                }}
                            }})
                            .collect::<Vec<_>>()
                    }},
                    _ => vec![],
                }}
            }})
            .collect::<Vec<_>>()
            .join("&");
        if !query_string.is_empty() {{
            uri.push_str("?");
            uri.push_str(&query_string);
        }}
    }}

    // Build request with optional body
    let mut request_builder = Request::builder()
        .method("{method}")
        .uri(uri);

    // Add headers from fixture if present
    if let Some(headers) = fixture["request"]["headers"].as_object() {{
        for (key, value) in headers {{
            if let Some(value_str) = value.as_str() {{
                request_builder = request_builder.header(key.as_str(), value_str);
            }}
        }}
    }}

    // Add cookies from fixture if present
    if let Some(cookies) = fixture["request"]["cookies"].as_object() {{
        let cookie_header: Vec<String> = cookies
            .iter()
            .map(|(name, value)| {{
                if let Some(value_str) = value.as_str() {{
                    format!("{{}}={{}}", name, value_str)
                }} else {{
                    format!("{{}}={{}}", name, value)
                }}
            }})
            .collect();
        if !cookie_header.is_empty() {{
            request_builder = request_builder.header("cookie", cookie_header.join("; "));
        }}
    }}

    // Add body if present in fixture
    let body = if let Some(files) = fixture["request"]["files"].as_array() {{
        // Handle multipart/form-data with files
        let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
        let mut multipart_body = String::new();

        // Add files
        for file in files {{
            let field_name = file["field_name"].as_str().unwrap();

            // Handle both regular content and magic_bytes (hex-encoded binary)
            let content_str = if let Some(content) = file["content"].as_str() {{
                content.to_string()
            }} else if let Some(magic_bytes) = file["magic_bytes"].as_str() {{
                // Decode hex string to bytes, then to string
                // For binary data, we'll use the hex representation as placeholder
                format!("<binary data: {{}}>", magic_bytes)
            }} else {{
                String::new()
            }};

            let filename = file["filename"].as_str();
            let content_type = file["content_type"].as_str();

            multipart_body.push_str(&format!("--{{}}\r\n", boundary));
            if let Some(fname) = filename {{
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{{}}\"; filename=\"{{}}\"\r\n", field_name, fname));
                if let Some(ct) = content_type {{
                    multipart_body.push_str(&format!("Content-Type: {{}}\r\n", ct));
                }}
            }} else {{
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{{}}\"\r\n", field_name));
            }}
            multipart_body.push_str("\r\n");
            multipart_body.push_str(&content_str);
            multipart_body.push_str("\r\n");
        }}

        // Add form data fields if present
        if let Some(data) = fixture["request"]["data"].as_object() {{
            for (key, value) in data {{
                multipart_body.push_str(&format!("--{{}}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{{}}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {{
                    multipart_body.push_str(s);
                }} else {{
                    multipart_body.push_str(&value.to_string());
                }}
                multipart_body.push_str("\r\n");
            }}
        }}

        multipart_body.push_str(&format!("--{{}}--\r\n", boundary));

        // Set Content-Type header with boundary
        request_builder = request_builder.header("content-type", format!("multipart/form-data; boundary={{}}", boundary));

        Body::from(multipart_body)
    }} else if let Some(data) = fixture["request"]["data"].as_object() {{
        // Multipart with only form data (no files)
        let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
        let mut multipart_body = String::new();

        for (key, value) in data {{
            multipart_body.push_str(&format!("--{{}}\r\n", boundary));
            multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{{}}\"\r\n\r\n", key));
            if let Some(s) = value.as_str() {{
                multipart_body.push_str(s);
            }} else {{
                multipart_body.push_str(&value.to_string());
            }}
            multipart_body.push_str("\r\n");
        }}

        multipart_body.push_str(&format!("--{{}}--\r\n", boundary));

        // Set Content-Type header with boundary
        request_builder = request_builder.header("content-type", format!("multipart/form-data; boundary={{}}", boundary));

        Body::from(multipart_body)
    }} else if let Some(request_body) = fixture["request"]["body"].as_str() {{
        // Body is already encoded as a string (e.g., URL-encoded form data)
        // Don't override Content-Type if already set
        Body::from(request_body.to_string())
    }} else if let Some(request_body) = fixture["request"]["body"].as_object() {{
        // Body is a JSON object, encode it
        // Only add content-type header if not already set by fixture headers
        if fixture["request"]["headers"]["Content-Type"].is_null() {{
            request_builder = request_builder.header("content-type", "application/json");
        }}
        let body_str = serde_json::to_string(request_body).unwrap();
        Body::from(body_str)
    }} else if let Some(form_data) = fixture["request"]["form_data"].as_object() {{
        // Handle URL-encoded form data
        use percent_encoding::{{percent_encode, NON_ALPHANUMERIC}};

        let form_params: Vec<String> = form_data
            .iter()
            .flat_map(|(key, value)| {{
                match value {{
                    serde_json::Value::Array(arr) => {{
                        // For arrays, repeat the key for each value
                        arr.iter()
                            .map(|item| {{
                                let encoded_value = match item {{
                                    serde_json::Value::String(s) => percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    serde_json::Value::Number(n) => n.to_string(),
                                    serde_json::Value::Bool(b) => b.to_string(),
                                    _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                }};
                                format!("{{}}={{}}", key, encoded_value)
                            }})
                            .collect::<Vec<_>>()
                    }}
                    serde_json::Value::String(s) => {{
                        let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                        vec![format!("{{}}={{}}", key, encoded_value)]
                    }}
                    serde_json::Value::Number(n) => {{
                        vec![format!("{{}}={{}}", key, n)]
                    }}
                    serde_json::Value::Bool(b) => {{
                        vec![format!("{{}}={{}}", key, b)]
                    }}
                    _ => {{
                        let encoded_value = percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                        vec![format!("{{}}={{}}", key, encoded_value)]
                    }}
                }}
            }})
            .collect();

        let body_str = form_params.join("&");

        // Only add content-type header if not already set by fixture headers
        if fixture["request"]["headers"]["Content-Type"].is_null() {{
            request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
        }}

        Body::from(body_str)
    }} else {{
        Body::empty()
    }};

    let request = request_builder.body(body).unwrap();

    let server = TestServer::new(app).unwrap();
    let response = server.call(request).await;
    let snapshot = snapshot_response(response).await.unwrap();

    assert_eq!(
        snapshot.status,
        {expected_status},
        "Expected status {expected_status}, got {{}}",
        snapshot.status
    );
{streaming_assertions}{background_assertions}{header_assertions}
}}
"#,
            category = test_name,
            case_name = case_name,
            fixture_name = fixture.name,
            description = fixture.description,
            fixture_path = fixture_path,
            method = method,
            path = path,
            expected_status = expected_status,
            streaming_assertions = generate_streaming_assertions(&streaming_info),
            background_assertions = generate_background_assertions(&background_info),
            header_assertions = generate_header_assertions(&fixture.expected_response.headers),
        );

        test_cases.push(test_case);
    }

    Ok(format!(
        r#"//! Tests for {category} fixtures
//! Generated from: testing_data/{category}

#[cfg(test)]
mod {test_name} {{
use axum::body::Body;
use axum::http::Request;
use axum_test::TestServer;
use serde_json::Value;
use spikard_http::testing::snapshot_response;

{test_cases}
}}
"#,
        category = category,
        test_name = test_name,
        test_cases = test_cases.join("\n"),
    ))
}

fn generate_common_module() -> String {
    r#"//! Common test utilities

pub mod client {
    // TODO: Add HTTP client helpers
}

pub mod fixtures {
    // TODO: Add fixture loading helpers
}
"#
    .to_string()
}

fn generate_streaming_assertions(info: &Option<crate::streaming::StreamingFixtureData>) -> String {
    if let Some(stream) = info {
        let expected_literal = rust_vec_literal(&stream.expected_bytes);
        format!(
            "    let expected: Vec<u8> = {expected_literal};\n    assert_eq!(snapshot.body, expected);\n    return;\n",
            expected_literal = expected_literal
        )
    } else {
        String::new()
    }
}

fn generate_background_assertions(background: &Option<BackgroundFixtureData>) -> String {
    if let Some(bg) = background {
        let expected_value =
            serde_json::json!({ bg.state_key.clone(): serde_json::Value::Array(bg.expected_state.clone()) });
        let expected_literal = serde_json::to_string(&expected_value).unwrap();
        return format!(
            r##"    let state_request = Request::builder()
        .method("GET")
        .uri("{state_path}")
        .body(Body::empty())
        .unwrap();
    let state_response = server.call(state_request).await;
    let state_snapshot = snapshot_response(state_response).await.unwrap();
    assert_eq!(state_snapshot.status, 200);
    let state_json: Value = serde_json::from_slice(&state_snapshot.body).unwrap();
    let expected_state: Value = serde_json::from_str(r#"{expected}"#).unwrap();
    assert_eq!(state_json, expected_state);
"##,
            state_path = bg.state_path,
            expected = expected_literal
        );
    }
    String::new()
}

fn rust_vec_literal(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "Vec::new()".to_string()
    } else {
        let entries = bytes
            .iter()
            .map(|byte| format!("0x{:02x}u8", byte))
            .collect::<Vec<_>>()
            .join(", ");
        format!("vec![{}]", entries)
    }
}

fn generate_header_assertions(headers: &Option<HashMap<String, String>>) -> String {
    if let Some(map) = headers {
        if map.is_empty() {
            return String::new();
        }
        let mut code = String::from("    let headers = &snapshot.headers;\n");
        for (key, value) in map {
            let lower_key = key.to_ascii_lowercase();
            let escaped_key = escape_rust_string(&lower_key);
            match value.as_str() {
                "<<absent>>" => {
                    code.push_str(&format!(
                        "    assert!(\n        !headers.contains_key(\"{key}\")\n    , \"Expected header '{orig}' to be absent, got {{:?}}\", headers.get(\"{key}\"));\n",
                        key = escaped_key,
                        orig = escape_rust_string(key)
                    ));
                }
                "<<present>>" => {
                    code.push_str(&format!(
                        "    assert!(headers.contains_key(\"{key}\"), \"Expected header '{orig}' to be present\");\n",
                        key = escaped_key,
                        orig = escape_rust_string(key)
                    ));
                }
                "<<uuid>>" => {
                    code.push_str(&format!(
                        "    if let Some(value) = headers.get(\"{key}\") {{\n        assert!(uuid::Uuid::parse_str(value).is_ok(), \"Expected header '{orig}' to be a UUID, got {{}}\", value);\n    }} else {{\n        panic!(\"Expected header '{orig}' to be present\");\n    }}\n",
                        key = escaped_key,
                        orig = escape_rust_string(key)
                    ));
                }
                _ => {
                    let escaped_value = escape_rust_string(value);
                    code.push_str(&format!(
                        "    if let Some(actual) = headers.get(\"{key}\") {{\n        assert_eq!(actual, \"{value}\", \"Mismatched header '{orig}'\");\n    }} else {{\n        panic!(\"Expected header '{orig}' to be present\");\n    }}\n",
                        key = escaped_key,
                        value = escaped_value,
                        orig = escape_rust_string(key)
                    ));
                }
            }
        }
        return code;
    }
    String::new()
}

fn escape_rust_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\"', "\\\"")
}
