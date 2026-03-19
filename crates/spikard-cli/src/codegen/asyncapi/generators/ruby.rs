//! Ruby `AsyncAPI` code generation.

use anyhow::{Result, bail};
use heck::{ToPascalCase, ToSnakeCase};
use serde_json::Value;

use super::base::sanitize_identifier;
use super::{AsyncApiGenerator, ChannelInfo, ChannelMessage};

/// Ruby `AsyncAPI` code generator
pub struct RubyAsyncApiGenerator;

/// Convert identifier to `PascalCase` for Ruby class names
fn to_pascal_case(name: &str) -> String {
    let identifier = sanitize_identifier(name);
    let parts: Vec<&str> = identifier.split('_').collect();
    parts
        .iter()
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}

impl AsyncApiGenerator for RubyAsyncApiGenerator {
    fn generate_test_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        let mut code = String::new();

        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# Test application generated from AsyncAPI specification\n\n");

        match protocol {
            "websocket" => {
                code.push_str("require 'faye/websocket'\n");
                code.push_str("require 'eventmachine'\n");
            }
            "sse" => {
                code.push_str("require 'net/http'\n");
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported protocol for Ruby test app: {protocol}"));
            }
        }

        code.push_str("require 'json'\n\n");

        if protocol == "websocket" {
            code.push_str("def handle_websocket(ws)\n");
            code.push_str("  ws.on :message do |event|\n");
            code.push_str("    data = JSON.parse(event.data)\n");
            code.push_str("    puts \"Received: #{data}\"\n");
            code.push_str("  end\n");
            code.push_str("end\n\n");
        }

        code.push_str("def main\n");
        code.push_str("  uri = ENV['URI'] || 'ws://localhost:8000");
        if let Some(first_channel) = channels.first() {
            code.push_str(&first_channel.path);
        }
        code.push_str("'\n");
        code.push_str("  puts \"Connecting to #{uri}...\"\n");
        code.push_str("  # TODO: Implement connection logic\n");
        code.push_str("end\n\n");
        code.push_str("main if __FILE__ == $PROGRAM_NAME\n");

