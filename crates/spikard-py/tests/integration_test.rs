//! Integration tests for PythonHandler
//!
//! These tests verify that PythonHandler correctly implements the Handler trait
//! and properly bridges Python code to Rust.

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use pyo3::prelude::*;
use serde_json::json;
use spikard_http::ServerConfig;
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::sync::Arc;

/// Helper function to initialize Python for tests
fn init_python() {
    pyo3::prepare_freethreaded_python();
    let _ = _spikard::init_python_event_loop();
}

#[test]
fn test_python_handler_creation() {
    init_python();

    Python::with_gil(|py| {
        // Create a simple Python function
        let code = r#"
def simple_handler(path_params, query_params, body, headers, cookies):
    return {"status_code": 200, "body": {"message": "Hello"}}
"#;

        let module = PyModule::from_code(py, code, "test.py", "test").unwrap();
        let handler_fn = module.getattr("simple_handler").unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();

        // Create PythonHandler
        let python_handler = _spikard::PythonHandler::new(
            handler_py, false, // not async
            None,  // no request validator
            None,  // no response validator
            None,  // no parameter validator
        );

        // Just verify it was created successfully
        assert!(std::mem::size_of_val(&python_handler) > 0);
    });
}

#[tokio::test]
async fn test_python_handler_sync_execution() {
    init_python();

    Python::with_gil(|py| {
        // Create a synchronous Python handler
        let code = r#"
def sync_handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {
            "message": "sync response",
            "got_body": body
        }
    }
"#;

        let module = PyModule::from_code(py, code, "test.py", "test").unwrap();
        let handler_fn = module.getattr("sync_handler").unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();

        let python_handler = _spikard::PythonHandler::new(handler_py, false, None, None, None);

        let handler: Arc<dyn Handler> = Arc::new(python_handler);

        // Create test request data
        let request_data = RequestData {
            path_params: HashMap::new(),
            query_params: serde_json::Value::Null,
            raw_query_params: HashMap::new(),
            body: json!({"test": "data"}),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            method: "POST".to_string(),
            path: "/test".to_string(),
        };

        let request = Request::builder()
            .method(Method::POST)
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        // Execute handler
        let result = handler.call(request, request_data);

        // Run the future
        let response = result.await;

        assert!(response.is_ok());
        let resp = response.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    });
}

#[tokio::test]
async fn test_python_handler_async_execution() {
    init_python();

    Python::with_gil(|py| {
        // Create an asynchronous Python handler
        let code = r#"
import asyncio

async def async_handler(path_params, query_params, body, headers, cookies):
    # Simulate some async work
    await asyncio.sleep(0.001)
    return {
        "status_code": 200,
        "body": {
            "message": "async response",
            "path_params": path_params
        }
    }
"#;

        let module = PyModule::from_code(py, code, "test.py", "test").unwrap();
        let handler_fn = module.getattr("async_handler").unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();

        let python_handler = _spikard::PythonHandler::new(
            handler_py, true, // async handler
            None, None, None,
        );

        let handler: Arc<dyn Handler> = Arc::new(python_handler);

        // Create test request data with path params
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "42".to_string());

        let request_data = RequestData {
            path_params,
            query_params: serde_json::Value::Null,
            raw_query_params: HashMap::new(),
            body: serde_json::Value::Null,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            method: "GET".to_string(),
            path: "/items/42".to_string(),
        };

        let request = Request::builder()
            .method(Method::GET)
            .uri("/items/42")
            .body(Body::empty())
            .unwrap();

        // Execute handler
        let result = handler.call(request, request_data);

        // Run the future
        let response = result.await;

        assert!(response.is_ok());
        let resp = response.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    });
}

