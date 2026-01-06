//! `AsyncAPI` v3 specification parsing and extraction.
//!
//! This module handles parsing `AsyncAPI` v3 specs and extracting structured data
//! for code generation, including channels, messages, operations, and metadata.

use anyhow::{Context, Result};
use asyncapiv3::spec::{AsyncApiSpec, AsyncApiV3Spec};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Message definition with schema and examples
#[derive(Debug, Clone)]
pub struct MessageDefinition {
    pub schema: Value,
    pub examples: Vec<Value>,
}

/// Message operation metadata from `AsyncAPI` spec
#[derive(Debug, Clone)]
pub struct MessageOperationMetadata {
    pub name: String,
    pub action: String,
    pub replies: Vec<String>,
}

/// Channel operation metadata from `AsyncAPI` spec
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ChannelOperation {
    pub name: String,
    pub action: String,
    pub messages: Vec<String>,
    pub replies: Vec<String>,
}

/// Parse an `AsyncAPI` v3 specification file
///
/// Supports both JSON and YAML formats
pub fn parse_asyncapi_schema(path: &Path) -> Result<AsyncApiV3Spec> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read AsyncAPI file: {}", path.display()))?;

    let spec: AsyncApiSpec = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI JSON from {}", path.display()))?
    } else {
        serde_saphyr::from_str(&content)
            .with_context(|| format!("Failed to parse AsyncAPI YAML from {}", path.display()))?
    };

    match spec {
        AsyncApiSpec::V3_0_0(v3_spec) => Ok(v3_spec),
    }
}

/// Extract message schemas from `AsyncAPI` spec for fixture generation
///
/// Returns a map of message name -> JSON Schema for generating test fixtures
pub fn extract_message_schemas(spec: &AsyncApiV3Spec) -> Result<HashMap<String, MessageDefinition>> {
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

/// Extract JSON Schema from an `AsyncAPI` Message object
fn extract_schema_from_message(
    message: &asyncapiv3::spec::message::Message,
    message_name: &str,
) -> Result<Option<Value>> {
    use asyncapiv3::spec::common::Either;

    let payload = if let Some(payload_ref_or) = &message.payload {
        payload_ref_or
    } else {
        tracing::debug!("Message {} has no payload", message_name);
        return Ok(None);
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
/// Creates a simple valid example based on the schema properties
pub fn generate_example_from_schema(schema: &Value) -> Result<Vec<Value>> {
    let mut examples = Vec::new();

    if let Some(schema_examples) = schema.get("examples").and_then(|e| e.as_array()) {
        examples.extend(schema_examples.clone());
    }

    if examples.is_empty()
        && schema
            .get("type")
            .and_then(|value| value.as_str())
            .is_some_and(|ty| ty.eq_ignore_ascii_case("array"))
    {
        if let Some(items) = schema.get("items") {
            let generated = generate_example_from_schema(items)?;
            let template = generated
                .into_iter()
                .next()
                .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
            let min_items = schema.get("minItems").and_then(serde_json::Value::as_u64).unwrap_or(1);
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
                                "email" => Value::String("user@example.com".to_string()),
                                "uri" => Value::String("https://example.com".to_string()),
                                "uuid" => Value::String("550e8400-e29b-41d4-a716-446655440000".to_string()),
                                _ => Value::String(format!("example_{prop_name}")),
                            }
                        } else {
                            Value::String(format!("example_{prop_name}"))
                        }
                    }
                    "number" => Value::Number(
                        serde_json::Number::from_f64(std::f64::consts::PI)
                            .unwrap_or_else(|| serde_json::Number::from(314)),
                    ),
                    "integer" => Value::Number(serde_json::Number::from(42)),
                    "boolean" => Value::Bool(true),
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
        examples.push(Value::Object(serde_json::Map::new()));
    }

    Ok(examples)
}

/// Protocol types supported by `AsyncAPI`
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
    /// Detect protocol from `AsyncAPI` server definition
    #[must_use]
    pub fn from_protocol_string(protocol: &str) -> Self {
        match protocol.to_lowercase().as_str() {
            "ws" | "wss" | "websocket" | "websockets" => Self::WebSocket,
            "sse" | "server-sent-events" => Self::Sse,
            "http" | "https" => Self::Http,
            "kafka" => Self::Kafka,
            "mqtt" => Self::Mqtt,
            "amqp" => Self::Amqp,
            _ => Self::Other,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::WebSocket => "websocket",
            Self::Sse => "sse",
            Self::Http => "http",
            Self::Kafka => "kafka",
            Self::Mqtt => "mqtt",
            Self::Amqp => "amqp",
            Self::Other => "other",
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Determine primary protocol from `AsyncAPI` spec
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

/// Decode JSON pointer segments
pub fn decode_pointer_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

/// Resolve channel reference to channel path
pub fn resolve_channel_from_ref(reference: &str) -> Option<String> {
    let raw = reference.strip_prefix("#/channels/")?;
    let decoded = raw.split('/').map(decode_pointer_segment).collect::<Vec<_>>().join("/");
    let normalized = decoded.trim_start_matches('/').to_string();
    Some(format!("/{normalized}"))
}

/// Resolve message reference to message name
pub fn resolve_message_from_ref(reference: &str) -> Option<String> {
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

/// Get operation action name as string
pub const fn operation_action_name(action: &asyncapiv3::spec::operation::OperationAction) -> &'static str {
    use asyncapiv3::spec::operation::OperationAction;
    match action {
        OperationAction::Send => "send",
        OperationAction::Receive => "receive",
    }
}

/// Collect message channel addresses from spec
pub fn collect_message_channels(spec: &AsyncApiV3Spec) -> (HashMap<String, String>, HashMap<String, String>) {
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
            format!("/{address}")
        };

        if let Either::Right(channel) = channel_ref_or {
            for (message_name, message_ref) in &channel.messages {
                let slug = channel_path.trim_start_matches('/').replace('/', "_");
                let inline_key = format!("{slug}_{message_name}");
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

/// Collect message operations from spec
pub fn collect_message_operations(
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

/// Collect channel operations from spec
pub fn collect_channel_operations(spec: &AsyncApiV3Spec) -> HashMap<String, Vec<ChannelOperation>> {
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

    #[test]
    fn test_decode_pointer_segment() {
        assert_eq!(decode_pointer_segment("hello~1world"), "hello/world");
        assert_eq!(decode_pointer_segment("test~0value"), "test~value");
    }

    #[test]
    fn test_resolve_message_from_ref_components() {
        let result = resolve_message_from_ref("#/components/messages/UserMessage");
        assert_eq!(result, Some("UserMessage".to_string()));
    }
}
