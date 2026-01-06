use anyhow::Result;
use heck::ToPascalCase;
use serde_json::Value;

/// TypeScript + Zod fragments for a message DTO.
#[derive(Debug, Clone)]
pub struct TypeScriptDto {
    pub schema_ident: String,
    pub type_ident: String,
    pub schema_declaration: String,
    pub type_declaration: String,
}

/// Generate TypeScript + Zod declarations for a JSON Schema payload.
pub fn generate_typescript_dto(message_name: &str, schema: &Value) -> Result<TypeScriptDto> {
    let type_ident = format!("{}Message", camel_case(message_name));
    let schema_ident = format!("{type_ident}Schema");

    let zod_expr = schema_to_zod(schema, false);
    let ts_type = schema_to_typescript(schema, false);

    let schema_declaration = format!("const {schema_ident} = {zod_expr};\n");
    let type_declaration = format!("type {type_ident} = {ts_type};\n");

    Ok(TypeScriptDto {
        schema_ident,
        type_ident,
        schema_declaration,
        type_declaration,
    })
}

/// Convert a JSON Schema to a TypeScript type expression.
fn schema_to_typescript(schema: &Value, optional: bool) -> String {
    let mut base = match detect_type(schema) {
        Some("string") => "string".to_string(),
        Some("number" | "integer") => "number".to_string(),
        Some("boolean") => "boolean".to_string(),
        Some("array") => {
            if let Some(items) = schema.get("items") {
                format!("{}[]", schema_to_typescript(items, false))
            } else {
                "unknown[]".to_string()
            }
        }
        Some("object") => object_to_typescript(schema),
        _ => {
            if let Some(Value::Array(variants)) = schema.get("enum")
                && !variants.is_empty()
            {
                return variants.iter().map(literal_type).collect::<Vec<_>>().join(" | ");
            }
            if let Some(constant) = schema.get("const") {
                return literal_type(constant);
            }
            "Record<string, unknown>".to_string()
        }
    };

    if optional {
        base.push_str(" | undefined");
    }

    base
}

fn object_to_typescript(schema: &Value) -> String {
    if let Some(additional) = schema.get("additionalProperties") {
        if additional == &Value::Bool(true) {
            return "Record<string, unknown>".to_string();
        }
        if let Value::Object(_) = additional {
            return format!("Record<string, {}>", schema_to_typescript(additional, false));
        }
    }

    let mut fields = Vec::new();
    let required = required_set(schema);

    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        for (name, subschema) in props {
            let optional = !required.contains(name);
            let ts_type = schema_to_typescript(subschema, optional);
            fields.push(format!("  {}: {};", format_property(name), ts_type));
        }
    }

    if fields.is_empty() {
        "Record<string, unknown>".to_string()
    } else {
        format!("{{\n{}\n}}", fields.join("\n"))
    }
}

/// Convert a JSON Schema to a Zod expression.
fn schema_to_zod(schema: &Value, optional: bool) -> String {
    let mut base = match detect_type(schema) {
        Some("string") => {
            if let Some(enum_values) = schema.get("enum") {
                enum_literal(enum_values)
            } else if let Some(constant) = schema.get("const") {
                format!("z.literal({})", literal_value(constant))
            } else {
                "z.string()".to_string()
            }
        }
        Some("number" | "integer") => "z.number()".to_string(),
        Some("boolean") => "z.boolean()".to_string(),
        Some("array") => {
            if let Some(items) = schema.get("items") {
                format!("z.array({})", schema_to_zod(items, false))
            } else {
                "z.array(z.unknown())".to_string()
            }
        }
        Some("object") => object_to_zod(schema),
        _ => {
            if let Some(enum_values) = schema.get("enum")
                && enum_values.is_array()
            {
                return enum_literal(enum_values);
            }
            if let Some(constant) = schema.get("const") {
                return format!("z.literal({})", literal_value(constant));
            }
            "z.record(z.string(), z.unknown())".to_string()
        }
    };

    if schema.get("nullable").and_then(serde_json::Value::as_bool).unwrap_or(false) {
        base.push_str(".nullable()");
    }

    if optional {
        base.push_str(".optional()");
    }

    base
}

fn object_to_zod(schema: &Value) -> String {
    if matches!(schema.get("additionalProperties"), Some(Value::Bool(true))) {
        return "z.record(z.string(), z.unknown())".to_string();
    }

    let mut fields = Vec::new();
    let required = required_set(schema);

    if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
        for (name, subschema) in props {
            let optional = !required.contains(name);
            let expr = schema_to_zod(subschema, optional);
            fields.push(format!("  {}: {},", format_property(name), expr));
        }
    }

    if fields.is_empty() {
        "z.record(z.string(), z.unknown())".to_string()
    } else {
        format!("z.object({{\n{}\n}})", fields.join("\n"))
    }
}

fn detect_type(schema: &Value) -> Option<&str> {
    match schema.get("type") {
        Some(Value::String(single)) => Some(single.as_str()),
        Some(Value::Array(types)) => types.iter().filter_map(|value| value.as_str()).find(|ty| *ty != "null"),
        _ => {
            if schema.get("properties").is_some() {
                Some("object")
            } else if schema.get("items").is_some() {
                Some("array")
            } else {
                None
            }
        }
    }
}

fn required_set(schema: &Value) -> std::collections::HashSet<String> {
    schema
        .get("required")
        .and_then(|v| v.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(std::string::ToString::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn enum_literal(values: &Value) -> String {
    let mut literals = Vec::new();
    if let Some(arr) = values.as_array() {
        for value in arr {
            literals.push(format!("z.literal({})", literal_value(value)));
        }
    }
    if literals.is_empty() {
        "z.unknown()".to_string()
    } else if literals.len() == 1 {
        literals.remove(0)
    } else {
        format!("z.union([{}])", literals.join(", "))
    }
}

fn literal_value(value: &Value) -> String {
    match value {
        Value::String(s) => format!("{s:?}"),
        Value::Number(num) => num.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        other => serde_json::to_string(other).unwrap_or_else(|_| "null".to_string()),
    }
}

fn literal_type(value: &Value) -> String {
    match value {
        Value::String(s) => format!("{s:?}"),
        Value::Number(num) => num.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Convert a JSON value to a TypeScript literal expression.
pub fn json_value_to_ts_literal(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                "{}".to_string()
            } else {
                let mut parts = Vec::new();
                for (key, val) in map {
                    parts.push(format!("{}: {}", format_property(key), json_value_to_ts_literal(val)));
                }
                format!("{{ {} }}", parts.join(", "))
            }
        }
        Value::Array(items) => {
            if items.is_empty() {
                "[]".to_string()
            } else {
                let inner = items
                    .iter()
                    .map(json_value_to_ts_literal)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{inner}]")
            }
        }
        Value::String(s) => format!("{s:?}"),
        Value::Number(num) => num.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}

fn camel_case(name: &str) -> String {
    let converted = name
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .to_pascal_case();
    if converted.is_empty() {
        "Message".to_string()
    } else {
        converted
    }
}

fn format_property(name: &str) -> String {
    let valid_ident = name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_');
    if valid_ident {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}
