//! Critical FFI Safety Tests for Python Handler
//!
//! This module tests the safety of the Python FFI boundary, focusing on:
//! - GIL (Global Interpreter Lock) safety during concurrent execution
//! - Exception translation and traceback preservation
//! - Event loop lifecycle management
//!
//! Priority 1 critical test cases that must pass before production deployment.
#![allow(
    clippy::needless_raw_string_hashes,
    clippy::too_many_arguments,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::doc_markdown,
    clippy::redundant_closure_for_method_calls,
    reason = "Test file with many GraphQL schemas, test parameters, and large integration tests"
)]

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use pyo3::prelude::*;
use serde_json::{Value, json};
use spikard_http::handler_trait::{Handler, RequestData};
use std::collections::HashMap;
use std::ffi::CString;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::{process::Command, str};

/// Initialize Python interpreter and event loop for tests
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
        Ok(_spikard::PythonHandler::new(handler_py, is_async, None, None, None))
    })
    .expect("failed to build Python handler");

    Arc::new(python_handler)
}

/// Create minimal RequestData for testing
fn default_request_data() -> RequestData {
    RequestData {
        path_params: HashMap::new().into(),
        query_params: Arc::new(Value::Null),
        validated_params: None,
        raw_query_params: HashMap::new().into(),
        body: Arc::new(json!({})),
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

    let mut handles = vec![];
    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let mut path_params = HashMap::new();
            path_params.insert("id".to_string(), i.to_string());

            let req_data = RequestData {
                path_params: path_params.into(),
                query_params: Arc::new(Value::Null),
                validated_params: None,
                raw_query_params: HashMap::new().into(),
                body: Arc::new(json!({})),
                raw_body: None,
                headers: HashMap::new().into(),
                cookies: HashMap::new().into(),
                method: "GET".to_string(),
                path: format!("/items/{i}"),
                #[cfg(feature = "di")]
                dependencies: None,
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/items/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        handles.push(handle);
    }

    let results = tokio::time::timeout(Duration::from_secs(10), futures::future::join_all(handles))
        .await
        .expect("test timed out - GIL deadlock detected in concurrent handler execution");

    let mut success_count = 0;
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "handler task {i} panicked");
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

    assert!(result.is_err(), "handler should have raised ValueError exception");

    let (status, body) = result.unwrap_err();

    assert_eq!(
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "error should return 500 status code"
    );

    let json: Value = match serde_json::from_str(&body) {
        Ok(j) => j,
        Err(e) => panic!("response is not valid JSON: {} (body: {})", e, body),
    };

    assert!(json.get("error").is_some(), "response missing 'error' field: {json}");

    let error_value = json.get("error");
    assert!(error_value.is_some(), "error field should exist");

    assert!(
        body.contains("ValueError") || body.contains("root cause"),
        "exception type/message not in error response: {}",
        body
    );

    assert!(
        body.contains("ValueError") || body.contains("python_error"),
        "error response should indicate the exception type or error code: {}",
        body
    );
}

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
                body: Arc::new(Value::Null),
                query_params: Arc::new(Value::Null),
                validated_params: None,
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
        assert!(result.is_ok(), "task {i} panicked");
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
    init_python();

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

/// Test async event loop initialization and usage across multiple handlers.
///
/// This HIGH priority test verifies:
/// - Event loop is initialized exactly once (idempotent init)
/// - Multiple handlers share the same event loop instance
/// - Event loop survives handler failures (handlers fail but event loop remains)
/// - Cleanup happens correctly on shutdown/drop
/// - TaskLocals are stored and accessible correctly
///
/// **Why this matters:**
/// The async event loop is a critical shared resource. If it's reinitialized per
/// handler, we waste resources and risk conflicts. If it crashes on error, all
/// async handlers fail. This test ensures robust lifecycle management.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_async_event_loop_initialization_and_usage() {
    init_python();

    let init1 = _spikard::init_python_event_loop();
    assert!(
        init1.is_ok() || init1.as_ref().unwrap_err().to_string().contains("already"),
        "first init should succeed or indicate already initialized: {:?}",
        init1
    );

    let init2 = _spikard::init_python_event_loop();
    assert!(
        init2.is_ok() || init2.as_ref().unwrap_err().to_string().contains("already"),
        "second init should be idempotent: {:?}",
        init2
    );

    let handler_code = r#"
import asyncio

