use serde_json::Value;

/// Encode JSON form data as application/x-www-form-urlencoded bytes.
pub fn encode_urlencoded_body(value: &Value) -> Result<Vec<u8>, String> {
    match value {
        Value::String(s) => Ok(s.as_bytes().to_vec()),
        Value::Null => Ok(Vec::new()),
        Value::Bool(b) => Ok(b.to_string().into_bytes()),
        Value::Number(num) => Ok(num.to_string().into_bytes()),
        Value::Object(_) | Value::Array(_) => serde_qs::to_string(value)
            .map(|encoded| encoded.into_bytes())
            .map_err(|err| err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn encodes_string_as_raw_bytes() {
        let out = encode_urlencoded_body(&Value::String("a=b&c=d".to_string())).unwrap();
        assert_eq!(out, b"a=b&c=d");
    }

    #[test]
    fn encodes_null_as_empty_body() {
        let out = encode_urlencoded_body(&Value::Null).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn encodes_bool_as_text() {
        assert_eq!(encode_urlencoded_body(&json!(true)).unwrap(), b"true");
        assert_eq!(encode_urlencoded_body(&json!(false)).unwrap(), b"false");
    }

    #[test]
    fn encodes_number_as_text() {
        assert_eq!(encode_urlencoded_body(&json!(42)).unwrap(), b"42");
        assert_eq!(encode_urlencoded_body(&json!(3.5)).unwrap(), b"3.5");
    }

    #[test]
    fn encodes_object_using_querystring_format() {
        let out = encode_urlencoded_body(&json!({"a": 1, "b": "x"})).unwrap();
        let text = std::str::from_utf8(&out).unwrap();
        assert!(text.contains("a=1"));
        assert!(text.contains("b=x"));
    }
}
