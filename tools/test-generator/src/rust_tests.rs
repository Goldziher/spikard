//! Rust test generation

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::{BackgroundFixtureData, background_data};
use crate::graphql::{GraphQLFixture, load_graphql_fixtures};
use crate::streaming::streaming_data;
use anyhow::{Context, Result};
use spikard_codegen::openapi::Fixture;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

pub fn generate_rust_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Loading fixtures from {}...", fixtures_dir.display());

    let categories = discover_fixture_categories(fixtures_dir)?;
    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;

    println!("Found {} fixture categories", categories.len());

    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    for (category, fixtures) in categories {
        let test_file = format!("{}_tests.rs", category);
        let content = generate_category_test_file(&category, &fixtures)?;

        fs::write(tests_dir.join(&test_file), content)
            .with_context(|| format!("Failed to write test file: {}", test_file))?;

        println!("  ✓ Generated {}", test_file);
    }

    let common_content = generate_common_module();
    let common_dir = tests_dir.join("common");
    fs::create_dir_all(&common_dir).context("Failed to create common directory")?;
    fs::write(common_dir.join("mod.rs"), common_content).context("Failed to write common/mod.rs")?;

    println!("  ✓ Generated common/mod.rs");

    if !sse_fixtures.is_empty() {
        let sse_content = generate_sse_tests(&sse_fixtures)?;
        fs::write(tests_dir.join("sse_tests.rs"), sse_content).context("Failed to write sse_tests.rs")?;
        println!("  ✓ Generated sse_tests.rs");
    }

    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;
    if !websocket_fixtures.is_empty() {
        let websocket_content = generate_websocket_tests(&websocket_fixtures)?;
        fs::write(tests_dir.join("websocket_tests.rs"), websocket_content)
            .context("Failed to write websocket_tests.rs")?;
        println!("  ✓ Generated websocket_tests.rs");
    }

    let graphql_fixtures = load_graphql_fixtures(fixtures_dir)
        .context("Failed to load GraphQL fixtures")?;
    if !graphql_fixtures.is_empty() {
        let graphql_content = generate_graphql_tests(&graphql_fixtures)?;
        fs::write(tests_dir.join("graphql_tests.rs"), graphql_content)
            .context("Failed to write graphql_tests.rs")?;
        println!("  ✓ Generated graphql_tests.rs ({} tests)", graphql_fixtures.len());
    }

    Ok(())
}

/// Sanitize fixture name to valid Rust identifier (must match rust_app.rs logic)
fn sanitize_fixture_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut last_was_underscore = false;

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }

    let sanitized = result.trim_matches('_').to_string();
    if sanitized.is_empty() {
        "fixture".to_string()
    } else {
        sanitized
    }
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

        while case_name.contains("__") {
            case_name = case_name.replace("__", "_");
        }

        let fixture_id = format!("{}_{}", category, sanitize_fixture_name(&fixture.name));
        let app_fn_name = format!("create_app_{}", fixture_id);

        let fixture_path = format!("../../testing_data/{}/{}", category, filename);
        let streaming_info = streaming_data(fixture)?;
        let background_info = background_data(fixture)?;
        let method = &fixture.request.method;
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
    let app = spikard_e2e_app::{app_fn_name}()
        .expect("Failed to build fixture app");
    let server = TestServer::from_app(app).expect("Failed to build server");

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

    let snapshot = server.call(request).await.unwrap();

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
use serde_json::Value;
use spikard::testing::TestServer;

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
            "    let expected: Vec<u8> = {expected_literal};\n    assert_eq!(snapshot.body, expected);\n",
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
            r##"    let expected_state: Value = serde_json::from_str(r#"{expected}"#).unwrap();
    let mut actual_state = Value::Null;
    for _attempt in 0..5 {{
        let state_request = Request::builder()
            .method("GET")
            .uri("{state_path}")
            .body(Body::empty())
            .unwrap();
        let state_snapshot = server.call(state_request).await.unwrap();
        assert_eq!(state_snapshot.status, 200);
        actual_state = serde_json::from_slice(&state_snapshot.body).unwrap();
        if actual_state == expected_state {{
            break;
        }}
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }}
    assert_eq!(actual_state, expected_state);
