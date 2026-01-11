//! Example of implementing `ConfigSource` for a language binding
//!
//! This example demonstrates how a language binding (e.g., Python, Node.js, Ruby, PHP)
//! would implement the `ConfigSource` trait to extract `ServerConfig` from language-specific objects.

use spikard_bindings_shared::{ConfigExtractor, ConfigSource};
use std::collections::HashMap;

/// Example: `PyO3` Python dict wrapper
struct PyDictWrapper {
    data: HashMap<String, String>,
}

impl PyDictWrapper {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

impl ConfigSource for PyDictWrapper {
    fn get_bool(&self, key: &str) -> Option<bool> {
        self.data.get(key).and_then(|v| match v.as_str() {
            "true" | "True" => Some(true),
            "false" | "False" => Some(false),
            _ => v.parse().ok(),
        })
    }

    fn get_u64(&self, key: &str) -> Option<u64> {
        self.data.get(key).and_then(|v| v.parse().ok())
    }

    fn get_u16(&self, key: &str) -> Option<u16> {
        self.data.get(key).and_then(|v| v.parse().ok())
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn get_vec_string(&self, key: &str) -> Option<Vec<String>> {
        self.data
            .get(key)
            .map(|s| s.split(',').map(|item| item.trim().to_string()).collect())
    }

    fn get_nested(&self, _key: &str) -> Option<Box<dyn ConfigSource + '_>> {
        None
    }

    fn has_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

fn main() {
    println!("=== ConfigExtractor Example ===\n");

    println!("1. Extracting compression configuration:");
    let mut compression_config = PyDictWrapper::new();
    compression_config.insert("gzip", "true");
    compression_config.insert("brotli", "true");
    compression_config.insert("min_size", "2048");
    compression_config.insert("quality", "9");

    match ConfigExtractor::extract_compression_config(&compression_config) {
        Ok(config) => {
            println!("   gzip: {}", config.gzip);
            println!("   brotli: {}", config.brotli);
            println!("   min_size: {}", config.min_size);
            println!("   quality: {}\n", config.quality);
        }
        Err(e) => println!("   Error: {e}\n"),
    }

    println!("2. Extracting JWT authentication configuration:");
    let mut jwt_config = PyDictWrapper::new();
    jwt_config.insert("secret", "my-secret-key");
    jwt_config.insert("algorithm", "HS256");
    jwt_config.insert("leeway", "60");

    match ConfigExtractor::extract_jwt_config(&jwt_config) {
        Ok(config) => {
            println!("   secret: [REDACTED]");
            println!("   algorithm: {}", config.algorithm);
            println!("   leeway: {}\n", config.leeway);
        }
        Err(e) => println!("   Error: {e}\n"),
    }

    println!("3. Extracting API Key authentication configuration:");
    let mut api_key_config = PyDictWrapper::new();
    api_key_config.insert("keys", "key1,key2,key3");
    api_key_config.insert("header_name", "X-API-Key");

    match ConfigExtractor::extract_api_key_config(&api_key_config) {
        Ok(config) => {
            println!("   keys: {:?}", config.keys);
            println!("   header_name: {}\n", config.header_name);
        }
        Err(e) => println!("   Error: {e}\n"),
    }

    println!("4. Extracting rate limit configuration:");
    let mut rate_limit_config = PyDictWrapper::new();
    rate_limit_config.insert("per_second", "100");
    rate_limit_config.insert("burst", "20");
    rate_limit_config.insert("ip_based", "true");

    match ConfigExtractor::extract_rate_limit_config(&rate_limit_config) {
        Ok(config) => {
            println!("   per_second: {}", config.per_second);
            println!("   burst: {}", config.burst);
            println!("   ip_based: {}\n", config.ip_based);
        }
        Err(e) => println!("   Error: {e}\n"),
    }

    println!("5. Testing error handling (missing 'burst' field):");
    let rate_limit_config = PyDictWrapper::new();

    match ConfigExtractor::extract_rate_limit_config(&rate_limit_config) {
        Ok(_config) => println!("   Success (unexpected!)"),
        Err(e) => println!("   Expected error: {e}\n"),
    }

    println!("=== Example Complete ===");
}
