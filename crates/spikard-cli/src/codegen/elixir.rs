//! Elixir code generation from `OpenAPI` schemas.

use super::ElixirDtoStyle;
use super::base::OpenApiGenerator;
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use openapiv3::{
    OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, Schema, SchemaKind,
    StatusCode, StringFormat, Type, VariantOrUnknownOrEmpty,
};
use serde_json::{Map, Number, Value};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::codegen::SchemaRegistry;

pub struct ElixirGenerator {
    spec: OpenAPI,
    registry: SchemaRegistry,
    style: ElixirDtoStyle,
}

#[derive(Default)]
struct ElixirParamHelperUsage {
    uuid: bool,
    integer: bool,
    float: bool,
    boolean: bool,
    date: bool,
    datetime: bool,
    enum_values: bool,
}

impl ElixirGenerator {
    pub fn new(spec: OpenAPI, style: ElixirDtoStyle) -> Self {
        let registry = SchemaRegistry::from_spec(&spec);
        Self { spec, registry, style }
    }

    fn root_module_name(&self) -> String {
        let base = self
            .spec
            .info
            .title
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        match base.as_str() {
            "" => "GeneratedApi".to_string(),
            value => {
                let module = value.to_pascal_case();
                if module.ends_with("Api") {
                    module
                } else {
                    format!("{module}Api")
                }
            }
        }
    }

    fn schema_type_name(&self, name: &str) -> String {
        name.to_snake_case()
    }

    fn route_path(&self, path: &str) -> String {
        let mut route = path.to_string();
        for segment in path.split('/') {
            if segment.starts_with('{') && segment.ends_with('}') {
                let name = segment.trim_matches(|c| c == '{' || c == '}');
                route = route.replace(&format!("{{{name}}}"), &format!(":{}", name.to_snake_case()));
            }
        }
        route
    }

    fn escape_string(&self, value: &str) -> String {
        value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
    }

