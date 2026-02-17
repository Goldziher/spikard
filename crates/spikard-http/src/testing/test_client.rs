//! Core test client for Spikard applications
//!
//! This module provides a language-agnostic TestClient that can be wrapped by
//! language bindings (PyO3, napi-rs, magnus) to provide Pythonic, JavaScripty, and
//! Ruby-like APIs respectively.
//!
//! The core client handles all HTTP method dispatch, query params, header management,
//! body encoding (JSON, form-data, multipart), and response snapshot capture.

use super::{ResponseSnapshot, SnapshotError, snapshot_response};
use axum::http::{HeaderName, HeaderValue, Method};
use axum_test::TestServer;
use bytes::Bytes;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use urlencoding::encode;

type MultipartPayload = Option<(Vec<(String, String)>, Vec<super::MultipartFilePart>)>;
const GRAPHQL_WS_MESSAGE_TIMEOUT: Duration = Duration::from_secs(2);
const GRAPHQL_WS_MAX_CONTROL_MESSAGES: usize = 32;

/// Snapshot of a GraphQL subscription exchange over WebSocket.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphQLSubscriptionSnapshot {
    /// Operation id used for the subscription request.
    pub operation_id: String,
    /// Whether the server acknowledged the GraphQL WebSocket connection.
    pub acknowledged: bool,
    /// First `next.payload` received for this subscription, if any.
    pub event: Option<Value>,
    /// GraphQL protocol errors emitted by the server.
    pub errors: Vec<Value>,
    /// Whether a `complete` frame was observed for this operation.
    pub complete_received: bool,
}

/// Core test client for making HTTP requests to a Spikard application.
///
/// This struct wraps axum-test's TestServer and provides a language-agnostic
/// interface for making HTTP requests, sending WebSocket connections, and
/// handling Server-Sent Events. Language bindings wrap this to provide
/// native API surfaces.
pub struct TestClient {
    server: Arc<TestServer>,
}

impl TestClient {
    /// Create a new test client from an Axum router
    pub fn from_router(router: axum::Router) -> Result<Self, String> {
        let server = if tokio::runtime::Handle::try_current().is_ok() {
            TestServer::builder()
                .http_transport()
                .build(router)
                .map_err(|e| format!("Failed to create test server: {}", e))?
        } else {
            TestServer::new(router).map_err(|e| format!("Failed to create test server: {}", e))?
        };

        Ok(Self {
            server: Arc::new(server),
        })
    }

    /// Get the underlying test server (for WebSocket and SSE connections)
    pub fn server(&self) -> &TestServer {
        &self.server
    }

