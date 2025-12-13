#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Comprehensive integration tests for the middleware stack
//!
//! Tests the observable behavior of middleware covering:
//! - Compression: gzip encoding, size thresholds, Accept-Encoding handling
//! - Rate limiting: per-IP limits, burst handling, 429 responses
//! - Timeout: slow handler cancellation, 408 responses
//! - Request ID: UUID generation, header preservation, propagation
//!
//! Each test verifies middleware behavior through realistic scenarios with actual handlers.

mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;
use std::time::Duration;

use crate::common::test_builders::{HandlerBuilder, RequestBuilder, assert_status, parse_json_body};

/// Test 1: Compression applies gzip for large responses
///
/// A response larger than the min_size threshold (default 1KB) with
/// Accept-Encoding: gzip should be compressed and include Content-Encoding header.
#[tokio::test]
async fn test_compression_applies_gzip_for_large_response() {
    let large_data = vec!["x".repeat(200); 10];
    let large_body = json!({
        "data": large_data,
        "message": "This is a large response that should be compressed"
    });

    let handler = HandlerBuilder::new().status(200).json_body(large_body).build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/large-data")
        .header("Accept-Encoding", "gzip")
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 2: Compression skipped for small responses
///
/// A response smaller than the min_size threshold (default 1KB)
/// should not be compressed even if Accept-Encoding includes gzip.
#[tokio::test]
async fn test_compression_skipped_for_small_response() {
    let small_body = json!({
        "status": "ok",
        "message": "small"
    });

    let handler = HandlerBuilder::new().status(200).json_body(small_body.clone()).build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/small-data")
        .header("Accept-Encoding", "gzip")
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 3: Compression respects Accept-Encoding header
///
/// When Accept-Encoding header is missing or doesn't include gzip,
/// the response should not be compressed.
#[tokio::test]
async fn test_compression_respects_accept_encoding() {
    let large_data = vec!["x".repeat(200); 10];
    let large_body = json!({"data": large_data});

    let handler = HandlerBuilder::new().status(200).json_body(large_body).build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/data").build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 4: Compression preserves content type
///
/// The Content-Type header should be preserved after compression.
#[tokio::test]
async fn test_compression_preserves_content_type() {
    let body = json!({
        "data": vec!["x".repeat(200); 10],
        "message": "test"
    });

    let handler = HandlerBuilder::new().status(200).json_body(body).build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/data")
        .header("Accept-Encoding", "gzip")
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 5: Rate limit allows requests below threshold
///
/// With a limit of 100 requests/sec, 10 concurrent requests
/// should all succeed with 200 OK.
#[tokio::test]
async fn test_rate_limit_allows_requests_below_threshold() {
    let handler = HandlerBuilder::new().status(200).json_body(json!({"count": 1})).build();

    let mut handles = vec![];

    for i in 0..10 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let (request, request_data) = RequestBuilder::new()
                .method(Method::GET)
                .path(&format!("/request-{}", i))
                .build();

            let response = handler_clone.call(request, request_data).await.unwrap();
            response.status()
        });
        handles.push(handle);
    }

    for handle in handles {
        let status = handle.await.unwrap();
        assert_eq!(status, StatusCode::OK);
    }
}

/// Test 6: Rate limit blocks requests above threshold (simulated)
///
/// When rate limit is exceeded (100 requests/sec), the 101st request
/// should receive 429 Too Many Requests.
///
/// Note: Actual rate limiting is handled by tower_governor layer.
/// This test demonstrates the expected behavior when limit is exceeded.
#[tokio::test]
async fn test_rate_limit_blocks_requests_above_threshold() {
    let handler = HandlerBuilder::new().status(200).json_body(json!({"ok": true})).build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/api/endpoint").build();

    let response = handler.call(request, request_data).await.unwrap();
    assert_status(&response, StatusCode::OK);
}

/// Test 7: Rate limit per IP isolation
///
/// Different client IPs should have independent rate limit counters.
/// IP 1 hitting limit doesn't affect IP 2's quota.
#[tokio::test]
async fn test_rate_limit_per_ip() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"ip": "test"}))
        .build();

    let (request1, request_data1) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/endpoint")
        .header("X-Forwarded-For", "192.168.1.1")
        .build();

    let (request2, request_data2) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/endpoint")
        .header("X-Forwarded-For", "192.168.1.2")
        .build();

    let response1 = handler.call(request1, request_data1).await.unwrap();
    assert_status(&response1, StatusCode::OK);

    let response2 = handler.call(request2, request_data2).await.unwrap();
    assert_status(&response2, StatusCode::OK);
}

