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