    fn render_elixir_value(&self, value: &Value, indent_level: usize) -> String {
        let indent = "  ".repeat(indent_level);
        let child_indent = "  ".repeat(indent_level + 1);

        match value {
            Value::Null => "nil".to_string(),
            Value::Bool(boolean) => boolean.to_string(),
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{}\"", self.escape_string(string)),
            Value::Array(items) => {
                if items.is_empty() {
                    "[]".to_string()
                } else {
                    let rendered = items
                        .iter()
                        .map(|item| format!("{child_indent}{}", self.render_elixir_value(item, indent_level + 1)))
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
                                self.escape_string(key),
                                self.render_elixir_value(item, indent_level + 1)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",\n");
                    format!("%{{\n{rendered}\n{indent}}}")
                }
            }
        }
    }

    fn render_schema_literal(&self, schema: &Schema) -> Result<String> {
        let value = serde_json::to_value(schema)?;
        Ok(self.render_elixir_value(&value, 1))
    }

    fn resolve_boxed_schema<'a>(&'a self, schema_ref: &'a ReferenceOr<Box<Schema>>) -> Option<&'a Schema> {
        match schema_ref {
            ReferenceOr::Item(schema) => Some(schema.as_ref()),
            ReferenceOr::Reference { reference } => self.registry.resolve_reference(reference),
        }
    }

    fn safe_required_key(&self, name: &str) -> String {
        let atom_name = name.to_snake_case();

        if atom_name
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || (index > 0 && ch == '?'))
            && !atom_name.is_empty()
            && !atom_name.starts_with(|c: char| c.is_ascii_digit())
        {
            format!(":{atom_name}")
        } else {
            "String.t()".to_string()
        }
    }

    fn schema_to_typespec(&self, schema: &Schema, nullable: bool) -> String {
        let base = match &schema.schema_kind {
            SchemaKind::Type(Type::String(string_type)) => {
                if string_type.enumeration.iter().flatten().next().is_none() {
                    "String.t()".to_string()
                } else {
                    "String.t()".to_string()
                }
            }
            SchemaKind::Type(Type::Number(_)) => "float()".to_string(),
            SchemaKind::Type(Type::Integer(_)) => "integer()".to_string(),
            SchemaKind::Type(Type::Boolean(_)) => "boolean()".to_string(),
            SchemaKind::Type(Type::Array(array)) => {
                let item_type = array
                    .items
                    .as_ref()
                    .and_then(|item| self.resolve_boxed_schema(item))
                    .map_or_else(|| "term()".to_string(), |item| self.schema_to_typespec(item, false));
                format!("[{item_type}]")
            }
            SchemaKind::Type(Type::Object(object)) => {
                if object.properties.is_empty() {
                    "map()".to_string()
                } else {
                    let fields = object
                        .properties
                        .iter()
                        .map(|(name, schema_ref)| {
                            let resolved = self.resolve_boxed_schema(schema_ref);
                            let field_type = resolved
                                .map(|item| self.schema_to_typespec(item, !object.required.contains(name)))
                                .unwrap_or_else(|| "term()".to_string());
                            let key_type = if object.required.contains(name) {
                                format!("required({})", self.safe_required_key(name))
                            } else {
                                format!("optional({})", self.safe_required_key(name))
                            };
                            format!("{key_type} => {field_type}")
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("%{{{fields}}}")
                }
            }
            SchemaKind::AllOf { .. } | SchemaKind::AnyOf { .. } | SchemaKind::OneOf { .. } => "map()".to_string(),
            _ => "term()".to_string(),
        };

        if nullable || schema.schema_data.nullable {
            format!("{base} | nil")
        } else {
            base
        }
    }

    fn schema_placeholder(&self, schema: &Schema) -> Value {
        if let Some(example) = schema.schema_data.example.clone() {
            return example;
        }

        match &schema.schema_kind {
            SchemaKind::Type(Type::String(string_type)) => {
                if let Some(first) = string_type.enumeration.iter().flatten().next() {
                    Value::String(first.clone())
                } else {
                    Value::String("TODO".to_string())
                }
            }
            SchemaKind::Type(Type::Number(_)) => Value::Number(Number::from_f64(0.0).unwrap()),
            SchemaKind::Type(Type::Integer(_)) => Value::Number(Number::from(0)),
            SchemaKind::Type(Type::Boolean(_)) => Value::Bool(false),
            SchemaKind::Type(Type::Array(array)) => {
                if let Some(item) = &array.items
                    && let Some(resolved) = self.resolve_boxed_schema(item)
                {
                    Value::Array(vec![self.schema_placeholder(resolved)])
                } else {
                    Value::Array(vec![])
                }
            }
            SchemaKind::Type(Type::Object(object)) => {
                let mut map = Map::new();
                for (name, schema_ref) in &object.properties {
                    let value = self
                        .resolve_boxed_schema(schema_ref)
                        .map(|item| self.schema_placeholder(item))
                        .unwrap_or(Value::Null);
                    map.insert(name.clone(), value);
                }
                Value::Object(map)
            }
            _ => Value::Null,
        }
    }

    fn parameter_schema(&self, operation: &Operation) -> Option<Schema> {
        let mut properties = Map::new();
        let mut required = Vec::new();

        for parameter_ref in &operation.parameters {
            let ReferenceOr::Item(parameter) = parameter_ref else {
                continue;
            };

            match parameter {
                Parameter::Path { parameter_data, .. }
                | Parameter::Query { parameter_data, .. }
                | Parameter::Header { parameter_data, .. }
                | Parameter::Cookie { parameter_data, .. } => {
                    let openapiv3::ParameterSchemaOrContent::Schema(schema_ref) = &parameter_data.format else {
                        continue;
                    };
                    let Some(schema) = self.registry.resolve(schema_ref) else {
                        continue;
                    };
                    let Ok(value) = serde_json::to_value(schema) else {
                        continue;
                    };
                    properties.insert(parameter_data.name.clone(), value);
                    if parameter_data.required {
                        required.push(Value::String(parameter_data.name.clone()));
                    }
                }
            }
        }

        if properties.is_empty() {
            return None;
        }

        let schema_json = Value::Object(Map::from_iter([
            ("type".to_string(), Value::String("object".to_string())),
            ("properties".to_string(), Value::Object(properties)),
            ("required".to_string(), Value::Array(required)),
        ]));

        serde_json::from_value(schema_json).ok()
    }

    fn request_body_schema<'a>(&'a self, operation: &'a Operation) -> Option<&'a Schema> {
        let body = operation.request_body.as_ref()?;
        let request_body = match body {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { reference } => {
                return self.registry.resolve_reference(reference);
            }
        };
        let media_type = request_body.content.get("application/json")?;
        media_type
            .schema
            .as_ref()
            .and_then(|schema_ref| self.registry.resolve(schema_ref))
    }

    fn response_schema<'a>(&'a self, operation: &'a Operation) -> Option<(u16, &'a Schema)> {
        let response = operation
            .responses
            .responses
            .iter()
            .find_map(|(status, response_ref)| match status {
                StatusCode::Code(code) if (200..300).contains(code) => Some((*code, response_ref)),
                StatusCode::Range(2) => Some((200, response_ref)),
                _ => None,
            })?;

        let status = response.0;
        let response = match response.1 {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { reference } => {
                return self
                    .registry
                    .resolve_reference(reference)
                    .map(|schema| (status, schema));
            }
        };

        let media_type = response.content.get("application/json")?;
        media_type
            .schema
            .as_ref()
            .and_then(|schema_ref| self.registry.resolve(schema_ref))
            .map(|schema| (status, schema))
    }

    fn route_options(&self, operation_id: &str, operation: &Operation) -> Result<(String, Vec<String>)> {
        let mut prelude = Vec::new();
        let mut options = Vec::new();

        if let Some(parameter_schema) = self.parameter_schema(operation) {
            let attr_name = format!("{operation_id}_params_schema");
            prelude.push(format!(
                "  @{} {}\n",
                attr_name,
                self.render_schema_literal(&parameter_schema)?
            ));
            options.push(format!("parameter_schema: @{}", attr_name));
        }

        if let Some(schema) = self.request_body_schema(operation) {
            let attr_name = format!("{operation_id}_request_schema");
            prelude.push(format!("  @{} {}\n", attr_name, self.render_schema_literal(schema)?));
            options.push(format!("request_schema: @{}", attr_name));
        }

        if let Some((_, schema)) = self.response_schema(operation) {
            let attr_name = format!("{operation_id}_response_schema");
            prelude.push(format!("  @{} {}\n", attr_name, self.render_schema_literal(schema)?));
            options.push(format!("response_schema: @{}", attr_name));
        }

        Ok((prelude.join(""), options))
    }

    fn parameter_binding(&self, parameter: &Parameter) -> Option<String> {
        let (parameter_data, getter) = match parameter {
            Parameter::Path { parameter_data, .. } => (parameter_data, "get_path_param"),
            Parameter::Query { parameter_data, .. } => (parameter_data, "get_query_param"),
            Parameter::Header { parameter_data, .. } => (parameter_data, "get_header"),
            Parameter::Cookie { parameter_data, .. } => (parameter_data, "get_cookie"),
        };

        let variable = format!("_{}", parameter_data.name.to_snake_case());
        let access = format!("Spikard.Request.{getter}(request, \"{}\")", parameter_data.name);
        Some(format!(
            "    {} = {}\n",
            variable,
            self.parameter_coercion_expr(parameter_data, &access)
        ))
    }

    fn parameter_coercion_expr(&self, parameter_data: &ParameterData, access: &str) -> String {
        match &parameter_data.format {
            ParameterSchemaOrContent::Schema(schema_ref) => {
                self.schema_param_coercion_expr(schema_ref, access, &parameter_data.name)
            }
            ParameterSchemaOrContent::Content(_) => access.to_string(),
        }
    }

    fn schema_param_coercion_expr(&self, schema_ref: &ReferenceOr<Schema>, access: &str, name: &str) -> String {
        let Some(schema) = self.registry.resolve(schema_ref) else {
            return access.to_string();
        };
        self.inline_schema_param_coercion_expr(schema, access, name)
    }

    fn inline_schema_param_coercion_expr(&self, schema: &Schema, access: &str, name: &str) -> String {
        match &schema.schema_kind {
            SchemaKind::Type(Type::String(string_type)) => {
                let enum_values = string_type
                    .enumeration
                    .iter()
                    .flatten()
                    .map(|value| format!("\"{}\"", self.escape_string(value)))
                    .collect::<Vec<_>>();

                if !enum_values.is_empty() {
                    return format!(
                        "coerce_enum_param!({}, \"{}\", [{}])",
                        access,
                        name,
                        enum_values.join(", ")
                    );
                }

                match &string_type.format {
                    VariantOrUnknownOrEmpty::Item(StringFormat::Date) => {
                        format!("coerce_date_param!({}, \"{}\")", access, name)
                    }
                    VariantOrUnknownOrEmpty::Item(StringFormat::DateTime) => {
                        format!("coerce_datetime_param!({}, \"{}\")", access, name)
                    }
                    VariantOrUnknownOrEmpty::Unknown(format) if format == "uuid" => {
                        format!("coerce_uuid_param!({}, \"{}\")", access, name)
                    }
                    _ => access.to_string(),
                }
            }
            SchemaKind::Type(Type::Integer(_)) => format!("coerce_integer_param!({}, \"{}\")", access, name),
            SchemaKind::Type(Type::Number(_)) => format!("coerce_float_param!({}, \"{}\")", access, name),
            SchemaKind::Type(Type::Boolean(_)) => format!("coerce_boolean_param!({}, \"{}\")", access, name),
            _ => access.to_string(),
        }
    }

    fn collect_param_helper_usage(&self, operation: &Operation, usage: &mut ElixirParamHelperUsage) {
        for parameter_ref in &operation.parameters {
            let ReferenceOr::Item(parameter) = parameter_ref else {
                continue;
            };

            let parameter_data = match parameter {
                Parameter::Path { parameter_data, .. }
                | Parameter::Query { parameter_data, .. }
                | Parameter::Header { parameter_data, .. }
                | Parameter::Cookie { parameter_data, .. } => parameter_data,
            };

            let ParameterSchemaOrContent::Schema(schema_ref) = &parameter_data.format else {
                continue;
            };
            let Some(schema) = self.registry.resolve(schema_ref) else {
                continue;
            };

            self.collect_schema_helper_usage(schema, usage);
        }
    }

    fn collect_schema_helper_usage(&self, schema: &Schema, usage: &mut ElixirParamHelperUsage) {
        match &schema.schema_kind {
            SchemaKind::Type(Type::String(string_type)) => {
                if string_type.enumeration.iter().flatten().next().is_some() {
                    usage.enum_values = true;
                }
                match &string_type.format {
                    VariantOrUnknownOrEmpty::Item(StringFormat::Date) => usage.date = true,
                    VariantOrUnknownOrEmpty::Item(StringFormat::DateTime) => usage.datetime = true,
                    VariantOrUnknownOrEmpty::Unknown(format) if format == "uuid" => usage.uuid = true,
                    _ => {}
                }
            }
            SchemaKind::Type(Type::Integer(_)) => usage.integer = true,
            SchemaKind::Type(Type::Number(_)) => usage.float = true,
            SchemaKind::Type(Type::Boolean(_)) => usage.boolean = true,
            _ => {}
        }
    }

    fn render_param_helpers(&self, usage: &ElixirParamHelperUsage) -> String {
        let mut helpers = String::new();

        if usage.uuid {
            helpers.push_str(
                r#"  @uuid_regex ~r/\A[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\z/

  defp coerce_uuid_param!(nil, _name), do: nil

  defp coerce_uuid_param!(value, name) do
    if Regex.match?(@uuid_regex, value) do
      value
    else
      invalid_parameter!(name, "must be a UUID")
    end
  end

"#,
            );
        }

        if usage.integer {
            helpers.push_str(
                r#"  defp coerce_integer_param!(nil, _name), do: nil

  defp coerce_integer_param!(value, name) do
    case Integer.parse(value) do
      {integer, ""} -> integer
      _ -> invalid_parameter!(name, "must be an integer")
    end
  end

"#,
            );
        }

        if usage.float {
            helpers.push_str(
                r#"  defp coerce_float_param!(nil, _name), do: nil

  defp coerce_float_param!(value, name) do
    case Float.parse(value) do
      {float, ""} -> float
      _ -> invalid_parameter!(name, "must be a float")
    end
  end

"#,
            );
        }

        if usage.boolean {
            helpers.push_str(
                r#"  defp coerce_boolean_param!(nil, _name), do: nil
  defp coerce_boolean_param!(true, _name), do: true
  defp coerce_boolean_param!(false, _name), do: false
  defp coerce_boolean_param!("true", _name), do: true
  defp coerce_boolean_param!("false", _name), do: false
  defp coerce_boolean_param!("1", _name), do: true
  defp coerce_boolean_param!("0", _name), do: false
  defp coerce_boolean_param!(_value, name), do: invalid_parameter!(name, "must be a boolean")

"#,
            );
        }

        if usage.date {
            helpers.push_str(
                r#"  defp coerce_date_param!(nil, _name), do: nil

  defp coerce_date_param!(value, name) do
    case Date.from_iso8601(value) do
      {:ok, date} -> date
      {:error, _reason} -> invalid_parameter!(name, "must be an ISO 8601 date")
    end
  end

"#,
            );
        }

        if usage.datetime {
            helpers.push_str(
                r#"  defp coerce_datetime_param!(nil, _name), do: nil

  defp coerce_datetime_param!(value, name) do
    case DateTime.from_iso8601(value) do
      {:ok, datetime, _offset} -> datetime
      {:error, _reason} -> invalid_parameter!(name, "must be an ISO 8601 date-time")
    end
  end

"#,
            );
        }

        if usage.enum_values {
            helpers.push_str(
                r#"  defp coerce_enum_param!(nil, _name, _allowed), do: nil

  defp coerce_enum_param!(value, name, allowed) do
    if value in allowed do
      value
    else
      invalid_parameter!(name, "must be one of: #{Enum.join(allowed, ", ")}")
    end
  end

"#,
            );
        }

        if !helpers.is_empty() {
            helpers.push_str(
                r#"  defp invalid_parameter!(name, message) do
    raise ArgumentError, "invalid parameter #{name}: #{message}"
  end

"#,
            );
        }

        helpers
    }

    fn handler_stub(&self, operation: &Operation, operation_id: &str) -> String {
        let mut code = String::new();
        let has_request_data = !operation.parameters.is_empty() || operation.request_body.is_some();
        let request_name = if has_request_data { "request" } else { "_request" };

        code.push_str(&format!(
            "  @spec {}(Spikard.Request.t()) :: Spikard.Response.t()\n",
            operation_id
        ));
        code.push_str(&format!("  def {}({}) do\n", operation_id, request_name));

        if has_request_data {
            for parameter_ref in &operation.parameters {
                let ReferenceOr::Item(parameter) = parameter_ref else {
                    continue;
                };
                if let Some(binding) = self.parameter_binding(parameter) {
                    code.push_str(&binding);
                }
            }

            if self.request_body_schema(operation).is_some() {
                code.push_str("    _body = Spikard.Request.get_body(request)\n");
            }
            code.push('\n');
        }

        if let Some((status, schema)) = self.response_schema(operation) {
            let payload = self.render_elixir_value(&self.schema_placeholder(schema), 3);
            code.push_str(&format!(
                "    Response.json(\n      {payload},\n      status: {status}\n    )\n"
            ));
        } else {
            let status = operation
                .responses
                .responses
                .keys()
                .find_map(|status| match status {
                    StatusCode::Code(code) if (200..300).contains(code) => Some(*code),
                    StatusCode::Range(2) => Some(200),
                    _ => None,
                })
                .unwrap_or(200);
            code.push_str(&format!("    Response.status({status})\n"));
        }

        code.push_str("  end\n\n");
        code
    }

    fn format_generated(&self, code: &str) -> String {
        let mut command = match Command::new("elixir")
            .arg("-e")
            .arg(
                r#"input = IO.read(:stdio, :all)
IO.write(IO.iodata_to_binary(Code.format_string!(input)))"#,
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(command) => command,
            Err(_) => return code.to_string(),
        };

        let Some(stdin) = command.stdin.as_mut() else {
            return code.to_string();
        };
        if stdin.write_all(code.as_bytes()).is_err() {
            return code.to_string();
        }

        match command.wait_with_output() {
            Ok(output) if output.status.success() => {
                let mut formatted = String::from_utf8(output.stdout).unwrap_or_else(|_| code.to_string());
                if !formatted.ends_with('\n') {
                    formatted.push('\n');
                }
                formatted
            }
            _ => {
                let mut fallback = code.to_string();
                if !fallback.ends_with('\n') {
                    fallback.push('\n');
                }
                fallback
            }
        }
    }
}

