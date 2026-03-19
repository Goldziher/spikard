//! Elixir `AsyncAPI` code generation.

use anyhow::{Result, bail};
use heck::{ToPascalCase, ToSnakeCase};
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

use super::base::sanitize_identifier;
use super::{AsyncApiGenerator, ChannelInfo, ChannelMessage};

/// Elixir `AsyncAPI` code generator.
pub struct ElixirAsyncApiGenerator;

impl AsyncApiGenerator for ElixirAsyncApiGenerator {
    fn generate_test_app(&self, _channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        bail!("Unsupported protocol for Elixir test app: {protocol}");
    }

    fn generate_handler_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String> {
        if channels.is_empty() {
            bail!("AsyncAPI spec does not define any channels");
        }

        match protocol {
            "websocket" | "sse" => {}
            other => bail!("Protocol {other} is not supported for Elixir handler generation"),
        }

        let mut code = String::new();
        code.push_str("defmodule AsyncApiTypes.Assertions do\n");
        code.push_str("  @moduledoc false\n\n");
        code.push_str("  @spec expect_field(map(), String.t(), (term() -> {:ok, term()} | {:error, String.t()})) ::\n");
        code.push_str("          {:ok, term()} | {:error, String.t()}\n");
        code.push_str("  def expect_field(payload, key, coercer) when is_map(payload) do\n");
        code.push_str("    case Map.fetch(payload, key) do\n");
        code.push_str("      {:ok, value} -> coercer.(value)\n");
        code.push_str("      :error -> {:error, \"Missing required field #{key}\"}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str(
            "  @spec optional_field(map(), String.t(), (term() -> {:ok, term()} | {:error, String.t()})) ::\n",
        );
        code.push_str("          {:ok, term() | nil} | {:error, String.t()}\n");
        code.push_str("  def optional_field(payload, key, coercer) when is_map(payload) do\n");
        code.push_str("    case Map.fetch(payload, key) do\n");
        code.push_str("      {:ok, nil} -> {:ok, nil}\n");
        code.push_str("      {:ok, value} -> coercer.(value)\n");
        code.push_str("      :error -> {:ok, nil}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str("  @spec coerce_string(term()) :: {:ok, String.t()} | {:error, String.t()}\n");
        code.push_str("  def coerce_string(value) when is_binary(value), do: {:ok, value}\n");
        code.push_str("  def coerce_string(_value), do: {:error, \"Expected string value\"}\n\n");
        code.push_str("  @spec coerce_integer(term()) :: {:ok, integer()} | {:error, String.t()}\n");
        code.push_str("  def coerce_integer(value) when is_integer(value), do: {:ok, value}\n");
        code.push_str("  def coerce_integer(_value), do: {:error, \"Expected integer value\"}\n\n");
        code.push_str("  @spec coerce_float(term()) :: {:ok, float()} | {:error, String.t()}\n");
        code.push_str("  def coerce_float(value) when is_float(value), do: {:ok, value}\n");
        code.push_str("  def coerce_float(value) when is_integer(value), do: {:ok, value * 1.0}\n");
        code.push_str("  def coerce_float(_value), do: {:error, \"Expected number value\"}\n\n");
        code.push_str("  @spec coerce_boolean(term()) :: {:ok, boolean()} | {:error, String.t()}\n");
        code.push_str("  def coerce_boolean(value) when is_boolean(value), do: {:ok, value}\n");
        code.push_str("  def coerce_boolean(_value), do: {:error, \"Expected boolean value\"}\n\n");
        code.push_str("  @spec coerce_map(term()) :: {:ok, map()} | {:error, String.t()}\n");
        code.push_str("  def coerce_map(value) when is_map(value), do: {:ok, value}\n");
        code.push_str("  def coerce_map(_value), do: {:error, \"Expected object value\"}\n\n");
        code.push_str("  @spec coerce_date(term()) :: {:ok, Date.t()} | {:error, String.t()}\n");
        code.push_str("  def coerce_date(value) when is_binary(value) do\n");
        code.push_str("    case Date.from_iso8601(value) do\n");
        code.push_str("      {:ok, parsed} -> {:ok, parsed}\n");
        code.push_str("      {:error, _reason} -> {:error, \"Expected ISO 8601 date string\"}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str("  def coerce_date(_value), do: {:error, \"Expected ISO 8601 date string\"}\n\n");
        code.push_str("  @spec coerce_datetime(term()) :: {:ok, DateTime.t()} | {:error, String.t()}\n");
        code.push_str("  def coerce_datetime(value) when is_binary(value) do\n");
        code.push_str("    case DateTime.from_iso8601(value) do\n");
        code.push_str("      {:ok, parsed, _offset} -> {:ok, parsed}\n");
        code.push_str("      {:error, _reason} -> {:error, \"Expected ISO 8601 datetime string\"}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str("  def coerce_datetime(_value), do: {:error, \"Expected ISO 8601 datetime string\"}\n\n");
        code.push_str("  @spec coerce_string_enum(term(), [String.t()]) :: {:ok, String.t()} | {:error, String.t()}\n");
        code.push_str("  def coerce_string_enum(value, allowed) when is_binary(value) do\n");
        code.push_str("    if value in allowed do\n");
        code.push_str("      {:ok, value}\n");
        code.push_str("    else\n");
        code.push_str("      {:error, \"Unexpected enum value #{inspect(value)}\"}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str("  def coerce_string_enum(_value, _allowed), do: {:error, \"Expected string enum value\"}\n\n");
        code.push_str("  @spec coerce_object(term(), module()) :: {:ok, term()} | {:error, String.t()}\n");
        code.push_str("  def coerce_object(value, module) when is_map(value), do: module.from_map(value)\n");
        code.push_str("  def coerce_object(_value, _module), do: {:error, \"Expected object value\"}\n\n");
        code.push_str("  @spec coerce_list(term(), (term() -> {:ok, term()} | {:error, String.t()})) ::\n");
        code.push_str("          {:ok, [term()]} | {:error, String.t()}\n");
        code.push_str("  def coerce_list(values, coercer) when is_list(values) do\n");
        code.push_str("    values\n");
        code.push_str("    |> Enum.reduce_while({:ok, []}, fn value, {:ok, acc} ->\n");
        code.push_str("      case coercer.(value) do\n");
        code.push_str("        {:ok, coerced} -> {:cont, {:ok, [coerced | acc]}}\n");
        code.push_str("        {:error, reason} -> {:halt, {:error, reason}}\n");
        code.push_str("      end\n");
        code.push_str("    end)\n");
        code.push_str("    |> case do\n");
        code.push_str("      {:ok, coerced} -> {:ok, Enum.reverse(coerced)}\n");
        code.push_str("      {:error, reason} -> {:error, reason}\n");
        code.push_str("    end\n");
        code.push_str("  end\n\n");
        code.push_str("  def coerce_list(_value, _coercer), do: {:error, \"Expected list value\"}\n\n");
        code.push_str("  @spec put_if_present(map(), String.t(), term()) :: map()\n");
        code.push_str("  def put_if_present(map, _key, nil), do: map\n");
        code.push_str("  def put_if_present(map, key, value), do: Map.put(map, key, value)\n");
        code.push_str("end\n\n");

        for channel in channels {
            code.push_str(&generate_channel_message_models(channel));
        }

        for channel in channels {
            code.push_str(&generate_channel_handler(channel, protocol));
            code.push('\n');
        }

        code.push_str("defmodule AsyncApiHandlers do\n");
        code.push_str("  @moduledoc false\n\n");
        match protocol {
            "websocket" => {
                code.push_str("  @spec websocket_routes() :: [%{path: String.t(), handler: module()}]\n");
                code.push_str("  def websocket_routes do\n");
                code.push_str("    [\n");
                for (index, channel) in channels.iter().enumerate() {
                    let suffix = if index + 1 == channels.len() { "" } else { "," };
                    code.push_str(&format!(
                        "      %{{path: \"{}\", handler: {}}}{}\n",
                        channel.path,
                        handler_module_name(channel, protocol),
                        suffix
                    ));
                }
                code.push_str("    ]\n");
                code.push_str("  end\n");
            }
            "sse" => {
                code.push_str("  @spec sse_routes() :: [%{path: String.t(), producer: module()}]\n");
                code.push_str("  def sse_routes do\n");
                code.push_str("    [\n");
                for (index, channel) in channels.iter().enumerate() {
                    let suffix = if index + 1 == channels.len() { "" } else { "," };
                    code.push_str(&format!(
                        "      %{{path: \"{}\", producer: {}}}{}\n",
                        channel.path,
                        handler_module_name(channel, protocol),
                        suffix
                    ));
                }
                code.push_str("    ]\n");
                code.push_str("  end\n");
            }
            _ => {}
        }
        code.push_str("end\n");

        Ok(format_elixir(&code))
    }
}

fn generate_channel_message_models(channel: &ChannelInfo) -> String {
    let mut code = String::new();

    for message in &channel.message_definitions {
        if let Some(schema) = &message.schema {
            code.push_str(&generate_named_schema(
                &elixir_message_module_name(channel, message),
                schema,
            ));
            code.push('\n');
        }
    }

    code
}

fn generate_channel_handler(channel: &ChannelInfo, protocol: &str) -> String {
    match protocol {
        "websocket" => generate_websocket_handler(channel),
        "sse" => generate_sse_handler(channel),
        _ => String::new(),
    }
}

fn generate_websocket_handler(channel: &ChannelInfo) -> String {
    let handler_module = handler_module_name(channel, "websocket");
    let payload_modules = channel
        .message_definitions
        .iter()
        .filter(|message| message.schema.is_some())
        .map(|message| format!("AsyncApiTypes.{}", elixir_message_module_name(channel, message)))
        .collect::<Vec<_>>();
    let mut code = String::new();
    code.push_str(&format!("defmodule {handler_module} do\n"));
    code.push_str("  @moduledoc false\n");
    code.push_str("  use Spikard.WebSocket\n\n");
    if payload_modules.is_empty() {
        code.push_str("  @impl true\n");
        code.push_str("  def handle_message(message, state), do: {:reply, message, state}\n");
        code.push_str("end\n");
        return code;
    }

    code.push_str("  @payload_modules [\n");
    for (index, module_name) in payload_modules.iter().enumerate() {
        let suffix = if index + 1 == payload_modules.len() { "" } else { "," };
        code.push_str(&format!("    {module_name}{suffix}\n"));
    }
    code.push_str("  ]\n\n");
    code.push_str("  @impl true\n");
    code.push_str("  def handle_message(message, state) when is_map(message) do\n");
    code.push_str("    with {:ok, parsed} <- parse_message(message) do\n");
    code.push_str("      {:reply, payload_to_map(parsed), state}\n");
    code.push_str("    end\n");
    code.push_str("  end\n\n");
    code.push_str("  def handle_message(_message, _state), do: {:error, \"Expected map payload\"}\n\n");
    code.push_str("  defp parse_message(payload) do\n");
    code.push_str("    Enum.find_value(@payload_modules, {:error, \"Unsupported message payload\"}, fn module ->\n");
    code.push_str("      case module.from_map(payload) do\n");
    code.push_str("        {:ok, parsed} -> {:ok, parsed}\n");
    code.push_str("        {:error, _reason} -> false\n");
    code.push_str("      end\n");
    code.push_str("    end)\n");
    code.push_str("  end\n\n");
    code.push_str("  defp payload_to_map(%{__struct__: module} = payload), do: module.to_map(payload)\n");
    code.push_str("end\n");
    code
}

fn generate_sse_handler(channel: &ChannelInfo) -> String {
    let producer_module = handler_module_name(channel, "sse");
    let first_message_module = channel
        .message_definitions
        .iter()
        .find(|message| message.schema.is_some())
        .map(|message| format!("AsyncApiTypes.{}", elixir_message_module_name(channel, message)));
    let first_event_name = channel
        .message_definitions
        .iter()
        .find(|message| message.schema.is_some())
        .map(|message| message.name.clone())
        .unwrap_or_else(|| channel.name.clone());
    let mut code = String::new();
    code.push_str(&format!("defmodule {producer_module} do\n"));
    code.push_str("  @moduledoc false\n");
    code.push_str("  use Spikard.Sse.Producer\n\n");
    code.push_str("  alias Spikard.Sse.Event\n\n");
    code.push_str("  @impl true\n");
    code.push_str("  def init(_opts), do: {:ok, :ready}\n\n");
    code.push_str("  @impl true\n");
    code.push_str("  def next_event(:ready) do\n");
    match first_message_module {
        Some(module_name) => {
            code.push_str(&format!(
                "    payload = {module_name}.example() |> {module_name}.to_map()\n"
            ));
            code.push_str(&format!(
                "    {{:ok, %Event{{event: \"{}\", data: payload}}, :done}}\n",
                escape_string(&first_event_name)
            ));
        }
        None => {
            code.push_str(&format!(
                "    {{:ok, %Event{{event: \"{}\", data: %{{}}}}, :done}}\n",
                escape_string(&first_event_name)
            ));
        }
    }
    code.push_str("  end\n\n");
    code.push_str("  def next_event(:done), do: :done\n");
    code.push_str("end\n");
    code
}

fn generate_named_schema(module_name: &str, schema: &Value) -> String {
    let mut code = String::new();

    for (field_name, field_schema) in object_properties(schema) {
        if schema_has_named_object_shape(field_schema) {
            let nested_name = format!("{module_name}{}", field_name.to_pascal_case());
            code.push_str(&generate_named_schema(&nested_name, field_schema));
            code.push('\n');
        } else if let Some(items) = field_schema.get("items")
            && schema_has_named_object_shape(items)
        {
            let nested_name = format!("{module_name}{}Item", field_name.to_pascal_case());
            code.push_str(&generate_named_schema(&nested_name, items));
            code.push('\n');
        }
    }

    let required = required_field_names(schema);
    let mut fields = object_properties(schema);
    fields.sort_by(|(left, _), (right, _)| left.cmp(right));

    code.push_str(&format!("defmodule AsyncApiTypes.{module_name} do\n"));
    code.push_str("  @moduledoc false\n");
    code.push_str("  alias AsyncApiTypes.Assertions\n\n");

    let enforce_keys = fields
        .iter()
        .filter(|(field_name, _)| required.iter().any(|required_name| required_name == *field_name))
        .map(|(field_name, _)| format!(":{}", sanitize_elixir_field_name(field_name)))
        .collect::<Vec<_>>();
    if !enforce_keys.is_empty() {
        code.push_str(&format!("  @enforce_keys [{}]\n", enforce_keys.join(", ")));
    }

    if fields.is_empty() {
        code.push_str("  defstruct []\n");
        code.push_str("  @type t :: %__MODULE__{}\n\n");
        code.push_str("  @spec from_map(map()) :: {:ok, t()}\n");
        code.push_str("  def from_map(_payload), do: {:ok, %__MODULE__{}}\n\n");
        code.push_str("  @spec matches?(map()) :: boolean()\n");
        code.push_str("  def matches?(payload), do: match?({:ok, _}, from_map(payload))\n\n");
        code.push_str("  @spec to_map(t()) :: map()\n");
        code.push_str("  def to_map(_payload), do: %{}\n\n");
        code.push_str("  @spec example() :: t()\n");
        code.push_str("  def example, do: %__MODULE__{}\n");
        code.push_str("end\n");
        return code;
    }

    code.push_str("  defstruct ");
    code.push_str(
        &fields
            .iter()
            .map(|(field_name, _)| format!("{}: nil", sanitize_elixir_field_name(field_name)))
            .collect::<Vec<_>>()
            .join(", "),
    );
    code.push_str("\n");
    code.push_str("  @type t :: %__MODULE__{\n");
    for (index, (field_name, field_schema)) in fields.iter().enumerate() {
        let suffix = if index + 1 == fields.len() { "" } else { "," };
        let field_atom = sanitize_elixir_field_name(field_name);
        let is_required = required.iter().any(|required_name| required_name == *field_name);
        let field_type = schema_to_elixir_type(module_name, field_name, field_schema, is_required);
        code.push_str(&format!("          {field_atom}: {field_type}{suffix}\n"));
    }
    code.push_str("        }\n\n");

    code.push_str("  @spec from_map(map()) :: {:ok, t()} | {:error, String.t()}\n");
    code.push_str("  def from_map(payload) when is_map(payload) do\n");
    if fields.is_empty() {
        code.push_str("    {:ok, %__MODULE__{}}\n");
    } else {
        code.push_str("    with ");
        for (index, (field_name, field_schema)) in fields.iter().enumerate() {
            let field_atom = sanitize_elixir_field_name(field_name);
            let is_required = required.iter().any(|required_name| required_name == *field_name);
            let expr = schema_from_map_expr(module_name, field_name, field_schema, is_required);
            let prefix = if index == 0 { "" } else { "         " };
            let suffix = if index + 1 == fields.len() { " do\n" } else { ",\n" };
            code.push_str(&format!("{prefix}{{:ok, {field_atom}}} <- {expr}{suffix}"));
        }
    }
    code.push_str("      {:ok, %__MODULE__{\n");
    for (index, (field_name, _)) in fields.iter().enumerate() {
        let suffix = if index + 1 == fields.len() { "" } else { "," };
        let field_atom = sanitize_elixir_field_name(field_name);
        code.push_str(&format!("        {field_atom}: {field_atom}{suffix}\n"));
    }
    code.push_str("      }}\n");
    code.push_str("    end\n");
    code.push_str("  end\n\n");
    code.push_str("  def from_map(_payload), do: {:error, \"Expected map payload\"}\n\n");

    code.push_str("  @spec matches?(map()) :: boolean()\n");
    code.push_str("  def matches?(payload), do: match?({:ok, _}, from_map(payload))\n\n");

    code.push_str("  @spec to_map(t()) :: map()\n");
    code.push_str("  def to_map(payload) do\n");
    code.push_str("    %{}\n");
    for (field_name, field_schema) in &fields {
        let field_atom = sanitize_elixir_field_name(field_name);
        let value_expr = schema_to_wire_expr(module_name, field_name, field_schema, &format!("payload.{field_atom}"));
        code.push_str(&format!(
            "    |> Assertions.put_if_present(\"{}\", {})\n",
            escape_string(field_name),
            value_expr
        ));
    }
    code.push_str("  end\n\n");

    code.push_str("  @spec example() :: t()\n");
    code.push_str("  def example do\n");
    code.push_str("    %__MODULE__{\n");
    for (index, (field_name, field_schema)) in fields.iter().enumerate() {
        let suffix = if index + 1 == fields.len() { "" } else { "," };
        let field_atom = sanitize_elixir_field_name(field_name);
        code.push_str(&format!(
            "      {field_atom}: {}{suffix}\n",
            schema_example_expr(module_name, field_name, field_schema)
        ));
    }
    code.push_str("    }\n");
    code.push_str("  end\n");
    code.push_str("end\n");

    code
}

fn handler_module_name(channel: &ChannelInfo, protocol: &str) -> String {
    let base = sanitize_identifier(&channel.name).to_pascal_case();
    match protocol {
        "websocket" => format!("{base}WebSocketHandler"),
        "sse" => format!("{base}SseProducer"),
        _ => base,
    }
}

fn elixir_message_module_name(channel: &ChannelInfo, message: &ChannelMessage) -> String {
    format!(
        "{}Payload",
        format!("{}_{}", channel.name, message.schema_name).to_pascal_case()
    )
}

fn sanitize_elixir_field_name(name: &str) -> String {
    let candidate = name.to_snake_case();
    match candidate.as_str() {
        "end" | "when" | "fn" | "do" | "case" | "receive" | "after" | "rescue" | "catch" => {
            format!("{candidate}_field")
        }
        _ => candidate,
    }
}

fn schema_to_elixir_type(module_name: &str, field_name: &str, schema: &Value, required: bool) -> String {
    let base_type = if schema.get("const").is_some() || schema.get("enum").is_some() {
        "String.t()".to_string()
    } else {
        match schema.get("type").and_then(Value::as_str) {
            Some("string") => match schema.get("format").and_then(Value::as_str) {
                Some("date-time") => "DateTime.t()".to_string(),
                Some("date") => "Date.t()".to_string(),
                _ => "String.t()".to_string(),
            },
            Some("integer") => "integer()".to_string(),
            Some("number") => "float()".to_string(),
            Some("boolean") => "boolean()".to_string(),
            Some("array") => {
                let item_type = schema
                    .get("items")
                    .map(|items| schema_to_array_item_type(module_name, field_name, items))
                    .unwrap_or_else(|| "term()".to_string());
                format!("[{item_type}]")
            }
            Some("object") => {
                if schema_has_named_object_shape(schema) {
                    format!("AsyncApiTypes.{}.t()", nested_module_name(module_name, field_name))
                } else {
                    "map()".to_string()
                }
            }
            _ => "term()".to_string(),
        }
    };

    if required {
        base_type
    } else {
        format!("{base_type} | nil")
    }
}

fn schema_to_array_item_type(module_name: &str, field_name: &str, schema: &Value) -> String {
    if schema_has_named_object_shape(schema) {
        format!(
            "AsyncApiTypes.{}.t()",
            nested_array_module_name(module_name, field_name)
        )
    } else if schema.get("const").is_some() || schema.get("enum").is_some() {
        "String.t()".to_string()
    } else {
        match schema.get("type").and_then(Value::as_str) {
            Some("string") => match schema.get("format").and_then(Value::as_str) {
                Some("date-time") => "DateTime.t()".to_string(),
                Some("date") => "Date.t()".to_string(),
                _ => "String.t()".to_string(),
            },
            Some("integer") => "integer()".to_string(),
            Some("number") => "float()".to_string(),
            Some("boolean") => "boolean()".to_string(),
            Some("object") => "map()".to_string(),
            _ => "term()".to_string(),
        }
    }
}

fn schema_from_map_expr(module_name: &str, field_name: &str, schema: &Value, required: bool) -> String {
    let accessor = if required {
        "Assertions.expect_field"
    } else {
        "Assertions.optional_field"
    };
    let key = escape_string(field_name);

    if let Some(enum_values) = schema.get("enum").and_then(Value::as_array) {
        let allowed = enum_values
            .iter()
            .filter_map(Value::as_str)
            .map(|value| format!("\"{}\"", escape_string(value)))
            .collect::<Vec<_>>()
            .join(", ");
        return format!(
            "{accessor}(payload, \"{key}\", fn value -> Assertions.coerce_string_enum(value, [{allowed}]) end)"
        );
    }

    if let Some(const_value) = schema.get("const").and_then(Value::as_str) {
        return format!(
            "{accessor}(payload, \"{key}\", fn value -> Assertions.coerce_string_enum(value, [\"{}\"]) end)",
            escape_string(const_value)
        );
    }

    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date-time") => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_datetime/1)"),
            Some("date") => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_date/1)"),
            _ => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_string/1)"),
        },
        Some("integer") => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_integer/1)"),
        Some("number") => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_float/1)"),
        Some("boolean") => format!("{accessor}(payload, \"{key}\", &Assertions.coerce_boolean/1)"),
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                let nested_module = nested_module_name(module_name, field_name);
                format!(
                    "{accessor}(payload, \"{key}\", fn value -> Assertions.coerce_object(value, AsyncApiTypes.{nested_module}) end)"
                )
            } else {
                format!("{accessor}(payload, \"{key}\", &Assertions.coerce_map/1)")
            }
        }
        Some("array") => {
            let coercer = schema
                .get("items")
                .map(|items| array_item_coercer_expr(module_name, field_name, items))
                .unwrap_or_else(|| "&Function.identity/1".to_string());
            format!("{accessor}(payload, \"{key}\", fn value -> Assertions.coerce_list(value, {coercer}) end)")
        }
        _ => format!("{accessor}(payload, \"{key}\", fn value -> {{:ok, value}} end)"),
    }
}

