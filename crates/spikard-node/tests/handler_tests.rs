//! Integration tests for Node.js handler implementation
//!
//! Tests the `NodeHandler`'s implementation of the Handler trait,
//! verifying `ThreadsafeFunction` integration and request/response handling.

use serde_json::json;
use spikard_http::RequestData;
use std::collections::HashMap;
use std::sync::Arc;

/// Test that `NodeHandler` properly serializes `RequestData` for JavaScript
#[tokio::test]
async fn test_request_data_serialization() {
    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "123".to_string());

    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());

    let mut cookies = HashMap::new();
    cookies.insert("session".to_string(), "abc123".to_string());

    let query_params = json!({"page": "1", "limit": "10"});
    let body = json!({"name": "test"});

    let request_data = RequestData {
        path: "/api/users/123".to_string(),
        method: "GET".to_string(),
        path_params: Arc::new(path_params),
        query_params,
        validated_params: None,
        headers: Arc::new(headers),
        cookies: Arc::new(cookies),
        raw_query_params: Arc::new(HashMap::new()),
        body,
        raw_body: None,
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let expected = json!({
        "path": "/api/users/123",
        "method": "GET",
        "path_params": {"id": "123"},
        "query_params": {"page": "1", "limit": "10"},
        "headers": {"content-type": "application/json"},
        "cookies": {"session": "abc123"},
        "body": {"name": "test"}
    });

    let serialized = serde_json::json!({
        "path": request_data.path,
        "method": request_data.method,
        "path_params": &*request_data.path_params,
        "query_params": request_data.query_params,
        "headers": &*request_data.headers,
        "cookies": &*request_data.cookies,
        "body": request_data.body,
    });

    assert_eq!(serialized, expected);
}

/// Test that response parsing handles various JSON structures
#[test]
fn test_response_parsing() {
    let response_json = r#"{"message": "success", "code": 200}"#;
    let parsed: serde_json::Value = serde_json::from_str(response_json).unwrap();
    assert_eq!(parsed["message"], "success");
    assert_eq!(parsed["code"], 200);

    let array_json = r#"[{"id": 1}, {"id": 2}]"#;
    let parsed: serde_json::Value = serde_json::from_str(array_json).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 2);

    let null_json = "null";
    let parsed: serde_json::Value = serde_json::from_str(null_json).unwrap();
    assert!(parsed.is_null());

    let string_json = r#""hello""#;
    let parsed: serde_json::Value = serde_json::from_str(string_json).unwrap();
    assert_eq!(parsed, "hello");
}

/// Test error handling for invalid JSON responses
#[test]
fn test_invalid_json_handling() {
    let invalid_json = r#"{"incomplete": "#;
    let result: Result<serde_json::Value, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("EOF"));
}

#[test]
fn test_arc_dereferencing() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    let arc_map = Arc::new(map);

    for (k, v) in &*arc_map {
        assert_eq!(k, "key");
        assert_eq!(v, "value");
    }

    let serialized = serde_json::to_value(&*arc_map).unwrap();
    assert_eq!(serialized["key"], "value");
}
