//! Python test generator
//!
//! Generates pytest test suites from fixtures for e2e testing.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::background_data;
use crate::codegen_utils::json_to_python;
use crate::dependencies::{DependencyConfig, has_cleanup, requires_multi_request_test};
use crate::fixture_filter::is_http_fixture_category;
use crate::graphql::{GraphQLFixture, load_graphql_fixtures};
use crate::grpc::GrpcFixture;
use crate::jsonrpc::JsonRpcFixture;
use crate::middleware::parse_middleware;
use crate::streaming::streaming_data;
use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate Python test suite from fixtures
pub fn generate_python_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Python tests...");

    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    let conftest_content = generate_conftest();
    fs::write(tests_dir.join("conftest.py"), conftest_content).context("Failed to write conftest.py")?;

    fs::write(tests_dir.join("__init__.py"), "\"\"\"E2E tests.\"\"\"\n").context("Failed to write __init__.py")?;

    let mut fixtures_by_category: HashMap<String, Vec<Fixture>> = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path.file_name().unwrap().to_str().unwrap().to_string();
            if !is_http_fixture_category(&category) {
                continue;
            }
            let fixtures = load_fixtures_from_dir(&path)?;

            if !fixtures.is_empty() {
                fixtures_by_category.insert(category, fixtures);
            }
        }
    }
    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;
    let jsonrpc_fixtures =
        crate::jsonrpc::load_jsonrpc_fixtures(fixtures_dir).context("Failed to load JSON-RPC fixtures")?;
    let graphql_fixtures = load_graphql_fixtures(fixtures_dir).context("Failed to load GraphQL fixtures")?;

    for (category, fixtures) in fixtures_by_category.iter() {
        let test_content = generate_test_file(category, fixtures)?;
        let test_file = tests_dir.join(format!("test_{}.py", category));
        fs::write(&test_file, test_content).with_context(|| format!("Failed to write test file for {}", category))?;
        println!("  ✓ Generated tests/test_{}.py ({} tests)", category, fixtures.len());
    }

    if !sse_fixtures.is_empty() {
        let sse_content = generate_sse_test_module(&sse_fixtures)?;
        fs::write(tests_dir.join("test_asyncapi_sse.py"), sse_content)
            .context("Failed to write test_asyncapi_sse.py")?;
        println!("  ✓ Generated tests/test_asyncapi_sse.py");
    }

    if !websocket_fixtures.is_empty() {
        let websocket_content = generate_websocket_test_module(&websocket_fixtures)?;
        fs::write(tests_dir.join("test_asyncapi_websocket.py"), websocket_content)
            .context("Failed to write test_asyncapi_websocket.py")?;
        println!("  ✓ Generated tests/test_asyncapi_websocket.py");
    }

    if !jsonrpc_fixtures.is_empty() {
        generate_jsonrpc_tests(&jsonrpc_fixtures, output_dir)?;
        println!("  ✓ Generated tests/test_jsonrpc.py");
    }

    if !graphql_fixtures.is_empty() {
        let graphql_content = generate_graphql_test_module(&graphql_fixtures)?;
        fs::write(tests_dir.join("test_graphql.py"), graphql_content).context("Failed to write test_graphql.py")?;
        println!("  ✓ Generated tests/test_graphql.py ({} tests)", graphql_fixtures.len());
    }

    // gRPC test generation
    let grpc_fixtures_result = crate::grpc::load_grpc_fixtures(fixtures_dir);
    if let Ok(grpc_fixtures) = grpc_fixtures_result {
        if !grpc_fixtures.is_empty() {
            for fixture in &grpc_fixtures {
                let test_code = generate_grpc_test(fixture)
                    .context(format!("Failed to generate gRPC test for {}", fixture.name))?;

                let test_name = sanitize_test_name(&fixture.name);
                let test_file = tests_dir.join(format!("test_grpc_{}.py", test_name));

                fs::write(&test_file, test_code)
                    .with_context(|| format!("Failed to write gRPC test file for {}", fixture.name))?;
                println!("  ✓ Generated tests/test_grpc_{}.py", test_name);
            }
        }
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
    let mut needs_asyncio_sleep = false;
    let mut needs_uuid_import = false;
    let mut needs_re_import = false;
    let mut needs_pytest_import = false;
    let mut needs_time_import = false;
    for fixture in fixtures {
        let metadata = parse_middleware(fixture)?;
        if metadata
            .rate_limit
            .as_ref()
            .and_then(|cfg| cfg.sleep_ms_between)
            .is_some()
        {
            needs_asyncio_sleep = true;
        }
        if fixture
            .expected_response
            .headers
            .as_ref()
            .is_some_and(|headers| headers.values().any(|value| value == "<<uuid>>"))
        {
            needs_uuid_import = true;
        }
        if fixture
            .expected_response
            .headers
            .as_ref()
            .is_some_and(|headers| headers.values().any(|value| is_regex_pattern(value)))
        {
            needs_re_import = true;
        }
        if should_skip_due_to_http_client(fixture) {
            needs_pytest_import = true;
        }
        if let Some(di_config) = DependencyConfig::from_fixture(fixture)? {
            if has_cleanup(&di_config) {
                needs_time_import = true;
            }
        }
    }

    let mut code = String::new();

    let mut app_factories: Vec<String> = fixtures
        .iter()
        .map(|fixture| {
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            format!("create_app_{}", fixture_id)
        })
        .collect();
    app_factories.sort();
    app_factories.dedup();

    code.push_str(&format!("\"\"\"E2E tests for {}.\"\"\"\n\n", category));
    if needs_pytest_import {
        code.push_str("import pytest\n");
    }
    if needs_asyncio_sleep {
        code.push_str("import asyncio\n");
    }
    if needs_re_import {
        code.push_str("import re\n");
    }
    if needs_time_import {
        code.push_str("import time\n");
    }
    if needs_uuid_import {
        code.push_str("from uuid import UUID\n");
    }
    if needs_asyncio_sleep || needs_uuid_import || needs_re_import || needs_time_import {
        code.push('\n');
    }
    code.push_str("from spikard.testing import TestClient\n");
    if !app_factories.is_empty() {
        code.push_str("from app.main import (\n");
        for factory in &app_factories {
            code.push_str(&format!("    {},\n", factory));
        }
        code.push_str(")\n");
    }
    code.push('\n');

    for fixture in fixtures {
        let test_function = generate_test_function(category, fixture)?;
        code.push_str(&test_function);
        code.push_str("\n\n");
    }

    Ok(code)
}