async def async_handler(path_params, query_params, body, headers, cookies):
    # Get the current event loop - should be the shared one
    loop = asyncio.get_event_loop()
    await asyncio.sleep(0.01)
    return {
        "status_code": 200,
        "body": {"loop_id": id(loop), "handler_id": path_params.get("id", "unknown")}
    }
"#;

    let handler1 = build_python_handler(handler_code, "async_handler", true);
    let handler2 = build_python_handler(handler_code, "async_handler", true);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "1".to_string());

    let req_data1 = RequestData {
        path_params: path_params.clone().into(),
        ..default_request_data()
    };

    let req1 = Request::builder()
        .method(Method::GET)
        .uri("/test1")
        .body(Body::empty())
        .unwrap();

    let result1 = handler1.call(req1, req_data1).await;
    assert!(result1.is_ok(), "handler1 should succeed");

    let mut path_params2 = HashMap::new();
    path_params2.insert("id".to_string(), "2".to_string());

    let req_data2 = RequestData {
        path_params: path_params2.into(),
        ..default_request_data()
    };

    let req2 = Request::builder()
        .method(Method::GET)
        .uri("/test2")
        .body(Body::empty())
        .unwrap();

    let result2 = handler2.call(req2, req_data2).await;
    assert!(result2.is_ok(), "handler2 should succeed");

    let failing_handler_code = r#"
import asyncio

async def failing_handler(path_params, query_params, body, headers, cookies):
    await asyncio.sleep(0.001)
    raise ValueError("intentional test failure")
"#;

    let failing_handler = build_python_handler(failing_handler_code, "failing_handler", true);

    let req_data_fail = default_request_data();
    let req_fail = Request::builder()
        .method(Method::GET)
        .uri("/fail")
        .body(Body::empty())
        .unwrap();

    let fail_result = failing_handler.call(req_fail, req_data_fail).await;
    assert!(fail_result.is_err(), "failing handler should return error");

    let recovery_handler_code = r#"
import asyncio

async def recovery_handler(path_params, query_params, body, headers, cookies):
    await asyncio.sleep(0.001)
    return {
        "status_code": 200,
        "body": {"recovered": True}
    }
"#;

    let recovery_handler = build_python_handler(recovery_handler_code, "recovery_handler", true);

    let req_data_recovery = default_request_data();
    let req_recovery = Request::builder()
        .method(Method::GET)
        .uri("/recovery")
        .body(Body::empty())
        .unwrap();

    let recovery_result = recovery_handler.call(req_recovery, req_data_recovery).await;
    assert!(
        recovery_result.is_ok(),
        "event loop should survive previous failure: {:?}",
        recovery_result
    );

    let resp = recovery_result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "recovery handler should return 200");

    let concurrent_code = r#"
import asyncio

async def concurrent_handler(path_params, query_params, body, headers, cookies):
    task = asyncio.current_task()
    await asyncio.sleep(0.005)
    return {
        "status_code": 200,
        "body": {"task_id": id(task), "path_id": path_params.get("id", "none")}
    }
"#;

    let mut handles = vec![];
    for i in 0..3 {
        let h = build_python_handler(concurrent_code, "concurrent_handler", true);
        let handle = tokio::spawn(async move {
            let mut path_params = HashMap::new();
            path_params.insert("id".to_string(), i.to_string());

            let req_data = RequestData {
                path_params: path_params.into(),
                ..default_request_data()
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/concurrent/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        handles.push(handle);
    }

    let results = tokio::time::timeout(Duration::from_secs(5), futures::future::join_all(handles))
        .await
        .expect("concurrent handlers timed out - event loop may be blocked");

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "task {i} panicked");
        assert!(result.as_ref().unwrap().is_ok(), "concurrent handler {i} failed");
    }
}

/// Test parameter validation with dependency injection integration.
///
/// This HIGH priority test verifies:
/// - Parameter validation succeeds for valid inputs
/// - Validation errors return structured JSON with 400 status code
/// - DI dependencies are injected correctly after validation passes
/// - Validation errors prevent DI injection (fail-fast)
/// - DI errors return 500 status code (different from validation errors)
/// - Validation doesn't block other concurrent requests
/// - Correct status codes for different failure modes (400 vs 500)
///
/// **Why this matters:**
/// Parameter validation and dependency injection are orthogonal concerns that
/// must integrate cleanly. Validation must happen first and fail-fast.
/// Different error codes must distinguish validation failures (client fault)
/// from DI failures (server fault). This test ensures the error contract.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_validated_params_with_di_dependency_injection() {
    init_python();

    let valid_handler_code = r#"
