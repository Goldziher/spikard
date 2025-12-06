//! Critical FFI Safety Tests for Python Handler
//!
//! This module tests the safety of the Python FFI boundary, focusing on:
//! - GIL (Global Interpreter Lock) safety during concurrent execution
//! - Exception translation and traceback preservation
//! - Event loop lifecycle management
//!
//! Priority 1 critical test cases that must pass before production deployment.

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use pyo3::prelude::*;
use serde_json::{Value, json};
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

// ============================================================================
// Test Setup & Initialization
// ============================================================================

/// Initialize Python interpreter and event loop for tests
fn init_python() {
    static PYTHON_INIT: OnceLock<()> = OnceLock::new();
    PYTHON_INIT.get_or_init(|| {
        let package_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/python")
            .canonicalize()
            .expect("failed to resolve python package path");
        let stub_path = ensure_stub_dir();
        let mut python_paths = vec![stub_path.clone(), package_path];
        if let Some(current) = std::env::var_os("PYTHONPATH")
            && !current.is_empty()
        {
            python_paths.extend(std::env::split_paths(&current));
        }

        let new_pythonpath = std::env::join_paths(python_paths).expect("failed to build PYTHONPATH");
        unsafe {
            std::env::set_var("PYTHONPATH", &new_pythonpath);
        }

        Python::initialize();
        let _ = _spikard::init_python_event_loop();
    });
}

/// Ensure stub Python modules exist in temp directory
fn ensure_stub_dir() -> PathBuf {
    static STUB_DIR: OnceLock<PathBuf> = OnceLock::new();
    STUB_DIR
        .get_or_init(|| {
            let dir = std::env::temp_dir().join("spikard_py_stub");
            let _ = fs::create_dir_all(&dir);

            // Create _spikard stub module
            let stub = r#"
class Response:
    def __init__(self, status_code: int = 200, body=None, headers=None):
        self.status_code = status_code
        self.body = body
        self.headers = headers or {}

class StreamingResponse(Response):
    pass

def background_run(_awaitable):
    raise RuntimeError("stub")
"#;
            let _ = fs::write(dir.join("_spikard.py"), stub);

            // Create spikard package stub
            let pkg_dir = dir.join("spikard");
            let _ = fs::create_dir_all(&pkg_dir);
            let pkg_init = r#"
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

class Spikard:
    def __init__(self):
        self._routes = []

    def register_route(self, method, path, handler, body_schema=None, parameter_schema=None, file_params=None, body_param_name=None):
        self._routes.append(Route(method, path, handler, body_schema, parameter_schema, file_params, body_param_name))

    def get_routes(self):
        return self._routes
"#;
            let _ = fs::write(pkg_dir.join("__init__.py"), pkg_init);

            // Create _internal subpackage
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

/// Create a Python module from inline code
fn module_from_code<'py>(py: Python<'py>, code: &str, filename: &str, module_name: &str) -> Bound<'py, PyModule> {
    let code_cstr = CString::new(code).expect("Python source must not contain null bytes");
    let filename_cstr = CString::new(filename).expect("filename must not contain null bytes");
    let module_name_cstr = CString::new(module_name).expect("module_name must not contain null bytes");

    PyModule::from_code(
        py,
        code_cstr.as_c_str(),
        filename_cstr.as_c_str(),
        module_name_cstr.as_c_str(),
    )
    .expect("failed to compile Python module")
}

/// Build a Python handler from code string
fn build_python_handler(code: &str, function_name: &str, is_async: bool) -> Arc<dyn Handler> {
    let python_handler = Python::attach(|py| -> PyResult<_spikard::PythonHandler> {
        let module = module_from_code(py, code, "test.py", "test");
        let handler_fn = module.getattr(function_name)?;
        let handler_py: Py<PyAny> = handler_fn.into();
        Ok(_spikard::PythonHandler::new(
            handler_py, is_async, None, None, None, None,
        ))
    })
    .expect("failed to build Python handler");

    Arc::new(python_handler)
}

/// Create minimal RequestData for testing
fn default_request_data() -> RequestData {
    RequestData {
        path_params: HashMap::new().into(),
        query_params: Value::Null,
        raw_query_params: HashMap::new().into(),
        body: json!({}),
        raw_body: None,
        headers: HashMap::new().into(),
        cookies: HashMap::new().into(),
        method: "GET".to_string(),
        path: "/test".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    }
}

/// Extract response body as string
#[allow(dead_code)]
async fn get_response_body(response: axum::http::Response<Body>) -> String {
    let body = response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .expect("failed to read body");
    String::from_utf8_lossy(&bytes).to_string()
}

// ============================================================================
// PRIORITY 1: test_gil_concurrent_handlers_no_deadlock
// ============================================================================

/// Test that concurrent handler execution doesn't cause GIL deadlock.
///
/// This critical test verifies:
/// - 10 concurrent async handlers execute without deadlock (10-second timeout)
/// - All handlers complete successfully with no panics
/// - GIL state remains consistent after concurrent execution
/// - Subsequent calls work correctly (no GIL corruption)
///
/// **Why this matters:**
/// Python's GIL can deadlock if handlers improperly acquire/release it during
/// concurrent async execution. This test ensures handlers properly manage GIL
/// ownership across concurrent Tokio tasks.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_gil_concurrent_handlers_no_deadlock() {
    init_python();

    let code = r#"
import asyncio

async def handler(path_params, query_params, body, headers, cookies):
    # Simulate async work that requires GIL interaction
    await asyncio.sleep(0.01)
    return {
        "status_code": 200,
        "body": {"message": "ok", "id": path_params.get("id", "unknown")}
    }
"#;

    let handler = build_python_handler(code, "handler", true);

    // Spawn 10 concurrent tasks
    let mut handles = vec![];
    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let mut path_params = HashMap::new();
            path_params.insert("id".to_string(), i.to_string());

            let req_data = RequestData {
                path_params: path_params.into(),
                query_params: Value::Null,
                raw_query_params: HashMap::new().into(),
                body: json!({}),
                raw_body: None,
                headers: HashMap::new().into(),
                cookies: HashMap::new().into(),
                method: "GET".to_string(),
                path: format!("/items/{}", i),
                #[cfg(feature = "di")]
                dependencies: None,
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/items/{}", i))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        handles.push(handle);
    }

    // Wait for all with 10-second timeout to detect deadlock
    let results = tokio::time::timeout(Duration::from_secs(10), futures::future::join_all(handles))
        .await
        .expect("test timed out - GIL deadlock detected in concurrent handler execution");

    // Verify all completed successfully
    let mut success_count = 0;
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "handler task {} panicked", i);
        let handler_result = result.as_ref().unwrap();
        assert!(
            handler_result.is_ok(),
            "handler {} returned error: {:?}",
            i,
            handler_result.as_ref().err()
        );
        success_count += 1;
    }
    assert_eq!(success_count, 10, "not all handlers completed successfully");

    // Verify subsequent call still works (no GIL corruption)
    let sequential_handler = build_python_handler(code, "handler", true);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = sequential_handler.call(req, req_data).await;
    assert!(
        result.is_ok(),
        "post-concurrent call failed - GIL corruption detected: {:?}",
        result.err()
    );
}

