//! PHP SSE producer bindings
//!
//! This module provides the FFI bridge between PHP Generator-based SSE producers
//! and the Rust spikard-http SseEventProducer trait.
//!
//! # Design Principles
//!
//! PHP Generators are synchronous, unlike Python's async generators. The bridge:
//! 1. Accepts a PHP callable that returns a Generator<int, string, mixed, void>
//! 2. Converts the Generator into an implementation of SseEventProducer
//! 3. Handles the Generator protocol: valid() → current() → next() iteration
//! 4. Converts yielded JSON strings to serde_json::Value for SseEvent creation
//! 5. Runs synchronously on the PHP thread to avoid cross-thread PHP calls
//!
//! # PHP Generator Protocol
//!
//! PHP Generators implement the Iterator interface with methods:
//! - `valid()`: Returns true if current position is valid
//! - `current()`: Returns the current yielded value
//! - `next()`: Advances to the next yield point
//! - `rewind()`: Resets to beginning (optional, not used here)
//!
//! # Error Handling
//!
//! All PHP-to-Rust boundary crossings use PhpResult for proper error propagation.
//! Errors during event generation log and return None to gracefully end the stream.

use ext_php_rs::types::ZendCallable;
use serde_json::Value;
use spikard_http::{SseEvent, SseEventProducer};
use tracing::{debug, error};

/// PHP implementation of SseEventProducer
///
/// Wraps a PHP callable that returns a Generator yielding JSON event strings.
/// The callable is stored by index in a global registry since ZendCallable
/// is not Send/Sync.
pub struct PhpSseEventProducer {
    /// Index into PHP_SSE_PRODUCER_REGISTRY for the generator factory callable
    producer_index: usize,
}

#[derive(Debug)]
struct ProducerState {
    callable: ext_php_rs::types::Zval,
    generator: Option<ext_php_rs::types::Zval>,
    done: bool,
}

// NOTE: thread_local because Zval is not Send/Sync (PHP is single-threaded).
thread_local! {
    static PHP_SSE_PRODUCER_REGISTRY: std::cell::RefCell<Vec<ProducerState>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub fn leak_sse_producer_registry() {
    PHP_SSE_PRODUCER_REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let producers = std::mem::take(&mut *registry);
        std::mem::forget(producers);
    });
}

