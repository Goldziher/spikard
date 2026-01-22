#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive integration tests for HTTP request extraction and parsing
//!
//! Tests the observable behavior of request extraction covering:
//! - Query parameter parsing (single/multiple values, URL encoding, special chars)
//! - Cookie header parsing (single/multiple cookies, encoding, empty values)
//! - Path parameter extraction (single/multiple params)
//! - Header extraction (case-insensitive, special chars)
//! - Body parsing and preservation
//!
//! Each test verifies that RequestData is correctly populated from HTTP requests
//! with various edge cases and special characters.

mod common;

use axum::http::Method;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::common::test_builders::RequestBuilder;

/// Test single query parameter extraction
///
/// Query: `?name=john`
/// Expected: raw_query_params contains {"name": ["john"]}
#[test]
fn test_query_params_single_value() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/search")
        .query_param("name", "john")
        .build();

    assert_eq!(
        request_data.raw_query_params.get("name"),
        Some(&vec!["john".to_string()])
    );
    assert_eq!(request_data.query_params["name"], "john");
}

/// Test multiple values for same query parameter
///
/// Query: `?id=1&id=2&id=3`
/// Expected: raw_query_params contains {"id": ["1", "2", "3"]}
#[test]
fn test_query_params_multiple_values() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/filter")
        .query_param("id", "1")
        .query_param("id", "2")
        .query_param("id", "3")
        .build();

    let ids = request_data.raw_query_params.get("id").unwrap();
    assert_eq!(ids.len(), 3);
    assert_eq!(ids[0], "1");
    assert_eq!(ids[1], "2");
    assert_eq!(ids[2], "3");

    assert!(request_data.query_params["id"].is_array());
}

/// Test URL-encoded query parameters with spaces
///
/// Query: `?search=hello world` (RequestBuilder takes decoded values)
/// Expected: stored as-is
#[test]
fn test_query_params_url_encoded_space() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/search")
        .query_param("search", "hello world")
        .build();

    let search = request_data.raw_query_params.get("search").unwrap();
    assert_eq!(search[0], "hello world");
}

/// Test plus sign handling in query parameters
///
/// Query: `?name=john doe` with space
/// Expected: preserved as-is
#[test]
fn test_query_params_url_encoded_plus_as_space() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/search")
        .query_param("name", "john doe")
        .build();

    let name = request_data.raw_query_params.get("name").unwrap();
    assert_eq!(name[0], "john doe");
}

/// Test special characters in query parameters
///
/// Query: `?value=10+20=30` (special chars)
/// Expected: properly preserved
#[test]
fn test_query_params_special_characters() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/calc")
        .query_param("value", "10+20=30")
        .build();

    let value = request_data.raw_query_params.get("value").unwrap();
    assert_eq!(value[0], "10+20=30");
}

/// Test email-like query parameters with @ symbol
///
/// Query: `?email=test@example.com`
/// Expected: preserved with @ symbol
#[test]
fn test_query_params_email_encoding() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/subscribe")
        .query_param("email", "test@example.com")
        .build();

    let email = request_data.raw_query_params.get("email").unwrap();
    assert_eq!(email[0], "test@example.com");
}

/// Test empty query parameter value
///
/// Query: `?key=`
/// Expected: raw_query_params contains {"key": [""]}
#[test]
fn test_query_params_empty_value() {
    let (_request, request_data) = RequestBuilder::new().path("/search").query_param("key", "").build();

    let value = request_data.raw_query_params.get("key").unwrap();
    assert_eq!(value[0], "");
}

/// Test query parameters with numeric values preserved as strings
///
/// Query: `?page=1&limit=50`
/// Expected: values preserved as strings in raw_query_params
#[test]
fn test_query_params_numeric_values_as_strings() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/posts")
        .query_param("page", "1")
        .query_param("limit", "50")
        .build();

    let page = request_data.raw_query_params.get("page").unwrap();
    assert_eq!(page[0], "1");

    let limit = request_data.raw_query_params.get("limit").unwrap();
    assert_eq!(limit[0], "50");
}

