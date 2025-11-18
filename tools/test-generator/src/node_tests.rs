//! Node.js test generator
//!
//! Generates vitest test suites from fixtures for e2e testing.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::background_data;
use crate::middleware::parse_middleware;
use crate::streaming::streaming_data;
use crate::ts_target::TypeScriptTarget;
use anyhow::{Context, Result, ensure};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use spikard_cli::codegen::ts_schema::{TypeScriptDto, generate_typescript_dto};
use spikard_codegen::openapi::from_fixtures::FixtureFile;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const MAX_SAFE_INTEGER: i128 = 9007199254740991; // 2^53 - 1

/// Generate Node.js test suite from fixtures
pub fn generate_node_tests(fixtures_dir: &Path, output_dir: &Path, target: &TypeScriptTarget) -> Result<()> {
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

    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;

    let mut dto_map: HashMap<String, TypeScriptDto> = HashMap::new();
    for fixture in sse_fixtures.iter().chain(websocket_fixtures.iter()) {
        if dto_map.contains_key(&fixture.name) {
            continue;
        }
        let dto = generate_typescript_dto(&fixture.name, &fixture.schema)
            .with_context(|| format!("Failed to build DTO for {}", fixture.name))?;
        dto_map.insert(fixture.name.clone(), dto);
    }

    // Generate test file for each category
    for (category, fixtures) in fixtures_by_category.iter() {
        if category == "background" {
            continue;
        }
        let test_content = generate_test_file(category, fixtures, target)?;
        let test_file = tests_dir.join(format!("{}.test.ts", category));
        fs::write(&test_file, test_content).with_context(|| format!("Failed to write test file for {}", category))?;
        println!("  ✓ Generated tests/{}.test.ts ({} tests)", category, fixtures.len());
    }

    if !sse_fixtures.is_empty() {
        let sse_content = generate_sse_test_file(&sse_fixtures, &dto_map, target)?;
        fs::write(tests_dir.join("asyncapi_sse.test.ts"), sse_content)
            .context("Failed to write asyncapi_sse.test.ts")?;
        println!("  ✓ Generated tests/asyncapi_sse.test.ts");
    }

    if !websocket_fixtures.is_empty() {
        let websocket_content = generate_websocket_test_file(&websocket_fixtures, &dto_map, target)?;
        fs::write(tests_dir.join("asyncapi_websocket.test.ts"), websocket_content)
            .context("Failed to write asyncapi_websocket.test.ts")?;
        println!("  ✓ Generated tests/asyncapi_websocket.test.ts");
    }

    format_generated_ts(output_dir)?;

    Ok(())
}

