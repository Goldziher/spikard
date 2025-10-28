//! Fast query string parser
//!
//! Vendored and adapted from https://github.com/litestar-org/fast-query-parsers
//! Original author: Naaman Hirschfeld (same author as Spikard)
//!
//! This parser handles multiple values for the same key and auto-converts types.

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use serde_json::{Value, from_str};
use std::convert::Infallible;
use urlencoding::decode;

lazy_static! {
    static ref PARENTHESES_RE: Regex = Regex::new(r"(^\[.*\]$|^\{.*\}$)").unwrap();
}

/// Parse a query string into a vector of (key, value) tuples.
///
/// Handles URL encoding and supports multiple values for the same key.
///
/// # Arguments
/// * `qs` - The query string bytes
/// * `separator` - The separator character (typically '&')
///
/// # Example
/// ```ignore
/// let result = parse_query_string(b"foo=1&foo=2&bar=test", '&');
/// // vec![("foo", "1"), ("foo", "2"), ("bar", "test")]
/// ```
#[inline]
pub fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    String::from_utf8(qs.to_vec())
        .unwrap_or_default()
        .replace('+', " ")
        .split(separator)
        .filter_map(|value| {
            if !value.is_empty() {
                return match decode(value).unwrap_or_default().split_once('=') {
                    Some((key, value)) => Some((key.to_owned(), value.to_owned())),
                    None => Some((value.to_owned(), String::from(""))),
                };
            }
            None
        })
        .collect::<Vec<(String, String)>>()
}

/// Decode a string value into a JSON Value with type conversion.
///
/// Handles:
/// - JSON objects and arrays (if wrapped in brackets)
/// - Booleans (true/false)
/// - Null
/// - Numbers (if parse_numbers is true)
/// - Strings (fallback)
#[inline]
fn decode_value(json_str: String, parse_numbers: bool) -> Value {
    // Try to parse as JSON if it looks like an object or array
    if PARENTHESES_RE.is_match(json_str.as_str()) {
        let result: Value = match from_str(json_str.as_str()) {
            Ok(value) => value,
            Err(_) => match from_str(json_str.replace('\'', "\"").as_str()) {
                Ok(normalized) => normalized,
                Err(_) => Value::Null,
            },
        };
        return result;
    }

    let normalized = json_str.replace('"', "");
    let json_boolean = normalized.parse::<bool>();
    let json_null = Ok::<_, Infallible>(normalized == "null");

    if parse_numbers {
        let json_integer = normalized.parse::<i64>();
        let json_float = normalized.parse::<f64>();
        return match (json_integer, json_float, json_boolean, json_null) {
            (Ok(json_integer), _, _, _) => Value::from(json_integer),
            (_, Ok(json_float), _, _) => Value::from(json_float),
            (_, _, Ok(json_boolean), _) => Value::from(json_boolean),
            (_, _, _, Ok(true)) => Value::Null,
            _ => Value::from(normalized),
        };
    }

    match (json_boolean, json_null) {
        (Ok(json_boolean), _) => Value::from(json_boolean),
        (_, Ok(true)) => Value::Null,
        _ => Value::from(normalized),
    }
}

/// Parse a query string into a JSON Value.
///
/// This function:
/// - Handles multiple values for the same key (creates arrays)
/// - Auto-converts types (numbers, booleans, null, objects, arrays)
/// - Collapses single-item arrays into single values
///
/// # Arguments
/// * `qs` - The query string bytes
/// * `parse_numbers` - Whether to parse numeric strings into numbers
///
/// # Example
/// ```ignore
/// let result = parse_query_string_to_json(b"foo=1&foo=2&bar=test&active=true", true);
/// // {"foo": [1, 2], "bar": "test", "active": true}
/// ```
#[inline]
pub fn parse_query_string_to_json(qs: &[u8], parse_numbers: bool) -> Value {
    let mut array_map: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    for (key, value) in parse_query_string(qs, '&') {
        match array_map.get_mut(&key) {
            Some(entry) => {
                entry.push(decode_value(value, parse_numbers));
            }
            None => {
                array_map.insert(key, vec![decode_value(value, parse_numbers)]);
            }
        }
    }

    array_map
        .iter()
        .map(|(key, value)| {
            if value.len() == 1 {
                (key, value[0].to_owned())
            } else {
                (key, Value::Array(value.to_owned()))
            }
        })
        .collect::<Value>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_string};

    fn eq_str(value: Value, string: &str) {
        assert_eq!(&to_string(&value).unwrap_or_default(), string)
    }

    #[test]
    fn test_ampersand_separator() {
        assert_eq!(
            parse_query_string(b"key=1&key=2&anotherKey=a&yetAnother=z", '&'),
            vec![
                (String::from("key"), String::from("1")),
                (String::from("key"), String::from("2")),
                (String::from("anotherKey"), String::from("a")),
                (String::from("yetAnother"), String::from("z")),
            ]
        );
    }

    #[test]
    fn test_handles_url_encoded_ampersand() {
        assert_eq!(
            parse_query_string(b"first=%26%40A.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("&@A.ac")),
                (String::from("second"), String::from("aaa")),
            ]
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_simple_string() {
        eq_str(parse_query_string_to_json(b"0=foo", true), r#"{"0":"foo"}"#);
    }

    #[test]
    fn parse_query_string_to_json_parses_numbers() {
        assert_eq!(parse_query_string_to_json(b"a=1", true), json!({"a": 1}));
        assert_eq!(parse_query_string_to_json(b"a=1.1", true), json!({"a": 1.1}));
    }

    #[test]
    fn parse_query_string_to_json_parses_booleans() {
        assert_eq!(parse_query_string_to_json(b"a=true", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=false", false), json!({"a": false}));
    }

    #[test]
    fn parse_query_string_to_json_parses_multiple_values() {
        assert_eq!(
            parse_query_string_to_json(b"a=1&a=2&a=3", true),
            json!({ "a": [1,2,3] })
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_null() {
        assert_eq!(parse_query_string_to_json(b"a=null", true), json!({ "a": null }));
    }

    #[test]
    fn parse_query_string_to_json_parses_empty_string() {
        assert_eq!(parse_query_string_to_json(b"a=", true), json!({ "a": "" }));
    }
}
