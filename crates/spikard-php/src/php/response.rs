use ext_php_rs::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible Response container (placeholder).
#[php_class(name = "Spikard\\Internal\\Response")]
pub struct PhpResponse {
    status: i64,
    body: Value,
    headers: HashMap<String, String>,
}

#[php_impl]
impl PhpResponse {
    #[constructor]
    pub fn __construct(body: Option<Value>, status: Option<i64>, headers: Option<HashMap<String, String>>) -> Self {
        Self {
            status: status.unwrap_or(200),
            body: body.unwrap_or(Value::Null),
            headers: headers.unwrap_or_default(),
        }
    }

    #[getter]
    pub fn status(&self) -> i64 {
        self.status
    }

    #[getter]
    pub fn body(&self) -> &Value {
        &self.body
    }

    #[getter]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Build a JSON response.
    pub fn json(body: Value, status: Option<i64>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Self {
            status: status.unwrap_or(200),
            body,
            headers,
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        for (k, v) in headers {
            self.headers.insert(k.to_ascii_lowercase(), v);
        }
        self
    }
}
