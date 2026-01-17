//! PHP PHPUnit test generator.
//!
//! Generates per-fixture tests that exercise the HTTP surface. Includes SSE/WS
//! tests when the native extension is available.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::grpc::GrpcFixture;
use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use urlencoding::encode;

pub fn generate_php_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let tests_dir = output_dir.join("tests");
    if tests_dir.exists() {
        fs::remove_dir_all(&tests_dir).context("Failed to clear existing PHP tests")?;
    }
    fs::create_dir_all(&tests_dir).context("Failed to create PHP tests directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;
    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;
    let test_code = build_test_file(&fixtures_by_category, &sse_fixtures, &websocket_fixtures);
    fs::write(tests_dir.join("GeneratedTest.php"), test_code).context("Failed to write GeneratedTest.php")?;

    // gRPC test generation
    let grpc_fixtures_result = crate::grpc::load_grpc_fixtures(fixtures_dir);
    if let Ok(grpc_fixtures) = grpc_fixtures_result {
        if !grpc_fixtures.is_empty() {
            for fixture in &grpc_fixtures {
                let test_code = generate_grpc_test(fixture)
                    .context(format!("Failed to generate gRPC test for {}", fixture.name))?;

                let test_name = sanitize_identifier(&fixture.name);
                let test_name_pascal = to_pascal_case(&test_name);
                let test_file = tests_dir.join(format!("Grpc{}Test.php", test_name_pascal));

                fs::write(&test_file, test_code).with_context(|| format!("Failed to write gRPC test file for {}", fixture.name))?;
                println!("  ✓ Generated tests/Grpc{}Test.php", test_name_pascal);
            }
        }
    }

    let bootstrap = r#"<?php
declare(strict_types=1);

require_once __DIR__ . '/../bootstrap.php';
"#;
    fs::write(tests_dir.join("bootstrap.php"), bootstrap).context("Failed to write test bootstrap")?;
    fs::write(tests_dir.join("helpers.php"), helpers_file()?).context("Failed to write test helpers")?;

    fs::write(output_dir.join("bootstrap.php"), bootstrap_file()?).context("Failed to write bootstrap.php")?;
    fs::write(output_dir.join("phpunit.xml"), phpunit_config()?).context("Failed to write phpunit.xml")?;
    fs::write(output_dir.join("route_helper.php"), route_helper()).context("Failed to write route_helper.php")?;

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
            if category == "sse" || category == "websockets" || category == "jsonrpc" {
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

fn build_test_file(
    fixtures_by_category: &BTreeMap<String, Vec<Fixture>>,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> String {
    let mut code = String::new();
    code.push_str(
        "<?php\ndeclare(strict_types=1);\n\nuse PHPUnit\\Framework\\TestCase;\nuse Spikard\\Testing\\TestClient;\nuse E2E\\Php\\AppFactory;\n\n/**\n * Generated from testing_data fixtures.\n * @phpstan-type ResponseBody array<string, mixed>|string|int|float|bool|null\n */\nfinal class GeneratedTest extends TestCase\n{\n",
    );

    for (category, fixtures) in fixtures_by_category {
        for (index, fixture) in fixtures.iter().enumerate() {
            code.push_str(&build_fixture_test(category, index, fixture));
        }
    }

    for (idx, fixture) in sse_fixtures.iter().enumerate() {
        let channel = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let method_name = format!("test_sse_{}_{}", sanitize_identifier(&channel), idx + 1);
        let factory = format!("create_sse_{}_{}", sanitize_identifier(&channel), idx + 1);
        let expected_events = if fixture.examples.is_empty() {
            "[]".to_string()
        } else {
            let items = fixture
                .examples
                .iter()
                .map(value_to_php_expected)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{items}]")
        };
        code.push_str(&format!(
            "    public function {method}(): void\n    {{\n        $app = AppFactory::{factory}();\n        $client = TestClient::create($app);\n        $stream = $client->connectSse('{path}');\n\n        /** @var array<mixed> $events */\n        $events = $stream->eventsAsJson();\n        $this->assertEquals({expected}, $events);\n        $client->close();\n    }}\n\n",
            method = method_name,
            factory = factory,
            path = channel,
            expected = expected_events
        ));
    }

    for (idx, fixture) in websocket_fixtures.iter().enumerate() {
        let channel = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let method_name = format!("test_websocket_{}_{}", sanitize_identifier(&channel), idx + 1);
        let factory = format!("create_websocket_{}_{}", sanitize_identifier(&channel), idx + 1);
        let payload = fixture
            .examples
            .get(0)
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let send_text = php_string_literal(&payload.to_string());
        code.push_str(&format!(
            "    public function {method}(): void\n    {{\n        $app = AppFactory::{factory}();\n        $client = TestClient::create($app);\n        $ws = $client->connectWebSocket('{path}');\n        $ws->sendJson({send});\n        $this->assertFalse($ws->isClosed());\n        $ws->close();\n        $this->assertTrue($ws->isClosed());\n        $client->close();\n    }}\n\n",
            method = method_name,
            factory = factory,
            path = channel,
            send = send_text
        ));
    }

    code.push_str("}\n");
    code
}

fn build_fixture_test(category: &str, index: usize, fixture: &Fixture) -> String {
    let method_name = format!(
        "test_{}_{}",
        sanitize_identifier(category),
        sanitize_identifier(&fixture.name)
    );
    let factory = format!(
        "create_{}_{}_{}",
        sanitize_identifier(category),
        sanitize_identifier(&fixture.name),
        index + 1
    );
    let method = fixture.request.method.to_ascii_uppercase();
    let (path_only, merged_query) = normalize_path_and_query(fixture);
    let path = if let Some(encoded) = build_query_string(&merged_query) {
        format!("{}?{}", path_only, encoded)
    } else {
        path_only
    };

    let mut options = Vec::new();
    let mut headers = fixture.request.headers.clone().unwrap_or_default();
    if let Some(content_type) = fixture.request.content_type.as_ref() {
        let has_content_type = headers.keys().any(|key| key.eq_ignore_ascii_case("content-type"));
        if !has_content_type {
            headers.insert("Content-Type".to_string(), content_type.clone());
        }
    }
    if !headers.is_empty() {
        options.push(format!("'headers' => {}", string_map_to_php(&headers)));
    }
    if let Some(cookies) = fixture.request.cookies.as_ref() {
        if !cookies.is_empty() {
            options.push(format!("'cookies' => {}", string_map_to_php(cookies)));
        }
    }
    if let Some(files) = fixture.request.files.as_ref() {
        options.push(format!("'files' => {}", value_to_php(&serde_json::json!(files))));
    }
    if let Some(form_data) = fixture.request.form_data.as_ref() {
        let form_value = serde_json::to_value(form_data).unwrap_or(serde_json::Value::Null);
        options.push(format!("'form_data' => {}", value_to_php(&form_value)));
    }
    if let Some(data) = fixture.request.data.as_ref() {
        if !data.is_empty() {
            let data_value = serde_json::to_value(data).unwrap_or(serde_json::Value::Null);
            options.push(format!("'data' => {}", value_to_php(&data_value)));
        }
    }
    if let Some(body) = fixture.request.body.as_ref() {
        options.push(format!("'body' => {}", value_to_php(body)));
    }

    let options_literal = if options.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", options.join(", "))
    };

    let expected_status = fixture.expected_response.status_code;
    let expected_body = fixture
        .expected_response
        .body
        .as_ref()
        .map(value_to_php_expected)
        .unwrap_or_else(|| "null".to_string());

    // Determine if we need type assertions for the expected body
    let is_array_value = fixture
        .expected_response
        .body
        .as_ref()
        .map(|v| matches!(v, serde_json::Value::Array(_) | serde_json::Value::Object(_)))
        .unwrap_or(false);

    let body_assertion = if is_array_value {
        format!(
            "        $body = $response->body;\n        /** @var array<string, mixed>|string|int|float|bool|null $expected */\n        $expected = {};\n        $this->assertEquals($expected, $body);",
            expected_body
        )
    } else {
        format!("        $body = $response->body;\n        $this->assertEquals({}, $body);", expected_body)
    };

    format!(
        "    public function {method_name}(): void\n    {{\n        $app = AppFactory::{factory}();\n        $client = TestClient::create($app);\n        $response = $client->request('{http_method}', '{path}', {options});\n\n        /** @var int $statusCode */\n        $statusCode = $response->statusCode;\n        $this->assertSame({status}, $statusCode);\n\n{body_assertion}\n    }}\n\n",
        method_name = method_name,
        factory = factory,
        http_method = method,
        path = path,
        options = options_literal,
        status = expected_status,
        body_assertion = body_assertion
    )
}

fn build_query_string(query: &std::collections::HashMap<String, serde_json::Value>) -> Option<String> {
    if query.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    let mut keys: Vec<_> = query.keys().collect();
    keys.sort();
    for key in keys {
        if let Some(value) = query.get(key) {
            match value {
                serde_json::Value::Array(items) => {
                    for item in items {
                        parts.push(format!("{}={}", encode(key), encode(&query_value_str(item))));
                    }
                }
                _ => parts.push(format!("{}={}", encode(key), encode(&query_value_str(value)))),
            }
        }
    }
    Some(parts.join("&"))
}

fn string_map_to_php(map: &std::collections::HashMap<String, String>) -> String {
    let mut parts = Vec::new();
    for (k, v) in map {
        parts.push(format!("{} => {}", php_string_literal(k), php_string_literal(v)));
    }
    format!("[{}]", parts.join(", "))
}

fn value_to_php(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => php_string_literal(s),
        serde_json::Value::Array(arr) => {
            let items = arr.iter().map(value_to_php).collect::<Vec<_>>().join(", ");
            format!("[{items}]")
        }
        serde_json::Value::Object(map) => {
            if map.is_empty() {
                return "(object)[]".to_string();
            }
            let mut parts = Vec::new();
            for (k, v) in map {
                parts.push(format!("{} => {}", php_string_literal(k), value_to_php(v)));
            }
            format!("[{}]", parts.join(", "))
        }
    }
}

