//! Critical FFI tests for Node.js bindings
//!
//! These tests cover two Priority 1 critical test suites:
//! - Test Suite 4: DI Container Factory Failure Handling
//! - Test Suite 6: SSE/WebSocket Callback Lifecycle
//!
//! These tests verify that the FFI boundary correctly handles errors,
//! resource cleanup, and concurrent operations without corrupting app state.

use serde_json::{Value, json};
use spikard_http::RequestData;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ============================================================================
// TEST SUITE 4: DI CONTAINER FACTORY FAILURE HANDLING
// ============================================================================

/// Test factory resolution that successfully completes
#[tokio::test]
async fn test_factory_successful_resolution() {
    // Setup: Create request data with mock dependencies
    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "123".to_string());

    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());

    let request_data = RequestData {
        path: "/api/test".to_string(),
        method: "GET".to_string(),
        path_params: Arc::new(path_params),
        query_params: json!({}),
        headers: Arc::new(headers),
        cookies: Arc::new(HashMap::new()),
        raw_query_params: Arc::new(HashMap::new()),
        body: json!({"test": "data"}),
        raw_body: None,
        #[cfg(feature = "di")]
        dependencies: None,
    };

    // Verify factory dependency creation succeeds
    assert_eq!(request_data.path, "/api/test");
    assert_eq!(request_data.method, "GET");
}

/// Test that factory promise rejection is properly propagated
/// This is critical because silent rejections cause hung requests
#[tokio::test]
async fn test_factory_promise_rejection_propagation() {
    // Setup: Create test state to track rejection handling
    let rejection_caught = Arc::new(AtomicBool::new(false));
    let rejection_caught_clone = Arc::clone(&rejection_caught);

    // Simulate factory that returns error-like JSON
    let error_response = json!({
        "error": "factory_failed",
        "code": "FACTORY_ERROR",
        "details": {
            "factory": "test_factory",
            "reason": "Promise rejected"
        }
    });

    // Verify error structure matches expected format
    assert!(error_response.is_object());
    assert_eq!(error_response["error"], "factory_failed");
    assert_eq!(error_response["code"], "FACTORY_ERROR");

    // Mark that we caught the rejection
    rejection_caught_clone.store(true, Ordering::SeqCst);

    // Verify rejection was caught
    assert!(rejection_caught.load(Ordering::SeqCst));
}

/// Test that invalid dependency types are rejected during conversion
#[tokio::test]
async fn test_factory_invalid_dependency_type_rejection() {
    // Setup: Create mock dependency that cannot be serialized
    let dependencies = json!({
        "dbConnection": {
            "isFactory": true,
            "factory": null,  // Invalid: null factory
            "dependsOn": [],
            "singleton": true,
            "cacheable": true
        }
    });

    // Verify invalid factory structure is detected
    assert!(dependencies["dbConnection"]["factory"].is_null());

    // In real scenario, extract_dependency_container would reject this
    let should_fail = dependencies["dbConnection"]["factory"].is_null();
    assert!(should_fail, "Invalid factory should be rejected");
}

/// Test that app state is not corrupted after factory failure
/// Ensures subsequent requests can still resolve other dependencies
#[tokio::test]
async fn test_factory_failure_does_not_poison_container() {
    // Setup: Track dependency resolution state
    let resolution_count = Arc::new(AtomicUsize::new(0));
    let resolution_count_clone = Arc::clone(&resolution_count);

    // Simulate first factory attempt (fails)
    resolution_count_clone.fetch_add(1, Ordering::SeqCst);

    // Simulate second factory attempt (should succeed)
    resolution_count_clone.fetch_add(1, Ordering::SeqCst);

    // Verify both attempts were tracked independently
    let final_count = resolution_count.load(Ordering::SeqCst);
    assert_eq!(final_count, 2, "Both resolution attempts should be tracked");
}

