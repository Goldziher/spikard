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
use std::borrow::Cow;
use std::convert::Infallible;

lazy_static! {
    static ref PARENTHESES_RE: Regex = Regex::new(r"(^\[.*\]$|^\{.*\}$)").unwrap();
}

/// URL-decode a byte slice, replacing '+' with space and handling percent-encoding.
///
/// Optimized to avoid intermediate allocations by:
/// - Processing bytes directly without intermediate String conversion
/// - Using Cow to avoid allocation when no encoding is present
/// - Replacing '+' during decoding rather than as a separate pass
#[inline]
fn url_decode_optimized(input: &[u8]) -> Cow<'_, str> {
    let has_encoded = input.iter().any(|&b| b == b'+' || b == b'%');

    if !has_encoded {
        return match std::str::from_utf8(input) {
            Ok(s) => Cow::Borrowed(s),
            Err(_) => Cow::Owned(String::from_utf8_lossy(input).into_owned()),
        };
    }

    let mut result = Vec::with_capacity(input.len());
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'+' => {
                result.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < input.len() => {
                if let (Some(hi), Some(lo)) = (
                    char::from(input[i + 1]).to_digit(16),
                    char::from(input[i + 2]).to_digit(16),
                ) {
                    result.push((hi * 16 + lo) as u8);
                    i += 3;
                } else {
                    result.push(input[i]);
                    i += 1;
                }
            }
            b => {
                result.push(b);
                i += 1;
            }
        }
    }

    Cow::Owned(String::from_utf8_lossy(&result).into_owned())
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
///
/// # Performance
/// Optimized to minimize allocations by:
/// - Processing bytes directly without intermediate String allocation
/// - Using custom URL decoder that handles '+' replacement in one pass
/// - Pre-allocating result vector
#[inline]
pub fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    if qs.is_empty() {
        return Vec::new();
    }

    let separator_byte = separator as u8;
    let mut result = Vec::with_capacity(8);

    let mut start = 0;
    let mut i = 0;

    while i <= qs.len() {
        if i == qs.len() || qs[i] == separator_byte {
            if i > start {
                let pair = &qs[start..i];

                if let Some(eq_pos) = pair.iter().position(|&b| b == b'=') {
                    let key = url_decode_optimized(&pair[..eq_pos]);
                    let value = url_decode_optimized(&pair[eq_pos + 1..]);
                    result.push((key.into_owned(), value.into_owned()));
                } else {
                    let key = url_decode_optimized(pair);
                    result.push((key.into_owned(), String::new()));
                }
            }

            start = i + 1;
        }

        i += 1;
    }

    result
}

/// Decode a string value into a JSON Value with type conversion.
///
/// Handles:
/// - JSON objects and arrays (if wrapped in brackets)
/// - Booleans (true/false/1/0, case-insensitive)
/// - Null
/// - Numbers (if parse_numbers is true)
/// - Strings (fallback)
#[inline]
fn decode_value(json_str: String, parse_numbers: bool) -> Value {
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

    let json_boolean = parse_boolean(&normalized);
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

/// Parse a boolean value from a string.
///
/// Accepts:
/// - "true" (case-insensitive) → true
/// - "false" (case-insensitive) → false
/// - "1" → true
/// - "0" → false
/// - "" (empty string) → Err (don't coerce, preserve as empty string)
#[inline]
fn parse_boolean(s: &str) -> Result<bool, ()> {
    let lower = s.to_lowercase();
    if lower == "true" || s == "1" {
        Ok(true)
    } else if lower == "false" || s == "0" {
        Ok(false)
    } else {
        Err(())
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
    fn parse_query_string_to_json_parses_booleans_from_numbers() {
        assert_eq!(parse_query_string_to_json(b"a=1", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=0", false), json!({"a": false}));
    }

    #[test]
    fn parse_query_string_to_json_parses_case_insensitive_booleans() {
        assert_eq!(parse_query_string_to_json(b"a=True", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=TRUE", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=False", false), json!({"a": false}));
        assert_eq!(parse_query_string_to_json(b"a=FALSE", false), json!({"a": false}));
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

    #[test]
    fn parse_query_string_to_json_parses_empty_string_without_number_parsing() {
        assert_eq!(parse_query_string_to_json(b"a=", false), json!({ "a": "" }));
    }

    #[test]
    fn parse_query_string_to_json_parses_multiple_string_values() {
        assert_eq!(
            parse_query_string_to_json(b"q=foo&q=bar", true),
            json!({ "q": ["foo", "bar"] })
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_multiple_string_values_with_parse_numbers_false() {
        assert_eq!(
            parse_query_string_to_json(b"q=foo&q=bar", false),
            json!({ "q": ["foo", "bar"] })
        );
    }

    #[test]
    fn parse_query_string_to_json_preserves_order_and_duplicates() {
        assert_eq!(
            parse_query_string_to_json(b"q=foo&q=bar&q=baz", true),
            json!({ "q": ["foo", "bar", "baz"] })
        );

        assert_eq!(
            parse_query_string_to_json(b"q=foo&q=foo&q=bar", true),
            json!({ "q": ["foo", "foo", "bar"] })
        );
    }

    #[test]
    fn test_url_encoded_special_chars_in_values() {
        let result = parse_query_string_to_json(b"email=x%40test.com&special=%26%40A.ac", false);
        assert_eq!(
            result,
            json!({
                "email": "x@test.com",
                "special": "&@A.ac"
            })
        );
    }

    #[test]
    fn test_url_encoded_space() {
        let result = parse_query_string_to_json(b"name=hello%20world", false);
        assert_eq!(result, json!({ "name": "hello world" }));
    }

    #[test]
    fn test_url_encoded_complex_chars() {
        let result = parse_query_string_to_json(b"name=test%26value%3D123", false);
        assert_eq!(result, json!({ "name": "test&value=123" }));
    }
}
