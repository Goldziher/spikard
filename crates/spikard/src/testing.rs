//! Testing utilities for Spikard applications.
//!
//! This module provides the [`TestClient`] type for testing HTTP endpoints,
//! WebSocket connections, and Server-Sent Events in Spikard applications
//! without a real network connection.
//!
//! # Example
//!
//! ```rust,no_run
//! # use spikard::{App, testing::{test_client_from_app, TestClient}};
//! # use spikard_http::testing::SnapshotError;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut app = App::new();
//! // ... register routes ...
//! let client: TestClient = test_client_from_app(app)?;
//! let response = client.get("/", None, None).await?;
//! assert_eq!(response.status, 200);
//! # Ok(())
//! # }
//! ```

use super::{App, AppError};
use axum_test::{TestServer as AxumTestServer, TestServerConfig, Transport};

pub use spikard_http::testing::SseEvent as TestSseEvent;
pub use spikard_http::testing::{
    GraphQLSubscriptionSnapshot, MultipartFilePart, ResponseSnapshot, SnapshotError, SseStream, TestClient,
    WebSocketConnection, WebSocketMessage, build_multipart_body, encode_urlencoded_body,
};

/// Construct a [`TestClient`] from a fully-built [`App`].
///
/// This is the canonical entry point for testing a Spikard application.
/// It converts the application into an Axum router and wraps it in a
/// [`TestClient`] that dispatches requests in-process without a network.
///
/// # Errors
///
/// Returns an error if the application router cannot be built (invalid route
/// configuration) or if the test server cannot be initialized.
pub fn test_client_from_app(app: App) -> Result<TestClient, SnapshotError> {
    let router = app
        .into_router()
        .map_err(|e| SnapshotError::Decompression(format!("Failed to build app router: {e}")))?;
    TestClient::from_router(router).map_err(SnapshotError::Decompression)
}

/// Spikard-native test server wrapper that hides the Axum test harness.
///
/// Tests can build an `App`, convert it into a `TestServer`, and then issue
/// HTTP/SSE/WebSocket requests without touching `axum-test` directly.
pub struct TestServer {
    mock_server: AxumTestServer,
    http_server: AxumTestServer,
}

impl TestServer {
    /// Build a test server from an `App`.
    ///
    /// # Errors
    ///
    /// Returns an error if the application router construction fails.
    pub fn from_app(app: App) -> Result<Self, AppError> {
        let router = app.into_router()?;
        Self::from_router(router)
    }

    /// Build a test server from an Axum router.
    ///
    /// # Errors
    ///
    /// Returns an error if test server construction fails.
    pub fn from_router(router: axum::Router) -> Result<Self, AppError> {
        let mock_server = AxumTestServer::try_new(router.clone()).map_err(|err| AppError::Server(err.to_string()))?;
        let config = TestServerConfig {
            transport: Some(Transport::HttpRandomPort),
            ..Default::default()
        };
        let http_server =
            AxumTestServer::try_new_with_config(router, config).map_err(|err| AppError::Server(err.to_string()))?;
        Ok(Self {
            mock_server,
            http_server,
        })
    }

    /// Execute an HTTP request and return a snapshot of the response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request execution or response snapshot fails.
    pub async fn call(
        &self,
        request: axum::http::Request<axum::body::Body>,
    ) -> Result<ResponseSnapshot, SnapshotError> {
        let response = spikard_http::testing::call_test_server(&self.mock_server, request).await;
        spikard_http::testing::snapshot_response(response).await
    }

    /// Open a WebSocket connection for the provided path.
    pub async fn connect_websocket(&self, path: &str) -> WebSocketConnection {
        spikard_http::testing::connect_websocket(&self.http_server, path).await
    }
}
