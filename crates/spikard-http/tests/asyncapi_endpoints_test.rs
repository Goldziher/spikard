//! Integration tests for AsyncAPI HTTP endpoints
//!
//! Covers:
//! - POST /asyncapi/parse   — valid spec → 200; bad spec → 400 ProblemDetails
//! - POST /asyncapi/validate — valid payload → {"valid":true}; bad payload → {"valid":false, "errors":[…]}
//! - GET  /asyncapi.json    — registered spec → 200; unregistered → 404

use axum::http::StatusCode;
use serde_json::{Value, json};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{AsyncApiConfig, ServerConfig};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn build_server_with_asyncapi(spec: Option<Value>) -> axum_test::TestServer {
    let config = ServerConfig {
        asyncapi: Some(AsyncApiConfig { enabled: true, spec }),
        ..ServerConfig::default()
    };
    let router = build_router_with_handlers_and_config(vec![], config, vec![]).expect("router built");
    axum_test::TestServer::new(router)
}

fn minimal_valid_spec() -> Value {
    json!({
        "asyncapi": "3.0.0",
        "info": { "title": "Chat API", "version": "1.0.0" },
        "channels": {
            "chat": {
                "address": "/chat",
                "messages": {
                    "ChatMessage": {
                        "payload": {
                            "type": "object",
                            "properties": {
                                "text": { "type": "string" },
                                "user_id": { "type": "string" }
                            },
                            "required": ["text", "user_id"]
                        }
                    }
                }
            }
        },
        "operations": {
            "sendChat": {
                "action": "send",
                "channel": { "$ref": "#/channels/chat" }
            }
        },
        "components": {}
    })
}

fn order_api_spec() -> Value {
    json!({
        "asyncapi": "3.0.0",
        "info": { "title": "Order API", "version": "1.0.0" },
        "channels": {
            "orders/new": {
                "address": "orders/new",
                "messages": {
                    "NewOrder": {
                        "payload": {
                            "type": "object",
                            "properties": {
                                "order_id": { "type": "string" },
                                "amount": { "type": "number" },
                                "currency": { "type": "string" }
                            },
                            "required": ["order_id", "amount", "currency"]
                        }
                    }
                }
            }
        },
        "operations": {},
        "components": {}
    })
}

// ── POST /asyncapi/parse ──────────────────────────────────────────────────────

#[tokio::test]
async fn post_asyncapi_parse_valid_spec_returns_200_with_structured_result() {
    let server = build_server_with_asyncapi(None);
    let spec = minimal_valid_spec();

    let response = server.post("/asyncapi/parse").json(&spec).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["spec_version"], "3.0.0");
    assert_eq!(body["title"], "Chat API");
    assert_eq!(body["api_version"], "1.0.0");
    let channels = body["channels"].as_array().expect("channels array");
    assert_eq!(channels.len(), 1);
    assert_eq!(channels[0]["name"], "chat");
    let operations = body["operations"].as_array().expect("operations array");
    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0]["name"], "sendChat");
    assert_eq!(operations[0]["action"], "send");
}

#[tokio::test]
async fn post_asyncapi_parse_missing_asyncapi_field_returns_400_problem_details() {
    let server = build_server_with_asyncapi(None);
    let spec = json!({
        "info": { "title": "No version field", "version": "1.0.0" },
        "channels": {}
    });

    let response = server.post("/asyncapi/parse").json(&spec).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = response.json();
    assert_eq!(body["status"], 400);
    let detail = body["detail"].as_str().expect("detail string");
    assert!(
        detail.contains("asyncapi") || detail.contains("parse"),
        "Unexpected detail: {detail}"
    );
}

#[tokio::test]
async fn post_asyncapi_parse_unsupported_version_returns_400() {
    let server = build_server_with_asyncapi(None);
    let spec = json!({
        "asyncapi": "2.0.0",
        "info": { "title": "Old API", "version": "1.0.0" },
        "channels": {}
    });

    let response = server.post("/asyncapi/parse").json(&spec).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = response.json();
    let detail = body["detail"].as_str().expect("detail string");
    assert!(
        detail.contains("2.0.0") || detail.contains("unsupported"),
        "Unexpected detail: {detail}"
    );
}

