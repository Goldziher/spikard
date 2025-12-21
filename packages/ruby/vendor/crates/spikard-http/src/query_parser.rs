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
fn decode_value(raw: &str, parse_numbers: bool) -> Value {
    if PARENTHESES_RE.is_match(raw) {
        let result: Value = match from_str(raw) {
            Ok(value) => value,
            Err(_) => match from_str(raw.replace('\'', "\"").as_str()) {
                Ok(normalized) => normalized,
                Err(_) => Value::Null,
            },
        };
        return result;
    }

    let normalized = if raw.as_bytes().contains(&b'"') {
        Cow::Owned(raw.replace('"', ""))
    } else {
        Cow::Borrowed(raw)
    };

    let json_boolean = parse_boolean(&normalized);
    let json_null = Ok::<_, Infallible>(normalized.as_ref() == "null");

    if parse_numbers {
        let json_integer = normalized.as_ref().parse::<i64>();
        let json_float = normalized.as_ref().parse::<f64>();
        return match (json_integer, json_float, json_boolean, json_null) {
            (Ok(json_integer), _, _, _) => Value::from(json_integer),
            (_, Ok(json_float), _, _) => Value::from(json_float),
            (_, _, Ok(json_boolean), _) => Value::from(json_boolean),
            (_, _, _, Ok(true)) => Value::Null,
            _ => Value::from(normalized.as_ref()),
        };
    }

    match (json_boolean, json_null) {
        (Ok(json_boolean), _) => Value::from(json_boolean),
        (_, Ok(true)) => Value::Null,
        _ => Value::from(normalized.as_ref()),
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
    if s.eq_ignore_ascii_case("true") || s == "1" {
        Ok(true)
    } else if s.eq_ignore_ascii_case("false") || s == "0" {
        Ok(false)
    } else {
        Err(())
    }
}

/// Convert already-decoded query pairs into a JSON Value.
///
/// This is useful when callers need both:
/// - the raw decoded pairs (for error messages / multi-value handling), and
/// - a JSON object with type coercion (for downstream consumers),
///
/// while avoiding a second URL-decoding pass.
#[inline]
pub fn parse_query_pairs_to_json(pairs: &[(String, String)], parse_numbers: bool) -> Value {
    let mut array_map: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    for (key, value) in pairs {
        match array_map.get_mut(key) {
            Some(entry) => {
                entry.push(decode_value(value, parse_numbers));
            }
            None => {
                array_map.insert(key.clone(), vec![decode_value(value, parse_numbers)]);
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
    let pairs = parse_query_string(qs, '&');
    parse_query_pairs_to_json(&pairs, parse_numbers)
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

    #[test]
    fn test_malformed_percent_single_char() {
        let result = parse_query_string(b"key=%", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("%"))]);
    }

    #[test]
    fn test_malformed_percent_single_hex_only() {
        let result = parse_query_string(b"key=%2", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("%2"))]);
    }

    #[test]
    fn test_malformed_percent_invalid_hex_chars() {
        let result = parse_query_string(b"key=%GG&other=value", '&');
        assert_eq!(
            result,
            vec![
                (String::from("key"), String::from("%GG")),
                (String::from("other"), String::from("value")),
            ]
        );
    }

    #[test]
    fn test_malformed_percent_mixed_invalid_hex() {
        let result = parse_query_string(b"key=%2G&other=value", '&');
        assert_eq!(
            result,
            vec![
                (String::from("key"), String::from("%2G")),
                (String::from("other"), String::from("value")),
            ]
        );
    }

    #[test]
    fn test_percent_encoding_lowercase_hex() {
        let result = parse_query_string(b"key=%2f&other=test", '&');
        assert_eq!(
            result,
            vec![
                (String::from("key"), String::from("/")),
                (String::from("other"), String::from("test")),
            ]
        );
    }

    #[test]
    fn test_percent_encoding_uppercase_hex() {
        let result = parse_query_string(b"key=%2F&other=test", '&');
        assert_eq!(
            result,
            vec![
                (String::from("key"), String::from("/")),
                (String::from("other"), String::from("test")),
            ]
        );
    }

    #[test]
    fn test_percent_encoding_mixed_case_hex() {
        let result = parse_query_string(b"key=%2f%3D%4A", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("/=J"))]);
    }

    #[test]
    fn test_plus_as_space_in_value() {
        let result = parse_query_string(b"message=hello+world", '&');
        assert_eq!(result, vec![(String::from("message"), String::from("hello world"))]);
    }

    #[test]
    fn test_plus_as_space_multiple_plus() {
        let result = parse_query_string(b"message=a+b+c+d", '&');
        assert_eq!(result, vec![(String::from("message"), String::from("a b c d"))]);
    }

    #[test]
    fn test_percent_encoded_space_vs_plus() {
        let result = parse_query_string(b"a=%20space&b=+plus", '&');
        assert_eq!(
            result,
            vec![
                (String::from("a"), String::from(" space")),
                (String::from("b"), String::from(" plus")),
            ]
        );
    }

    #[test]
    fn test_mixed_plus_and_percent_encoded_space() {
        let result = parse_query_string(b"text=hello+%20world", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("hello  world"))]);
    }

    #[test]
    fn test_ampersand_in_value_encoded() {
        let result = parse_query_string(b"text=foo%26bar", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("foo&bar"))]);
    }

    #[test]
    fn test_equals_in_value_encoded() {
        let result = parse_query_string(b"text=a%3Db", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("a=b"))]);
    }

    #[test]
    fn test_question_mark_in_value_encoded() {
        let result = parse_query_string(b"text=what%3F", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("what?"))]);
    }

    #[test]
    fn test_hash_in_value_encoded() {
        let result = parse_query_string(b"text=anchor%23top", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("anchor#top"))]);
    }

    #[test]
    fn test_multiple_encoded_special_chars() {
        let result = parse_query_string(b"text=%26%3D%3F%23", '&');
        assert_eq!(result, vec![(String::from("text"), String::from("&=?#"))]);
    }

    #[test]
    fn test_empty_query_string() {
        let result = parse_query_string(b"", '&');
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_multiple_consecutive_separators() {
        let result = parse_query_string(b"a=1&&&b=2", '&');
        assert_eq!(
            result,
            vec![
                (String::from("a"), String::from("1")),
                (String::from("b"), String::from("2")),
            ]
        );
    }

    #[test]
    fn test_key_without_value() {
        let result = parse_query_string(b"key=", '&');
        assert_eq!(result, vec![(String::from("key"), String::from(""))]);
    }

    #[test]
    fn test_key_without_equals() {
        let result = parse_query_string(b"key", '&');
        assert_eq!(result, vec![(String::from("key"), String::from(""))]);
    }

    #[test]
    fn test_value_without_key() {
        let result = parse_query_string(b"=value", '&');
        assert_eq!(result, vec![(String::from(""), String::from("value"))]);
    }

    #[test]
    fn test_multiple_equals_in_pair() {
        let result = parse_query_string(b"key=val=more", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("val=more"))]);
    }

    #[test]
    fn test_separator_at_start() {
        let result = parse_query_string(b"&key=value", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("value"))]);
    }

    #[test]
    fn test_separator_at_end() {
        let result = parse_query_string(b"key=value&", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("value"))]);
    }

    #[test]
    fn test_separator_at_both_ends() {
        let result = parse_query_string(b"&key=value&", '&');
        assert_eq!(result, vec![(String::from("key"), String::from("value"))]);
    }

    #[test]
    fn test_multiple_values_same_key() {
        let result = parse_query_string(b"tag=foo&tag=bar&tag=baz", '&');
        assert_eq!(
            result,
            vec![
                (String::from("tag"), String::from("foo")),
                (String::from("tag"), String::from("bar")),
                (String::from("tag"), String::from("baz")),
            ]
        );
    }

    #[test]
    fn test_multiple_values_mixed_keys() {
        let result = parse_query_string(b"tag=foo&id=1&tag=bar&id=2", '&');
        assert_eq!(
            result,
            vec![
                (String::from("tag"), String::from("foo")),
                (String::from("id"), String::from("1")),
                (String::from("tag"), String::from("bar")),
                (String::from("id"), String::from("2")),
            ]
        );
    }

    #[test]
    fn test_json_conversion_empty_key() {
        let result = parse_query_string_to_json(b"=value", false);
        assert_eq!(result, json!({ "": "value" }));
    }

    #[test]
    fn test_json_conversion_all_empty_values() {
        let result = parse_query_string_to_json(b"a=&b=&c=", false);
        assert_eq!(result, json!({ "a": "", "b": "", "c": "" }));
    }

    #[test]
    fn test_json_conversion_malformed_json_object() {
        let result = parse_query_string_to_json(b"data={invalid", false);
        assert_eq!(result, json!({ "data": "{invalid" }));
    }

    #[test]
    fn test_json_conversion_malformed_json_array() {
        let result = parse_query_string_to_json(b"items=[1,2,", false);
        assert_eq!(result, json!({ "items": "[1,2," }));
    }

    #[test]
    fn test_json_conversion_with_quotes_in_value() {
        let result = parse_query_string_to_json(b"text=\"hello\"", false);
        assert_eq!(result, json!({ "text": "hello" }));
    }

    #[test]
    fn test_json_conversion_single_quotes_in_object() {
        let result = parse_query_string_to_json(b"obj={'key':'value'}", false);
        let value = result.get("obj");
        assert!(value.is_some());
    }

    #[test]
    fn test_boolean_case_insensitive_variations() {
        assert_eq!(parse_query_string_to_json(b"a=tRuE", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=FaLsE", false), json!({"a": false}));
        assert_eq!(
            parse_query_string_to_json(b"a=tRuE&b=FaLsE", false),
            json!({"a": true, "b": false})
        );
    }

    #[test]
    fn test_boolean_with_numbers_no_parse() {
        assert_eq!(parse_query_string_to_json(b"a=1", false), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=0", false), json!({"a": false}));
    }

    #[test]
    fn test_number_parsing_negative() {
        assert_eq!(parse_query_string_to_json(b"a=-123", true), json!({"a": -123}));
        assert_eq!(parse_query_string_to_json(b"a=-1.5", true), json!({"a": -1.5}));
    }

    #[test]
    fn test_number_parsing_zero() {
        assert_eq!(parse_query_string_to_json(b"a=0", true), json!({"a": 0}));
        assert_eq!(parse_query_string_to_json(b"a=0.0", true), json!({"a": 0.0}));
    }

    #[test]
    fn test_number_parsing_scientific_notation() {
        assert_eq!(parse_query_string_to_json(b"a=1e10", true), json!({"a": 1e10}));
        assert_eq!(parse_query_string_to_json(b"a=1.5e-3", true), json!({"a": 1.5e-3}));
    }

    #[test]
    fn test_array_mixed_types_with_number_parsing() {
        assert_eq!(
            parse_query_string_to_json(b"vals=1&vals=2.5&vals=true&vals=test", true),
            json!({"vals": [1, 2.5, true, "test"]})
        );
    }

    #[test]
    fn test_array_mixed_types_without_number_parsing() {
        assert_eq!(
            parse_query_string_to_json(b"vals=1&vals=2.5&vals=true&vals=test", false),
            json!({"vals": [true, "2.5", true, "test"]})
        );
    }

    #[test]
    fn test_utf8_chinese_characters() {
        let result = parse_query_string("name=中文".as_bytes(), '&');
        assert_eq!(result, vec![(String::from("name"), String::from("中文"))]);
    }

    #[test]
    fn test_utf8_emoji() {
        let result = parse_query_string("emoji=🚀".as_bytes(), '&');
        assert_eq!(result, vec![(String::from("emoji"), String::from("🚀"))]);
    }

    #[test]
    fn test_utf8_mixed_with_encoding() {
        let result = parse_query_string("text=hello%20中文".as_bytes(), '&');
        assert_eq!(result, vec![(String::from("text"), String::from("hello 中文"))]);
    }

    #[test]
    fn test_custom_separator_semicolon() {
        let result = parse_query_string(b"a=1;b=2;c=3", ';');
        assert_eq!(
            result,
            vec![
                (String::from("a"), String::from("1")),
                (String::from("b"), String::from("2")),
                (String::from("c"), String::from("3")),
            ]
        );
    }

    #[test]
    fn test_custom_separator_comma() {
        let result = parse_query_string(b"a=1,b=2,c=3", ',');
        assert_eq!(
            result,
            vec![
                (String::from("a"), String::from("1")),
                (String::from("b"), String::from("2")),
                (String::from("c"), String::from("3")),
            ]
        );
    }

    #[test]
    fn test_percent_encoding_all_byte_values() {
        let result = parse_query_string(b"space=%20&at=%40&hash=%23&dollar=%24", '&');
        assert_eq!(
            result,
            vec![
                (String::from("space"), String::from(" ")),
                (String::from("at"), String::from("@")),
                (String::from("hash"), String::from("#")),
                (String::from("dollar"), String::from("$")),
            ]
        );
    }

    #[test]
    fn test_high_byte_values_in_percent_encoding() {
        let result = parse_query_string(b"high=%ff%fe%fd", '&');
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "high");
    }

    #[test]
    fn test_very_long_query_string() {
        let mut long_query = String::from("key=");
        long_query.push_str(&"a".repeat(10000));
        let result = parse_query_string(long_query.as_bytes(), '&');
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "key");
        assert_eq!(result[0].1.len(), 10000);
    }

    #[test]
    fn test_very_large_number_of_parameters() {
        let mut query = String::new();
        for i in 0..100 {
            if i > 0 {
                query.push('&');
            }
            query.push_str(&format!("param{}=value{}", i, i));
        }
        let result = parse_query_string(query.as_bytes(), '&');
        assert_eq!(result.len(), 100);
        assert_eq!(result[0].0, "param0");
        assert_eq!(result[99].0, "param99");
    }

    #[test]
    fn test_literal_space_in_value() {
        let result = parse_query_string(b"name=hello world", '&');
        assert_eq!(result, vec![(String::from("name"), String::from("hello world"))]);
    }

    #[test]
    fn test_tab_in_value() {
        let result = parse_query_string(b"name=hello\tworld", '&');
        assert_eq!(result, vec![(String::from("name"), String::from("hello\tworld"))]);
    }

    #[test]
    fn test_newline_in_value() {
        let result = parse_query_string(b"name=hello\nworld", '&');
        assert_eq!(result, vec![(String::from("name"), String::from("hello\nworld"))]);
    }
}