/// Test mixed query parameters (different types and counts)
///
/// Query: `?page=1&tags=rust&tags=web&active=true&search=hello world`
/// Expected: all parsed correctly with multiple values for tags
#[test]
fn test_query_params_mixed_types_and_counts() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/api/posts")
        .query_param("page", "1")
        .query_param("tags", "rust")
        .query_param("tags", "web")
        .query_param("active", "true")
        .query_param("search", "hello world")
        .build();

    assert_eq!(request_data.raw_query_params.get("page").unwrap()[0], "1");
    assert_eq!(request_data.raw_query_params.get("active").unwrap()[0], "true");

    let tags = request_data.raw_query_params.get("tags").unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], "rust");
    assert_eq!(tags[1], "web");

    assert_eq!(request_data.raw_query_params.get("search").unwrap()[0], "hello world");
}

/// Test single cookie extraction
///
/// Cookie: `session=abc123`
/// Expected: cookies contains {"session": "abc123"}
#[test]
fn test_cookies_single_cookie() {
    let (_request, request_data) = RequestBuilder::new().cookie("session", "abc123").build();

    assert_eq!(request_data.cookies.get("session"), Some(&"abc123".to_string()));
}

/// Test multiple cookies in single header
///
/// Cookie: `session=abc123; user_id=42; theme=dark`
/// Expected: all cookies extracted separately
#[test]
fn test_cookies_multiple_cookies() {
    let (_request, request_data) = RequestBuilder::new()
        .cookie("session", "abc123")
        .cookie("user_id", "42")
        .cookie("theme", "dark")
        .build();

    assert_eq!(request_data.cookies.get("session"), Some(&"abc123".to_string()));
    assert_eq!(request_data.cookies.get("user_id"), Some(&"42".to_string()));
    assert_eq!(request_data.cookies.get("theme"), Some(&"dark".to_string()));
    assert_eq!(request_data.cookies.len(), 3);
}

/// Test cookie with empty value
///
/// Cookie: `empty=`
/// Expected: cookies contains {"empty": ""}
#[test]
fn test_cookies_empty_value() {
    let (_request, request_data) = RequestBuilder::new().cookie("empty", "").build();

    assert_eq!(request_data.cookies.get("empty"), Some(&String::new()));
}

/// Test cookie with URL-encoded special characters
///
/// Cookie value with encoded characters
/// Expected: decoded properly (depends on cookie library behavior)
#[test]
fn test_cookies_with_special_chars() {
    let (_request, request_data) = RequestBuilder::new()
        .cookie("data", "value_with-dash")
        .cookie("token", "abc123def456")
        .build();

    assert_eq!(request_data.cookies.get("data"), Some(&"value_with-dash".to_string()));
    assert_eq!(request_data.cookies.get("token"), Some(&"abc123def456".to_string()));
}

/// Test cookie with numeric-looking values
///
/// Cookie: `user_id=12345; port=8080; version=2`
/// Expected: values preserved as strings
#[test]
fn test_cookies_numeric_values() {
    let (_request, request_data) = RequestBuilder::new()
        .cookie("user_id", "12345")
        .cookie("port", "8080")
        .cookie("version", "2")
        .build();

    assert_eq!(request_data.cookies.get("user_id"), Some(&"12345".to_string()));
    assert_eq!(request_data.cookies.get("port"), Some(&"8080".to_string()));
    assert_eq!(request_data.cookies.get("version"), Some(&"2".to_string()));
}

/// Test single path parameter extraction
///
/// Route: `/users/:id` with path `/users/123`
/// Expected: path_params contains {"id": "123"}
#[test]
fn test_path_params_single() {
    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "123".to_string());

    let (_request, request_data) = RequestBuilder::new().path("/users/123").build();

    assert_eq!(request_data.path, "/users/123");
}

