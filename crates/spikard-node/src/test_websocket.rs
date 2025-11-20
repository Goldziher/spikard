
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;
use spikard_http::testing::{WebSocketConnection as RustWebSocketConnection, WebSocketMessage as RustWebSocketMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Node.js wrapper for WebSocket test client
#[napi]
pub struct WebSocketTestConnection {
    inner: Arc<Mutex<RustWebSocketConnection>>,
}

impl WebSocketTestConnection {
    pub fn new(inner: RustWebSocketConnection) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[napi]
impl WebSocketTestConnection {
    /// Send a text message
    #[napi]
    pub async fn send_text(&self, text: String) -> Result<()> {
        let mut ws = self.inner.lock().await;
        ws.send_text(text).await;
        Ok(())
    }

    /// Send a JSON message
    #[napi]
    pub async fn send_json(&self, obj: serde_json::Value) -> Result<()> {
        println!("sending json from node test websocket: {obj}");
        let mut ws = self.inner.lock().await;
        ws.send_json(&obj).await;
        Ok(())
    }

    /// Receive a text message
    #[napi]
    pub async fn receive_text(&self) -> Result<String> {
        let mut ws = self.inner.lock().await;
        let text = ws.receive_text().await;
        Ok(text)
    }

    /// Receive and parse a JSON message
    #[napi]
    pub async fn receive_json(&self) -> Result<serde_json::Value> {
        let mut ws = self.inner.lock().await;
        let json_value: Value = ws.receive_json().await;
        println!("received json from websocket: {json_value}");
        Ok(json_value)
    }

    /// Receive raw bytes
    #[napi]
    pub async fn receive_bytes(&self) -> Result<Buffer> {
        let mut ws = self.inner.lock().await;
        let bytes = ws.receive_bytes().await;
        Ok(Buffer::from(bytes.to_vec()))
    }

    /// Receive a message and return WebSocketMessage
    #[napi]
    pub async fn receive_message(&self) -> Result<WebSocketMessage> {
        let mut ws = self.inner.lock().await;
        let msg = ws.receive_message().await;
        Ok(WebSocketMessage::from_rust(msg))
    }

    /// Close the WebSocket connection
    #[napi]
    pub async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// Node.js wrapper for WebSocket messages
#[napi]
pub struct WebSocketMessage {
    inner: RustWebSocketMessage,
}

impl WebSocketMessage {
    pub fn from_rust(msg: RustWebSocketMessage) -> Self {
        Self { inner: msg }
    }
}

#[napi]
impl WebSocketMessage {
    /// Get message as text if it's a text message
    #[napi]
    pub fn as_text(&self) -> Option<String> {
        self.inner.as_text().map(|s| s.to_string())
    }

    /// Get message as JSON if it's a text message containing JSON
    #[napi]
    pub fn as_json(&self) -> Result<Option<serde_json::Value>> {
        match self.inner.as_json() {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }

    /// Get message as binary if it's a binary message
    #[napi]
    pub fn as_binary(&self) -> Option<Buffer> {
        self.inner.as_binary().map(|bytes| Buffer::from(bytes.to_vec()))
    }

    /// Check if this is a close message
    #[napi]
    pub fn is_close(&self) -> bool {
        self.inner.is_close()
    }
}