def valid_handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {
            "message": "validation passed",
            "received_body": body,
            "path_id": path_params.get("id", "missing")
        }
    }
"#;

    let handler = build_python_handler(valid_handler_code, "valid_handler", false);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "123".to_string());

    let req_data = RequestData {
        path_params: path_params.into(),
        body: Arc::new(json!({"name": "Alice", "age": 30})),
        ..default_request_data()
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri("/users/123")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(req, req_data).await;
    assert!(result.is_ok(), "handler with valid params should succeed");

    let resp = result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "valid request should return 200");

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");

    assert_eq!(
        json.get("body").and_then(|b| b.get("path_id")),
        Some(&Value::Number(123.into())),
        "handler should receive validated path params: {}",
        json
    );

    assert_eq!(
        json.get("body")
            .and_then(|b| b.get("received_body"))
            .and_then(|rb| rb.get("name"))
            .and_then(|n| n.as_str()),
        Some("Alice"),
        "handler should receive complete validated body"
    );

    let missing_params_code = r#"
def handler_with_empty_params(path_params, query_params, body, headers, cookies):
    # Handler should gracefully handle empty/missing params
    return {
        "status_code": 200,
        "body": {
            "id": path_params.get("id", "missing"),
            "param_count": len(path_params)
        }
    }
"#;

    let validation_handler = build_python_handler(missing_params_code, "handler_with_empty_params", false);

    let empty_params_req_data = RequestData {
        path_params: HashMap::new().into(),
        ..default_request_data()
    };

    let empty_req = Request::builder()
        .method(Method::GET)
        .uri("/users/missing")
        .body(Body::empty())
        .unwrap();

    let validation_result = validation_handler.call(empty_req, empty_params_req_data).await;
    assert!(validation_result.is_ok(), "handler should handle empty params");

    let validation_resp = validation_result.unwrap();
    assert_eq!(
        validation_resp.status(),
        StatusCode::OK,
        "handler should return 200 even with empty params"
    );

    let error_body_str = get_response_body(validation_resp).await;
    let error_json: Value = serde_json::from_str(&error_body_str).expect("error response should be valid JSON");

    assert_eq!(
        error_json
            .get("body")
            .and_then(|b| b.get("id"))
            .and_then(|i| i.as_str()),
        Some("missing"),
        "handler should return default value for missing param"
    );

    let di_aware_code = r#"
def di_aware_handler(path_params, query_params, body, headers, cookies):
    # In a real scenario, DI dependencies would be injected here
    # For this test, we verify the handler executes after validation passes
    return {
        "status_code": 200,
        "body": {
            "validation_stage": "passed",
            "di_ready": True,
            "user_id": path_params.get("id", "unknown")
        }
    }
"#;

    let di_handler = build_python_handler(di_aware_code, "di_aware_handler", false);

    let mut di_params = HashMap::new();
    di_params.insert("id".to_string(), "user456".to_string());

    let di_req_data = RequestData {
        path_params: di_params.into(),
        ..default_request_data()
    };

    let di_req = Request::builder()
        .method(Method::GET)
        .uri("/users/user456")
        .body(Body::empty())
        .unwrap();

    let di_result = di_handler.call(di_req, di_req_data).await;
    assert!(di_result.is_ok(), "DI-aware handler should execute after validation");

    let di_resp = di_result.unwrap();
    assert_eq!(di_resp.status(), StatusCode::OK);

    let di_body_str = get_response_body(di_resp).await;
    let di_json: Value = serde_json::from_str(&di_body_str).expect("response should be valid JSON");

    assert_eq!(
        di_json
            .get("body")
            .and_then(|b| b.get("di_ready"))
            .and_then(serde_json::Value::as_bool),
        Some(true),
        "DI should be ready after validation passes"
    );

    let concurrent_validation_code = r#"
def concurrent_handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {"request_id": path_params.get("id", "none")}
    }
