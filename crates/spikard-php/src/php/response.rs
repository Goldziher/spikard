//! PHP-visible HTTP response struct.

#![allow(non_snake_case)]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible HTTP response container.
#[php_class]
#[php(name = "Spikard\\Internal\\Response")]
pub struct PhpResponse {
    #[php(prop, name = "statusCode")]
    pub(crate) status_code: i64,
    pub(crate) body: Value,
    #[php(prop)]
    pub(crate) headers: HashMap<String, String>,
    #[php(prop)]
    pub(crate) cookies: HashMap<String, String>,
}

#[php_impl]
impl PhpResponse {
    #[php(constructor)]
    pub fn __construct(
        body: Option<&Zval>,
        statusCode: Option<i64>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
    ) -> PhpResult<Self> {
        Self::new(body, statusCode, headers, cookies)
    }

    /// Create a new response.
    ///
    /// This intentionally matches `packages/php/src/Http/Response.php` so PHP code can use named
    /// arguments like `statusCode:` even when the native extension is loaded.
    #[php(name = "create")]
    #[allow(non_snake_case)]
    pub fn new(
        body: Option<&Zval>,
        statusCode: Option<i64>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
    ) -> PhpResult<Self> {
        let body_value = match body {
            None => Value::Null,
            Some(v) => crate::php::zval_to_json(v).map_err(PhpException::default)?,
        };

        let mut headers = headers.unwrap_or_default();
        headers = headers.into_iter().map(|(k, v)| (k.to_ascii_lowercase(), v)).collect();

        Ok(Self {
            status_code: statusCode.unwrap_or(200),
            body: body_value,
            headers,
            cookies: cookies.unwrap_or_default(),
        })
    }

    /// Get the status code.
    #[php(name = "getStatus")]
    pub fn get_status(&self) -> i64 {
        self.status_code
    }

    /// Alias for status code.
    #[php(name = "getStatusCode")]
    pub fn get_status_code(&self) -> i64 {
        self.status_code
    }

    /// PHP property getter for `body`.
    #[php(getter, name = "body")]
    pub fn body_prop(&self) -> PhpResult<Zval> {
        crate::php::json_to_zval(&self.body)
    }

    /// Get the body as JSON string.
    #[php(name = "getBody")]
    pub fn get_body(&self) -> String {
        match &self.body {
            Value::String(s) => s.clone(),
            _ => serde_json::to_string(&self.body).unwrap_or_else(|_| "{}".to_string()),
        }
    }

    /// Parse response body as JSON and return as PHP array.
    #[php(name = "parseJson")]
    pub fn parse_json(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let body_str = self.get_body();
        let parsed: Value =
            serde_json::from_str(&body_str).map_err(|e| PhpException::default(format!("Invalid JSON body: {}", e)))?;
        crate::php::json_to_php_table(&parsed)
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

    /// Get cookies as a PHP array.
    #[php(name = "getCookies")]
    pub fn get_cookies(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.cookies {
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
        self.status_code = status;
    }

    /// Create a JSON response.
    #[php(name = "json")]
    pub fn json(data: &Zval, status: Option<i64>, headers: Option<HashMap<String, String>>) -> PhpResult<Self> {
        let mut response = Self::new(Some(data), status, headers, None)?;
        response
            .headers
            .insert("content-type".to_string(), "application/json".to_string());
        Ok(response)
    }

    /// Create a text response.
    #[php(name = "text")]
    pub fn text_response(
        body: String,
        status: Option<i64>,
        headers: Option<HashMap<String, String>>,
    ) -> PhpResult<Self> {
        let mut response = Self::new(None, status, headers, None)?;
        response.body = Value::String(body);
        response
            .headers
            .insert("content-type".to_string(), "text/plain; charset=utf-8".to_string());
        Ok(response)
    }

    /// Return a copy of this response with new cookies.
    #[php(name = "withCookies")]
    pub fn with_cookies(&self, cookies: HashMap<String, String>) -> Self {
        Self {
            status_code: self.status_code,
            body: self.body.clone(),
            headers: self.headers.clone(),
            cookies,
        }
    }
}

impl PhpResponse {
    /// Build a JSON response (internal use).
    pub fn with_json(body: Value, status: Option<i64>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Self {
            status_code: status.unwrap_or(200),
            body,
            headers,
            cookies: HashMap::new(),
        }
    }

    /// Build a plain-text response (internal use).
    pub fn text(body: String, status: Option<i64>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        Self {
            status_code: status.unwrap_or(200),
            body: Value::String(body),
            headers,
            cookies: HashMap::new(),
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
