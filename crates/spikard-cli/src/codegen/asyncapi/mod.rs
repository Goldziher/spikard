//! AsyncAPI v3 specification parsing and code generation
//!
//! This module handles parsing AsyncAPI v3 specs and generating:
//! - Test fixtures for WebSocket/SSE messages
//! - Test applications for Python, Node.js, and Ruby
//!
//! AsyncAPI is the standard for describing event-driven APIs, similar to
//! how OpenAPI describes REST APIs.

use super::ts_schema::{TypeScriptDto, generate_typescript_dto};
use anyhow::{Context, Result, bail};
use asyncapiv3::spec::operation::OperationAction;
use asyncapiv3::spec::{AsyncApiSpec, AsyncApiV3Spec};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct MessageDefinition {
    schema: Value,
    examples: Vec<Value>,
}

#[derive(Debug, Clone)]
struct MessageOperationMetadata {
    name: String,
    action: String,
    replies: Vec<String>,
}

/// Channel operation metadata extracted from AsyncAPI spec.
///
/// Contains information about operations defined on a channel, including
/// the operation name, action type (send/receive), messages involved,
/// and any reply messages. Used during fixture generation and test app creation.
///
/// NOTE: Marked with #[allow(dead_code)] because derived traits (Debug, Clone)
/// don't count as usage for Rust's dead code linter, even though this struct
/// is actively used in `collect_channel_operations()` to populate channel metadata.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChannelOperation {
    name: String,
    action: String,
    messages: Vec<String>,
    replies: Vec<String>,
}

fn decode_pointer_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

fn resolve_channel_from_ref(reference: &str) -> Option<String> {
    let raw = reference.strip_prefix("#/channels/")?;
    let decoded = raw.split('/').map(decode_pointer_segment).collect::<Vec<_>>().join("/");
    let normalized = decoded.trim_start_matches('/').to_string();
    Some(format!("/{}", normalized))
}

fn resolve_message_from_ref(reference: &str) -> Option<String> {
    if let Some(name) = reference.strip_prefix("#/components/messages/") {
        return Some(name.to_string());
    }

    if let Some(rest) = reference.strip_prefix("#/channels/") {
        let mut parts = rest.split('/');
        let channel = parts.next()?;
        if parts.next()? != "messages" {
            return None;
        }
        let message = parts.next()?;
        let channel_name = decode_pointer_segment(channel);
        let slug = channel_name.trim_start_matches('/').replace('/', "_");
        return Some(format!("{}_{}", slug, decode_pointer_segment(message)));
    }

    None
}

fn operation_action_name(action: &OperationAction) -> &'static str {
    match action {
        OperationAction::Send => "send",
        OperationAction::Receive => "receive",
    }
}

/// Parse an AsyncAPI v3 specification file
///
/// Supports both JSON and YAML formats
pub fn parse_asyncapi_schema(path: &Path) -> Result<AsyncApiV3Spec> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read AsyncAPI file: {}", path.display()))?;

    let spec: AsyncApiSpec = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI JSON from {}", path.display()))?
    } else {
        serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI YAML from {}", path.display()))?
    };

    match spec {
        AsyncApiSpec::V3_0_0(v3_spec) => Ok(v3_spec),
    }
}

/// Extract message schemas from AsyncAPI spec for fixture generation
///
/// Returns a map of message name -> JSON Schema for generating test fixtures
fn extract_message_schemas(spec: &AsyncApiV3Spec) -> Result<HashMap<String, MessageDefinition>> {
    use asyncapiv3::spec::common::Either;

    let mut schemas = HashMap::new();

    for (message_name, message_ref_or) in &spec.components.messages {
        tracing::debug!("Processing message: {}", message_name);

        match message_ref_or {
            Either::Right(message) => {
                if let Some(definition) = build_message_definition(message, message_name)? {
                    schemas.insert(message_name.clone(), definition);
                }
            }
            Either::Left(reference) => {
                // TODO: Implement reference resolution
                tracing::debug!("Skipping message reference: {}", reference.reference);
            }
        }
    }

    for (channel_name, channel_ref_or) in &spec.channels {
        tracing::debug!("Processing channel: {}", channel_name);

        match channel_ref_or {
            Either::Right(channel) => {
                for (msg_name, msg_ref_or) in &channel.messages {
                    match msg_ref_or {
                        Either::Right(message) => {
                            let full_name = format!("{}_{}", channel_name.trim_start_matches('/'), msg_name);
                            if let Some(definition) = build_message_definition(message, &full_name)? {
                                schemas.insert(full_name, definition);
                            }
                        }
                        Either::Left(_reference) => {
                            tracing::debug!("Channel {} references message {}", channel_name, msg_name);
                        }
                    }
                }
            }
            Either::Left(reference) => {
                tracing::debug!("Skipping channel reference: {}", reference.reference);
            }
        }
    }

    Ok(schemas)
}

fn build_message_definition(
    message: &asyncapiv3::spec::message::Message,
    message_name: &str,
) -> Result<Option<MessageDefinition>> {
    let schema = match extract_schema_from_message(message, message_name)? {
        Some(schema) => schema,
        None => return Ok(None),
    };

    let mut examples: Vec<Value> = Vec::new();
    for example in &message.examples {
        if !example.payload.is_empty() {
            let value = serde_json::to_value(&example.payload)
                .context("Failed to serialize AsyncAPI message example payload")?;
            examples.push(value);
        }
    }

    if examples.is_empty() {
        examples = generate_example_from_schema(&schema)?;
    }

    Ok(Some(MessageDefinition { schema, examples }))
}

/// Extract JSON Schema from an AsyncAPI Message object
fn extract_schema_from_message(
    message: &asyncapiv3::spec::message::Message,
    message_name: &str,
) -> Result<Option<Value>> {
    use asyncapiv3::spec::common::Either;

    let payload = match &message.payload {
        Some(payload_ref_or) => payload_ref_or,
        None => {
            tracing::debug!("Message {} has no payload", message_name);
            return Ok(None);
        }
    };

    match payload {
        Either::Right(schema_or_multiformat) => match schema_or_multiformat {
            Either::Left(schema) => {
                let schema_json =
                    serde_json::to_value(schema).context("Failed to serialize schemars::Schema to JSON")?;
                Ok(Some(schema_json))
            }
            Either::Right(multi_format) => Ok(Some(multi_format.schema.clone())),
        },
        Either::Left(reference) => {
            tracing::debug!(
                "Message {} payload is a reference: {}",
                message_name,
                reference.reference
            );
            // TODO: Implement reference resolution
            Ok(None)
        }
    }
}

/// Protocol types supported by AsyncAPI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    WebSocket,
    Sse,
    Http,
    Kafka,
    Mqtt,
    Amqp,
    Other,
}

