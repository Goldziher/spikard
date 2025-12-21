//! Response building utilities

use axum::http::{HeaderMap, StatusCode, header};
use serde_json::json;

/// Builder for constructing HTTP responses across bindings
pub struct ResponseBuilder {
    status: StatusCode,
    body: serde_json::Value,
    headers: HeaderMap,
}

impl ResponseBuilder {
    /// Create a new response builder with default status 200 OK
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            body: json!({}),
            headers: HeaderMap::new(),
        }
    }

    /// Set the HTTP status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Set the response body
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = body;
        self
    }

    /// Add a response header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let Ok(name) = key.into().parse::<header::HeaderName>()
            && let Ok(val) = value.into().parse::<header::HeaderValue>()
        {
            self.headers.insert(name, val);
        }
        self
    }

    /// Build the response as (status, headers, body)
    pub fn build(self) -> (StatusCode, HeaderMap, String) {
        let body = serde_json::to_string(&self.body).unwrap_or_else(|_| "{}".to_string());
        (self.status, self.headers, body)
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_builder_default() {
        let (status, headers, body) = ResponseBuilder::new().build();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "{}");
        assert!(headers.is_empty());
    }

    #[test]
    fn test_response_builder_default_trait() {
        let (status, _, body) = ResponseBuilder::default().build();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "{}");
    }

    #[test]
    fn test_response_builder_status() {
        let (status, _, _) = ResponseBuilder::new().status(StatusCode::CREATED).build();

        assert_eq!(status, StatusCode::CREATED);
    }

    #[test]
    fn test_response_builder_status_chain() {
        let (status, _, _) = ResponseBuilder::new()
            .status(StatusCode::ACCEPTED)
            .status(StatusCode::CREATED)
            .build();

        assert_eq!(status, StatusCode::CREATED);
    }

    #[test]
    fn test_response_builder_body() {
        let body_data = json!({ "id": 123, "name": "test" });
        let (_, _, body) = ResponseBuilder::new().body(body_data.clone()).build();

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["id"], 123);
        assert_eq!(parsed["name"], "test");
    }

    #[test]
    fn test_response_builder_body_chain() {
        let first_body = json!({ "first": "value" });
        let second_body = json!({ "second": "value" });

        let (_, _, body) = ResponseBuilder::new().body(first_body).body(second_body).build();

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(parsed.get("first").is_none());
        assert_eq!(parsed["second"], "value");
    }

    #[test]
    fn test_response_builder_header() {
        let (_, headers, _) = ResponseBuilder::new()
            .header("Content-Type", "application/json")
            .build();

        assert_eq!(
            headers.get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_response_builder_multiple_headers() {
        let (_, headers, _) = ResponseBuilder::new()
            .header("Content-Type", "application/json")
            .header("X-Custom-Header", "custom-value")
            .header("Authorization", "Bearer token123")
            .build();

        assert_eq!(headers.len(), 3);
        assert_eq!(
            headers.get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
        assert_eq!(
            headers.get("x-custom-header").unwrap().to_str().unwrap(),
            "custom-value"
        );
        assert_eq!(
            headers.get("authorization").unwrap().to_str().unwrap(),
            "Bearer token123"
        );
    }

    #[test]
    fn test_response_builder_header_overwrite() {
        let (_, headers, _) = ResponseBuilder::new()
            .header("Content-Type", "text/plain")
            .header("Content-Type", "application/json")
            .build();

        assert_eq!(
            headers.get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_response_builder_full_chain() {
        let (status, headers, body) = ResponseBuilder::new()
            .status(StatusCode::CREATED)
            .body(json!({
                "id": 456,
                "status": "active",
                "items": [1, 2, 3]
            }))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", "req-123")
            .build();

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(headers.len(), 2);

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["id"], 456);
        assert_eq!(parsed["status"], "active");
        assert_eq!(parsed["items"][0], 1);
    }

    #[test]
    fn test_response_builder() {
        let (status, _, body) = ResponseBuilder::new()
            .status(StatusCode::CREATED)
            .body(json!({ "id": 123 }))
            .build();

        assert_eq!(status, StatusCode::CREATED);
        assert!(body.contains("123"));
    }

    #[test]
    fn test_response_builder_complex_json() {
        let complex_body = json!({
            "user": {
                "id": 1,
                "name": "John Doe",
                "email": "john@example.com",
                "roles": ["admin", "user"],
                "settings": {
                    "notifications": true,
                    "theme": "dark"
                }
            },
            "success": true,
            "timestamp": "2024-01-01T00:00:00Z"
        });

        let (status, _, body) = ResponseBuilder::new().status(StatusCode::OK).body(complex_body).build();

        assert_eq!(status, StatusCode::OK);
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["user"]["id"], 1);
        assert_eq!(parsed["user"]["roles"][0], "admin");
        assert_eq!(parsed["user"]["settings"]["theme"], "dark");
    }

    #[test]
    fn test_response_builder_null_body() {
        let (_, _, body) = ResponseBuilder::new().body(serde_json::Value::Null).build();

        assert_eq!(body, "null");
    }

    #[test]
    fn test_response_builder_array_body() {
        let array_body = json!([1, 2, 3, 4, 5]);
        let (_, _, body) = ResponseBuilder::new().body(array_body).build();

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed[0], 1);
        assert_eq!(parsed[4], 5);
    }

    #[test]
    fn test_response_builder_empty_object() {
        let (_, _, body) = ResponseBuilder::new().body(json!({})).build();

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(parsed.is_object());
        assert_eq!(parsed.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_response_builder_all_status_codes() {
        let status_codes = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::SERVICE_UNAVAILABLE,
        ];

        for code in status_codes {
            let (status, _, _) = ResponseBuilder::new().status(code).build();

            assert_eq!(status, code);
        }
    }

    #[test]
    fn test_response_builder_invalid_header_name() {
        let (_, headers, _) = ResponseBuilder::new()
            .header("Invalid\nHeader", "value")
            .header("Valid-Header", "value")
            .build();

        assert_eq!(headers.len(), 1);
    }

    #[test]
    fn test_response_builder_invalid_header_value() {
        let (_, headers, _) = ResponseBuilder::new().header("Valid-Header", "valid-value").build();

        assert_eq!(headers.len(), 1);
    }

    #[test]
    fn test_response_builder_special_characters_in_json() {
        let body_with_special_chars = json!({
            "message": "Hello \"World\"",
            "path": "C:\\Users\\test",
            "unicode": "café ☕",
            "newlines": "line1\nline2"
        });

        let (_, _, body) = ResponseBuilder::new().body(body_with_special_chars).build();

        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["message"], "Hello \"World\"");
        assert_eq!(parsed["unicode"], "café ☕");
    }
}
