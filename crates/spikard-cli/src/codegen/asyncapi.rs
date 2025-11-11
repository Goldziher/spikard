//! AsyncAPI v3 specification parsing and code generation
//!
//! This module handles parsing AsyncAPI v3 specs and generating:
//! - Test fixtures for WebSocket/SSE messages
//! - Test applications for Python, Node.js, and Ruby
//!
//! AsyncAPI is the standard for describing event-driven APIs, similar to
//! how OpenAPI describes REST APIs.

use anyhow::{Context, Result};
use asyncapiv3::spec::{AsyncApiSpec, AsyncApiV3Spec};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parse an AsyncAPI v3 specification file
///
/// Supports both JSON and YAML formats
pub fn parse_asyncapi_schema(path: &Path) -> Result<AsyncApiV3Spec> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read AsyncAPI file: {}", path.display()))?;

    // Try to determine format from extension
    let spec: AsyncApiSpec = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI JSON from {}", path.display()))?
    } else {
        // Assume YAML (handles .yaml, .yml, or no extension)
        serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI YAML from {}", path.display()))?
    };

    // Extract v3 spec from enum
    match spec {
        AsyncApiSpec::V3_0_0(v3_spec) => Ok(v3_spec),
    }
}

/// Extract message schemas from AsyncAPI spec for fixture generation
///
/// Returns a map of message name -> JSON Schema for generating test fixtures
pub fn extract_message_schemas(spec: &AsyncApiV3Spec) -> Result<HashMap<String, Value>> {
    use asyncapiv3::spec::common::Either;

    let mut schemas = HashMap::new();

    // Extract from components.messages (primary source)
    for (message_name, message_ref_or) in &spec.components.messages {
        tracing::debug!("Processing message: {}", message_name);

        // Handle either direct Message or reference
        match message_ref_or {
            Either::Right(message) => {
                // Direct message definition
                if let Some(schema) = extract_schema_from_message(message, message_name)? {
                    schemas.insert(message_name.clone(), schema);
                }
            }
            Either::Left(reference) => {
                // Reference to another message
                // TODO: Implement reference resolution
                tracing::debug!("Skipping message reference: {}", reference.reference);
            }
        }
    }

    // Extract from channels (may reference components.messages)
    for (channel_name, channel_ref_or) in &spec.channels {
        tracing::debug!("Processing channel: {}", channel_name);

        // Handle either direct Channel or reference
        match channel_ref_or {
            Either::Right(channel) => {
                // Extract messages from the channel
                for (msg_name, msg_ref_or) in &channel.messages {
                    match msg_ref_or {
                        Either::Right(message) => {
                            let full_name = format!("{}_{}", channel_name.trim_start_matches('/'), msg_name);
                            if let Some(schema) = extract_schema_from_message(message, &full_name)? {
                                schemas.insert(full_name, schema);
                            }
                        }
                        Either::Left(_reference) => {
                            // References are handled from components.messages
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

/// Extract JSON Schema from an AsyncAPI Message object
fn extract_schema_from_message(
    message: &asyncapiv3::spec::message::Message,
    message_name: &str,
) -> Result<Option<Value>> {
    use asyncapiv3::spec::common::Either;

    // Check if message has a payload
    let payload = match &message.payload {
        Some(payload_ref_or) => payload_ref_or,
        None => {
            tracing::debug!("Message {} has no payload", message_name);
            return Ok(None);
        }
    };

    // Handle payload (either reference or direct schema)
    match payload {
        Either::Right(schema_or_multiformat) => {
            // Either schemars::Schema or MultiFormatSchema
            match schema_or_multiformat {
                Either::Left(schema) => {
                    // schemars::Schema - convert to JSON Value
                    let schema_json =
                        serde_json::to_value(schema).context("Failed to serialize schemars::Schema to JSON")?;
                    Ok(Some(schema_json))
                }
                Either::Right(multi_format) => {
                    // MultiFormatSchema - already has schema as Value
                    Ok(Some(multi_format.schema.clone()))
                }
            }
        }
        Either::Left(reference) => {
            // Reference to another schema
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
}

/// Determine primary protocol from AsyncAPI spec
pub fn detect_primary_protocol(spec: &AsyncApiV3Spec) -> Result<Protocol> {
    use asyncapiv3::spec::common::Either;

    // Check servers for protocol information
    for server_or_ref in spec.servers.values() {
        match server_or_ref {
            Either::Right(server) => {
                // Server has protocol defined as a String (not Option)
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

    // Default to WebSocket if we can't determine
    tracing::warn!("Could not determine protocol from spec, defaulting to WebSocket");
    Ok(Protocol::WebSocket)
}

/// Generate fixture files from message schemas
///
/// Creates JSON fixture files in the output directory for each message type
pub fn generate_fixtures(spec: &AsyncApiV3Spec, output_dir: &Path, protocol: Protocol) -> Result<usize> {
    // Extract message schemas
    let schemas = extract_message_schemas(spec)?;

    if schemas.is_empty() {
        tracing::warn!("No message schemas found in AsyncAPI spec");
        return Ok(0);
    }

    // Determine subdirectory based on protocol
    let subdir = match protocol {
        Protocol::WebSocket => "websockets",
        Protocol::Sse => "sse",
        Protocol::Http => "http",
        _ => "asyncapi",
    };

    let target_dir = output_dir.join(subdir);
    fs::create_dir_all(&target_dir).with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    let mut generated_count = 0;

    // Generate a fixture file for each message schema
    for (message_name, schema) in &schemas {
        let fixture_path = target_dir.join(format!("{}.json", message_name));

        // Create fixture with metadata
        let fixture = serde_json::json!({
            "name": message_name,
            "description": format!("Test fixture for {} message", message_name),
            "schema": schema,
            "examples": generate_example_from_schema(schema)?
        });

        // Write fixture file
        let fixture_json = serde_json::to_string_pretty(&fixture).context("Failed to serialize fixture to JSON")?;

        fs::write(&fixture_path, fixture_json)
            .with_context(|| format!("Failed to write fixture: {}", fixture_path.display()))?;

        println!("  Generated: {}", fixture_path.display());
        generated_count += 1;
    }

    Ok(generated_count)
}

/// Generate example data from JSON Schema
///
/// Creates a simple valid example based on the schema properties
fn generate_example_from_schema(schema: &Value) -> Result<Vec<Value>> {
    let mut examples = Vec::new();

    // Try to extract examples from schema
    if let Some(schema_examples) = schema.get("examples").and_then(|e| e.as_array()) {
        examples.extend(schema_examples.clone());
    }

    // If no examples, generate a basic one from schema
    if examples.is_empty()
        && let Some(obj) = schema.get("properties").and_then(|p| p.as_object())
    {
        let mut example = serde_json::Map::new();

        for (prop_name, prop_schema) in obj {
            // Generate simple example values based on type
            let example_value = if let Some(const_val) = prop_schema.get("const") {
                const_val.clone()
            } else if let Some(type_str) = prop_schema.get("type").and_then(|t| t.as_str()) {
                match type_str {
                    "string" => {
                        // Check for format hints
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

    // Ensure at least one example
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

    // Imports
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

    // Extract channels and operations
    let channels = extract_channel_info(spec)?;

    // Generate fixture loader
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

    // Generate message handlers based on protocol
    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    // Generate main function
    code.push_str("\n\nif __name__ == \"__main__\":\n");
    code.push_str("    asyncio.run(main())\n");

    Ok(code)
}

/// Channel information extracted from AsyncAPI spec
#[derive(Debug)]
#[allow(dead_code)]
struct ChannelInfo {
    name: String,
    path: String,
    messages: Vec<String>,
}

/// Extract channel information from AsyncAPI spec
fn extract_channel_info(spec: &AsyncApiV3Spec) -> Result<Vec<ChannelInfo>> {
    use asyncapiv3::spec::common::Either;

    let mut channels = Vec::new();

    for (channel_path, channel_ref_or) in &spec.channels {
        match channel_ref_or {
            Either::Right(channel) => {
                let messages: Vec<String> = channel.messages.keys().cloned().collect();

                channels.push(ChannelInfo {
                    name: channel_path.trim_start_matches('/').replace('/', "_"),
                    path: channel_path.clone(),
                    messages,
                });
            }
            Either::Left(_reference) => {
                tracing::debug!("Skipping channel reference: {}", channel_path);
            }
        }
    }

    Ok(channels)
}

/// Generate WebSocket message handlers for Python
fn generate_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    // Message validation function
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

    // WebSocket handler function
    code.push_str("async def handle_websocket(uri: str) -> None:\n");
    code.push_str("    \"\"\"Connect to WebSocket and handle messages\"\"\"\n");
    code.push_str("    print(f\"Connecting to {uri}...\")\n");
    code.push_str("    \n");
    code.push_str("    async with websockets.connect(uri) as websocket:\n");
    code.push_str("        print(\"✓ Connected\")\n");
    code.push_str("        \n");

    // Load message schemas
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

    // Main function
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

    // Imports and header
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

    code.push_str("import { readFileSync } from 'fs';\n");
    code.push_str("import { join, dirname } from 'path';\n");
    code.push_str("import { fileURLToPath } from 'url';\n\n");

    // Extract channels and operations
    let channels = extract_channel_info(spec)?;

    // Generate fixture loader
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

    code.push_str("function loadFixture(name: string): any {\n");
    code.push_str("  const fixturePath = join(FIXTURES_DIR, `${name}.json`);\n");
    code.push_str("  const content = readFileSync(fixturePath, 'utf-8');\n");
    code.push_str("  return JSON.parse(content);\n");
    code.push_str("}\n\n");

    // Generate message handlers based on protocol
    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_nodejs_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_nodejs_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    // Generate main function
    code.push_str("\n// Run main function\n");
    code.push_str("main().catch(console.error);\n");

    Ok(code)
}

/// Generate Node.js WebSocket message handlers
fn generate_nodejs_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    // Message validation function
    code.push_str("function validateMessage(message: any, fixtureName: string): boolean {\n");
    code.push_str("  try {\n");
    code.push_str("    const fixture = loadFixture(fixtureName);\n");
    code.push_str("    const schema = fixture.schema || {};\n");
    code.push_str("    const required = schema.required || [];\n");
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

    // WebSocket handler function
    code.push_str("async function handleWebSocket(uri: string): Promise<void> {\n");
    code.push_str("  console.log(`Connecting to ${uri}...`);\n");
    code.push_str("  \n");
    code.push_str("  return new Promise((resolve, reject) => {\n");
    code.push_str("    const ws = new WebSocket(uri);\n");
    code.push_str("    \n");
    code.push_str("    ws.on('open', () => {\n");
    code.push_str("      console.log('✓ Connected');\n");
    code.push_str("      \n");

    // Load message schemas
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
    code.push_str("      const message = JSON.parse(data.toString());\n");
    code.push_str("      const msgType = message.type || 'unknown';\n");
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

    // Main function
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

    // SSE handler function
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
    code.push_str("          const message = JSON.parse(data);\n");
    code.push_str("          console.log('Received event:', message);\n");
    code.push_str("        } catch {\n");
    code.push_str("          console.log('Received:', data);\n");
    code.push_str("        }\n");
    code.push_str("      }\n");
    code.push_str("    }\n");
    code.push_str("  }\n");
    code.push_str("}\n\n");

    // Main function
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

    // Shebang and header
    code.push_str("#!/usr/bin/env ruby\n");
    code.push_str("# frozen_string_literal: true\n\n");
    code.push_str("# Test application generated from AsyncAPI specification\n\n");

    // Requires
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

    // Extract channels and operations
    let channels = extract_channel_info(spec)?;

    // Generate fixture loader
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

    // Generate message handlers based on protocol
    match protocol {
        Protocol::WebSocket => {
            code.push_str(&generate_ruby_websocket_handlers(spec, &channels)?);
        }
        Protocol::Sse => {
            code.push_str(&generate_ruby_sse_handlers(spec, &channels)?);
        }
        _ => {}
    }

    // Generate main execution
    code.push_str("\n# Run main function\n");
    code.push_str("main\n");

    Ok(code)
}

/// Generate Ruby WebSocket message handlers
fn generate_ruby_websocket_handlers(spec: &AsyncApiV3Spec, channels: &[ChannelInfo]) -> Result<String> {
    let mut code = String::new();

    // Message validation function
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

    // WebSocket handler function
    code.push_str("def handle_websocket(uri)\n");
    code.push_str("  puts \"Connecting to #{uri}...\"\n");
    code.push_str("  \n");
    code.push_str("  EM.run do\n");
    code.push_str("    ws = Faye::WebSocket::Client.new(uri)\n");
    code.push_str("    \n");
    code.push_str("    ws.on :open do |_event|\n");
    code.push_str("      puts '✓ Connected'\n");
    code.push_str("      \n");

    // Load message schemas
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

    // Main function
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

    // SSE handler function
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

    // Main function
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

    // SSE handler function
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

    // Main function
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