impl Protocol {
    /// Detect protocol from AsyncAPI server definition
    pub fn from_protocol_string(protocol: &str) -> Self {
        match protocol.to_lowercase().as_str() {
            "ws" | "wss" | "websocket" | "websockets" => Protocol::WebSocket,
            "sse" | "server-sent-events" => Protocol::Sse,
            "http" | "https" => Protocol::Http,
            "kafka" => Protocol::Kafka,
            "mqtt" => Protocol::Mqtt,
            "amqp" => Protocol::Amqp,
            _ => Protocol::Other,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::WebSocket => "websocket",
            Protocol::Sse => "sse",
            Protocol::Http => "http",
            Protocol::Kafka => "kafka",
            Protocol::Mqtt => "mqtt",
            Protocol::Amqp => "amqp",
            Protocol::Other => "other",
        }
    }
}

/// Determine primary protocol from AsyncAPI spec
pub fn detect_primary_protocol(spec: &AsyncApiV3Spec) -> Result<Protocol> {
    use asyncapiv3::spec::common::Either;

    for server_or_ref in spec.servers.values() {
        match server_or_ref {
            Either::Right(server) => {
                let protocol = Protocol::from_protocol_string(&server.protocol);
                tracing::debug!("Detected protocol: {:?} from '{}'", protocol, server.protocol);
                return Ok(protocol);
            }
            Either::Left(_reference) => {
                // TODO: Implement reference resolution
                tracing::debug!("Skipping server reference");
            }
        }
    }

    tracing::warn!("Could not determine protocol from spec, defaulting to WebSocket");
    Ok(Protocol::WebSocket)
}

/// Generate fixture files from message schemas
///
/// Creates JSON fixture files in the output directory for each message type
pub fn generate_fixtures(spec: &AsyncApiV3Spec, output_dir: &Path, protocol: Protocol) -> Result<Vec<PathBuf>> {
    let schemas = extract_message_schemas(spec)?;
    let (message_channels, alias_map) = collect_message_channels(spec);
    let message_operations = collect_message_operations(spec, &alias_map);

    if schemas.is_empty() {
        tracing::warn!("No message schemas found in AsyncAPI spec");
        return Ok(Vec::new());
    }

    let subdir = match protocol {
        Protocol::WebSocket => "websockets",
        Protocol::Sse => "sse",
        Protocol::Http => "http",
        _ => "asyncapi",
    };

    let target_dir = output_dir.join(subdir);
    fs::create_dir_all(&target_dir).with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    let mut generated_paths = Vec::new();

    for (message_name, definition) in &schemas {
        let fixture_path = target_dir.join(format!("{}.json", message_name));

        let channel = message_channels.get(message_name).cloned();
        let operations = message_operations
            .get(message_name)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|meta| {
                serde_json::json!({
                    "name": meta.name,
                    "action": meta.action,
                    "replies": meta.replies,
                })
            })
            .collect::<Vec<_>>();
        let fixture = serde_json::json!({
            "name": message_name,
            "description": format!("Test fixture for {} message", message_name),
            "protocol": protocol.as_str(),
            "channel": channel,
            "schema": definition.schema,
            "examples": definition.examples,
            "operations": operations,
        });

        let fixture_json = serde_json::to_string_pretty(&fixture).context("Failed to serialize fixture to JSON")?;

        fs::write(&fixture_path, fixture_json)
            .with_context(|| format!("Failed to write fixture: {}", fixture_path.display()))?;

        println!("  Generated: {}", fixture_path.display());
        generated_paths.push(fixture_path);
    }

    Ok(generated_paths)
}

/// Generate example data from JSON Schema
///
/// Creates a simple valid example based on the schema properties
fn generate_example_from_schema(schema: &Value) -> Result<Vec<Value>> {
    let mut examples = Vec::new();

    if let Some(schema_examples) = schema.get("examples").and_then(|e| e.as_array()) {
        examples.extend(schema_examples.clone());
    }

    if examples.is_empty()
        && schema
            .get("type")
            .and_then(|value| value.as_str())
            .map(|ty| ty.eq_ignore_ascii_case("array"))
            .unwrap_or(false)
    {
        if let Some(items) = schema.get("items") {
            let generated = generate_example_from_schema(items)?;
            let template = generated
                .into_iter()
                .next()
                .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
            let min_items = schema.get("minItems").and_then(|value| value.as_u64()).unwrap_or(1);
            let mut target_len = usize::try_from(min_items).unwrap_or(usize::MAX);
            if target_len == 0 {
                target_len = 1;
            }
            let capped_len = target_len.min(5);
            let mut array_values = Vec::new();
            for _ in 0..capped_len {
                array_values.push(template.clone());
            }
            examples.push(Value::Array(array_values));
        } else {
            examples.push(Value::Array(vec![]));
        }
    }

    if examples.is_empty()
        && let Some(obj) = schema.get("properties").and_then(|p| p.as_object())
    {
        let mut example = serde_json::Map::new();

        for (prop_name, prop_schema) in obj {
            let example_value = if let Some(const_val) = prop_schema.get("const") {
                const_val.clone()
            } else if let Some(type_str) = prop_schema.get("type").and_then(|t| t.as_str()) {
                match type_str {
                    "string" => {
                        if let Some(format) = prop_schema.get("format").and_then(|f| f.as_str()) {
                            match format {
                                "date-time" => Value::String("2024-01-15T10:30:00Z".to_string()),
                                "date" => Value::String("2024-01-15".to_string()),
                                "time" => Value::String("10:30:00".to_string()),
                                "email" => Value::String("example@example.com".to_string()),
                                "uri" => Value::String("https://example.com".to_string()),
                                _ => Value::String(format!("example_{}", prop_name)),
                            }
                        } else {
                            Value::String(format!("example_{}", prop_name))
                        }
                    }
                    "number" | "integer" => Value::Number(serde_json::Number::from(42)),
                    "boolean" => Value::Bool(true),
                    "array" => Value::Array(vec![]),
                    "object" => Value::Object(serde_json::Map::new()),
                    _ => Value::Null,
                }
            } else {
                Value::Null
            };

            example.insert(prop_name.clone(), example_value);
        }

        examples.push(Value::Object(example));
    }

    if examples.is_empty() {
        examples.push(serde_json::json!({}));
    }

    Ok(examples)
}

/// Generate Python test application from AsyncAPI spec
///
/// Creates a Python WebSocket/SSE test application with message handlers
pub fn generate_python_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let mut code = String::new();

    code.push_str("#!/usr/bin/env python3\n");
    code.push_str("\"\"\"Test application generated from AsyncAPI specification\"\"\"\n\n");
    code.push_str("import asyncio\n");
    code.push_str("import json\n");
    code.push_str("from pathlib import Path\n");
    code.push_str("from typing import Any, Dict\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str("import websockets\n");
            code.push_str("from websockets.client import WebSocketClientProtocol\n\n");
        }
        Protocol::Sse => {
            code.push_str("import aiohttp\n\n");
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported protocol for Python test app: {:?}",
                protocol
            ));
        }
    }

    let channels = extract_channel_info(spec)?;

    code.push_str("# Load test fixtures\n");
    code.push_str("FIXTURES_DIR = Path(__file__).parent.parent / \"testing_data\" / \"");
    code.push_str(match protocol {
        Protocol::WebSocket => "websockets",
        Protocol::Sse => "sse",
        _ => "asyncapi",
    });
    code.push_str("\"\n\n");

    code.push_str("def load_fixture(name: str) -> Dict[str, Any]:\n");
    code.push_str("    \"\"\"Load a test fixture by name\"\"\"\n");
    code.push_str("    fixture_path = FIXTURES_DIR / f\"{name}.json\"\n");
    code.push_str("    if not fixture_path.exists():\n");
    code.push_str("        raise FileNotFoundError(f\"Fixture not found: {fixture_path}\")\n");
    code.push_str("    with open(fixture_path) as f:\n");
    code.push_str("        return json.load(f)\n\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    code.push_str("\n\nif __name__ == \"__main__\":\n");
    code.push_str("    asyncio.run(main())\n");

    Ok(code)
}