impl PhpSseEventProducer {
    /// Create a new PHP SSE event producer from a callable
    ///
    /// The callable should return a PHP Generator that yields JSON strings
    /// representing SSE events. Each yielded value should be a JSON string
    /// with the following optional fields:
    /// - `data`: The event data (required)
    /// - `event_type`: Optional event type string
    /// - `id`: Optional event ID for client reconnection
    /// - `retry`: Optional retry timeout in milliseconds
    ///
    /// # Arguments
    /// * `callable` - PHP callable that returns Generator<int, string, mixed, void>
    ///
    /// # Returns
    /// A new PhpSseEventProducer that can be used with SseState
    ///
    /// # Example PHP
    ///
    /// ```php
    /// $producer = function(): Generator {
    ///     for ($i = 0; $i < 10; $i++) {
    ///         yield json_encode(['data' => ['count' => $i]]);
    ///     }
    /// };
    /// ```
    /// Create from a Zval containing a callable
    ///
    /// This version takes a Zval directly, allowing us to clone it before
    /// it gets wrapped in ZendCallable with a lifetime constraint.
    pub fn new_from_zval(callable_zval: &ext_php_rs::types::Zval) -> Result<Self, String> {
        if !callable_zval.is_callable() {
            return Err("SSE producer must be callable".to_string());
        }

        let idx = PHP_SSE_PRODUCER_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let idx = registry.len();

            let zval_copy = callable_zval.shallow_clone();
            registry.push(ProducerState {
                callable: zval_copy,
                generator: None,
                done: false,
            });
            idx
        });

        Ok(Self { producer_index: idx })
    }

    /// Convert PHP Generator yielded value (JSON string) to SseEvent
    ///
    /// Parses the JSON string and extracts SSE event fields:
    /// - `data`: Event data payload (required)
    /// - `event_type`: Optional event type
    /// - `id`: Optional event ID
    /// - `retry`: Optional retry timeout in milliseconds
    ///
    /// # Arguments
    /// * `json_str` - JSON string from PHP Generator::current()
    ///
    /// # Returns
    /// * `Ok(SseEvent)` - Successfully parsed event
    /// * `Err(String)` - Parse error with message
    fn parse_event_json(json_str: &str) -> Result<SseEvent, String> {
        let value: Value =
            serde_json::from_str(json_str).map_err(|e| format!("Failed to parse SSE event JSON: {}", e))?;

        let data = value
            .get("data")
            .ok_or_else(|| "SSE event missing required 'data' field".to_string())?
            .clone();

        let event_type = value.get("event_type").and_then(|v| v.as_str()).map(|s| s.to_string());

        let id = value.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());

        let retry = value.get("retry").and_then(|v| v.as_u64());

        let mut event = if let Some(et) = event_type {
            SseEvent::with_type(et, data)
        } else {
            SseEvent::new(data)
        };

        if let Some(id_str) = id {
            event = event.with_id(id_str);
        }

        if let Some(retry_ms) = retry {
            event = event.with_retry(retry_ms);
        }

        Ok(event)
    }

    /// Invoke the PHP generator factory and iterate to next event
    ///
    /// This is called synchronously on the PHP thread to avoid cross-thread PHP calls.
    /// It retrieves the generator callable from the registry, invokes it,
    /// and iterates through the Generator protocol to get the next event.
    ///
    /// # Returns
    /// * `Some(SseEvent)` - Successfully retrieved next event
    /// * `None` - Stream ended or error occurred
    fn get_next_event_sync(&self) -> Option<SseEvent> {
        PHP_SSE_PRODUCER_REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let state = registry.get_mut(self.producer_index)?;

            if state.done {
                return None;
            }

            if state.generator.is_none() {
                let callable = match ZendCallable::new(&state.callable) {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to reconstruct PHP SSE producer callable: {:?}", e);
                        return None;
                    }
                };

                let generator = match callable.try_call(vec![]) {
                    Ok(generator) => generator,
                    Err(e) => {
                        error!("Failed to invoke PHP SSE producer: {:?}", e);
                        return None;
                    }
                };

                if generator.object().is_none() {
                    error!("PHP SSE producer did not return a Generator object");
                    return None;
                }

                if let Err(e) = generator.try_call_method("rewind", vec![]) {
                    error!("Failed to call rewind() on PHP Generator: {:?}", e);
                    return None;
                }

                state.generator = Some(generator);
            }

            let generator = state.generator.as_ref()?;

            let is_valid = match generator.try_call_method("valid", vec![]) {
                Ok(valid_zval) => valid_zval.bool().unwrap_or(false),
                Err(e) => {
                    error!("Failed to call valid() on PHP Generator: {:?}", e);
                    return None;
                }
            };

            if !is_valid {
                debug!("PHP SSE Generator exhausted (valid() returned false)");
                state.generator = None;
                state.done = true;
                return None;
            }

            let current_value = match generator.try_call_method("current", vec![]) {
                Ok(val) => val,
                Err(e) => {
                    error!("Failed to call current() on PHP Generator: {:?}", e);
                    return None;
                }
            };

            if let Err(e) = generator.try_call_method("next", vec![]) {
                error!("Failed to call next() on PHP Generator: {:?}", e);
            }

            let is_valid_after = generator
                .try_call_method("valid", vec![])
                .ok()
                .and_then(|val| val.bool())
                .unwrap_or(false);

            if !is_valid_after {
                state.generator = None;
                state.done = true;
            }

            let json_str = match current_value.string() {
                Some(s) => s.to_string(),
                None => {
                    if let Ok(json_value) = crate::php::zval_to_json(&current_value) {
                        debug!("PHP SSE producer: using raw JSON event data");
                        return Some(SseEvent::new(json_value));
                    }
                    error!("Generator yielded non-string value");
                    return None;
                }
            };

            match Self::parse_event_json(&json_str) {
                Ok(event) => {
                    debug!("PHP SSE producer: parsed event successfully");
                    Some(event)
                }
                Err(e) => {
                    if let Ok(value) = serde_json::from_str::<Value>(&json_str) {
                        debug!("PHP SSE producer: using raw JSON event data");
                        return Some(SseEvent::new(value));
                    }
                    error!("PHP SSE event parse error: {}", e);
                    None
                }
            }
        })
    }
}