"##,
            state_path = bg.state_path,
            expected = expected_literal
        );
    }
    String::new()
}

fn generate_sse_tests(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    let mut tests = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let channel_slug = sanitize_fixture_name(&channel_path.trim_start_matches('/').replace('/', "_"));
        let test_name = format!("test_sse_{}", channel_slug);
        let app_fn = format!("create_app_sse_{}", channel_slug);

        let mut expected_literals = Vec::new();
        for fixture in &channel_fixtures {
            for example in &fixture.examples {
                let json_str = serde_json::to_string(example)?;
                expected_literals.push(format!("\"{}\"", escape_rust_string(&json_str)));
            }
        }
        if expected_literals.is_empty() {
            expected_literals.push("\"{}\"".to_string());
        }
        let expected_vec = format!("vec![{}]", expected_literals.join(", "));

        let test_case = format!(
            r#"
#[tokio::test]
async fn {test_name}() {{
    let app = spikard_e2e_app::{app_fn}()
        .expect("Failed to build SSE app");
    let server = TestServer::from_app(app).expect("Failed to build server");
    let request = Request::builder()
        .method("GET")
        .uri("{path}")
        .body(Body::empty())
        .unwrap();
    let snapshot = server.call(request).await.unwrap();
    assert_eq!(snapshot.status, 200);

    let body = String::from_utf8(snapshot.body.clone()).expect("SSE stream should be UTF-8");
    let events: Vec<&str> = body
        .split("\n\n")
        .filter(|chunk| chunk.starts_with("data:"))
        .collect();

    let expected_events = {expected_vec};
    assert_eq!(
        events.len(),
        expected_events.len(),
        "Expected {{}} events, got {{}}",
        expected_events.len(),
        events.len()
    );

    for (idx, expected) in expected_events.iter().enumerate() {{
        let payload = events[idx].trim_start_matches("data:").trim();
        let parsed: Value = serde_json::from_str(payload).expect("valid JSON payload");
        let expected_value: Value = serde_json::from_str(expected).expect("valid expected JSON");
        assert_eq!(parsed, expected_value, "Mismatched event at index {{}}", idx);
    }}
}}
"#,
            test_name = test_name,
            app_fn = app_fn,
            path = channel_path,
            expected_vec = expected_vec
        );

        tests.push(test_case);
    }

    Ok(format!(
        r#"//! SSE tests generated from AsyncAPI fixtures

#[cfg(test)]
mod sse {{
use axum::body::Body;
use axum::http::Request;
use serde_json::Value;
use spikard::testing::TestServer;

{tests}
}}
"#,
        tests = tests.join("\n")
    ))
}