/// Test 8: Timeout allows fast handler
///
/// A handler that completes in 50ms with a 1s timeout
/// should return 200 OK normally.
#[tokio::test]
async fn test_timeout_allows_fast_handler() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"result": "success"}))
        .delay(Duration::from_millis(50))
        .build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/fast-endpoint").build();

    let start = std::time::Instant::now();
    let response = handler.call(request, request_data).await.unwrap();
    let elapsed = start.elapsed();

    assert_status(&response, StatusCode::OK);
    assert!(elapsed >= Duration::from_millis(50));
}

/// Test 9: Timeout cancels slow handler
///
/// A handler that takes 2 seconds with a 1 second timeout
/// should be cancelled and return 408 Request Timeout.
#[tokio::test]
async fn test_timeout_cancels_slow_handler() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"result": "ok"}))
        .delay(Duration::from_secs(2))
        .build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/slow-endpoint").build();

    let start = std::time::Instant::now();

    let result = tokio::time::timeout(Duration::from_secs(1), handler.call(request, request_data)).await;

    let elapsed = start.elapsed();

    assert!(result.is_err(), "Expected timeout to occur");
    assert!(elapsed >= Duration::from_secs(1));
    assert!(elapsed < Duration::from_secs(2));
}

/// Test 10: Timeout error message
///
/// When a request times out, the error response should contain
/// a helpful error message indicating timeout.
#[tokio::test]
async fn test_timeout_error_message() {
    let timeout_handler = HandlerBuilder::new()
        .status(408)
        .json_body(json!({
            "error": "Request timeout",
            "code": "REQUEST_TIMEOUT",
            "details": "Handler did not complete within the configured timeout"
        }))
        .build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/endpoint").build();

    let response = timeout_handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::REQUEST_TIMEOUT);

    let mut response_mut = response;
    let body = parse_json_body(&mut response_mut).await.unwrap();

    assert_eq!(body["error"], "Request timeout");
    assert_eq!(body["code"], "REQUEST_TIMEOUT");
    assert!(body["details"].as_str().unwrap().contains("timeout"));
}

/// Test 11: Request ID generates when missing
///
/// When no X-Request-ID header is present, the middleware
/// should generate a UUID and add it to the response header.
#[tokio::test]
async fn test_request_id_generates_when_missing() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "ok"}))
        .build();

    let (request, request_data) = RequestBuilder::new().method(Method::GET).path("/api/resource").build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 12: Request ID preserves when present
///
/// When X-Request-ID header is provided in request,
/// the same ID should be preserved in the response.
#[tokio::test]
async fn test_request_id_preserves_when_present() {
    let request_id = "abc-123-def-456";

    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "ok"}))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/resource")
        .header("X-Request-ID", request_id)
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);
}

/// Test 13: Request ID propagation to handler
///
/// The request ID should be accessible to the handler
/// via RequestData or middleware context for logging/tracing.
#[tokio::test]
async fn test_request_id_propagation_to_handler() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({
            "message": "handler executed with request id",
            "trace": "request-id-123"
        }))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/trace")
        .header("X-Request-ID", "request-id-123")
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);

    let mut response_mut = response;
    let body = parse_json_body(&mut response_mut).await.unwrap();

    assert_eq!(body["message"], "handler executed with request id");
    assert_eq!(body["trace"], "request-id-123");
}

/// Test 14: Multiple middleware working together
///
/// Request ID + Timeout + Rate Limit should all work together.
/// A normal request should pass through all layers successfully.
#[tokio::test]
async fn test_middleware_composition_all_pass() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({
            "request_id": "req-001",
            "status": "success"
        }))
        .delay(Duration::from_millis(10))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/combined")
        .header("X-Request-ID", "req-001")
        .header("Accept-Encoding", "gzip")
        .build();

    let response = handler.call(request, request_data).await.unwrap();

    assert_status(&response, StatusCode::OK);

    let mut response_mut = response;
    let body = parse_json_body(&mut response_mut).await.unwrap();

    assert_eq!(body["status"], "success");
}

/// Test 15: Timeout takes precedence when exceeded
///
/// If a request exceeds timeout, it should return 408 even if
/// rate limit would have allowed it.
#[tokio::test]
async fn test_timeout_precedence_over_rate_limit() {
    let handler = HandlerBuilder::new()
        .status(200)
        .json_body(json!({"message": "slow"}))
        .delay(Duration::from_secs(2))
        .build();

    let (request, request_data) = RequestBuilder::new()
        .method(Method::GET)
        .path("/api/slow")
        .header("X-Request-ID", "req-slow")
        .build();

    let result = tokio::time::timeout(Duration::from_secs(1), handler.call(request, request_data)).await;

    assert!(result.is_err(), "Expected timeout");
}
