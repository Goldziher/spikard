#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! SSE Notifications Server Example
//!
//! Demonstrates Server-Sent Events support in Spikard matching the AsyncAPI notifications specification.

use axum::{Router, routing::get};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use spikard_http::{SseEvent, SseEventProducer, SseState, sse_handler};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{Duration, sleep};
use tracing::info;

/// Notification event types matching AsyncAPI specification
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
enum Notification {
    #[serde(rename = "system_alert")]
    SystemAlert {
        level: String,
        message: String,
        source: String,
        timestamp: String,
    },
    #[serde(rename = "user_notification")]
    UserNotification {
        #[serde(rename = "userId")]
        user_id: String,
        title: String,
        body: String,
        priority: String,
        timestamp: String,
    },
    #[serde(rename = "status_update")]
    StatusUpdate {
        service: String,
        status: String,
        message: Option<String>,
        metadata: serde_json::Value,
        timestamp: String,
    },
}

/// Notification producer implementing SseEventProducer trait
struct NotificationProducer {
    counter: AtomicUsize,
}

impl NotificationProducer {
    fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    fn create_notification(&self, index: usize) -> Notification {
        let timestamp = Utc::now().to_rfc3339();

        match index % 3 {
            0 => Notification::SystemAlert {
                level: "info".to_string(),
                message: format!("System checkpoint {} reached", index),
                source: "monitoring-system".to_string(),
                timestamp,
            },
            1 => Notification::UserNotification {
                user_id: format!("user_{}", index),
                title: "New Update Available".to_string(),
                body: format!("Version 1.{} is now available for download", index),
                priority: "normal".to_string(),
                timestamp,
            },
            _ => Notification::StatusUpdate {
                service: "api-gateway".to_string(),
                status: "operational".to_string(),
                message: Some(format!("Health check {} passed", index)),
                metadata: json!({
                    "response_time_ms": 50 + (index % 100),
                    "active_connections": 100 + (index % 50)
                }),
                timestamp,
            },
        }
    }
}

impl SseEventProducer for NotificationProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        sleep(Duration::from_secs(2)).await;

        let count = self.counter.fetch_add(1, Ordering::Relaxed);

        if count >= 10 {
            info!("Completed sending 10 notifications");
            return None;
        }

        let notification = self.create_notification(count);
        let event_type = match &notification {
            Notification::SystemAlert { .. } => "system_alert",
            Notification::UserNotification { .. } => "user_notification",
            Notification::StatusUpdate { .. } => "status_update",
        };

        info!("Sending notification #{}: {}", count + 1, event_type);

        let data = serde_json::to_value(notification).unwrap();

        Some(
            SseEvent::with_type(event_type, data)
                .with_id(format!("event_{}", count))
                .with_retry(3000),
        )
    }

    async fn on_connect(&self) {
        info!("Client connected to notifications stream");
    }

    async fn on_disconnect(&self) {
        info!("Client disconnected from notifications stream");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,sse_notifications=debug")
        .init();

    let producer = NotificationProducer::new();
    let sse_state = SseState::new(producer);

    let app = Router::new()
        .route("/notifications", get(sse_handler::<NotificationProducer>))
        .with_state(sse_state);

    let addr = "127.0.0.1:8000";
    info!("SSE notifications server listening on {}", addr);
    info!("Connect at: http://{}/notifications", addr);
    info!("Try: curl -N http://{}/notifications", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");

    axum::serve(listener, app).await.expect("Server error");
}
