//! AsyncAPI v3 HTTP endpoints and spec parsing
//!
//! Provides three HTTP endpoints for AsyncAPI spec interaction:
//! - `POST /asyncapi/parse` — parse a raw AsyncAPI 3.0 spec JSON and return structured data
//! - `POST /asyncapi/validate` — validate a message payload against a channel's message schema
//! - `GET /asyncapi.json` — return the registered AsyncAPI spec from app state
//!
//! All business logic lives here. Language bindings only wire config and convert types.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

/// AsyncAPI HTTP endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AsyncApiConfig {
    /// Enable AsyncAPI endpoints (default: false)
    pub enabled: bool,
    /// Pre-registered AsyncAPI spec to serve from GET /asyncapi.json
    pub spec: Option<serde_json::Value>,
}

/// State shared across AsyncAPI HTTP handlers
#[derive(Clone)]
pub(crate) struct AsyncApiState {
    /// Optionally pre-registered spec for GET /asyncapi.json
    pub registered_spec: Option<Arc<Value>>,
}

/// A single channel extracted from an AsyncAPI spec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedChannel {
    /// Channel key from the spec (e.g. "chat/messages")
    pub name: String,
    /// Channel address / path
    pub address: String,
    /// Message names declared on this channel
    pub messages: Vec<String>,
    /// Bindings (ws / http / amqp / …) as raw JSON for forward-compatibility
    pub bindings: Option<serde_json::Value>,
}

/// A single operation extracted from an AsyncAPI spec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedOperation {
    /// Operation name
    pub name: String,
    /// Operation action: "send" or "receive"
    pub action: String,
    /// Channel reference (resolved to the channel name)
    pub channel: String,
}

/// A resolved message (name + JSON Schema)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedMessage {
    /// Message name
    pub name: String,
    /// Resolved JSON Schema for the message payload, if available
    pub schema: Option<serde_json::Value>,
}

/// Full parse result returned by `POST /asyncapi/parse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub spec_version: String,
    pub title: String,
    pub api_version: String,
    pub channels: Vec<ParsedChannel>,
    pub operations: Vec<ParsedOperation>,
    pub messages: Vec<ParsedMessage>,
}

/// Parse an AsyncAPI 3.0 spec from a JSON [`Value`] and return structured data.
///
/// # Errors
///
/// Returns an error string if the spec is not a valid AsyncAPI 3.0.0 document.
pub fn parse_asyncapi_value(spec: &Value) -> Result<ParseResult, String> {
    use asyncapiv3::spec::AsyncApiSpec;

    if let Some(version) = spec.get("asyncapi").and_then(Value::as_str) {
        if version != "3.0.0" {
            return Err(format!("unsupported AsyncAPI version: {version}, expected 3.0.0"));
        }
    } else if spec.get("asyncapi").is_none() {
        return Err("missing required field: asyncapi".to_string());
    }

    let raw: AsyncApiSpec =
        serde_json::from_value(spec.clone()).map_err(|e| format!("Failed to parse AsyncAPI spec: {e}"))?;

    let AsyncApiSpec::V3_0_0(v3) = raw;

    let spec_version = "3.0.0".to_string();

    let title = v3.info.title.clone();
    let api_version = v3.info.version.clone();

    let spec_doc =
        serde_json::to_value(&v3).map_err(|e| format!("Failed to serialize AsyncAPI spec for $ref resolution: {e}"))?;

    let channels = extract_channels(&v3, &spec_doc)?;
    let operations = extract_operations(&v3)?;
    let messages = extract_messages(&v3, &spec_doc)?;

    Ok(ParseResult {
        spec_version,
        title,
        api_version,
        channels,
        operations,
        messages,
    })
}

fn extract_channels(v3: &asyncapiv3::spec::AsyncApiV3Spec, spec_doc: &Value) -> Result<Vec<ParsedChannel>, String> {
    use asyncapiv3::spec::common::Either;

    let mut channels = Vec::new();

    for (name, channel_ref_or) in &v3.channels {
        match channel_ref_or {
            Either::Right(channel) => {
                let address = channel.address.clone().unwrap_or_else(|| name.clone());
                let messages: Vec<String> = channel.messages.keys().cloned().collect();
                // ~keep asyncapiv3 does not expose channel binding fields, so raw spec_doc preserves them.
                let bindings = spec_doc
                    .pointer(&format!("/channels/{}/bindings", name.replace('/', "~1")))
                    .cloned();
                channels.push(ParsedChannel {
                    name: name.clone(),
                    address,
                    messages,
                    bindings,
                });
            }
            Either::Left(_) => {}
        }
    }

    channels.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(channels)
}

