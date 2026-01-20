//! WebSocket test client bindings for Ruby

use magnus::prelude::*;
use magnus::{Error, Ruby, Value, method};
use serde_json::Value as JsonValue;
use std::cell::RefCell;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};
use url::Url;

#[derive(Debug)]
pub enum WebSocketIoError {
    Timeout,
    Closed,
    Other(String),
}

#[derive(Debug)]
pub struct WebSocketConnection {
    stream: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    pub(crate) fn connect(url: Url, timeout: Duration) -> Result<Self, WebSocketIoError> {
        if url.scheme() == "wss" {
            return Err(WebSocketIoError::Other(
                "wss is not supported for Ruby test sockets".to_string(),
            ));
        }

        let host = url
            .host_str()
            .ok_or_else(|| WebSocketIoError::Other("Missing WebSocket host".to_string()))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| WebSocketIoError::Other("Missing WebSocket port".to_string()))?;
        let addr = (host, port)
            .to_socket_addrs()
            .map_err(|err| WebSocketIoError::Other(err.to_string()))?
            .next()
            .ok_or_else(|| WebSocketIoError::Other("Unable to resolve WebSocket host".to_string()))?;
        let tcp_stream = TcpStream::connect_timeout(&addr, timeout).map_err(map_io_error)?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|err| WebSocketIoError::Other(err.to_string()))?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|err| WebSocketIoError::Other(err.to_string()))?;

        let request = url.as_str();
        let (stream, _) = tungstenite::client::client(request, MaybeTlsStream::Plain(tcp_stream))
            .map_err(|err| WebSocketIoError::Other(err.to_string()))?;
        Ok(Self { stream })
    }

    pub(crate) fn send_text(&mut self, text: String) -> Result<(), WebSocketIoError> {
        self.stream
            .write_message(Message::Text(text.into()))
            .map_err(map_tungstenite_error)
    }

    pub(crate) fn send_json(&mut self, json_value: &JsonValue) -> Result<(), WebSocketIoError> {
        let text = serde_json::to_string(json_value).map_err(|err| WebSocketIoError::Other(err.to_string()))?;
        self.send_text(text)
    }

    pub(crate) fn receive_message(&mut self) -> Result<Message, WebSocketIoError> {
        match self.stream.read_message() {
            Ok(message) => Ok(message),
            Err(err) => Err(map_tungstenite_error(err)),
        }
    }

    pub(crate) fn receive_text(&mut self) -> Result<String, WebSocketIoError> {
        let message = self.receive_message()?;
        message_to_text(message)
    }

    pub(crate) fn receive_json(&mut self) -> Result<JsonValue, WebSocketIoError> {
        let text = self.receive_text()?;
        serde_json::from_str(&text).map_err(|err| WebSocketIoError::Other(err.to_string()))
    }

    pub(crate) fn receive_bytes(&mut self) -> Result<bytes::Bytes, WebSocketIoError> {
        let message = self.receive_message()?;
        message_to_bytes(message)
    }

    pub(crate) fn close(mut self) -> Result<(), WebSocketIoError> {
        self.stream.close(None).map_err(map_tungstenite_error)
    }
}

fn map_tungstenite_error(err: tungstenite::Error) -> WebSocketIoError {
    match err {
        tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed => WebSocketIoError::Closed,
        tungstenite::Error::Io(io_err) => match io_err.kind() {
            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => WebSocketIoError::Timeout,
            _ => WebSocketIoError::Other(io_err.to_string()),
        },
        other => WebSocketIoError::Other(other.to_string()),
    }
}

fn map_io_error(err: std::io::Error) -> WebSocketIoError {
    match err.kind() {
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => WebSocketIoError::Timeout,
        _ => WebSocketIoError::Other(err.to_string()),
    }
}

fn message_to_text(message: Message) -> Result<String, WebSocketIoError> {
    match message {
        Message::Text(text) => Ok(text.to_string()),
        Message::Binary(bytes) => {
            String::from_utf8(bytes.to_vec()).map_err(|err| WebSocketIoError::Other(err.to_string()))
        }
        Message::Close(frame) => Ok(frame.map(|f| f.reason.to_string()).unwrap_or_default()),
        Message::Ping(_) | Message::Pong(_) => Ok(String::new()),
        Message::Frame(_) => Err(WebSocketIoError::Other(
            "Unexpected frame message while reading text".to_string(),
        )),
    }
}

