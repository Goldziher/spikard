//! Ruby `OpenRPC` code generation.

use anyhow::Result;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

use super::OpenRpcGenerator;

/// Ruby `OpenRPC` code generator
pub struct RubyOpenRpcGenerator;

impl OpenRpcGenerator for RubyOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        code.push_str("#!/usr/bin/env ruby\n");
        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# JSON-RPC 2.0 handlers generated from OpenRPC specification.\n");
        code.push_str("#\n");
        code.push_str("# Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\n");

        code.push_str("require 'json'\n");
        code.push_str("require 'async'\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# JSON-RPC Method Handlers\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_ruby_handler(&mut code, method)?;
        }

        code.push_str("# ============================================================================\n");
        code.push_str("# Handler Registry\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("HANDLERS = {\n");
        for method in &spec.methods {
            let handler_class = format!("Handle{}", pascal_case(&method.name));
            code.push_str(&format!("  \"{}\" => {},\n", method.name, handler_class));
        }
        code.push_str("}.freeze\n\n");

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

    code.push_str(&format!("class {handler_name}\n"));
    code.push_str("  # JSON-RPC 2.0 handler method\n");
    if let Some(summary) = &method.summary {
        let safe_summary = summary.replace('"', "\\\"").replace('\n', " ");
        code.push_str(&format!("  # {safe_summary}\n"));
    }
    code.push_str("  def execute(params)\n");

    if !method.params.is_empty() {
        code.push_str("    validate_params(params)\n");
    }

    // Implementation placeholder
    code.push_str("\n    # TODO: Implement business logic\n");
    code.push_str("    # This handler receives parameters and should:\n");
    code.push_str("    # 1. Validate inputs against the schema\n");
    code.push_str("    # 2. Execute business logic\n");
    code.push_str("    # 3. Return result as Hash matching result schema\n");
    code.push_str("    # 4. Raise appropriate JSON-RPC errors on failure\n\n");

    code.push_str("    # Example return structure (update with actual data):\n");
    code.push_str("    result = {}\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            let safe_field = field_name.replace('"', "\\\"");
            code.push_str(&format!("    result[\"{safe_field}\"] = nil  # TODO: implement\n"));
        }
        if props.len() > 3 {
            code.push_str("    # ... add remaining fields\n");
        }
    }
    code.push_str("    result\n");
    code.push_str("  end\n");

    if !method.params.is_empty() {
        code.push_str("\n  private\n\n");
        code.push_str("  def validate_params(params)\n");
        for param in &method.params {
            if param.required {
                code.push_str(&format!(
                    "    raise JsonRpcError.invalid_params(\"Missing required parameter: {}\") unless params[\"{}\"]\n",
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
        .collect::<String>()
}