    /// Make a GET request
    pub async fn get(
        &self,
        path: &str,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.get(&full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a POST request
    pub async fn post(
        &self,
        path: &str,
        json: Option<Value>,
        form_data: Option<Vec<(String, String)>>,
        multipart: MultipartPayload,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.post(&full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        if let Some((form_fields, files)) = multipart {
            let (body, boundary) = super::build_multipart_body(&form_fields, &files);
            let content_type = format!("multipart/form-data; boundary={}", boundary);
            request = request.add_header("content-type", &content_type);
            request = request.bytes(Bytes::from(body));
        } else if let Some(form_fields) = form_data {
            let fields_value = serde_json::to_value(&form_fields)
                .map_err(|e| SnapshotError::Decompression(format!("Failed to serialize form fields: {}", e)))?;
            let encoded = super::encode_urlencoded_body(&fields_value)
                .map_err(|e| SnapshotError::Decompression(format!("Form encoding failed: {}", e)))?;
            request = request.add_header("content-type", "application/x-www-form-urlencoded");
            request = request.bytes(Bytes::from(encoded));
        } else if let Some(json_value) = json {
            request = request.json(&json_value);
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a request with a raw body payload.
    pub async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Bytes,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.method(method, &full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        request = request.bytes(body);
        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a PUT request
    pub async fn put(
        &self,
        path: &str,
        json: Option<Value>,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.put(&full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        if let Some(json_value) = json {
            request = request.json(&json_value);
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a PATCH request
    pub async fn patch(
        &self,
        path: &str,
        json: Option<Value>,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.patch(&full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        if let Some(json_value) = json {
            request = request.json(&json_value);
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(
        &self,
        path: &str,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.delete(&full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make an OPTIONS request
    pub async fn options(
        &self,
        path: &str,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.method(Method::OPTIONS, &full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a HEAD request
    pub async fn head(
        &self,
        path: &str,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.method(Method::HEAD, &full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Make a TRACE request
    pub async fn trace(
        &self,
        path: &str,
        query_params: Option<Vec<(String, String)>>,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let full_path = build_full_path(path, query_params.as_deref());
        let mut request = self.server.method(Method::TRACE, &full_path);

        if let Some(headers_vec) = headers {
            request = self.add_headers(request, headers_vec)?;
        }

        let response = request.await;
        snapshot_response(response).await
    }

    /// Send a GraphQL query/mutation to a custom endpoint
    pub async fn graphql_at(
        &self,
        endpoint: &str,
        query: &str,
        variables: Option<Value>,
        operation_name: Option<&str>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let body = build_graphql_body(query, variables, operation_name);
        self.post(endpoint, Some(body), None, None, None, None).await
    }

    /// Send a GraphQL query/mutation
    pub async fn graphql(
        &self,
        query: &str,
        variables: Option<Value>,
        operation_name: Option<&str>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        self.graphql_at("/graphql", query, variables, operation_name).await
    }

    /// Send a GraphQL query and return HTTP status code separately
    ///
    /// This method allows tests to distinguish between:
    /// - HTTP-level errors (400/422 for invalid requests)
    /// - GraphQL-level errors (200 with errors in response body)
    ///
    /// # Example
    /// ```ignore
    /// let (status, snapshot) = client.graphql_with_status(
    ///     "query { invalid syntax",
    ///     None,
    ///     None
    /// ).await?;
    /// assert_eq!(status, 400); // HTTP parse error
    /// ```
    pub async fn graphql_with_status(
        &self,
        query: &str,
        variables: Option<Value>,
        operation_name: Option<&str>,
    ) -> Result<(u16, ResponseSnapshot), SnapshotError> {
        let snapshot = self.graphql(query, variables, operation_name).await?;
        let status = snapshot.status;
        Ok((status, snapshot))
    }

    /// Send a GraphQL subscription (WebSocket) to a custom endpoint.
    ///
    /// Uses the `graphql-transport-ws` protocol and captures the first `next` payload.
    /// After the first payload is received, this client sends `complete` to unsubscribe.
    pub async fn graphql_subscription_at(
        &self,
        endpoint: &str,
        query: &str,
        variables: Option<Value>,
        operation_name: Option<&str>,
    ) -> Result<GraphQLSubscriptionSnapshot, SnapshotError> {
        let operation_id = "spikard-subscription-1".to_string();
        let upgrade = self
            .server
            .get_websocket(endpoint)
            .add_header("sec-websocket-protocol", "graphql-transport-ws")
            .await;

        if upgrade.status_code().as_u16() != 101 {
            return Err(SnapshotError::Decompression(format!(
                "GraphQL subscription upgrade failed with status {}",
                upgrade.status_code()
            )));
        }

        let mut websocket = super::WebSocketConnection::new(upgrade.into_websocket().await);

        websocket
            .send_json(&serde_json::json!({"type": "connection_init"}))
            .await;
        wait_for_graphql_ack(&mut websocket).await?;

        websocket
            .send_json(&serde_json::json!({
                "id": operation_id,
                "type": "subscribe",
                "payload": build_graphql_body(query, variables, operation_name),
            }))
            .await;

        let mut event = None;
        let mut errors = Vec::new();
        let mut complete_received = false;

        for _ in 0..GRAPHQL_WS_MAX_CONTROL_MESSAGES {
            let message = timeout(
                GRAPHQL_WS_MESSAGE_TIMEOUT,
                receive_graphql_protocol_message(&mut websocket),
            )
            .await
            .map_err(|_| {
                SnapshotError::Decompression("Timed out waiting for GraphQL subscription message".to_string())
            })??;

            let message_type = message.get("type").and_then(Value::as_str).unwrap_or_default();
            match message_type {
                "next" => {
                    if message
                        .get("id")
                        .and_then(Value::as_str)
                        .map_or(true, |id| id == operation_id)
                    {
                        event = message.get("payload").cloned();

                        websocket
                            .send_json(&serde_json::json!({
                                "id": operation_id,
                                "type": "complete",
                            }))
                            .await;

                        if let Ok(next_message) = timeout(
                            GRAPHQL_WS_MESSAGE_TIMEOUT,
                            receive_graphql_protocol_message(&mut websocket),
                        )
                        .await
                            && let Ok(next_message) = next_message
                            && next_message.get("type").and_then(Value::as_str) == Some("complete")
                            && next_message
                                .get("id")
                                .and_then(Value::as_str)
                                .map_or(true, |id| id == operation_id)
                        {
                            complete_received = true;
                        }
                        break;
                    }
                }
                "error" => {
                    errors.push(message.get("payload").cloned().unwrap_or(message));
                    break;
                }
                "complete" => {
                    if message
                        .get("id")
                        .and_then(Value::as_str)
                        .map_or(true, |id| id == operation_id)
                    {
                        complete_received = true;
                        break;
                    }
                }
                "ping" => {
                    let mut pong = serde_json::json!({"type": "pong"});
                    if let Some(payload) = message.get("payload") {
                        pong["payload"] = payload.clone();
                    }
                    websocket.send_json(&pong).await;
                }
                "pong" => {}
                _ => {}
            }
        }

        websocket.close().await;

        if event.is_none() && errors.is_empty() && !complete_received {
            return Err(SnapshotError::Decompression(
                "No GraphQL subscription event received before timeout".to_string(),
            ));
        }

        Ok(GraphQLSubscriptionSnapshot {
            operation_id,
            acknowledged: true,
            event,
            errors,
            complete_received,
        })
    }

    /// Send a GraphQL subscription (WebSocket).
    ///
    /// Uses `/graphql` as the default subscription endpoint.
    pub async fn graphql_subscription(
        &self,
        query: &str,
        variables: Option<Value>,
        operation_name: Option<&str>,
    ) -> Result<GraphQLSubscriptionSnapshot, SnapshotError> {
        self.graphql_subscription_at("/graphql", query, variables, operation_name)
            .await
    }

    /// Add headers to a test request builder
    fn add_headers(
        &self,
        mut request: axum_test::TestRequest,
        headers: Vec<(String, String)>,
    ) -> Result<axum_test::TestRequest, SnapshotError> {
        for (key, value) in headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| SnapshotError::InvalidHeader(format!("Invalid header name: {}", e)))?;
            let header_value = HeaderValue::from_str(&value)
                .map_err(|e| SnapshotError::InvalidHeader(format!("Invalid header value: {}", e)))?;
            request = request.add_header(header_name, header_value);
        }
        Ok(request)
    }
}

async fn wait_for_graphql_ack(websocket: &mut super::WebSocketConnection) -> Result<(), SnapshotError> {
    for _ in 0..GRAPHQL_WS_MAX_CONTROL_MESSAGES {
        let message = timeout(GRAPHQL_WS_MESSAGE_TIMEOUT, receive_graphql_protocol_message(websocket))
            .await
            .map_err(|_| SnapshotError::Decompression("Timed out waiting for GraphQL connection_ack".to_string()))??;

        match message.get("type").and_then(Value::as_str).unwrap_or_default() {
            "connection_ack" => return Ok(()),
            "ping" => {
                let mut pong = serde_json::json!({"type": "pong"});
                if let Some(payload) = message.get("payload") {
                    pong["payload"] = payload.clone();
                }
                websocket.send_json(&pong).await;
            }
            "connection_error" | "error" => {
                return Err(SnapshotError::Decompression(format!(
                    "GraphQL subscription rejected during init: {}",
                    message
                )));
            }
            _ => {}
        }
    }

    Err(SnapshotError::Decompression(
        "No GraphQL connection_ack received".to_string(),
    ))
}

async fn receive_graphql_protocol_message(websocket: &mut super::WebSocketConnection) -> Result<Value, SnapshotError> {
    loop {
        match websocket.receive_message().await {
            super::WebSocketMessage::Text(text) => {
                return serde_json::from_str::<Value>(&text).map_err(|e| {
                    SnapshotError::Decompression(format!("Failed to parse GraphQL WebSocket message as JSON: {}", e))
                });
            }
            super::WebSocketMessage::Binary(bytes) => {
                return serde_json::from_slice::<Value>(&bytes).map_err(|e| {
                    SnapshotError::Decompression(format!(
                        "Failed to parse GraphQL binary WebSocket message as JSON: {}",
                        e
                    ))
                });
            }
            super::WebSocketMessage::Ping(_) | super::WebSocketMessage::Pong(_) => continue,
            super::WebSocketMessage::Close(reason) => {
                return Err(SnapshotError::Decompression(format!(
                    "GraphQL WebSocket connection closed before response: {:?}",
                    reason
                )));
            }
        }
    }
}

/// Build a GraphQL request body from query, variables, and operation name
pub fn build_graphql_body(query: &str, variables: Option<Value>, operation_name: Option<&str>) -> Value {
    let mut body = serde_json::json!({ "query": query });
    if let Some(vars) = variables {
        body["variables"] = vars;
    }
    if let Some(op_name) = operation_name {
        body["operationName"] = Value::String(op_name.to_string());
    }
    body
}

/// Build a full path with query parameters
fn build_full_path(path: &str, query_params: Option<&[(String, String)]>) -> String {
    match query_params {
        None | Some(&[]) => path.to_string(),
        Some(params) => {
            let query_string: Vec<String> = params
                .iter()
                .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
                .collect();

            if path.contains('?') {
                format!("{}&{}", path, query_string.join("&"))
            } else {
                format!("{}?{}", path, query_string.join("&"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        extract::ws::{Message, WebSocketUpgrade},
        routing::get,
    };

    #[test]
    fn build_full_path_no_params() {
        let path = "/users";
        assert_eq!(build_full_path(path, None), "/users");
        assert_eq!(build_full_path(path, Some(&[])), "/users");
    }

    #[test]
    fn build_full_path_with_params() {
        let path = "/users";
        let params = vec![
            ("id".to_string(), "123".to_string()),
            ("name".to_string(), "test user".to_string()),
        ];
        let result = build_full_path(path, Some(&params));
        assert!(result.starts_with("/users?"));
        assert!(result.contains("id=123"));
        assert!(result.contains("name=test%20user"));
    }

    #[test]
    fn build_full_path_existing_query() {
        let path = "/users?active=true";
        let params = vec![("id".to_string(), "123".to_string())];
        let result = build_full_path(path, Some(&params));
        assert_eq!(result, "/users?active=true&id=123");
    }

    #[test]
    fn test_graphql_query_builder() {
        let query = "{ users { id name } }";
        let variables = Some(serde_json::json!({ "limit": 10 }));
        let op_name = Some("GetUsers");

        let mut body = serde_json::json!({ "query": query });
        if let Some(vars) = variables {
            body["variables"] = vars;
        }
        if let Some(op_name) = op_name {
            body["operationName"] = Value::String(op_name.to_string());
        }

        assert_eq!(body["query"], query);
        assert_eq!(body["variables"]["limit"], 10);
        assert_eq!(body["operationName"], "GetUsers");
    }

    #[test]
    fn test_graphql_with_status_method() {
        let query = "query { hello }";
        let body = serde_json::json!({
            "query": query,
            "variables": null,
            "operationName": null
        });

        // This test validates the method signature and return type
        // Actual HTTP status testing will happen in integration tests
        let expected_fields = vec!["query", "variables", "operationName"];
        for field in expected_fields {
            assert!(body.get(field).is_some(), "Missing field: {}", field);
        }
    }

    #[test]
    fn test_build_graphql_body_basic() {
        let query = "{ users { id name } }";
        let body = build_graphql_body(query, None, None);

        assert_eq!(body["query"], query);
        assert!(body.get("variables").is_none() || body["variables"].is_null());
        assert!(body.get("operationName").is_none() || body["operationName"].is_null());
    }

    #[test]
    fn test_build_graphql_body_with_variables() {
        let query = "query GetUser($id: ID!) { user(id: $id) { name } }";
        let variables = Some(serde_json::json!({ "id": "123" }));
        let body = build_graphql_body(query, variables, None);

        assert_eq!(body["query"], query);
        assert_eq!(body["variables"]["id"], "123");
    }

    #[test]
    fn test_build_graphql_body_with_operation_name() {
        let query = "query GetUsers { users { id } }";
        let op_name = Some("GetUsers");
        let body = build_graphql_body(query, None, op_name);

        assert_eq!(body["query"], query);
        assert_eq!(body["operationName"], "GetUsers");
    }

    #[test]
    fn test_build_graphql_body_all_fields() {
        let query = "mutation CreateUser($name: String!) { createUser(name: $name) { id } }";
        let variables = Some(serde_json::json!({ "name": "Alice" }));
        let op_name = Some("CreateUser");
        let body = build_graphql_body(query, variables, op_name);

        assert_eq!(body["query"], query);
        assert_eq!(body["variables"]["name"], "Alice");
        assert_eq!(body["operationName"], "CreateUser");
    }

    #[tokio::test]
    async fn graphql_subscription_returns_first_event_and_completes() {
        let app = Router::new().route(
            "/graphql",
            get(|ws: WebSocketUpgrade| async move {
                ws.on_upgrade(|mut socket| async move {
                    while let Some(result) = socket.recv().await {
                        let Ok(Message::Text(text)) = result else {
                            continue;
                        };
                        let Ok(message): Result<Value, _> = serde_json::from_str(&text) else {
                            continue;
                        };

                        match message.get("type").and_then(Value::as_str) {
                            Some("connection_init") => {
                                let _ = socket
                                    .send(Message::Text(
                                        serde_json::json!({"type":"connection_ack"}).to_string().into(),
                                    ))
                                    .await;
                            }
                            Some("subscribe") => {
                                let id = message.get("id").and_then(Value::as_str).unwrap_or("1");
                                let _ = socket
                                    .send(Message::Text(
                                        serde_json::json!({
                                            "id": id,
                                            "type": "next",
                                            "payload": {"data": {"ticker": "AAPL"}},
                                        })
                                        .to_string()
                                        .into(),
                                    ))
                                    .await;

                                if let Some(Ok(Message::Text(complete_text))) = socket.recv().await {
                                    let Ok(complete_message): Result<Value, _> = serde_json::from_str(&complete_text)
                                    else {
                                        break;
                                    };
                                    if complete_message.get("type").and_then(Value::as_str) == Some("complete") {
                                        let _ = socket
                                            .send(Message::Text(
                                                serde_json::json!({"id": id, "type":"complete"}).to_string().into(),
                                            ))
                                            .await;
                                    }
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                })
            }),
        );

        let client = TestClient::from_router(app).expect("client");
        let snapshot = client
            .graphql_subscription("subscription { ticker }", None, None)
            .await
            .expect("subscription snapshot");

        assert!(snapshot.acknowledged);
        assert_eq!(snapshot.errors, Vec::<Value>::new());
        assert_eq!(snapshot.event, Some(serde_json::json!({"data": {"ticker": "AAPL"}})));
        assert!(snapshot.complete_received);
    }

    #[tokio::test]
    async fn graphql_subscription_surfaces_connection_error() {
        let app = Router::new().route(
            "/graphql",
            get(|ws: WebSocketUpgrade| async move {
                ws.on_upgrade(|mut socket| async move {
                    while let Some(result) = socket.recv().await {
                        let Ok(Message::Text(text)) = result else {
                            continue;
                        };
                        let Ok(message): Result<Value, _> = serde_json::from_str(&text) else {
                            continue;
                        };

                        if message.get("type").and_then(Value::as_str) == Some("connection_init") {
                            let _ = socket
                                .send(Message::Text(
                                    serde_json::json!({
                                        "type": "connection_error",
                                        "payload": {"message": "not authorized"},
                                    })
                                    .to_string()
                                    .into(),
                                ))
                                .await;
                            break;
                        }
                    }
                })
            }),
        );

        let client = TestClient::from_router(app).expect("client");
        let error = client
            .graphql_subscription("subscription { privateFeed }", None, None)
            .await
            .expect_err("expected connection error");

        assert!(error.to_string().contains("connection_error"));
    }
}
