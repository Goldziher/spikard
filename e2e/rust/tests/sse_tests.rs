//! SSE tests generated from AsyncAPI fixtures

#[cfg(test)]
mod sse {
use axum::body::Body;
use axum::http::Request;
use serde_json::Value;
use spikard::testing::TestServer;


#[tokio::test]
async fn test_sse_notifications() {
    let app = spikard_e2e_app::create_app_sse_notifications()
        .expect("Failed to build SSE app");
    let server = TestServer::from_app(app).expect("Failed to build server");
    let request = Request::builder()
        .method("GET")
        .uri("/notifications")
        .body(Body::empty())
        .unwrap();
    let snapshot = server.call(request).await.unwrap();
    assert_eq!(snapshot.status, 200);

    let body = String::from_utf8(snapshot.body.clone()).expect("SSE stream should be UTF-8");
    let events: Vec<&str> = body
        .split("\n\n")
        .filter(|chunk| chunk.starts_with("data:"))
        .collect();

    let expected_events = vec!["{\"level\":\"critical\",\"message\":\"Database connection pool exhausted\",\"source\":\"database-service\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}", "[{\"message\":\"example_message\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"example_type\"},{\"message\":\"example_message\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"example_type\"}]", "{\"body\":\"You have received a new direct message\",\"priority\":\"high\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"New message from John\",\"type\":\"user_notification\",\"userId\":\"user_12345\"}", "{\"message\":\"All systems operational\",\"metadata\":{\"region\":\"us-east-1\",\"uptime\":99.99},\"service\":\"payment-gateway\",\"status\":\"operational\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}"];
    assert_eq!(
        events.len(),
        expected_events.len(),
        "Expected {} events, got {}",
        expected_events.len(),
        events.len()
    );

    for (idx, expected) in expected_events.iter().enumerate() {
        let payload = events[idx].trim_start_matches("data:").trim();
        let parsed: Value = serde_json::from_str(payload).expect("valid JSON payload");
        let expected_value: Value = serde_json::from_str(expected).expect("valid expected JSON");
        assert_eq!(parsed, expected_value, "Mismatched event at index {}", idx);
    }
}

}
