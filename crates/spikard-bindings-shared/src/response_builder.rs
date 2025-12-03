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
    fn test_response_builder() {
        let (status, _, body) = ResponseBuilder::new()
            .status(StatusCode::CREATED)
            .body(json!({ "id": 123 }))
            .build();

        assert_eq!(status, StatusCode::CREATED);
        assert!(body.contains("123"));
    }
}