fn value_to_php_expected(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) if map.is_empty() => "[]".to_string(),
        serde_json::Value::Array(arr) => {
            let items = arr.iter().map(value_to_php_expected).collect::<Vec<_>>().join(", ");
            format!("[{items}]")
        }
        serde_json::Value::Object(map) => {
            let mut parts = Vec::new();
            for (k, v) in map {
                parts.push(format!("{} => {}", php_string_literal(k), value_to_php_expected(v)));
            }
            format!("[{}]", parts.join(", "))
        }
        other => value_to_php(other),
    }
}

fn route_helper() -> String {
    r#"<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\JsonRpcMethodInfo;

/**
 * Register a basic route for E2E fixtures using App internals.
 */
function register_route(App $app, string $method, string $path, HandlerInterface $handler): App
{
    return register_route_internal($app, $method, $path, $handler, null, null, null, null);
}

/**
 * Register a schema-backed route for E2E fixtures using App internals.
 *
 * @param array<string, mixed>|null $requestSchema
 * @param array<string, mixed>|null $responseSchema
 * @param array<string, mixed>|null $parameterSchema
 */
function register_route_with_schemas(
    App $app,
    string $method,
    string $path,
    HandlerInterface $handler,
    ?array $requestSchema,
    ?array $responseSchema,
    ?array $parameterSchema,
): App {
    return register_route_internal($app, $method, $path, $handler, $requestSchema, $responseSchema, $parameterSchema, null);
}

/**
 * @param array<string, mixed>|null $requestSchema
 * @param array<string, mixed>|null $responseSchema
 * @param array<string, mixed>|null $parameterSchema
 */
function register_route_internal(
    App $app,
    string $method,
    string $path,
    HandlerInterface $handler,
    ?array $requestSchema,
    ?array $responseSchema,
    ?array $parameterSchema,
    ?JsonRpcMethodInfo $jsonRpcMethod,
): App {
    $refMethod = new \ReflectionMethod($app, 'registerRoute');
    $refMethod->setAccessible(true);
    /** @var App $result */
    $result = $refMethod->invoke(
        $app,
        $method,
        $path,
        $handler,
        $requestSchema,
        $responseSchema,
        $parameterSchema,
        $jsonRpcMethod,
    );
    return $result;
}
"#
    .to_string()
}