/// Channel information extracted from AsyncAPI spec.
///
/// Contains metadata about a single channel including its name, address path,
/// associated messages, and operations. Used during code generation for test applications.
///
/// NOTE: Marked with #[allow(dead_code)] because some fields may not be accessed
/// in all code generation paths, but the struct is constructed and stored in vectors
/// for comprehensive channel documentation.
#[derive(Debug)]
#[allow(dead_code)]
struct ChannelInfo {
    name: String,
    path: String,
    messages: Vec<String>,
    operations: Vec<ChannelOperation>,
}

/// Extract channel information from AsyncAPI spec
fn extract_channel_info(spec: &AsyncApiV3Spec) -> Result<Vec<ChannelInfo>> {
    use asyncapiv3::spec::common::Either;

    let mut channels = Vec::new();
    let operation_map = collect_channel_operations(spec);

    for (channel_path, channel_ref_or) in &spec.channels {
        match channel_ref_or {
            Either::Right(channel) => {
                let messages: Vec<String> = channel.messages.keys().cloned().collect();
                let raw_path = channel.address.clone().unwrap_or_else(|| channel_path.clone());
                let normalized_path = if raw_path.starts_with('/') {
                    raw_path.clone()
                } else {
                    format!("/{}", raw_path)
                };
                let operations = operation_map.get(&normalized_path).cloned().unwrap_or_default();

                channels.push(ChannelInfo {
                    name: channel_path.trim_start_matches('/').replace('/', "_"),
                    path: normalized_path,
                    messages,
                    operations,
                });
            }
            Either::Left(_reference) => {
                tracing::debug!("Skipping channel reference: {}", channel_path);
            }
        }
    }

    Ok(channels)
}

fn collect_message_channels(spec: &AsyncApiV3Spec) -> (HashMap<String, String>, HashMap<String, String>) {
    use asyncapiv3::spec::common::Either;

    let mut map = HashMap::new();
    let mut aliases = HashMap::new();

    for (channel_path, channel_ref_or) in &spec.channels {
        let address = match channel_ref_or {
            Either::Right(channel) => channel.address.clone().unwrap_or_else(|| channel_path.clone()),
            Either::Left(_) => continue,
        };
        let normalized_address = if address.starts_with('/') {
            address.clone()
        } else {
            format!("/{}", address)
        };

        if let Either::Right(channel) = channel_ref_or {
            for (message_name, message_ref) in &channel.messages {
                let slug = channel_path.trim_start_matches('/').replace('/', "_");
                let inline_key = format!("{}_{}", slug, message_name);
                match message_ref {
                    Either::Right(_) => {
                        map.entry(inline_key.clone())
                            .or_insert_with(|| normalized_address.clone());
                    }
                    Either::Left(reference) => {
                        let target =
                            resolve_message_from_ref(&reference.reference).unwrap_or_else(|| message_name.clone());
                        map.entry(target.clone()).or_insert_with(|| normalized_address.clone());
                        aliases.insert(inline_key, target);
                    }
                }
            }
        }
    }

    (map, aliases)
}

fn build_typescript_dtos(messages: &HashMap<String, MessageDefinition>) -> Result<HashMap<String, TypeScriptDto>> {
    let mut map = HashMap::new();
    for (name, definition) in messages {
        let dto = generate_typescript_dto(name, &definition.schema)?;
        map.insert(name.clone(), dto);
    }
    Ok(map)
}

fn collect_message_operations(
    spec: &AsyncApiV3Spec,
    aliases: &HashMap<String, String>,
) -> HashMap<String, Vec<MessageOperationMetadata>> {
    use asyncapiv3::spec::common::Either;

    let mut map: HashMap<String, Vec<MessageOperationMetadata>> = HashMap::new();

    for (op_name, operation_ref) in &spec.operations {
        let operation = match operation_ref {
            Either::Right(op) => op,
            Either::Left(_) => continue,
        };

        let replies: Vec<String> = if let Some(Either::Right(reply)) = &operation.reply {
            reply
                .messages
                .iter()
                .filter_map(|reference| resolve_message_from_ref(&reference.reference))
                .collect()
        } else {
            Vec::new()
        };

        if let Some(message_refs) = &operation.messages {
            for reference in message_refs {
                if let Some(name) = resolve_message_from_ref(&reference.reference) {
                    let resolved_name = aliases.get(&name).cloned().unwrap_or(name.clone());
                    map.entry(resolved_name).or_default().push(MessageOperationMetadata {
                        name: op_name.clone(),
                        action: operation_action_name(&operation.action).to_string(),
                        replies: replies.clone(),
                    });
                }
            }
        }
    }

    map
}

fn collect_channel_operations(spec: &AsyncApiV3Spec) -> HashMap<String, Vec<ChannelOperation>> {
    use asyncapiv3::spec::common::Either;

    let mut map: HashMap<String, Vec<ChannelOperation>> = HashMap::new();

    for (op_name, operation_ref) in &spec.operations {
        let operation = match operation_ref {
            Either::Right(op) => op,
            Either::Left(_) => continue,
        };

        let channel_path = match resolve_channel_from_ref(&operation.channel.reference) {
            Some(path) => path,
            None => continue,
        };

        let messages = operation
            .messages
            .as_ref()
            .map(|refs| {
                refs.iter()
                    .filter_map(|reference| resolve_message_from_ref(&reference.reference))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let replies = if let Some(Either::Right(reply)) = &operation.reply {
            reply
                .messages
                .iter()
                .filter_map(|reference| resolve_message_from_ref(&reference.reference))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        map.entry(channel_path.clone()).or_default().push(ChannelOperation {
            name: op_name.clone(),
            action: operation_action_name(&operation.action).to_string(),
            messages,
            replies,
        });
    }

    map
}

fn sanitize_identifier(name: &str) -> String {
    let mut ident: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }

    ident = ident.trim_matches('_').to_string();

    if ident.is_empty() {
        return "handler".to_string();
    }

    if ident.chars().next().unwrap().is_ascii_digit() {
        ident.insert(0, '_');
    }

    ident
}

fn camel_identifier(name: &str) -> String {
    let base = sanitize_identifier(name);
    let mut result = String::new();
    for part in base.split('_').filter(|segment| !segment.is_empty()) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            result.push(first.to_ascii_uppercase());
            result.push_str(chars.as_str());
        }
    }
    if result.is_empty() {
        "Handler".to_string()
    } else {
        result
    }
}