#[tokio::test]
async fn post_asyncapi_parse_multi_channel_spec_returns_all_channels() {
    let server = build_server_with_asyncapi(None);
    let spec = json!({
        "asyncapi": "3.0.0",
        "info": { "title": "Multi-Channel API", "version": "1.0.0" },
        "channels": {
            "chat/messages": {
                "address": "chat/messages",
                "messages": {
                    "ChatMessage": { "$ref": "#/components/messages/ChatMessage" }
                }
            },
            "user/events": {
                "address": "user/events",
                "messages": {
                    "UserEvent": { "$ref": "#/components/messages/UserEvent" }
                }
            }
        },
        "operations": {},
        "components": {
            "messages": {
                "ChatMessage": {
                    "payload": { "type": "object", "properties": { "text": { "type": "string" } } }
                },
                "UserEvent": {
                    "payload": { "type": "object", "properties": { "event": { "type": "string" } } }
                }
            }
        }
    });

    let response = server.post("/asyncapi/parse").json(&spec).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    let channels = body["channels"].as_array().expect("channels array");
    assert_eq!(channels.len(), 2);
    let names: Vec<&str> = channels
        .iter()
        .map(|c| c["name"].as_str().expect("channel name"))
        .collect();
    assert!(names.contains(&"chat/messages"), "Missing chat/messages");
    assert!(names.contains(&"user/events"), "Missing user/events");
}

#[tokio::test]
async fn post_asyncapi_parse_summary_fields_fixture() {
    let server = build_server_with_asyncapi(None);
    let spec = json!({
        "asyncapi": "3.0.0",
        "info": {
            "title": "Inventory Service",
            "version": "3.2.1"
        },
        "channels": {
            "inventory/updates": {
                "address": "inventory/updates",
                "messages": {
                    "InventoryUpdate": { "payload": { "type": "object" } }
                }
            }
        },
        "operations": {},
        "components": {}
    });

    let response = server.post("/asyncapi/parse").json(&spec).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["spec_version"], "3.0.0");
    assert_eq!(body["title"], "Inventory Service");
    assert_eq!(body["api_version"], "3.2.1");
    assert_eq!(body["channels"].as_array().map(|a| a.len()), Some(1));
}

// ── POST /asyncapi/validate ───────────────────────────────────────────────────

#[tokio::test]
async fn post_asyncapi_validate_valid_payload_returns_valid_true() {
    let server = build_server_with_asyncapi(None);
    let request_body = json!({
        "spec": order_api_spec(),
        "channel": "orders/new",
        "message": "NewOrder",
        "payload": {
            "order_id": "ORD-001",
            "amount": 99.99,
            "currency": "USD"
        }
    });

    let response = server.post("/asyncapi/validate").json(&request_body).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["valid"], true);
}

#[tokio::test]
async fn post_asyncapi_validate_missing_required_fields_returns_valid_false_with_errors() {
    let server = build_server_with_asyncapi(None);
    let request_body = json!({
        "spec": order_api_spec(),
        "channel": "orders/new",
        "message": "NewOrder",
        "payload": {
            "order_id": "ORD-002"
        }
    });

    let response = server.post("/asyncapi/validate").json(&request_body).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["valid"], false);
    let errors = body["errors"].as_array().expect("errors array");
    assert!(
        !errors.is_empty(),
        "Expected validation errors for missing required fields"
    );
}

#[tokio::test]
async fn post_asyncapi_validate_unknown_channel_returns_400() {
    let server = build_server_with_asyncapi(None);
    let request_body = json!({
        "spec": order_api_spec(),
        "channel": "nonexistent_channel",
        "message": "NewOrder",
        "payload": { "order_id": "ORD-001", "amount": 99.99, "currency": "USD" }
    });

    let response = server.post("/asyncapi/validate").json(&request_body).await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    let body: Value = response.json();
    assert_eq!(body["status"], 400);
}

// ── GET /asyncapi.json ────────────────────────────────────────────────────────

#[tokio::test]
async fn get_asyncapi_json_returns_registered_spec() {
    let spec = minimal_valid_spec();
    let server = build_server_with_asyncapi(Some(spec.clone()));

    let response = server.get("/asyncapi.json").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["asyncapi"], "3.0.0");
    assert_eq!(body["info"]["title"], "Chat API");
}

#[tokio::test]
async fn get_asyncapi_json_without_registered_spec_returns_404_problem_details() {
    // enabled=true but spec=None
    let server = build_server_with_asyncapi(None);

    let response = server.get("/asyncapi.json").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    let body: Value = response.json();
    assert_eq!(body["status"], 404);
}

#[tokio::test]
async fn get_asyncapi_json_using_with_asyncapi_spec_builder_method() {
    let spec = minimal_valid_spec();
    let config = ServerConfig::builder().with_asyncapi_spec(spec.clone()).build();
    let router = build_router_with_handlers_and_config(vec![], config, vec![]).expect("router built");
    let server = axum_test::TestServer::new(router);

    let response = server.get("/asyncapi.json").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let body: Value = response.json();
    assert_eq!(body["info"]["title"], "Chat API");
}
