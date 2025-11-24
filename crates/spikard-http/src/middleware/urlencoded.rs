//! URL-encoded form data parsing

use std::collections::HashMap;

/// Parse URL-encoded form data to JSON
///
/// This handles:
/// - Array notation: tags[]=value1&tags[]=value2 → {"tags": ["value1", "value2"]}
/// - Nested objects: profile[name]=John → {"profile": {"name": "John"}}
/// - Type conversion: age=30 → {"age": 30}, active=true → {"active": true}
/// - Multiple values: tags=a&tags=b → {"tags": ["a", "b"]}
/// - Empty strings: Preserved as empty strings (unlike query parameter parsing)
///
/// Strategy:
/// - If brackets present → use serde_qs (handles nested objects, arrays with [])
/// - Otherwise → use custom parser that preserves empty strings and handles duplicate keys
pub fn parse_urlencoded_to_json(data: &[u8]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let body_str = std::str::from_utf8(data)?;

    if body_str.contains('[') {
        let config = serde_qs::Config::new(10, false);
        let parsed: HashMap<String, serde_json::Value> = config.deserialize_str(body_str)?;
        let mut json_value = serde_json::to_value(parsed)?;
        convert_types_recursive(&mut json_value);
        Ok(json_value)
    } else {
        Ok(parse_urlencoded_simple(data))
    }
}

/// Parse simple URL-encoded data (no brackets) while preserving empty strings
fn parse_urlencoded_simple(data: &[u8]) -> serde_json::Value {
    use rustc_hash::FxHashMap;
    use urlencoding::decode;

    let mut array_map: FxHashMap<String, Vec<serde_json::Value>> = FxHashMap::default();

    let body_str = String::from_utf8_lossy(data);
    let body_str = body_str.replace('+', " ");

    for pair in body_str.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = if let Some((k, v)) = pair.split_once('=') {
            (
                decode(k).unwrap_or_default().to_string(),
                decode(v).unwrap_or_default().to_string(),
            )
        } else {
            (pair.to_string(), String::new())
        };

        let json_value = convert_string_to_json_value(&value);

        match array_map.get_mut(&key) {
            Some(entry) => {
                entry.push(json_value);
            }
            None => {
                array_map.insert(key, vec![json_value]);
            }
        }
    }

    array_map
        .iter()
        .map(|(key, value)| {
            if value.len() == 1 {
                (key, value[0].clone())
            } else {
                (key, serde_json::Value::Array(value.clone()))
            }
        })
        .collect::<serde_json::Value>()
}

/// Try to parse a string as an integer
fn try_parse_integer(s: &str) -> Option<serde_json::Value> {
    s.parse::<i64>().ok().map(|i| serde_json::Value::Number(i.into()))
}

/// Try to parse a string as a float
fn try_parse_float(s: &str) -> Option<serde_json::Value> {
    s.parse::<f64>()
        .ok()
        .and_then(|f| serde_json::Number::from_f64(f).map(serde_json::Value::Number))
}

/// Try to parse a string as a boolean (true/false, case-insensitive)
fn try_parse_boolean(s: &str) -> Option<serde_json::Value> {
    match s.to_lowercase().as_str() {
        "true" => Some(serde_json::Value::Bool(true)),
        "false" => Some(serde_json::Value::Bool(false)),
        _ => None,
    }
}

/// Convert a string value to appropriate JSON type while preserving empty strings
pub fn convert_string_to_json_value(s: &str) -> serde_json::Value {
    if s.is_empty() {
        return serde_json::Value::String(String::new());
    }

    try_parse_integer(s)
        .or_else(|| try_parse_float(s))
        .or_else(|| try_parse_boolean(s))
        .or_else(|| {
            if s == "null" {
                Some(serde_json::Value::Null)
            } else {
                None
            }
        })
        .unwrap_or_else(|| serde_json::Value::String(s.to_string()))
}

/// Recursively convert string values to appropriate types (numbers, booleans)
/// while preserving empty strings
fn convert_types_recursive(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            if s.is_empty() {
                return;
            }

            if let Some(parsed) = try_parse_integer(s)
                .or_else(|| try_parse_float(s))
                .or_else(|| try_parse_boolean(s))
            {
                *value = parsed;
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                convert_types_recursive(item);
            }
        }
        serde_json::Value::Object(obj) => {
            for (_, v) in obj.iter_mut() {
                convert_types_recursive(v);
            }
        }
        _ => {}
    }
}