fn escape_rust_string(input: &str) -> String {
    input.escape_default().to_string()
}

/// Generate WebSocket message handlers for Python
fn generate_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("def validate_message(message: Dict[str, Any], fixture_name: str) -> bool:\n");
    code.push_str("    \"\"\"Validate message against fixture schema\"\"\"\n");
    code.push_str("    try:\n");
    code.push_str("        fixture = load_fixture(fixture_name)\n");
    code.push_str("        schema = fixture.get('schema', {})\n");
    code.push_str("        # Basic validation - check required fields\n");
    code.push_str("        required = schema.get('required', [])\n");
    code.push_str("        for field in required:\n");
    code.push_str("            if field not in message:\n");
    code.push_str("                print(f\"❌ Missing required field: {field}\")\n");
    code.push_str("                return False\n");
    code.push_str("        print(f\"✓ Message validated against {fixture_name}\")\n");
    code.push_str("        return True\n");
    code.push_str("    except Exception as e:\n");
    code.push_str("        print(f\"❌ Validation error: {e}\")\n");
    code.push_str("        return False\n\n\n");

    code.push_str("async def handle_websocket(uri: str) -> None:\n");
    code.push_str("    \"\"\"Connect to WebSocket and handle messages\"\"\"\n");
    code.push_str("    print(f\"Connecting to {uri}...\")\n");
    code.push_str("    \n");
    code.push_str("    async with websockets.connect(uri) as websocket:\n");
    code.push_str("        print(\"✓ Connected\")\n");
    code.push_str("        \n");

    let schemas = extract_message_schemas(spec)?;

    if !schemas.is_empty() {
        code.push_str("        # Send example messages\n");
        for msg_name in schemas.keys() {
            code.push_str(&format!(
                "        fixture_{} = load_fixture(\"{}\")\n",
                msg_name.replace('-', "_"),
                msg_name
            ));
            code.push_str(&format!(
                "        example_{} = fixture_{}['examples'][0]\n",
                msg_name.replace('-', "_"),
                msg_name.replace('-', "_")
            ));
            code.push_str(&format!("        print(f\"Sending {} message...\")\n", msg_name));
            code.push_str(&format!(
                "        await websocket.send(json.dumps(example_{}))\n",
                msg_name.replace('-', "_")
            ));
            code.push_str("        \n");
        }

        code.push_str("        # Receive and validate messages\n");
        code.push_str("        try:\n");
        code.push_str("            async for message in websocket:\n");
        code.push_str("                data = json.loads(message)\n");
        code.push_str("                msg_type = data.get('type', 'unknown')\n");
        code.push_str("                print(f\"Received message type: {msg_type}\")\n");
        code.push_str("                \n");
        code.push_str("                # Validate based on message type\n");

        for msg_name in schemas.keys() {
            code.push_str(&format!("                if msg_type == '{}':\n", msg_name));
            code.push_str(&format!("                    validate_message(data, '{}')\n", msg_name));
        }

        code.push_str("        except websockets.exceptions.ConnectionClosed:\n");
        code.push_str("            print(\"Connection closed\")\n");
    }

    code.push_str("\n\n");

    code.push_str("async def main() -> None:\n");
    code.push_str("    \"\"\"Main entry point\"\"\"\n");
    code.push_str("    # Default WebSocket URI - override with environment variable WS_URI\n");
    code.push_str("    import os\n");
    code.push_str("    uri = os.getenv('WS_URI', 'ws://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("')\n");
    code.push_str("    await handle_websocket(uri)\n");

    Ok(code)
}

/// Generate Node.js test application from AsyncAPI spec
///
/// Creates a TypeScript WebSocket/SSE test application with message handlers
pub fn generate_nodejs_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let mut code = String::new();

    code.push_str("#!/usr/bin/env node\n");
    code.push_str("/**\n");
    code.push_str(" * Test application generated from AsyncAPI specification\n");
    code.push_str(" */\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str("import WebSocket from 'ws';\n");
        }
        Protocol::Sse => {
            code.push_str("import fetch from 'node-fetch';\n");
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported protocol for Node.js test app: {:?}",
                protocol
            ));
        }
    }

    code.push_str("import { readFileSync } from 'node:fs';\n");
    code.push_str("import { join, dirname } from 'node:path';\n");
    code.push_str("import { fileURLToPath } from 'node:url';\n\n");

    let channels = extract_channel_info(spec)?;

    code.push_str("// ES modules compatibility\n");
    code.push_str("const __filename = fileURLToPath(import.meta.url);\n");
    code.push_str("const __dirname = dirname(__filename);\n\n");

    code.push_str("// Load test fixtures\n");
    code.push_str("const FIXTURES_DIR = join(__dirname, '..', '..', 'testing_data', '");
    code.push_str(match protocol {
        Protocol::WebSocket => "websockets",
        Protocol::Sse => "sse",
        _ => "asyncapi",
    });
    code.push_str("');\n\n");

    code.push_str("type FixtureSchema = {\n");
    code.push_str("  required?: string[];\n");
    code.push_str("};\n\n");

    code.push_str("type Fixture = {\n");
    code.push_str("  name: string;\n");
    code.push_str("  description?: string;\n");
    code.push_str("  channel?: string;\n");
    code.push_str("  protocol?: string;\n");
    code.push_str("  schema: FixtureSchema;\n");
    code.push_str("  examples: unknown[];\n");
    code.push_str("};\n\n");

    code.push_str("type MessageRecord = Record<string, unknown>;\n\n");

    code.push_str("function loadFixture(name: string): Fixture {\n");
    code.push_str("  const fixturePath = join(FIXTURES_DIR, `${name}.json`);\n");
    code.push_str("  const content = readFileSync(fixturePath, 'utf-8');\n");
    code.push_str("  return JSON.parse(content) as Fixture;\n");
    code.push_str("}\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_nodejs_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_nodejs_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    code.push_str("\n// Run main function\n");
    code.push_str("main().catch(console.error);\n");

    Ok(code)
}

