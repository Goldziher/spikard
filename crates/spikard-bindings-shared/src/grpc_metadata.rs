//! Shared gRPC metadata utilities
//!
//! This module provides common metadata conversion functions used across all
//! language bindings (Python, Node.js, Ruby, PHP) to avoid code duplication.

use std::collections::HashMap;
use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};

/// Extract metadata from gRPC `MetadataMap` to a simple `HashMap`.
///
/// This function converts gRPC metadata to a language-agnostic `HashMap` format
/// that can be easily passed to language bindings. Only ASCII metadata is
/// included; binary metadata is skipped with optional logging.
///
/// # Arguments
///
/// * `metadata` - The gRPC `MetadataMap` to extract from
/// * `log_binary_skip` - Whether to log when binary metadata is skipped
///
/// # Returns
///
/// A `HashMap` containing all ASCII metadata key-value pairs
///
/// # Examples
///
/// ```
/// use tonic::metadata::MetadataMap;
/// use spikard_bindings_shared::grpc_metadata::extract_metadata_to_hashmap;
///
/// let mut metadata = MetadataMap::new();
/// metadata.insert("authorization", "Bearer token123".parse().unwrap());
///
/// let map = extract_metadata_to_hashmap(&metadata, false);
/// assert_eq!(map.get("authorization"), Some(&"Bearer token123".to_string()));
/// ```
pub fn extract_metadata_to_hashmap(metadata: &MetadataMap, log_binary_skip: bool) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for key_value in metadata.iter() {
        match key_value {
            tonic::metadata::KeyAndValueRef::Ascii(key, value) => {
                let key_str = key.as_str().to_string();
                let value_str = value.to_str().unwrap_or("").to_string();
                map.insert(key_str, value_str);
            }
            tonic::metadata::KeyAndValueRef::Binary(key, _value) => {
                // Binary metadata is skipped as we only support string values
                if log_binary_skip {
                    tracing::debug!("Skipping binary metadata key: {}", key.as_str());
                }
            }
        }
    }

    map
}

/// Convert a `HashMap` to gRPC `MetadataMap`.
///
/// This function converts a language-agnostic `HashMap` into a gRPC `MetadataMap`
/// that can be used in responses. All keys and values are validated and errors
/// are returned if any are invalid.
///
/// # Arguments
///
/// * `map` - The `HashMap` to convert
///
/// # Returns
///
/// A Result containing the `MetadataMap` or an error message
///
/// # Errors
///
/// Returns an error if:
/// - A metadata key is invalid (contains invalid characters)
/// - A metadata value is invalid (contains invalid characters)
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use spikard_bindings_shared::grpc_metadata::hashmap_to_metadata;
///
/// let mut map = HashMap::new();
/// map.insert("content-type".to_string(), "application/grpc".to_string());
///
/// let metadata = hashmap_to_metadata(&map).unwrap();
/// assert!(metadata.contains_key("content-type"));
/// ```
pub fn hashmap_to_metadata<S: std::hash::BuildHasher>(
    map: &std::collections::HashMap<String, String, S>,
) -> Result<MetadataMap, String> {
    let mut metadata = MetadataMap::new();

    for (key, value) in map {
        let metadata_key =
            MetadataKey::from_bytes(key.as_bytes()).map_err(|err| format!("Invalid metadata key '{key}': {err}"))?;

        let metadata_value =
            MetadataValue::try_from(value).map_err(|err| format!("Invalid metadata value for '{key}': {err}"))?;

        metadata.insert(metadata_key, metadata_value);
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_empty_metadata() {
        let metadata = MetadataMap::new();
        let map = extract_metadata_to_hashmap(&metadata, false);
        assert!(map.is_empty());
    }

    #[test]
    fn test_extract_single_metadata() {
        let mut metadata = MetadataMap::new();
        metadata.insert("content-type", "application/grpc".parse().unwrap());

        let map = extract_metadata_to_hashmap(&metadata, false);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("content-type"), Some(&"application/grpc".to_string()));
    }

    #[test]
    fn test_extract_multiple_metadata() {
        let mut metadata = MetadataMap::new();
        metadata.insert("content-type", "application/grpc".parse().unwrap());
        metadata.insert("authorization", "Bearer token123".parse().unwrap());
        metadata.insert("x-custom-header", "custom-value".parse().unwrap());

        let map = extract_metadata_to_hashmap(&metadata, false);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("content-type"), Some(&"application/grpc".to_string()));
        assert_eq!(map.get("authorization"), Some(&"Bearer token123".to_string()));
        assert_eq!(map.get("x-custom-header"), Some(&"custom-value".to_string()));
    }

    #[test]
    fn test_hashmap_to_metadata_empty() {
        let map = HashMap::new();
        let metadata = hashmap_to_metadata(&map).unwrap();
        assert_eq!(metadata.len(), 0);
    }

    #[test]
    fn test_hashmap_to_metadata_single() {
        let mut map = HashMap::new();
        map.insert("content-type".to_string(), "application/grpc".to_string());

        let metadata = hashmap_to_metadata(&map).unwrap();
        assert_eq!(metadata.len(), 1);
        assert!(metadata.contains_key("content-type"));
    }

    #[test]
    fn test_hashmap_to_metadata_multiple() {
        let mut map = HashMap::new();
        map.insert("content-type".to_string(), "application/grpc".to_string());
        map.insert("authorization".to_string(), "Bearer token".to_string());

        let metadata = hashmap_to_metadata(&map).unwrap();
        assert_eq!(metadata.len(), 2);
        assert!(metadata.contains_key("content-type"));
        assert!(metadata.contains_key("authorization"));
    }

    #[test]
    fn test_hashmap_to_metadata_invalid_key() {
        let mut map = HashMap::new();
        map.insert("invalid\nkey".to_string(), "value".to_string());

        let result = hashmap_to_metadata(&map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid metadata key"));
    }

    #[test]
    fn test_roundtrip_metadata() {
        let mut original_metadata = MetadataMap::new();
        original_metadata.insert("content-type", "application/grpc".parse().unwrap());
        original_metadata.insert("x-custom", "value".parse().unwrap());

        // Extract to HashMap
        let map = extract_metadata_to_hashmap(&original_metadata, false);

        // Convert back to MetadataMap
        let new_metadata = hashmap_to_metadata(&map).unwrap();

        // Verify both have the same entries
        assert_eq!(new_metadata.len(), original_metadata.len());
        assert!(new_metadata.contains_key("content-type"));
        assert!(new_metadata.contains_key("x-custom"));
    }
}