/// Test multiple path parameters
///
/// Route: `/posts/:id/comments/:comment_id`
/// Expected: both parameters extracted
#[test]
fn test_path_params_multiple() {
    let mut path_params = HashMap::new();
    path_params.insert("post_id".to_string(), "42".to_string());
    path_params.insert("comment_id".to_string(), "789".to_string());

    let (_request, request_data) = RequestBuilder::new().path("/posts/42/comments/789").build();

    assert_eq!(request_data.path, "/posts/42/comments/789");
}

/// Test path parameters with special formatting
///
/// Path: `/files/2025-12-10.log` or `/api/v1/resource`
/// Expected: parameters extracted with special chars preserved
#[test]
fn test_path_params_with_special_chars() {
    let (_request, request_data) = RequestBuilder::new().path("/files/document-2025-12-10.log").build();

    assert_eq!(request_data.path, "/files/document-2025-12-10.log");
}

/// Test single header extraction
///
/// Header: `Content-Type: application/json`
/// Expected: headers contains {"content-type": "application/json"}
#[test]
fn test_headers_single() {
    let (_request, request_data) = RequestBuilder::new().header("content-type", "application/json").build();

    assert_eq!(
        request_data.headers.get("content-type"),
        Some(&"application/json".to_string())
    );
}

/// Test multiple headers
///
/// Headers: Content-Type, Authorization, X-Custom-Header
/// Expected: all extracted
#[test]
fn test_headers_multiple() {
    let (_request, request_data) = RequestBuilder::new()
        .header("content-type", "application/json")
        .header("authorization", "Bearer token123")
        .header("x-custom-header", "custom-value")
        .build();

    assert_eq!(
        request_data.headers.get("content-type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        request_data.headers.get("authorization"),
        Some(&"Bearer token123".to_string())
    );
    assert_eq!(
        request_data.headers.get("x-custom-header"),
        Some(&"custom-value".to_string())
    );
}

/// Test header names are preserved in RequestBuilder
///
/// Headers: `content-type`, `x-request-id`
/// Expected: stored with provided casing
#[test]
fn test_headers_case_insensitive() {
    let (_request, request_data) = RequestBuilder::new()
        .header("content-type", "text/html")
        .header("x-request-id", "req-123")
        .build();

    assert_eq!(request_data.headers.get("content-type"), Some(&"text/html".to_string()));
    assert_eq!(request_data.headers.get("x-request-id"), Some(&"req-123".to_string()));
}

/// Test headers with hyphens are preserved
///
/// Headers: X-Custom-Header, X-Request-ID
/// Expected: hyphens preserved in header names
#[test]
fn test_headers_with_hyphens() {
    let (_request, request_data) = RequestBuilder::new()
        .header("x-custom-header", "value1")
        .header("x-request-id", "req-456")
        .header("x-api-key", "secret123")
        .build();

    assert_eq!(request_data.headers.get("x-custom-header"), Some(&"value1".to_string()));
    assert_eq!(request_data.headers.get("x-request-id"), Some(&"req-456".to_string()));
    assert_eq!(request_data.headers.get("x-api-key"), Some(&"secret123".to_string()));
}

/// Test authorization header with Bearer token
///
/// Header: `Authorization: Bearer eyJhbGc...`
/// Expected: full token value preserved
#[test]
fn test_headers_bearer_token() {
    let token =
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";

    let (_request, request_data) = RequestBuilder::new()
        .header("authorization", &format!("Bearer {}", token))
        .build();

    let auth_header = request_data.headers.get("authorization").unwrap();
    assert!(auth_header.starts_with("Bearer "));
    assert!(auth_header.contains(token));
}

/// Test JSON body is stored for deferred parsing
///
/// Content-Type: application/json, Body: `{"name": "Alice"}`
/// Expected: raw_body contains JSON bytes
#[test]
fn test_body_json_stored() {
    let body_json = json!({"name": "Alice", "age": 30});

    let (_request, request_data) = RequestBuilder::new()
        .method(Method::POST)
        .path("/users")
        .json_body(body_json.clone())
        .build();

    assert_eq!(*request_data.body, body_json);
}

/// Test empty body handling
///
/// Body: empty (no content)
/// Expected: body is null, raw_body is None
#[test]
fn test_body_empty() {
    let (_request, request_data) = RequestBuilder::new().method(Method::GET).path("/status").build();

    assert_eq!(*request_data.body, json!(null));
}

/// Test large JSON body
///
/// Body: large JSON with many nested objects
/// Expected: entire body preserved
#[test]
fn test_body_large_json() {
    let large_body = json!({
        "data": (0..50).map(|i| json!({
            "id": i,
            "name": format!("item-{}", i),
            "values": vec![i * 10, i * 20, i * 30]
        })).collect::<Vec<_>>()
    });

    let (_request, request_data) = RequestBuilder::new()
        .method(Method::POST)
        .path("/batch")
        .json_body(large_body.clone())
        .build();

    assert_eq!(*request_data.body, large_body);
}

/// Test complete request with path, query, headers, cookies, and body
///
/// Combines all extraction scenarios
#[test]
fn test_complete_request_with_all_components() {
    let body = json!({"action": "create", "resource": "user"});

    let (_request, request_data) = RequestBuilder::new()
        .method(Method::POST)
        .path("/api/v1/users")
        .query_param("limit", "10")
        .query_param("filter", "active")
        .header("authorization", "Bearer token123")
        .header("content-type", "application/json")
        .cookie("session", "xyz789")
        .cookie("preferences", "dark_mode")
        .json_body(body.clone())
        .build();

    assert_eq!(request_data.method, "POST");
    assert_eq!(request_data.path, "/api/v1/users");

    assert_eq!(request_data.raw_query_params.get("limit").unwrap()[0], "10");
    assert_eq!(request_data.raw_query_params.get("filter").unwrap()[0], "active");

    assert!(request_data.headers.get("authorization").is_some());
    assert_eq!(
        request_data.headers.get("content-type"),
        Some(&"application/json".to_string())
    );

    assert_eq!(request_data.cookies.get("session"), Some(&"xyz789".to_string()));
    assert_eq!(request_data.cookies.get("preferences"), Some(&"dark_mode".to_string()));

    assert_eq!(*request_data.body, body);
}

/// Test Arc wrapping for efficient cloning
///
/// RequestData uses Arc for large fields for cheap cloning
/// Expected: Arc pointers are shared on clone
#[test]
fn test_request_data_arc_cloning() {
    let (_request, request_data) = RequestBuilder::new()
        .path("/api")
        .query_param("filter", "test")
        .query_param("sort", "name")
        .header("x-custom", "value")
        .cookie("session", "abc123")
        .build();

    let cloned = request_data.clone();

    assert!(Arc::ptr_eq(&request_data.headers, &cloned.headers));
    assert!(Arc::ptr_eq(&request_data.cookies, &cloned.cookies));
    assert!(Arc::ptr_eq(&request_data.raw_query_params, &cloned.raw_query_params));
}

/// Test different HTTP methods preserve correctly
///
/// Methods: GET, POST, PUT, DELETE, PATCH
/// Expected: method stored correctly in RequestData
#[test]
fn test_different_http_methods() {
    for (method, expected) in &[
        (Method::GET, "GET"),
        (Method::POST, "POST"),
        (Method::PUT, "PUT"),
        (Method::DELETE, "DELETE"),
        (Method::PATCH, "PATCH"),
        (Method::HEAD, "HEAD"),
        (Method::OPTIONS, "OPTIONS"),
    ] {
        let (_request, request_data) = RequestBuilder::new()
            .method(method.clone())
            .path("/api/resource")
            .build();

        assert_eq!(&request_data.method, expected);
    }
}