fn array_item_coercer_expr(module_name: &str, field_name: &str, schema: &Value) -> String {
    if let Some(enum_values) = schema.get("enum").and_then(Value::as_array) {
        let allowed = enum_values
            .iter()
            .filter_map(Value::as_str)
            .map(|value| format!("\"{}\"", escape_string(value)))
            .collect::<Vec<_>>()
            .join(", ");
        return format!("fn value -> Assertions.coerce_string_enum(value, [{allowed}]) end");
    }

    if let Some(const_value) = schema.get("const").and_then(Value::as_str) {
        return format!(
            "fn value -> Assertions.coerce_string_enum(value, [\"{}\"]) end",
            escape_string(const_value)
        );
    }

    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date-time") => "&Assertions.coerce_datetime/1".to_string(),
            Some("date") => "&Assertions.coerce_date/1".to_string(),
            _ => "&Assertions.coerce_string/1".to_string(),
        },
        Some("integer") => "&Assertions.coerce_integer/1".to_string(),
        Some("number") => "&Assertions.coerce_float/1".to_string(),
        Some("boolean") => "&Assertions.coerce_boolean/1".to_string(),
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                let nested_module = nested_array_module_name(module_name, field_name);
                format!("fn value -> Assertions.coerce_object(value, AsyncApiTypes.{nested_module}) end")
            } else {
                "&Assertions.coerce_map/1".to_string()
            }
        }
        _ => "fn value -> {:ok, value} end".to_string(),
    }
}

