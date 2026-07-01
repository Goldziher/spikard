// Lifecycle hook FFI — included verbatim into service.rs via include!().
// This file is NOT a separate Rust module; it shares the service module's namespace.
// `AppOpaque`, `CStr`, `CString`, `c_char`, `c_void`, and `Arc` are in scope from service.rs.

/// FFI bridge that wraps a C function pointer as a lifecycle hook.
///
/// The callback receives a JSON-serialized request or response and must return
/// a JSON string encoding `{ "action": "continue" | "short_circuit", "body": … }`.
/// The lifecycle hook name is encoded as a null-terminated C string.
struct FfiLifecycleHook {
    name: String,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
}

// SAFETY: The C callback and context are opaque handles whose lifetimes are
// managed by the caller. The caller guarantees they remain valid for the
// lifetime of the hook.
unsafe impl Send for FfiLifecycleHook {}
unsafe impl Sync for FfiLifecycleHook {}

impl spikard::LifecycleHook<axum::http::Request<spikard::Body>, axum::http::Response<spikard::Body>>
    for FfiLifecycleHook
{
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &self,
        req: axum::http::Request<spikard::Body>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<
                        spikard::HookResult<axum::http::Request<spikard::Body>, axum::http::Response<spikard::Body>>,
                        String,
                    >,
                > + Send
                + 'a,
        >,
    > {
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let payload = format!(r#"{{"method":"{method}","uri":"{uri}"}}"#);
        let callback = self.callback;
        let context = self.context as usize;

        Box::pin(async move {
            let c_str = match CString::new(payload) {
                Ok(s) => s,
                Err(_) => return Ok(spikard::HookResult::Continue(req)),
            };

            let resp_addr =
                tokio::task::spawn_blocking(move || callback(context as *mut c_void, c_str.as_ptr()) as usize)
                    .await
                    .map_err(|e| e.to_string())?;

            let resp_ptr = resp_addr as *mut c_char;
            if resp_ptr.is_null() {
                return Ok(spikard::HookResult::Continue(req));
            }

            // SAFETY: resp_ptr is non-null and was produced by the C callback.
            let resp_str = unsafe { CStr::from_ptr(resp_ptr) }.to_string_lossy().into_owned();
            // SAFETY: resp_ptr is non-null and from the C callback.
            unsafe {
                extern "C" {
                    fn free(ptr: *mut std::ffi::c_void);
                }
                free(resp_ptr as *mut std::ffi::c_void);
            }

            let action = serde_json::from_str::<serde_json::Value>(&resp_str)
                .ok()
                .and_then(|v| v.get("action").and_then(|a| a.as_str()).map(str::to_string));

            match action.as_deref() {
                Some("short_circuit") => {
                    let response = axum::http::Response::builder()
                        .status(200)
                        .body(spikard::Body::empty())
                        .unwrap_or_else(|_| axum::http::Response::new(spikard::Body::empty()));
                    Ok(spikard::HookResult::ShortCircuit(response))
                }
                _ => Ok(spikard::HookResult::Continue(req)),
            }
        })
    }

    fn execute_response<'a>(
        &self,
        resp: axum::http::Response<spikard::Body>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<
                        spikard::HookResult<axum::http::Response<spikard::Body>, axum::http::Response<spikard::Body>>,
                        String,
                    >,
                > + Send
                + 'a,
        >,
    > {
        let status = resp.status().as_u16();
        let payload = format!(r#"{{"status":{status}}}"#);
        let callback = self.callback;
        let context = self.context as usize;

        Box::pin(async move {
            let c_str = match CString::new(payload) {
                Ok(s) => s,
                Err(_) => return Ok(spikard::HookResult::Continue(resp)),
            };

            let resp_addr =
                tokio::task::spawn_blocking(move || callback(context as *mut c_void, c_str.as_ptr()) as usize)
                    .await
                    .map_err(|e| e.to_string())?;

            let resp_ptr = resp_addr as *mut c_char;
            if resp_ptr.is_null() {
                return Ok(spikard::HookResult::Continue(resp));
            }

            // SAFETY: resp_ptr is non-null and was produced by the C callback.
            let resp_str = unsafe { CStr::from_ptr(resp_ptr) }.to_string_lossy().into_owned();
            // SAFETY: resp_ptr is non-null and from the C callback.
            unsafe {
                extern "C" {
                    fn free(ptr: *mut std::ffi::c_void);
                }
                free(resp_ptr as *mut std::ffi::c_void);
            }

            let action = serde_json::from_str::<serde_json::Value>(&resp_str)
                .ok()
                .and_then(|v| v.get("action").and_then(|a| a.as_str()).map(str::to_string));

            match action.as_deref() {
                Some("short_circuit") => {
                    let response = axum::http::Response::builder()
                        .status(200)
                        .body(spikard::Body::empty())
                        .unwrap_or_else(|_| axum::http::Response::new(spikard::Body::empty()));
                    Ok(spikard::HookResult::ShortCircuit(response))
                }
                _ => Ok(spikard::HookResult::Continue(resp)),
            }
        })
    }
}

/// Register an `on_request` lifecycle hook on the given App.
///
/// The `callback` receives a JSON-encoded request summary and must return
/// a JSON-encoded action (`{"action":"continue"}` or `{"action":"short_circuit"}`).
/// A null return value from the callback is treated as `continue`.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `name` must be a valid null-terminated UTF-8 C string.
/// - `callback` must remain valid for the lifetime of the App.
/// - `context` must remain valid for the lifetime of the App.
/// Returns 0 on success, non-zero on failure.
#[no_mangle]
pub extern "C" fn spikard_app_on_request(
    owner: *mut AppOpaque,
    name: *const c_char,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
) -> i32 {
    if owner.is_null() || name.is_null() {
        return 1;
    }
    // SAFETY: name is non-null and caller guarantees valid UTF-8 C string.
    let hook_name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    let hook = Arc::new(FfiLifecycleHook {
        name: hook_name,
        callback,
        context,
    });
    // SAFETY: owner was allocated by _new() and is valid until freed.
    unsafe {
        match (*owner).inner.as_mut() {
            Some(app) => app.on_request(hook),
            None => return 1,
        }
    };
    0
}

/// Register a `pre_validation` lifecycle hook on the given App.
///
/// # Safety
/// Same as `spikard_app_on_request`.
#[no_mangle]
pub extern "C" fn spikard_app_pre_validation(
    owner: *mut AppOpaque,
    name: *const c_char,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
) -> i32 {
    if owner.is_null() || name.is_null() {
        return 1;
    }
    let hook_name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    let hook = Arc::new(FfiLifecycleHook {
        name: hook_name,
        callback,
        context,
    });
    unsafe {
        match (*owner).inner.as_mut() {
            Some(app) => app.pre_validation(hook),
            None => return 1,
        }
    };
    0
}

/// Register a `pre_handler` lifecycle hook on the given App.
///
/// # Safety
/// Same as `spikard_app_on_request`.
#[no_mangle]
pub extern "C" fn spikard_app_pre_handler(
    owner: *mut AppOpaque,
    name: *const c_char,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
) -> i32 {
    if owner.is_null() || name.is_null() {
        return 1;
    }
    let hook_name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    let hook = Arc::new(FfiLifecycleHook {
        name: hook_name,
        callback,
        context,
    });
    unsafe {
        match (*owner).inner.as_mut() {
            Some(app) => app.pre_handler(hook),
            None => return 1,
        }
    };
    0
}

/// Register an `on_response` lifecycle hook on the given App.
///
/// # Safety
/// Same as `spikard_app_on_request`.
#[no_mangle]
pub extern "C" fn spikard_app_on_response(
    owner: *mut AppOpaque,
    name: *const c_char,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
) -> i32 {
    if owner.is_null() || name.is_null() {
        return 1;
    }
    let hook_name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    let hook = Arc::new(FfiLifecycleHook {
        name: hook_name,
        callback,
        context,
    });
    unsafe {
        match (*owner).inner.as_mut() {
            Some(app) => app.on_response(hook),
            None => return 1,
        }
    };
    0
}

/// Register an `on_error` lifecycle hook on the given App.
///
/// # Safety
/// Same as `spikard_app_on_request`.
#[no_mangle]
pub extern "C" fn spikard_app_on_error(
    owner: *mut AppOpaque,
    name: *const c_char,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
) -> i32 {
    if owner.is_null() || name.is_null() {
        return 1;
    }
    let hook_name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
    let hook = Arc::new(FfiLifecycleHook {
        name: hook_name,
        callback,
        context,
    });
    unsafe {
        match (*owner).inner.as_mut() {
            Some(app) => app.on_error(hook),
            None => return 1,
        }
    };
    0
}
