//! PHP PHPUnit test generator.
//!
//! Generates per-fixture tests that exercise the HTTP surface. Tests are marked
//! incomplete until the PHP bindings are implemented, but assertions are
//! emitted to drive TDD once the runtime is ready.

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
    let test_code = build_test_file(&fixtures_by_category);
    fs::write(tests_dir.join("GeneratedTest.php"), test_code).context("Failed to write GeneratedTest.php")?;

    let bootstrap = r#"<?php
declare(strict_types=1);

require_once __DIR__ . '/../bootstrap.php';
"#;
    fs::write(tests_dir.join("bootstrap.php"), bootstrap).context("Failed to write test bootstrap")?;

    fs::write(output_dir.join("bootstrap.php"), bootstrap_file()?).context("Failed to write bootstrap.php")?;
    fs::write(output_dir.join("phpunit.xml"), phpunit_config()?).context("Failed to write phpunit.xml")?;

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
            if category == "sse" || category == "websockets" {
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

fn build_test_file(fixtures_by_category: &BTreeMap<String, Vec<Fixture>>) -> String {
    let mut code = String::new();
    code.push_str(
        "<?php\ndeclare(strict_types=1);\n\nuse PHPUnit\\Framework\\TestCase;\nuse Spikard\\Testing\\TestClient;\nuse E2E\\Php\\AppFactory;\n\n/**\n * Generated from testing_data fixtures.\n * Tests are marked incomplete until the PHP bindings are wired.\n */\nfinal class GeneratedTest extends TestCase\n{\n    protected function setUp(): void\n    {\n        $this->markTestIncomplete('PHP bindings not implemented yet.');\n    }\n\n",
    );

    for (category, fixtures) in fixtures_by_category {
        for (index, fixture) in fixtures.iter().enumerate() {
            code.push_str(&build_fixture_test(category, index, fixture));
        }
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
    let factory = format!("create_{}", sanitize_identifier(category));
    let method = fixture.request.method.to_ascii_uppercase();
    let mut path = fixture.request.path.clone();

    if let Some(query) = fixture.request.query_params.as_ref() {
        if !query.is_empty() {
            if let Some(encoded) = build_query_string(query) {
                path = format!("{}?{}", path, encoded);
            }
        }
    }

    let mut options = Vec::new();
    if let Some(headers) = fixture.request.headers.as_ref() {
        if !headers.is_empty() {
            options.push(format!("'headers' => {}", string_map_to_php(headers)));
        }
    }
    if let Some(cookies) = fixture.request.cookies.as_ref() {
        if !cookies.is_empty() {
            options.push(format!("'cookies' => {}", string_map_to_php(cookies)));
        }
    }
    if let Some(files) = fixture.request.files.as_ref() {
        if !files.is_empty() {
            options.push(format!("'files' => {}", value_to_php(&serde_json::json!(files))));
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
        .map(value_to_php)
        .unwrap_or_else(|| "null".to_string());

    format!(
        "    public function {method_name}(): void\n    {{\n        $app = AppFactory::{factory}();\n        $client = TestClient::create($app);\n        $response = $client->request('{http_method}', '{path}', {options});\n\n        $this->assertSame({status}, $response->statusCode);\n        $this->assertEquals({expected_body}, $response->body);\n    }}\n\n",
        method_name = method_name,
        factory = factory,
        http_method = method,
        path = path,
        options = options_literal,
        status = expected_status,
        expected_body = expected_body
    )
}

fn build_query_string(query: &std::collections::HashMap<String, serde_json::Value>) -> Option<String> {
    if query.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    for (key, value) in query {
        match value {
            serde_json::Value::Array(items) => {
                for item in items {
                    parts.push(format!("{}={}", encode(key), encode(&item.to_string())));
                }
            }
            _ => parts.push(format!("{}={}", encode(key), encode(&value.to_string()))),
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
            format!("[{}]", items)
        }
        serde_json::Value::Object(map) => {
            let mut parts = Vec::new();
            for (k, v) in map {
                parts.push(format!("{} => {}", php_string_literal(k), value_to_php(v)));
            }
            format!("[{}]", parts.join(", "))
        }
    }
}

fn php_string_literal(input: &str) -> String {
    let escaped = input.replace('\\', "\\\\").replace('\'', "\\'");
    format!("'{}'", escaped)
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
require_once __DIR__ . '/app/main.php';
"#;
    Ok(bootstrap.to_string())
}