fn message_to_bytes(message: Message) -> Result<bytes::Bytes, WebSocketIoError> {
    match message {
        Message::Text(text) => Ok(bytes::Bytes::from(text.to_string())),
        Message::Binary(bytes) => Ok(bytes),
        Message::Close(frame) => Ok(bytes::Bytes::from(
            frame.map(|f| f.reason.to_string()).unwrap_or_default(),
        )),
        Message::Ping(data) => Ok(data),
        Message::Pong(data) => Ok(data),
        Message::Frame(_) => Err(WebSocketIoError::Other(
            "Unexpected frame message while reading bytes".to_string(),
        )),
    }
}

/// Ruby wrapper for WebSocket test client
#[derive(Default)]
#[magnus::wrap(class = "Spikard::Native::WebSocketTestConnection", free_immediately)]
pub struct WebSocketTestConnection {
    inner: RefCell<Option<WebSocketConnection>>,
}

impl WebSocketTestConnection {
    /// Create a new WebSocket test connection (public for lib.rs)
    pub(crate) fn new(inner: WebSocketConnection) -> Self {
        Self {
            inner: RefCell::new(Some(inner)),
        }
    }

    /// Send a text message
    fn send_text(&self, text: String) -> Result<(), Error> {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let result = crate::call_without_gvl!(
            block_on_send_text,
            args: (timeout_duration, Duration, ws, &mut WebSocketConnection, text, String),
            return_type: Result<(), WebSocketIoError>
        );
        match result {
            Ok(()) => Ok(()),
            Err(WebSocketIoError::Timeout) => Err(Error::new(
                magnus::exception::runtime_error(),
                format!("WebSocket send timed out after {}ms", timeout_duration.as_millis()),
            )),
            Err(WebSocketIoError::Closed) => Err(Error::new(
                magnus::exception::runtime_error(),
                "WebSocket connection closed".to_string(),
            )),
            Err(WebSocketIoError::Other(message)) => Err(Error::new(
                magnus::exception::runtime_error(),
                format!("WebSocket send failed: {}", message),
            )),
        }?;

        Ok(())
    }

    /// Send a JSON message
    fn send_json(ruby: &Ruby, this: &Self, obj: Value) -> Result<(), Error> {
        let json_value = ruby_to_json(ruby, obj)?;
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let json_value_ref = &json_value;
        let result = crate::call_without_gvl!(
            block_on_send_json,
            args: (
                timeout_duration, Duration,
                ws, &mut WebSocketConnection,
                json_value_ref, &JsonValue
            ),
            return_type: Result<(), WebSocketIoError>
        );
        match result {
            Ok(()) => Ok(()),
            Err(WebSocketIoError::Timeout) => Err(Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket send timed out after {}ms", timeout_duration.as_millis()),
            )),
            Err(WebSocketIoError::Closed) => Err(Error::new(
                ruby.exception_runtime_error(),
                "WebSocket connection closed".to_string(),
            )),
            Err(WebSocketIoError::Other(message)) => Err(Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket send failed: {}", message),
            )),
        }?;

        Ok(())
    }

    /// Receive a text message
    fn receive_text(&self) -> Result<String, Error> {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let text = crate::call_without_gvl!(
            block_on_receive_text,
            args: (
                timeout_duration, Duration,
                ws, &mut WebSocketConnection
            ),
            return_type: Result<String, WebSocketIoError>
        )
        .map_err(|err| match err {
            WebSocketIoError::Timeout => Error::new(
                magnus::exception::runtime_error(),
                format!("WebSocket receive timed out after {}ms", timeout_duration.as_millis()),
            ),
            WebSocketIoError::Closed => Error::new(
                magnus::exception::runtime_error(),
                "WebSocket connection closed".to_string(),
            ),
            WebSocketIoError::Other(message) => Error::new(
                magnus::exception::runtime_error(),
                format!("WebSocket receive failed: {}", message),
            ),
        })?;

        Ok(text)
    }

    /// Receive and parse a JSON message
    fn receive_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let json_value = crate::call_without_gvl!(
            block_on_receive_json,
            args: (
                timeout_duration, Duration,
                ws, &mut WebSocketConnection
            ),
            return_type: Result<JsonValue, WebSocketIoError>
        )
        .map_err(|err| match err {
            WebSocketIoError::Timeout => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive timed out after {}ms", timeout_duration.as_millis()),
            ),
            WebSocketIoError::Closed => Error::new(
                ruby.exception_runtime_error(),
                "WebSocket connection closed".to_string(),
            ),
            WebSocketIoError::Other(message) => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive failed: {}", message),
            ),
        })?;

        json_to_ruby(ruby, &json_value)
    }

    /// Receive raw bytes
    fn receive_bytes(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let result = crate::call_without_gvl!(
            block_on_receive_bytes,
            args: (
                timeout_duration, Duration,
                ws, &mut WebSocketConnection
            ),
            return_type: Result<bytes::Bytes, WebSocketIoError>
        );
        let bytes = result.map_err(|err| match err {
            WebSocketIoError::Timeout => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive timed out after {}ms", timeout_duration.as_millis()),
            ),
            WebSocketIoError::Closed => Error::new(
                ruby.exception_runtime_error(),
                "WebSocket connection closed".to_string(),
            ),
            WebSocketIoError::Other(message) => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive failed: {}", message),
            ),
        })?;

        Ok(ruby.str_from_slice(&bytes).as_value())
    }

    /// Receive a message and return WebSocketMessage
    fn receive_message(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut inner = this.inner.borrow_mut();
        let ws = inner
            .as_mut()
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "WebSocket closed"))?;

        let timeout_duration = websocket_timeout();
        let result = crate::call_without_gvl!(
            block_on_receive_message,
            args: (
                timeout_duration, Duration,
                ws, &mut WebSocketConnection
            ),
            return_type: Result<WebSocketMessageData, WebSocketIoError>
        );
        let msg = result.map_err(|err| match err {
            WebSocketIoError::Timeout => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive timed out after {}ms", timeout_duration.as_millis()),
            ),
            WebSocketIoError::Closed => Error::new(
                ruby.exception_runtime_error(),
                "WebSocket connection closed".to_string(),
            ),
            WebSocketIoError::Other(message) => Error::new(
                ruby.exception_runtime_error(),
                format!("WebSocket receive failed: {}", message),
            ),
        })?;

        let ws_msg = WebSocketMessage::new(msg);
        Ok(ruby.obj_wrap(ws_msg).as_value())
    }

    /// Close the WebSocket connection
    fn close(&self) -> Result<(), Error> {
        let mut inner = self.inner.borrow_mut();
        let ws = inner
            .take()
            .ok_or_else(|| Error::new(magnus::exception::runtime_error(), "WebSocket closed"))?;
        let result = crate::call_without_gvl!(
            block_on_close,
            args: (ws, WebSocketConnection),
            return_type: Result<(), WebSocketIoError>
        );
        result.map_err(|err| match err {
            WebSocketIoError::Timeout => Error::new(
                magnus::exception::runtime_error(),
                "WebSocket close timed out".to_string(),
            ),
            WebSocketIoError::Closed => Error::new(
                magnus::exception::runtime_error(),
                "WebSocket connection closed".to_string(),
            ),
            WebSocketIoError::Other(message) => Error::new(
                magnus::exception::runtime_error(),
                format!("WebSocket close failed: {}", message),
            ),
        })
    }
}