#[tokio::test]
async fn test_python_handler_error_handling() {
    init_python();

    Python::with_gil(|py| {
        // Create a Python handler that raises an exception
        let code = r#"
def error_handler(path_params, query_params, body, headers, cookies):
    raise ValueError("Test error")
"#;

        let module = PyModule::from_code(py, code, "test.py", "test").unwrap();
        let handler_fn = module.getattr("error_handler").unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();

        let python_handler = _spikard::PythonHandler::new(handler_py, false, None, None, None);

        let handler: Arc<dyn Handler> = Arc::new(python_handler);

        let request_data = RequestData {
            path_params: HashMap::new(),
            query_params: serde_json::Value::Null,
            raw_query_params: HashMap::new(),
            body: serde_json::Value::Null,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            method: "GET".to_string(),
            path: "/error".to_string(),
        };

        let request = Request::builder()
            .method(Method::GET)
            .uri("/error")
            .body(Body::empty())
            .unwrap();

        // Execute handler
        let result = handler.call(request, request_data).await;

        // Should return error
        assert!(result.is_err());
        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(message.contains("Python error"));
    });
}

#[tokio::test]
async fn test_python_handler_with_headers_and_cookies() {
    init_python();

    Python::with_gil(|py| {
        // Create a handler that echoes headers and cookies
        let code = r#"
def echo_handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {
            "headers": headers,
            "cookies": cookies
        }
    }
"#;

        let module = PyModule::from_code(py, code, "test.py", "test").unwrap();
        let handler_fn = module.getattr("echo_handler").unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();

        let python_handler = _spikard::PythonHandler::new(handler_py, false, None, None, None);

        let handler: Arc<dyn Handler> = Arc::new(python_handler);

        // Create request with headers and cookies
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("authorization".to_string(), "Bearer token123".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "abc123".to_string());

        let request_data = RequestData {
            path_params: HashMap::new(),
            query_params: serde_json::Value::Null,
            raw_query_params: HashMap::new(),
            body: serde_json::Value::Null,
            headers,
            cookies,
            method: "GET".to_string(),
            path: "/echo".to_string(),
        };

        let request = Request::builder()
            .method(Method::GET)
            .uri("/echo")
            .body(Body::empty())
            .unwrap();

        // Execute handler
        let result = handler.call(request, request_data).await;

        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    });
}

#[test]
fn test_event_loop_initialization() {
    init_python();

    // Verify event loop is initialized
    let result = _spikard::init_python_event_loop();
    assert!(result.is_ok());

    // Should be idempotent - calling again should not fail
    let result2 = _spikard::init_python_event_loop();
    assert!(result2.is_ok());
}

#[test]
fn test_extract_routes_from_app() {
    init_python();

    Python::with_gil(|py| {
        // Create a minimal Spikard app with routes
        let code = r#"
from spikard import Spikard

app = Spikard()

def handler1():
    return {"message": "handler1"}

def handler2():
    return {"message": "handler2"}

# Register routes manually
app.register_route(
    "GET",
    "/test1",
    handler=handler1,
    body_schema=None,
    parameter_schema=None,
    file_params=None
)

app.register_route(
    "POST",
    "/test2",
    handler=handler2,
    body_schema=None,
    parameter_schema=None,
    file_params=None
)
"#;

        // Import spikard package (assumes it's in PYTHONPATH)
        let sys = py.import("sys").unwrap();
        let sys_path = sys.getattr("path").unwrap();
        sys_path.call_method1("insert", (0, "packages/python")).unwrap();

        let module = PyModule::from_code(py, code, "test_app.py", "test_app").unwrap();
        let app = module.getattr("app").unwrap();

        // Extract routes
        let routes = _spikard::extract_routes_from_app(py, app);

        assert!(routes.is_ok());
        let route_list = routes.unwrap();
        assert_eq!(route_list.len(), 2);

        // Verify route paths
        let paths: Vec<String> = route_list.iter().map(|r| r.metadata.path.clone()).collect();
        assert!(paths.contains(&"/test1".to_string()));
        assert!(paths.contains(&"/test2".to_string()));
    });
}
