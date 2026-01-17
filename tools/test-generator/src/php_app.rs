//! PHP test app generator.
//!
//! Generates AppFactory with routes registered with schemas, delegating all validation
//! to the Rust stack via the native extension. Also writes routes.json for parity with
//! other bindings.

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::grpc::GrpcFixture;
use crate::middleware::{MiddlewareMetadata, parse_middleware};
use anyhow::{Context, Result};
use serde_json::{Value, json};
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

fn build_app_factory(
    fixtures_by_category: &BTreeMap<String, Vec<Fixture>>,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> String {
    let mut code = String::new();
    let mut handler_classes = String::new();

    code.push_str(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace E2E\\Php;\n\nrequire_once __DIR__ . '/../route_helper.php';\n\nuse Spikard\\App;\nuse Spikard\\Config\\ServerConfig;\nuse Spikard\\Config\\CompressionConfig;\nuse Spikard\\Config\\RateLimitConfig;\nuse Spikard\\Config\\ApiKeyConfig;\nuse Spikard\\Config\\JwtConfig;\nuse Spikard\\Config\\CorsConfig;\nuse Spikard\\Config\\OpenApiConfig;\nuse Spikard\\Handlers\\HandlerInterface;\nuse Spikard\\Handlers\\SseEventProducerInterface;\nuse Spikard\\Handlers\\WebSocketHandlerInterface;\nuse Spikard\\Http\\Request;\nuse Spikard\\Http\\Response;\n\n/**\n * Generated App factory for PHP e2e tests.\n * Routes are registered with schemas and executed via the native Rust stack.\n */\nfinal class AppFactory\n{\n",
    );

    if fixtures_by_category.is_empty() && sse_fixtures.is_empty() && websocket_fixtures.is_empty() {
        code.push_str("    public static function create(): App\n    {\n        return new App();\n    }\n}\n");
        return code;
    }

    for (index, fixture) in sse_fixtures.iter().enumerate() {
        let channel_name = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let channel = normalize_path(&channel_name);
        let factory_method = format!("create_sse_{}_{}", sanitize_identifier(&channel_name), index + 1);
        let producer_class = format!("SseProducer_{}", index + 1);

        code.push_str(&format!(
            "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $app = $app->addSse('{channel}', new {producer_class}());\n        return $app;\n    }}\n\n",
            factory_method = factory_method,
            channel = channel,
            producer_class = producer_class
        ));

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
            "final class {producer_class} implements SseEventProducerInterface\n{{\n    /** @return \\Generator<int, string, mixed, void> */\n    public function __invoke(): \\Generator\n    {{\n        $events = {events};\n        foreach ($events as $event) {{\n            yield json_encode(['data' => $event]);\n        }}\n    }}\n}}\n\n",
            producer_class = producer_class,
            events = events_literal
        ));
    }

    for (index, fixture) in websocket_fixtures.iter().enumerate() {
        let channel_name = fixture.channel.clone().unwrap_or_else(|| fixture.name.clone());
        let channel = normalize_path(&channel_name);
        let factory_method = format!("create_websocket_{}_{}", sanitize_identifier(&channel_name), index + 1);
        let handler_class = format!("WebSocketHandler_{}", index + 1);

        code.push_str(&format!(
            "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n        $app = $app->addWebSocket('{channel}', new {handler_class}());\n        return $app;\n    }}\n\n",
            factory_method = factory_method,
            channel = channel,
            handler_class = handler_class
        ));

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
            "final class {handler_class} implements WebSocketHandlerInterface\n{{\n    private array $messages = [];\n    private int $messageIndex = 0;\n\n    public function __construct()\n    {{\n        $this->messages = {messages};\n    }}\n\n    public function onConnect(): void\n    {{\n        // Connection established\n    }}\n\n    public function onMessage(string $message): void\n    {{\n        // Handle incoming message\n    }}\n\n    public function onClose(int $code, ?string $reason = null): void\n    {{\n        // Connection closed\n    }}\n\n    public function getNextMessage(): ?array\n    {{\n        if ($this->messageIndex < count($this->messages)) {{\n            return $this->messages[$this->messageIndex++];\n        }}\n        return null;\n    }}\n}}\n\n",
            handler_class = handler_class,
            messages = messages_literal
        ));
    }

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

            let metadata = parse_middleware(fixture).unwrap_or_default();

            let request_schema = if let Some(handler) = &fixture.handler {
                if let Some(schema) = &handler.body_schema {
                    php_encode_json_value(schema)
                } else {
                    "null".to_string()
                }
            } else {
                "null".to_string()
            };

            let response_schema = "null".to_string();

            let parameter_schema = if let Some(handler) = &fixture.handler {
                if let Some(params) = &handler.parameters {
                    build_parameter_schema_php(params)
                } else {
                    "null".to_string()
                }
            } else {
                "null".to_string()
            };

            let raw_middleware = fixture.handler.as_ref().and_then(|h| h.middleware.as_ref());
            let config_str = generate_server_config_php(&metadata, raw_middleware);

            let handler_class = generate_handler_class_php(fixture, metadata.compression.is_some());

            if config_str == "null" {
                code.push_str(&format!(
                    "    public static function {factory_method}(): App\n    {{\n        $app = new App();\n{handler}\n        $app = \\register_route_with_schemas($app, '{method}', '{path}', $handler, {req}, {resp}, {params});\n        return $app;\n    }}\n\n",
                    factory_method = factory_method,
                    method = method,
                    path = path,
                    handler = handler_class,
                    req = request_schema,
                    resp = response_schema,
                    params = parameter_schema
                ));
            } else {
                code.push_str(&format!(
                    "    public static function {factory_method}(): App\n    {{\n        $config = {config};\n        $app = new App($config);\n{handler}\n        $app = \\register_route_with_schemas($app, '{method}', '{path}', $handler, {req}, {resp}, {params});\n        return $app;\n    }}\n\n",
                    factory_method = factory_method,
                    config = config_str,
                    method = method,
                    path = path,
                    handler = handler_class,
                    req = request_schema,
                    resp = response_schema,
                    params = parameter_schema
                ));
            }
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
    let mut s = input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s.trim_matches('_').to_ascii_lowercase()
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

    for fixture in sse_fixtures {
        let path = normalize_path(&fixture.channel.clone().unwrap_or_else(|| fixture.name.clone()));
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
        let path = normalize_path(&fixture.channel.clone().unwrap_or_else(|| fixture.name.clone()));
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

/// Generate handler class that returns expected response from fixture
fn generate_handler_class_php(fixture: &Fixture, strip_compression_headers: bool) -> String {
    let expected_status = fixture.expected_response.status_code;
    let expected_body = fixture.expected_response.body.as_ref();
    let expected_headers = &fixture.expected_response.headers;

    let body_literal = if let Some(body_value) = expected_body {
        value_to_php(body_value)
    } else {
        "null".to_string()
    };

    let headers_literal = if let Some(headers_map) = expected_headers {
        let mut header_pairs = Vec::new();
        for (key, value) in headers_map {
            if strip_compression_headers
                && (key.eq_ignore_ascii_case("content-encoding") || key.eq_ignore_ascii_case("vary"))
            {
                continue;
            }
            header_pairs.push(format!("{} => {}", php_string_literal(key), php_string_literal(value)));
        }
        if header_pairs.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}]", header_pairs.join(", "))
        }
    } else {
        "[]".to_string()
    };

    format!(
        "        $handler = new class implements HandlerInterface {{\n            public function matches(Request $request): bool {{ return true; }}\n            public function handle(Request $request): Response {{\n                return new Response({}, {}, {});\n            }}\n            public function __invoke(Request $request): Response {{\n                return $this->handle($request);\n            }}\n        }};",
        body_literal, expected_status, headers_literal
    )
}