fn block_on_send_text(
    timeout_duration: Duration,
    ws: &mut WebSocketConnection,
    text: String,
) -> Result<(), WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.send_text(text)
}

fn block_on_send_json(
    timeout_duration: Duration,
    ws: &mut WebSocketConnection,
    json_value: &JsonValue,
) -> Result<(), WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.send_json(json_value)
}

fn block_on_receive_bytes(
    timeout_duration: Duration,
    ws: &mut WebSocketConnection,
) -> Result<bytes::Bytes, WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.receive_bytes()
}

fn block_on_receive_text(timeout_duration: Duration, ws: &mut WebSocketConnection) -> Result<String, WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.receive_text()
}

fn block_on_receive_json(
    timeout_duration: Duration,
    ws: &mut WebSocketConnection,
) -> Result<JsonValue, WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.receive_json()
}

fn block_on_receive_message(
    timeout_duration: Duration,
    ws: &mut WebSocketConnection,
) -> Result<WebSocketMessageData, WebSocketIoError> {
    set_stream_timeouts(ws, timeout_duration)?;
    ws.receive_message().and_then(WebSocketMessageData::from_tungstenite)
}

fn block_on_close(ws: WebSocketConnection) -> Result<(), WebSocketIoError> {
    ws.close()
}

fn set_stream_timeouts(ws: &mut WebSocketConnection, timeout: Duration) -> Result<(), WebSocketIoError> {
    if let MaybeTlsStream::Plain(stream) = ws.stream.get_mut() {
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|e| WebSocketIoError::Other(e.to_string()))?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(|e| WebSocketIoError::Other(e.to_string()))?;
    }
    Ok(())
}