        Ok(code)
    }

    fn generate_handler_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        if channels.is_empty() {
            bail!("AsyncAPI spec does not define any channels");
        }

        match protocol {
            "websocket" | "sse" => {}
            other => bail!("Protocol {other} is not supported for Ruby handler generation"),
        }

        let mut code = String::new();
        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("require 'date'\n");
        code.push_str("require 'time'\n");
        code.push_str("require 'spikard'\n\n");

        code.push_str("module AsyncApiTypes\n");
        code.push_str(&generate_ruby_assertions());
        let mut models = String::new();
        for channel in channels {
            models.push_str(&generate_channel_message_models(channel));
        }
        while models.ends_with("\n\n") {
            models.pop();
        }
        code.push_str(&models);
        code.push_str("end\n\n");
        code.push_str("app = Spikard::App.new\n\n");

        for channel in channels {
            let handler_name = to_pascal_case(&channel.name);

            match protocol {
                "websocket" => {
                    code.push_str(&format!("# WebSocket handler for {}\n", channel.path));
                    code.push_str(&format!("class {handler_name}Handler < Spikard::WebSocketHandler\n"));
                    if channel.message_definitions.len() > 1 {
                        code.push_str("  PAYLOAD_CLASSES = [\n");
                        let payload_messages = channel
                            .message_definitions
                            .iter()
                            .filter(|message| message.schema.is_some())
                            .collect::<Vec<_>>();
                        for (index, message) in payload_messages.iter().enumerate() {
                            let payload_class = ruby_message_type_name(channel, message);
                            let suffix = if index + 1 == payload_messages.len() { "" } else { "," };
                            code.push_str(&format!("    AsyncApiTypes::{payload_class}{suffix}\n"));
                        }
                        code.push_str("  ].freeze\n\n");
                    }
                    code.push_str("  def handle_message(message)\n");
                    code.push_str("    payload = parse_message(message)\n");
                    code.push_str(&format!("    # TODO: Handle messages for {}\n", channel.path));
                    code.push_str("    payload.respond_to?(:to_h) ? payload.to_h : payload\n");
                    code.push_str("  end\n\n");
                    code.push_str("  private\n\n");
                    code.push_str("  def parse_message(message)\n");
                    code.push_str("    payload = AsyncApiTypes::Assertions.deep_symbolize(message)\n");
                    code.push_str("    raise ArgumentError, 'Expected Hash payload' unless payload.is_a?(Hash)\n");
                    code.push_str("\n");
                    match channel.message_definitions.as_slice() {
                        [] => {
                            code.push_str("    payload\n");
                        }
                        [message] => {
                            let payload_class = ruby_message_type_name(channel, message);
                            code.push_str(&format!("    AsyncApiTypes::{payload_class}.from_h(payload)\n"));
                        }
                        _messages => {
                            code.push_str("    PAYLOAD_CLASSES.each do |payload_class|\n");
                            code.push_str(
                                "      return payload_class.from_h(payload) if payload_class.matches?(payload)\n",
                            );
                            code.push_str("    end\n");
                            code.push_str(&format!(
                                "    raise ArgumentError, 'Unsupported message payload for {}'\n",
                                channel.path
                            ));
                        }
                    }
                    code.push_str("  end\n");
                    code.push_str("end\n\n");
                    code.push_str(&format!("app.websocket('{}') do\n", channel.path));
                    code.push_str(&format!("  {handler_name}Handler.new\n"));
                    code.push_str("end\n\n");
                }
                "sse" => {
                    code.push_str(&format!("# Server-Sent Events producer for {}\n", channel.path));
                    code.push_str(&format!("class {handler_name}Producer < Spikard::SseEventProducer\n"));
                    code.push_str("  def next_event\n");
                    code.push_str(&format!("    # TODO: Stream events for {}\n", channel.path));
                    if let Some(message) = channel
                        .message_definitions
                        .iter()
                        .find(|message| message.schema.is_some())
                    {
                        let payload_class = ruby_message_type_name(channel, message);
                        code.push_str(&format!("    payload = AsyncApiTypes::{payload_class}.example\n"));
                        code.push_str("    Spikard::SseEvent.new(data: payload.to_h, event_type: 'message')\n");
                    } else {
                        code.push_str(
                            "    Spikard::SseEvent.new(data: { event: 'replace-me' }, event_type: 'message')\n",
                        );
                    }
                    code.push_str("  end\n");
                    code.push_str("end\n\n");
                    code.push_str(&format!("app.sse('{}') do\n", channel.path));
                    code.push_str(&format!("  {handler_name}Producer.new\n"));
                    code.push_str("end\n\n");
                }
                _ => {}
            }
        }

        code.push_str("app.run if $PROGRAM_NAME == __FILE__\n");

        Ok(code)
    }
}

fn generate_ruby_assertions() -> String {
    let mut code = String::new();
    code.push_str("  # Shared coercion helpers for generated AsyncAPI payloads.\n");
    code.push_str("  module Assertions\n");
    code.push_str("    module_function\n\n");
    code.push_str("    def deep_symbolize(value)\n");
    code.push_str("      case value\n");
    code.push_str("      when Hash\n");
    code.push_str("        value.each_with_object({}) do |(key, nested), result|\n");
    code.push_str("          result[key.to_sym] = deep_symbolize(nested)\n");
    code.push_str("        end\n");
    code.push_str("      when Array\n");
    code.push_str("        value.map { |item| deep_symbolize(item) }\n");
    code.push_str("      else\n");
    code.push_str("        value\n");
    code.push_str("      end\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_string(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected string for #{key}\" unless value.is_a?(String)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_integer(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected integer for #{key}\" unless value.is_a?(Integer)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_float(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected number for #{key}\" unless value.is_a?(Numeric)\n");
    code.push_str("\n");
    code.push_str("      value.to_f\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_boolean(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected boolean for #{key}\" unless [true, false].include?(value)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_hash(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected object for #{key}\" unless value.is_a?(Hash)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_array(payload, key)\n");
    code.push_str("      value = payload[key]\n");
    code.push_str("      raise ArgumentError, \"Expected array for #{key}\" unless value.is_a?(Array)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n\n");
    code.push_str("    def map_object_array(payload, key, klass)\n");
    code.push_str("      expect_array(payload, key).map { |item| klass.from_h(item) }\n");
    code.push_str("    end\n\n");
    code.push_str("    def map_optional_object_array(payload, key, klass)\n");
    code.push_str("      return nil unless payload.key?(key)\n");
    code.push_str("\n");
    code.push_str("      map_object_array(payload, key, klass)\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_date(payload, key)\n");
    code.push_str("      Date.iso8601(expect_string(payload, key))\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_time(payload, key)\n");
    code.push_str("      Time.iso8601(expect_string(payload, key))\n");
    code.push_str("    end\n\n");
    code.push_str("    def expect_string_enum(payload, key, allowed)\n");
    code.push_str("      value = expect_string(payload, key)\n");
    code.push_str("      raise ArgumentError, \"Unexpected value for #{key}\" unless allowed.include?(value)\n");
    code.push_str("\n");
    code.push_str("      value\n");
    code.push_str("    end\n");
    code.push_str("  end\n\n");
    code
}