impl OpenApiGenerator for ElixirGenerator {
    fn spec(&self) -> &OpenAPI {
        &self.spec
    }

    fn registry(&self) -> &SchemaRegistry {
        &self.registry
    }

    fn generate(&self) -> Result<String> {
        let mut output = String::new();
        output.push_str(&self.generate_header());
        output.push_str(&self.generate_models()?);
        output.push_str(&self.generate_routes()?);

        Ok(self.format_generated(&output))
    }

    fn generate_header(&self) -> String {
        let module_name = self.root_module_name();
        let _ = self.style;
        format!(
            "defmodule {module_name}.Router do\n  @moduledoc \"\"\"\n  Generated by Spikard OpenAPI code generator.\n\n  This router wraps the operations defined in the OpenAPI specification and\n  attaches request/response schemas for runtime validation and OpenAPI export.\n  \"\"\"\n\n  use Spikard.Router\n\n  alias {module_name}.Handlers\n\n"
        )
    }

    fn generate_models(&self) -> Result<String> {
        let mut output = String::new();

        self.iter_schemas(|name, schema| {
            let type_name = self.schema_type_name(name);

            output.push_str(&format!("  @typedoc \"OpenAPI schema for {name}.\"\n"));
            output.push_str(&format!(
                "  @type {} :: {}\n",
                type_name,
                self.schema_to_typespec(schema, false)
            ));
            output.push('\n');
            Ok(())
        })?;

        Ok(output)
    }

