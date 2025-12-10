//! Ruby OpenRPC code generation.

use anyhow::Result;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

use super::OpenRpcGenerator;

/// Ruby OpenRPC code generator
pub struct RubyOpenRpcGenerator;

impl OpenRpcGenerator for RubyOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        // Header
        code.push_str("#!/usr/bin/env ruby\n");
        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# JSON-RPC 2.0 handlers generated from OpenRPC specification.\n");
        code.push_str("#\n");
        code.push_str("# Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\n");

        // Requires
        code.push_str("require 'json'\n");
        code.push_str("require 'async'\n\n");

        // Generate handler classes
        code.push_str("# ============================================================================\n");
        code.push_str("# JSON-RPC Method Handlers\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_ruby_handler(&mut code, method)?;
        }

        // Generate handler registry
        code.push_str("# ============================================================================\n");
        code.push_str("# Handler Registry\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("HANDLERS = {\n");
        for method in &spec.methods {
            let handler_class = format!("Handle{}", pascal_case(&method.name));
            code.push_str(&format!("  \"{}\" => {}Async,\n", method.name, handler_class));
        }
        code.push_str("}.freeze\n\n");

        // Generate method router
        code.push_str("# ============================================================================\n");
        code.push_str("# Method Router\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("class JsonRpcRouter\n");
        code.push_str("  def self.handle_call(method_name, params, request_id)\n");
        code.push_str("    handler_class = HANDLERS[method_name]\n");
        code.push_str("    return error_response(-32601, \"Method not found\", request_id) unless handler_class\n\n");

        code.push_str("    handler = handler_class.new\n");
        code.push_str("    Async do\n");
        code.push_str("      result = handler.execute(params)\n");
        code.push_str("      {\n");
        code.push_str("        jsonrpc: \"2.0\",\n");
        code.push_str("        result: result,\n");
        code.push_str("        id: request_id\n");
        code.push_str("      }\n");
        code.push_str("    end\n");
        code.push_str("  rescue StandardError => e\n");
        code.push_str("    error_response(-32603, \"Internal error\", request_id, e.message)\n");
        code.push_str("  end\n\n");

        code.push_str("  def self.error_response(code, message, request_id, data = nil)\n");
        code.push_str("    response = {\n");
        code.push_str("      jsonrpc: \"2.0\",\n");
        code.push_str("      error: { code: code, message: message },\n");
        code.push_str("      id: request_id\n");
        code.push_str("    }\n");
        code.push_str("    response[:error][:data] = data if data\n");
        code.push_str("    response\n");
        code.push_str("  end\n");
        code.push_str("end\n\n");

        // Example usage
        code.push_str("# ============================================================================\n");
        code.push_str("# Example Usage\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("if __FILE__ == $0\n");
        code.push_str("  require 'webrick'\n\n");

        code.push_str("  server = WEBrick::HTTPServer.new(Port: 8000)\n");
        code.push_str("  server.mount_proc '/rpc' do |req, res|\n");
        code.push_str("    if req.request_method == 'POST'\n");
        code.push_str("      request = JSON.parse(req.body)\n");
        code.push_str("      method = request['method']\n");
        code.push_str("      params = request['params'] || {}\n");
        code.push_str("      request_id = request['id']\n\n");

        code.push_str("      response = JsonRpcRouter.handle_call(method, params, request_id)\n");
        code.push_str("      res['Content-Type'] = 'application/json'\n");
        code.push_str("      res.body = response.to_json\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");

        code.push_str("  trap('INT') { server.shutdown }\n");
        code.push_str("  server.start\n");
        code.push_str("end\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "ruby"
    }
}

fn generate_ruby_handler(
    code: &mut String,
    method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod,
) -> Result<()> {
    let handler_name = format!("Handle{}", pascal_case(&method.name));

    code.push_str(&format!("class {}Async\n", handler_name));
    code.push_str("  # JSON-RPC handler\n");
    if let Some(summary) = &method.summary {
        code.push_str(&format!("  # {}\n", summary));
    }
    code.push_str("  def execute(params)\n");

    // Validate params
    if !method.params.is_empty() {
        code.push_str("    validate_params(params)\n");
    }

    // TODO comment
    code.push_str("\n    # TODO: Implement business logic\n");
    code.push_str("    # This handler receives parameters and should:\n");
    code.push_str("    # 1. Validate inputs\n");
    code.push_str("    # 2. Execute business logic\n");
    code.push_str("    # 3. Return result as Hash matching schema\n");
    code.push_str("    # 4. Raise appropriate JSON-RPC errors on failure\n\n");

    // Placeholder return
    code.push_str("    # Example return structure (update with real data):\n");
    code.push_str("    result = {}\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            code.push_str(&format!("    result[\"{}\"] = \"TODO\"\n", field_name));
        }
    }
    code.push_str("    result\n");
    code.push_str("  end\n");

    // Parameter validation
    if !method.params.is_empty() {
        code.push_str("\n  private\n\n");
        code.push_str("  def validate_params(params)\n");
        for param in &method.params {
            if param.required {
                code.push_str(&format!(
                    "    raise \"Missing required parameter: {}\" unless params[\"{}\"]\n",
                    param.name, param.name
                ));
            }
        }
        code.push_str("  end\n");
    }

    code.push_str("end\n\n");

    Ok(())
}

fn pascal_case(input: &str) -> String {
    input
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