fn generate_sse_test_module(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref()
            && (fixture.operations.is_empty()
                || fixture
                    .operations
                    .iter()
                    .any(|op| op.action.eq_ignore_ascii_case("send")))
        {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut test_cases = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("create_app_sse_{}", slug);
        let fixture_literal = build_sse_fixture_names(&channel_fixtures);
        test_cases.push((channel_path, slug, factory_name, fixture_literal));
    }

    let mut imports = String::new();
    imports.push_str("from app.main import (\n");
    for (_, _, factory_name, _) in &test_cases {
        imports.push_str(&format!("    {},\n", factory_name));
    }
    imports.push_str(")\n\n");

    let mut tests = String::new();
    for (channel_path, slug, factory_name, fixture_literal) in test_cases {
        tests.push_str(&format!("async def test_sse_{slug}() -> None:\n"));
        tests.push_str(&format!("    \"\"\"SSE channel test for {channel_path}.\"\"\"\n"));
        tests.push_str(&format!("    async with TestClient({}()) as client:\n", factory_name));
        tests.push_str(&format!("        response = await client.get(\"{channel_path}\")\n"));
        tests.push_str("        assert response.status_code == 200\n");
        tests.push_str("        body = response.text\n");
        tests.push_str("        normalized = body.replace(\"\\r\\n\", \"\\n\")\n");
        tests.push_str(
            "        events = [chunk[5:] for chunk in normalized.split(\"\\n\\n\") if chunk.startswith(\"data:\")]\n",
        );
        tests.push_str(&format!("        fixture_names = {}\n", fixture_literal));
        tests.push_str("        expected = []\n");
        tests.push_str("        for fixture_name in fixture_names:\n");
        tests.push_str("            expected.extend(load_fixture_examples(SSE_FIXTURE_ROOT, fixture_name))\n");
        tests.push_str("        assert len(events) == len(expected)\n");
        tests.push_str("        for payload, expected_json in zip(events, expected):\n");
        tests.push_str("            assert json.loads(payload.strip()) == json.loads(expected_json)\n\n");
    }

    let module = format!(
        "\"\"\"AsyncAPI SSE tests.\"\"\"\n\nimport json\nfrom pathlib import Path\n\nfrom spikard.testing import TestClient\n\nROOT_DIR = Path(__file__).resolve().parents[3]\nSSE_FIXTURE_ROOT = ROOT_DIR / \"testing_data\" / \"sse\"\n\n\ndef load_async_fixture(root: Path, name: str) -> dict:\n    fixture_path = root / f\"{{name}}.json\"\n    with fixture_path.open() as handle:\n        return json.load(handle)\n\n\ndef load_fixture_examples(root: Path, name: str) -> list[str]:\n    data = load_async_fixture(root, name)\n    examples = data.get(\"examples\", [])\n    if not isinstance(examples, list) or not examples:\n        return [json.dumps({{}})]\n    return [json.dumps(example) for example in examples]\n\n{imports}{tests}",
        imports = imports,
        tests = tests
    );

    Ok(module)
}

fn build_sse_fixture_names(fixtures: &[&AsyncFixture]) -> String {
    if fixtures.is_empty() {
        return "[]".to_string();
    }
    let names: Vec<String> = fixtures.iter().map(|fixture| format!("\"{}\"", fixture.name)).collect();
    format!("[{}]", names.join(", "))
}

