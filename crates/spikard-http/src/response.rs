//! HTTP Response types
//!
//! Response types for returning custom responses with status codes, headers, and content

use serde_json::Value;
use std::collections::HashMap;

/// HTTP Response with custom status code, headers, and content
#[derive(Debug, Clone)]
pub struct Response {
    /// Response body content
    pub content: Option<Value>,
    /// HTTP status code (defaults to 200)
    pub status_code: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
}

impl Response {
    /// Create a new Response with default status 200
    pub fn new(content: Option<Value>) -> Self {
        Self {
            content,
            status_code: 200,
            headers: HashMap::new(),
        }
    }

    /// Create a response with a specific status code
    pub fn with_status(content: Option<Value>, status_code: u16) -> Self {
        Self {
            content,
            status_code,
            headers: HashMap::new(),
        }
    }

    /// Set a header
    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /// Set a cookie in the response
    #[allow(clippy::too_many_arguments)]
    pub fn set_cookie(
        &mut self,
        key: String,
        value: String,
        max_age: Option<i64>,
        domain: Option<String>,
        path: Option<String>,
        secure: bool,
        http_only: bool,
        same_site: Option<String>,
    ) {
        let mut cookie_value = format!("{}={}", key, value);

        if let Some(age) = max_age {
            cookie_value.push_str(&format!("; Max-Age={}", age));
        }
        if let Some(d) = domain {
            cookie_value.push_str(&format!("; Domain={}", d));
        }
        if let Some(p) = path {
            cookie_value.push_str(&format!("; Path={}", p));
        }
        if secure {
            cookie_value.push_str("; Secure");
        }
        if http_only {
            cookie_value.push_str("; HttpOnly");
        }
        if let Some(ss) = same_site {
            cookie_value.push_str(&format!("; SameSite={}", ss));
        }

        self.headers.insert("set-cookie".to_string(), cookie_value);
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new(None)
    }
}
