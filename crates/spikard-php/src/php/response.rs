//! PHP-visible HTTP response struct.

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible HTTP response container.
#[php_class]
#[php(name = "Spikard\\Internal\\Response")]
pub struct PhpResponse {
    pub(crate) status: i64,
    pub(crate) body: Value,
    pub(crate) headers: HashMap<String, String>,
}

#[php_impl]
impl PhpResponse {
    /// Create a new response.
    pub fn new(body: Option<String>, status: Option<i64>, headers: Option<HashMap<String, String>>) -> Self {
        let body_value = body
            .map(|b| serde_json::from_str(&b).unwrap_or(Value::String(b)))
            .unwrap_or(Value::Null);
        Self {
            status: status.unwrap_or(200),
            body: body_value,
            headers: headers.unwrap_or_default(),
        }
    }

    /// Get the status code.
    #[php(name = "getStatus")]
    pub fn get_status(&self) -> i64 {
        self.status
    }

    /// Alias for status code.
    #[php(name = "getStatusCode")]
    pub fn get_status_code(&self) -> i64 {
        self.status
    }

    /// Get the body as JSON string.
    #[php(name = "getBody")]
    pub fn get_body(&self) -> String {
        serde_json::to_string(&self.body).unwrap_or_default()
    }

    /// Get headers as a PHP array.
    #[php(name = "getHeaders")]
    pub fn get_headers(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.headers {
            table.insert(k.as_str(), v.as_str())?;
        }
        Ok(table)
    }

    /// Set a header on this response.
    #[php(name = "setHeader")]
    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key.to_ascii_lowercase(), value);
    }

    /// Set the status code.
    #[php(name = "setStatus")]
    pub fn set_status(&mut self, status: i64) {
        self.status = status;
    }
}

impl PhpResponse {
    /// Build a JSON response (internal use).
    pub fn json(body: Value, status: Option<i64>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Self {
            status: status.unwrap_or(200),
            body,
            headers,
        }
    }

    /// Build a plain-text response (internal use).
    pub fn text(body: String, status: Option<i64>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        Self {
            status: status.unwrap_or(200),
            body: Value::String(body),
            headers,
        }
    }

    /// Add headers (internal use).
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        for (k, v) in headers {
            self.headers.insert(k.to_ascii_lowercase(), v);
        }
        self
    }
}
