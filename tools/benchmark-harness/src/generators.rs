//!
//! Generates realistic test payloads for benchmarking across different payload sizes and
//! structures.

use serde_json::{Value, json};
use std::collections::HashMap;

/// Generate a small JSON payload (~100-500 bytes)
#[must_use]
pub fn generate_json_small() -> Value {
    json!({
        "name": "benchmark_item_small",
        "description": "Small payload for benchmarking with enough entropy to avoid trivial compression.",
        "price": 19.99,
        "tax": 1.52
    })
}

/// Generate a medium JSON payload (~1-10KB)
#[must_use]
pub fn generate_json_medium() -> Value {
    let image_name = "benchmark_image_".repeat(40);
    let image_url = format!("https://cdn.example.com/{}", "image-path/".repeat(30));
    json!({
        "name": "benchmark_item_medium",
        "price": 129.99,
        "image": {
            "url": image_url,
            "name": image_name
        }
    })
}

/// Generate a large JSON payload (~10-100KB)
#[must_use]
pub fn generate_json_large() -> Value {
    json!({
        "name": "benchmark_item_large_".repeat(40),
        "price": 1249.50,
        "seller": {
            "name": "benchmark_seller_".repeat(60),
            "address": {
                "street": "123 Benchmark Avenue ".repeat(400),
                "city": "Performance City ".repeat(120),
                "country": {
                    "name": "Benchmark Republic ".repeat(80),
                    "code": "BR"
                }
            }
        }
    })
}

/// Generate a very large JSON payload (~100KB-1MB)
#[must_use]
pub fn generate_json_very_large() -> Value {
    let tags: Vec<Value> = (0..500).map(|idx| json!(format!("tag_{idx:04}"))).collect();
    let images: Vec<Value> = (0..800)
        .map(|idx| {
            json!({
                "url": format!("https://images.example.com/{}/{}", "path".repeat(10), idx),
                "name": format!("benchmark_image_{}{}", idx, "_x".repeat(20))
            })
        })
        .collect();

    json!({
        "name": "benchmark_item_very_large_".repeat(80),
        "tags": tags,
        "images": images
    })
}

/// Generate a simple URL-encoded form (3-5 fields)
#[must_use]
pub fn generate_urlencoded_simple() -> HashMap<String, String> {
    let mut form = HashMap::new();
    form.insert("name".to_string(), "Test User".to_string());
    form.insert("email".to_string(), "test.user@example.com".to_string());
    form.insert("age".to_string(), "30".to_string());
    form.insert("subscribe".to_string(), "true".to_string());
    form
}

/// Generate a complex URL-encoded form (10-20 fields)
#[must_use]
pub fn generate_urlencoded_complex() -> HashMap<String, String> {
    let mut form = HashMap::new();
    form.insert("username".to_string(), "benchmark_user".to_string());
    form.insert("password".to_string(), "s3cureP@ssw0rd".to_string());
    form.insert("email".to_string(), "benchmark.user@example.com".to_string());
    form.insert("first_name".to_string(), "Benchmark".to_string());
    form.insert("last_name".to_string(), "User".to_string());
    form.insert("age".to_string(), "42".to_string());
    form.insert("country".to_string(), "USA".to_string());
    form.insert("state".to_string(), "CA".to_string());
    form.insert("city".to_string(), "Performance City".to_string());
    form.insert("zip".to_string(), "94105-0001".to_string());
    form.insert("phone".to_string(), "+14155550199".to_string());
    form.insert("company".to_string(), "Benchmark Labs".to_string());
    form.insert("job_title".to_string(), "Performance Engineer".to_string());
    form.insert("subscribe".to_string(), "true".to_string());
    form.insert("newsletter".to_string(), "false".to_string());
    form.insert("terms_accepted".to_string(), "true".to_string());
    form.insert("privacy_accepted".to_string(), "true".to_string());
    form.insert("marketing_consent".to_string(), "false".to_string());
    form.insert("two_factor_enabled".to_string(), "true".to_string());
    form
}

/// Generate random binary data of specified size
#[must_use]
pub fn generate_multipart_file(size_bytes: usize) -> Vec<u8> {
    (0..size_bytes).map(|i| (i % 256) as u8).collect()
}

/// Generate random string of specified length
#[must_use]
pub fn generate_random_string(length: usize) -> String {
    "x".repeat(length)
}

/// Convert `HashMap` to URL-encoded string
#[must_use]
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
        assert_eq!(form.len(), 4);
    }

    #[test]
    fn test_generate_urlencoded_complex() {
        let form = generate_urlencoded_complex();
        assert_eq!(form.len(), 19);
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