fn php_string_literal(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len() + 2);
    escaped.push('"');
    for ch in input.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '$' => escaped.push_str("\\$"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{0008}' => escaped.push_str("\\b"),
            '\u{000C}' => escaped.push_str("\\f"),
            '\0' => escaped.push_str("\\x00"),
            _ if ch.is_control() => escaped.push_str(&format!("\\x{:02x}", ch as u32)),
            _ => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

fn sanitize_identifier(input: &str) -> String {
    let mut s = input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s.trim_matches('_').to_ascii_lowercase()
}

fn normalize_path_and_query(fixture: &Fixture) -> (String, std::collections::HashMap<String, serde_json::Value>) {
    let mut base_path = fixture.request.path.clone();
    let mut merged: std::collections::HashMap<String, serde_json::Value> =
        fixture.request.query_params.clone().unwrap_or_default();

    if let Some((path_part, query_part)) = fixture.request.path.split_once('?') {
        base_path = path_part.to_string();
        let parsed = parse_query_map(query_part);
        for (k, v) in parsed {
            merged
                .entry(k)
                .and_modify(|existing| match (existing, v.clone()) {
                    (serde_json::Value::Array(arr), serde_json::Value::Array(mut incoming)) => {
                        arr.append(&mut incoming);
                    }
                    (serde_json::Value::Array(arr), other) => {
                        arr.push(other);
                    }
                    (existing_val, serde_json::Value::Array(incoming)) => {
                        let mut combined = vec![existing_val.take()];
                        combined.extend(incoming);
                        *existing_val = serde_json::Value::Array(combined);
                    }
                    (existing_val, other) => {
                        let prev = existing_val.take();
                        *existing_val = serde_json::Value::Array(vec![prev, other]);
                    }
                })
                .or_insert(v);
        }
    }

    (base_path, merged)
}

fn parse_query_map(raw: &str) -> std::collections::HashMap<String, serde_json::Value> {
    let mut map = std::collections::HashMap::new();
    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut split = pair.splitn(2, '=');
        let key = split.next().unwrap_or("").to_string();
        let val = split.next().unwrap_or("");
        let decoded_key = urlencoding::decode(&key)
            .unwrap_or_else(|_| key.clone().into())
            .to_string();
        let decoded_val = urlencoding::decode(val).unwrap_or_else(|_| val.into()).to_string();
        map.entry(decoded_key)
            .and_modify(|existing| {
                if let serde_json::Value::Array(arr) = existing {
                    arr.push(serde_json::Value::String(decoded_val.clone()));
                } else {
                    let prev = existing.take();
                    *existing = serde_json::Value::Array(vec![prev, serde_json::Value::String(decoded_val.clone())]);
                }
            })
            .or_insert_with(|| serde_json::Value::String(decoded_val));
    }
    map
}

