//! Elixir `OpenRPC` code generation.

use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::codegen::openrpc::spec_parser::{OpenRpcSpec, resolve_schema, schema_ref_name};

use super::OpenRpcGenerator;

/// Elixir `OpenRPC` code generator.
pub struct ElixirOpenRpcGenerator;

impl OpenRpcGenerator for ElixirOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();
        let module_name = root_module_name(spec);
        let types_module_name = format!("{module_name}.Types");
        let has_component_types = !spec.components.schemas.is_empty();
        let needs_types_alias = has_component_types
            && spec
                .methods
                .iter()
                .any(|method| schema_uses_component_ref(&params_schema(method)) || schema_uses_component_ref(&method.result.schema));

        if has_component_types {
            code.push_str(&generate_types_module(spec, &types_module_name));
        }

        code.push_str(&format!(
            r#"defmodule {module_name} do
  @moduledoc """
  JSON-RPC 2.0 handlers generated from OpenRPC specification.

  Generated from: {} v{}
  """

  use Spikard.Router

  alias Spikard.Request
  alias Spikard.Response
  alias {module_name}.Handlers

  post("/rpc", &__MODULE__.handle_rpc/1)

  @type request_id :: String.t() | integer() | nil
  @type jsonrpc_response :: map()

  @spec handle_rpc(Request.t()) :: Response.t()
  def handle_rpc(request) do
    body = Request.get_body(request)

    response =
      case body do
        %{{"jsonrpc" => "2.0", "method" => method}} = payload when is_binary(method) ->
          params = Map.get(payload, "params")
          request_id = Map.get(payload, "id")
          handle_jsonrpc_call(method, params, request_id)

        %{{"method" => _}} = payload ->
          error_response(-32600, "Invalid request", Map.get(payload, "id"))

        _ ->
          error_response(-32600, "Invalid request", nil)
      end

    Response.json(response)
  end

  @spec handle_jsonrpc_call(String.t(), term(), request_id()) :: jsonrpc_response()
  def handle_jsonrpc_call(method_name, params, request_id) do
    try do
      case normalize_params(params, request_id) do
        {{:ok, normalized_params}} ->
          dispatch(method_name, normalized_params, request_id)

        {{:error, response}} ->
          response
      end
    rescue
      error ->
        error_response(-32603, "Internal error", request_id, Exception.message(error))
    end
  end

  @spec dispatch(String.t(), map(), request_id()) :: jsonrpc_response()
  defp dispatch(method_name, params, request_id) do
    case method_name do
"#,
            spec.info.title, spec.info.version
        ));

        for method in &spec.methods {
            let handler_name = handler_name(&method.name);
            code.push_str(&format!(
                "      \"{}\" ->\n        case Handlers.{}(params) do\n          {{:ok, result}} -> success_response(result, request_id)\n          {{:error, code, message, data}} -> error_response(code, message, request_id, data)\n        end\n",
                escape_string(&method.name),
                handler_name
            ));
        }

        code.push_str(
            r#"      _ ->
        error_response(-32601, "Method not found", request_id)
    end
  end

  @spec normalize_params(term(), request_id()) :: {:ok, map()} | {:error, jsonrpc_response()}
  defp normalize_params(nil, _request_id), do: {:ok, %{}}
  defp normalize_params(params, _request_id) when is_map(params), do: {:ok, params}

  defp normalize_params(_params, request_id) do
    {:error, error_response(-32602, "Params must be a JSON object", request_id)}
  end

  @spec success_response(term(), request_id()) :: jsonrpc_response()
  defp success_response(result, request_id) do
    %{"jsonrpc" => "2.0", "result" => result, "id" => request_id}
  end

  @spec error_response(integer(), String.t(), request_id(), term()) :: jsonrpc_response()
  defp error_response(code, message, request_id, data \\ nil) do
    error = %{"code" => code, "message" => message}
    error = if is_nil(data), do: error, else: Map.put(error, "data", data)

    %{"jsonrpc" => "2.0", "error" => error, "id" => request_id}
  end
end

"#,
        );

        code.push_str(&format!("defmodule {module_name}.Handlers do\n  @moduledoc false\n\n"));
        if needs_types_alias {
            code.push_str(&format!("  alias {types_module_name}, as: Types\n\n"));
        }

        for method in &spec.methods {
            let type_base = method_type_name(&method.name);
            let params_type_name = format!("{type_base}_params");
            let result_type_name = format!("{type_base}_result");
            let required_params = method
                .params
                .iter()
                .filter(|param| param.required)
                .map(|param| format!("\"{}\"", escape_string(&param.name)))
                .collect::<Vec<_>>();

            code.push_str(&format!(
                "  @typedoc \"Parameters for {}.\"\n  @type {} :: {}\n\n",
                escape_string(&method.name),
                params_type_name,
                schema_to_typespec(spec, &params_schema(method), false, Some("Types"))
            ));
            code.push_str(&format!(
                "  @typedoc \"Result for {}.\"\n  @type {} :: {}\n",
                escape_string(&method.name),
                result_type_name,
                schema_to_typespec(spec, &method.result.schema, false, Some("Types"))
            ));
            code.push_str(&format!(
                "  @spec {}({}) :: {{:ok, {}}} | {{:error, integer(), String.t(), term()}}\n",
                handler_name(&method.name),
                params_type_name,
                result_type_name
            ));
            code.push_str(&format!("  def {}(params) do\n", handler_name(&method.name)));
            code.push_str("    case params do\n");
            code.push_str("      params when is_map(params) ->\n");
            if required_params.is_empty() {
                code.push_str("        result = ");
                code.push_str(&render_elixir_value(
                    &placeholder_from_schema(spec, &method.result.schema),
                    4,
                ));
                code.push_str("\n        {:ok, result}\n");
            } else {
                code.push_str(&format!(
                    "        with :ok <- require_params(params, [{}]) do\n",
                    required_params.join(", ")
                ));
                code.push_str("          result = ");
                code.push_str(&render_elixir_value(
                    &placeholder_from_schema(spec, &method.result.schema),
                    5,
                ));
                code.push_str("\n          {:ok, result}\n");
                code.push_str("        end\n");
            }
            code.push_str("      _ ->\n");
            code.push_str("        {:error, -32602, \"Params must be a JSON object\", nil}\n");
            code.push_str("    end\n");

            code.push_str("  end\n\n");
        }

        code.push_str(
            r#"  @spec require_params(map(), [String.t()]) :: :ok | {:error, integer(), String.t(), nil}
  defp require_params(params, required_names) do
    case Enum.find(required_names, fn name -> !Map.has_key?(params, name) end) do
      nil -> :ok
      missing -> {:error, -32602, "Missing required parameter: #{missing}", nil}
    end
  end
end
"#,
        );

        Ok(format_elixir(&code))
    }

    fn language_name(&self) -> &'static str {
        "elixir"
    }
}

