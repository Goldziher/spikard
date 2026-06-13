// TestClient FFI — included verbatim into service.rs via include!().
// This file is NOT a separate Rust module; it shares the service module's namespace.
// `AppOpaque`, `CStr`, `CString`, `c_char`, and `Arc` are in scope from service.rs.

/// Opaque handle to a TestClient instance.
///
/// Allocated by `spikard_test_client_new()`, freed by `spikard_test_client_free()`.
/// The TestClient is backed by an in-process Axum router and does not open a TCP socket.
#[repr(C)]
pub struct TestClientOpaque {
    pub(crate) inner: Option<Box<spikard::TestClient>>,
}

/// Create a TestClient from the given App.
///
/// This consumes the App's inner state (the router is extracted) and leaves the
/// `AppOpaque` shell empty. Call `spikard_app_free()` on `app` after this to
/// release the shell.
///
/// Returns a new `TestClientOpaque` that must be freed with `spikard_test_client_free()`.
/// Returns null on failure.
///
/// # Safety
/// - `app` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - The returned pointer must be freed with `spikard_test_client_free()`.
#[no_mangle]
pub extern "C" fn spikard_test_client_new(app: *mut AppOpaque) -> *mut TestClientOpaque {
    if app.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: app was allocated by _new() and is valid until freed.
    let inner_app = match unsafe { (*app).inner.take() } {
        Some(boxed) => *boxed,
        None => return std::ptr::null_mut(),
    };

    let router = match inner_app.into_router() {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };

    let client = match spikard::TestClient::from_router(router) {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    Box::into_raw(Box::new(TestClientOpaque { inner: Some(Box::new(client)) }))
}

/// Free a TestClient handle.
///
/// # Safety
/// - `ptr` must have been allocated by `spikard_test_client_new()`.
/// - After this call `ptr` is invalid and must not be dereferenced.
/// - Safe to call with a null pointer (no-op).
#[no_mangle]
pub extern "C" fn spikard_test_client_free(ptr: *mut TestClientOpaque) {
    if !ptr.is_null() {
        // SAFETY: ptr was allocated by Box::into_raw above.
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

/// Opaque handle for an HTTP response snapshot returned by TestClient methods.
///
/// Allocated by `spikard_test_client_get()` etc., freed by `spikard_test_response_free()`.
/// This is distinct from the auto-generated `spikard::Response` handle in lib.rs.
#[repr(C)]
pub struct TestResponseOpaque {
    pub(crate) inner: Box<spikard::ResponseSnapshot>,
}

/// Free a response handle returned by a TestClient method.
///
/// # Safety
/// - `ptr` must have been returned by a TestClient method (`spikard_test_client_get` etc.).
/// - Safe to call with a null pointer (no-op).
#[no_mangle]
pub extern "C" fn spikard_test_response_free(ptr: *mut TestResponseOpaque) {
    if !ptr.is_null() {
        // SAFETY: ptr was allocated by Box::into_raw; we are the sole owner.
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

/// Get the HTTP status code from a TestClient response handle.
///
/// Returns 0 if `ptr` is null.
///
/// # Safety
/// - `ptr` must be a valid handle returned by a TestClient method.
#[no_mangle]
pub extern "C" fn spikard_test_response_status(ptr: *const TestResponseOpaque) -> u16 {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: non-null check above.
    unsafe { (*ptr).inner.status }
}

/// Get the TestClient response body as a null-terminated JSON string.
///
/// Returns null if `ptr` is null or the body is not valid JSON.
/// The returned string must be freed with `spikard_free_string()`.
///
/// # Safety
/// - `ptr` must be a valid handle returned by a TestClient method.
#[no_mangle]
pub extern "C" fn spikard_test_response_json(ptr: *const TestResponseOpaque) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null check above.
    let snapshot = unsafe { &(*ptr).inner };
    match snapshot.json() {
        Ok(v) => {
            let s = v.to_string();
            CString::new(s).map(CString::into_raw).unwrap_or(std::ptr::null_mut())
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Get the TestClient response body as a plain text null-terminated string.
///
/// Returns null if `ptr` is null or text extraction fails.
/// The returned string must be freed with `spikard_free_string()`.
///
/// # Safety
/// - `ptr` must be a valid handle returned by a TestClient method.
#[no_mangle]
pub extern "C" fn spikard_test_response_text(ptr: *const TestResponseOpaque) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null check above.
    let snapshot = unsafe { &(*ptr).inner };
    match snapshot.text() {
        Ok(s) => CString::new(s).map(CString::into_raw).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Helper: run a future on a new Tokio runtime and return its output.
///
/// Used because the TestClient methods are async and the FFI layer is synchronous.
fn block_on<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    tokio::runtime::Runtime::new()
        .expect("failed to create Tokio runtime for TestClient")
        .block_on(future)
}

/// Perform a GET request via the TestClient.
///
/// `path` must be a valid null-terminated UTF-8 C string.
/// Returns an opaque response handle or null on failure.
/// The returned pointer must be freed with `spikard_test_response_free()`.
///
/// # Safety
/// - `ptr` must be a valid handle returned by `spikard_test_client_new()`.
/// - `path` must be a valid null-terminated UTF-8 C string.
#[no_mangle]
pub extern "C" fn spikard_test_client_get(ptr: *const TestClientOpaque, path: *const c_char) -> *mut TestResponseOpaque {
    if ptr.is_null() || path.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null checks above.
    let client = match unsafe { (*ptr).inner.as_ref() } {
        Some(c) => c,
        None => return std::ptr::null_mut(),
    };
    // SAFETY: non-null check above; caller guarantees valid UTF-8 C string.
    let path_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();

    match block_on(client.get(&path_str, None, None)) {
        Ok(snapshot) => Box::into_raw(Box::new(TestResponseOpaque { inner: Box::new(snapshot) })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Perform a POST request with a JSON body via the TestClient.
///
/// `path` must be a valid null-terminated UTF-8 C string.
/// `json_body` may be null (sends empty body) or a null-terminated UTF-8 JSON string.
/// Returns an opaque response handle or null on failure.
/// The returned pointer must be freed with `spikard_test_response_free()`.
///
/// # Safety
/// - `ptr` must be a valid handle returned by `spikard_test_client_new()`.
/// - `path` must be a valid null-terminated UTF-8 C string.
/// - `json_body`, if non-null, must be valid UTF-8 JSON.
#[no_mangle]
pub extern "C" fn spikard_test_client_post(
    ptr: *const TestClientOpaque,
    path: *const c_char,
    json_body: *const c_char,
) -> *mut TestResponseOpaque {
    if ptr.is_null() || path.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null checks above.
    let client = match unsafe { (*ptr).inner.as_ref() } {
        Some(c) => c,
        None => return std::ptr::null_mut(),
    };
    // SAFETY: non-null check above; caller guarantees valid UTF-8 C string.
    let path_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();

    let body = if json_body.is_null() {
        None
    } else {
        // SAFETY: non-null check above; caller guarantees valid UTF-8 JSON.
        let raw = unsafe { CStr::from_ptr(json_body) }.to_string_lossy();
        serde_json::from_str::<serde_json::Value>(&raw).ok()
    };

    match block_on(client.post(&path_str, body, None, None, None, None)) {
        Ok(snapshot) => Box::into_raw(Box::new(TestResponseOpaque { inner: Box::new(snapshot) })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Perform a DELETE request via the TestClient.
///
/// # Safety
/// - `ptr` must be a valid handle returned by `spikard_test_client_new()`.
/// - `path` must be a valid null-terminated UTF-8 C string.
#[no_mangle]
pub extern "C" fn spikard_test_client_delete(ptr: *const TestClientOpaque, path: *const c_char) -> *mut TestResponseOpaque {
    if ptr.is_null() || path.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: non-null checks above.
    let client = match unsafe { (*ptr).inner.as_ref() } {
        Some(c) => c,
        None => return std::ptr::null_mut(),
    };
    // SAFETY: non-null check above; caller guarantees valid UTF-8 C string.
    let path_str = unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned();

    match block_on(client.delete(&path_str, None, None)) {
        Ok(snapshot) => Box::into_raw(Box::new(TestResponseOpaque { inner: Box::new(snapshot) })),
        Err(_) => std::ptr::null_mut(),
    }
}