fn generate_channel_message_models(channel: &ChannelInfo) -> String {
    let mut code = String::new();
    for message in &channel.message_definitions {
        if let Some(schema) = &message.schema {
            let class_name = ruby_message_type_name(channel, message);
            code.push_str(&generate_named_schema(&class_name, schema));
            code.push('\n');
        }
    }
    code
}

fn ruby_channel_payload_type(channel: &ChannelInfo) -> Option<String> {
    match channel.message_definitions.as_slice() {
        [] => None,
        [message] => message
            .schema
            .as_ref()
            .map(|_| format!("AsyncApiTypes::{}", ruby_message_type_name(channel, message))),
        _ => Some(
            channel
                .message_definitions
                .iter()
                .filter(|message| message.schema.is_some())
                .map(|message| format!("AsyncApiTypes::{}", ruby_message_type_name(channel, message)))
                .collect::<Vec<_>>()
                .join(" | "),
        ),
    }
}

fn ruby_message_type_name(channel: &ChannelInfo, message: &ChannelMessage) -> String {
    format!(
        "{}Payload",
        format!("{}_{}", channel.name, message.schema_name).to_pascal_case()
    )
}

fn ruby_field_name(field_name: &str) -> String {
    sanitize_identifier(&field_name.to_snake_case())
}

fn generate_named_schema(class_name: &str, schema: &Value) -> String {
    let mut code = String::new();

    for (field_name, field_schema) in object_properties(schema) {
        if schema_has_named_object_shape(field_schema) {
            let nested_name = format!("{class_name}{}", field_name.to_pascal_case());
            code.push_str(&generate_named_schema(&nested_name, field_schema));
            code.push('\n');
        } else if let Some(items) = field_schema.get("items")
            && schema_has_named_object_shape(items)
        {
            let nested_name = format!("{class_name}{}Item", field_name.to_pascal_case());
            code.push_str(&generate_named_schema(&nested_name, items));
            code.push('\n');
        }
    }

    let properties = object_properties(schema);
    let required = required_field_names(schema);

    code.push_str("  # Typed payload model for a generated AsyncAPI message.\n");
    code.push_str(&format!("  class {class_name}\n"));
    if !properties.is_empty() {
        code.push_str("    attr_reader ");
        code.push_str(
            &properties
                .iter()
                .map(|(field_name, _)| format!(":{}", ruby_field_name(field_name)))
                .collect::<Vec<_>>()
                .join(", "),
        );
        code.push_str("\n\n");
    }
    code.push_str("    def initialize(attributes)\n");
    for field_name in properties.iter().map(|(field_name, _)| ruby_field_name(field_name)) {
        code.push_str(&format!("      @{field_name} = attributes[:{field_name}]\n"));
    }
    code.push_str("    end\n\n");

    code.push_str("    # rubocop:disable Metrics/MethodLength, Lint/RedundantCopDisableDirective\n");
    code.push_str("    def self.from_h(payload)\n");
    code.push_str("      new(\n");
    for (index, (field_name, field_schema)) in properties.iter().enumerate() {
        let is_required = required.iter().any(|required_name| required_name == field_name);
        let separator = if index + 1 == properties.len() { "" } else { "," };
        let expr = schema_to_ruby_value_expr(class_name, field_name, field_schema, is_required);
        code.push_str(&format!("        {}: {expr}{separator}\n", ruby_field_name(field_name)));
    }
    code.push_str("      )\n");
    code.push_str("    end\n\n");
    code.push_str("    # rubocop:enable Metrics/MethodLength, Lint/RedundantCopDisableDirective\n\n");

    code.push_str("    def self.matches?(payload)\n");
    code.push_str("      from_h(payload)\n");
    code.push_str("      true\n");
    code.push_str("    rescue ArgumentError, TypeError\n");
    code.push_str("      false\n");
    code.push_str("    end\n\n");

    code.push_str("    def self.example\n");
    code.push_str("      new(\n");
    for (index, (field_name, field_schema)) in properties.iter().enumerate() {
        let separator = if index + 1 == properties.len() { "" } else { "," };
        let expr = schema_to_ruby_example_expr(class_name, field_name, field_schema);
        code.push_str(&format!("        {}: {expr}{separator}\n", ruby_field_name(field_name)));
    }
    code.push_str("      )\n");
    code.push_str("    end\n\n");

    code.push_str("    def to_h\n");
    code.push_str("      {\n");
    for (index, (field_name, field_schema)) in properties.iter().enumerate() {
        let expr = schema_to_ruby_serialize_expr(&ruby_field_name(field_name), field_schema);
        let suffix = if index + 1 == properties.len() { "" } else { "," };
        code.push_str(&format!("        {field_name}: {expr}{suffix}\n"));
    }
    code.push_str("      }\n");
    code.push_str("    end\n");
    code.push_str("  end\n");

    code
}

