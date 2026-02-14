//! WebSocket tests generated from AsyncAPI fixtures

#[cfg(test)]
mod websocket {
use serde_json::Value;
use spikard::testing::TestServer;


#[tokio::test]
async fn test_websocket_chat_msg_1() {
    let app = spikard_e2e_app::create_app_websocket_chat()
        .expect("Failed to build WebSocket app");
    let server = TestServer::from_app(app).expect("Failed to build server");

    let mut ws = server.connect_websocket("/chat").await;

    let message: Value = serde_json::from_str("{\"text\":\"Hello, everyone!\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"message\",\"user\":\"alice\"}").expect("valid JSON");

    ws.send_json(&message).await;

    let response: Value = ws.receive_json().await;

    assert_eq!(response["validated"], Value::Bool(true), "Should have validated field set to true");

    if let Some(obj) = message.as_object() {
        for (key, value) in obj {
            assert_eq!(response[key], *value, "Field should match original value");
        }
    }

    ws.close().await;
}


#[tokio::test]
async fn test_websocket_chat_msg_2() {
    let app = spikard_e2e_app::create_app_websocket_chat()
        .expect("Failed to build WebSocket app");
    let server = TestServer::from_app(app).expect("Failed to build server");

    let mut ws = server.connect_websocket("/chat").await;

    let message: Value = serde_json::from_str("{\"timestamp\":\"2024-01-15T10:35:00Z\",\"type\":\"userLeft\",\"user\":\"charlie\"}").expect("valid JSON");

    ws.send_json(&message).await;

    let response: Value = ws.receive_json().await;

    assert_eq!(response["validated"], Value::Bool(true), "Should have validated field set to true");

    if let Some(obj) = message.as_object() {
        for (key, value) in obj {
            assert_eq!(response[key], *value, "Field should match original value");
        }
    }

    ws.close().await;
}


#[tokio::test]
async fn test_websocket_chat_msg_3() {
    let app = spikard_e2e_app::create_app_websocket_chat()
        .expect("Failed to build WebSocket app");
    let server = TestServer::from_app(app).expect("Failed to build server");

    let mut ws = server.connect_websocket("/chat").await;

    let message: Value = serde_json::from_str("{\"timestamp\":\"2024-01-15T10:29:55Z\",\"type\":\"userJoined\",\"user\":\"bob\"}").expect("valid JSON");

    ws.send_json(&message).await;

    let response: Value = ws.receive_json().await;

    assert_eq!(response["validated"], Value::Bool(true), "Should have validated field set to true");

    if let Some(obj) = message.as_object() {
        for (key, value) in obj {
            assert_eq!(response[key], *value, "Field should match original value");
        }
    }

    ws.close().await;
}

}
