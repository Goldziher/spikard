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
    let mut handler_defs = String::new();

    code.push_str(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace E2E\\Php;\n\nuse Spikard\\App;\nuse Spikard\\Handlers\\HandlerInterface;\nuse Spikard\\Http\\Request;\nuse Spikard\\Http\\Response;\n\n/**\n * Generated App factory for PHP e2e tests.\n */\nfinal class AppFactory\n{\n",
    );

    if fixtures_by_category.is_empty() {
        code.push_str("    public static function create(): App\n    {\n        return new App();\n    }\n}\n");
        return code;
    }

    for (category, fixtures) in fixtures_by_category {
        for (index, fixture) in fixtures.iter().enumerate() {
            let handler_name = format!("Handler{}_{}", sanitize_identifier(category), index + 1);
            let factory_method = format!(
                "create_{}_{}_{}",
                sanitize_identifier(category),
                sanitize_identifier(&fixture.name),
                index + 1
            );
            let method = fixture.request.method.to_ascii_uppercase();
            let (path, merged_query) = normalize_path_and_query(fixture);
            let status = fixture.expected_response.status_code;
            let body_literal = value_to_php(
                fixture
                    .expected_response
                    .body
                    .as_ref()
                    .unwrap_or(&serde_json::Value::Null),
            );
            let headers_literal = string_map_to_php(fixture.expected_response.headers.as_ref());
            let expected_headers = string_map_to_php_lower(fixture.request.headers.as_ref());
            let expected_cookies = string_map_to_php(fixture.request.cookies.as_ref());
            let expected_query = query_to_php(Some(&merged_query));
            let expected_body = if let Some(files) = fixture.request.files.as_ref() {
                if !files.is_empty() {
                    serde_json::to_value(files)
                        .map(|v| value_to_php(&v))
                        .unwrap_or_else(|_| "null".to_string())
                } else {
                    match &fixture.request.body {
                        Some(b) => value_to_php(b),
                        None => "null".to_string(),
                    }
                }
            } else {
                match &fixture.request.body {
                    Some(b) => value_to_php(b),
                    None => "null".to_string(),
                }
            };

            code.push_str(&format!(
                "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $app = $app->addRoute('{method}', '{path}', new {handler_name}());\n        return $app;\n    }}\n\n",
                factory_method = factory_method,
                method = method,
                path = path,
                handler_name = handler_name
            ));

            let handler_tpl = r#"final class {handler_name} implements HandlerInterface {
    public function __construct() {
        $this->expectedHeaders = {expected_headers};
        $this->expectedCookies = {expected_cookies};
        $this->expectedQuery = {expected_query};
        $this->expectedBody = {expected_body};
    }

    public function matches(Request $request): bool
    {
        if (!$this->matchQuery($request->queryParams)) {
            return false;
        }
        if (!$this->matchHeaders($request->headers)) {
            return false;
        }
        if (!$this->matchCookies($request->cookies)) {
            return false;
        }
        if (!$this->matchBody($request->body)) {
            return false;
        }
        return true;
    }

    public function handle(Request $request): Response
    {
        return new Response({body}, {status}, {headers});
    }

    private function matchQuery(array $actual): bool
    {
        if ($this->expectedQuery === []) {
            return $actual === [];
        }
        if (\count($actual) !== \count($this->expectedQuery)) {
            return false;
        }
        foreach ($this->expectedQuery as $key => $expectedValues) {
            if (!\array_key_exists($key, $actual)) {
                return false;
            }
            $candidate = $actual[$key];
            $normalizedCandidate = \is_array($candidate) ? $candidate : [$candidate];
            \sort($normalizedCandidate);
            $normalizedExpected = $expectedValues;
            \sort($normalizedExpected);
            foreach ($normalizedExpected as $val) {
                if (!\in_array($val, $normalizedCandidate, true)) {
                    return false;
                }
            }
        }
        return true;
    }

    private function matchHeaders(array $actual): bool
    {
        if ($this->expectedHeaders === []) {
            return true;
        }
        $normalized = [];
        foreach ($actual as $k => $v) {
            $normalized[strtolower($k)] = $v;
        }
        return $this->arrayEquals($normalized, $this->expectedHeaders);
    }

    private function matchCookies(array $actual): bool
    {
        if ($this->expectedCookies === []) {
            return true;
        }
        return $this->arrayEquals($actual, $this->expectedCookies);
    }

    private function matchBody(mixed $actual): bool
    {
        if ($this->expectedBody === null) {
            return true;
        }
        return json_encode($actual) === json_encode($this->expectedBody);
    }

    /** @param array<string, array<int, string>|string> $a */
    private function arrayEquals(array $a, array $b): bool
    {
        ksort($a);
        ksort($b);
        foreach ($a as $key => $value) {
            if (!array_key_exists($key, $b)) {
                return false;
            }
            $aval = $value;
            $bval = $b[$key];
            if (is_array($aval) && is_array($bval)) {
                sort($aval);
                sort($bval);
                if ($aval !== $bval) {
                    return false;
                }
            } elseif ($aval !== $bval) {
                return false;
            }
        }
        return count($a) === count($b);
    }

    /** @var array<string, string> */
    private array $expectedHeaders;
    /** @var array<string, string> */
    private array $expectedCookies;
    /** @var array<string, array<int, string>> */
    private array $expectedQuery;
    private mixed $expectedBody;
}

"#;
            handler_defs.push_str(
                &handler_tpl
                    .replace("{handler_name}", &handler_name)
                    .replace("{expected_headers}", &expected_headers)
                    .replace("{expected_cookies}", &expected_cookies)
                    .replace("{expected_query}", &expected_query)
                    .replace("{expected_body}", &expected_body)
                    .replace("{body}", &body_literal)
                    .replace("{status}", &status.to_string())
                    .replace("{headers}", &headers_literal),
            );
        }
    }

    code.push_str("}\n\n");
    code.push_str(&handler_defs);
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
    let mut keys: Vec<_> = map.map(|m| m.keys().cloned().collect()).unwrap_or_default();
    keys.sort();
    let mut parts = Vec::new();
    if let Some(map) = map {
        for k in keys {
            if let Some(v) = map.get(&k) {
                parts.push(format!("{} => {}", php_string_literal(&k), php_string_literal(v)));
            }
        }
    }
    format!("[{}]", parts.join(", "))
}