fn schema_to_ruby_value_expr(parent_name: &str, field_name: &str, schema: &Value, required: bool) -> String {
    let inner = if let Some(enum_values) = schema.get("enum").and_then(Value::as_array) {
        let literals = enum_values
            .iter()
            .filter_map(Value::as_str)
            .map(|value| format!("'{}'", value.replace('\'', "\\'")))
            .collect::<Vec<_>>();
        if literals.is_empty() {
            format!("Assertions.expect_string(payload, :{field_name})")
        } else {
            ruby_string_enum_expr(field_name, &literals)
        }
    } else if let Some(const_value) = schema.get("const").and_then(Value::as_str) {
        ruby_string_enum_expr(field_name, &[format!("'{}'", const_value.replace('\'', "\\'"))])
    } else {
        match schema.get("type").and_then(Value::as_str) {
            Some("string") => match schema.get("format").and_then(Value::as_str) {
                Some("date") => format!("Assertions.expect_date(payload, :{field_name})"),
                Some("date-time") => format!("Assertions.expect_time(payload, :{field_name})"),
                _ => format!("Assertions.expect_string(payload, :{field_name})"),
            },
            Some("integer") => format!("Assertions.expect_integer(payload, :{field_name})"),
            Some("number") => format!("Assertions.expect_float(payload, :{field_name})"),
            Some("boolean") => format!("Assertions.expect_boolean(payload, :{field_name})"),
            Some("array") => {
                if let Some(items) = schema.get("items")
                    && schema_has_named_object_shape(items)
                {
                    let item_class = format!("{parent_name}{}Item", field_name.to_pascal_case());
                    if required {
                        format!("Assertions.map_object_array(payload, :{field_name}, {item_class})")
                    } else {
                        format!("Assertions.map_optional_object_array(payload, :{field_name}, {item_class})")
                    }
                } else {
                    format!("Assertions.expect_array(payload, :{field_name})")
                }
            }
            Some("object") => {
                if schema_has_named_object_shape(schema) {
                    let nested_class = format!("{parent_name}{}", field_name.to_pascal_case());
                    format!("{nested_class}.from_h(Assertions.expect_hash(payload, :{field_name}))")
                } else {
                    format!("Assertions.expect_hash(payload, :{field_name})")
                }
            }
            _ => format!("payload[:{field_name}]"),
        }
    };

    if required {
        inner
    } else if inner.starts_with("Assertions.map_optional_object_array") {
        inner
    } else if inner.contains('\n') || inner.len() > 70 {
        format!("if payload.key?(:{field_name})\n          {inner}\n        end")
    } else {
        format!("payload.key?(:{field_name}) ? {inner} : nil")
    }
}

fn schema_to_ruby_example_expr(class_name: &str, field_name: &str, schema: &Value) -> String {
    if let Some(const_value) = schema.get("const") {
        return literal_ruby_value(const_value);
    }
    if let Some(enum_values) = schema.get("enum").and_then(Value::as_array)
        && let Some(first) = enum_values.first()
    {
        return literal_ruby_value(first);
    }

    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date") => "Date.iso8601('2025-01-01')".to_string(),
            Some("date-time") => "Time.iso8601('2025-01-01T00:00:00Z')".to_string(),
            _ => format!("'{}'", field_name.replace('\'', "\\'")),
        },
        Some("integer") => "1".to_string(),
        Some("number") => "1.0".to_string(),
        Some("boolean") => "true".to_string(),
        Some("array") => {
            if let Some(items) = schema.get("items")
                && schema_has_named_object_shape(items)
            {
                let item_class = format!("{class_name}{}Item", field_name.to_pascal_case());
                return format!("[{}.example]", item_class);
            }
            "[]".to_string()
        }
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                format!("{}{}.example", class_name, field_name.to_pascal_case())
            } else {
                "{}".to_string()
            }
        }
        _ => "nil".to_string(),
    }
}

