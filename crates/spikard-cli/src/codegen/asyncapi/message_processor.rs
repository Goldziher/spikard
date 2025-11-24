//! Message processing and schema extraction from AsyncAPI specifications.
//!
//! This module handles:
//! - Parsing message definitions from AsyncAPI spec
//! - Extracting JSON schemas from message payloads
//! - Generating example data from schemas
//! - Building intermediate message representations

use anyhow::{Context, Result};
use asyncapiv3::spec::message::Message as AsyncApiMessage;
use asyncapiv3::spec::common::Either;
use asyncapiv3::spec::{AsyncApiV3Spec};
use serde_json::Value;
use std::collections::HashMap;

use super::generators::Message;

/// Internal representation of a message definition
#[derive(Debug, Clone)]
pub(crate) struct MessageDefinition {
    pub(crate) schema: Value,
    pub(crate) examples: Vec<Value>,
}

/// Extract message schemas from AsyncAPI spec
///
/// Returns a map of message name -> JSON Schema for generating test fixtures
pub fn extract_message_schemas(spec: &AsyncApiV3Spec) -> Result<HashMap<String, MessageDefinition>> {
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

/// Build a message definition from AsyncAPI message object
fn build_message_definition(
    message: &AsyncApiMessage,
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
    message: &AsyncApiMessage,
    message_name: &str,
) -> Result<Option<Value>> {
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

/// Generate example data from JSON Schema
///
/// Creates a simple valid example based on the schema properties.
/// This is used when AsyncAPI message doesn't define explicit examples.
pub fn generate_example_from_schema(schema: &Value) -> Result<Vec<Value>> {
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

/// Convert internal MessageDefinition to public Message
pub fn to_public_message(name: String, definition: MessageDefinition) -> Message {
    Message {
        name,
        schema: definition.schema,
        examples: definition.examples,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_example_from_empty_schema() {
        let schema = serde_json::json!({});
        let examples = generate_example_from_schema(&schema).unwrap();
        assert!(!examples.is_empty());
        assert_eq!(examples[0], serde_json::json!({}));
    }

    #[test]
    fn test_generate_example_from_object_schema() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            }
        });
        let examples = generate_example_from_schema(&schema).unwrap();
        assert!(!examples.is_empty());
        let example = &examples[0];
        assert!(example.is_object());
        assert!(example.get("name").is_some());
        assert!(example.get("age").is_some());
    }

    #[test]
    fn test_generate_example_from_array_schema() {
        let schema = serde_json::json!({
            "type": "array",
            "items": { "type": "string" },
            "minItems": 2
        });
        let examples = generate_example_from_schema(&schema).unwrap();
        assert!(!examples.is_empty());
        let example = &examples[0];
        assert!(example.is_array());
        assert!(example.as_array().unwrap().len() >= 2);
    }

    #[test]
    fn test_to_public_message() {
        let definition = MessageDefinition {
            schema: serde_json::json!({ "type": "object" }),
            examples: vec![serde_json::json!({ "test": "data" })],
        };
        let msg = to_public_message("test_msg".to_string(), definition);
        assert_eq!(msg.name, "test_msg");
        assert_eq!(msg.examples.len(), 1);
    }
}