fn string_map_to_php_lower(map: Option<&std::collections::HashMap<String, String>>) -> String {
    let mut keys: Vec<_> = map.map(|m| m.keys().cloned().collect()).unwrap_or_default();
    keys.sort();
    let mut parts = Vec::new();
    if let Some(map) = map {
        for k in keys {
            if let Some(v) = map.get(&k) {
                parts.push(format!(
                    "{} => {}",
                    php_string_literal(&k.to_ascii_lowercase()),
                    php_string_literal(v)
                ));
            }
        }
    }
    format!("[{}]", parts.join(", "))
}

fn specificity(fixture: &Fixture) -> usize {
    let headers = fixture.request.headers.as_ref().map(|h| h.len()).unwrap_or(0);
    let cookies = fixture.request.cookies.as_ref().map(|c| c.len()).unwrap_or(0);
    let query = fixture.request.query_params.as_ref().map(|q| q.len()).unwrap_or(0);
    let body = fixture
        .request
        .body
        .as_ref()
        .map(|b| if b.is_null() { 0 } else { 2 })
        .unwrap_or(0);
    headers * 4 + cookies * 2 + query * 2 + body
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
                        parts.push(format!(
                            "{}={}",
                            urlencoding::encode(key),
                            urlencoding::encode(&query_value_str(item))
                        ));
                    }
                }
                _ => parts.push(format!(
                    "{}={}",
                    urlencoding::encode(key),
                    urlencoding::encode(&query_value_str(value))
                )),
            }
        }
    }
    Some(parts.join("&"))
}

fn query_to_php(query: Option<&std::collections::HashMap<String, serde_json::Value>>) -> String {
    let mut parts = Vec::new();
    if let Some(map) = query {
        let mut keys: Vec<_> = map.keys().collect();
        keys.sort();
        for k in keys {
            if let Some(v) = map.get(k) {
                match v {
                    serde_json::Value::Array(items) => {
                        let mut values = items
                            .iter()
                            .map(|item| php_string_literal(&query_value_str(item)))
                            .collect::<Vec<_>>();
                        values.sort();
                        parts.push(format!("{} => [{}]", php_string_literal(k), values.join(", ")));
                    }
                    _ => {
                        parts.push(format!(
                            "{} => [{}]",
                            php_string_literal(k),
                            php_string_literal(&query_value_str(v))
                        ));
                    }
                }
            }
        }
    }
    format!("[{}]", parts.join(", "))
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
