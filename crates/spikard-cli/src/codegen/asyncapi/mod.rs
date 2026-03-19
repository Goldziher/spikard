//! `AsyncAPI` v3 specification parsing and code generation
//!
//! This module orchestrates `AsyncAPI` spec parsing and code generation across
//! multiple languages. The actual generation logic is delegated to language-specific
//! generators in the `generators/` module.
//!
//! `AsyncAPI` is the standard for describing event-driven APIs, similar to
//! how `OpenAPI` describes REST APIs.

pub mod generators;
pub mod spec_parser;

pub use generators::{
    AsyncApiGenerator, ChannelInfo, ChannelMessage, ElixirAsyncApiGenerator, PhpAsyncApiGenerator,
    PythonAsyncApiGenerator, RubyAsyncApiGenerator, RustAsyncApiGenerator, TypeScriptAsyncApiGenerator,
};
pub use spec_parser::{
    Protocol, collect_channel_operations, collect_message_channels, collect_message_operations,
    detect_primary_protocol, extract_message_schemas, parse_asyncapi_schema, resolve_message_from_ref,
};

use anyhow::{Context, Result, bail};
use asyncapiv3::spec::AsyncApiV3Spec;
use std::fs;
use std::path::{Path, PathBuf};

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
        let fixture_path = target_dir.join(format!("{message_name}.json"));

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

/// Extract channel information from `AsyncAPI` spec for code generation
pub fn extract_channel_info(spec: &AsyncApiV3Spec) -> Result<Vec<ChannelInfo>> {
    use asyncapiv3::spec::common::Either;

    let mut channels = Vec::new();
    let operation_map = collect_channel_operations(spec);
    let message_schemas = extract_message_schemas(spec)?;
    let (_message_channels, alias_map) = collect_message_channels(spec);

    for (channel_path, channel_ref_or) in &spec.channels {
        match channel_ref_or {
            Either::Right(channel) => {
                let messages: Vec<String> = channel.messages.keys().cloned().collect();
                let slug = channel_path.trim_start_matches('/').replace('/', "_");
                let message_definitions = channel
                    .messages
                    .iter()
                    .map(|(message_name, message_ref_or)| {
                        let inline_key = format!("{slug}_{message_name}");
                        let schema_name = match message_ref_or {
                            Either::Right(_) => inline_key.clone(),
                            Either::Left(reference) => alias_map
                                .get(&inline_key)
                                .cloned()
                                .or_else(|| resolve_message_from_ref(&reference.reference))
                                .unwrap_or_else(|| inline_key.clone()),
                        };
                        let definition = message_schemas.get(&schema_name);

                        ChannelMessage {
                            name: message_name.clone(),
                            schema_name,
                            schema: definition.map(|definition| definition.schema.clone()),
                            examples: definition
                                .map(|definition| definition.examples.clone())
                                .unwrap_or_default(),
                        }
                    })
                    .collect();
                let raw_path = channel.address.clone().unwrap_or_else(|| channel_path.clone());
                let normalized_path = if raw_path.starts_with('/') {
                    raw_path.clone()
                } else {
                    format!("/{raw_path}")
                };
                let _operations = operation_map.get(&normalized_path).cloned().unwrap_or_default();

                channels.push(ChannelInfo {
                    name: channel_path.trim_start_matches('/').replace('/', "_"),
                    path: normalized_path,
                    messages,
                    message_definitions,
                });
            }
            Either::Left(_reference) => {
                tracing::debug!("Skipping channel reference: {}", channel_path);
            }
        }
    }

    Ok(channels)
}

/// Generate Python test application from `AsyncAPI` spec
pub fn generate_python_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Python test app: {other:?}"),
    };

    let generator = PythonAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate Node.js test application from `AsyncAPI` spec
pub fn generate_nodejs_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for TypeScript test app: {other:?}"),
    };

    let generator = TypeScriptAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate Ruby test application from `AsyncAPI` spec
pub fn generate_ruby_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Ruby test app: {other:?}"),
    };

    let generator = RubyAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate PHP test application from `AsyncAPI` spec
pub fn generate_php_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for PHP test app: {other:?}"),
    };

    let generator = PhpAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate Rust test application from `AsyncAPI` spec
pub fn generate_rust_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Rust test app: {other:?}"),
    };

    let generator = RustAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate Elixir test application from `AsyncAPI` spec
pub fn generate_elixir_test_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Elixir test app: {other:?}"),
    };

    let generator = ElixirAsyncApiGenerator;
    generator.generate_test_app(&channels, protocol_str)
}

/// Generate Python handler scaffolding from `AsyncAPI` spec
pub fn generate_python_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Python handler generation: {other:?}"),
    };

    let generator = PythonAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

/// Generate Node.js handler scaffolding from `AsyncAPI` spec
pub fn generate_nodejs_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for TypeScript handler generation: {other:?}"),
    };

    let generator = TypeScriptAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

/// Generate Ruby handler scaffolding from `AsyncAPI` spec
pub fn generate_ruby_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Ruby handler generation: {other:?}"),
    };

    let generator = RubyAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

/// Generate Rust handler scaffolding from `AsyncAPI` spec
pub fn generate_rust_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Rust handler generation: {other:?}"),
    };

    let generator = RustAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

/// Generate PHP handler scaffolding from `AsyncAPI` spec
pub fn generate_php_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for PHP handler generation: {other:?}"),
    };

    let generator = PhpAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

/// Generate Elixir handler scaffolding from `AsyncAPI` spec
pub fn generate_elixir_handler_app(spec: &AsyncApiV3Spec, protocol: Protocol) -> Result<String> {
    let channels = extract_channel_info(spec)?;
    let protocol_str = match protocol {
        Protocol::WebSocket => "websocket",
        Protocol::Sse => "sse",
        other => bail!("Unsupported protocol for Elixir handler generation: {other:?}"),
    };

    let generator = ElixirAsyncApiGenerator;
    generator.generate_handler_app(&channels, protocol_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use asyncapiv3::spec::AsyncApiSpec;

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

    #[test]
    fn test_extract_channel_info_includes_message_definitions() {
        let spec_value = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Chat API", "version": "1.0.0" },
            "servers": {
                "primary": { "host": "ws.example.com", "protocol": "ws" }
            },
            "channels": {
                "chat": {
                    "address": "/chat",
                    "messages": {
                        "chatEvent": {
                            "payload": {
                                "type": "object",
                                "properties": {
                                    "type": { "const": "chatEvent", "type": "string" },
                                    "body": { "type": "string" }
                                },
                                "required": ["type", "body"]
                            }
                        }
                    }
                }
            }
        });

        let spec = match serde_json::from_value::<AsyncApiSpec>(spec_value).expect("valid asyncapi spec") {
            AsyncApiSpec::V3_0_0(v3) => v3,
        };

        let channels = extract_channel_info(&spec).expect("channel extraction should succeed");
        assert_eq!(channels.len(), 1);
        assert_eq!(channels[0].messages, vec!["chatEvent"]);
        assert_eq!(channels[0].message_definitions.len(), 1);
        assert_eq!(channels[0].message_definitions[0].name, "chatEvent");
        assert!(channels[0].message_definitions[0].schema.is_some());
    }
}