    fn generate_routes(&self) -> Result<String> {
        let module_name = self.root_module_name();
        let mut router = String::new();
        let mut handlers = String::new();
        let mut helper_usage = ElixirParamHelperUsage::default();

        handlers.push_str(&format!(
            "defmodule {module_name}.Handlers do\n  @moduledoc false\n\n  alias Spikard.Response\n\n"
        ));

        self.iter_paths(|path, method, operation| {
            let operation_id = self.generate_operation_id(path, method, operation);
            let (prelude, options) = self.route_options(&operation_id, operation)?;
            if !prelude.is_empty() {
                router.push_str(&prelude);
            }

            let route = self.route_path(path);
            let handler_ref = format!("&Handlers.{}/1", operation_id);
            if !options.is_empty() {
                router.push_str(&format!(
                    "  {}(\"{}\", {}, {})",
                    method,
                    route,
                    handler_ref,
                    options.join(", ")
                ));
            } else {
                router.push_str(&format!("  {} \"{}\", {}", method, route, handler_ref));
            }
            router.push_str("\n\n");

            self.collect_param_helper_usage(operation, &mut helper_usage);
            handlers.push_str(&self.handler_stub(operation, &operation_id));
            Ok(())
        })?;

        while router.ends_with("\n\n") {
            router.pop();
        }
        router.push_str("end\n\n");
        while handlers.ends_with("\n\n") {
            handlers.pop();
        }
        handlers.push_str(&self.render_param_helpers(&helper_usage));
        handlers.push_str("end\n");

        Ok(format!("{router}{handlers}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{Info, Paths};

    #[test]
    fn generates_elixir_module_name_from_title() {
        let generator = ElixirGenerator::new(
            OpenAPI {
                openapi: "3.1.0".to_string(),
                info: Info {
                    title: "Example Service".to_string(),
                    version: "1.0.0".to_string(),
                    ..Default::default()
                },
                paths: Paths::default(),
                ..Default::default()
            },
            ElixirDtoStyle::Typespecs,
        );

        assert_eq!(generator.root_module_name(), "ExampleServiceApi");
    }

    #[test]
    fn converts_openapi_paths_to_spikard_paths() {
        let generator = ElixirGenerator::new(OpenAPI::default(), ElixirDtoStyle::Typespecs);
        assert_eq!(
            generator.route_path("/users/{id}/posts/{post_id}"),
            "/users/:id/posts/:post_id"
        );
    }
}