/// Generate test file for a category
fn generate_test_file(category: &str, fixtures: &[Fixture], target: &TypeScriptTarget) -> Result<String> {
    let mut code = String::new();

    // File header
    code.push_str(&format!("/**\n * E2E tests for {}\n * @generated\n */\n\n", category));
    code.push_str(&format!(
        "import {{ TestClient }} from \"{}\";\n",
        target.binding_package
    ));
    code.push_str("import { describe, expect, test } from \"vitest\";\n");

    // Import all app factories for this category
    let mut app_factories = Vec::new();
    for fixture in fixtures {
        let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
        let app_factory_name = format!("createApp{}", to_pascal_case(&fixture_id));
        app_factories.push(app_factory_name.clone());
    }
    app_factories.sort();
    app_factories.dedup();
    if app_factories.len() <= 4 && app_factories.join(", ").len() <= 120 {
        code.push_str(&format!(
            "import {{ {} }} from \"../app/main.js\";\n\n",
            app_factories.join(", ")
        ));
    } else {
        code.push_str("import {\n");
        for factory in &app_factories {
            code.push_str(&format!("\t{},\n", factory));
        }
        code.push_str("} from \"../app/main.js\";\n\n");
    }

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

fn generate_sse_test_file(
    fixtures: &[AsyncFixture],
    dto_map: &HashMap<String, TypeScriptDto>,
    target: &TypeScriptTarget,
) -> Result<String> {
    use std::collections::{BTreeMap, BTreeSet};

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut factory_imports = BTreeSet::new();
    let mut schema_imports = BTreeSet::new();
    let mut test_blocks = String::new();

    for (channel, channel_fixtures) in &grouped {
        let channel_path = if channel.starts_with('/') {
            channel.clone()
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("createApp{}", to_pascal_case(&format!("sse_{}", slug)));
        factory_imports.insert(factory_name.clone());

        let mut fixture_entries = Vec::new();
        for fixture in channel_fixtures {
            if let Some(dto) = dto_map.get(&fixture.name) {
                schema_imports.insert(dto.schema_ident.clone());
                fixture_entries.push((fixture.name.clone(), dto.schema_ident.clone()));
            }
        }

        if fixture_entries.is_empty() {
            continue;
        }

        test_blocks.push_str(&format!("\ttest(\"SSE {path}\", async () => {{\n", path = channel_path));
        test_blocks.push_str(&format!("\t\tconst app = {factory_name}();\n"));
        test_blocks.push_str("\t\tconst client = new TestClient(app);\n");
        test_blocks.push_str(&format!(
            "\t\tconst response = await client.get(\"{path}\");\n",
            path = channel_path
        ));
        test_blocks.push_str("\t\texpect(response.statusCode).toBe(200);\n");
        test_blocks.push_str("\t\tconst normalized = response.text().replace(/\\r\\n/g, \"\\n\");\n");
        test_blocks.push_str("\t\tconst events = normalized\n");
        test_blocks.push_str("\t\t\t.split(\"\\n\\n\")\n");
        test_blocks.push_str("\t\t\t.filter((chunk) => chunk.startsWith(\"data:\"))\n");
        test_blocks.push_str("\t\t\t.map((chunk) => chunk.slice(5).trim());\n");
        test_blocks.push_str("\t\tconst fixtures = [\n");
        for (name, schema_ident) in &fixture_entries {
            test_blocks.push_str(&format!(
                "\t\t\t{{ name: \"{name}\", schema: {schema} }},\n",
                name = name,
                schema = schema_ident
            ));
        }
        test_blocks.push_str("\t\t];\n");
        test_blocks.push_str("\t\tconst expected = fixtures.flatMap(({ name, schema }) =>\n");
        test_blocks.push_str("\t\t\tloadFixtureExamples(name).map((payload) => schema.parse(JSON.parse(payload))),\n");
        test_blocks.push_str("\t\t);\n");
        test_blocks.push_str("\t\texpect(events.length).toBe(expected.length);\n");
        test_blocks.push_str("\t\tevents.forEach((payload, index) => {\n");
        test_blocks.push_str("\t\t\texpect(JSON.parse(payload)).toEqual(expected[index]);\n");
        test_blocks.push_str("\t\t});\n");
        test_blocks.push_str("\t});\n");
    }

    let mut file_content = String::new();
    file_content.push_str("/**\n * AsyncAPI SSE tests\n * @generated\n */\n\n");
    file_content.push_str(&format!(
        "import {{ TestClient }} from \"{}\";\n",
        target.binding_package
    ));
    file_content.push_str("import { describe, expect, test } from \"vitest\";\n");
    file_content.push_str("import { readFileSync } from \"node:fs\";\n");
    file_content.push_str("import path from \"node:path\";\n");
    file_content.push_str("import {\n");
    for name in factory_imports.iter().chain(schema_imports.iter()) {
        file_content.push_str(&format!("\t{},\n", name));
    }
    file_content.push_str("} from \"../app/main.js\";\n\n");
    file_content.push_str("const ROOT_DIR = path.resolve(__dirname, \"../../..\");\n");
    file_content.push_str("const SSE_FIXTURE_ROOT = path.join(ROOT_DIR, \"testing_data\", \"sse\");\n\n");
    file_content.push_str("function loadFixtureExamples(name: string): string[] {\n");
    file_content.push_str("\tconst fixturePath = path.join(SSE_FIXTURE_ROOT, `${name}.json`);\n");
    file_content.push_str("\tconst data = JSON.parse(readFileSync(fixturePath, \"utf-8\"));\n");
    file_content.push_str("\tconst examples = Array.isArray(data.examples) ? data.examples : [];\n");
    file_content.push_str("\tif (examples.length === 0) {\n");
    file_content.push_str("\t\treturn [JSON.stringify({})];\n");
    file_content.push_str("\t}\n");
    file_content.push_str("\treturn examples.map((example) => JSON.stringify(example));\n");
    file_content.push_str("}\n\n");
    file_content.push_str("describe(\"asyncapi_sse\", () => {\n");
    file_content.push_str(&test_blocks);
    file_content.push_str("});\n");

    Ok(file_content)
}

fn generate_websocket_test_file(
    fixtures: &[AsyncFixture],
    dto_map: &HashMap<String, TypeScriptDto>,
    target: &TypeScriptTarget,
) -> Result<String> {
    use std::collections::{BTreeMap, BTreeSet};

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    if grouped.is_empty() {
        return Ok(String::new());
    }

    let mut factory_imports = BTreeSet::new();
    let mut schema_imports = BTreeSet::new();
    let mut test_blocks = String::new();

    for (channel, channel_fixtures) in &grouped {
        let channel_path = if channel.starts_with('/') {
            channel.clone()
        } else {
            format!("/{}", channel)
        };
        let slug = sanitize_identifier(&channel_path.trim_start_matches('/').replace('/', "_"));
        let factory_name = format!("createApp{}", to_pascal_case(&format!("websocket_{}", slug)));
        factory_imports.insert(factory_name.clone());

        let mut fixture_entries = Vec::new();
        for fixture in channel_fixtures {
            if let Some(dto) = dto_map.get(&fixture.name) {
                schema_imports.insert(dto.schema_ident.clone());
                fixture_entries.push((fixture.name.clone(), dto.schema_ident.clone()));
            }
        }

        if fixture_entries.is_empty() {
            continue;
        }

        test_blocks.push_str(&format!(
            "\ttest(\"WebSocket {path}\", async () => {{\n",
            path = channel_path
        ));
        test_blocks.push_str(&format!("\t\tconst app = {factory_name}();\n"));
        test_blocks.push_str("\t\tconst client = new TestClient(app);\n");
        test_blocks.push_str(&format!(
            "\t\tconst ws = await client.websocketConnect(\"{path}\");\n",
            path = channel_path
        ));
        test_blocks.push_str("\t\tconst fixtures = [\n");
        for (name, schema_ident) in &fixture_entries {
            test_blocks.push_str(&format!(
                "\t\t\t{{ name: \"{name}\", schema: {schema} }},\n",
                name = name,
                schema = schema_ident
            ));
        }
        test_blocks.push_str("\t\t];\n");
        test_blocks.push_str("\t\tfor (const { name, schema } of fixtures) {\n");
        test_blocks
            .push_str("\t\t\tconst payload = schema.parse(JSON.parse(loadFixtureExamples(name)[0] ?? \"{}\"));\n");
        test_blocks.push_str("\t\t\tawait ws.sendJson(payload);\n");
        test_blocks.push_str("\t\t\tconst response = await ws.receiveJson();\n");
        test_blocks.push_str("\t\t\texpect(response.validated).toBe(true);\n");
        test_blocks.push_str("\t\t\tfor (const [key, value] of Object.entries(payload)) {\n");
        test_blocks.push_str("\t\t\t\texpect(response[key]).toEqual(value);\n");
        test_blocks.push_str("\t\t\t}\n");
        test_blocks.push_str("\t\t}\n");
        test_blocks.push_str("\t\tawait ws.close();\n");
        test_blocks.push_str("\t});\n");
    }

    let mut file_content = String::new();
    file_content.push_str("/**\n * AsyncAPI WebSocket tests\n * @generated\n */\n\n");
    file_content.push_str(&format!(
        "import {{ TestClient }} from \"{}\";\n",
        target.binding_package
    ));
    file_content.push_str("import { describe, expect, test } from \"vitest\";\n");
    file_content.push_str("import { readFileSync } from \"node:fs\";\n");
    file_content.push_str("import path from \"node:path\";\n");
    file_content.push_str("import {\n");
    for name in factory_imports.iter().chain(schema_imports.iter()) {
        file_content.push_str(&format!("\t{},\n", name));
    }
    file_content.push_str("} from \"../app/main.js\";\n\n");
    file_content.push_str("const ROOT_DIR = path.resolve(__dirname, \"../../..\");\n");
    file_content.push_str("const WEBSOCKET_FIXTURE_ROOT = path.join(ROOT_DIR, \"testing_data\", \"websockets\");\n\n");
    file_content.push_str("function loadFixtureExamples(name: string): string[] {\n");
    file_content.push_str("\tconst fixturePath = path.join(WEBSOCKET_FIXTURE_ROOT, `${name}.json`);\n");
    file_content.push_str("\tconst data = JSON.parse(readFileSync(fixturePath, \"utf-8\"));\n");
    file_content.push_str("\tconst examples = Array.isArray(data.examples) ? data.examples : [];\n");
    file_content.push_str("\tif (examples.length === 0) {\n");
    file_content.push_str("\t\treturn [JSON.stringify({})];\n");
    file_content.push_str("\t}\n");
    file_content.push_str("\treturn examples.map((example) => JSON.stringify(example));\n");
    file_content.push_str("}\n\n");
    file_content.push_str("describe(\"asyncapi_websocket\", () => {\n");
    file_content.push_str(&test_blocks);
    file_content.push_str("});\n");

    Ok(file_content)
}

/// Generate a single test function
fn generate_test_function(category: &str, fixture: &Fixture) -> Result<String> {
    let test_name = sanitize_test_name(&fixture.name);
    let mut code = String::new();
    let streaming_info = streaming_data(fixture)?;
    let background_info = background_data(fixture)?;
    let middleware = parse_middleware(fixture)?;
    if fixture_should_skip(category, fixture) {
        code.push_str(&format!("\ttest.skip(\"{}\", async () => {{\n", test_name));
        code.push_str("\t\t// Not supported by the in-memory HTTP client\n");
        code.push_str("\t});\n");
        return Ok(code);
    }
    let expects_binary_body = fixture_requires_binary_body(fixture);
    let expected_content_length = expected_content_length(fixture);

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
    let mut option_fields: Vec<&str> = Vec::new();

    let mut header_entries: Vec<(String, String)> = Vec::new();
    if let Some(ref headers) = fixture.request.headers {
        for (key, value) in headers {
            header_entries.push((key.clone(), value.clone()));
        }
    }

    if let Some(cookies) = fixture.request.cookies.as_ref().filter(|c| !c.is_empty()) {
        let cookie_value = cookies
            .iter()
            .map(|(name, value)| format!("{}={}", name, escape_string(value)))
            .collect::<Vec<_>>()
            .join("; ");
        header_entries.push(("Cookie".to_string(), cookie_value));
    }

    if !header_entries.is_empty() {
        code.push_str("\t\tconst headers = {\n");
        for (key, value) in header_entries {
            let escaped_value = escape_string(&value);
            code.push_str(&format!(
                "\t\t\t{}: \"{}\",\n",
                format_ts_property_key(&key),
                escaped_value
            ));
        }
        code.push_str("\t\t};\n");
        option_fields.push("headers");
    }

    let content_type = fixture.request.content_type.clone().or_else(|| {
        fixture
            .request
            .headers
            .as_ref()
            .and_then(|h| h.get("Content-Type").cloned())
    });

    let content_type_lc = content_type.as_ref().map(|s| s.to_ascii_lowercase());
    // Check if fixture has files field (even if empty - needed for optional file params)
    let has_files_field = fixture.request.files.is_some();
    let has_files = fixture
        .request
        .files
        .as_ref()
        .map(|files| !files.is_empty())
        .unwrap_or(false);

    let is_multipart = content_type_lc
        .as_ref()
        .map(|ct| ct.contains("multipart/form-data"))
        .unwrap_or(false)
        || has_files
        || has_files_field; // Include even empty files array for optional params

    let has_form_data_field = fixture.request.form_data.is_some();
    let has_form_data = fixture
        .request
        .form_data
        .as_ref()
        .map(|data| !data.is_empty())
        .unwrap_or(false);

    let is_form = content_type_lc
        .as_ref()
        .map(|ct| ct.contains("application/x-www-form-urlencoded"))
        .unwrap_or(false)
        || (!is_multipart && (has_form_data || has_form_data_field));

    if is_multipart {
        if let Some(definition) = build_multipart_definition(fixture)? {
            code.push_str(&definition);
            option_fields.push("multipart");
        }
    } else if is_form {
        if let Some(definition) = build_form_definition(fixture)? {
            code.push_str(&definition);
            option_fields.push("form");
        }
    } else if let Some(ref body) = fixture.request.body {
        let json_literal = json_to_typescript(body);
        code.push_str(&format!("\t\tconst json = {};\n", json_literal));
        option_fields.push("json");
    }

    // Build query params string if present
    let path_with_query = if let Some(ref query_params) = fixture.request.query_params {
        if !query_params.is_empty() {
            build_path_with_query(path, query_params)
        } else {
            path.clone()
        }
    } else {
        path.clone()
    };

    let request_call = build_request_call(&method, &path_with_query, &option_fields);

    if let Some(rate_limit) = middleware.rate_limit.as_ref()
        && rate_limit.warmup_requests > 0
    {
        code.push_str(&format!(
            "\t\tfor (let i = 0; i < {}; i += 1) {{\n",
            rate_limit.warmup_requests
        ));
        code.push_str(&format!("\t\t\tconst warmupResponse = {};\n", request_call));
        let warmup_status = rate_limit.warmup_expect_status.unwrap_or(200);
        code.push_str(&format!(
            "\t\t\texpect(warmupResponse.statusCode).toBe({});\n",
            warmup_status
        ));
        if let Some(delay) = rate_limit.sleep_ms_between {
            code.push_str(&format!(
                "\t\t\tawait new Promise((resolve) => setTimeout(resolve, {}));\n",
                delay
            ));
        }
        code.push_str("\t\t}\n\n");
    }

    code.push_str(&format!("\t\tconst response = {};\n\n", request_call));

    if let Some(stream_info) = streaming_info.as_ref() {
        let expected_base64 = BASE64_STANDARD.encode(&stream_info.expected_bytes);
        code.push_str(&format!(
            "\t\tconst expected = Buffer.from(\"{}\", \"base64\");\n",
            expected_base64
        ));
        code.push_str("\t\texpect(response.bytes()).toStrictEqual(expected);\n");
        if stream_info.is_text_only {
            code.push_str("\t\texpect(response.text()).toBe(expected.toString());\n");
        }
        code.push_str("\t});\n");
        return Ok(code);
    }

    // Assert status code
    code.push_str(&format!(
        "\t\texpect(response.statusCode).toBe({});\n",
        fixture.expected_response.status_code
    ));

    let status_code = fixture.expected_response.status_code;

    if let Some(bg) = background_info.as_ref() {
        code.push_str(&format!(
            "\t\tconst stateResponse = await client.get(\"{}\");\n",
            bg.state_path
        ));
        code.push_str("\t\texpect(stateResponse.statusCode).toBe(200);\n");
        let expected_body =
            serde_json::json!({ bg.state_key.clone(): serde_json::Value::Array(bg.expected_state.clone()) });
        code.push_str(&format!(
            "\t\texpect(stateResponse.json()).toStrictEqual({});\n",
            json_to_typescript(&expected_body)
        ));
        code.push_str("\t});\n");
        return Ok(code);
    }

    // Different assertion strategies based on status code
    if status_code == 200 {
        if expects_binary_body {
            if let Some(target_len) = expected_content_length {
                code.push_str("\t\tconst bodyBytes = response.bytes();\n");
                code.push_str(&format!("\t\texpect(bodyBytes.length).toBe({});\n", target_len));
                if let Some(body) = fixture.expected_response.body.as_ref() {
                    code.push_str(&format!(
                        "\t\texpect(bodyBytes.toString(\"utf-8\").startsWith({})).toBe(true);\n",
                        json_to_typescript(body)
                    ));
                }
            }
        } else {
            // Success case - verify response matches expected
            let has_expected_body = fixture
                .expected_response
                .body
                .as_ref()
                .map(|body| !is_value_effectively_empty(body))
                .unwrap_or(false);
            let has_request_body = fixture
                .request
                .body
                .as_ref()
                .map(|body| !is_value_effectively_empty(body))
                .unwrap_or(false);
            let has_form_data = fixture
                .request
                .form_data
                .as_ref()
                .map(|data| !data.is_empty())
                .unwrap_or(false);
            let has_data_entries = fixture
                .request
                .data
                .as_ref()
                .map(|data| !data.is_empty())
                .unwrap_or(false);
            let has_query_params = fixture
                .request
                .query_params
                .as_ref()
                .map(|params| !params.is_empty())
                .unwrap_or(false);
            let requires_response_data =
                has_expected_body || has_request_body || has_form_data || has_data_entries || has_query_params;

            let needs_response_text = category == "static_files"
                && fixture
                    .expected_response
                    .body
                    .as_ref()
                    .map(|body| body.is_string())
                    .unwrap_or(false);
            let should_parse_json =
                !needs_response_text && (method != "head" || fixture.expected_response.body.is_some());

            if requires_response_data && should_parse_json {
                code.push_str("\t\tconst responseData = response.json();\n");
            } else if needs_response_text {
                code.push_str("\t\tconst responseText = response.text();\n");
            }

            // If fixture has expected response body, assert against that
            if let Some(ref expected_body) = fixture.expected_response.body {
                let expected_body_is_empty = is_value_effectively_empty(expected_body);
                if !expected_body_is_empty {
                    if needs_response_text {
                        code.push_str(&format!(
                            "\t\texpect(responseText).toBe({});\n",
                            json_to_typescript(expected_body)
                        ));
                    } else {
                        generate_body_assertions(&mut code, expected_body, "responseData", 2, status_code >= 400);
                    }
                } else if requires_response_data && should_parse_json {
                    // Fallback: verify echoed parameters match what we sent
                    if let Some(ref body) = fixture.request.body {
                        generate_echo_assertions(&mut code, body, "responseData", 2);
                    }

                    if let Some(ref form_data) = fixture.request.form_data {
                        for (key, value) in form_data {
                            let value_path = format_property_access("responseData", key);
                            code.push_str(&format!(
                                "\t\texpect({}).toBe({});\n",
                                value_path,
                                json_to_typescript(value)
                            ));
                        }
                    }

                    if let Some(ref query_params) = fixture.request.query_params {
                        for (key, value) in query_params {
                            let value_path = format_property_access("responseData", key);
                            code.push_str(&format!(
                                "\t\texpect({}).toBe({});\n",
                                value_path,
                                json_to_typescript(value)
                            ));
                        }
                    }
                }
            } else if requires_response_data && should_parse_json {
                // Fallback: verify echoed parameters match what we sent
                if let Some(ref body) = fixture.request.body {
                    generate_echo_assertions(&mut code, body, "responseData", 2);
                }

                if let Some(ref form_data) = fixture.request.form_data {
                    for (key, value) in form_data {
                        let value_path = format_property_access("responseData", key);
                        code.push_str(&format!(
                            "\t\texpect({}).toBe({});\n",
                            value_path,
                            json_to_typescript(value)
                        ));
                    }
                }

                if let Some(ref query_params) = fixture.request.query_params {
                    for (key, value) in query_params {
                        let value_path = format_property_access("responseData", key);
                        code.push_str(&format!(
                            "\t\texpect({}).toBe({});\n",
                            value_path,
                            json_to_typescript(value)
                        ));
                    }
                }
            }
        }
    } else if status_code < 400 {
        if expects_binary_body {
            if let Some(target_len) = expected_content_length {
                code.push_str("\t\tconst bodyBytes = response.bytes();\n");
                code.push_str(&format!("\t\texpect(bodyBytes.length).toBe({});\n", target_len));
                if let Some(body) = fixture.expected_response.body.as_ref() {
                    code.push_str(&format!(
                        "\t\texpect(bodyBytes.toString(\"utf-8\").startsWith({})).toBe(true);\n",
                        json_to_typescript(body)
                    ));
                }
            }
        } else if let Some(body) = fixture
            .expected_response
            .body
            .as_ref()
            .filter(|body| !is_value_effectively_empty(body))
        {
            code.push_str("\t\tconst responseData = response.json();\n");
            generate_body_assertions(&mut code, body, "responseData", 2, false);
        }
    }

    if let Some(headers) = fixture.expected_response.headers.as_ref().filter(|map| !map.is_empty()) {
        code.push_str("\t\tconst responseHeaders = response.headers();\n");
        for (key, value) in headers.iter() {
            let lower_key = key.to_ascii_lowercase();
            let header_access = format_property_access("responseHeaders", &lower_key);
            match value.as_str() {
                "<<uuid>>" => {
                    code.push_str(&format!(
                        "\t\texpect({}).toMatch(/^[0-9a-fA-F-]{{36}}$/);\n",
                        header_access
                    ));
                }
                "<<present>>" => {
                    code.push_str(&format!("\t\texpect({}).not.toBeUndefined();\n", header_access));
                }
                "<<absent>>" => {
                    code.push_str(&format!("\t\texpect({}).toBeUndefined();\n", header_access));
                }
                _ => {
                    let escaped_value = escape_string(value);
                    code.push_str(&format!("\t\texpect({}).toBe(\"{}\");\n", header_access, escaped_value));
                }
            }
        }
    }

    code.push_str("\t});\n");

    Ok(code)
}

fn build_form_definition(fixture: &Fixture) -> Result<Option<String>> {
    if let Some(body_str) = fixture.request.body.as_ref().and_then(|value| value.as_str()) {
        let escaped = escape_string(body_str);
        return Ok(Some(format!("\t\tconst form = \"{}\";\n", escaped)));
    }

    if let Some(data) = fixture
        .request
        .form_data
        .as_ref()
        .filter(|data| !data.is_empty())
        .or_else(|| fixture.request.data.as_ref().filter(|data| !data.is_empty()))
    {
        let mut map = serde_json::Map::new();
        for (key, value) in data.iter() {
            map.insert(key.clone(), value.clone());
        }
        let form_literal = json_to_typescript(&serde_json::Value::Object(map));
        Ok(Some(format!("\t\tconst form = {};\n", form_literal)))
    } else {
        Ok(None)
    }
}

fn build_multipart_definition(fixture: &Fixture) -> Result<Option<String>> {
    // Check if fixture explicitly has files, form_data, or data fields
    // (even if they're empty - we need to send an empty multipart request for validation)
    let has_files_field = fixture.request.files.is_some();
    let has_form_data_field = fixture.request.form_data.is_some();
    let has_data_field = fixture.request.data.is_some();

    // If none of these fields are present, don't generate multipart
    if !has_files_field && !has_form_data_field && !has_data_field {
        return Ok(None);
    }

    let mut parts = Vec::new();

    let field_source = fixture
        .request
        .data
        .as_ref()
        .filter(|data| !data.is_empty())
        .or_else(|| fixture.request.form_data.as_ref().filter(|data| !data.is_empty()));

    if let Some(fields) = field_source {
        let mut map = serde_json::Map::new();
        for (key, value) in fields.iter() {
            map.insert(key.clone(), value.clone());
        }
        let fields_literal = json_to_typescript(&serde_json::Value::Object(map));
        parts.push(format!("fields: {}", fields_literal));
    }

    if let Some(files) = fixture.request.files.as_ref().filter(|files| !files.is_empty()) {
        let mut entries = Vec::new();
        for file in files {
            entries.push(format_fixture_file(file)?);
        }
        parts.push(format!("files: [{}]", entries.join(", ")));
    }

    // Always generate multipart if files or form_data fields are present
    // (even if parts is empty - this handles optional file parameters)
    if !parts.iter().any(|part| part.starts_with("files:")) {
        parts.push("files: []".to_string());
    }

    Ok(Some(format!("\t\tconst multipart = {{ {} }};\n", parts.join(", "))))
}

fn format_fixture_file(file: &FixtureFile) -> Result<String> {
    let mut props = Vec::new();
    props.push(format!("name: \"{}\"", escape_string(&file.field_name)));

    if let Some(filename) = &file.filename {
        props.push(format!("filename: \"{}\"", escape_string(filename)));
    }

    if let Some(content) = &file.content {
        props.push(format!("content: \"{}\"", escape_string(content)));
    }

    if let Some(content_type) = &file.content_type {
        props.push(format!("contentType: \"{}\"", escape_string(content_type)));
    }

    if let Some(magic_bytes) = &file.magic_bytes {
        props.push(format!("magic_bytes: \"{}\"", magic_bytes));
    }

    Ok(format!("{{ {} }}", props.join(", ")))
}

fn build_path_with_query(path: &str, params: &std::collections::HashMap<String, serde_json::Value>) -> String {
    let mut pairs = Vec::new();
    for (key, value) in params {
        append_query_value(&mut pairs, key, value);
    }
    if pairs.is_empty() {
        path.to_string()
    } else {
        let separator = if path.contains('?') {
            if path.ends_with('?') || path.ends_with('&') {
                ""
            } else {
                "&"
            }
        } else {
            "?"
        };
        format!("{}{}{}", path, separator, pairs.join("&"))
    }
}

fn append_query_value(pairs: &mut Vec<String>, key: &str, value: &serde_json::Value) {
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                append_query_value(pairs, key, item);
            }
        }
        serde_json::Value::Object(_) => {
            let json_str = value.to_string();
            pairs.push(format!("{}={}", key, urlencoding::encode(&json_str)));
        }
        _ => {
            let literal = json_to_query_literal(value);
            pairs.push(format!("{}={}", key, urlencoding::encode(&literal)));
        }
    }
}

