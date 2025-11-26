//! PHP test app generator.
//!
//! Generates a namespaced `AppFactory` with per-category creators. Each
//! factory registers handlers that return the fixture's expected response to
//! drive TDD for the PHP bindings.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn generate_php_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("app");
    if app_dir.exists() {
        fs::remove_dir_all(&app_dir).context("Failed to clear existing PHP app directory")?;
    }
    fs::create_dir_all(&app_dir).context("Failed to create PHP app directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;
    let code = build_app_factory(&fixtures_by_category);
    fs::write(app_dir.join("main.php"), code).context("Failed to write PHP app main.php")?;
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

fn build_app_factory(fixtures_by_category: &BTreeMap<String, Vec<Fixture>>) -> String {
    let mut code = String::new();
    code.push_str(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace E2E\\Php;\n\nuse Spikard\\App;\nuse Spikard\\Handlers\\HandlerInterface;\nuse Spikard\\Http\\Request;\nuse Spikard\\Http\\Response;\n\n/**\n * Generated App factory for PHP e2e tests.\n */\nfinal class AppFactory\n{\n",
    );

    if fixtures_by_category.is_empty() {
        code.push_str("    public static function create(): App\n    {\n        return new App();\n    }\n}\n");
        return code;
    }

    for (category, fixtures) in fixtures_by_category {
        let method_name = format!("create_{}", sanitize_identifier(category));
        code.push_str(&format!(
            "    public static function {method}(): App\n    {{\n        $app = new App();\n",
            method = method_name
        ));

        for (index, fixture) in fixtures.iter().enumerate() {
            let handler_name = format!("Handler{}_{}", sanitize_identifier(category), index + 1);
            let method = fixture.request.method.to_ascii_uppercase();
            let mut path = fixture.request.path.clone();
            if let Some(query) = fixture.request.query_params.as_ref() {
                if !query.is_empty() {
                    if let Some(encoded) = build_query_string(query) {
                        path = format!("{}?{}", path, encoded);
                    }
                }
            }
            let status = fixture.expected_response.status_code;
            let body_literal = value_to_php(
                fixture
                    .expected_response
                    .body
                    .as_ref()
                    .unwrap_or(&serde_json::Value::Null),
            );
            let headers_literal = string_map_to_php(fixture.expected_response.headers.as_ref());

            code.push_str(&format!(
                "        $app = $app->addRoute('{method}', '{path}', new {handler_name}());\n",
                method = method,
                path = path,
                handler_name = handler_name
            ));
            code.push_str(&format!(
                "        class {handler_name} implements HandlerInterface {{\n            public function handle(Request $request): Response\n            {{\n                $response = new Response({body}, {status}, {headers});\n                return $response;\n            }}\n        }}\n\n",
                handler_name = handler_name,
                body = body_literal,
                status = status,
                headers = headers_literal
            ));
        }

        code.push_str("        return $app;\n    }\n\n");
    }

    code.push_str("}\n");
    code
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

fn string_map_to_php(map: Option<&std::collections::HashMap<String, String>>) -> String {
    let mut parts = Vec::new();
    if let Some(map) = map {
        for (k, v) in map {
            parts.push(format!("{} => {}", php_string_literal(k), php_string_literal(v)));
        }
    }
    format!("[{}]", parts.join(", "))
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
                    parts.push(format!(
                        "{}={}",
                        urlencoding::encode(key),
                        urlencoding::encode(&item.to_string())
                    ));
                }
            }
            _ => parts.push(format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(&value.to_string())
            )),
        }
    }
    Some(parts.join("&"))
}
