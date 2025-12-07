//! Unit tests for FFI conversion logic
//!
//! This test module verifies Rust-side conversion logic for HandlerInput/HandlerOutput
//! that is unit testable without a Node.js runtime or napi::Env.
//!
//! These tests focus on:
//! - HandlerInput::from(&RequestData) field conversions
//! - HandlerOutput::into_response() response building
//! - Header and cookie marshalling
//! - Error response structure validation
//!
//! NOTE: Actual FFI boundaries (ThreadsafeFunction calls, Promise handling, etc.)
//! are NOT tested here because they require a Node.js runtime. Those tests belong
//! in packages/node/tests/ as JavaScript integration tests.

#[cfg(test)]
mod conversion_tests {
    use crate::{HandlerInput, HandlerOutput};
    use serde_json::{Value, json};
    use spikard_http::RequestData;
    use std::collections::HashMap;
    use std::sync::Arc;

    // ========================================================================
    // CONVERSION TESTS: HandlerInput::from(&RequestData)
    // ========================================================================

    /// Test basic HandlerInput conversion from RequestData
    #[test]
    fn test_handler_input_basic_conversion() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("authorization".to_string(), "Bearer token123".to_string());

        let request = RequestData {
            path: "/api/users".to_string(),
            method: "POST".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({"limit": 10}),
            headers: Arc::new(headers),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: json!({"name": "Alice"}),
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.method, "POST");
        assert_eq!(input.path, "/api/users");
        assert_eq!(input.headers.get("content-type").unwrap(), "application/json");
        assert_eq!(input.headers.get("authorization").unwrap(), "Bearer token123");
        assert_eq!(input.query_params["limit"], 10);
        assert_eq!(input.body["name"], "Alice");
    }

    /// Test HandlerInput with path parameters
    #[test]
    fn test_handler_input_path_params_conversion() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "42".to_string());
        path_params.insert("slug".to_string(), "my-article".to_string());

        let request = RequestData {
            path: "/api/articles/:id/:slug".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(path_params),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.path_params.get("id").unwrap(), "42");
        assert_eq!(input.path_params.get("slug").unwrap(), "my-article");
        assert_eq!(input.path_params.len(), 2);
    }

    /// Test HandlerInput with cookies conversion
    #[test]
    fn test_handler_input_cookies_conversion() {
        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "abc123def456".to_string());
        cookies.insert("preferences".to_string(), "dark_mode=true".to_string());

        let request = RequestData {
            path: "/dashboard".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(cookies),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.cookies.get("session_id").unwrap(), "abc123def456");
        assert_eq!(input.cookies.get("preferences").unwrap(), "dark_mode=true");
        assert_eq!(input.cookies.len(), 2);
    }

    /// Test HandlerInput with complex JSON body
    #[test]
    fn test_handler_input_complex_body_conversion() {
        let complex_body = json!({
            "user": {
                "name": "Bob",
                "email": "bob@example.com",
                "roles": ["admin", "user"],
                "metadata": {
                    "created": "2025-01-01T00:00:00Z",
                    "active": true
                }
            },
            "options": [1, 2, 3, 4, 5]
        });

        let request = RequestData {
            path: "/api/users".to_string(),
            method: "POST".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: complex_body.clone(),
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        // Verify nested structure is preserved
        assert_eq!(input.body["user"]["name"], "Bob");
        assert_eq!(input.body["user"]["email"], "bob@example.com");
        assert_eq!(input.body["user"]["roles"].as_array().unwrap().len(), 2);
        assert_eq!(input.body["user"]["metadata"]["active"], true);
        assert_eq!(input.body["options"].as_array().unwrap().len(), 5);
    }

    /// Test HandlerInput with query parameters
    #[test]
    fn test_handler_input_query_params_conversion() {
        let query_params = json!({
            "page": 1,
            "limit": 20,
            "sort": "created_at",
            "filter": {
                "status": "active"
            }
        });

        let request = RequestData {
            path: "/api/items".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.query_params["page"], 1);
        assert_eq!(input.query_params["limit"], 20);
        assert_eq!(input.query_params["sort"], "created_at");
        assert_eq!(input.query_params["filter"]["status"], "active");
    }

    // ========================================================================
    // CONVERSION TESTS: HandlerOutput::into_response()
    // ========================================================================

    /// Test basic HandlerOutput response building
    #[test]
    fn test_handler_output_basic_response() {
        let output = HandlerOutput {
            status: 200,
            headers: None,
            body: Some(json!({"message": "OK"})),
        };

        let response = output.into_response().expect("response should build");

        assert_eq!(response.status().as_u16(), 200);
    }

    /// Test HandlerOutput with headers
    #[test]
    fn test_handler_output_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "custom_value".to_string());
        headers.insert("content-type".to_string(), "application/json".to_string());

        let output = HandlerOutput {
            status: 201,
            headers: Some(headers),
            body: Some(json!({"id": 123})),
        };

        let response = output.into_response().expect("response should build");

        assert_eq!(response.status().as_u16(), 201);
    }

    /// Test HandlerOutput with null body
    #[test]
    fn test_handler_output_null_body() {
        let output = HandlerOutput {
            status: 204,
            headers: None,
            body: None,
        };

        let response = output.into_response().expect("response should build");

        assert_eq!(response.status().as_u16(), 204);
    }

    /// Test HandlerOutput with complex body
    #[test]
    fn test_handler_output_complex_body() {
        let complex_body = json!({
            "data": [
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"},
                {"id": 3, "name": "Item 3"}
            ],
            "pagination": {
                "page": 1,
                "total": 3,
                "per_page": 20
            },
            "meta": {
                "timestamp": "2025-12-06T10:00:00Z"
            }
        });

        let output = HandlerOutput {
            status: 200,
            headers: None,
            body: Some(complex_body),
        };

        let response = output.into_response().expect("response should build");

        assert_eq!(response.status().as_u16(), 200);
    }

    /// Test HandlerOutput with various valid HTTP status codes
    #[test]
    fn test_handler_output_edge_case_statuses() {
        for status_code in &[100, 201, 301, 400, 404, 500, 503] {
            let output = HandlerOutput {
                status: *status_code,
                headers: None,
                body: Some(json!({"status": status_code})),
            };

            let response = output
                .into_response()
                .expect(&format!("status {} should build", status_code));
            assert_eq!(response.status().as_u16(), *status_code);
        }
    }

    // ========================================================================
    // CONVERSION TESTS: Header Marshalling
    // ========================================================================

    /// Test header case preservation
    #[test]
    fn test_header_case_preservation() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Custom-Header".to_string(), "value".to_string());
        headers.insert("authorization".to_string(), "Bearer token".to_string());

        let request = RequestData {
            path: "/test".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(headers.clone()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.headers.len(), 3);
        assert!(input.headers.contains_key("Content-Type"));
        assert!(input.headers.contains_key("X-Custom-Header"));
        assert!(input.headers.contains_key("authorization"));
    }

    /// Test header value with special characters
    #[test]
    fn test_header_special_characters() {
        let mut headers = HashMap::new();
        headers.insert(
            "x-custom".to_string(),
            "value with spaces and-dashes_underscores".to_string(),
        );

        let request = RequestData {
            path: "/test".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(headers),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);
        let header_val = input.headers.get("x-custom").unwrap();
        assert_eq!(header_val, "value with spaces and-dashes_underscores");
    }

    // ========================================================================
    // CONVERSION TESTS: Cookie Marshalling
    // ========================================================================

    /// Test cookie with special characters
    #[test]
    fn test_cookie_special_characters() {
        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "abc123!@#$%^&*()".to_string());
        cookies.insert("prefs".to_string(), "dark=true;lang=en".to_string());

        let request = RequestData {
            path: "/test".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(cookies),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert_eq!(input.cookies.get("session_id").unwrap(), "abc123!@#$%^&*()");
        assert_eq!(input.cookies.get("prefs").unwrap(), "dark=true;lang=en");
    }

    // ========================================================================
    // CONVERSION TESTS: Error Response Structures
    // ========================================================================

    /// Test error response structure compliance
    #[test]
    fn test_error_response_structure_validation() {
        let error_structures = vec![
            json!({
                "error": "handler_input_conversion_failed",
                "code": "CONVERSION_ERROR",
                "details": {
                    "field": "body",
                    "reason": "Invalid JSON"
                }
            }),
            json!({
                "error": "response_serialization_failed",
                "code": "SERIALIZATION_ERROR",
                "details": {
                    "field": "body_value",
                    "reason": "Circular reference detected"
                }
            }),
            json!({
                "error": "header_invalid",
                "code": "HEADER_ERROR",
                "details": {
                    "header_name": "x-custom",
                    "reason": "Invalid header value"
                }
            }),
        ];

        for error in error_structures {
            assert!(error.is_object(), "Error must be object");
            assert!(error["error"].is_string(), "error field required");
            assert!(error["code"].is_string(), "code field required");
            assert!(error["details"].is_object(), "details field required");
            assert!(!error["error"].as_str().unwrap().is_empty());
            assert!(!error["code"].as_str().unwrap().is_empty());
        }
    }

    // ========================================================================
    // EDGE CASES: Empty and Null Values
    // ========================================================================

    /// Test HandlerInput with empty collections
    #[test]
    fn test_handler_input_empty_collections() {
        let request = RequestData {
            path: "/empty".to_string(),
            method: "GET".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: Value::Null,
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert!(input.path_params.is_empty());
        assert!(input.headers.is_empty());
        assert!(input.cookies.is_empty());
        assert!(input.query_params.is_object());
        assert!(input.body.is_null());
    }

    /// Test HandlerOutput with all optional fields None
    #[test]
    fn test_handler_output_minimal() {
        let output = HandlerOutput {
            status: 200,
            headers: None,
            body: None,
        };

        let response = output.into_response().expect("response should build");
        assert_eq!(response.status().as_u16(), 200);
    }

    // ========================================================================
    // EDGE CASES: Large Data Structures
    // ========================================================================

    /// Test HandlerInput with large arrays
    #[test]
    fn test_handler_input_large_array_body() {
        let large_array: Vec<Value> = (0..1000)
            .map(|i| {
                json!({
                    "id": i,
                    "name": format!("item_{}", i),
                    "active": i % 2 == 0
                })
            })
            .collect();

        let request = RequestData {
            path: "/api/items".to_string(),
            method: "POST".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: json!(large_array),
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        assert!(input.body.is_array());
        assert_eq!(input.body.as_array().unwrap().len(), 1000);
    }

    /// Test HandlerInput with deeply nested objects
    #[test]
    fn test_handler_input_deeply_nested_body() {
        let mut nested = json!({"value": "deep"});
        for _ in 0..49 {
            nested = json!({"nested": nested});
        }

        let request = RequestData {
            path: "/api/nested".to_string(),
            method: "POST".to_string(),
            path_params: Arc::new(HashMap::new()),
            query_params: json!({}),
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            raw_query_params: Arc::new(HashMap::new()),
            body: nested.clone(),
            raw_body: None,
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let input = HandlerInput::from(&request);

        let mut current = input.body.clone();
        for _ in 0..49 {
            assert!(current.is_object());
            current = current["nested"].clone();
        }
        assert_eq!(current["value"], "deep");
    }
}