/// Generate ServerConfig from middleware metadata
fn generate_server_config_php(metadata: &MiddlewareMetadata, raw_middleware: Option<&Value>) -> String {
    let mut config_lines = Vec::new();

    if let Some(compression) = &metadata.compression {
        let mut args = Vec::new();
        if let Some(gzip) = compression.gzip {
            args.push(format!("gzip: {}", if gzip { "true" } else { "false" }));
        }
        if let Some(brotli) = compression.brotli {
            args.push(format!("brotli: {}", if brotli { "true" } else { "false" }));
        }
        if let Some(min_size) = compression.min_size {
            args.push(format!("minSize: {}", min_size));
        }
        if let Some(quality) = compression.quality {
            args.push(format!("quality: {}", quality));
        }
        if args.is_empty() {
            config_lines.push("            compression: new CompressionConfig()".to_string());
        } else {
            config_lines.push(format!(
                "            compression: new CompressionConfig({})",
                args.join(", ")
            ));
        }
    }

    if let Some(rate_limit) = &metadata.rate_limit {
        let mut args = vec![
            format!("perSecond: {}", rate_limit.per_second),
            format!("burst: {}", rate_limit.burst),
        ];
        if let Some(ip_based) = rate_limit.ip_based {
            args.push(format!("ipBased: {}", if ip_based { "true" } else { "false" }));
        }
        config_lines.push(format!(
            "            rateLimit: new RateLimitConfig({})",
            args.join(", ")
        ));
    }

    if let Some(timeout) = &metadata.request_timeout {
        config_lines.push(format!("            requestTimeout: {}", timeout.seconds));
    }

    if let Some(request_id) = &metadata.request_id {
        if let Some(enabled) = request_id.enabled {
            config_lines.push(format!(
                "            enableRequestId: {}",
                if enabled { "true" } else { "false" }
            ));
        }
    }

    if let Some(body_limit) = &metadata.body_limit {
        if let Some(max_bytes) = body_limit.max_bytes {
            config_lines.push(format!("            maxBodySize: {}", max_bytes));
        } else {
            config_lines.push("            maxBodySize: null".to_string());
        }
    }

    if let Some(middleware) = raw_middleware {
        if let Some(jwt) = middleware.get("jwt_auth") {
            config_lines.push(build_jwt_config_php(jwt));
        }

        if let Some(api_key) = middleware.get("api_key_auth") {
            config_lines.push(build_api_key_config_php(api_key));
        }

        if let Some(cors) = middleware.get("cors") {
            config_lines.push(build_cors_config_php(cors));
        }

        if let Some(openapi) = middleware.get("openapi") {
            config_lines.push(build_openapi_config_php(openapi));
        }
    }

    if config_lines.is_empty() {
        "null".to_string()
    } else {
        format!("new ServerConfig(\n{}\n        )", config_lines.join(",\n"))
    }
}