fn schema_to_wire_expr(module_name: &str, field_name: &str, schema: &Value, value_expr: &str) -> String {
    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date-time") => format!("if(is_nil({value_expr}), do: nil, else: DateTime.to_iso8601({value_expr}))"),
            Some("date") => format!("if(is_nil({value_expr}), do: nil, else: Date.to_iso8601({value_expr}))"),
            _ => value_expr.to_string(),
        },
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                let nested_module = nested_module_name(module_name, field_name);
                format!("if(is_nil({value_expr}), do: nil, else: AsyncApiTypes.{nested_module}.to_map({value_expr}))")
            } else {
                value_expr.to_string()
            }
        }
        Some("array") => {
            if let Some(items) = schema.get("items")
                && schema_has_named_object_shape(items)
            {
                let nested_module = nested_array_module_name(module_name, field_name);
                return format!(
                    "if(is_nil({value_expr}), do: nil, else: Enum.map({value_expr}, &AsyncApiTypes.{nested_module}.to_map/1))"
                );
            }
            value_expr.to_string()
        }
        _ => value_expr.to_string(),
    }
}

fn schema_example_expr(module_name: &str, field_name: &str, schema: &Value) -> String {
    if let Some(const_value) = schema.get("const").and_then(Value::as_str) {
        return format!("\"{}\"", escape_string(const_value));
    }

    if let Some(enum_value) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .and_then(Value::as_str)
    {
        return format!("\"{}\"", escape_string(enum_value));
    }

    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date-time") => "~U[2025-01-01 00:00:00Z]".to_string(),
            Some("date") => "~D[2025-01-01]".to_string(),
            _ => format!("\"{}\"", escape_string(field_name)),
        },
        Some("integer") => "1".to_string(),
        Some("number") => "1.0".to_string(),
        Some("boolean") => "true".to_string(),
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                format!(
                    "AsyncApiTypes.{}.example()",
                    nested_module_name(module_name, field_name)
                )
            } else {
                "%{}".to_string()
            }
        }
        Some("array") => {
            if let Some(items) = schema.get("items") {
                if schema_has_named_object_shape(items) {
                    return format!(
                        "[AsyncApiTypes.{}.example()]",
                        nested_array_module_name(module_name, field_name)
                    );
                }
                return format!("[{}]", array_item_example_expr(module_name, field_name, items));
            }
            "[]".to_string()
        }
        _ => "nil".to_string(),
    }
}