fn json_to_query_literal(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            const MAX_SAFE: i128 = 9007199254740991; // 2^53 - 1
            if let Some(i) = n.as_i64() {
                let magnitude = i128::from(i).abs();
                if magnitude > MAX_SAFE {
                    format!("{}n", i)
                } else {
                    i.to_string()
                }
            } else if let Some(u) = n.as_u64() {
                if u as i128 > MAX_SAFE {
                    format!("{}n", u)
                } else {
                    u.to_string()
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => value.to_string(),
    }
}

/// Generate assertions for echoed parameters (success cases)
fn generate_echo_assertions(code: &mut String, sent_value: &serde_json::Value, path: &str, indent_level: usize) {
    let indent = "\t".repeat(indent_level);

    match sent_value {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_path = format_property_access(path, key);
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
fn generate_body_assertions(
    code: &mut String,
    body: &serde_json::Value,
    path: &str,
    indent_level: usize,
    skip_error_fields: bool,
) {
    let indent = "\t".repeat(indent_level);

    match body {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let in_errors_path = path_contains_segment(path, "errors");
                let skip_entire_property = (in_errors_path && (key == "ctx" || skip_error_fields))
                    || (skip_error_fields && path == "responseData" && (key == "detail" || key == "errors"));
                if skip_entire_property {
                    continue;
                }

                let new_path = format_property_access(path, key);
                code.push_str(&format!("{}expect({}).toHaveProperty(\"{}\");\n", indent, path, key));

                match value {
                    serde_json::Value::Object(_) => {
                        // Skip "ctx" objects in validation errors
                        if !(in_errors_path && key == "ctx") {
                            generate_body_assertions(code, value, &new_path, indent_level, skip_error_fields);
                        }
                    }
                    serde_json::Value::Array(_) => {
                        generate_body_assertions(code, value, &new_path, indent_level, skip_error_fields);
                    }
                    serde_json::Value::Number(n) if is_large_integer(n) => {
                        code.push_str(&format!("{}expect({}).toBe(\"{}\");\n", indent, new_path, n));
                    }
                    _ => {
                        // Skip certain fields inside validation errors
                        let skip_assertion = (in_errors_path && (key == "input" || key == "msg" || key == "type"))
                            || (skip_error_fields && path == "responseData" && key == "detail");

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
            if skip_error_fields && path_contains_segment(path, "errors") {
                return;
            }
            code.push_str(&format!("{}expect({}.length).toBe({});\n", indent, path, arr.len()));
            for (idx, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                generate_body_assertions(code, item, &new_path, indent_level, skip_error_fields);
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
        serde_json::Value::Number(n) => {
            if is_large_integer(n) {
                if let Some(i) = n.as_i64() {
                    format!("{}n", i)
                } else if let Some(u) = n.as_u64() {
                    format!("{}n", u)
                } else {
                    n.to_string()
                }
            } else {
                n.to_string()
            }
        }
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
                .map(|(k, v)| format!("{}: {}", format_ts_property_key(k), json_to_typescript(v)))
                .collect();
            format!("{{ {} }}", items.join(", "))
        }
    }
}

fn is_large_integer(number: &serde_json::Number) -> bool {
    if let Some(i) = number.as_i64() {
        i128::from(i).abs() > MAX_SAFE_INTEGER
    } else if let Some(u) = number.as_u64() {
        (u as i128) > MAX_SAFE_INTEGER
    } else {
        false
    }
}

fn is_value_effectively_empty(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null => true,
        serde_json::Value::Bool(_) => false,
        serde_json::Value::Number(_) => false,
        serde_json::Value::String(s) => s.is_empty(),
        serde_json::Value::Array(arr) => arr.is_empty(),
        serde_json::Value::Object(obj) => obj.is_empty(),
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

fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '_' || c == '$' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for ch in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            continue;
        }
        return false;
    }
    true
}

fn format_ts_property_key(key: &str) -> String {
    if is_valid_identifier(key) {
        key.to_string()
    } else {
        format!("\"{}\"", escape_string(key))
    }
}

fn format_property_access(base: &str, key: &str) -> String {
    if is_valid_identifier(key) {
        format!("{}.{}", base, key)
    } else {
        format!("{}[\"{}\"]", base, escape_string(key))
    }
}

fn path_contains_segment(path: &str, segment: &str) -> bool {
    path == segment || path.contains(&format!(".{}", segment)) || path.contains(&format!("[\"{}\"]", segment))
}

fn format_generated_ts(dir: &Path) -> Result<()> {
    let status = Command::new("pnpm")
        .current_dir(dir)
        .args(["biome", "check", "--write", "."])
        .status()
        .context("Failed to run `pnpm biome check --write .` in e2e/node")?;
    ensure!(
        status.success(),
        "`pnpm biome check --write .` exited with non-zero status"
    );

    Ok(())
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

fn expected_content_length(fixture: &Fixture) -> Option<usize> {
    fixture.expected_response.headers.as_ref().and_then(|headers| {
        headers.iter().find_map(|(key, value)| {
            if key.eq_ignore_ascii_case("content-length") {
                value.parse::<usize>().ok()
            } else {
                None
            }
        })
    })
}

fn fixture_requires_binary_body(fixture: &Fixture) -> bool {
    expected_content_length(fixture).is_some()
        && fixture
            .expected_response
            .body
            .as_ref()
            .is_some_and(|body| body.is_string())
}

fn fixture_should_skip(category: &str, fixture: &Fixture) -> bool {
    category == "content_types" && fixture.name == "20_content_length_mismatch"
}

fn build_request_call(method: &str, path_with_query: &str, option_fields: &[&str]) -> String {
    let has_options = !option_fields.is_empty();
    let joined = option_fields.join(", ");

    if (method == "get" || method == "delete")
        && has_options
        && option_fields.len() == 1
        && option_fields[0] == "headers"
    {
        return format!("await client.{}(\"{}\", headers)", method, path_with_query);
    }

    if has_options {
        format!("await client.{}(\"{}\", {{ {} }})", method, path_with_query, joined)
    } else {
        format!("await client.{}(\"{}\")", method, path_with_query)
    }
}