/// Build JWT config block for PHP
fn build_jwt_config_php(jwt: &Value) -> String {
    let mut args = Vec::new();
    if let Some(secret) = jwt.get("secret") {
        args.push(format!("secret: {}", value_to_php(secret)));
    }
    if let Some(algorithm) = jwt.get("algorithm") {
        args.push(format!("algorithm: {}", value_to_php(algorithm)));
    }
    if let Some(audience) = jwt.get("audience") {
        args.push(format!("audience: {}", value_to_php(audience)));
    }
    if let Some(issuer) = jwt.get("issuer") {
        args.push(format!("issuer: {}", value_to_php(issuer)));
    }
    if let Some(leeway) = jwt.get("leeway") {
        args.push(format!("leeway: {}", value_to_php(leeway)));
    }
    format!("            jwtAuth: new JwtConfig({})", args.join(", "))
}

/// Build API Key config block for PHP
fn build_api_key_config_php(api_key: &Value) -> String {
    let mut args = Vec::new();
    if let Some(keys) = api_key.get("keys") {
        args.push(format!("keys: {}", value_to_php(keys)));
    }
    if let Some(header_name) = api_key.get("header_name") {
        args.push(format!("headerName: {}", value_to_php(header_name)));
    }
    format!("            apiKeyAuth: new ApiKeyConfig({})", args.join(", "))
}

