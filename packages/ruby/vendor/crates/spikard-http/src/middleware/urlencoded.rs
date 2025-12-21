//! URL-encoded form data parsing

use std::collections::HashMap;
use url::form_urlencoded;

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
    if data.contains(&b'[') {
        let body_str = std::str::from_utf8(data)?;
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

    let mut array_map: FxHashMap<String, Vec<serde_json::Value>> = FxHashMap::default();

    for (key, value) in form_urlencoded::parse(data) {
        let json_value = convert_string_to_json_value(&value);
        array_map.entry(key.into_owned()).or_default().push(json_value);
    }

    let mut obj = serde_json::Map::with_capacity(array_map.len());
    for (key, mut values) in array_map {
        if values.len() == 1 {
            obj.insert(key, values.pop().unwrap_or(serde_json::Value::Null));
        } else {
            obj.insert(key, serde_json::Value::Array(values));
        }
    }
    serde_json::Value::Object(obj)
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
    if s.eq_ignore_ascii_case("true") {
        Some(serde_json::Value::Bool(true))
    } else if s.eq_ignore_ascii_case("false") {
        Some(serde_json::Value::Bool(false))
    } else {
        None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_key_value() {
        let data = b"name=value";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result.is_object());
        assert_eq!(result["name"], "value");
    }

    #[test]
    fn test_array_notation() {
        let data = b"tags[]=a&tags[]=b&tags[]=c";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["tags"].is_array());
        let tags = result["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "a");
        assert_eq!(tags[1], "b");
        assert_eq!(tags[2], "c");
    }

    #[test]
    fn test_nested_objects() {
        let data = b"profile[name]=John&profile[age]=30";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["profile"].is_object());
        assert_eq!(result["profile"]["name"], "John");
        assert_eq!(result["profile"]["age"], 30);
    }

    #[test]
    fn test_type_conversion_integers() {
        let data = b"age=30&count=1000&id=12345";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["age"].is_number());
        assert_eq!(result["age"], 30);
        assert!(result["count"].is_number());
        assert_eq!(result["count"], 1000);
        assert!(result["id"].is_number());
        assert_eq!(result["id"], 12345);
    }

    #[test]
    fn test_type_conversion_booleans() {
        let data = b"active=true&enabled=false&visible=True&disabled=False";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["active"].is_boolean());
        assert_eq!(result["active"], true);
        assert!(result["enabled"].is_boolean());
        assert_eq!(result["enabled"], false);
        assert!(result["visible"].is_boolean());
        assert_eq!(result["visible"], true);
        assert!(result["disabled"].is_boolean());
        assert_eq!(result["disabled"], false);
    }

    #[test]
    fn test_type_conversion_floats() {
        let data = b"price=19.99&rating=4.5&discount=0.25";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["price"].is_number());
        assert_eq!(result["price"], 19.99);
        assert!(result["rating"].is_number());
        assert_eq!(result["rating"], 4.5);
        assert!(result["discount"].is_number());
        assert_eq!(result["discount"], 0.25);
    }

    #[test]
    fn test_multiple_values_same_key() {
        let data = b"tags=a&tags=b&tags=c";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["tags"].is_array());
        let tags = result["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "a");
        assert_eq!(tags[1], "b");
        assert_eq!(tags[2], "c");
    }

    #[test]
    fn test_empty_strings() {
        let data = b"name=&description=&value=";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["name"], "");
        assert_eq!(result["description"], "");
        assert_eq!(result["value"], "");
    }

    #[test]
    fn test_url_encoded_spaces() {
        let data = b"name=John+Doe&message=Hello%20World";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["name"], "John Doe");
        assert_eq!(result["message"], "Hello World");
    }

    #[test]
    fn test_url_encoded_special_chars() {
        let data = b"email=user%40example.com&url=https%3A%2F%2Fexample.com";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["email"], "user@example.com");
        assert_eq!(result["url"], "https://example.com");
    }

    #[test]
    fn test_null_value() {
        let data = b"value=null&other=something";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["value"].is_null());
        assert_eq!(result["other"], "something");
    }

    #[test]
    fn test_multiple_fields() {
        let data = b"username=john&password=secret123&remember=true&age=28";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["username"], "john");
        assert_eq!(result["password"], "secret123");
        assert_eq!(result["remember"], true);
        assert_eq!(result["age"], 28);
    }

    #[test]
    fn test_convert_string_to_json_value_integer() {
        let value = convert_string_to_json_value("42");
        assert_eq!(value, 42);
    }

    #[test]
    fn test_convert_string_to_json_value_float() {
        let value = convert_string_to_json_value("3.14");
        assert_eq!(value, 3.14);
    }

    #[test]
    fn test_convert_string_to_json_value_boolean_true() {
        let value = convert_string_to_json_value("true");
        assert_eq!(value, true);
    }

    #[test]
    fn test_convert_string_to_json_value_boolean_false() {
        let value = convert_string_to_json_value("false");
        assert_eq!(value, false);
    }

    #[test]
    fn test_convert_string_to_json_value_null() {
        let value = convert_string_to_json_value("null");
        assert!(value.is_null());
    }

    #[test]
    fn test_convert_string_to_json_value_empty_string() {
        let value = convert_string_to_json_value("");
        assert_eq!(value, "");
    }

    #[test]
    fn test_convert_string_to_json_value_regular_string() {
        let value = convert_string_to_json_value("hello");
        assert_eq!(value, "hello");
    }

    #[test]
    fn test_try_parse_integer() {
        assert!(try_parse_integer("123").is_some());
        assert!(try_parse_integer("-456").is_some());
        assert!(try_parse_integer("0").is_some());
        assert!(try_parse_integer("abc").is_none());
        assert!(try_parse_integer("12.34").is_none());
    }

    #[test]
    fn test_try_parse_float() {
        assert!(try_parse_float("3.14").is_some());
        assert!(try_parse_float("10.0").is_some());
        assert!(try_parse_float("-2.5").is_some());
        assert!(try_parse_float("abc").is_none());
        assert!(try_parse_float("123").is_some());
    }

    #[test]
    fn test_try_parse_boolean() {
        assert_eq!(try_parse_boolean("true"), Some(serde_json::Value::Bool(true)));
        assert_eq!(try_parse_boolean("false"), Some(serde_json::Value::Bool(false)));
        assert_eq!(try_parse_boolean("True"), Some(serde_json::Value::Bool(true)));
        assert_eq!(try_parse_boolean("FALSE"), Some(serde_json::Value::Bool(false)));
        assert_eq!(try_parse_boolean("1"), None);
        assert_eq!(try_parse_boolean("yes"), None);
    }

    #[test]
    fn test_mixed_types_in_array() {
        let data = b"values=1&values=hello&values=2.5&values=true";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["values"].is_array());
        let values = result["values"].as_array().unwrap();
        assert_eq!(values.len(), 4);
        assert_eq!(values[0], 1);
        assert_eq!(values[1], "hello");
        assert_eq!(values[2], 2.5);
        assert_eq!(values[3], true);
    }

    #[test]
    fn test_deeply_nested_objects() {
        let data = b"user[profile]=test&user[id]=123";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["user"].is_object());
        assert_eq!(result["user"]["profile"], "test");
        assert_eq!(result["user"]["id"], 123);
    }

    #[test]
    fn test_single_key_without_value() {
        let data = b"key";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["key"], "");
    }

    #[test]
    fn test_empty_form_data() {
        let data = b"";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result.is_object());
        assert!(result.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_negative_numbers() {
        let data = b"temp=-15&balance=-1000.50";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["temp"], -15);
        assert_eq!(result["balance"], -1000.50);
    }

    #[test]
    fn test_large_numbers() {
        let data = b"big=9223372036854775807&decimal=999999.99";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["big"].is_number());
        assert!(result["decimal"].is_number());
    }

    #[test]
    fn test_unicode_values() {
        let data = "name=José&city=São+Paulo".as_bytes();
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["name"], "José");
        assert_eq!(result["city"], "São Paulo");
    }

    #[test]
    fn test_ampersand_escaping() {
        let data = b"text=A%26B&code=1%262";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["text"], "A&B");
        assert_eq!(result["code"], "1&2");
    }

    #[test]
    fn test_equals_sign_escaping() {
        let data = b"expression=1%3D1&formula=2%3D2";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["expression"], "1=1");
        assert_eq!(result["formula"], "2=2");
    }

    #[test]
    fn test_leading_zeros_in_numbers() {
        let data = b"code=00123&id=007";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["code"].is_number());
        assert!(result["id"].is_number());
    }

    #[test]
    fn test_boolean_case_insensitivity() {
        let data = b"a=true&b=TRUE&c=True&d=false&e=FALSE&f=False";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["a"], true);
        assert_eq!(result["b"], true);
        assert_eq!(result["c"], true);
        assert_eq!(result["d"], false);
        assert_eq!(result["e"], false);
        assert_eq!(result["f"], false);
    }

    #[test]
    fn test_array_with_mixed_content_and_objects() {
        let data = b"items[]=1&items[]=text&categories[]=a&categories[]=b";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["items"].is_array());
        assert!(result["categories"].is_array());
        let items = result["items"].as_array().unwrap();
        assert_eq!(items[0], 1);
        assert_eq!(items[1], "text");
        let categories = result["categories"].as_array().unwrap();
        assert_eq!(categories[0], "a");
        assert_eq!(categories[1], "b");
    }

    #[test]
    fn test_scientific_notation() {
        let data = b"value=1e5&small=1e-3";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert!(result["value"].is_number());
        assert!(result["small"].is_number());
    }

    #[test]
    fn test_plus_vs_space_encoding() {
        let data = b"message=Hello+World&greeting=Hi%20There";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["message"], "Hello World");
        assert_eq!(result["greeting"], "Hi There");
    }

    #[test]
    fn test_no_value_key_in_middle() {
        let data = b"a=1&empty&c=3";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["a"], 1);
        assert_eq!(result["empty"], "");
        assert_eq!(result["c"], 3);
    }

    #[test]
    fn test_consecutive_ampersands() {
        let data = b"a=1&&b=2";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["a"], 1);
        assert_eq!(result["b"], 2);
        assert_eq!(result.as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_trailing_ampersand() {
        let data = b"a=1&b=2&";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["a"], 1);
        assert_eq!(result["b"], 2);
        assert_eq!(result.as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_leading_ampersand() {
        let data = b"&a=1&b=2";
        let result = parse_urlencoded_to_json(data).unwrap();

        assert_eq!(result["a"], 1);
        assert_eq!(result["b"], 2);
        assert_eq!(result.as_object().unwrap().len(), 2);
    }
}
