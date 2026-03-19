//! Ruby `OpenRPC` code generation.

use anyhow::Result;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

use super::OpenRpcGenerator;

/// Ruby `OpenRPC` code generator
pub struct RubyOpenRpcGenerator;

impl OpenRpcGenerator for RubyOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# JSON-RPC 2.0 handlers generated from OpenRPC specification.\n");
        code.push_str("#\n");
        code.push_str("# Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\n");

        code.push_str("require 'json'\n");
        code.push_str("require 'webrick'\n\n");

        code.push_str("# Represents a JSON-RPC error that should be returned to the client.\n");
        code.push_str("class JsonRpcError < StandardError\n");
        code.push_str("  attr_reader :code, :data\n\n");
        code.push_str("  def initialize(code, message, data: nil)\n");
        code.push_str("    super(message)\n");
        code.push_str("    @code = code\n");
        code.push_str("    @data = data\n");
        code.push_str("  end\n\n");
        code.push_str("  def self.invalid_params(message)\n");
        code.push_str("    new(-32_602, message)\n");
        code.push_str("  end\n");
        code.push_str("end\n\n");

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
        let handler_entries = spec
            .methods
            .iter()
            .map(|method| {
                let handler_class = format!("Handle{}", pascal_case(&method.name));
                format!("  '{}' => {handler_class}", escape_single_quoted(&method.name))
            })
            .collect::<Vec<_>>();
        if !handler_entries.is_empty() {
            code.push_str(&handler_entries.join(",\n"));
            code.push('\n');
        }
        code.push_str("}.freeze\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# Method Router\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("# Routes JSON-RPC requests to the generated handler classes.\n");
        code.push_str("class JsonRpcRouter\n");
        code.push_str("  def self.handle_call(method_name, params, request_id)\n");
        code.push_str("    handler_class = HANDLERS[method_name]\n");
        code.push_str("    return method_not_found_response(request_id) unless handler_class\n\n");
        code.push_str("    result = handler_class.new.execute(normalize_params(params))\n");
        code.push_str("    success_response(result, request_id)\n");
        code.push_str("  rescue JsonRpcError => e\n");
        code.push_str("    error_response(e.code, e.message, request_id, e.data)\n");
        code.push_str("  rescue StandardError => e\n");
        code.push_str("    error_response(-32_603, 'Internal error', request_id, e.message)\n");
        code.push_str("  end\n\n");

        code.push_str("  class << self\n");
        code.push_str("    private\n\n");
        code.push_str("    def normalize_params(params)\n");
        code.push_str("      return {} if params.nil?\n");
        code.push_str("      return params if params.is_a?(Hash)\n\n");
        code.push_str("      raise JsonRpcError.invalid_params('Params must be a JSON object')\n");
        code.push_str("    end\n\n");
        code.push_str("    def success_response(result, request_id)\n");
        code.push_str("      {\n");
        code.push_str("        jsonrpc: '2.0',\n");
        code.push_str("        result: result,\n");
        code.push_str("        id: request_id\n");
        code.push_str("      }\n");
        code.push_str("    end\n\n");
        code.push_str("    def method_not_found_response(request_id)\n");
        code.push_str("      error_response(-32_601, 'Method not found', request_id)\n");
        code.push_str("    end\n\n");
        code.push_str("    def error_response(code, message, request_id, data = nil)\n");
        code.push_str("      response = {\n");
        code.push_str("        jsonrpc: '2.0',\n");
        code.push_str("        error: { code: code, message: message },\n");
        code.push_str("        id: request_id\n");
        code.push_str("      }\n");
        code.push_str("      response[:error][:data] = data if data\n");
        code.push_str("      response\n");
        code.push_str("    end\n");
        code.push_str("  end\n");
        code.push_str("end\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# Example Usage\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("if $PROGRAM_NAME == __FILE__\n");
        code.push_str("  server = WEBrick::HTTPServer.new(Port: 8000)\n");
        code.push_str("  server.mount_proc '/rpc' do |req, res|\n");
        code.push_str("    unless req.request_method == 'POST'\n");
        code.push_str("      res.status = 405\n");
        code.push_str("      res.body = { error: 'Only POST is supported' }.to_json\n");
        code.push_str("      next\n");
        code.push_str("    end\n\n");
        code.push_str("    request = JSON.parse(req.body)\n");
        code.push_str(
            "    response = JsonRpcRouter.handle_call(request['method'], request['params'], request['id'])\n",
        );
        code.push_str("    res['Content-Type'] = 'application/json'\n");
        code.push_str("    res.body = response.to_json\n");
        code.push_str("  rescue JSON::ParserError => e\n");
        code.push_str("    res.status = 400\n");
        code.push_str("    res['Content-Type'] = 'application/json'\n");
        code.push_str("    res.body = JsonRpcRouter.error_response(-32_700, 'Parse error', nil, e.message).to_json\n");
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

    code.push_str(&format!(
        "# Handles the '{}' JSON-RPC method.\n",
        escape_single_quoted(&method.name)
    ));
    code.push_str(&format!("class {handler_name}\n"));
    code.push_str("  # Executes the JSON-RPC method implementation.\n");
    if let Some(summary) = &method.summary {
        let safe_summary = escape_comment(summary);
        code.push_str(&format!("  # {safe_summary}\n"));
    }
    code.push_str("  def execute(params)\n");

    if !method.params.is_empty() {
        code.push_str("    validate_params(params)\n");
    }

    code.push_str(&format!(
        "    raise NotImplementedError, 'Implement {handler_name}#execute for {}'\n",
        escape_single_quoted(&method.name)
    ));
    code.push_str("  end\n");

    if !method.params.is_empty() {
        let required_params = method.params.iter().filter(|param| param.required).collect::<Vec<_>>();
        code.push_str("\n  private\n\n");
        if required_params.is_empty() {
            code.push_str("  def validate_params(_params); end\n");
        } else {
            code.push_str("  def validate_params(params)\n");
            for param in required_params {
                code.push_str(&format!(
                    "    raise JsonRpcError.invalid_params('Missing required parameter: {}') unless params.key?('{}')\n",
                    escape_single_quoted(&param.name),
                    escape_single_quoted(&param.name)
                ));
            }
            code.push_str("  end\n");
        }
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

fn escape_single_quoted(input: &str) -> String {
    input.replace('\\', "\\\\").replace('\'', "\\'")
}

fn escape_comment(input: &str) -> String {
    input.replace('\n', " ")
}
