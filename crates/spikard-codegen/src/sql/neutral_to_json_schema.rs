//! Map scythe's neutral type strings to OpenAPI 3.1 JSON Schema fragments.
//!
//! Scythe's type system is documented at
//! `scythe-core/src/analyzer/type_conversion.rs`. The canonical neutral types
//! are: `int16`, `int32`, `int64`, `float32`, `float64`, `string`, `bool`,
//! `bytes`, `uuid`, `date`, `datetime`, `datetime_tz`, `time`, `time_tz`,
//! `interval`, `json`, `inet`, `decimal`, plus the composite forms
//! `array<T>`, `range<T>`, `enum::<name>`, `composite::<name>`, and
//! `json_typed<TypeName>` (produced by scythe's `@json` mapping).
//!
//! Nullability is layered by [`json_schema_for`] which wraps the schema with
//! `{"oneOf": [<schema>, {"type": "null"}]}` when `nullable` is true. This is
//! the OpenAPI 3.1 idiom (3.0's `"nullable": true` flag is not used).

use scythe_core::analyzer::EnumInfo;
use scythe_core::catalog::Catalog;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use thiserror::Error;

/// Build-time knobs for the neutral-type → JSON Schema mapping.
#[derive(Debug, Clone)]
pub struct BuildOptions {
    /// How to render the `decimal` neutral type. JSON Schema has no native
    /// exact-decimal, so users pick between lossless (`StringPattern`) and
    /// lossy-but-ergonomic (`Number`).
    pub decimal_mode: DecimalMode,
    /// When true, an unrecognised neutral type is an error. When false, it
    /// falls back to `{}` (any-JSON) so partial schemas still emit.
    pub strict: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            decimal_mode: DecimalMode::StringPattern,
            strict: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DecimalMode {
    /// Render as `{"type": "string", "pattern": "^-?\\d+(\\.\\d+)?$"}`.
    StringPattern,
    /// Render as `{"type": "number"}` (lossy — loses precision).
    Number,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NeutralTypeError {
    #[error("unknown neutral type '{0}'")]
    Unknown(String),
}

/// Translate a single neutral type string to a JSON Schema fragment. Does not
/// apply nullability — see [`json_schema_for`] for the wrapper that does.
pub fn neutral_to_json_schema(
    neutral: &str,
    enums: &[EnumInfo],
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Value, NeutralTypeError> {
    if let Some(inner) = strip_wrapper(neutral, "array<") {
        let item = neutral_to_json_schema(inner, enums, catalog, opts)?;
        return Ok(json!({ "type": "array", "items": item }));
    }
    if let Some(inner) = strip_wrapper(neutral, "range<") {
        let bound = neutral_to_json_schema(inner, enums, catalog, opts)?;
        let mut props = Map::new();
        props.insert("lower".to_string(), bound.clone());
        props.insert("upper".to_string(), bound);
        props.insert("lower_inclusive".to_string(), json!({ "type": "boolean" }));
        props.insert("upper_inclusive".to_string(), json!({ "type": "boolean" }));
        return Ok(json!({ "type": "object", "properties": Value::Object(props) }));
    }
    if let Some(enum_name) = neutral.strip_prefix("enum::") {
        let values: Vec<&str> = enums
            .iter()
            .find(|e| e.sql_name.eq_ignore_ascii_case(enum_name))
            .map(|e| e.values.iter().map(String::as_str).collect())
            .unwrap_or_default();
        return Ok(json!({ "type": "string", "enum": values }));
    }
    if let Some(composite_name) = neutral.strip_prefix("composite::") {
        let composite = catalog.get_composite(composite_name);
        let mut props = Map::new();
        if let Some(comp) = composite {
            for field in &comp.fields {
                let neutral_field = scythe_core_neutral_for(&field.sql_type, catalog);
                let field_schema = neutral_to_json_schema(&neutral_field, enums, catalog, opts)?;
                props.insert(field.name.clone(), field_schema);
            }
        }
        return Ok(json!({ "type": "object", "properties": Value::Object(props) }));
    }
    if neutral.starts_with("json_typed<") {
        // `@json col = TypeName` is currently emitted as opaque JSON; future
        // versions can resolve to `$ref` once a schema registry exists.
        return Ok(json!({}));
    }

    let schema = match neutral {
        "int16" => json!({ "type": "integer", "minimum": -32_768, "maximum": 32_767 }),
        "int32" => json!({ "type": "integer", "format": "int32" }),
        "int64" => json!({ "type": "integer", "format": "int64" }),
        "float32" => json!({ "type": "number", "format": "float" }),
        "float64" => json!({ "type": "number", "format": "double" }),
        "string" => json!({ "type": "string" }),
        "bool" => json!({ "type": "boolean" }),
        "bytes" => json!({ "type": "string", "format": "byte" }),
        "uuid" => json!({ "type": "string", "format": "uuid" }),
        "date" => json!({ "type": "string", "format": "date" }),
        "datetime" | "datetime_tz" => json!({ "type": "string", "format": "date-time" }),
        "time" | "time_tz" => json!({ "type": "string", "format": "time" }),
        "interval" => json!({ "type": "string", "format": "duration" }),
        "json" => json!({}),
        "inet" => json!({
            "type": "string",
            "oneOf": [{ "format": "ipv4" }, { "format": "ipv6" }]
        }),
        "decimal" => match opts.decimal_mode {
            DecimalMode::StringPattern => json!({
                "type": "string",
                "pattern": "^-?\\d+(\\.\\d+)?$"
            }),
            DecimalMode::Number => json!({ "type": "number" }),
        },
        other => {
            if opts.strict {
                return Err(NeutralTypeError::Unknown(other.to_string()));
            }
            json!({})
        }
    };
    Ok(schema)
}

/// Wrap [`neutral_to_json_schema`]'s output for nullability when the column or
/// parameter is nullable.
pub fn json_schema_for(
    neutral: &str,
    nullable: bool,
    enums: &[EnumInfo],
    catalog: &Catalog,
    opts: &BuildOptions,
) -> Result<Value, NeutralTypeError> {
    let base = neutral_to_json_schema(neutral, enums, catalog, opts)?;
    if nullable {
        Ok(json!({ "oneOf": [base, { "type": "null" }] }))
    } else {
        Ok(base)
    }
}

/// Re-derive a neutral type for a SQL type string by consulting the catalog.
/// Composite field types arrive as raw SQL strings, so we lean on scythe's own
/// resolver via its public API.
fn scythe_core_neutral_for(sql_type: &str, catalog: &Catalog) -> String {
    // The exact mapping function in scythe-core is `pub(super)` and not
    // exposed. We approximate by deferring to the catalog's enum/composite
    // lookups plus a small static table that covers the common cases. The
    // analyzer has already run, so any neutral type we miss here only affects
    // recursively-defined composite fields.
    let lower = sql_type.to_lowercase();
    let stripped = lower.split('(').next().unwrap_or(&lower).trim().to_string();
    match stripped.as_str() {
        "integer" | "int" | "int4" | "serial" => "int32".into(),
        "smallint" | "int2" | "smallserial" => "int16".into(),
        "bigint" | "int8" | "bigserial" => "int64".into(),
        "real" | "float4" => "float32".into(),
        "double precision" | "float8" | "double" | "float" => "float64".into(),
        "numeric" | "decimal" => "decimal".into(),
        "text" | "varchar" | "char" | "character" | "character varying" => "string".into(),
        "boolean" | "bool" => "bool".into(),
        "bytea" | "blob" | "binary" | "varbinary" => "bytes".into(),
        "uuid" => "uuid".into(),
        "date" => "date".into(),
        "timestamp" | "timestamp without time zone" => "datetime".into(),
        "timestamp with time zone" | "timestamptz" => "datetime_tz".into(),
        "time" => "time".into(),
        "interval" => "interval".into(),
        "json" | "jsonb" => "json".into(),
        "inet" | "cidr" => "inet".into(),
        other => {
            if catalog.get_enum(other).is_some() {
                format!("enum::{other}")
            } else if catalog.get_composite(other).is_some() {
                format!("composite::{other}")
            } else {
                other.to_string()
            }
        }
    }
}

fn strip_wrapper<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = s.strip_prefix(prefix)?;
    rest.strip_suffix('>')
}

#[cfg(test)]
mod tests {
    use super::*;
    use scythe_core::analyzer::EnumInfo;

    fn opts() -> BuildOptions {
        BuildOptions::default()
    }

    fn empty_catalog() -> Catalog {
        Catalog::from_ddl(&[]).unwrap()
    }

    fn s(neutral: &str) -> Value {
        neutral_to_json_schema(neutral, &[], &empty_catalog(), &opts()).unwrap()
    }

    #[test]
    fn int16_carries_bounds() {
        assert_eq!(
            s("int16"),
            json!({ "type": "integer", "minimum": -32_768, "maximum": 32_767 })
        );
    }

    #[test]
    fn int32_has_format() {
        assert_eq!(s("int32"), json!({ "type": "integer", "format": "int32" }));
    }

    #[test]
    fn int64_has_format() {
        assert_eq!(s("int64"), json!({ "type": "integer", "format": "int64" }));
    }

    #[test]
    fn float32_and_float64_have_formats() {
        assert_eq!(s("float32"), json!({ "type": "number", "format": "float" }));
        assert_eq!(s("float64"), json!({ "type": "number", "format": "double" }));
    }

    #[test]
    fn string_and_bool() {
        assert_eq!(s("string"), json!({ "type": "string" }));
        assert_eq!(s("bool"), json!({ "type": "boolean" }));
    }

    #[test]
    fn bytes_is_byte_format() {
        assert_eq!(s("bytes"), json!({ "type": "string", "format": "byte" }));
    }

    #[test]
    fn uuid_format() {
        assert_eq!(s("uuid"), json!({ "type": "string", "format": "uuid" }));
    }

    #[test]
    fn date_and_datetime_formats() {
        assert_eq!(s("date"), json!({ "type": "string", "format": "date" }));
        assert_eq!(s("datetime"), json!({ "type": "string", "format": "date-time" }));
        assert_eq!(s("datetime_tz"), json!({ "type": "string", "format": "date-time" }));
    }

    #[test]
    fn time_and_time_tz_formats() {
        assert_eq!(s("time"), json!({ "type": "string", "format": "time" }));
        assert_eq!(s("time_tz"), json!({ "type": "string", "format": "time" }));
    }

    #[test]
    fn interval_format() {
        assert_eq!(s("interval"), json!({ "type": "string", "format": "duration" }));
    }

    #[test]
    fn json_is_any() {
        assert_eq!(s("json"), json!({}));
    }

    #[test]
    fn inet_one_of_v4_v6() {
        assert_eq!(
            s("inet"),
            json!({
                "type": "string",
                "oneOf": [{ "format": "ipv4" }, { "format": "ipv6" }]
            })
        );
    }

    #[test]
    fn decimal_string_pattern_by_default() {
        assert_eq!(
            s("decimal"),
            json!({ "type": "string", "pattern": "^-?\\d+(\\.\\d+)?$" })
        );
    }

    #[test]
    fn decimal_number_mode() {
        let o = BuildOptions {
            decimal_mode: DecimalMode::Number,
            ..BuildOptions::default()
        };
        assert_eq!(
            neutral_to_json_schema("decimal", &[], &empty_catalog(), &o).unwrap(),
            json!({ "type": "number" })
        );
    }

    #[test]
    fn array_of_strings_recurses() {
        assert_eq!(
            s("array<string>"),
            json!({ "type": "array", "items": { "type": "string" } })
        );
    }

    #[test]
    fn array_of_int32_recurses() {
        assert_eq!(
            s("array<int32>"),
            json!({ "type": "array", "items": { "type": "integer", "format": "int32" } })
        );
    }

    #[test]
    fn nested_array_recurses() {
        assert_eq!(
            s("array<array<string>>"),
            json!({
                "type": "array",
                "items": { "type": "array", "items": { "type": "string" } }
            })
        );
    }

    #[test]
    fn range_emits_object_with_bounds() {
        let v = s("range<int32>");
        assert_eq!(v["type"], "object");
        assert!(v["properties"]["lower"].is_object());
        assert!(v["properties"]["upper"].is_object());
        assert_eq!(v["properties"]["lower_inclusive"], json!({ "type": "boolean" }));
    }

    #[test]
    fn enum_resolves_values_from_enum_info() {
        let enums = vec![EnumInfo {
            sql_name: "mood".to_string(),
            values: vec!["sad".into(), "ok".into(), "happy".into()],
        }];
        let v = neutral_to_json_schema("enum::mood", &enums, &empty_catalog(), &opts()).unwrap();
        assert_eq!(v["type"], "string");
        assert_eq!(v["enum"], json!(["sad", "ok", "happy"]));
    }

    #[test]
    fn unknown_enum_emits_empty_enum_list() {
        let v = s("enum::missing");
        assert_eq!(v, json!({ "type": "string", "enum": [] }));
    }

    #[test]
    fn composite_emits_object_from_catalog() {
        let catalog = Catalog::from_ddl(&["CREATE TYPE addr AS (street TEXT, zip INTEGER);"]).unwrap();
        let v = neutral_to_json_schema("composite::addr", &[], &catalog, &opts()).unwrap();
        assert_eq!(v["type"], "object");
        assert_eq!(v["properties"]["street"]["type"], "string");
        assert_eq!(v["properties"]["zip"]["type"], "integer");
    }

    #[test]
    fn json_typed_emits_any() {
        assert_eq!(s("json_typed<MyType>"), json!({}));
    }

    #[test]
    fn unknown_type_falls_back_to_any_in_lenient_mode() {
        assert_eq!(s("mysterious"), json!({}));
    }

    #[test]
    fn unknown_type_errors_in_strict_mode() {
        let o = BuildOptions {
            strict: true,
            ..BuildOptions::default()
        };
        let err = neutral_to_json_schema("mysterious", &[], &empty_catalog(), &o).unwrap_err();
        assert!(matches!(err, NeutralTypeError::Unknown(_)));
    }

    #[test]
    fn nullable_wraps_in_oneof_null() {
        let v = json_schema_for("string", true, &[], &empty_catalog(), &opts()).unwrap();
        assert_eq!(
            v,
            json!({
                "oneOf": [{ "type": "string" }, { "type": "null" }]
            })
        );
    }

    #[test]
    fn nonnullable_returns_bare_schema() {
        let v = json_schema_for("string", false, &[], &empty_catalog(), &opts()).unwrap();
        assert_eq!(v, json!({ "type": "string" }));
    }
}
