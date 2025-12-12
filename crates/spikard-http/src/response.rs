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

    #[test]
    fn response_various_status_codes() {
        let status_codes = vec![
            (200, "OK"),
            (201, "Created"),
            (204, "No Content"),
            (301, "Moved Permanently"),
            (302, "Found"),
            (304, "Not Modified"),
            (400, "Bad Request"),
            (401, "Unauthorized"),
            (403, "Forbidden"),
            (404, "Not Found"),
            (500, "Internal Server Error"),
            (502, "Bad Gateway"),
            (503, "Service Unavailable"),
        ];

        for (code, _name) in status_codes {
            let response = Response::with_status(None, code);
            assert_eq!(response.status_code, code);
            assert!(response.headers.is_empty());
        }
    }

    #[test]
    fn response_with_large_json_body() {
        let mut items = vec![];
        for i in 0..1000 {
            items.push(json!({"id": i, "name": format!("item_{}", i)}));
        }
        let large_array = serde_json::Value::Array(items);
        let response = Response::new(Some(large_array.clone()));
        assert_eq!(response.status_code, 200);
        assert_eq!(response.content, Some(large_array));
    }

    #[test]
    fn response_with_deeply_nested_json() {
        let nested = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "level5": {
                                "data": "deeply nested value"
                            }
                        }
                    }
                }
            }
        });
        let response = Response::new(Some(nested.clone()));
        assert_eq!(response.content, Some(nested));
    }

    #[test]
    fn response_with_empty_json_object() {
        let empty_obj = json!({});
        let response = Response::new(Some(empty_obj.clone()));
        assert_eq!(response.content, Some(empty_obj));
        assert_ne!(response.content, None);
    }

    #[test]
    fn response_with_empty_json_array() {
        let empty_array = json!([]);
        let response = Response::new(Some(empty_array.clone()));
        assert_eq!(response.content, Some(empty_array));
        assert_ne!(response.content, None);
    }

    #[test]
    fn response_with_null_vs_none() {
        let null_value = json!(null);
        let response_with_null = Response::new(Some(null_value.clone()));
        let response_with_none = Response::new(None);

        assert_eq!(response_with_null.content, Some(null_value));
        assert_eq!(response_with_none.content, None);
        assert_ne!(response_with_null.content, response_with_none.content);
    }

    #[test]
    fn response_with_json_primitives() {
        let test_cases = vec![
            json!(true),
            json!(false),
            json!(0),
            json!(-1),
            json!(42),
            json!(3.14),
            json!("string"),
            json!(""),
        ];

        for test_value in test_cases {
            let response = Response::new(Some(test_value.clone()));
            assert_eq!(response.content, Some(test_value));
        }
    }

    #[test]
    fn response_header_case_sensitivity() {
        let mut response = Response::new(None);
        response.set_header("Content-Type".to_string(), "application/json".to_string());
        response.set_header("content-type".to_string(), "text/plain".to_string());

        assert_eq!(response.headers.len(), 2);
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(response.headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn response_header_with_empty_value() {
        let mut response = Response::new(None);
        response.set_header("X-Empty".to_string(), "".to_string());
        assert_eq!(response.headers.get("X-Empty"), Some(&"".to_string()));
    }

    #[test]
    fn response_header_with_special_chars() {
        let mut response = Response::new(None);
        response.set_header("X-Special".to_string(), "value; charset=utf-8".to_string());
        assert_eq!(
            response.headers.get("X-Special"),
            Some(&"value; charset=utf-8".to_string())
        );
    }

    #[test]
    fn response_multiple_different_cookies() {
        let mut response = Response::new(None);
        response.set_cookie(
            "session".to_string(),
            "abc123".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie_count = response.headers.iter().filter(|(k, _)| *k == "set-cookie").count();
        assert_eq!(cookie_count, 1);
    }

    #[test]
    fn response_cookie_empty_value() {
        let mut response = Response::new(None);
        response.set_cookie(
            "empty".to_string(),
            "".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert_eq!(cookie, "empty=");
    }

    #[test]
    fn response_cookie_with_equals_in_value() {
        let mut response = Response::new(None);
        response.set_cookie(
            "data".to_string(),
            "key=value&other=123".to_string(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains("key=value&other=123"));
    }

    #[test]
    fn response_cookie_attribute_order() {
        let mut response = Response::new(None);
        response.set_cookie(
            "test".to_string(),
            "value".to_string(),
            Some(3600),
            Some("example.com".to_string()),
            Some("/".to_string()),
            true,
            true,
            Some("Strict".to_string()),
        );
        let cookie = response.headers.get("set-cookie").unwrap();

        let parts: Vec<&str> = cookie.split("; ").collect();
        assert_eq!(parts.len(), 7);
        assert!(parts[0].starts_with("test="));
        assert!(parts[1].starts_with("Max-Age="));
        assert!(parts[2].starts_with("Domain="));
        assert!(parts[3].starts_with("Path="));
        assert_eq!(parts[4], "Secure");
        assert_eq!(parts[5], "HttpOnly");
        assert!(parts[6].starts_with("SameSite="));
    }

    #[test]
    fn response_cookie_with_very_long_value() {
        let mut response = Response::new(None);
        let long_value = "x".repeat(4096);
        response.set_cookie(
            "long".to_string(),
            long_value.clone(),
            None,
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains(&format!("long={}", long_value)));
    }

    #[test]
    fn response_cookie_max_age_large_value() {
        let mut response = Response::new(None);
        let max_age_value = 86400 * 365;
        response.set_cookie(
            "session".to_string(),
            "token".to_string(),
            Some(max_age_value),
            None,
            None,
            false,
            false,
            None,
        );
        let cookie = response.headers.get("set-cookie").unwrap();
        assert!(cookie.contains(&format!("Max-Age={}", max_age_value)));
    }

    #[test]
    fn response_status_code_informational() {
        let response = Response::with_status(None, 100);
        assert_eq!(response.status_code, 100);
    }

    #[test]
    fn response_status_code_redirect_with_location() {
        let mut response = Response::with_status(None, 301);
        response.set_header("Location".to_string(), "https://example.com/new".to_string());
        assert_eq!(response.status_code, 301);
        assert_eq!(
            response.headers.get("Location"),
            Some(&"https://example.com/new".to_string())
        );
    }

    #[test]
    fn response_with_error_status_and_content() {
        let error_content = json!({
            "error": "Unauthorized",
            "code": 401,
            "message": "Invalid credentials"
        });
        let response = Response::with_status(Some(error_content.clone()), 401);
        assert_eq!(response.status_code, 401);
        assert_eq!(response.content, Some(error_content));
    }

    #[test]
    fn response_clone_preserves_state() {
        let mut original = Response::with_status(Some(json!({"key": "value"})), 202);
        original.set_header("X-Custom".to_string(), "header-value".to_string());
        original.set_cookie(
            "session".to_string(),
            "token".to_string(),
            Some(3600),
            None,
            None,
            true,
            false,
            None,
        );

        let cloned = original.clone();

        assert_eq!(cloned.status_code, 202);
        assert_eq!(cloned.content, original.content);
        assert_eq!(cloned.headers, original.headers);
    }

    #[test]
    fn response_with_numeric_status_boundaries() {
        let boundary_codes = vec![1, 99, 100, 199, 200, 299, 300, 399, 400, 499, 500, 599, 600, 999, 65535];
        for code in boundary_codes {
            let response = Response::with_status(None, code);
            assert_eq!(response.status_code, code);
        }
    }

    #[test]
    fn response_header_unicode_value() {
        let mut response = Response::new(None);
        response.set_header("X-Unicode".to_string(), "こんにちは".to_string());
        assert_eq!(response.headers.get("X-Unicode"), Some(&"こんにちは".to_string()));
    }

    #[test]
    fn response_debug_trait() {
        let response = Response::with_status(Some(json!({"test": "data"})), 200);
        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("Response"));
        assert!(debug_str.contains("200"));
    }
}