fn schema_to_ruby_serialize_expr(field_name: &str, schema: &Value) -> String {
    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date") => format!("@{field_name}&.iso8601"),
            Some("date-time") => format!("@{field_name}&.iso8601"),
            _ => format!("@{field_name}"),
        },
        Some("array") => {
            if let Some(items) = schema.get("items")
                && schema_has_named_object_shape(items)
            {
                return format!("@{field_name}&.map(&:to_h)");
            }
            format!("@{field_name}")
        }
        Some("object") if schema_has_named_object_shape(schema) => format!("@{field_name}&.to_h"),
        _ => format!("@{field_name}"),
    }
}

fn literal_ruby_value(value: &Value) -> String {
    match value {
        Value::String(value) => format!("'{}'", value.replace('\'', "\\'")),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Null => "nil".to_string(),
        _ => "nil".to_string(),
    }
}

fn ruby_string_enum_expr(field_name: &str, literals: &[String]) -> String {
    let bare_words = literals
        .iter()
        .map(|literal| literal.trim_matches('\'').to_string())
        .collect::<Vec<_>>();
    let use_word_array = bare_words.iter().all(|word| {
        !word.is_empty()
            && word
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    });
    let array_literal = if use_word_array {
        format!("%w[{}]", bare_words.join(" "))
    } else {
        format!("[{}]", literals.join(", "))
    };
    let one_line = format!("Assertions.expect_string_enum(payload, :{field_name}, {array_literal})");
    if one_line.len() <= 100 {
        one_line
    } else {
        let multiline_array = if use_word_array {
            format!("%w[\n            {}\n          ]", bare_words.join("\n            "))
        } else {
            format!("[\n            {}\n          ]", literals.join(",\n            "))
        };
        format!(
            "Assertions.expect_string_enum(\n          payload,\n          :{field_name},\n          {multiline_array}\n        )"
        )
    }
}

fn object_properties(schema: &Value) -> Vec<(&str, &Value)> {
    schema
        .get("properties")
        .and_then(Value::as_object)
        .map(|properties| properties.iter().map(|(key, value)| (key.as_str(), value)).collect())
        .unwrap_or_default()
}

fn required_field_names(schema: &Value) -> Vec<String> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect()
}

fn schema_has_named_object_shape(schema: &Value) -> bool {
    schema
        .get("type")
        .and_then(Value::as_str)
        .is_some_and(|schema_type| schema_type == "object")
        && schema
            .get("properties")
            .and_then(Value::as_object)
            .is_some_and(|properties| !properties.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruby_generator_test_app_websocket() {
        let generator = RubyAsyncApiGenerator;
        let channels = vec![ChannelInfo {
            name: "chat".to_string(),
            path: "/chat".to_string(),
            messages: vec!["message".to_string()],
            message_definitions: vec![],
        }];

        let code = generator.generate_test_app(&channels, "websocket").unwrap();
        assert!(code.contains("faye/websocket"));
        assert!(code.contains("/chat"));
    }

    #[test]
    fn test_ruby_generator_handler_app() {
        let generator = RubyAsyncApiGenerator;
        let channels = vec![ChannelInfo {
            name: "chat".to_string(),
            path: "/chat".to_string(),
            messages: vec!["message".to_string()],
            message_definitions: vec![ChannelMessage {
                name: "chatEvent".to_string(),
                schema_name: "chatEvent".to_string(),
                schema: Some(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "type": { "const": "chatEvent" },
                        "body": { "type": "string" }
                    },
                    "required": ["type", "body"]
                })),
                examples: vec![],
            }],
        }];

        let code = generator.generate_handler_app(&channels, "websocket").unwrap();
        assert!(code.contains("Spikard::WebSocketHandler"));
        assert!(code.contains("AsyncApiTypes::ChatChatEventPayload.from_h"));
        assert!(code.contains("app.websocket('/chat') do"));
    }
}
