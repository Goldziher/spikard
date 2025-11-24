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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn response_new_creates_default_status() {
        let response = Response::new(None);
        assert_eq!(response.status_code, 200);
        assert!(response.headers.is_empty());
        assert!(response.content.is_none());
    }

    #[test]
    fn response_new_with_content() {
        let content = json!({"key": "value"});
        let response = Response::new(Some(content.clone()));
        assert_eq!(response.status_code, 200);
        assert_eq!(response.content, Some(content));
    }

    #[test]
    fn response_with_status() {
        let response = Response::with_status(None, 404);
        assert_eq!(response.status_code, 404);
        assert!(response.headers.is_empty());
    }

    #[test]
    fn response_with_status_and_content() {
        let content = json!({"error": "not found"});
        let response = Response::with_status(Some(content.clone()), 404);
        assert_eq!(response.status_code, 404);
        assert_eq!(response.content, Some(content));
    }

    #[test]
    fn response_set_header() {
        let mut response = Response::new(None);
        response.set_header("X-Custom".to_string(), "custom-value".to_string());
        assert_eq!(response.headers.get("X-Custom"), Some(&"custom-value".to_string()));
    }

    #[test]
    fn response_set_multiple_headers() {
        let mut response = Response::new(None);
        response.set_header("Content-Type".to_string(), "application/json".to_string());
        response.set_header("X-Custom".to_string(), "custom-value".to_string());
        assert_eq!(response.headers.len(), 2);
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(response.headers.get("X-Custom"), Some(&"custom-value".to_string()));
    }

    #[test]
    fn response_set_header_overwrites() {
        let mut response = Response::new(None);
        response.set_header("X-Custom".to_string(), "value1".to_string());
        response.set_header("X-Custom".to_string(), "value2".to_string());
        assert_eq!(response.headers.get("X-Custom"), Some(&"value2".to_string()));
    }

    #[test]
    fn response_set_cookie_minimal() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session_id".to_string(),
            "abc123".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert_eq!(cookie, "session_id=abc123");
    }

    #[test]
    fn response_set_cookie_with_max_age() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            Some(3600),
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("session=token"));
        assert!(cookie.contains("Max-Age=3600"));
    }

    #[test]
    fn response_set_cookie_with_domain() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            None,
            Some("example.com".to_string()),
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("Domain=example.com"));
    }

    #[test]
    fn response_set_cookie_with_path() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            None,
            None,
            Some("/app".to_string()),
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("Path=/app"));
    }

    #[test]
    fn response_set_cookie_secure() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            None,
            None,
            None,
            true,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("Secure"));
    }

    #[test]
    fn response_set_cookie_http_only() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            None,
            None,
            None,
            false,
            true,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("HttpOnly"));
    }

    #[test]
    fn response_set_cookie_same_site() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            None,
            None,
            None,
            false,
            false,
            Some("Strict".to_string()),
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("SameSite=Strict"));
    }

    #[test]
    fn response_set_cookie_all_attributes() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token123".to_string(),
            Some(3600),
            Some("example.com".to_string()),
            Some("/app".to_string()),
            true,
            true,
            Some("Lax".to_string()),
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("session=token123"));
        assert!(cookie.contains("Max-Age=3600"));
        assert!(cookie.contains("Domain=example.com"));
        assert!(cookie.contains("Path=/app"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
    }

    #[test]
    fn response_set_cookie_overwrites_previous() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "old_token".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        response.set_cookie(
            "session".to_string(),
            "new_token".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("new_token"));
        assert!(!cookie.contains("old_token"));
    }

    #[test]
    fn response_default() {
        let response = Response::default();
        assert_eq!(response.status_code, 200);
        assert!(response.headers.is_empty());
        assert!(response.content.is_none());
    }

    #[test]
    fn response_cookie_with_special_chars_in_value() {
        let mut response = Response::new(None);
        response.set_cookie(
            "name".to_string(),
            "value%3D123".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert_eq!(cookie, "name=value%3D123");
    }

    #[test]
    fn response_same_site_variants() {
        for same_site in &["Strict", "Lax", "None"] {
            let mut response = Response::new(None);
            response.set_cookie(
                "test".to_string(),
                "value".to_string(),
                None,
                None,
                None,
                false,
                false,
                Some(same_site.to_string()),
            );
            let cookie = response.headers.get("set-cookie").unwrap();
            assert!(cookie.contains(&format!("SameSite={}", same_site)));
        }
    }

    #[test]
    fn response_zero_max_age() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            Some(0),
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("Max-Age=0"));
    }

    #[test]
    fn response_negative_max_age() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            Some(-1),
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("Max-Age=-1"));
    }
}