fn root_module_name(spec: &OpenRpcSpec) -> String {
    let base = spec
        .info
        .title
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    match base.as_str() {
        "" => "GeneratedJsonRpc".to_string(),
        value => format!("{}JsonRpc", value.to_pascal_case()),
    }
}

fn handler_name(method_name: &str) -> String {
    format!("handle_{}", method_name.replace(['.', '-'], "_").to_snake_case())
}

fn method_type_name(method_name: &str) -> String {
    method_name.replace(['.', '-'], "_").to_snake_case()
}

fn component_type_name(component_name: &str) -> String {
    component_name.to_snake_case()
}

fn generate_types_module(spec: &OpenRpcSpec, module_name: &str) -> String {
    let mut code = String::new();
    code.push_str(&format!("defmodule {module_name} do\n  @moduledoc false\n\n"));

    let components = spec
        .components
        .schemas
        .iter()
        .map(|(name, schema)| (name.clone(), schema))
        .collect::<BTreeMap<_, _>>();

    for (name, schema) in components {
        code.push_str(&format!(
            "  @typedoc \"OpenRPC schema for {}.\"\n  @type {} :: {}\n\n",
            escape_string(&name),
            component_type_name(&name),
            schema_to_typespec(spec, schema, false, None)
        ));
    }

    code.push_str("end\n\n");
    code
}

fn params_schema(method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod) -> Value {
    let mut properties = Map::new();
    let mut required = Vec::new();

    for param in &method.params {
        properties.insert(param.name.clone(), param.schema.clone());
        if param.required {
            required.push(Value::String(param.name.clone()));
        }
    }

    let mut result = Map::new();
    result.insert("type".to_string(), Value::String("object".to_string()));
    result.insert("properties".to_string(), Value::Object(properties));
    if !required.is_empty() {
        result.insert("required".to_string(), Value::Array(required));
    }
    Value::Object(result)
}

fn schema_uses_component_ref(schema: &Value) -> bool {
    match schema {
        Value::Object(map) => map.contains_key("$ref") || map.values().any(schema_uses_component_ref),
        Value::Array(values) => values.iter().any(schema_uses_component_ref),
        _ => false,
    }
}

fn json_key_typespec(name: &str) -> String {
    format!(":\"{}\"", escape_string(name))
}

