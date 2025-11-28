//! PHP streaming response implementation.
//!
//! Bridges PHP Generators to Rust async Streams for streaming HTTP responses.
//! Uses thread-local storage pattern for PHP Generator objects (non-Send/Sync).

use async_stream::stream;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use bytes::Bytes;
use ext_php_rs::types::Zval;
use spikard_http::HandlerResponse;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

/// Global registry for PHP Generator objects.
///
/// Since Zval contains raw pointers to PHP's single-threaded structures,
/// we use thread-local storage similar to PhpHandler registry.
thread_local! {
    static GENERATOR_REGISTRY: RefCell<Vec<GeneratorHandle>> = RefCell::new(Vec::new());
}

/// Handle to a PHP Generator stored in thread-local registry.
struct GeneratorHandle {
    /// The PHP Generator Zval (must stay on PHP thread)
    generator: Zval,
    /// Whether the generator has been exhausted
    exhausted: bool,
}

/// Streaming response configuration from PHP.
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            status_code: 200,
            headers: HashMap::new(),
        }
    }
}

/// Register a PHP Generator and return its registry index.
///
/// # Parameters
/// * `generator_zval` - PHP Generator object (must have `valid()` and `current()` methods)
/// * `config` - Optional streaming configuration
///
/// # Returns
/// Registry index for later streaming, or error if not a valid generator
pub fn register_generator(
    generator_zval: &Zval,
    config: Option<StreamingConfig>,
) -> Result<(usize, StreamingConfig), String> {
    // Verify it's a Generator object
    if let Some(obj) = generator_zval.object() {
        if let Ok(class_name) = obj.get_class_name() {
            if !class_name.contains("Generator") {
                return Err(format!("Expected Generator, got {}", class_name));
            }
        } else {
            return Err("Cannot get class name from generator".to_string());
        }
    } else {
        return Err("StreamingResponse requires a Generator object".to_string());
    }

    let idx = GENERATOR_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let idx = registry.len();

        let handle = GeneratorHandle {
            generator: generator_zval.shallow_clone(),
            exhausted: false,
        };

        registry.push(handle);
        idx
    });

    let final_config = config.unwrap_or_default();
    Ok((idx, final_config))
}

/// Convert a registered PHP Generator to a Rust async Stream.
///
/// This creates a Stream<Item = Result<Bytes, BoxError>> that polls the PHP
/// Generator on each chunk using spawn_blocking.
pub fn generator_to_stream(
    generator_idx: usize,
) -> impl futures::Stream<Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> {
    stream! {
        loop {
            // Poll the generator on the PHP thread
            let result = tokio::task::spawn_blocking(move || {
                poll_generator(generator_idx)
            }).await;

            match result {
                Ok(Ok(Some(bytes))) => {
                    yield Ok(bytes);
                }
                Ok(Ok(None)) => {
                    // Generator exhausted
                    break;
                }
                Ok(Err(err)) => {
                    yield Err(Box::new(io::Error::other(err)) as Box<dyn std::error::Error + Send + Sync>);
                    break;
                }
                Err(err) => {
                    yield Err(Box::new(io::Error::other(format!("Task error: {}", err))) as Box<dyn std::error::Error + Send + Sync>);
                    break;
                }
            }
        }
    }
}

/// Poll the PHP Generator for the next chunk.
///
/// This runs on the PHP thread (via spawn_blocking) and calls:
/// 1. `$generator->valid()` to check if more data available
/// 2. `$generator->current()` to get the chunk
/// 3. `$generator->next()` to advance to next chunk
fn poll_generator(generator_idx: usize) -> Result<Option<Bytes>, String> {
    GENERATOR_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let handle = registry
            .get_mut(generator_idx)
            .ok_or_else(|| format!("Generator {} not found in registry", generator_idx))?;

        if handle.exhausted {
            return Ok(None);
        }

        let obj = handle
            .generator
            .object()
            .ok_or_else(|| "Generator is not an object".to_string())?;

        // Check if generator has more values: $generator->valid()
        let valid_result = obj
            .try_call_method("valid", vec![])
            .map_err(|e| format!("Failed to call valid(): {:?}", e))?;

        let is_valid = valid_result
            .bool()
            .ok_or_else(|| "valid() did not return boolean".to_string())?;

        if !is_valid {
            handle.exhausted = true;
            return Ok(None);
        }

        // Get current value: $generator->current()
        let current_result = obj
            .try_call_method("current", vec![])
            .map_err(|e| format!("Failed to call current(): {:?}", e))?;

        // Convert chunk to bytes
        let bytes = convert_chunk_to_bytes(&current_result)?;

        // Advance generator: $generator->next()
        obj.try_call_method("next", vec![])
            .map_err(|e| format!("Failed to call next(): {:?}", e))?;

        Ok(Some(bytes))
    })
}

/// Convert a PHP Zval chunk to Bytes.
///
/// Supports:
/// - String chunks (UTF-8 encoded)
/// - Array/object chunks (JSON encoded)
fn convert_chunk_to_bytes(chunk: &Zval) -> Result<Bytes, String> {
    // Try string first
    if let Some(s) = chunk.string() {
        return Ok(Bytes::from(s.to_string()));
    }

    // Try array or object (serialize to JSON)
    if chunk.is_array() || chunk.is_object() {
        let json_val = crate::php::zval_to_json(chunk)
            .map_err(|e| format!("Failed to convert chunk to JSON: {}", e))?;
        let json_str = serde_json::to_string(&json_val)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        return Ok(Bytes::from(json_str));
    }

    Err("StreamingResponse chunks must be strings or JSON-serializable values".to_string())
}

/// Create a HandlerResponse from a registered generator.
///
/// This is called from `interpret_php_response` when it detects a
/// StreamingResponse object.
pub fn create_handler_response(
    generator_idx: usize,
    config: StreamingConfig,
) -> Result<HandlerResponse, String> {
    let status = StatusCode::from_u16(config.status_code)
        .map_err(|e| format!("Invalid status code {}: {}", config.status_code, e))?;

    let header_pairs: Vec<(HeaderName, HeaderValue)> = config
        .headers
        .into_iter()
        .map(|(name, value)| {
            let header_name = HeaderName::from_str(&name)
                .map_err(|e| format!("Invalid header '{}': {}", name, e))?;
            let header_value = HeaderValue::from_str(&value)
                .map_err(|e| format!("Invalid header value '{}': {}", value, e))?;
            Ok((header_name, header_value))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let stream = generator_to_stream(generator_idx);

    let mut response = HandlerResponse::stream(stream).with_status(status);
    for (name, value) in header_pairs {
        response = response.with_header(name, value);
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_config_default() {
        let config = StreamingConfig::default();
        assert_eq!(config.status_code, 200);
        assert_eq!(config.headers.len(), 0);
    }

    #[test]
    fn test_convert_string_chunk() {
        let zval = Zval::from("test chunk");
        let bytes = convert_chunk_to_bytes(&zval).unwrap();
        assert_eq!(bytes, Bytes::from("test chunk"));
    }
}
