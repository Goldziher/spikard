//! Test fixture generation from AsyncAPI message schemas.
//!
//! This module handles:
//! - Creating JSON fixture files for testing
//! - Writing fixtures to the file system
//! - Organizing fixtures by protocol type
//! - Including schema and example metadata

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::generators::Message;

/// Generate fixture files from message schemas
///
/// Creates JSON fixture files in the output directory for each message type.
/// Fixtures are organized by protocol (WebSocket, SSE, etc.).
#[allow(dead_code)]
pub fn generate_fixtures(messages: &[Message], output_dir: &Path, protocol: &str) -> Result<Vec<PathBuf>> {
    let subdir = match protocol {
        "websocket" => "websockets",
        "sse" => "sse",
        "http" => "http",
        _ => "asyncapi",
    };

    let target_dir = output_dir.join(subdir);
    fs::create_dir_all(&target_dir).with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    let mut generated_paths = Vec::new();

    for message in messages {
        let fixture_path = target_dir.join(format!("{}.json", message.name));

        let fixture = serde_json::json!({
            "name": message.name,
            "description": format!("Test fixture for {} message", message.name),
            "protocol": protocol,
            "schema": message.schema,
            "examples": message.examples,
        });

        let fixture_json = serde_json::to_string_pretty(&fixture).context("Failed to serialize fixture to JSON")?;

        fs::write(&fixture_path, fixture_json)
            .with_context(|| format!("Failed to write fixture: {}", fixture_path.display()))?;

        println!("  Generated: {}", fixture_path.display());
        generated_paths.push(fixture_path);
    }

    Ok(generated_paths)
}

/// Generate fixture with channel and operation metadata
///
/// Creates a more detailed fixture that includes channel path and operation information.
#[allow(dead_code)]
pub fn generate_fixture_with_metadata(
    message: &Message,
    channel: Option<String>,
    operations: Vec<Value>,
    protocol: &str,
) -> Value {
    serde_json::json!({
        "name": message.name,
        "description": format!("Test fixture for {} message", message.name),
        "protocol": protocol,
        "channel": channel,
        "schema": message.schema,
        "examples": message.examples,
        "operations": operations,
    })
}

/// Write fixtures to file system
///
/// Takes a map of fixture names to fixture values and writes them to disk.
#[allow(dead_code)]
pub fn write_fixtures_to_disk(fixtures: HashMap<String, Value>, output_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut generated_paths = Vec::new();

    fs::create_dir_all(output_dir).with_context(|| format!("Failed to create directory: {}", output_dir.display()))?;

    for (name, fixture) in fixtures {
        let fixture_path = output_dir.join(format!("{}.json", name));
        let fixture_json = serde_json::to_string_pretty(&fixture).context("Failed to serialize fixture to JSON")?;

        fs::write(&fixture_path, fixture_json)
            .with_context(|| format!("Failed to write fixture: {}", fixture_path.display()))?;

        println!("  Generated: {}", fixture_path.display());
        generated_paths.push(fixture_path);
    }

    Ok(generated_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fixture_with_metadata() {
        let msg = Message {
            name: "test_msg".to_string(),
            schema: serde_json::json!({ "type": "object" }),
            examples: vec![serde_json::json!({ "test": "data" })],
        };

        let fixture = generate_fixture_with_metadata(&msg, Some("/test".to_string()), vec![], "websocket");

        assert_eq!(fixture["name"], "test_msg");
        assert_eq!(fixture["protocol"], "websocket");
        assert_eq!(fixture["channel"], "/test");
        assert!(fixture["schema"].is_object());
        assert!(fixture["examples"].is_array());
    }

    #[test]
    fn test_fixture_serialization() {
        let msg = Message {
            name: "test_message".to_string(),
            schema: serde_json::json!({"type": "object", "properties": {"id": {"type": "string"}}}),
            examples: vec![serde_json::json!({"id": "123"})],
        };

        let fixture = serde_json::json!({
            "name": msg.name,
            "description": format!("Test fixture for {} message", msg.name),
            "protocol": "websocket",
            "schema": msg.schema,
            "examples": msg.examples,
        });

        assert_eq!(fixture["name"], "test_message");
        assert_eq!(fixture["protocol"], "websocket");
        assert!(fixture["schema"]["properties"]["id"].is_object());
        assert_eq!(fixture["examples"][0]["id"], "123");
    }
}