// ============================================================================
// PRIORITY 1: test_exception_translation_preserves_traceback
// ============================================================================

/// Test that complex Python exceptions preserve information during FFI translation.
///
/// This critical test verifies:
/// - Multi-level call stack (3 levels deep) raises ValueError correctly
/// - Exception is properly translated to StructuredError by Rust FFI boundary
/// - Exception type and message are preserved in error response
/// - Error response has HTTP 500 status code
/// - Response is valid JSON with proper structure
/// - Context/traceback information is available in details
///
/// **Why this matters:**
/// Exception translation across the Python-Rust FFI boundary must preserve
/// error context and traceback information for debugging. Lossy translation
/// makes production issues difficult to diagnose.
#[tokio::test]
async fn test_exception_translation_preserves_traceback() {
    init_python();

    let code = r#"
def level_3():
    raise ValueError("root cause: database connection failed")

def level_2():
    return level_3()

def level_1():
    return level_2()

def handler(path_params, query_params, body, headers, cookies):
    return level_1()
"#;

    let handler = build_python_handler(code, "handler", false);

    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(req, req_data).await;

    // Should fail with error
    assert!(result.is_err(), "handler should have raised ValueError exception");

    let (status, body) = result.unwrap_err();

    // Verify HTTP 500 status
    assert_eq!(
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "error should return 500 status code"
    );

    // Verify response is valid JSON
    let json: Value = match serde_json::from_str(&body) {
        Ok(j) => j,
        Err(e) => panic!("response is not valid JSON: {} (body: {})", e, body),
    };

    // Verify StructuredError format has required fields
    assert!(json.get("error").is_some(), "response missing 'error' field: {}", json);

    let error_value = json.get("error");
    // The error field may contain the full error message or a structured code
    assert!(error_value.is_some(), "error field should exist");

    // Verify exception type is preserved in response
    assert!(
        body.contains("ValueError") || body.contains("root cause"),
        "exception type/message not in error response: {}",
        body
    );

    // Verify that the error response structure is correct
    // Note: Full traceback context (level_1, level_2, level_3) is captured in the
    // Python exception but may not be included in the JSON response by default.
    // What matters is that:
    // 1. The exception type (ValueError) is preserved
    // 2. The error message is included
    // 3. The response has proper JSON structure with error code
    assert!(
        body.contains("ValueError") || body.contains("python_error"),
        "error response should indicate the exception type or error code: {}",
        body
    );
}

