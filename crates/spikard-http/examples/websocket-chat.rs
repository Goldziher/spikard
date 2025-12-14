#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! WebSocket Chat Server Example
//!
//! Demonstrates WebSocket support in Spikard matching the AsyncAPI chat specification.
//! This server implements a simple chat system with three message types:
//! - chatMessage: User sends a chat message
//! - userJoined: User joins the chat
//! - userLeft: User leaves the chat

use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spikard_http::{WebSocketHandler, WebSocketState, websocket_handler};
use tracing::{info, warn};

/// Chat message types matching AsyncAPI specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
enum ChatMessage {
    #[serde(rename = "message")]
    ChatMessage {
        user: String,
        text: String,
        timestamp: String,
    },
    #[serde(rename = "userJoined")]
    UserJoined { user: String, timestamp: String },
    #[serde(rename = "userLeft")]
    UserLeft { user: String, timestamp: String },
}

/// Chat handler implementing WebSocketHandler trait
struct ChatHandler;

impl WebSocketHandler for ChatHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        match serde_json::from_value::<ChatMessage>(message.clone()) {
            Ok(chat_msg) => {
                match chat_msg {
                    ChatMessage::ChatMessage { ref user, ref text, .. } => {
                        info!("Chat message from {}: {}", user, text);
                    }
                    ChatMessage::UserJoined { ref user, .. } => {
                        info!("User joined: {}", user);
                    }
                    ChatMessage::UserLeft { ref user, .. } => {
                        info!("User left: {}", user);
                    }
                }

                Some(message)
            }
            Err(e) => {
                warn!("Failed to parse chat message: {}", e);
                Some(serde_json::json!({
                    "type": "error",
                    "message": format!("Invalid message format: {}", e)
                }))
            }
        }
    }

    async fn on_connect(&self) {
        info!("Client connected to chat");
    }

    async fn on_disconnect(&self) {
        info!("Client disconnected from chat");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,websocket_chat=debug")
        .init();

    let ws_state = WebSocketState::new(ChatHandler);

    let app = Router::new()
        .route("/chat", get(websocket_handler::<ChatHandler>))
        .with_state(ws_state);

    let addr = "127.0.0.1:8000";
    info!("WebSocket chat server listening on {}", addr);
    info!("Connect at: ws://{}/chat", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server error");
}
