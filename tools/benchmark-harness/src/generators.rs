// /// Test data generators for different workload types.
//!
//! Generates realistic test payloads for benchmarking across different payload sizes and
//! structures.

use serde_json::{Map, Value, json};
use std::collections::HashMap;

/// Generate a small JSON payload (~100-500 bytes)
pub fn generate_json_small() -> Value {
    json!({
        "id": 12345,
        "name": "test_item",
        "active": true,
        "count": 42,
        "tags": ["tag1", "tag2", "tag3"]
    })
}

/// Generate a medium JSON payload (~1-10KB)
pub fn generate_json_medium() -> Value {
    let mut metadata = Map::new();
    for i in 0..10 {
        metadata.insert(format!("key_{}", i), json!(format!("value_{}", i)));
    }

    let items: Vec<Value> = (0..5).map(|_| generate_json_small()).collect();

    json!({
        "id": 67890,
        "metadata": metadata,
        "items": items,
        "description": "A".repeat(500),
        "nested": {
            "field1": "value1",
            "field2": 123,
            "field3": true
        }
    })
}

/// Generate a large JSON payload (~10-100KB)
pub fn generate_json_large() -> Value {
    let items: Vec<Value> = (0..200).map(|_| generate_json_small()).collect();
    // Add deterministic padding to keep payload size comfortably above 10KB while
    // staying well under the 100KB upper bound used in tests.
    let padding = "X".repeat(12_000);

    json!({
        "data": items,
        "total": 100,
        "page": 1,
        "metadata": {
            "timestamp": "2024-01-01T00:00:00Z",
            "version": "1.0.0"
        },
        "payload": padding
    })
}

/// Generate a very large JSON payload (~100KB-1MB)
pub fn generate_json_very_large() -> Value {
    let items: Vec<Value> = (0..2000).map(|_| generate_json_small()).collect();
    // Add a larger deterministic padding block so the serialized payload reliably
    // exceeds 100KB without approaching the 1MB ceiling.
    let padding = "Y".repeat(50_000);

    json!({
        "data": items,
        "total": 1000,
        "page": 1,
        "metadata": {
            "timestamp": "2024-01-01T00:00:00Z",
            "version": "1.0.0",
            "description": "Very large dataset for performance testing".repeat(10)
        },
        "payload": padding
    })
}

/// Generate a simple URL-encoded form (3-5 fields)
pub fn generate_urlencoded_simple() -> HashMap<String, String> {
    let mut form = HashMap::new();
    form.insert("username".to_string(), "testuser".to_string());
    form.insert("email".to_string(), "test@example.com".to_string());
    form.insert("age".to_string(), "30".to_string());
    form
}

/// Generate a complex URL-encoded form (10-20 fields)
pub fn generate_urlencoded_complex() -> HashMap<String, String> {
    let mut form = HashMap::new();
    for i in 0..15 {
        form.insert(format!("field_{}", i), format!("value_{}", i));
    }
    form
}

/// Generate random binary data of specified size
pub fn generate_multipart_file(size_bytes: usize) -> Vec<u8> {
    (0..size_bytes).map(|i| (i % 256) as u8).collect()
}

/// Generate random string of specified length
pub fn generate_random_string(length: usize) -> String {
    "x".repeat(length)
}

/// Convert HashMap to URL-encoded string
pub fn hashmap_to_urlencoded(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_json_small() {
        let json = generate_json_small();
        let s = serde_json::to_string(&json).unwrap();
        assert!(s.len() >= 50 && s.len() <= 500);
    }

    #[test]
    fn test_generate_json_medium() {
        let json = generate_json_medium();
        let s = serde_json::to_string(&json).unwrap();
        assert!(s.len() >= 1000 && s.len() <= 10_000);
    }

    #[test]
    fn test_generate_json_large() {
        let json = generate_json_large();
        let s = serde_json::to_string(&json).unwrap();
        assert!(s.len() >= 10_000 && s.len() <= 100_000);
    }

    #[test]
    fn test_generate_json_very_large() {
        let json = generate_json_very_large();
        let s = serde_json::to_string(&json).unwrap();
        assert!(s.len() >= 100_000);
    }

    #[test]
    fn test_generate_urlencoded_simple() {
        let form = generate_urlencoded_simple();
        assert_eq!(form.len(), 3);
    }

    #[test]
    fn test_generate_urlencoded_complex() {
        let form = generate_urlencoded_complex();
        assert_eq!(form.len(), 15);
    }

    #[test]
    fn test_generate_multipart_file() {
        let file = generate_multipart_file(1024);
        assert_eq!(file.len(), 1024);
    }

    #[test]
    fn test_generate_random_string() {
        let s = generate_random_string(100);
        assert_eq!(s.len(), 100);
    }

    #[test]
    fn test_hashmap_to_urlencoded() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value 2".to_string());
        let encoded = hashmap_to_urlencoded(&map);
        assert!(encoded.contains("key1=value1"));
        assert!(encoded.contains("key2=value%202"));
    }
}
