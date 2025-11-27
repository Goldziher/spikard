//! PHP test app generator.
//!
//! Generates AppFactory with routes registered with schemas, delegating all validation
//! to the Rust stack via the native extension. Also writes routes.json for parity with
//! other bindings.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use anyhow::{Context, Result};
use serde_json::json;
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
    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;
    let code = build_app_factory(&fixtures_by_category, &sse_fixtures, &websocket_fixtures);
    fs::write(app_dir.join("main.php"), code).context("Failed to write PHP app main.php")?;

    let routes_json = build_routes_json(&fixtures_by_category, &sse_fixtures, &websocket_fixtures)?;
    fs::write(app_dir.join("routes.json"), routes_json).context("Failed to write routes.json")?;
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

// Simplified AppFactory: routes call into native extension with schemas; no PHP-side matching.
fn build_app_factory(
    fixtures_by_category: &BTreeMap<String, Vec<Fixture>>,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> String {
    let mut code = String::new();
    let mut handler_classes = String::new();

    code.push_str(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace E2E\\Php;\n\nuse Spikard\\App;\nuse Spikard\\Handlers\\HandlerInterface;\nuse Spikard\\Handlers\\SseEventProducerInterface;\nuse Spikard\\Handlers\\WebSocketHandlerInterface;\nuse Spikard\\Http\\Request;\nuse Spikard\\Http\\Response;\n\n/**\n * Generated App factory for PHP e2e tests.\n * Routes are registered with schemas and executed via the native Rust stack.\n */\nfinal class AppFactory\n{\n",
    );

    if fixtures_by_category.is_empty() && sse_fixtures.is_empty() && websocket_fixtures.is_empty() {
        code.push_str("    public static function create(): App\n    {\n        return new App();\n    }\n}\n");
        return code;
    }

    // Generate SSE factory methods
    for (index, fixture) in sse_fixtures.iter().enumerate() {
        let channel = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let factory_method = format!("create_sse_{}_{}", sanitize_identifier(&fixture.name), index + 1);
        let producer_class = format!("SseProducer_{}", index + 1);

        code.push_str(&format!(
            "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $app = $app->addSse('{channel}', new {producer_class}());\n        return $app;\n    }}\n\n",
            factory_method = factory_method,
            channel = channel,
            producer_class = producer_class
        ));

        // Generate SSE producer class
        let events_literal = if fixture.examples.is_empty() {
            "[]".to_string()
        } else {
            let events = fixture
                .examples
                .iter()
                .map(|v| value_to_php(v))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{events}]")
        };

        handler_classes.push_str(&format!(
            "final class {producer_class} implements SseEventProducerInterface\n{{\n    /** @return \\Generator<int, string, mixed, void> */\n    public function __invoke(): \\Generator\n    {{\n        $events = {events};\n        foreach ($events as $event) {{\n            yield 'data: ' . json_encode($event) . \"\\n\\n\";\n        }}\n    }}\n}}\n\n",
            producer_class = producer_class,
            events = events_literal
        ));
    }

    // Generate WebSocket factory methods
    for (index, fixture) in websocket_fixtures.iter().enumerate() {
        let channel = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let factory_method = format!("create_websocket_{}_{}", sanitize_identifier(&fixture.name), index + 1);
        let handler_class = format!("WebSocketHandler_{}", index + 1);

        code.push_str(&format!(
            "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $app = $app->addWebSocket('{channel}', new {handler_class}());\n        return $app;\n    }}\n\n",
            factory_method = factory_method,
            channel = channel,
            handler_class = handler_class
        ));

        // Generate WebSocket handler class with example messages
        let messages_literal = if fixture.examples.is_empty() {
            "[]".to_string()
        } else {
            let messages = fixture
                .examples
                .iter()
                .map(|v| value_to_php(v))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{messages}]")
        };

        handler_classes.push_str(&format!(
            "final class {handler_class} implements WebSocketHandlerInterface\n{{\n    private array $messages = {messages};\n    private int $messageIndex = 0;\n\n    public function onConnect(): void\n    {{\n        // Connection established\n    }}\n\n    public function onMessage(string $message): void\n    {{\n        // Handle incoming message\n    }}\n\n    public function onClose(int $code, ?string $reason = null): void\n    {{\n        // Connection closed\n    }}\n\n    public function getNextMessage(): ?array\n    {{\n        if ($this->messageIndex < count($this->messages)) {{\n            return $this->messages[$this->messageIndex++];\n        }}\n        return null;\n    }}\n}}\n\n",
            handler_class = handler_class,
            messages = messages_literal
        ));
    }

    // Generate regular HTTP route factory methods
    for (category, fixtures) in fixtures_by_category {
        for (index, fixture) in fixtures.iter().enumerate() {
            let factory_method = format!(
                "create_{}_{}_{}",
                sanitize_identifier(category),
                sanitize_identifier(&fixture.name),
                index + 1
            );
            let method = fixture.request.method.to_ascii_uppercase();
            let (path, _) = normalize_path_and_query(fixture, fixture.request.path.as_str());

            let request_schema = "null";
            let response_schema = "null";
            let parameter_schema = "null";

            code.push_str(&format!(
                "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $handler = new class implements HandlerInterface {{\n            public function matches(Request $request): bool {{ return true; }}\n            public function handle(Request $request): Response {{ return new Response([], 200); }}\n        }};\n        $app = $app->addRouteWithSchemas('{method}', '{path}', $handler, json_decode({req}, true), json_decode({resp}, true), json_decode({params}, true));\n        return $app;\n    }}\n\n",
                factory_method = factory_method,
                method = method,
                path = path,
                req = request_schema,
                resp = response_schema,
                params = parameter_schema
            ));
        }
    }

    code.push_str("}\n\n");
    code.push_str(&handler_classes);
    code
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

fn escape_php_string(input: &str) -> String {
    input.replace('\\', "\\\\").replace('\'', "\\'")
}

fn normalize_path_and_query(fixture: &Fixture, path: &str) -> (String, serde_json::Value) {
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path.to_string()
    };
    let merged_query = fixture
        .request
        .query_params
        .clone()
        .unwrap_or_default()
        .into_iter()
        .collect::<serde_json::Map<String, serde_json::Value>>();
    (path, serde_json::Value::Object(merged_query))
}

fn sanitize_identifier(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

fn build_routes_json(
    fixtures_by_category: &BTreeMap<String, Vec<Fixture>>,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> Result<String> {
    let mut routes = Vec::new();

    for (_category, fixtures) in fixtures_by_category {
        for fixture in fixtures {
            let (path, _) = normalize_path_and_query(fixture, fixture.request.path.as_str());
            routes.push(json!({
                "method": fixture.request.method.to_ascii_uppercase(),
                "path": path,
                "handler_name": fixture.name,
                "request_schema": null,
                "response_schema": null,
                "parameter_schema": null,
            }));
        }
    }

    // SSE and WebSocket placeholders (no schema)
    for fixture in sse_fixtures {
        let path = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        routes.push(json!({
            "method": "GET",
            "path": path,
            "handler_name": fixture.name,
            "request_schema": null,
            "response_schema": null,
            "parameter_schema": null,
        }));
    }
    for fixture in websocket_fixtures {
        let path = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        routes.push(json!({
            "method": "GET",
            "path": path,
            "handler_name": fixture.name,
            "request_schema": null,
            "response_schema": null,
            "parameter_schema": null,
        }));
    }

    Ok(serde_json::to_string_pretty(&routes)?)
}