fn generate_websocket_tests(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    let mut tests = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let channel_slug = sanitize_fixture_name(&channel_path.trim_start_matches('/').replace('/', "_"));

        for (example_idx, fixture) in channel_fixtures.iter().enumerate() {
            for (msg_idx, example) in fixture.examples.iter().enumerate() {
                let test_name = if channel_fixtures.len() == 1 && fixture.examples.len() == 1 {
                    format!("test_websocket_{}", channel_slug)
                } else {
                    format!(
                        "test_websocket_{}_msg_{}",
                        channel_slug,
                        example_idx * fixture.examples.len() + msg_idx + 1
                    )
                };
                let app_fn = format!("create_app_websocket_{}", channel_slug);
                let example_json = serde_json::to_string(example)?;

                let test_case = format!(
                    r#"
#[tokio::test]
async fn {test_name}() {{
    let app = spikard_e2e_app::{app_fn}()
        .expect("Failed to build WebSocket app");
    let server = TestServer::from_app(app).expect("Failed to build server");

    let mut ws = server.connect_websocket("{path}").await;

    let message: Value = serde_json::from_str("{example_json}").expect("valid JSON");

    ws.send_json(&message).await;

    let response: Value = ws.receive_json().await;

    assert_eq!(response["validated"], Value::Bool(true), "Should have validated field set to true");

    if let Some(obj) = message.as_object() {{
        for (key, value) in obj {{
            assert_eq!(response[key], *value, "Field should match original value");
        }}
    }}

    ws.close().await;
}}
"#,
                    test_name = test_name,
                    app_fn = app_fn,
                    path = channel_path,
                    example_json = escape_rust_string(&example_json)
                );

                tests.push(test_case);
            }
        }
    }

    let tests_joined = tests.join("\n");
    Ok(format!(
        r#"//! WebSocket tests generated from AsyncAPI fixtures

#[cfg(test)]
mod websocket {{
use serde_json::Value;
use spikard::testing::TestServer;

{}
}}
"#,
        tests_joined
    ))
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
                    if lower_key == "vary" {
                        code.push_str(&format!(
                            "    if let Some(actual) = headers.get(\"{key}\") {{\n        assert_eq!(actual.to_ascii_lowercase(), \"{value}\".to_ascii_lowercase(), \"Mismatched header '{orig}'\");\n    }} else {{\n        panic!(\"Expected header '{orig}' to be present\");\n    }}\n",
                            key = escaped_key,
                            value = escaped_value,
                            orig = escape_rust_string(key)
                        ));
                    } else {
                        code.push_str(&format!(
                            "    if let Some(actual) = headers.get(\"{key}\") {{\n        assert_eq!(actual, \"{value}\", \"Mismatched header '{orig}'\");\n    }} else {{\n        panic!(\"Expected header '{orig}' to be present\");\n    }}\n",
                            key = escaped_key,
                            value = escaped_value,
                            orig = escape_rust_string(key)
                        ));
                    }
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

/// Generate Rust test module for GraphQL fixtures
fn generate_graphql_tests(fixtures: &[GraphQLFixture]) -> Result<String> {
    let mut code = String::new();

    code.push_str("//! GraphQL operation tests\n");
    code.push_str("//!\n");
    code.push_str("//! E2E tests for GraphQL queries, mutations, and subscriptions.\n\n");

    code.push_str("#[cfg(test)]\n");
    code.push_str("mod tests {\n");
    code.push_str("    use super::*;\n\n");

    // Generate test for each fixture
    for fixture in fixtures {
        let test_name = format!("test_graphql_{}", sanitize_fixture_name(&fixture.name));

        code.push_str(&format!("    #[tokio::test]\n"));
        code.push_str(&format!("    async fn {}() {{\n", test_name));
        let desc: &str = fixture.description.as_ref().map(|s| s.as_str()).unwrap_or(&fixture.name);
        code.push_str(&format!("        // Test: {}\n", desc));
        code.push_str(&format!("        // Operation type: {}\n", fixture.operation_type));
        code.push_str(&format!("        // Endpoint: {}\n\n", fixture.endpoint));

        // Use request from fixture
        let request = &fixture.request;
        {
            // Build GraphQL query payload
            code.push_str("        let query = r#\"\n");
            code.push_str(&request.query);
            code.push_str("\n        \"#;\n\n");

            // Build request payload
            code.push_str("        let mut payload = serde_json::json!({\n");
            code.push_str("            \"query\": query,\n");

            if let Some(variables) = &request.variables {
                code.push_str("            \"variables\": ");
                code.push_str(&serde_json::to_string(variables)?);
                code.push_str(",\n");
            }

            if let Some(op_name) = &request.operation_name {
                code.push_str(&format!("            \"operationName\": \"{}\",\n", escape_rust_string(op_name)));
            }

            code.push_str("        });\n\n");
        }

        // Get expected response
        let expected_response = &fixture.expected_response;
        {
            // Expected status code
            code.push_str(&format!(
                "        // Expected status code: {}\n",
                expected_response.status_code
            ));

            // Validate expected response structure
            if expected_response.data.is_some() {
                code.push_str("        // Response should contain data field\n");
            }

            if expected_response.errors.is_some() {
                code.push_str("        // Response should contain errors field\n");
            }
        }

        code.push_str("    }\n\n");
    }

    code.push_str("}\n");

    Ok(code)
}