fn websocket_timeout() -> Duration {
    const DEFAULT_TIMEOUT_MS: u64 = 30_000;
    let timeout_ms = std::env::var("SPIKARD_RB_WS_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    Duration::from_millis(timeout_ms)
}

/// Ruby wrapper for WebSocket messages
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WebSocketMessageData {
    Text(String),
    Binary(Vec<u8>),
    Close(Option<String>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

impl WebSocketMessageData {
    fn from_tungstenite(message: Message) -> Result<Self, WebSocketIoError> {
        match message {
            Message::Text(text) => Ok(Self::Text(text.to_string())),
            Message::Binary(bytes) => Ok(Self::Binary(bytes.to_vec())),
            Message::Close(frame) => Ok(Self::Close(frame.map(|f| f.reason.to_string()))),
            Message::Ping(bytes) => Ok(Self::Ping(bytes.to_vec())),
            Message::Pong(bytes) => Ok(Self::Pong(bytes.to_vec())),
            Message::Frame(_) => Err(WebSocketIoError::Other(
                "Unexpected frame message while reading WebSocket".to_string(),
            )),
        }
    }
}

#[magnus::wrap(class = "Spikard::Native::WebSocketMessage", free_immediately)]
pub struct WebSocketMessage {
    inner: WebSocketMessageData,
}

impl WebSocketMessage {
    pub fn new(inner: WebSocketMessageData) -> Self {
        Self { inner }
    }

    /// Get message as text if it's a text message
    fn as_text(&self) -> Result<Option<String>, Error> {
        match &self.inner {
            WebSocketMessageData::Text(text) => Ok(Some(text.clone())),
            _ => Ok(None),
        }
    }

    /// Get message as JSON if it's a text message containing JSON
    fn as_json(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        match &this.inner {
            WebSocketMessageData::Text(text) => match serde_json::from_str::<JsonValue>(text) {
                Ok(value) => json_to_ruby(ruby, &value),
                Err(_) => Ok(ruby.qnil().as_value()),
            },
            _ => Ok(ruby.qnil().as_value()),
        }
    }

    /// Get message as binary if it's a binary message
    fn as_binary(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        match &this.inner {
            WebSocketMessageData::Binary(bytes) => Ok(ruby.str_from_slice(bytes).as_value()),
            WebSocketMessageData::Ping(bytes) => Ok(ruby.str_from_slice(bytes).as_value()),
            WebSocketMessageData::Pong(bytes) => Ok(ruby.str_from_slice(bytes).as_value()),
            _ => Ok(ruby.qnil().as_value()),
        }
    }

    /// Check if this is a close message
    fn is_close(&self) -> bool {
        matches!(self.inner, WebSocketMessageData::Close(_))
    }
}

/// Helper to convert Ruby object to JSON
fn ruby_to_json(ruby: &Ruby, value: Value) -> Result<JsonValue, Error> {
    let json_module = ruby.class_object().const_get::<_, magnus::RModule>("JSON")?;
    let json_str: String = json_module.funcall("generate", (value,))?;
    serde_json::from_str(&json_str).map_err(|e| {
        Error::new(
            magnus::exception::runtime_error(),
            format!("Failed to parse JSON: {}", e),
        )
    })
}

/// Helper to convert JSON to Ruby object
fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    crate::conversion::json_to_ruby(ruby, value)
}

/// Initialize WebSocket test client bindings
pub fn init(ruby: &Ruby, module: &magnus::RModule) -> Result<(), Error> {
    let native_module = module.define_module("Native")?;

    let ws_conn_class = native_module.define_class("WebSocketTestConnection", ruby.class_object())?;
    ws_conn_class.define_method("send_text", method!(WebSocketTestConnection::send_text, 1))?;
    ws_conn_class.define_method("send_json", method!(WebSocketTestConnection::send_json, 1))?;
    ws_conn_class.define_method("receive_text", method!(WebSocketTestConnection::receive_text, 0))?;
    ws_conn_class.define_method("receive_json", method!(WebSocketTestConnection::receive_json, 0))?;
    ws_conn_class.define_method("receive_bytes", method!(WebSocketTestConnection::receive_bytes, 0))?;
    ws_conn_class.define_method("receive_message", method!(WebSocketTestConnection::receive_message, 0))?;
    ws_conn_class.define_method("close", method!(WebSocketTestConnection::close, 0))?;

    let ws_msg_class = native_module.define_class("WebSocketMessage", ruby.class_object())?;
    ws_msg_class.define_method("as_text", method!(WebSocketMessage::as_text, 0))?;
    ws_msg_class.define_method("as_json", method!(WebSocketMessage::as_json, 0))?;
    ws_msg_class.define_method("as_binary", method!(WebSocketMessage::as_binary, 0))?;
    ws_msg_class.define_method("is_close", method!(WebSocketMessage::is_close, 0))?;

    Ok(())
}