/// Build CORS config block for PHP
fn build_cors_config_php(cors: &Value) -> String {
    let mut args = Vec::new();
    if let Some(allowed_origins) = cors.get("allowed_origins") {
        args.push(format!("allowedOrigins: {}", value_to_php(allowed_origins)));
    }
    if let Some(allowed_methods) = cors.get("allowed_methods") {
        args.push(format!("allowedMethods: {}", value_to_php(allowed_methods)));
    }
    if let Some(allowed_headers) = cors.get("allowed_headers") {
        args.push(format!("allowedHeaders: {}", value_to_php(allowed_headers)));
    }
    if let Some(allow_credentials) = cors.get("allow_credentials") {
        args.push(format!("allowCredentials: {}", value_to_php(allow_credentials)));
    }
    if let Some(max_age) = cors.get("max_age") {
        args.push(format!("maxAge: {}", value_to_php(max_age)));
    }
    format!("            cors: new CorsConfig({})", args.join(", "))
}

/// Build OpenAPI config block for PHP
fn build_openapi_config_php(openapi: &Value) -> String {
    let mut args = Vec::new();
    if let Some(title) = openapi.get("title") {
        args.push(format!("title: {}", value_to_php(title)));
    }
    if let Some(version) = openapi.get("version") {
        args.push(format!("version: {}", value_to_php(version)));
    }
    if let Some(description) = openapi.get("description") {
        args.push(format!("description: {}", value_to_php(description)));
    }
    format!("            openapi: new OpenApiConfig({})", args.join(", "))
}

/// Build parameter schema in PHP format
fn build_parameter_schema_php(params: &Value) -> String {
    php_encode_json_value(params)
}

fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

/// Encode JSON value as PHP code that can be evaluated
fn php_encode_json_value(value: &Value) -> String {
    match serde_json::to_string(value) {
        Ok(json_str) => format!("json_decode('{}', true)", escape_php_string(&json_str)),
        Err(_) => "null".to_string(),
    }
}

/// Generate PHP gRPC handler function from a gRPC fixture
pub fn generate_grpc_handler(fixture: &GrpcFixture) -> Result<String> {
    let mut code = String::new();

    // Extract handler details
    let handler_name = sanitize_identifier(&fixture.name);
    let service_name = &fixture.handler.service;
    let method_name = &fixture.handler.method;

    // Build response payload from expected response
    let response_json = serde_json::to_string(&fixture.expected_response.message)
        .context("Failed to serialize expected response")?;

    // PHP opening tag and namespace
    code.push_str("<?php\n\n");
    code.push_str("use Spikard\\Grpc\\GrpcRequest;\n");
    code.push_str("use Spikard\\Grpc\\GrpcResponse;\n\n");

    // Handler function signature with type hints
    code.push_str(&format!(
        "function handleGrpc{}(GrpcRequest $request): GrpcResponse\n",
        to_pascal_case(&handler_name)
    ));
    code.push_str("{\n");

    // Add description as comment if available
    if let Some(description) = &fixture.description {
        let escaped_desc = escape_php_comment(description);
        code.push_str(&format!("    // {}\n", escaped_desc));
    }
    code.push('\n');

    // Build response payload using json_encode
    code.push_str("    // Build response from fixture\n");
    code.push_str(&format!(
        "    $responsePayload = json_encode({});\n",
        value_to_php(&fixture.expected_response.message.clone().unwrap_or(serde_json::Value::Null))
    ));

    // Initialize metadata
    code.push_str("    $metadata = [];\n");

    // Add metadata from request if present
    if let Some(ref metadata) = fixture.request.metadata {
        if !metadata.is_empty() {
            code.push_str("\n    // Request metadata (headers)\n");
            for (key, value) in metadata {
                let escaped_key = escape_php_string(key);
                let escaped_value = escape_php_string(value);
                code.push_str(&format!("    // {}: \"{}\"\n", escaped_key, escaped_value));
            }
        }
    }

    code.push('\n');

    // Return GrpcResponse with named arguments (PHP 8.0+)
    code.push_str("    return new GrpcResponse(\n");
    code.push_str("        payload: $responsePayload,\n");
    code.push_str("        metadata: $metadata,\n");
    code.push_str(&format!(
        "        statusCode: '{}'\n",
        fixture.expected_response.status_code
    ));
    code.push_str("    );\n");
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
