//! Ruby AsyncAPI code generation.

use anyhow::{Result, bail};

use super::base::sanitize_identifier;
use super::{AsyncApiGenerator, ChannelInfo};

/// Ruby AsyncAPI code generator
pub struct RubyAsyncApiGenerator;

impl AsyncApiGenerator for RubyAsyncApiGenerator {
    fn generate_test_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        let mut code = String::new();

        code.push_str("#!/usr/bin/env ruby\n");
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
                return Err(anyhow::anyhow!("Unsupported protocol for Ruby test app: {}", protocol));
            }
        }

        code.push_str("require 'json'\n\n");

        if protocol == "websocket" {
            code.push_str("def handle_websocket(ws)\n");
            code.push_str("  # Handle Faye::WebSocket messages\n");
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
        code.push_str("end\n\n");
        code.push_str("main\n");

        Ok(code)
    }

    fn generate_handler_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        if channels.is_empty() {
            bail!("AsyncAPI spec does not define any channels");
        }

        match protocol {
            "websocket" | "sse" => {}
            other => bail!("Protocol {} is not supported for Ruby handler generation", other),
        }

        let mut code = String::new();
        code.push_str("#!/usr/bin/env ruby\n");
        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("require \"spikard\"\n\n");
        code.push_str("app = Spikard::App.new\n\n");

        for channel in channels {
            let handler_name = sanitize_identifier(&channel.name);
            let message_description = if channel.messages.is_empty() {
                "messages".to_string()
            } else {
                channel.messages.join(", ")
            };

            match protocol {
                "websocket" => {
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
                "sse" => {
                    code.push_str(&format!(
                        "app.get(\"{}\", handler_name: \"{}\") do |_request|\n",
                        channel.path, handler_name
                    ));
                    code.push_str("  stream = Enumerator.new do |yielder|\n");
                    code.push_str("    yielder << \"data: {\\\"message\\\": \\\"replace with event\\\"}\\n\\n\"\n");
                    code.push_str("  end\n\n");
                    code.push_str(
                        "  Spikard::StreamingResponse.new(stream, status_code: 200, headers: { \"content-type\" => \"text/event-stream\" })\n",
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
        }];

        let code = generator.generate_test_app(&channels, "websocket").unwrap();
        assert!(code.contains("#!/usr/bin/env ruby"));
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
        }];

        let code = generator.generate_handler_app(&channels, "websocket").unwrap();
        assert!(code.contains("app.websocket"));
        assert!(code.contains("def handler.handle_message"));
    }
}