fn extract_operations(v3: &asyncapiv3::spec::AsyncApiV3Spec) -> Result<Vec<ParsedOperation>, String> {
    use asyncapiv3::spec::common::Either;

    let mut operations = Vec::new();

    for (name, op_ref_or) in &v3.operations {
        match op_ref_or {
            Either::Right(op) => {
                use asyncapiv3::spec::operation::OperationAction;
                let action = match op.action {
                    OperationAction::Send => "send",
                    OperationAction::Receive => "receive",
                }
                .to_string();

                let channel_ref = &op.channel.reference;
                let channel = channel_ref
                    .strip_prefix("#/channels/")
                    .map(|s| s.split('/').next().unwrap_or(s).to_string())
                    .unwrap_or_else(|| channel_ref.clone());

                operations.push(ParsedOperation {
                    name: name.clone(),
                    action,
                    channel,
                });
            }
            Either::Left(_) => {}
        }
    }

    operations.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(operations)
}

fn extract_messages(v3: &asyncapiv3::spec::AsyncApiV3Spec, spec_doc: &Value) -> Result<Vec<ParsedMessage>, String> {
    use asyncapiv3::spec::common::Either;
    use asyncapiv3::spec::message::Message;

    let mut messages: HashMap<String, Option<Value>> = HashMap::new();

    for (msg_name, msg_ref_or) in &v3.components.messages {
        match msg_ref_or {
            Either::Right(msg) => {
                let schema = extract_schema_from_message(msg, spec_doc);
                messages.insert(msg_name.clone(), schema);
            }
            Either::Left(reference) => {
                if let Some(msg) = resolve_ref_as::<Message>(spec_doc, &reference.reference) {
                    let schema = extract_schema_from_message(&msg, spec_doc);
                    messages.insert(msg_name.clone(), schema);
                }
            }
        }
    }

    for (channel_name, channel_ref_or) in &v3.channels {
        match channel_ref_or {
            Either::Right(channel) => {
                for (msg_name, msg_ref_or) in &channel.messages {
                    let slug = channel_name.trim_start_matches('/').replace('/', "_");
                    let full_name = format!("{slug}_{msg_name}");
                    match msg_ref_or {
                        Either::Right(msg) => {
                            let schema = extract_schema_from_message(msg, spec_doc);
                            messages.entry(full_name).or_insert(schema);
                        }
                        Either::Left(_) => {}
                    }
                }
            }
            Either::Left(_) => {}
        }
    }

    let mut result: Vec<ParsedMessage> = messages
        .into_iter()
        .map(|(name, schema)| ParsedMessage { name, schema })
        .collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}

fn extract_schema_from_message(message: &asyncapiv3::spec::message::Message, spec_doc: &Value) -> Option<Value> {
    use asyncapiv3::spec::common::Either;

    let payload = message.payload.as_ref()?;
    match payload {
        Either::Right(schema_or_multi) => match schema_or_multi {
            Either::Left(schema) => serde_json::to_value(schema).ok(),
            Either::Right(multi_format) => Some(multi_format.schema.clone()),
        },
        Either::Left(reference) => resolve_ref_value(spec_doc, &reference.reference).map(normalize_schema_ref_value),
    }
}

