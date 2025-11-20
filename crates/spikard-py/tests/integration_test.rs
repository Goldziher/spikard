//! Integration tests for PythonHandler
//!
//! These tests verify that PythonHandler correctly implements the Handler trait
//! and properly bridges Python code to Rust.

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use pyo3::prelude::*;
use serde_json::json;
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

/// Helper function to initialize Python for tests
fn init_python() {
    let package_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/python")
        .canonicalize()
        .expect("failed to resolve python package path");
    let stub_path = ensure_stub_dir();
    let new_pythonpath = if let Ok(current) = std::env::var("PYTHONPATH") {
        if current.is_empty() {
            format!("{}:{}", stub_path.display(), package_path.display())
        } else {
            format!("{}:{}:{}", stub_path.display(), package_path.display(), current)
        }
    } else {
        format!("{}:{}", stub_path.display(), package_path.display())
    };
    // SAFETY: setting an env var for the test process only; no cross-thread invariants.
    unsafe {
        std::env::set_var("PYTHONPATH", &new_pythonpath);
    }
    pyo3::prepare_freethreaded_python();
    let _ = _spikard::init_python_event_loop();
}

fn ensure_stub_dir() -> PathBuf {
    static STUB_DIR: OnceLock<PathBuf> = OnceLock::new();
    STUB_DIR
        .get_or_init(|| {
            let dir = std::env::temp_dir().join("spikard_py_stub");
            let _ = fs::create_dir_all(&dir);
            let stub = r#"
class Response:
    def __init__(self, status_code: int = 200, body=None, headers=None):
        self.status_code = status_code
        self.body = body
        self.headers = headers or {}

class StreamingResponse(Response):
    ...

def background_run(_awaitable):
    # Raise to force Python fallback in background.py
    raise RuntimeError("stub")
"#;
            let _ = fs::write(dir.join("_spikard.py"), stub);
            let pkg_dir = dir.join("spikard");
            let _ = fs::create_dir_all(&pkg_dir);
            let pkg_init = r#"
class Route:
    def __init__(self, method, path, handler, body_schema=None, parameter_schema=None, file_params=None):
        self.method = method
        self.path = path
        self.handler = handler
        self.handler_name = getattr(handler, "__name__", "handler")
        self.request_schema = body_schema
        self.response_schema = None
        self.parameter_schema = parameter_schema
        self.file_params = file_params
        self.is_async = False

class Spikard:
    def __init__(self):
        self._routes = []

    def register_route(self, method, path, handler, body_schema=None, parameter_schema=None, file_params=None):
        self._routes.append(Route(method, path, handler, body_schema, parameter_schema, file_params))

    def get_routes(self):
        return self._routes
"#;
            let _ = fs::write(pkg_dir.join("__init__.py"), pkg_init);
            let internal_dir = pkg_dir.join("_internal");
            let _ = fs::create_dir_all(&internal_dir);
            let _ = fs::write(internal_dir.join("__init__.py"), "");
            let _ = fs::write(
                internal_dir.join("converters.py"),
                "def register_decoder(_decoder):\n    return None\n\ndef clear_decoders():\n    return None\n\ndef convert_params(params, handler_func=None, strict=False):\n    return params\n",
            );
            dir
        })
        .clone()
}

fn module_from_code<'py>(py: Python<'py>, code: &str, filename: &str, module_name: &str) -> Bound<'py, PyModule> {
    let code_cstr = CString::new(code).expect("Python source must not contain null bytes");
    let filename_cstr = CString::new(filename).expect("filename must not contain null bytes");
    let module_cstr = CString::new(module_name).expect("module name must not contain null bytes");

    PyModule::from_code(
        py,
        code_cstr.as_c_str(),
        filename_cstr.as_c_str(),
        module_cstr.as_c_str(),
    )
    .expect("failed to compile inline Python module")
}

fn build_python_handler(code: &str, function: &str, is_async: bool) -> Arc<dyn Handler> {
    Python::with_gil(|py| {
        let module = module_from_code(py, code, "test.py", "test");
        let handler_fn = module.getattr(function).unwrap();
        let handler_py: Py<PyAny> = handler_fn.into();
        let python_handler = _spikard::PythonHandler::new(handler_py, is_async, None, None, None);
        Arc::new(python_handler)
    })
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

        let module = module_from_code(py, code, "test.py", "test");
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

    let handler = build_python_handler(code, "sync_handler", false);

    let request_data = RequestData {
        path_params: HashMap::new().into(),
        query_params: serde_json::Value::Null,
        raw_query_params: HashMap::new().into(),
        body: json!({"test": "data"}),
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "POST".to_string(),
        path: "/test".to_string(),
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = handler.call(request, request_data).await;

    eprintln!("sync handler result: {:?}", response);
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_python_handler_async_execution() {
    init_python();

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

    let handler = build_python_handler(code, "async_handler", true);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "42".to_string());

    let request_data = RequestData {
        path_params: path_params.into(),
        query_params: serde_json::Value::Null,
        raw_query_params: HashMap::new().into(),
        body: serde_json::Value::Null,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/items/42".to_string(),
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/items/42")
        .body(Body::empty())
        .unwrap();

    let response = handler.call(request, request_data).await;

    eprintln!("async handler result: {:?}", response);
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_python_handler_error_handling() {
    init_python();

    let code = r#"
def error_handler(path_params, query_params, body, headers, cookies):
    raise ValueError("Test error")
"#;

    let handler = build_python_handler(code, "error_handler", false);

    let request_data = RequestData {
        path_params: HashMap::new().into(),
        query_params: serde_json::Value::Null,
        raw_query_params: HashMap::new().into(),
        body: serde_json::Value::Null,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/error".to_string(),
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/error")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(request, request_data).await;

    assert!(result.is_err());
    let (status, message) = result.unwrap_err();
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(message.contains("Python error"));
}

#[tokio::test]
async fn test_python_handler_with_headers_and_cookies() {
    init_python();

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

    let handler = build_python_handler(code, "echo_handler", false);

    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());
    headers.insert("authorization".to_string(), "Bearer token123".to_string());

    let mut cookies = HashMap::new();
    cookies.insert("session_id".to_string(), "abc123".to_string());

    let request_data = RequestData {
        path_params: HashMap::new().into(),
        query_params: serde_json::Value::Null,
        raw_query_params: HashMap::new().into(),
        body: serde_json::Value::Null,
        headers: headers.clone().into(),
        cookies: cookies.clone().into(),
        method: "GET".to_string(),
        path: "/echo".to_string(),
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/echo")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(request, request_data).await;

    eprintln!("headers/cookies handler result: {:?}", result);
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
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

        let module = module_from_code(py, code, "test_app.py", "test_app");
        let app = module.getattr("app").unwrap();

        // Extract routes
        let routes = _spikard::extract_routes_from_app(py, &app);

        assert!(routes.is_ok());
        let route_list = routes.unwrap();
        assert_eq!(route_list.len(), 2);

        // Verify route paths
        let paths: Vec<String> = route_list.iter().map(|r| r.metadata.path.clone()).collect();
        assert!(paths.contains(&"/test1".to_string()));
        assert!(paths.contains(&"/test2".to_string()));
    });
}