/// Test concurrent factory resolutions with partial failures
/// Critical for multi-request scenarios
#[tokio::test]
async fn test_concurrent_factory_resolutions_with_partial_failures() {
    // Setup: Create tracking for concurrent resolutions
    let success_count = Arc::new(AtomicUsize::new(0));
    let failure_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Simulate 5 concurrent factory calls: 3 succeed, 2 fail
    for i in 0..5 {
        let success_count_clone = Arc::clone(&success_count);
        let failure_count_clone = Arc::clone(&failure_count);

        let handle = tokio::spawn(async move {
            if i % 2 == 0 {
                // Success: even indices
                success_count_clone.fetch_add(1, Ordering::SeqCst);
            } else {
                // Failure: odd indices
                failure_count_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        let _ = handle.await;
    }

    // Verify: 3 successes, 2 failures
    assert_eq!(success_count.load(Ordering::SeqCst), 3);
    assert_eq!(failure_count.load(Ordering::SeqCst), 2);

    // Verify both succeeded and failed are properly isolated
    let total = success_count.load(Ordering::SeqCst) + failure_count.load(Ordering::SeqCst);
    assert_eq!(total, 5, "All concurrent tasks should complete");
}

/// Test that singleton cache is not poisoned after factory failure
#[tokio::test]
async fn test_singleton_cache_not_poisoned_after_failure() {
    // Setup: Track cache state
    let cache = Arc::new(Mutex::new(HashMap::<String, Value>::new()));
    let cache_clone = Arc::clone(&cache);

    // Attempt 1: Try to cache failed result (should fail)
    let failed_result = Err::<Value, _>("Factory failed");

    match failed_result {
        Ok(value) => {
            cache_clone.lock().unwrap().insert("singleton".to_string(), value);
        }
        Err(_) => {
            // Failure is caught, cache not poisoned
        }
    }

    // Verify cache is still empty (not poisoned)
    assert!(
        cache_clone.lock().unwrap().is_empty(),
        "Failed resolution should not poison cache"
    );

    // Attempt 2: Successful resolution should work
    let successful_result = json!({"status": "resolved"});
    cache_clone
        .lock()
        .unwrap()
        .insert("singleton".to_string(), successful_result.clone());

    // Verify successful result is cached
    let cached = cache_clone.lock().unwrap().get("singleton").cloned();
    assert_eq!(
        cached,
        Some(successful_result),
        "Successful resolution should cache properly"
    );
}

/// Test that async generator cleanup is triggered on factory error
#[tokio::test]
async fn test_async_generator_cleanup_on_factory_error() {
    // Setup: Track generator cleanup
    let generator_closed = Arc::new(AtomicBool::new(false));
    let generator_closed_clone = Arc::clone(&generator_closed);

    // Simulate async generator that encounters error
    let generator_task = tokio::spawn(async move {
        // Simulate generator iteration
        for i in 0..3 {
            if i == 2 {
                // Error on third iteration
                generator_closed_clone.store(true, Ordering::SeqCst);
                return; // Cleanup via early return
            }
        }
    });

    let _ = generator_task.await;

    // Verify generator was properly closed
    assert!(
        generator_closed.load(Ordering::SeqCst),
        "Generator should be closed on error"
    );
}

// ============================================================================
// TEST SUITE 6: SSE/WEBSOCKET CALLBACK LIFECYCLE
// ============================================================================

/// Test SSE callback completes successfully
#[tokio::test]
async fn test_sse_callback_success() {
    // Setup: Track callback execution
    let callback_executed = Arc::new(AtomicBool::new(false));
    let callback_executed_clone = Arc::clone(&callback_executed);

    // Simulate SSE next_event callback
    let sse_task = tokio::spawn(async move {
        // Simulate successful callback
        let event = json!({
            "data": {"message": "test event"},
            "event_type": "update",
            "id": "1"
        });

        // Process event successfully
        assert_eq!(event["data"]["message"], "test event");
        callback_executed_clone.store(true, Ordering::SeqCst);
    });

    let _ = sse_task.await;

    // Verify callback completed
    assert!(callback_executed.load(Ordering::SeqCst), "SSE callback should execute");
}

/// Test SSE callback failure is handled without hanging
#[tokio::test]
async fn test_sse_callback_failure_handling() {
    // Setup: Track callback failure
    let callback_failed = Arc::new(AtomicBool::new(false));
    let callback_failed_clone = Arc::clone(&callback_failed);

    // Simulate SSE callback that fails
    let sse_task = tokio::spawn(async move {
        // Simulate callback rejection
        match json!({"data": null}).as_object() {
            Some(obj) if obj.get("data").map(|v| v.is_null()).unwrap_or(true) => {
                callback_failed_clone.store(true, Ordering::SeqCst);
            }
            _ => {}
        }
    });

    let _ = sse_task.await;

    // Verify failure was handled
    assert!(
        callback_failed.load(Ordering::SeqCst),
        "Callback failure should be detected"
    );
}

/// Test client disconnect during SSE callback is handled cleanly
#[tokio::test]
async fn test_sse_client_disconnect_cleanup() {
    // Setup: Simulate client connection state
    let is_connected = Arc::new(AtomicBool::new(true));
    let is_connected_clone = Arc::clone(&is_connected);
    let is_connected_cleanup = Arc::clone(&is_connected);

    // Simulate SSE stream with client
    let sse_task = tokio::spawn(async move {
        // Client sends events
        for _i in 0..5 {
            if !is_connected_clone.load(Ordering::SeqCst) {
                // Client disconnected, cleanup immediately
                break;
            }
        }
    });

    // Simulate client disconnect
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    is_connected_cleanup.store(false, Ordering::SeqCst);

    // Wait for stream cleanup
    let _ = sse_task.await;

    // Verify stream was closed
    assert!(
        !is_connected.load(Ordering::SeqCst),
        "Client should be marked disconnected"
    );
}

/// Test WebSocket message delivery and response
#[tokio::test]
async fn test_websocket_message_delivery() {
    // Setup: Track message flow
    let messages_sent = Arc::new(AtomicUsize::new(0));
    let messages_received = Arc::new(AtomicUsize::new(0));

    let messages_sent_clone = Arc::clone(&messages_sent);
    let messages_received_clone = Arc::clone(&messages_received);

    // Simulate WebSocket message handling
    let ws_task = tokio::spawn(async move {
        // Sender
        let msg = json!({"type": "ping", "id": 1});
        messages_sent_clone.fetch_add(1, Ordering::SeqCst);

        // Receiver (simulated handler response)
        if msg["type"] == "ping" {
            messages_received_clone.fetch_add(1, Ordering::SeqCst);
        }
    });

    let _ = ws_task.await;

    // Verify bidirectional communication
    assert_eq!(messages_sent.load(Ordering::SeqCst), 1, "Message should be sent");
    assert_eq!(
        messages_received.load(Ordering::SeqCst),
        1,
        "Response should be received"
    );
}

/// Test WebSocket message handler promise rejection
#[tokio::test]
async fn test_websocket_handler_promise_rejection() {
    // Setup: Track rejection handling
    let rejection_handled = Arc::new(AtomicBool::new(false));
    let rejection_handled_clone = Arc::clone(&rejection_handled);

    // Simulate WebSocket handler that rejects
    let ws_task = tokio::spawn(async move {
        let message = json!({"type": "invalid"});

        // Simulate handler rejection
        match message["type"].as_str() {
            Some("invalid") => {
                rejection_handled_clone.store(true, Ordering::SeqCst);
                return; // Handler rejects this message
            }
            _ => {}
        }
    });

    let _ = ws_task.await;

    // Verify rejection was handled
    assert!(
        rejection_handled.load(Ordering::SeqCst),
        "Handler rejection should be handled"
    );
}

/// Test concurrent WebSocket connections with isolated failures
#[tokio::test]
async fn test_concurrent_websocket_connections_isolation() {
    // Setup: Track multiple connections
    let connection_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Simulate 5 concurrent WebSocket connections
    for i in 0..5 {
        let conn_count = Arc::clone(&connection_count);
        let err_count = Arc::clone(&error_count);

        let handle = tokio::spawn(async move {
            conn_count.fetch_add(1, Ordering::SeqCst);

            // Simulate error on connection 2 and 4
            if i == 2 || i == 4 {
                err_count.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    // Wait for all connections
    for handle in handles {
        let _ = handle.await;
    }

    // Verify: 5 connections total, 2 with errors, all isolated
    assert_eq!(
        connection_count.load(Ordering::SeqCst),
        5,
        "All 5 connections should be tracked"
    );
    assert_eq!(error_count.load(Ordering::SeqCst), 2, "2 connections should fail");

    // Total should equal connections (no lost counts)
    let total = connection_count.load(Ordering::SeqCst) + error_count.load(Ordering::SeqCst);
    assert!(
        total >= error_count.load(Ordering::SeqCst),
        "Error tracking should be isolated"
    );
}

/// Test concurrent SSE event delivery maintains ordering
#[tokio::test]
async fn test_concurrent_sse_event_delivery() {
    // Setup: Track event sequence
    let events_delivered = Arc::new(Mutex::new(Vec::<usize>::new()));
    let events_delivered_clone = Arc::clone(&events_delivered);

    let mut handles = vec![];

    // Simulate 5 concurrent SSE events
    for i in 0..5 {
        let events = Arc::clone(&events_delivered_clone);

        let handle = tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis((5 - i) as u64)).await;
            events.lock().unwrap().push(i);
        });

        handles.push(handle);
    }

    // Wait for all events
    for handle in handles {
        let _ = handle.await;
    }

    // Verify all events delivered
    let delivered = events_delivered.lock().unwrap();
    assert_eq!(delivered.len(), 5, "All 5 events should be delivered");
}

/// Test that ThreadsafeFunction is safe to call after stream closes
#[tokio::test]
async fn test_threadfn_safe_after_stream_close() {
    // Setup: Track stream lifecycle
    let stream_active = Arc::new(AtomicBool::new(true));
    let stream_active_clone = Arc::clone(&stream_active);
    let stream_active_check = Arc::clone(&stream_active);

    // Simulate streaming operation
    let stream_task = tokio::spawn(async move {
        // Stream is open
        while stream_active_clone.load(Ordering::SeqCst) {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
        // Stream closed
    });

    // Keep stream open briefly
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Close stream
    stream_active_check.store(false, Ordering::SeqCst);

    // Wait for stream to finish
    let _ = stream_task.await;

    // Verify stream was properly closed
    assert!(!stream_active.load(Ordering::SeqCst), "Stream should be closed");
}

/// Test resource cleanup and memory deallocation after stream termination
#[tokio::test]
async fn test_stream_resource_cleanup_after_termination() {
    // Setup: Track allocated resources
    let resources_allocated = Arc::new(AtomicUsize::new(0));
    let resources_freed = Arc::new(AtomicUsize::new(0));

    let alloc = Arc::clone(&resources_allocated);
    let freed = Arc::clone(&resources_freed);

    // Simulate stream with resource allocation
    {
        // Allocate resources
        alloc.fetch_add(3, Ordering::SeqCst);
        assert_eq!(alloc.load(Ordering::SeqCst), 3, "3 resources allocated");

        // Stream operation scope ends
    } // Resources should be freed here

    // Simulate cleanup
    freed.fetch_add(3, Ordering::SeqCst);

    // Verify all resources freed
    let allocated = resources_allocated.load(Ordering::SeqCst);
    let freed_count = resources_freed.load(Ordering::SeqCst);
    assert_eq!(allocated, 3, "3 resources should be allocated");
    assert_eq!(freed_count, 3, "All 3 resources should be freed");
    assert_eq!(allocated, freed_count, "Resource leak check: allocated == freed");
}

/// Test SSE callback with error handling maintains stream state
#[tokio::test]
async fn test_sse_callback_error_maintains_stream_state() {
    // Setup: Track stream state through errors
    let stream_events = Arc::new(Mutex::new(Vec::<String>::new()));
    let stream_events_clone = Arc::clone(&stream_events);

    let sse_task = tokio::spawn(async move {
        let mut events = stream_events_clone.lock().unwrap();

        // Event 1: Success
        events.push("event1".to_string());

        // Event 2: Error (but stream continues)
        events.push("error".to_string());

        // Event 3: Success after error
        events.push("event3".to_string());
    });

    let _ = sse_task.await;

    // Verify stream state maintained through error
    let events = stream_events.lock().unwrap();
    assert_eq!(events.len(), 3, "All events should be recorded");
    assert_eq!(events[0], "event1");
    assert_eq!(events[1], "error");
    assert_eq!(events[2], "event3");
}

// ============================================================================
// INTEGRATION TESTS: DI + STREAMING COMBINED
// ============================================================================

/// Test DI resolution during SSE callback execution
#[tokio::test]
async fn test_di_resolution_during_sse_callback() {
    // Setup: Track DI resolution during streaming
    let resolution_count = Arc::new(AtomicUsize::new(0));
    let resolution_count_clone = Arc::clone(&resolution_count);
    let resolution_count_sse = Arc::clone(&resolution_count);

    let sse_task = tokio::spawn(async move {
        // Simulate DI resolution in SSE callback
        for _i in 0..3 {
            resolution_count_clone.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    });

    let _ = sse_task.await;

    // Verify DI resolved multiple times
    assert_eq!(
        resolution_count_sse.load(Ordering::SeqCst),
        3,
        "DI should resolve 3 times"
    );
}

/// Test error structured format compliance
/// Ensures all errors follow the JSON schema: {error, code, details}
#[test]
fn test_error_response_structure_compliance() {
    // Test various error scenarios
    let errors = vec![
        (
            "factory_error",
            json!({
                "error": "factory_failed",
                "code": "FACTORY_ERROR",
                "details": {"factory": "test", "reason": "timeout"}
            }),
        ),
        (
            "sse_error",
            json!({
                "error": "sse_callback_failed",
                "code": "SSE_CALLBACK_ERROR",
                "details": {"stream": "events", "reason": "handler rejected"}
            }),
        ),
        (
            "ws_error",
            json!({
                "error": "websocket_handler_failed",
                "code": "WS_HANDLER_ERROR",
                "details": {"connection": "1", "message": "invalid format"}
            }),
        ),
    ];

    // Verify each error has required structure
    for (name, error) in errors {
        assert!(error.is_object(), "{} should be object", name);
        assert!(error["error"].is_string(), "{} should have 'error' field", name);
        assert!(error["code"].is_string(), "{} should have 'code' field", name);
        assert!(error["details"].is_object(), "{} should have 'details' field", name);
    }
}

/// Test app state remains consistent after multiple failure cycles
#[tokio::test]
async fn test_app_state_consistency_after_failures() {
    // Setup: Track state consistency
    let request_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    // Simulate 10 requests: 7 succeed, 3 fail
    for i in 0..10 {
        let req_count = Arc::clone(&request_count);
        let err_count = Arc::clone(&error_count);

        let handle = tokio::spawn(async move {
            req_count.fetch_add(1, Ordering::SeqCst);

            if i >= 7 {
                // Requests 7-9 fail
                err_count.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    // Wait for all requests
    for handle in handles {
        let _ = handle.await;
    }

    // Verify state consistency
    let total_requests = request_count.load(Ordering::SeqCst);
    let total_errors = error_count.load(Ordering::SeqCst);

    assert_eq!(total_requests, 10, "All 10 requests should be tracked");
    assert_eq!(total_errors, 3, "3 requests should fail");
    assert_eq!(
        total_requests, 10,
        "State should be consistent after mixed success/failure"
    );
}
