//! Integration tests for `PythonHandler`
//!
//! These tests verify that `PythonHandler` correctly implements the Handler trait
//! and properly bridges Python code to Rust.

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use pyo3::prelude::*;
use serde_json::json;
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::ffi::CString;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::{process::Command, str};

/// Helper function to initialize Python for tests
fn init_python() {
    static PYTHON_INIT: OnceLock<()> = OnceLock::new();
    PYTHON_INIT.get_or_init(|| {
        use _spikard::_spikard as spikard_pymodule;

        let package_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/python")
            .canonicalize()
            .expect("failed to resolve python package path");
        let mut python_paths = vec![package_path];
        python_paths.extend(python_site_packages_paths());
        if let Some(current) = std::env::var_os("PYTHONPATH")
            && !current.is_empty()
        {
            python_paths.extend(std::env::split_paths(&current));
        }

        let new_pythonpath = std::env::join_paths(python_paths).expect("failed to build PYTHONPATH");
        unsafe {
            std::env::set_var("PYTHONPATH", &new_pythonpath);
        }

        pyo3::append_to_inittab!(spikard_pymodule);
        Python::initialize();
        let _ = _spikard::init_python_event_loop();
    });
}

fn python_site_packages_paths() -> Vec<PathBuf> {
    let Ok(pyo3_python) = std::env::var("PYO3_PYTHON") else {
        return Vec::new();
    };

    let output = Command::new(pyo3_python)
        .args([
            "-c",
            "import sysconfig\nprint(sysconfig.get_paths().get('purelib',''))\nprint(sysconfig.get_paths().get('platlib',''))",
        ])
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };

    if !output.status.success() {
        return Vec::new();
    }

    let Ok(stdout) = str::from_utf8(&output.stdout) else {
        return Vec::new();
    };

    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(PathBuf::from)
        .collect()
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

#[tokio::test]
async fn body_only_handler_uses_decoder_fast_path() {
    init_python();

    let (py_handler, request_data) = Python::attach(|py| -> PyResult<_> {
        let module = module_from_code(
            py,
            r#"
import msgspec

class Payload(msgspec.Struct):
    name: str

def handler(body: Payload):
    return {"name": body.name}
"#,
            "body_only.py",
            "body_only",
        );
        let handler = module.getattr("handler")?.unbind();

        let py_handler = _spikard::handler::PythonHandler::new(handler, false, None, None, Some("body".to_string()));

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!(null)),
            raw_body: Some(bytes::Bytes::from_static(br#"{"name":"x"}"#)),
            headers: Arc::new(headers),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/json/small".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        Ok((py_handler, request_data))
    })
    .expect("failed to construct handler");

    let request = Request::builder()
        .method(Method::POST)
        .uri("/json/small")
        .body(Body::empty())
        .unwrap();

    let response = py_handler.call(request, request_data).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let parsed: serde_json::Value = serde_json::from_slice(body.as_ref()).unwrap();
    assert_eq!(parsed, json!({"name": "x"}));
}

fn build_python_handler(code: &str, function: &str, is_async: bool) -> Arc<dyn Handler> {
    let python_handler = Python::attach(|py| -> PyResult<_spikard::PythonHandler> {
        let module = module_from_code(py, code, "test.py", "test");
        let handler_fn = module.getattr(function)?;
        let handler_py: Py<PyAny> = handler_fn.into();
        Ok(_spikard::PythonHandler::new(handler_py, is_async, None, None, None))
    })
    .expect("failed to build Python handler");

    Arc::new(python_handler)
}

#[test]
fn test_python_handler_creation() {
    init_python();

    let python_handler = Python::attach(|py| -> PyResult<_spikard::PythonHandler> {
        let code = r#"
def simple_handler(path_params, query_params, body, headers, cookies):
    return {"status_code": 200, "body": {"message": "Hello"}}
"#;

        let module = module_from_code(py, code, "test.py", "test");
        let handler_fn = module.getattr("simple_handler")?;
        let handler_py: Py<PyAny> = handler_fn.into();

        Ok(_spikard::PythonHandler::new(handler_py, false, None, None, None))
    })
    .expect("failed to build Python handler");

    assert!(std::mem::size_of_val(&python_handler) > 0);
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
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(json!({"test": "data"})),
        raw_body: None,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "POST".to_string(),
        path: "/test".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let request = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = handler.call(request, request_data).await;

    eprintln!("sync handler result: {response:?}");
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
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/items/42".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/items/42")
        .body(Body::empty())
        .unwrap();

    let response = handler.call(request, request_data).await;

    eprintln!("async handler result: {response:?}");
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_msgspec_struct_response_serialization() {
    init_python();

    let code = r#"
import msgspec

class Payload(msgspec.Struct):
    x: int
    y: str

def handler(path_params, query_params, body, headers, cookies):
    return Payload(1, "ok")
"#;

    let handler = build_python_handler(code, "handler", false);

    let request_data = RequestData {
        path_params: HashMap::new().into(),
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(json!({"ignored": true})),
        raw_body: None,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/test".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let response = handler
        .call(request, request_data)
        .await
        .expect("handler should succeed");
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read response body");
    let body_text = String::from_utf8_lossy(&body_bytes);

    assert!(body_text.contains(r#""x":1"#), "body: {body_text}");
    assert!(body_text.contains(r#""y":"ok""#), "body: {body_text}");
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
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/error".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
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
        query_params: Arc::new(serde_json::Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: headers.clone().into(),
        cookies: cookies.clone().into(),
        method: "GET".to_string(),
        path: "/echo".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    };

    let request = Request::builder()
        .method(Method::GET)
        .uri("/echo")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(request, request_data).await;

    eprintln!("headers/cookies handler result: {result:?}");
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn test_event_loop_initialization() {
    init_python();

    let result = _spikard::init_python_event_loop();
    assert!(result.is_ok());

    let result2 = _spikard::init_python_event_loop();
    assert!(result2.is_ok());
}

#[test]
fn test_extract_routes_from_app() {
    init_python();

    let route_list = Python::attach(|py| -> PyResult<_> {
        let code = r#"
class Route:
    def __init__(self, method, path, handler, body_schema=None, parameter_schema=None, file_params=None, body_param_name=None):
        self.method = method
        self.path = path
        self.handler = handler
        self.handler_name = getattr(handler, "__name__", "handler")
        self.request_schema = body_schema
        self.response_schema = None
        self.parameter_schema = parameter_schema
        self.file_params = file_params
        self.is_async = False
        self.body_param_name = body_param_name

class App:
    def __init__(self):
        self._routes = []

    def register_route(self, method, path, handler, body_schema=None, parameter_schema=None, file_params=None, body_param_name=None):
        self._routes.append(Route(method, path, handler, body_schema, parameter_schema, file_params, body_param_name))

    def get_routes(self):
        return self._routes

app = App()

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

        let module = module_from_code(py, code, "test_app.py", "test_app");
        let app = module.getattr("app")?;

        _spikard::extract_routes_from_app(py, &app)
    })
    .expect("failed to extract routes");

    assert_eq!(route_list.len(), 2);

    let paths: Vec<String> = route_list.iter().map(|r| r.metadata.path.clone()).collect();
    assert!(paths.contains(&"/test1".to_string()));
    assert!(paths.contains(&"/test2".to_string()));
}
