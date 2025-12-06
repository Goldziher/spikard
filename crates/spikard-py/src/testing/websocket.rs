//! WebSocket test client bindings for Python

use crate::conversion::{json_to_python, python_to_json};
use axum_test::TestServer as AxumTestServer;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use serde_json::Value;
use spikard_http::testing::{
    WebSocketConnection as RustWebSocketConnection, WebSocketMessage as RustWebSocketMessage, connect_websocket,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Python wrapper for WebSocket test client
#[pyclass]
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

#[pymethods]
impl WebSocketTestConnection {
    /// Send a text message
    fn send_text<'py>(&self, py: Python<'py>, text: &str) -> PyResult<Bound<'py, PyAny>> {
        let text = text.to_string();
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            ws.send_text(text).await;
            Ok(())
        })
    }

    /// Send a JSON message
    fn send_json<'py>(&self, py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let json_value = python_to_json(py, obj)?;
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            ws.send_json(&json_value).await;
            Ok(())
        })
    }

    /// Receive a text message
    fn receive_text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            let text = ws.receive_text().await;
            Ok(text)
        })
    }

    /// Receive a JSON message
    fn receive_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            let json_value: Value = ws.receive_json().await;
            Python::attach(|py| json_to_python(py, &json_value).map(|obj| obj.unbind()))
        })
    }

    /// Receive raw bytes
    fn receive_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            let bytes = ws.receive_bytes().await;
            Ok(bytes.to_vec())
        })
    }

    /// Receive a message and return WebSocketMessage
    fn receive_message<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut ws = inner.lock().await;
            let msg = ws.receive_message().await;
            Ok(WebSocketMessage::from_rust(msg))
        })
    }

    /// Close the WebSocket connection
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = Arc::clone(&self.inner);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            {
                let guard = inner.lock().await;
                drop(guard);
            };
            Ok(())
        })
    }
}

/// Python wrapper for WebSocket messages
#[pyclass]
#[derive(Clone)]
pub struct WebSocketMessage {
    inner: RustWebSocketMessage,
}

impl WebSocketMessage {
    fn from_rust(msg: RustWebSocketMessage) -> Self {
        Self { inner: msg }
    }
}

#[pymethods]
impl WebSocketMessage {
    /// Get message as text if it's a text message
    fn as_text(&self) -> Option<String> {
        self.inner.as_text().map(|s| s.to_string())
    }

    /// Get message as JSON if it's a text message containing JSON
    fn as_json(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        match self.inner.as_json() {
            Ok(value) => Ok(Some(json_to_python(py, &value)?.unbind())),
            Err(_) => Ok(None),
        }
    }

    /// Get message as binary if it's a binary message
    fn as_binary<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.inner.as_binary().map(|bytes| PyBytes::new(py, bytes))
    }

    /// Check if this is a close message
    fn is_close(&self) -> bool {
        self.inner.is_close()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Connect to a WebSocket endpoint for testing
pub async fn connect_websocket_for_test(server: &AxumTestServer, path: &str) -> PyResult<WebSocketTestConnection> {
    let ws = connect_websocket(server, path).await;
    Ok(WebSocketTestConnection::new(ws))
}