fn schema_to_typespec(spec: &OpenRpcSpec, schema: &Value, nullable: bool, ref_prefix: Option<&str>) -> String {
    let resolved = resolve_schema(spec, schema);

    let base = if let Some(reference_name) = schema_ref_name(schema) {
        let type_name = component_type_name(reference_name);
        match ref_prefix {
            Some(prefix) => format!("{prefix}.{type_name}()"),
            None => format!("{type_name}()"),
        }
    } else if let Some(enum_values) = resolved.get("enum").and_then(Value::as_array) {
        if enum_values.is_empty() {
            "String.t()".to_string()
        } else {
            match resolved.get("type").and_then(Value::as_str) {
                Some("integer") => "integer()".to_string(),
                Some("number") => "float()".to_string(),
                Some("boolean") => "boolean()".to_string(),
                _ => "String.t()".to_string(),
            }
        }
    } else {
        match resolved.get("type").and_then(Value::as_str) {
            Some("string") => "String.t()".to_string(),
            Some("integer") => "integer()".to_string(),
            Some("number") => "float()".to_string(),
            Some("boolean") => "boolean()".to_string(),
            Some("array") => {
                let item_type = resolved
                    .get("items")
                    .map(|item| schema_to_typespec(spec, item, false, ref_prefix))
                    .unwrap_or_else(|| "term()".to_string());
                format!("[{item_type}]")
            }
            Some("object") => object_typespec(spec, resolved, ref_prefix),
            _ => "term()".to_string(),
        }
    };

    if nullable { format!("{base} | nil") } else { base }
}

fn object_typespec(spec: &OpenRpcSpec, schema: &Value, ref_prefix: Option<&str>) -> String {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return "map()".to_string();
    };

    if properties.is_empty() {
        return "map()".to_string();
    }

    let required_names = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let fields = properties
        .iter()
        .map(|(name, value)| {
            let key = if required_names.iter().any(|required| required == name) {
                format!("required({})", json_key_typespec(name))
            } else {
                format!("optional({})", json_key_typespec(name))
            };
            let field_type = schema_to_typespec(
                spec,
                value,
                !required_names.iter().any(|required| required == name),
                ref_prefix,
            );
            format!("{key} => {field_type}")
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!("%{{{fields}}}")
}

fn placeholder_from_schema(spec: &OpenRpcSpec, schema: &Value) -> Value {
    let resolved = resolve_schema(spec, schema);

    if let Some(example) = resolved.get("example") {
        return example.clone();
    }

    if let Some(enum_values) = resolved.get("enum").and_then(Value::as_array)
        && let Some(first) = enum_values.first()
    {
        return first.clone();
    }

    match resolved.get("type").and_then(Value::as_str) {
        Some("string") => Value::String("TODO".to_string()),
        Some("integer") => Value::Number(Number::from(0)),
        Some("number") => Value::Number(Number::from_f64(0.0).unwrap_or_else(|| Number::from(0))),
        Some("boolean") => Value::Bool(false),
        Some("array") => {
            let items = resolved
                .get("items")
                .map(|item| placeholder_from_schema(spec, item))
                .map(|value| vec![value])
                .unwrap_or_default();
            Value::Array(items)
        }
        Some("object") => {
            let mut object = Map::new();
            if let Some(properties) = resolved.get("properties").and_then(Value::as_object) {
                for (name, value) in properties {
                    object.insert(name.clone(), placeholder_from_schema(spec, value));
                }
            }
            Value::Object(object)
        }
        _ => Value::Null,
    }
}

fn render_elixir_value(value: &Value, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    let child_indent = "  ".repeat(indent_level + 1);

    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(boolean) => boolean.to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(string) => format!("\"{}\"", escape_string(string)),
        Value::Array(items) => {
            if items.is_empty() {
                "[]".to_string()
            } else {
                let rendered = items
                    .iter()
                    .map(|item| format!("{child_indent}{}", render_elixir_value(item, indent_level + 1)))
                    .collect::<Vec<_>>()
                    .join(",\n");
                format!("[\n{rendered}\n{indent}]")
            }
        }
        Value::Object(map) => {
            if map.is_empty() {
                "%{}".to_string()
            } else {
                let rendered = map
                    .iter()
                    .map(|(key, item)| {
                        format!(
                            "{child_indent}\"{}\" => {}",
                            escape_string(key),
                            render_elixir_value(item, indent_level + 1)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",\n");
                format!("%{{\n{rendered}\n{indent}}}")
            }
        }
    }
}

fn format_elixir(code: &str) -> String {
    let mut command = match Command::new("elixir")
        .arg("-e")
        .arg(
            r#"input = IO.read(:stdio, :all)
IO.write(IO.iodata_to_binary(Code.format_string!(input, line_length: 120)))"#,
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(command) => command,
        Err(_) => return ensure_trailing_newline(code.to_string()),
    };

    let Some(stdin) = command.stdin.as_mut() else {
        return ensure_trailing_newline(code.to_string());
    };
    if stdin.write_all(code.as_bytes()).is_err() {
        return ensure_trailing_newline(code.to_string());
    }

    match command.wait_with_output() {
        Ok(output) if output.status.success() => {
            ensure_trailing_newline(String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string()))
        }
        _ => ensure_trailing_newline(code.to_string()),
    }
}

fn ensure_trailing_newline(mut code: String) -> String {
    if !code.ends_with('\n') {
        code.push('\n');
    }
    code
}

fn escape_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}