/// Validate a message payload against its channel/message schema.
///
/// Returns `(valid, errors)`.
pub fn validate_message(
    spec: &Value,
    channel_name: &str,
    message_name: &str,
    payload: &Value,
) -> Result<(bool, Vec<String>), String> {
    use asyncapiv3::spec::AsyncApiSpec;
    use asyncapiv3::spec::common::Either;

    let raw: AsyncApiSpec =
        serde_json::from_value(spec.clone()).map_err(|e| format!("Failed to parse AsyncAPI spec: {e}"))?;
    let AsyncApiSpec::V3_0_0(v3) = raw;

    let spec_doc = serde_json::to_value(&v3).map_err(|e| format!("Failed to serialize spec: {e}"))?;

    let channel = v3
        .channels
        .get(channel_name)
        .ok_or_else(|| format!("Channel '{channel_name}' not found in spec"))?;

    let channel = match channel {
        Either::Right(c) => c,
        Either::Left(_) => return Err(format!("Channel '{channel_name}' is a $ref, not inline")),
    };

    let msg_ref_or = channel
        .messages
        .get(message_name)
        .ok_or_else(|| format!("Message '{message_name}' not found on channel '{channel_name}'"))?;

    let schema = match msg_ref_or {
        Either::Right(msg) => extract_schema_from_message(msg, &spec_doc),
        Either::Left(reference) => {
            use asyncapiv3::spec::message::Message;
            resolve_ref_as::<Message>(&spec_doc, &reference.reference)
                .and_then(|msg| extract_schema_from_message(&msg, &spec_doc))
        }
    };

    let schema = match schema {
        Some(s) => s,
        None => {
            return Ok((true, Vec::new()));
        }
    };

    let compiled = jsonschema::validator_for(&schema).map_err(|e| format!("Failed to compile schema: {e}"))?;

    let errors: Vec<String> = compiled.iter_errors(payload).map(|e| e.to_string()).collect();

    Ok((errors.is_empty(), errors))
}

fn decode_pointer_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

fn reference_to_pointer(reference: &str) -> Option<String> {
    let raw = reference.strip_prefix("#/")?;
    let mut pointer = String::new();
    for segment in raw.split('/') {
        pointer.push('/');
        pointer.push_str(&decode_pointer_segment(segment));
    }
    Some(pointer)
}

fn resolve_ref_value(document: &Value, reference: &str) -> Option<Value> {
    let mut current = reference.to_string();
    let mut visited = HashSet::new();

    for _ in 0..32 {
        if !visited.insert(current.clone()) {
            return None;
        }
        let pointer = reference_to_pointer(&current)?;
        let value = document.pointer(&pointer)?;
        if let Some(next_ref) = value.get("$ref").and_then(Value::as_str) {
            current = next_ref.to_string();
            continue;
        }
        return Some(value.clone());
    }
    None
}

fn resolve_ref_as<T: serde::de::DeserializeOwned>(document: &Value, reference: &str) -> Option<T> {
    let value = resolve_ref_value(document, reference)?;
    serde_json::from_value(value).ok()
}

fn normalize_schema_ref_value(value: Value) -> Value {
    if let Some(obj) = value.as_object()
        && obj.get("schemaFormat").is_some()
        && let Some(schema) = obj.get("schema")
    {
        return schema.clone();
    }
    value
}

/// Request body for `POST /asyncapi/parse`
#[derive(Debug, Deserialize)]
pub struct ParseRequest {
    #[serde(flatten)]
    pub spec: serde_json::Value,
}

/// Response body for `POST /asyncapi/validate`
#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

/// Request body for `POST /asyncapi/validate`
#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub spec: serde_json::Value,
    pub channel: String,
    pub message: String,
    pub payload: serde_json::Value,
}

/// `POST /asyncapi/parse`
///
/// Accepts a raw AsyncAPI 3.0 spec JSON body, returns structured parse result.
pub(crate) async fn handle_asyncapi_parse(axum::extract::Json(body): axum::extract::Json<Value>) -> Response {
    match parse_asyncapi_value(&body) {
        Ok(result) => (StatusCode::OK, axum::Json(result)).into_response(),
        Err(error) => problem_response(StatusCode::BAD_REQUEST, &error),
    }
}

/// `POST /asyncapi/validate`
///
/// Validates a message payload against the declared channel schema.
pub(crate) async fn handle_asyncapi_validate(
    axum::extract::Json(body): axum::extract::Json<ValidateRequest>,
) -> Response {
    match validate_message(&body.spec, &body.channel, &body.message, &body.payload) {
        Ok((valid, errors)) => (StatusCode::OK, axum::Json(ValidationResponse { valid, errors })).into_response(),
        Err(error) => problem_response(StatusCode::BAD_REQUEST, &error),
    }
}

/// `GET /asyncapi.json`
///
/// Returns the spec registered via [`AsyncApiConfig::spec`].
/// Returns 404 ProblemDetails if no spec is registered.
pub(crate) async fn handle_asyncapi_json(State(state): State<AsyncApiState>) -> Response {
    match &state.registered_spec {
        Some(spec) => (StatusCode::OK, axum::Json((**spec).clone())).into_response(),
        None => problem_response(
            StatusCode::NOT_FOUND,
            "No AsyncAPI spec registered. Configure ServerConfig::asyncapi.spec to register one.",
        ),
    }
}