/// Generate a single test function
fn generate_test_function(category: &str, fixture: &Fixture) -> Result<String> {
    let test_name = sanitize_test_name(&fixture.name);
    let mut code = String::new();
    let streaming_info = streaming_data(fixture)?;
    let background_info = background_data(fixture)?;
    let middleware_meta = parse_middleware(fixture)?;

    code.push_str(&format!("async def test_{}() -> None:\n", test_name));
    code.push_str(&format!("    \"\"\"{}.\"\"\"\n", fixture.description));
    code.push('\n');

    if should_skip_due_to_http_client(fixture) {
        code.push_str(
            "    pytest.skip(\"HTTP client enforces this precondition; cannot emulate malformed request\")\n",
        );
        return Ok(code);
    }

    let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
    let app_factory_name = format!("create_app_{}", fixture_id);
    code.push_str(&format!(
        "    async with TestClient({}()) as client:\n",
        app_factory_name
    ));

    let method = fixture.request.method.to_lowercase();
    let path = &fixture.request.path;

    let mut request_kwargs = Vec::new();

    if let Some(ref query_params) = fixture.request.query_params
        && !query_params.is_empty()
    {
        code.push_str("        params = {\n");
        for (key, value) in query_params {
            code.push_str(&format!("            \"{}\": {},\n", key, json_to_python(value)));
        }
        code.push_str("        }\n");
        request_kwargs.push("params=params");
    }

    if let Some(ref headers) = fixture.request.headers
        && !headers.is_empty()
    {
        code.push_str("        headers = {\n");
        for (key, value) in headers {
            let escaped_value = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            code.push_str(&format!("            \"{}\": \"{}\",\n", key, escaped_value));
        }
        code.push_str("        }\n");
        request_kwargs.push("headers=headers");
    }

    if let Some(ref cookies) = fixture.request.cookies
        && !cookies.is_empty()
    {
        code.push_str("        cookies = {\n");
        for (key, value) in cookies {
            let escaped_value = value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            code.push_str(&format!("        \"{}\": \"{}\",\n", key, escaped_value));
        }
        code.push_str("        }\n");
        request_kwargs.push("cookies=cookies");
    }

    if let Some(ref body) = fixture.request.body {
        let content_type = fixture
            .request
            .headers
            .as_ref()
            .and_then(|headers| headers.get("Content-Type"))
            .map(|value| value.to_ascii_lowercase());

        let is_form_urlencoded = content_type
            .as_deref()
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let treat_as_json = content_type
            .as_deref()
            .map(|ct| ct.contains("application/json") || ct.contains("application/xml") || ct.contains("+json"))
            .unwrap_or(true);

        if is_form_urlencoded {
            code.push_str(&format!("        form_data = {}\n", json_to_python(body)));
            request_kwargs.push("data=form_data");
        } else if treat_as_json {
            code.push_str(&format!("        json_data = {}\n", json_to_python(body)));
            request_kwargs.push("json=json_data");
        } else {
            code.push_str(&format!("        raw_body = {}\n", json_to_python(body)));
            request_kwargs.push("data=raw_body");
        }
    }

    if let Some(ref form_data) = fixture.request.form_data
        && !form_data.is_empty()
    {
        code.push_str(&format!("        form_data = {}\n", hashmap_to_python(form_data)));
        request_kwargs.push("data=form_data");
    }

    if let Some(ref data) = fixture.request.data {
        code.push_str(&format!("        data = {}\n", hashmap_to_python(data)));
        request_kwargs.push("data=data");
    }

    if let Some(ref files) = fixture.request.files
        && !files.is_empty()
    {
        use std::collections::HashMap;
        let mut files_by_name: HashMap<&str, Vec<String>> = HashMap::new();

        for file in files {
            let field_name = file.field_name.as_str();
            let filename = file.filename.as_deref().unwrap_or("file.txt");

            let file_content = if let Some(ref content) = file.content {
                let escaped = content
                    .replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\r", "\\r")
                    .replace("\t", "\\t");
                format!("b\"{}\"", escaped)
            } else if let Some(ref magic_bytes) = file.magic_bytes {
                format!("bytes.fromhex(\"{}\")", magic_bytes)
            } else {
                "b\"\"".to_string()
            };

            let file_tuple = if let Some(ref content_type) = file.content_type {
                format!("(\"{}\", {}, \"{}\")", filename, file_content, content_type)
            } else {
                format!("(\"{}\", {})", filename, file_content)
            };

            files_by_name.entry(field_name).or_default().push(file_tuple);
        }

        let has_multiple_files_per_field = files_by_name.values().any(|v| v.len() > 1);

        if has_multiple_files_per_field {
            code.push_str("        files = [\n");
            for (field_name, file_tuples) in files_by_name.iter() {
                for file_tuple in file_tuples {
                    code.push_str(&format!("            (\"{}\", {}),\n", field_name, file_tuple));
                }
            }
            code.push_str("        ]\n");
        } else {
            code.push_str("        files = {\n");
            for (field_name, file_tuples) in files_by_name.iter() {
                code.push_str(&format!("            \"{}\": {},\n", field_name, file_tuples[0]));
            }
            code.push_str("        }\n");
        }
        request_kwargs.push("files=files");
    }

    let kwargs_str = if request_kwargs.is_empty() {
        String::new()
    } else {
        format!(", {}", request_kwargs.join(", "))
    };
    if let Some(rate_limit) = &middleware_meta.rate_limit
        && rate_limit.warmup_requests > 0
    {
        code.push_str(&format!("        for _ in range({}):\n", rate_limit.warmup_requests));
        code.push_str(&format!(
            "            warmup_response = await client.{}(\"{}\"{})\n",
            method, path, kwargs_str
        ));
        let warmup_status = rate_limit.warmup_expect_status.unwrap_or(200);
        code.push_str(&format!(
            "            assert warmup_response.status_code == {}\n",
            warmup_status
        ));
        if let Some(delay) = rate_limit.sleep_ms_between {
            let sleep_literal = format_sleep_seconds(delay);
            code.push_str(&format!("            await asyncio.sleep({})\n", sleep_literal));
        }
    }

    code.push_str(&format!(
        "        response = await client.{}(\"{}\"{})\n\n",
        method, path, kwargs_str
    ));

    if allows_content_length_timeout(fixture) {
        code.push_str("        assert response.status_code in (400, 408)\n");
        code.push_str("        if response.status_code == 408:\n");
        code.push_str("            return\n");
    } else {
        code.push_str(&format!(
            "        assert response.status_code == {}\n",
            fixture.expected_response.status_code
        ));
    }

    if let Some(stream_info) = streaming_info {
        let expected_literal = python_bytes_literal(&stream_info.expected_bytes);
        code.push_str(&format!("        expected_bytes = {}\n", expected_literal));
        code.push_str("        assert response.content == expected_bytes\n");
        if stream_info.is_text_only {
            code.push_str("        assert response.text == expected_bytes.decode()\n");
        }
        return Ok(code);
    }

    if let Some(bg) = background_info {
        code.push_str(&format!(
            "        state_response = await client.get(\"{}\")\n",
            bg.state_path
        ));
        code.push_str("        assert state_response.status_code == 200\n");
        let expected_state_value = serde_json::Value::Array(bg.expected_state.clone());
        let expected_body = format!("{{\"{}\": {} }}", bg.state_key, json_to_python(&expected_state_value));
        code.push_str(&format!("        assert state_response.json() == {}\n", expected_body));
        return Ok(code);
    }

    if let Some(di_config) = DependencyConfig::from_fixture(fixture)? {
        if requires_multi_request_test(&di_config) {
            let expected_keys = fixture
                .expected_response
                .body
                .as_ref()
                .and_then(|v| v.as_object())
                .map(|obj| obj.keys().cloned().collect::<std::collections::HashSet<_>>())
                .unwrap_or_default();

            code.push_str("\n");
            code.push_str("        # Second request to verify singleton caching\n");
            let request_str = if request_kwargs.is_empty() {
                format!("        response2 = await client.{}(\"{}\")\n", method, path)
            } else {
                format!(
                    "        response2 = await client.{}(\"{}\", {})\n",
                    method,
                    path,
                    request_kwargs.join(", ")
                )
            };
            code.push_str(&request_str);
            code.push_str("        assert response2.status_code == 200\n");
            code.push_str("        data1 = response.json()\n");
            code.push_str("        data2 = response2.json()\n");
            code.push_str("\n");
            if expected_keys.contains("counter_id") && expected_keys.contains("count") {
                code.push_str("        # Singleton counter should have stable counter_id and incremented count\n");
                code.push_str("        assert \"counter_id\" in data1 and \"counter_id\" in data2\n");
                code.push_str("        assert data1[\"counter_id\"] == data2[\"counter_id\"]\n");
                code.push_str("        assert data2[\"count\"] > data1[\"count\"]\n");
            } else if expected_keys.contains("pool_id") && expected_keys.contains("context_id") {
                code.push_str("        # pool_id is singleton; context_id is per-request\n");
                code.push_str("        assert \"pool_id\" in data1 and \"pool_id\" in data2\n");
                code.push_str("        assert data1[\"pool_id\"] == data2[\"pool_id\"]\n");
                code.push_str("        assert \"context_id\" in data1 and \"context_id\" in data2\n");
                code.push_str("        assert data1[\"context_id\"] != data2[\"context_id\"]\n");
            } else {
                code.push_str("        # Singleton should have same ID but incremented count\n");
                code.push_str("        assert \"id\" in data1 and \"id\" in data2\n");
                code.push_str("        assert data1[\"id\"] == data2[\"id\"]  # Same singleton instance\n");
                code.push_str("        if \"count\" in data1 and \"count\" in data2:\n");
                code.push_str("            assert data2[\"count\"] > data1[\"count\"]  # Count incremented\n");
            }
            return Ok(code);
        }

        if has_cleanup(&di_config) {
            code.push_str("\n");
            code.push_str("        # Allow async cleanup to complete\n");
            code.push_str("        time.sleep(0.1)\n");
            code.push_str("\n");
            code.push_str("        # Verify cleanup was called\n");
            code.push_str("        cleanup_response = await client.get(\"/api/cleanup-state\")\n");
            code.push_str("        assert cleanup_response.status_code == 200\n");
            code.push_str("        cleanup_state = cleanup_response.json()\n");
            code.push_str("        assert \"cleanup_events\" in cleanup_state\n");
            code.push_str("        events = cleanup_state[\"cleanup_events\"]\n");
            code.push_str("        assert \"session_opened\" in events\n");
            code.push_str("        assert \"session_closed\" in events\n");
            return Ok(code);
        }
    }

    let status_code = fixture.expected_response.status_code;
    let method = fixture.request.method.to_uppercase();
    let expected_string_body = fixture.expected_response.body.as_ref().and_then(|body| body.as_str());
    let content_length_header = expected_content_length(fixture);
    let requires_binary_assert = expected_string_body.is_some() && content_length_header.is_some();

    if status_code == 200 {
        let is_text_response = fixture
            .expected_response
            .headers
            .as_ref()
            .and_then(|h| h.get("content-type"))
            .map(|ct| ct.starts_with("text/"))
            .unwrap_or(false);

        let should_parse_json = !requires_binary_assert
            && !is_text_response
            && (method != "HEAD" || fixture.expected_response.body.is_some());

        if should_parse_json {
            code.push_str("        response_data = response.json()\n");
        }

        if let Some(ref expected_body) = fixture.expected_response.body {
            if requires_binary_assert {
                let expected_literal = python_bytes_literal(expected_body.as_str().unwrap().as_bytes());
                code.push_str("        body_bytes = response.content\n");
                code.push_str(&format!(
                    "        assert len(body_bytes) == {}\n",
                    content_length_header.unwrap()
                ));
                code.push_str(&format!("        assert body_bytes.startswith({})\n", expected_literal));
            } else if is_text_response && expected_body.is_string() {
                code.push_str(&format!(
                    "        assert response.text == {}\n",
                    json_to_python(expected_body)
                ));
            } else {
                generate_body_assertions(&mut code, expected_body, "response_data");
            }
        } else if should_parse_json {
            if let Some(ref body) = fixture.request.body {
                generate_echo_assertions(&mut code, body, "response_data");
            }

            if let Some(ref form_data) = fixture.request.form_data {
                for (key, value) in form_data {
                    code.push_str(&format!(
                        "        assert response_data[\"{}\"] == {}\n",
                        key,
                        json_to_python(value)
                    ));
                }
            }

            if let Some(ref query_params) = fixture.request.query_params {
                for (key, value) in query_params {
                    code.push_str(&format!(
                        "        assert response_data[\"{}\"] == {}\n",
                        key,
                        json_to_python(value)
                    ));
                }
            }
        }
    } else if status_code == 422 {
        code.push_str("        response_data = response.json()\n");
        code.push_str("        # Validation should be done by framework, not handler\n");
        code.push_str("        assert \"errors\" in response_data or \"detail\" in response_data\n");
    } else if let Some(ref body) = fixture.expected_response.body {
        if requires_binary_assert {
            let expected_literal = python_bytes_literal(body.as_str().unwrap().as_bytes());
            code.push_str("        body_bytes = response.content\n");
            code.push_str(&format!(
                "        assert len(body_bytes) == {}\n",
                content_length_header.unwrap()
            ));
            code.push_str(&format!("        assert body_bytes.startswith({})\n", expected_literal));
        } else {
            code.push_str("        response_data = response.json()\n");
            generate_body_assertions(&mut code, body, "response_data");
        }
    }

    if let Some(ref errors) = fixture.expected_response.validation_errors {
        code.push_str("        response_data = response.json()\n");
        code.push_str("        assert \"errors\" in response_data\n");
        code.push_str(&format!(
            "        assert len(response_data[\"errors\"]) == {}\n",
            errors.len()
        ));

        for (idx, error) in errors.iter().enumerate() {
            code.push_str(&format!("        error_{} = response_data[\"errors\"][{}]\n", idx, idx));
            code.push_str(&format!(
                "        assert error_{}[\"type\"] == \"{}\"\n",
                idx, error.error_type
            ));
            code.push_str(&format!(
                "        assert error_{}[\"loc\"] == [{}]\n",
                idx,
                error
                    .loc
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            code.push_str(&format!("        assert error_{}[\"msg\"] == \"{}\"\n", idx, error.msg));
        }
    }

    if let Some(headers) = fixture.expected_response.headers.as_ref().filter(|map| !map.is_empty()) {
        code.push_str("        response_headers = response.headers\n");
        for (key, value) in headers.iter() {
            let lookup_key = key.to_ascii_lowercase();
            match value.as_str() {
                "<<uuid>>" => {
                    code.push_str(&format!(
                        "        header_value = response_headers.get(\"{}\")\n",
                        lookup_key
                    ));
                    code.push_str("        assert header_value is not None\n");
                    code.push_str("        UUID(header_value)\n");
                }
                "<<present>>" => {
                    code.push_str(&format!(
                        "        assert response_headers.get(\"{}\") is not None\n",
                        lookup_key
                    ));
                }
                "<<absent>>" => {
                    code.push_str(&format!(
                        "        assert response_headers.get(\"{}\") is None\n",
                        lookup_key
                    ));
                }
                _ => {
                    if is_regex_pattern(value.as_str()) {
                        code.push_str(&format!(
                            "        header_value = response_headers.get(\"{}\")\n",
                            lookup_key
                        ));
                        code.push_str("        assert header_value is not None\n");
                        code.push_str(&format!(
                            "        assert re.match(r\"{}\", header_value)\n",
                            value.replace("\\", "\\\\")
                        ));
                    } else {
                        let expected = json_to_python(&serde_json::Value::String(value.clone()));
                        code.push_str(&format!(
                            "        assert response_headers.get(\"{}\") == {}\n",
                            lookup_key, expected
                        ));
                    }
                }
            }
        }
    }

    Ok(code)
}

fn python_bytes_literal(bytes: &[u8]) -> String {
    let mut literal = String::from("b\"");
    for &byte in bytes {
        match byte {
            b'\\' => literal.push_str("\\\\"),
            b'"' => literal.push_str("\\\""),
            b'\n' => literal.push_str("\\n"),
            b'\r' => literal.push_str("\\r"),
            b'\t' => literal.push_str("\\t"),
            0x20..=0x7e => literal.push(byte as char),
            _ => literal.push_str(&format!("\\x{:02x}", byte)),
        }
    }
    literal.push('"');
    literal
}

fn expected_content_length(fixture: &Fixture) -> Option<usize> {
    fixture
        .expected_response
        .headers
        .as_ref()
        .and_then(|headers| headers.get("Content-Length").or_else(|| headers.get("content-length")))
        .and_then(|value| value.parse::<usize>().ok())
}

fn allows_content_length_timeout(fixture: &Fixture) -> bool {
    fixture
        .category
        .as_deref()
        .map(|cat| cat == "content_types")
        .unwrap_or(false)
        && fixture.name.contains("content_length_mismatch")
}

fn should_skip_due_to_http_client(fixture: &Fixture) -> bool {
    allows_content_length_timeout(fixture)
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

/// Generate assertions for echoed parameters (success cases)
/// Verifies that the response contains the same values that were sent
fn generate_echo_assertions(code: &mut String, sent_value: &serde_json::Value, path: &str) {
    match sent_value {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("        assert \"{}\" in {}\n", key, path));

                match value {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        generate_echo_assertions(code, value, &new_path);
                    }
                    _ => {
                        code.push_str(&format!("        assert {} == {}\n", new_path, json_to_python(value)));
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("        assert len({}) == {}\n", path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_echo_assertions(code, item, &new_path);
            }
        }
        _ => {
            code.push_str(&format!("        assert {} == {}\n", path, json_to_python(sent_value)));
        }
    }
}

/// Generate assertions for response body
fn generate_body_assertions(code: &mut String, body: &serde_json::Value, path: &str) {
    match body {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format!("{}[\"{}\"]", path, key);
                code.push_str(&format!("        assert \"{}\" in {}\n", key, path));

                match value {
                    serde_json::Value::Object(_) => {
                        let skip_ctx = key == "ctx" && path.contains("[\"errors\"]");
                        if !skip_ctx {
                            generate_body_assertions(code, value, &new_path);
                        }
                    }
                    serde_json::Value::Array(_) => {
                        generate_body_assertions(code, value, &new_path);
                    }
                    _ => {
                        let in_errors = path.contains("[\"errors\"]");
                        let skip_assertion = in_errors && (key == "input" || key == "msg" || key == "type");

                        if !skip_assertion {
                            code.push_str(&format!("        assert {} == {}\n", new_path, json_to_python(value)));
                        }
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("        assert len({}) == {}\n", path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_body_assertions(code, item, &new_path);
            }
        }
        _ => {
            code.push_str(&format!("        assert {} == {}\n", path, json_to_python(body)));
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

/// Sanitize fixture name for test function
fn sanitize_test_name(name: &str) -> String {
    let mut result = name.to_lowercase().replace(
        [
            ' ', '-', '/', '.', '(', ')', '=', ',', ':', '+', '<', '>', '[', ']', '\'', '"',
        ],
        "_",
    );

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

    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

/// Checks if a string looks like a regex pattern
fn is_regex_pattern(value: &str) -> bool {
    value.contains(".*")
        || value.contains(".+")
        || value.contains("\\d")
        || value.contains("\\w")
        || value.contains("\\s")
        || value.contains("[")
        || value.contains("]")
        || value.contains("^")
        || value.contains("$")
        || (value.contains("?") && !value.starts_with("http"))
        || (value.contains("+") && !value.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '-'))
}

fn generate_websocket_test_module(fixtures: &[AsyncFixture]) -> Result<String> {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    let mut lookup: HashMap<&str, &AsyncFixture> = HashMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
        lookup.insert(fixture.name.as_str(), fixture);
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut test_cases = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let channel_path = if channel.starts_with('/') {
            channel
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("create_app_websocket_{}", slug);

        let mut test_messages = Vec::new();
        for fixture in &channel_fixtures {
            let has_receive = fixture.operations.is_empty()
                || fixture
                    .operations
                    .iter()
                    .any(|op| op.action.eq_ignore_ascii_case("receive"));
            if !has_receive {
                continue;
            }
            let reply_literal = websocket_reply_literal(fixture, &lookup)?;
            test_messages.push((fixture.name.clone(), reply_literal));
        }

        test_cases.push((channel_path, slug, factory_name, test_messages));
    }

    let mut imports = String::new();
    imports.push_str("from app.main import (\n");
    for (_, _, factory_name, _) in &test_cases {
        imports.push_str(&format!("    {},\n", factory_name));
    }
    imports.push_str(")\n\n");

    let mut tests = String::new();
    for (channel_path, slug, factory_name, test_messages) in test_cases {
        tests.push_str(&format!("async def test_websocket_{slug}() -> None:\n"));
        tests.push_str(&format!("    \"\"\"WebSocket channel test for {channel_path}.\"\"\"\n"));
        tests.push_str(&format!("    async with TestClient({}()) as client:\n", factory_name));
        tests.push_str(&format!(
            "        async with client.websocket(\"{channel_path}\") as ws:\n"
        ));

        for (fixture_name, reply_literal) in test_messages {
            tests.push_str(&format!("            # {fixture_name} messages\n"));
            tests.push_str(&format!(
                "            messages = load_fixture_examples(WEBSOCKET_FIXTURE_ROOT, \"{}\")\n",
                fixture_name
            ));
            tests.push_str("            for payload in messages:\n");
            tests.push_str("                sent_message = json.loads(payload)\n");
            tests.push_str("                await ws.send(json.dumps(sent_message))\n");
            tests.push_str("                response_str = await ws.recv()\n");
            tests.push_str("                response = json.loads(response_str)\n");
            if let Some(ref literal) = reply_literal {
                tests.push_str(&format!("                expected = {}\n", literal));
                tests.push_str("                assert response == expected\n");
            } else {
                tests.push_str("                assert response.get(\"validated\") is True\n");
                tests.push_str("                for key, value in sent_message.items():\n");
                tests.push_str("                    assert response.get(key) == value\n");
            }
            tests.push_str("                \n");
        }

        tests.push('\n');
    }

    let module = format!(
        "\"\"\"AsyncAPI WebSocket tests.\"\"\"\n\nimport json\nfrom pathlib import Path\n\nfrom spikard.testing import TestClient\n\nROOT_DIR = Path(__file__).resolve().parents[3]\nWEBSOCKET_FIXTURE_ROOT = ROOT_DIR / \"testing_data\" / \"websockets\"\n\n\ndef load_async_fixture(root: Path, name: str) -> dict:\n    fixture_path = root / f\"{{name}}.json\"\n    with fixture_path.open() as handle:\n        return json.load(handle)\n\n\ndef load_fixture_examples(root: Path, name: str) -> list[str]:\n    data = load_async_fixture(root, name)\n    examples = data.get(\"examples\", [])\n    if not isinstance(examples, list) or not examples:\n        return [json.dumps({{}})]\n    return [json.dumps(example) for example in examples]\n\n{imports}{tests}",
        imports = imports,
        tests = tests
    );

    Ok(module)
}

fn websocket_reply_literal(
    fixture: &AsyncFixture,
    lookup: &std::collections::HashMap<&str, &AsyncFixture>,
) -> Result<Option<String>> {
    let mut replies: Vec<serde_json::Value> = Vec::new();
    for op in fixture
        .operations
        .iter()
        .filter(|op| op.action.eq_ignore_ascii_case("receive"))
    {
        for reply in &op.replies {
            if let Some(reply_fixture) = lookup.get(reply.as_str())
                && let Some(example) = reply_fixture.examples.first()
            {
                replies.push(example.clone());
            }
        }
    }

    if replies.is_empty() {
        return Ok(None);
    }

    if replies.len() == 1 {
        Ok(Some(json_to_python(&replies[0])))
    } else {
        let literal = replies.iter().map(json_to_python).collect::<Vec<_>>().join(", ");
        Ok(Some(format!("[{}]", literal)))
    }
}

/// Generate JSON-RPC tests from fixtures
fn generate_jsonrpc_tests(fixtures: &[JsonRpcFixture], output_dir: &Path) -> Result<()> {
    if fixtures.is_empty() {
        return Ok(());
    }

    let test_file = output_dir.join("tests").join("test_jsonrpc.py");
    let mut code = String::new();

    code.push_str("\"\"\"JSON-RPC 2.0 e2e tests generated from fixtures.\"\"\"\n\n");
    code.push_str("import pytest\n");
    code.push_str("from spikard.testing import TestClient\n");
    code.push_str("from app.main import *\n\n");

    for fixture in fixtures {
        let factory_name = sanitize_identifier(&fixture.name);
        let method_name = &fixture.method;

        for (idx, example) in fixture.examples.iter().enumerate() {
            code.push_str("@pytest.mark.asyncio\n");
            code.push_str(&format!("async def test_{}_success_{}():\n", factory_name, idx + 1));
            code.push_str(&format!("    \"\"\"Test {}.\"\"\" \n", method_name));
            code.push_str(&format!("    app = create_app_{}()\n", factory_name));
            code.push_str("    async with TestClient(app) as client:\n");
            let endpoint = fixture.endpoint.as_deref().unwrap_or("/rpc");
            code.push_str(&format!(
                "        response = await client.post(\"{}\", json={{\n",
                endpoint
            ));
            code.push_str("            \"jsonrpc\": \"2.0\",\n");
            code.push_str(&format!("            \"method\": \"{}\",\n", method_name));
            let params_json = serde_json::to_string(&example.params)?;
            let params_py = json_to_python_literal(&params_json);
            code.push_str(&format!("            \"params\": {},\n", params_py));
            code.push_str("            \"id\": 1,\n");
            code.push_str("        })\n");
            code.push_str("        assert response.status_code == 200\n");
            code.push_str("        data = response.json()\n");
            code.push_str("        assert data[\"jsonrpc\"] == \"2.0\"\n");
            code.push_str("        assert \"result\" in data\n");
            code.push_str("        assert data[\"id\"] == 1\n");
            code.push_str("        # Result should match expected structure\n");
            code.push_str("        result = data[\"result\"]\n");
            code.push_str("        assert isinstance(result, dict)\n\n");
        }

        for error_case in &fixture.error_cases {
            let error_test_name = sanitize_identifier(&error_case.name);
            code.push_str("@pytest.mark.asyncio\n");
            code.push_str(&format!(
                "async def test_{}_{}_error():\n",
                factory_name, error_test_name
            ));
            code.push_str(&format!(
                "    \"\"\"Test {} - {} error case.\"\"\" \n",
                method_name, error_test_name
            ));
            code.push_str(&format!("    app = create_app_{}()\n", factory_name));
            code.push_str("    async with TestClient(app) as client:\n");
            let endpoint = fixture.endpoint.as_deref().unwrap_or("/rpc");
            code.push_str(&format!(
                "        response = await client.post(\"{}\", json={{\n",
                endpoint
            ));
            code.push_str("            \"jsonrpc\": \"2.0\",\n");
            code.push_str(&format!("            \"method\": \"{}\",\n", method_name));
            if let Some(params) = &error_case.params {
                let params_json = serde_json::to_string(params)?;
                let params_py = json_to_python_literal(&params_json);
                code.push_str(&format!("            \"params\": {},\n", params_py));
            }
            code.push_str("            \"id\": 1,\n");
            code.push_str("        })\n");
            code.push_str("        assert response.status_code == 200\n");
            code.push_str("        data = response.json()\n");
            code.push_str("        assert data[\"jsonrpc\"] == \"2.0\"\n");
            code.push_str("        assert \"error\" in data\n");
            code.push_str(&format!(
                "        assert data[\"error\"][\"code\"] == {}\n",
                error_case.error.code
            ));
            code.push_str("        assert \"message\" in data[\"error\"]\n");
            code.push_str("        assert data[\"id\"] == 1\n\n");
        }

        if fixture.examples.len() > 1 {
            code.push_str("@pytest.mark.asyncio\n");
            code.push_str(&format!("async def test_{}_batch_request():\n", factory_name));
            code.push_str(&format!("    \"\"\"Test {} - batch request.\"\"\" \n", method_name));
            code.push_str(&format!("    app = create_app_{}()\n", factory_name));
            code.push_str("    async with TestClient(app) as client:\n");
            code.push_str("        batch_request = [\n");
            for (idx, example) in fixture.examples.iter().take(2).enumerate() {
                code.push_str("            {\n");
                code.push_str("                \"jsonrpc\": \"2.0\",\n");
                code.push_str(&format!("                \"method\": \"{}\",\n", method_name));
                let params_json = serde_json::to_string(&example.params)?;
                let params_py = json_to_python_literal(&params_json);
                code.push_str(&format!("                \"params\": {},\n", params_py));
                code.push_str(&format!("                \"id\": {},\n", idx + 1));
                code.push_str("            },\n");
            }
            code.push_str("        ]\n");
            let endpoint = fixture.endpoint.as_deref().unwrap_or("/rpc");
            code.push_str(&format!(
                "        response = await client.post(\"{}\", json=batch_request)\n",
                endpoint
            ));
            code.push_str("        assert response.status_code == 200\n");
            code.push_str("        # Batch requests return array of responses\n");
            code.push_str("        responses = response.json()\n");
            code.push_str("        assert isinstance(responses, list)\n");
            code.push_str("        assert len(responses) >= 1\n\n");
        }
    }

    fs::create_dir_all(test_file.parent().unwrap())?;
    fs::write(&test_file, code)?;
    Ok(())
}

/// Convert JSON string to Python literal
fn json_to_python_literal(json_str: &str) -> String {
    json_str
        .replace("true", "True")
        .replace("false", "False")
        .replace("null", "None")
}

/// Generate Python test module for GraphQL fixtures
fn generate_graphql_test_module(fixtures: &[GraphQLFixture]) -> Result<String> {
    let mut code = String::new();

    code.push_str("\"\"\"E2E tests for GraphQL operations.\"\"\"\n\n");
    code.push_str("import pytest\n");
    code.push_str("from spikard.testing import TestClient\n");
    code.push_str("import app.main as app_main\n\n");

    // Generate test functions
    for fixture in fixtures {
        let fixture_id = sanitize_identifier(&fixture.name);
        let test_name = format!("test_graphql_{}", fixture_id);
        let factory_name = format!("create_app_graphql_{}", fixture_id);

        code.push_str("@pytest.mark.asyncio\n");
        code.push_str(&format!("async def {}() -> None:\n", test_name));
        let desc: &str = fixture
            .description
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(&fixture.name);
        code.push_str(&format!("    \"\"\"{}.\"\"\"\n", desc));
        code.push('\n');

        code.push_str(&format!(
            "    app_factory = getattr(app_main, \"{}\", None)\n",
            factory_name
        ));
        code.push_str("    if app_factory is None:\n");
        code.push_str(&format!(
            "        pytest.skip(\"Missing generated app factory: {}\")\n",
            factory_name
        ));
        code.push_str("    async with TestClient(app_factory()) as client:\n");

        // Use request from fixture
        let request = &fixture.request;
        {
            // Escape GraphQL query for Python string (optional for persisted queries)
            code.push_str("        response = await client.graphql(\n");
            let query_str = request.query.as_deref().unwrap_or("");
            code.push_str(&format!(
                "            query=\"{}\",\n",
                query_str
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t")
            ));

            if let Some(variables) = &request.variables {
                code.push_str(&format!("            variables={},\n", json_to_python(variables)));
            } else {
                code.push_str("            variables=None,\n");
            }

            if let Some(op_name) = &request.operation_name {
                code.push_str(&format!("            operation_name=\"{}\",\n", op_name));
            } else {
                code.push_str("            operation_name=None,\n");
            }

            code.push_str(&format!("            path=\"{}\"\n", fixture.endpoint));
            code.push_str("        )\n\n");
        }

        // Get expected response
        let expected_response = &fixture.expected_response;
        {
            // Assert status code
            code.push_str(&format!(
                "        assert response.status_code == {}\n",
                expected_response.status_code
            ));

            // Handle successful responses with data
            if let Some(ref expected_data) = expected_response.data {
                code.push_str("        response_data = response.json()\n");
                code.push_str("        assert \"data\" in response_data\n");

                generate_graphql_data_assertions(&mut code, expected_data, "response_data[\"data\"]");
            }
        }

        if let Some(ref errors) = expected_response.errors {
            // Handle error responses
            code.push_str("        response_data = response.json()\n");
            code.push_str("        assert \"errors\" in response_data\n");
            code.push_str(&format!(
                "        assert len(response_data[\"errors\"]) == {}\n",
                errors.len()
            ));

            for (idx, error) in errors.iter().enumerate() {
                code.push_str(&format!("        error_{} = response_data[\"errors\"][{}]\n", idx, idx));
                code.push_str(&format!(
                    "        assert error_{}[\"message\"] == \"{}\"\n",
                    idx,
                    error.message.replace('"', "\\\"")
                ));

                if let Some(ref path) = error.path {
                    if !path.is_empty() {
                        let path_literal = json_to_python(&serde_json::Value::Array(path.clone()));
                        code.push_str(&format!("        assert error_{}[\"path\"] == {}\n", idx, path_literal));
                    }
                }

                if let Some(ref locations) = error.locations {
                    if !locations.is_empty() {
                        code.push_str(&format!("        assert \"locations\" in error_{}\n", idx));
                        code.push_str(&format!("        assert len(error_{}[\"locations\"]) >= 1\n", idx));
                    }
                }
            }
        } else {
            // No errors expected
            code.push_str("        response_data = response.json()\n");
            code.push_str(
                "        assert response_data.get(\"errors\") is None or response_data.get(\"errors\") == []\n",
            );
        }

        code.push_str("\n\n");
    }

    Ok(code)
}

/// Generate assertions for GraphQL response data
fn generate_graphql_data_assertions(code: &mut String, value: &serde_json::Value, path: &str) {
    match value {
        serde_json::Value::Object(obj) => {
            for (key, val) in obj {
                code.push_str(&format!("        assert \"{}\" in {}\n", key, path));
                let new_path = format!("{}[\"{}\"]", path, key);

                match val {
                    serde_json::Value::Object(_) => {
                        generate_graphql_data_assertions(code, val, &new_path);
                    }
                    serde_json::Value::Array(_) => {
                        generate_graphql_data_assertions(code, val, &new_path);
                    }
                    _ => {
                        code.push_str(&format!("        assert {} == {}\n", new_path, json_to_python(val)));
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            code.push_str(&format!("        assert len({}) == {}\n", path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_graphql_data_assertions(code, item, &new_path);
            }
        }
        _ => {
            code.push_str(&format!("        assert {} == {}\n", path, json_to_python(value)));
        }
    }
}

/// Generate pytest test for a gRPC fixture
///
/// Creates a pytest async test function that:
/// - Imports the generated handler
/// - Creates a GrpcRequest from the fixture
/// - Calls the handler with the request
/// - Asserts the response matches expected values
/// - Validates metadata and status code
///
/// # Arguments
///
/// * `fixture` - The gRPC fixture containing request/response definitions
///
/// # Returns
///
/// A pytest async test function as a string
pub fn generate_grpc_test(fixture: &GrpcFixture) -> Result<String> {
    let mut code = String::new();

    let test_name = sanitize_identifier(&fixture.name);
    let handler_name = format!("handle_grpc_{}", test_name);

    code.push_str("import pytest\n");
    code.push_str("from spikard.grpc import GrpcRequest\n\n");

    // Test function
    code.push_str("@pytest.mark.asyncio\n");
    code.push_str(&format!("async def test_grpc_{}() -> None:\n", test_name));
    code.push_str(&format!(
        "    \"\"\"{}.\"\"\"\n",
        fixture.description.as_deref().unwrap_or(&fixture.name)
    ));
    code.push('\n');

    // Import handler (note: this would normally be imported at module level)
    code.push_str(&format!("    from app.main import {}\n", handler_name));
    code.push('\n');

    // Build request
    code.push_str("    # Build gRPC request from fixture\n");

    // Add metadata handling
    if let Some(ref metadata) = fixture.request.metadata {
        if !metadata.is_empty() {
            code.push_str("    metadata: dict[str, str] = {\n");
            for (key, value) in metadata {
                let escaped_value = value
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t");
                code.push_str(&format!("        \"{}\": \"{}\",\n", key, escaped_value));
            }
            code.push_str("    }\n");
        } else {
            code.push_str("    metadata: dict[str, str] = {}\n");
        }
    } else {
        code.push_str("    metadata: dict[str, str] = {}\n");
    }

    // Add request payload
    code.push_str("    request_payload: bytes = b\"{}\"\n");
    code.push_str("    request = GrpcRequest(\n");
    code.push_str(&format!("        service_name=\"{}\",\n", fixture.handler.service));
    code.push_str(&format!("        method_name=\"{}\",\n", fixture.handler.method));
    code.push_str("        payload=request_payload,\n");
    code.push_str("        metadata=metadata,\n");
    code.push_str("    )\n\n");

    // Call handler
    code.push_str("    # Call handler\n");
    code.push_str(&format!("    response = await {}(request)\n", handler_name));
    code.push('\n');

    // Assert response
    code.push_str("    # Verify response\n");

    // Assert payload if present
    if let Some(ref expected_msg) = fixture.expected_response.message {
        let expected_json = serde_json::to_string(expected_msg).context("Failed to serialize expected response")?;
        let expected_literal = python_bytes_literal(expected_json.as_bytes());
        code.push_str(&format!("    assert response.payload == {}\n", expected_literal));
    } else if let Some(ref expected_stream) = fixture.expected_response.stream {
        let expected_json = serde_json::to_string(expected_stream).context("Failed to serialize expected response")?;
        let expected_literal = python_bytes_literal(expected_json.as_bytes());
        code.push_str(&format!("    assert response.payload == {}\n", expected_literal));
    }

    // Assert metadata if checking for presence
    if let Some(ref metadata) = fixture.request.metadata {
        if !metadata.is_empty() {
            code.push_str("    assert response.metadata is not None\n");
        }
    }

    Ok(code)
}