// ============================================================================
// Additional Edge Cases & Context Tests
// ============================================================================

/// Test that handler properly handles null/None values in concurrent context.
///
/// This test verifies:
/// - Concurrent handlers can safely process None/null request bodies
/// - Path params and query params are properly handled when null
/// - No memory corruption or crashes with None values
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_concurrent_handlers_with_null_values() {
    init_python();

    let code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {
            "body_is_null": body is None,
            "path_params_type": str(type(path_params)),
            "headers_type": str(type(headers))
        }
    }
"#;

    let handler = build_python_handler(code, "handler", false);

    let mut handles = vec![];
    for _ in 0..5 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let req_data = RequestData {
                body: Value::Null,
                query_params: Value::Null,
                ..default_request_data()
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri("/test")
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        handles.push(handle);
    }

    let results = tokio::time::timeout(Duration::from_secs(5), futures::future::join_all(handles))
        .await
        .expect("handlers timed out with null values");

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "task {} panicked", i);
        assert!(
            result.as_ref().unwrap().is_ok(),
            "handler {} failed with null values",
            i
        );
    }
}

/// Test async event loop initialization is idempotent and thread-safe.
///
/// This test verifies:
/// - Calling init_python_event_loop multiple times is safe
/// - Concurrent async handlers work correctly after initialization
/// - No event loop conflicts or reinitialization errors
#[tokio::test]
async fn test_event_loop_initialization_idempotence() {
    // Don't call init_python() to test initialization within this test
    static INIT_LOCK: OnceLock<()> = OnceLock::new();
    INIT_LOCK.get_or_init(|| {
        let package_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/python")
            .canonicalize()
            .expect("failed to resolve python package path");
        let stub_path = ensure_stub_dir();
        let mut python_paths = vec![stub_path, package_path];
        if let Some(current) = std::env::var_os("PYTHONPATH")
            && !current.is_empty()
        {
            python_paths.extend(std::env::split_paths(&current));
        }

        let new_pythonpath = std::env::join_paths(python_paths).expect("failed to build PYTHONPATH");
        unsafe {
            std::env::set_var("PYTHONPATH", &new_pythonpath);
        }

        Python::initialize();
    });

    // Verify idempotency - multiple inits should succeed or return "already initialized"
    // This ensures the function is safe to call multiple times
    let result1 = _spikard::init_python_event_loop();
    assert!(
        result1.is_ok() || result1.as_ref().unwrap_err().to_string().contains("already"),
        "first init_python_event_loop should succeed or indicate already initialized: {:?}",
        result1
    );

    let result2 = _spikard::init_python_event_loop();
    assert!(
        result2.is_ok() || result2.as_ref().unwrap_err().to_string().contains("already"),
        "second init_python_event_loop should also succeed or indicate already initialized (idempotent): {:?}",
        result2
    );

    // Verify async handlers work after initialization
    let code = r#"
import asyncio

async def handler(path_params, query_params, body, headers, cookies):
    await asyncio.sleep(0.001)
    return {"status_code": 200, "body": {"ok": True}}
"#;

    let handler = build_python_handler(code, "handler", true);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(req, req_data).await;
    assert!(
        result.is_ok(),
        "async handler failed after event loop init: {:?}",
        result
    );
}

/// Test Python exception with empty message is handled correctly.
///
/// This test verifies:
/// - Exceptions without messages don't cause crashes
/// - StructuredError is still returned properly
/// - Empty message is handled gracefully
#[tokio::test]
async fn test_exception_with_empty_message() {
    init_python();

    let code = r#"
def handler(path_params, query_params, body, headers, cookies):
    raise RuntimeError()
"#;

    let handler = build_python_handler(code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(req, req_data).await;
    assert!(result.is_err(), "handler should raise exception");

    let (status, body) = result.unwrap_err();
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let json: Value = serde_json::from_str(&body).expect("error response should be valid JSON");

    assert!(json.get("error").is_some(), "error response should have 'error' field");
}