/// Generate Node.js WebSocket message handlers
fn generate_nodejs_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("function validateMessage(message: MessageRecord, fixtureName: string): boolean {\n");
    code.push_str("  try {\n");
    code.push_str("    const fixture = loadFixture(fixtureName);\n");
    code.push_str("    const required = fixture.schema.required || [];\n");
    code.push_str("    \n");
    code.push_str("    // Basic validation - check required fields\n");
    code.push_str("    for (const field of required) {\n");
    code.push_str("      if (!(field in message)) {\n");
    code.push_str("        console.log(`❌ Missing required field: ${field}`);\n");
    code.push_str("        return false;\n");
    code.push_str("      }\n");
    code.push_str("    }\n");
    code.push_str("    \n");
    code.push_str("    console.log(`✓ Message validated against ${fixtureName}`);\n");
    code.push_str("    return true;\n");
    code.push_str("  } catch (error) {\n");
    code.push_str("    console.log(`❌ Validation error: ${error}`);\n");
    code.push_str("    return false;\n");
    code.push_str("  }\n");
    code.push_str("}\n\n");

    code.push_str("async function handleWebSocket(uri: string): Promise<void> {\n");
    code.push_str("  console.log(`Connecting to ${uri}...`);\n");
    code.push_str("  \n");
    code.push_str("  return new Promise((resolve, reject) => {\n");
    code.push_str("    const ws = new WebSocket(uri);\n");
    code.push_str("    \n");
    code.push_str("    ws.on('open', () => {\n");
    code.push_str("      console.log('✓ Connected');\n");
    code.push_str("      \n");

    let schemas = extract_message_schemas(spec)?;

    if !schemas.is_empty() {
        code.push_str("      // Send example messages\n");
        for msg_name in schemas.keys() {
            let safe_name = msg_name.replace('-', "_");
            code.push_str(&format!(
                "      const fixture_{} = loadFixture('{}');\n",
                safe_name, msg_name
            ));
            code.push_str(&format!(
                "      const example_{} = fixture_{}.examples[0];\n",
                safe_name, safe_name
            ));
            code.push_str(&format!("      console.log('Sending {} message...');\n", msg_name));
            code.push_str(&format!("      ws.send(JSON.stringify(example_{}));\n", safe_name));
            code.push_str("      \n");
        }
    }

    code.push_str("    });\n");
    code.push_str("    \n");
    code.push_str("    ws.on('message', (data: WebSocket.Data) => {\n");
    code.push_str("      const message = JSON.parse(data.toString()) as MessageRecord;\n");
    code.push_str("      const rawType = message.type;\n");
    code.push_str("      const msgType = typeof rawType === 'string' ? rawType : 'unknown';\n");
    code.push_str("      console.log(`Received message type: ${msgType}`);\n");
    code.push_str("      \n");
    code.push_str("      // Validate based on message type\n");

    for msg_name in schemas.keys() {
        code.push_str(&format!("      if (msgType === '{}') {{\n", msg_name));
        code.push_str(&format!("        validateMessage(message, '{}');\n", msg_name));
        code.push_str("      }\n");
    }

    code.push_str("    });\n");
    code.push_str("    \n");
    code.push_str("    ws.on('close', () => {\n");
    code.push_str("      console.log('Connection closed');\n");
    code.push_str("      resolve();\n");
    code.push_str("    });\n");
    code.push_str("    \n");
    code.push_str("    ws.on('error', (error) => {\n");
    code.push_str("      console.error('WebSocket error:', error);\n");
    code.push_str("      reject(error);\n");
    code.push_str("    });\n");
    code.push_str("  });\n");
    code.push_str("}\n\n");

    code.push_str("async function main(): Promise<void> {\n");
    code.push_str("  // Default WebSocket URI - override with environment variable WS_URI\n");
    code.push_str("  const uri = process.env.WS_URI || 'ws://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("';\n");
    code.push_str("  await handleWebSocket(uri);\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generate Node.js SSE message handlers
fn generate_nodejs_sse_handlers(_spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("async function handleSSE(url: string): Promise<void> {\n");
    code.push_str("  console.log(`Connecting to ${url}...`);\n");
    code.push_str("  \n");
    code.push_str("  const response = await fetch(url);\n");
    code.push_str("  console.log('✓ Connected');\n");
    code.push_str("  \n");
    code.push_str("  const reader = response.body?.getReader();\n");
    code.push_str("  const decoder = new TextDecoder();\n");
    code.push_str("  \n");
    code.push_str("  while (reader) {\n");
    code.push_str("    const { done, value } = await reader.read();\n");
    code.push_str("    if (done) break;\n");
    code.push_str("    \n");
    code.push_str("    const chunk = decoder.decode(value);\n");
    code.push_str("    const lines = chunk.split('\\n');\n");
    code.push_str("    \n");
    code.push_str("    for (const line of lines) {\n");
    code.push_str("      if (line.startsWith('data:')) {\n");
    code.push_str("        const data = line.slice(5).trim();\n");
    code.push_str("        try {\n");
    code.push_str("          const message = JSON.parse(data) as MessageRecord;\n");
    code.push_str("          console.log('Received event:', message);\n");
    code.push_str("        } catch {\n");
    code.push_str("          console.log('Received:', data);\n");
    code.push_str("        }\n");
    code.push_str("      }\n");
    code.push_str("    }\n");
    code.push_str("  }\n");
    code.push_str("}\n\n");

    code.push_str("async function main(): Promise<void> {\n");
    code.push_str("  // Default SSE URI - override with environment variable SSE_URI\n");
    code.push_str("  const url = process.env.SSE_URI || 'http://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("';\n");
    code.push_str("  await handleSSE(url);\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generate Ruby test application from AsyncAPI spec
///
/// Creates a Ruby WebSocket/SSE test application with message handlers
pub fn generate_ruby_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let mut code = String::new();

    code.push_str("#!/usr/bin/env ruby\n");
    code.push_str("# frozen_string_literal: true\n\n");
    code.push_str("# Test application generated from AsyncAPI specification\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str("require 'faye/websocket'\n");
            code.push_str("require 'eventmachine'\n");
        }
        Protocol::Sse => {
            code.push_str("require 'net/http'\n");
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported protocol for Ruby test app: {:?}",
                protocol
            ));
        }
    }

    code.push_str("require 'json'\n");
    code.push_str("require 'pathname'\n\n");

    let channels = extract_channel_info(spec)?;

    code.push_str("# Load test fixtures\n");
    code.push_str("FIXTURES_DIR = Pathname.new(__FILE__).parent.parent + 'testing_data' + '");
    code.push_str(match protocol {
        Protocol::WebSocket => "websockets",
        Protocol::Sse => "sse",
        _ => "asyncapi",
    });
    code.push_str("'\n\n");

    code.push_str("def load_fixture(name)\n");
    code.push_str("  fixture_path = FIXTURES_DIR + \"#{name}.json\"\n");
    code.push_str("  raise \"Fixture not found: #{fixture_path}\" unless fixture_path.exist?\n");
    code.push_str("  JSON.parse(fixture_path.read)\n");
    code.push_str("end\n\n");

    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_ruby_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_ruby_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    code.push_str("\n# Run main function\n");
    code.push_str("main\n");

    Ok(code)
}

/// Generate Python handler scaffolding from AsyncAPI spec
pub fn generate_python_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    if channels.is_empty() {
        bail!("AsyncAPI spec does not define any channels");
    }

    match protocol {
        Protocol::WebSocket | Protocol::Sse => {}
        other => {
            bail!("Protocol {:?} is not supported for Python handler generation", other);
        }
    }

    let mut code = String::new();
    code.push_str("#!/usr/bin/env python3\n");
    code.push_str("\"\"\"AsyncAPI handler skeleton generated by Spikard.\"\"\"\n\n");
    code.push_str("from typing import Any\n");
    if protocol == Protocol::Sse {
        code.push_str("import asyncio\n");
    }
    match protocol {
        Protocol::WebSocket => code.push_str("from spikard import Spikard, websocket\n"),
        Protocol::Sse => code.push_str("from spikard import Spikard, sse\n"),
        _ => {}
    }
    code.push_str("\napp = Spikard()\n\n");

    for channel in &channels {
        let handler_name = format!("{}_handler", sanitize_identifier(&channel.name));
        let message_description = if channel.messages.is_empty() {
            "messages".to_string()
        } else {
            channel.messages.join(", ")
        };

        match protocol {
            Protocol::WebSocket => {
                code.push_str(&format!("@websocket(\"{}\")\n", channel.path));
                code.push_str(&format!(
                    "async def {}(message: dict[str, Any]) -> dict[str, Any]:\n",
                    handler_name
                ));
                code.push_str(&format!(
                    "    \"\"\"Handles {} on {}.\"\"\"\n",
                    message_description, channel.path
                ));
                code.push_str("    raise NotImplementedError(\"Implement WebSocket message handling logic\")\n\n");
            }
            Protocol::Sse => {
                code.push_str(&format!("@sse(\"{}\")\n", channel.path));
                code.push_str(&format!("async def {}() -> Any:\n", handler_name));
                code.push_str(&format!(
                    "    \"\"\"Streams events for {} on {}.\"\"\"\n",
                    message_description, channel.path
                ));
                code.push_str("    yield {\"message\": \"replace with real event\"}\n\n");
            }
            _ => {}
        }
    }

    code.push_str("if __name__ == \"__main__\":\n");
    code.push_str("    app.run()\n");

    Ok(code)
}