fn array_item_example_expr(module_name: &str, field_name: &str, schema: &Value) -> String {
    if let Some(const_value) = schema.get("const").and_then(Value::as_str) {
        return format!("\"{}\"", escape_string(const_value));
    }
    if let Some(enum_value) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .and_then(Value::as_str)
    {
        return format!("\"{}\"", escape_string(enum_value));
    }
    match schema.get("type").and_then(Value::as_str) {
        Some("string") => match schema.get("format").and_then(Value::as_str) {
            Some("date-time") => "~U[2025-01-01 00:00:00Z]".to_string(),
            Some("date") => "~D[2025-01-01]".to_string(),
            _ => format!("\"{}\"", escape_string(field_name)),
        },
        Some("integer") => "1".to_string(),
        Some("number") => "1.0".to_string(),
        Some("boolean") => "true".to_string(),
        Some("object") => {
            if schema_has_named_object_shape(schema) {
                format!(
                    "AsyncApiTypes.{}.example()",
                    nested_array_module_name(module_name, field_name)
                )
            } else {
                "%{}".to_string()
            }
        }
        _ => "nil".to_string(),
    }
}

fn nested_module_name(module_name: &str, field_name: &str) -> String {
    format!("{module_name}{}", field_name.to_pascal_case())
}

fn nested_array_module_name(module_name: &str, field_name: &str) -> String {
    format!("{module_name}{}Item", field_name.to_pascal_case())
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