fn query_value_str(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "".to_string(),
        serde_json::Value::Array(arr) => arr.iter().map(query_value_str).collect::<Vec<_>>().join(","),
        serde_json::Value::Object(_) => value.to_string(),
    }
}

fn phpunit_config() -> Result<String> {
    let config = r#"<?xml version="1.0" encoding="UTF-8"?>
<phpunit bootstrap="bootstrap.php" colors="true">
    <testsuites>
        <testsuite name="Spikard PHP E2E">
            <directory>tests</directory>
        </testsuite>
    </testsuites>
</phpunit>
"#;
    Ok(config.to_string())
}

fn bootstrap_file() -> Result<String> {
    let bootstrap = r#"<?php
declare(strict_types=1);

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';
require_once __DIR__ . '/tests/helpers.php';
require_once __DIR__ . '/app/main.php';
"#;
    Ok(bootstrap.to_string())
}

fn helpers_file() -> Result<String> {
    let helpers = r#"<?php
declare(strict_types=1);

namespace Spikard\Tests;

use Spikard\DI\ResolvedDependencies;
use Spikard\Http\Request;

/**
 * Create a Request while ensuring the body is stored in a variable
 * so extensions that require pass-by-reference can accept it.
 *
 * @param array<string, string> $headers
 * @param array<string, string> $cookies
 * @param array<string, array<int, string>> $queryParams
 * @param array<string, string> $pathParams
 * @param array<string, mixed>|null $validatedParams
 * @param array<string, mixed> $files
 */
function make_request(
    string $method,
    string $path,
    mixed $body,
    array $headers = [],
    array $cookies = [],
    array $queryParams = [],
    array $pathParams = [],
    ?array $validatedParams = null,
    array $files = [],
    ?ResolvedDependencies $dependencies = null,
): Request {
    $bodyRef = $body;
    return new Request(
        $method,
        $path,
        $bodyRef,
        $headers,
        $cookies,
        $queryParams,
        $pathParams,
        $validatedParams,
        $files,
        $dependencies,
    );
}

/**
 * Execute PHP code in a subprocess without loading the extension.
 *
 * @return array{int, string} [exitCode, output]
 */
function run_without_extension(string $code): array
{
    $autoloadPath = \realpath(__DIR__ . '/../../packages/php/vendor/autoload.php');
    if ($autoloadPath === false) {
        return [1, 'Failed to resolve autoload.php path'];
    }

    $command = \sprintf(
        '%s -n -d detect_unicode=0 -r %s',
        \escapeshellarg(PHP_BINARY),
        \escapeshellarg(\"require '{$autoloadPath}';\" . $code)
    );

    $output = [];
    $exitCode = 0;
    \exec($command . ' 2>&1', $output, $exitCode);

    return [$exitCode, \implode(\"\\n\", $output)];
}
"#;
    Ok(helpers.to_string())
}

/// Generate PHP PHPUnit test method for a gRPC fixture
pub fn generate_grpc_test(fixture: &GrpcFixture) -> Result<String> {
    let mut code = String::new();

    let test_name = sanitize_identifier(&fixture.name);
    let class_name = format!("Grpc{}Test", to_pascal_case(&test_name));
    let handler_name = format!("handleGrpc{}", to_pascal_case(&test_name));

    code.push_str("<?php\n");
    code.push_str("declare(strict_types=1);\n\n");
    code.push_str("use PHPUnit\\Framework\\TestCase;\n\n");
    code.push_str(&format!("final class {} extends TestCase\n{{\n", class_name));

    // Test method
    code.push_str(&format!(
        "    public function testGrpc{}(): void\n",
        to_pascal_case(&test_name)
    ));
    code.push_str("    {\n");

    // Add description as comment if available
    if let Some(description) = &fixture.description {
        let escaped_desc = escape_php_comment(description);
        code.push_str(&format!("        // {}\n", escaped_desc));
    }
    code.push('\n');

    // Build metadata
    let metadata_literal = if let Some(ref metadata) = fixture.request.metadata {
        if metadata.is_empty() {
            "[]".to_string()
        } else {
            let mut pairs = Vec::new();
            for (key, value) in metadata {
                pairs.push(format!("{} => {}", php_string_literal(key), php_string_literal(value)));
            }
            format!("[{}]", pairs.join(", "))
        }
    } else {
        "[]".to_string()
    };

    code.push_str("        // Build gRPC request from fixture\n");
    code.push_str(&format!("        $metadata = {};\n", metadata_literal));

    // Build request payload
    let request_payload = if let Some(ref message) = fixture.request.message {
        value_to_php_expected(message)
    } else {
        "[]".to_string()
    };

    code.push_str(&format!("        $requestPayload = json_encode({});\n", request_payload));
    code.push('\n');

    code.push_str("        $request = new \\Spikard\\Grpc\\GrpcRequest(\n");
    code.push_str(&format!("            serviceName: '{}',\n", fixture.handler.service));
    code.push_str(&format!("            methodName: '{}',\n", fixture.handler.method));
    code.push_str("            payload: $requestPayload,\n");
    code.push_str("            metadata: $metadata,\n");
    code.push_str("        );\n\n");

    // Call handler
    code.push_str("        // Call handler\n");
    code.push_str(&format!("        /** @var \\Spikard\\Grpc\\GrpcResponse $response */\n"));
    code.push_str(&format!("        $response = {}($request);\n", handler_name));
    code.push('\n');

    // Verify response
    code.push_str("        // Verify response\n");
    code.push_str("        /** @var string $statusCode */\n");
    code.push_str("        $statusCode = $response->statusCode;\n");
    code.push_str(&format!(
        "        $this->assertSame('{}', $statusCode);\n",
        fixture.expected_response.status_code
    ));

    // Assert payload if present
    if let Some(ref expected_msg) = fixture.expected_response.message {
        let expected_json = value_to_php_expected(expected_msg);
        code.push_str("\n");
        code.push_str("        /** @var string $payload */\n");
        code.push_str("        $payload = $response->payload;\n");
        code.push_str(&format!(
            "        $this->assertEquals(json_encode({}), $payload);\n",
            expected_json
        ));
    }

    // Assert metadata if present
    if let Some(ref metadata) = fixture.request.metadata {
        if !metadata.is_empty() {
            code.push_str("\n");
            code.push_str("        /** @var mixed $metadata */\n");
            code.push_str("        $metadata = $response->metadata;\n");
            code.push_str("        $this->assertNotNull($metadata);\n");
        }
    }

    code.push_str("    }\n\n");
    code.push_str("}\n");

    Ok(code)
}

/// Convert snake_case to PascalCase for class/function names
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}

/// Escape a string for use in PHP comments
fn escape_php_comment(s: &str) -> String {
    s.replace('\n', " ").replace('\r', " ")
}