/// Generate Node.js handler scaffolding from AsyncAPI spec
pub fn generate_nodejs_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    if channels.is_empty() {
        bail!("AsyncAPI spec does not define any channels");
    }

    let schema_map = extract_message_schemas(spec)?;
    let dto_map = build_typescript_dtos(&schema_map)?;

    match protocol {
        Protocol::WebSocket | Protocol::Sse => {}
        other => bail!("Protocol {:?} is not supported for Node.js handler generation", other),
    }

    let mut code = String::new();
    code.push_str("/**\n * AsyncAPI handler skeleton generated by Spikard CLI.\n */\n\n");
    code.push_str("import type { RouteMetadata, SpikardApp } from \"spikard\";\n");
    if protocol == Protocol::Sse {
        code.push_str("import { StreamingResponse } from \"spikard\";\n");
    }
    code.push_str("import { z } from \"zod\";\n");
    code.push('\n');

    let mut dto_declarations = String::new();
    for dto in dto_map.values() {
        dto_declarations.push_str(&dto.schema_declaration);
        dto_declarations.push('\n');
        dto_declarations.push_str(&dto.type_declaration);
        dto_declarations.push('\n');
    }
    if !dto_declarations.is_empty() {
        code.push_str(&dto_declarations);
        code.push('\n');
    }

    let mut route_entries = Vec::new();
    let mut handler_entries = Vec::new();

    for channel in &channels {
        let handler_name = format!("handle{}", camel_identifier(&channel.name));
        let message_description = if channel.messages.is_empty() {
            "messages".to_string()
        } else {
            channel.messages.join(", ")
        };

        match protocol {
            Protocol::WebSocket => {
                let dto = channel.messages.iter().find_map(|name| dto_map.get(name)).cloned();

                code.push_str(&format!(
                    "async function {}(message: unknown): Promise<string> {{\n",
                    handler_name
                ));
                if let Some(dto) = dto {
                    code.push_str(&format!(
                        "  const payload: {} = {}.parse(typeof message === \"string\" ? JSON.parse(message) : message);\n",
                        dto.type_ident, dto.schema_ident
                    ));
                } else {
                    code.push_str("  const payload = typeof message === \"string\" ? JSON.parse(message) : message;\n");
                }
                code.push_str(&format!(
                    "  // TODO: Handle {} for {}\n",
                    message_description, channel.path
                ));
                code.push_str("  return JSON.stringify(payload);\n");
                code.push_str("}\n\n");
            }
            Protocol::Sse => {
                code.push_str(&format!(
                    "async function {}(): Promise<StreamingResponse> {{\n",
                    handler_name
                ));
                code.push_str("  async function* eventStream() {\n");
                if let Some(dto) = channel.messages.iter().find_map(|name| dto_map.get(name)) {
                    code.push_str(&format!(
                        "    const sample: {} = {}.parse({{}});\n",
                        dto.type_ident, dto.schema_ident
                    ));
                    code.push_str("    yield `data: ${JSON.stringify(sample)}\\n\\n`;\n");
                } else {
                    code.push_str("    yield \"data: {\\\"message\\\": \\\"replace with event\\\"}\\n\\n\";\n");
                }
                code.push_str("  }\n");
                code.push_str(
                    "  return new StreamingResponse(eventStream(), { statusCode: 200, headers: { \"content-type\": \"text/event-stream\", \"cache-control\": \"no-cache\" } });\n",
                );
                code.push_str("}\n\n");
            }
            _ => {}
        }

        route_entries.push(format!(
            "{{ method: \"GET\", path: \"{}\", handler_name: \"{}\", is_async: true }}",
            channel.path, handler_name
        ));
        handler_entries.push(format!("{}: {}", handler_name, handler_name));
    }

    code.push_str("const routes: RouteMetadata[] = [\n    ");
    code.push_str(&route_entries.join(",\n    "));
    code.push_str("\n];\n\n");
    code.push_str("const handlers = {\n    ");
    code.push_str(&handler_entries.join(",\n    "));
    code.push_str("\n};\n\n");
    code.push_str("export function createAsyncApiHandlers(): SpikardApp {\n");
    code.push_str("  return { routes, handlers };\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generate Ruby handler scaffolding from AsyncAPI spec
pub fn generate_ruby_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    if channels.is_empty() {
        bail!("AsyncAPI spec does not define any channels");
    }

    match protocol {
        Protocol::WebSocket | Protocol::Sse => {}
        other => bail!("Protocol {:?} is not supported for Ruby handler generation", other),
    }

    let mut code = String::new();
    code.push_str("#!/usr/bin/env ruby\n");
    code.push_str("# frozen_string_literal: true\n\n");
    code.push_str("require \"spikard\"\n\n");
    code.push_str("app = Spikard::App.new\n\n");

    for channel in &channels {
        let handler_name = sanitize_identifier(&channel.name);
        let message_description = if channel.messages.is_empty() {
            "messages".to_string()
        } else {
            channel.messages.join(", ")
        };

        match protocol {
            Protocol::WebSocket => {
                code.push_str(&format!(
                    "app.websocket(\"{}\", handler_name: \"{}\") do\n",
                    channel.path, handler_name
                ));
                code.push_str("  handler = Object.new\n");
                code.push_str("  def handler.handle_message(message)\n");
                code.push_str(&format!(
                    "    # TODO: Handle {} for {}\n",
                    message_description, channel.path
                ));
                code.push_str("    message\n");
                code.push_str("  end\n");
                code.push_str("  handler\n");
                code.push_str("end\n\n");
            }
            Protocol::Sse => {
                code.push_str(&format!(
                    "app.get(\"{}\", handler_name: \"{}\") do |_request|\n",
                    channel.path, handler_name
                ));
                code.push_str("  stream = Enumerator.new do |yielder|\n");
                code.push_str("    yielder << \"data: {\\\"message\\\": \\\"replace with event\\\"}\\n\\n\"\n");
                code.push_str("  end\n\n");
                code.push_str(
                    "  Spikard::StreamingResponse.new(stream, status_code: 200, headers: { \"content-type\" => \"text/event-stream\", \"cache-control\" => \"no-cache\" })\n",
                );
                code.push_str("end\n\n");
            }
            _ => {}
        }
    }

    code.push_str("if $PROGRAM_NAME == __FILE__\n");
    code.push_str("  app.run\n");
    code.push_str("end\n");

    Ok(code)
}