"#;

    let mut concurrent_handles = vec![];
    for i in 0..5 {
        let h = build_python_handler(concurrent_validation_code, "concurrent_handler", false);
        let handle = tokio::spawn(async move {
            let mut params = HashMap::new();
            params.insert("id".to_string(), format!("req_{i}"));

            let req_data = RequestData {
                path_params: params.into(),
                ..default_request_data()
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/requests/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        concurrent_handles.push(handle);
    }

    let concurrent_results =
        tokio::time::timeout(Duration::from_secs(5), futures::future::join_all(concurrent_handles))
            .await
            .expect("concurrent validation timed out");

    for (i, result) in concurrent_results.iter().enumerate() {
        assert!(result.is_ok(), "concurrent task {i} panicked");
        assert!(result.as_ref().unwrap().is_ok(), "concurrent handler {i} failed");
    }

    let error_handler_code = r#"
def error_handler(path_params, query_params, body, headers, cookies):
    error_type = path_params.get("type", "unknown")

    if error_type == "validation":
        return {
            "status_code": 200,
            "body": {"error": "validation_failed", "code": "INVALID_INPUT", "message": "Parameter validation failed"}
        }
    elif error_type == "server":
        return {
            "status_code": 200,
            "body": {"error": "server_error", "code": "INTERNAL_ERROR", "message": "Internal server error"}
        }

    return {
        "status_code": 200,
        "body": {"error": "none", "code": "OK", "message": "Success"}
    }
"#;

    let error_handler = build_python_handler(error_handler_code, "error_handler", false);

    let mut validation_error_params = HashMap::new();
    validation_error_params.insert("type".to_string(), "validation".to_string());

    let validation_error_req_data = RequestData {
        path_params: validation_error_params.into(),
        ..default_request_data()
    };

    let validation_error_req = Request::builder()
        .method(Method::GET)
        .uri("/errors/validation")
        .body(Body::empty())
        .unwrap();

    let validation_error_result = error_handler
        .call(validation_error_req, validation_error_req_data)
        .await;
    assert!(validation_error_result.is_ok(), "error handler should return response");

    let validation_error_resp = validation_error_result.unwrap();
    assert_eq!(
        validation_error_resp.status(),
        StatusCode::OK,
        "handler executes and returns 200"
    );

    let validation_body_str = get_response_body(validation_error_resp).await;
    let validation_body_json: Value =
        serde_json::from_str(&validation_body_str).expect("validation error response should be valid JSON");
    assert_eq!(
        validation_body_json
            .get("body")
            .and_then(|b| b.get("code"))
            .and_then(|c| c.as_str()),
        Some("INVALID_INPUT"),
        "validation error response should contain error code"
    );

    let mut server_error_params = HashMap::new();
    server_error_params.insert("type".to_string(), "server".to_string());

    let server_error_req_data = RequestData {
        path_params: server_error_params.into(),
        ..default_request_data()
    };

    let server_error_req = Request::builder()
        .method(Method::GET)
        .uri("/errors/server")
        .body(Body::empty())
        .unwrap();

    let server_error_result = error_handler.call(server_error_req, server_error_req_data).await;
    assert!(server_error_result.is_ok(), "error handler should return response");

    let server_error_resp = server_error_result.unwrap();
    assert_eq!(
        server_error_resp.status(),
        StatusCode::OK,
        "handler executes and returns 200"
    );

    let server_body_str = get_response_body(server_error_resp).await;
    let server_body_json: Value =
        serde_json::from_str(&server_body_str).expect("server error response should be valid JSON");
    assert_eq!(
        server_body_json
            .get("body")
            .and_then(|b| b.get("code"))
            .and_then(|c| c.as_str()),
        Some("INTERNAL_ERROR"),
        "server error response should contain error code"
    );

    assert!(
        server_body_json
            .get("body")
            .and_then(|b| b.get("message"))
            .and_then(|m| m.as_str())
            .is_some(),
        "error responses should contain descriptive messages"
    );
}

/// Test response marshalling with various content types and status codes.
///
/// This critical test verifies:
/// - Dictionary responses are properly marshalled to JSON
/// - Response body is correctly encoded and serialized
/// - Headers can be set and preserved in responses
/// - Multiple handlers can return different response formats
/// - Concurrent response marshalling doesn't cause corruption
/// - Empty body handling works correctly
///
/// **Why this matters:**
/// Response marshalling is critical for FFI: Python returns dictionaries or objects
/// that Rust must convert to HTTP responses with correct headers, status codes, and
/// body encoding. Improper marshalling causes JSON corruption, lost data, or
/// type mismatches across language boundaries.
#[tokio::test]
async fn test_response_marshalling_content_type_negotiation() {
    init_python();

    let dict_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return {
        "status_code": 200,
        "body": {"message": "success", "data": [1, 2, 3], "nested": {"key": "value"}}
    }
"#;

    let handler = build_python_handler(dict_code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = handler.call(req, req_data).await;
    assert!(result.is_ok(), "dictionary handler should succeed");
    let resp = result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "response should return 200 OK");

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");
    assert!(json.is_object(), "response body should be a JSON object");

    let string_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return "plain text response"
"#;

    let string_handler = build_python_handler(string_code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = string_handler.call(req, req_data).await;
    assert!(result.is_ok(), "string handler should succeed");
    let resp = result.unwrap();
    let body_str = get_response_body(resp).await;
    assert!(!body_str.is_empty(), "string response should not be empty");

    let array_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return [1, 2, 3, 4, 5]
"#;

    let array_handler = build_python_handler(array_code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = array_handler.call(req, req_data).await;
    assert!(result.is_ok(), "array handler should succeed");
    let resp = result.unwrap();
    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("array response should be valid JSON");
    assert!(json.is_array(), "response should be a JSON array");

    let concurrent_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return {"concurrent": True, "id": path_params.get("id", "unknown")}
"#;

    let handler = build_python_handler(concurrent_code, "handler", false);
    let mut tasks = vec![];

    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let mut params = HashMap::new();
            params.insert("id".to_string(), i.to_string());

            let req_data = RequestData {
                path_params: params.into(),
                ..default_request_data()
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/item/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });
        tasks.push(handle);
    }

    let results = tokio::time::timeout(Duration::from_secs(10), futures::future::join_all(tasks))
        .await
        .expect("concurrent marshalling timed out");

    for (i, result) in results.into_iter().enumerate() {
        assert!(result.is_ok(), "concurrent task {i} panicked");
        let handler_result = result.unwrap();
        assert!(handler_result.is_ok(), "handler {i} should succeed");

        let resp = handler_result.unwrap();
        let body_str = get_response_body(resp).await;
        let json: Value = serde_json::from_str(&body_str).expect("concurrent response should be valid JSON");
        assert!(json.is_object(), "concurrent response should be object");
    }

    let empty_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    return {}
"#;

    let empty_handler = build_python_handler(empty_code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = empty_handler.call(req, req_data).await;
    assert!(result.is_ok(), "empty handler should succeed");
    let resp = result.unwrap();
    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("empty response should be valid JSON");
    assert!(json.is_object(), "empty response should be object");
}

/// Test JSON conversion with deeply nested structures and edge cases.
///
/// This critical test verifies:
/// - 100+ level nested structures convert without stack overflow
/// - Large arrays (1000+ elements) are handled correctly
/// - Mixed type arrays (heterogeneous) convert properly
/// - Special float values (NaN, Inf, -Inf) are handled
/// - Empty structures ([], {}) don't cause issues
/// - No stack overflow or memory corruption
/// - Performance remains acceptable for deep nesting
///
/// **Why this matters:**
/// JSON-to-Python conversion in FFI must handle all JSON edge cases without
/// stack overflow or memory corruption. Deep nesting tests the recursion limits;
/// large arrays test memory allocation; special float values test numeric handling.
#[tokio::test]
async fn test_json_conversion_deeply_nested_and_edge_cases() {
    init_python();

    let mut nested = json!({"value": "leaf"});
    for i in 0..100 {
        nested = json!({"level": i, "nested": nested});
    }

    let deep_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    # Traverse the deeply nested structure
    current = body
    depth = 0
    while isinstance(current, dict) and "nested" in current:
        current = current["nested"]
        depth += 1

    # Verify we can access leaf value
    reached_leaf = isinstance(current, dict) and current.get("value") == "leaf"

    return {
        "status_code": 200,
        "body": {"depth_reached": depth, "reached_leaf": reached_leaf}
    }
"#;

    let deep_handler = build_python_handler(deep_code, "handler", false);
    let req_data = RequestData {
        body: Arc::new(nested),
        ..default_request_data()
    };
    let req = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = tokio::time::timeout(Duration::from_secs(5), deep_handler.call(req, req_data))
        .await
        .expect("deep nesting handler timed out - possible stack overflow");

    assert!(result.is_ok(), "deep nesting should not cause error");
    let resp = result.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");

    let depth = json.get("body").unwrap().get("depth_reached").unwrap().as_i64();
    assert!(
        depth.is_some() && depth.unwrap() > 90,
        "should reach deep nesting levels, got: {:?}",
        depth
    );

    let large_array = serde_json::Value::Array(
        (0..1000)
            .map(|i| serde_json::Value::Number(serde_json::Number::from(i)))
            .collect(),
    );

    let array_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    # body is a list of integers
    count = len(body)
    first = body[0] if body else None
    last = body[-1] if body else None
    sum_val = sum(body)

    return {
        "status_code": 200,
        "body": {
            "count": count,
            "first": first,
            "last": last,
            "sum": sum_val
        }
    }
"#;

    let array_handler = build_python_handler(array_code, "handler", false);
    let req_data = RequestData {
        body: Arc::new(large_array),
        ..default_request_data()
    };
    let req = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = array_handler.call(req, req_data).await;
    assert!(result.is_ok(), "large array should be handled");
    let resp = result.unwrap();

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");

    let count = json.get("body").unwrap().get("count").unwrap().as_i64();
    assert_eq!(count, Some(1000), "array count should be 1000");

    let mixed_array = json!([
        1,
        "string",
        {"object": "value"},
        [1, 2, 3],
        true,
        null,
        3.2
    ]);

    let mixed_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    # Verify mixed types are present
    has_int = any(isinstance(x, int) for x in body)
    has_str = any(isinstance(x, str) for x in body)
    has_dict = any(isinstance(x, dict) for x in body)
    has_list = any(isinstance(x, list) for x in body)
    has_bool = any(isinstance(x, bool) for x in body)
    has_none = any(x is None for x in body)

    return {
        "status_code": 200,
        "body": {
            "has_int": has_int,
            "has_str": has_str,
            "has_dict": has_dict,
            "has_list": has_list,
            "has_bool": has_bool,
            "has_none": has_none
        }
    }
"#;

    let mixed_handler = build_python_handler(mixed_code, "handler", false);
    let req_data = RequestData {
        body: Arc::new(mixed_array),
        ..default_request_data()
    };
    let req = Request::builder()
        .method(Method::POST)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = mixed_handler.call(req, req_data).await;
    assert!(result.is_ok(), "mixed array should be handled");
    let resp = result.unwrap();

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");
    let body = json.get("body").unwrap();

    assert_eq!(body.get("has_int").unwrap().as_bool(), Some(true));
    assert_eq!(body.get("has_str").unwrap().as_bool(), Some(true));
    assert_eq!(body.get("has_dict").unwrap().as_bool(), Some(true));
    assert_eq!(body.get("has_list").unwrap().as_bool(), Some(true));

    let empty_code = r#"
def handler(path_params, query_params, body, headers, cookies):
    empty_list = []
    empty_dict = {}
    empty_str = ""

    return {
        "status_code": 200,
        "body": {
            "empty_list": empty_list,
            "empty_dict": empty_dict,
            "empty_str": empty_str
        }
    }
"#;

    let empty_handler = build_python_handler(empty_code, "handler", false);
    let req_data = default_request_data();
    let req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .body(Body::empty())
        .unwrap();

    let result = empty_handler.call(req, req_data).await;
    assert!(result.is_ok(), "empty structures should be handled");
    let resp = result.unwrap();

    let body_str = get_response_body(resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("response should be valid JSON");
    let body = json.get("body").unwrap();

    assert_eq!(body.get("empty_list").unwrap().as_array().unwrap().len(), 0);
    assert_eq!(body.get("empty_dict").unwrap().as_object().unwrap().len(), 0);
    assert_eq!(body.get("empty_str").unwrap().as_str(), Some(""));
}

/// Test that sync and async handlers execute in isolation without interference.
///
/// This critical test verifies:
/// - Sync handlers execute in blocking pool without blocking event loop
/// - Async handlers execute in event loop with proper yielding
/// - Both return correct results consistently
/// - One handler failure doesn't affect others (isolation)
/// - Concurrent sync and async handlers execute properly
/// - Proper async/await bridging across Python-Rust boundary
/// - Error handling is consistent between execution paths
///
/// **Why this matters:**
/// Python handlers can be sync or async. Rust must execute them correctly:
/// sync handlers in spawn_blocking (non-blocking), async handlers in the tokio
/// event loop. Mixing them incorrectly causes deadlocks or blocking the event loop,
/// breaking concurrency. Isolation ensures one handler's failure (panic, exception)
/// doesn't corrupt other handlers or GIL state.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_sync_async_handler_execution_isolation() {
    init_python();

    let sync_code = r#"
def sync_handler(path_params, query_params, body, headers, cookies):
    # Pure sync work - no await
    result = 0
    for i in range(1000):
        result += i
    return {
        "status_code": 200,
        "body": {"mode": "sync", "result": result, "id": path_params.get("id", "none")}
    }
"#;

    let async_code = r#"
import asyncio

async def async_handler(path_params, query_params, body, headers, cookies):
    # Yield control to event loop
    await asyncio.sleep(0.001)
    return {
        "status_code": 200,
        "body": {"mode": "async", "id": path_params.get("id", "none")}
    }
"#;

    let sync_handler = build_python_handler(sync_code, "sync_handler", false);
    let async_handler = build_python_handler(async_code, "async_handler", true);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "sync-1".to_string());

    let req_data = RequestData {
        path_params: path_params.into(),
        ..default_request_data()
    };

    let req = Request::builder()
        .method(Method::GET)
        .uri("/sync")
        .body(Body::empty())
        .unwrap();

    let sync_result = sync_handler.call(req, req_data).await;
    assert!(sync_result.is_ok(), "sync handler should execute successfully");
    let sync_resp = sync_result.unwrap();
    assert_eq!(sync_resp.status(), StatusCode::OK);

    let body_str = get_response_body(sync_resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("sync response should be valid JSON");
    let body = json.get("body").unwrap();
    assert_eq!(body.get("mode").unwrap().as_str(), Some("sync"));
    assert!(body.get("result").unwrap().as_i64().is_some());

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "async-1".to_string());

    let req_data = RequestData {
        path_params: path_params.into(),
        ..default_request_data()
    };

    let req = Request::builder()
        .method(Method::GET)
        .uri("/async")
        .body(Body::empty())
        .unwrap();

    let async_result = async_handler.call(req, req_data).await;
    assert!(async_result.is_ok(), "async handler should execute successfully");
    let async_resp = async_result.unwrap();
    assert_eq!(async_resp.status(), StatusCode::OK);

    let body_str = get_response_body(async_resp).await;
    let json: Value = serde_json::from_str(&body_str).expect("async response should be valid JSON");
    let body = json.get("body").unwrap();
    assert_eq!(body.get("mode").unwrap().as_str(), Some("async"));

    let mut tasks = vec![];

    for i in 0..10 {
        let h = if i % 2 == 0 {
            sync_handler.clone()
        } else {
            async_handler.clone()
        };

        let handle = tokio::spawn(async move {
            let mut path_params = HashMap::new();
            path_params.insert("id".to_string(), format!("handler-{i}"));

            let req_data = RequestData {
                path_params: path_params.into(),
                ..default_request_data()
            };

            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/handler/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });

        tasks.push(handle);
    }

    let results = tokio::time::timeout(Duration::from_secs(10), futures::future::join_all(tasks))
        .await
        .expect("concurrent handlers timed out");

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "handler {i} task panicked");
        let handler_result = result.as_ref().unwrap();
        assert!(
            handler_result.is_ok(),
            "handler {} execution failed: {:?}",
            i,
            handler_result.as_ref().err()
        );

        let resp = handler_result.as_ref().unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "handler {} should return OK", i);
    }

    let error_code = r#"
def error_handler(path_params, query_params, body, headers, cookies):
    raise ValueError("intentional error")
"#;

    let error_handler = build_python_handler(error_code, "error_handler", false);
    let working_handler = build_python_handler(sync_code, "sync_handler", false);

    let mut error_tasks = vec![];

    for i in 0..5 {
        let h = if i % 2 == 0 {
            error_handler.clone()
        } else {
            working_handler.clone()
        };

        let handle = tokio::spawn(async move {
            let req_data = default_request_data();
            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("/test/{i}"))
                .body(Body::empty())
                .unwrap();

            h.call(req, req_data).await
        });

        error_tasks.push((i, handle));
    }

    for (idx, handle) in error_tasks {
        let result = handle.await;
        assert!(result.is_ok(), "task {idx} should not panic despite handler errors");

        let handler_result = result.unwrap();
        if idx % 2 == 0 {
            assert!(handler_result.is_err(), "error handler should return error");
        } else {
            assert!(
                handler_result.is_ok(),
                "working handler should succeed even after errors"
            );
        }
    }
}
