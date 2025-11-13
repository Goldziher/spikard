//! SSE tests generated from AsyncAPI fixtures

#[cfg(test)]
mod sse {
    use axum::body::Body;
    use axum::http::Request;
    use axum_test::TestServer;
    use serde_json::Value;
    use spikard_http::testing::{call_test_server, snapshot_response};

    #[tokio::test]
    async fn test_sse_notifications() {
        let app = spikard_e2e_app::create_app_sse_notifications();
        let request = Request::builder()
            .method("GET")
            .uri("/notifications")
            .body(Body::empty())
            .unwrap();
        let server = TestServer::new(app).expect("Failed to build server");
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();
        assert_eq!(snapshot.status, 200);

        let body = String::from_utf8(snapshot.body.clone()).expect("SSE stream should be UTF-8");
        let events: Vec<&str> = body.split("\n\n").filter(|chunk| chunk.starts_with("data:")).collect();

        let expected_events = vec![
            "{\"level\":\"example_level\",\"message\":\"example_message\",\"source\":\"example_source\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}",
            "{\"body\":\"example_body\",\"priority\":\"example_priority\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"example_title\",\"type\":\"user_notification\",\"userId\":\"example_userId\"}",
            "{\"message\":\"example_message\",\"metadata\":{},\"service\":\"example_service\",\"status\":\"example_status\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}",
        ];
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