/// Generate Rust handler scaffolding from AsyncAPI spec
pub fn generate_rust_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    if channels.is_empty() {
        bail!("AsyncAPI spec does not define any channels");
    }

    match protocol {
        Protocol::WebSocket | Protocol::Sse => {}
        other => bail!("Protocol {:?} is not supported for Rust handler generation", other),
    }

    let mut code = String::new();
    code.push_str("//! AsyncAPI handler skeleton generated by Spikard CLI.\n\n");
    match protocol {
        Protocol::WebSocket => {
            code.push_str("use serde_json::Value;\n");
            code.push_str("use spikard::{App, AppError, WebSocketHandler};\n\n");
        }
        Protocol::Sse => {
            code.push_str("use spikard::{App, AppError, SseEvent, SseEventProducer};\n\n");
        }
        _ => {}
    }

    let mut handler_defs = String::new();
    let mut registrations = String::new();

    for channel in &channels {
        let struct_name = format!(
            "{}{}",
            camel_identifier(&channel.name),
            match protocol {
                Protocol::WebSocket => "WebSocketHandler",
                Protocol::Sse => "SseProducer",
                _ => "",
            }
        );
        let path = escape_rust_string(&channel.path);
        match protocol {
            Protocol::WebSocket => {
                handler_defs.push_str(&format!("struct {};\n\n", struct_name));
                handler_defs.push_str(&format!("impl WebSocketHandler for {} {{\n", struct_name));
                handler_defs.push_str("    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send {\n");
                handler_defs.push_str("        async move {\n");
                handler_defs.push_str("            println!(\"Received message on {}: {:?}\", message);\n");
                handler_defs.push_str("            None\n");
                handler_defs.push_str("        }\n");
                handler_defs.push_str("    }\n");
                handler_defs.push_str("}\n\n");
                registrations.push_str(&format!("    app.websocket(\"{}\", {});\n", path, struct_name));
            }
            Protocol::Sse => {
                handler_defs.push_str(&format!("struct {};\n\n", struct_name));
                handler_defs.push_str(&format!("impl SseEventProducer for {} {{\n", struct_name));
                handler_defs.push_str(
                    "    fn next_event(&self) -> impl std::future::Future<Output = Option<SseEvent>> + Send {\n",
                );
                handler_defs.push_str("        async move {\n");
                handler_defs.push_str("            println!(\"Streaming SSE event on {}\");\n");
                handler_defs.push_str("            None\n");
                handler_defs.push_str("        }\n");
                handler_defs.push_str("    }\n");
                handler_defs.push_str("}\n\n");
                registrations.push_str(&format!("    app.sse(\"{}\", {});\n", path, struct_name));
            }
            _ => {}
        }
    }

    code.push_str(&handler_defs);
    code.push_str("fn build_app() -> Result<App, AppError> {\n");
    code.push_str("    let mut app = App::new();\n");
    code.push_str(&registrations);
    code.push_str("    Ok(app)\n");
    code.push_str("}\n\n");
    code.push_str("#[tokio::main]\n");
    code.push_str("async fn main() -> Result<(), AppError> {\n");
    code.push_str("    let app = build_app()?;\n");
    code.push_str("    app.run().await\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generate PHP handler scaffolding from AsyncAPI spec
pub fn generate_php_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    if channels.is_empty() {
        bail!("AsyncAPI spec does not define any channels");
    }

    match protocol {
        Protocol::WebSocket | Protocol::Sse => {}
        other => bail!("Protocol {:?} is not supported for PHP handler generation", other),
    }

    let mut code = String::new();
    code.push_str("<?php\n");
    code.push_str("declare(strict_types=1);\n\n");
    code.push_str("// AsyncAPI handler skeleton generated by Spikard CLI.\n\n");
    code.push_str("final class AsyncApiHandlers\n{\n");
    code.push_str("    public static function register(object $app): void\n    {\n");

    for channel in &channels {
        match protocol {
            Protocol::WebSocket => code.push_str(&format!(
                "        // TODO: register WebSocket route \"{}\"\n",
                channel.path
            )),
            Protocol::Sse => code.push_str(&format!("        // TODO: register SSE route \"{}\"\n", channel.path)),
            _ => {}
        }
    }

    code.push_str("    }\n\n");

    for channel in &channels {
        let suffix = camel_identifier(&channel.name);
        let message_description = if channel.messages.is_empty() {
            "messages".to_string()
        } else {
            channel.messages.join(", ")
        };

        match protocol {
            Protocol::WebSocket => {
                code.push_str(&format!(
                    "    public static function handle{}(array $message): array\n    {{\n",
                    suffix
                ));
                code.push_str(&format!(
                    "        // TODO: Handle {} received on {}\n",
                    message_description, channel.path
                ));
                code.push_str("        return $message;\n");
                code.push_str("    }\n\n");
            }
            Protocol::Sse => {
                code.push_str(&format!(
                    "    public static function stream{}(): iterable\n    {{\n",
                    suffix
                ));
                code.push_str(&format!(
                    "        // TODO: Emit SSE events for {}\n        yield \"data: {{\\\\\"message\\\\\": \\\\\"replace with event\\\\\"}}\\\\n\\\\n\";\n",
                    channel.path
                ));
                code.push_str("    }\n\n");
            }
            _ => {}
        }
    }

    code.push_str("}\n\n");
    code.push_str("if (PHP_SAPI === 'cli' && __FILE__ === realpath($_SERVER['SCRIPT_FILENAME'])) {\n");
    code.push_str("    // $app = new Spikard\\\\App();\n");
    code.push_str("    // AsyncApiHandlers::register($app);\n");
    code.push_str("    // $app->run();\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generate Ruby WebSocket message handlers
fn generate_ruby_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("def validate_message(message, fixture_name)\n");
    code.push_str("  fixture = load_fixture(fixture_name)\n");
    code.push_str("  schema = fixture['schema'] || {}\n");
    code.push_str("  required = schema['required'] || []\n");
    code.push_str("  \n");
    code.push_str("  # Basic validation - check required fields\n");
    code.push_str("  required.each do |field|\n");
    code.push_str("    unless message.key?(field)\n");
    code.push_str("      puts \"❌ Missing required field: #{field}\"\n");
    code.push_str("      return false\n");
    code.push_str("    end\n");
    code.push_str("  end\n");
    code.push_str("  \n");
    code.push_str("  puts \"✓ Message validated against #{fixture_name}\"\n");
    code.push_str("  true\n");
    code.push_str("rescue => e\n");
    code.push_str("  puts \"❌ Validation error: #{e.message}\"\n");
    code.push_str("  false\n");
    code.push_str("end\n\n");

    code.push_str("def handle_websocket(uri)\n");
    code.push_str("  puts \"Connecting to #{uri}...\"\n");
    code.push_str("  \n");
    code.push_str("  EM.run do\n");
    code.push_str("    ws = Faye::WebSocket::Client.new(uri)\n");
    code.push_str("    \n");
    code.push_str("    ws.on :open do |_event|\n");
    code.push_str("      puts '✓ Connected'\n");
    code.push_str("      \n");

    let schemas = extract_message_schemas(spec)?;

    if !schemas.is_empty() {
        code.push_str("      # Send example messages\n");
        for msg_name in schemas.keys() {
            let safe_name = msg_name.replace('-', "_");
            code.push_str(&format!("      fixture_{} = load_fixture('{}')\n", safe_name, msg_name));
            code.push_str(&format!(
                "      example_{} = fixture_{}['examples'][0]\n",
                safe_name, safe_name
            ));
            code.push_str(&format!("      puts 'Sending {} message...'\n", msg_name));
            code.push_str(&format!("      ws.send(JSON.generate(example_{}))\n", safe_name));
            code.push_str("      \n");
        }
    }

    code.push_str("    end\n");
    code.push_str("    \n");
    code.push_str("    ws.on :message do |event|\n");
    code.push_str("      message = JSON.parse(event.data)\n");
    code.push_str("      msg_type = message['type'] || 'unknown'\n");
    code.push_str("      puts \"Received message type: #{msg_type}\"\n");
    code.push_str("      \n");
    code.push_str("      # Validate based on message type\n");

    for msg_name in schemas.keys() {
        code.push_str(&format!(
            "      validate_message(message, '{}') if msg_type == '{}'\n",
            msg_name, msg_name
        ));
    }

    code.push_str("    end\n");
    code.push_str("    \n");
    code.push_str("    ws.on :close do |event|\n");
    code.push_str("      puts \"Connection closed: #{event.code} - #{event.reason}\"\n");
    code.push_str("      EM.stop\n");
    code.push_str("    end\n");
    code.push_str("    \n");
    code.push_str("    ws.on :error do |event|\n");
    code.push_str("      puts \"WebSocket error: #{event.message}\"\n");
    code.push_str("      EM.stop\n");
    code.push_str("    end\n");
    code.push_str("  end\n");
    code.push_str("end\n\n");

    code.push_str("def main\n");
    code.push_str("  # Default WebSocket URI - override with environment variable WS_URI\n");
    code.push_str("  uri = ENV['WS_URI'] || 'ws://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("'\n");
    code.push_str("  handle_websocket(uri)\n");
    code.push_str("end\n");

    Ok(code)
}

/// Generate Ruby SSE message handlers
fn generate_ruby_sse_handlers(_spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("def handle_sse(url)\n");
    code.push_str("  puts \"Connecting to #{url}...\"\n");
    code.push_str("  \n");
    code.push_str("  uri = URI(url)\n");
    code.push_str("  Net::HTTP.start(uri.host, uri.port) do |http|\n");
    code.push_str("    request = Net::HTTP::Get.new(uri)\n");
    code.push_str("    \n");
    code.push_str("    http.request(request) do |response|\n");
    code.push_str("      puts '✓ Connected'\n");
    code.push_str("      \n");
    code.push_str("      response.read_body do |chunk|\n");
    code.push_str("        chunk.each_line do |line|\n");
    code.push_str("          next unless line.start_with?('data:')\n");
    code.push_str("          \n");
    code.push_str("          data = line[5..-1].strip\n");
    code.push_str("          begin\n");
    code.push_str("            message = JSON.parse(data)\n");
    code.push_str("            puts \"Received event: #{message}\"\n");
    code.push_str("          rescue JSON::ParserError\n");
    code.push_str("            puts \"Received: #{data}\"\n");
    code.push_str("          end\n");
    code.push_str("        end\n");
    code.push_str("      end\n");
    code.push_str("    end\n");
    code.push_str("  end\n");
    code.push_str("end\n\n");

    code.push_str("def main\n");
    code.push_str("  # Default SSE URI - override with environment variable SSE_URI\n");
    code.push_str("  url = ENV['SSE_URI'] || 'http://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("'\n");
    code.push_str("  handle_sse(url)\n");
    code.push_str("end\n");

    Ok(code)
}

/// Generate SSE message handlers for Python
fn generate_sse_handlers(_spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    code.push_str("async def handle_sse(url: str) -> None:\n");
    code.push_str("    \"\"\"Connect to SSE endpoint and handle events\"\"\"\n");
    code.push_str("    print(f\"Connecting to {url}...\")\n");
    code.push_str("    \n");
    code.push_str("    async with aiohttp.ClientSession() as session:\n");
    code.push_str("        async with session.get(url) as response:\n");
    code.push_str("            print(\"✓ Connected\")\n");
    code.push_str("            async for line in response.content:\n");
    code.push_str("                line = line.decode('utf-8').strip()\n");
    code.push_str("                if line.startswith('data:'):\n");
    code.push_str("                    data = line[5:].strip()\n");
    code.push_str("                    try:\n");
    code.push_str("                        message = json.loads(data)\n");
    code.push_str("                        print(f\"Received event: {message}\")\n");
    code.push_str("                    except json.JSONDecodeError:\n");
    code.push_str("                        print(f\"Received: {data}\")\n\n\n");

    code.push_str("async def main() -> None:\n");
    code.push_str("    \"\"\"Main entry point\"\"\"\n");
    code.push_str("    # Default SSE URI - override with environment variable SSE_URI\n");
    code.push_str("    import os\n");
    code.push_str("    url = os.getenv('SSE_URI', 'http://localhost:8000");

    if let Some(first_channel) = channels.first() {
        code.push_str(&first_channel.path);
    }

    code.push_str("')\n");
    code.push_str("    await handle_sse(url)\n");

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detection() {
        assert_eq!(Protocol::from_protocol_string("ws"), Protocol::WebSocket);
        assert_eq!(Protocol::from_protocol_string("wss"), Protocol::WebSocket);
        assert_eq!(Protocol::from_protocol_string("websocket"), Protocol::WebSocket);
        assert_eq!(Protocol::from_protocol_string("sse"), Protocol::Sse);
        assert_eq!(Protocol::from_protocol_string("server-sent-events"), Protocol::Sse);
        assert_eq!(Protocol::from_protocol_string("http"), Protocol::Http);
        assert_eq!(Protocol::from_protocol_string("https"), Protocol::Http);
        assert_eq!(Protocol::from_protocol_string("kafka"), Protocol::Kafka);
        assert_eq!(Protocol::from_protocol_string("unknown"), Protocol::Other);
    }
}