impl SseEventProducer for PhpSseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        debug!("PHP SSE producer: next_event called");

        let producer_index = self.producer_index;
        let producer = PhpSseEventProducer { producer_index };
        producer.get_next_event_sync()
    }

    async fn on_connect(&self) {
        debug!("PHP SSE producer: on_connect called");
    }

    async fn on_disconnect(&self) {
        debug!("PHP SSE producer: on_disconnect called");
    }
}

/// Create SseState from PHP producer callable
///
/// This is the main entry point for PHP code to create an SSE endpoint.
/// Pass a callable that returns a Generator yielding JSON event strings.
///
/// # Arguments
/// * `callable` - PHP callable returning Generator<int, string, mixed, void>
///
/// # Returns
/// SseState that can be registered with the HTTP server
///
/// # Example
///
/// ```php
/// use Spikard\Handlers\SseEventProducerInterface;
///
/// $producer = new class implements SseEventProducerInterface {
///     public function __invoke(): Generator {
///         for ($i = 0; $i < 100; $i++) {
///             yield json_encode([
///                 'data' => ['count' => $i, 'timestamp' => time()],
///                 'id' => "event-{$i}",
///             ]);
///             usleep(100000); // 100ms delay
///         }
///     }
/// };
///
/// $state = create_sse_state($producer);
/// ```
/// Create SSE state from a Zval containing a callable
///
/// Changed to accept `&ext_php_rs::types::Zval` instead of `ZendCallable`
/// to avoid lifetime issues.
pub fn create_sse_state(
    callable_zval: &ext_php_rs::types::Zval,
) -> Result<spikard_http::SseState<PhpSseEventProducer>, String> {
    let producer = PhpSseEventProducer::new_from_zval(callable_zval)?;
    Ok(spikard_http::SseState::new(producer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_event_json_minimal() {
        let json = r#"{"data": {"count": 42}}"#;
        let event = PhpSseEventProducer::parse_event_json(json).expect("valid event");
        assert!(event.event_type.is_none());
        assert!(event.id.is_none());
        assert!(event.retry.is_none());
        assert_eq!(event.data["count"], 42);
    }

    #[test]
    fn test_parse_event_json_full() {
        let json = r#"{
            "data": {"message": "hello"},
            "event_type": "notification",
            "id": "event-123",
            "retry": 5000
        }"#;
        let event = PhpSseEventProducer::parse_event_json(json).expect("valid event");
        assert_eq!(event.event_type, Some("notification".to_string()));
        assert_eq!(event.id, Some("event-123".to_string()));
        assert_eq!(event.retry, Some(5000));
        assert_eq!(event.data["message"], "hello");
    }

    #[test]
    fn test_parse_event_json_missing_data() {
        let json = r#"{"event_type": "test"}"#;
        let result = PhpSseEventProducer::parse_event_json(json);
        assert!(result.is_err());
        assert!(result.expect_err("should be error").contains("missing required 'data'"));
    }

    #[test]
    fn test_parse_event_json_invalid() {
        let json = "not valid json";
        let result = PhpSseEventProducer::parse_event_json(json);
        assert!(result.is_err());
        assert!(result.expect_err("should be error").contains("Failed to parse"));
    }
}