fn problem_response(status: StatusCode, detail: &str) -> Response {
    let body = serde_json::json!({
        "type": "about:blank",
        "title": status.canonical_reason().unwrap_or("Error"),
        "status": status.as_u16(),
        "detail": detail,
    });
    (
        status,
        [(
            axum::http::header::CONTENT_TYPE,
            spikard_core::problem::CONTENT_TYPE_PROBLEM_JSON,
        )],
        axum::Json(body),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chat_api_spec() -> Value {
        serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Chat API", "version": "1.0.0" },
            "channels": {
                "chat": {
                    "address": "/chat",
                    "messages": {
                        "ChatMessage": {
                            "payload": {
                                "type": "object",
                                "properties": {
                                    "text": { "type": "string" },
                                    "user_id": { "type": "string" }
                                },
                                "required": ["text", "user_id"]
                            }
                        }
                    }
                }
            },
            "operations": {
                "sendChat": {
                    "action": "send",
                    "channel": { "$ref": "#/channels/chat" }
                }
            },
            "components": {}
        })
    }

    #[test]
    fn test_parse_valid_spec_returns_structured_result() {
        let spec = chat_api_spec();
        let result = parse_asyncapi_value(&spec).expect("valid spec should parse");
        assert_eq!(result.spec_version, "3.0.0");
        assert_eq!(result.title, "Chat API");
        assert_eq!(result.api_version, "1.0.0");
        assert_eq!(result.channels.len(), 1);
        assert_eq!(result.channels[0].name, "chat");
        assert_eq!(result.operations.len(), 1);
        assert_eq!(result.operations[0].name, "sendChat");
        assert_eq!(result.operations[0].action, "send");
    }

    #[test]
    fn test_parse_invalid_version_returns_error() {
        let spec = serde_json::json!({
            "asyncapi": "2.0.0",
            "info": { "title": "Old API", "version": "1.0.0" },
            "channels": {}
        });
        let err = parse_asyncapi_value(&spec).expect_err("should fail for 2.0.0");
        assert!(
            err.contains("unsupported AsyncAPI version") || err.contains("2.0.0"),
            "Unexpected error: {err}"
        );
    }

    #[test]
    fn test_parse_missing_required_field_returns_error() {
        let spec = serde_json::json!({
            "info": { "title": "No version field", "version": "1.0.0" },
            "channels": {}
        });
        let result = parse_asyncapi_value(&spec);
        assert!(result.is_err(), "should fail when asyncapi field is missing");
    }

    #[test]
    fn test_validate_message_valid_payload() {
        let spec = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Order API", "version": "1.0.0" },
            "channels": {
                "orders/new": {
                    "address": "orders/new",
                    "messages": {
                        "NewOrder": {
                            "payload": {
                                "type": "object",
                                "properties": {
                                    "order_id": { "type": "string" },
                                    "amount": { "type": "number" },
                                    "currency": { "type": "string" }
                                },
                                "required": ["order_id", "amount", "currency"]
                            }
                        }
                    }
                }
            },
            "operations": {},
            "components": {}
        });
        let payload = serde_json::json!({
            "order_id": "ORD-001",
            "amount": 99.99,
            "currency": "USD"
        });
        let (valid, errors) = validate_message(&spec, "orders/new", "NewOrder", &payload).expect("validate");
        assert!(valid, "Expected valid, but got errors: {errors:?}");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_message_missing_required_fields() {
        let spec = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Order API", "version": "1.0.0" },
            "channels": {
                "orders/new": {
                    "address": "orders/new",
                    "messages": {
                        "NewOrder": {
                            "payload": {
                                "type": "object",
                                "properties": {
                                    "order_id": { "type": "string" },
                                    "amount": { "type": "number" },
                                    "currency": { "type": "string" }
                                },
                                "required": ["order_id", "amount", "currency"]
                            }
                        }
                    }
                }
            },
            "operations": {},
            "components": {}
        });
        let payload = serde_json::json!({ "order_id": "ORD-002" });
        let (valid, errors) = validate_message(&spec, "orders/new", "NewOrder", &payload).expect("validate");
        assert!(!valid, "Expected invalid");
        assert!(!errors.is_empty(), "Expected validation errors");
    }

    #[test]
    fn test_validate_message_unknown_channel_returns_error() {
        let spec = chat_api_spec();
        let payload = serde_json::json!({ "text": "hello", "user_id": "u1" });
        let result = validate_message(&spec, "nonexistent_channel", "ChatMessage", &payload);
        assert!(result.is_err(), "Should error for unknown channel");
    }

    #[test]
    fn test_parse_channel_extraction_fixture_asyncapi_channel_extraction() {
        let spec = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Chat API", "version": "1.0.0" },
            "channels": {
                "chat/messages": {
                    "address": "chat/messages",
                    "messages": {
                        "ChatMessage": { "$ref": "#/components/messages/ChatMessage" }
                    }
                },
                "user/events": {
                    "address": "user/events",
                    "messages": {
                        "UserEvent": { "$ref": "#/components/messages/UserEvent" }
                    }
                }
            },
            "operations": {},
            "components": {
                "messages": {
                    "ChatMessage": {
                        "payload": {
                            "type": "object",
                            "properties": {
                                "text": { "type": "string" },
                                "user_id": { "type": "string" }
                            },
                            "required": ["text", "user_id"]
                        }
                    },
                    "UserEvent": {
                        "payload": {
                            "type": "object",
                            "properties": {
                                "event": { "type": "string" },
                                "user_id": { "type": "string" }
                            },
                            "required": ["event", "user_id"]
                        }
                    }
                }
            }
        });
        let result = parse_asyncapi_value(&spec).expect("should parse");
        assert_eq!(result.channels.len(), 2);
        let names: Vec<&str> = result.channels.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"chat/messages"), "Missing chat/messages");
        assert!(names.contains(&"user/events"), "Missing user/events");
    }

    #[test]
    fn test_parse_operation_extraction_fixture() {
        let spec = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": { "title": "Notification API", "version": "2.0.0" },
            "channels": {
                "notifications": {
                    "address": "notifications",
                    "messages": {
                        "Notification": { "$ref": "#/components/messages/Notification" }
                    }
                }
            },
            "operations": {
                "receiveNotification": {
                    "action": "receive",
                    "channel": { "$ref": "#/channels/notifications" },
                    "messages": [{ "$ref": "#/channels/notifications/messages/Notification" }]
                },
                "sendAck": {
                    "action": "send",
                    "channel": { "$ref": "#/channels/notifications" }
                }
            },
            "components": {
                "messages": {
                    "Notification": {
                        "payload": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "body": { "type": "string" }
                            },
                            "required": ["id", "body"]
                        }
                    }
                }
            }
        });
        let result = parse_asyncapi_value(&spec).expect("should parse");
        assert_eq!(result.channels.len(), 1);
        assert_eq!(result.operations.len(), 2);
        let op_names: Vec<&str> = result.operations.iter().map(|o| o.name.as_str()).collect();
        assert!(op_names.contains(&"receiveNotification"));
        assert!(op_names.contains(&"sendAck"));
    }

    #[test]
    fn test_parse_validate_summary_fields_fixture() {
        let spec = serde_json::json!({
            "asyncapi": "3.0.0",
            "info": {
                "title": "Inventory Service",
                "version": "3.2.1"
            },
            "channels": {
                "inventory/updates": {
                    "address": "inventory/updates",
                    "messages": {
                        "InventoryUpdate": {
                            "payload": { "type": "object" }
                        }
                    }
                }
            },
            "operations": {},
            "components": {}
        });
        let result = parse_asyncapi_value(&spec).expect("should parse");
        assert_eq!(result.spec_version, "3.0.0");
        assert_eq!(result.title, "Inventory Service");
        assert_eq!(result.api_version, "3.2.1");
        assert_eq!(result.channels.len(), 1);
    }

    #[test]
    fn test_decode_pointer_segment() {
        assert_eq!(decode_pointer_segment("hello~1world"), "hello/world");
        assert_eq!(decode_pointer_segment("test~0value"), "test~value");
    }

    #[test]
    fn test_resolve_ref_value_follows_nested_local_refs() {
        let doc = serde_json::json!({
            "components": {
                "schemas": {
                    "A": { "$ref": "#/components/schemas/B" },
                    "B": { "type": "object", "properties": { "id": { "type": "string" } } }
                }
            }
        });
        let resolved = resolve_ref_value(&doc, "#/components/schemas/A").expect("resolved schema");
        assert_eq!(resolved["type"], "object");
        assert!(resolved["properties"].get("id").is_some());
    }
}
