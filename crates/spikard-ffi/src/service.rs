#![allow(
    clippy::too_many_arguments,
    clippy::not_unsafe_ptr_arg_deref,
    unused_variables,
    unused_mut
)]

use std::ffi::{c_char, c_void, CStr, CString};
use std::panic;
use std::sync::Arc;
/// FFI handler bridge for the `Handler` contract.
///
/// Wraps a C callback function pointer so it can be called from Rust async code.
/// The callback receives JSON-serialized request and returns JSON response.
pub struct FfiHandlerBridge {
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
}

// SAFETY: The C callback function pointer and context pointer are opaque handles.
// The caller is responsible for maintaining the invariant that the context
// pointer remains valid for the lifetime of the bridge. The callback itself
// must be safe to call from async Rust code.
unsafe impl Send for FfiHandlerBridge {}
unsafe impl Sync for FfiHandlerBridge {}
impl spikard::Handler for FfiHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                // Serialize request to JSON
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                let req_c_str =
                    CString::new(req_json).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                // Call the C callback on a blocking thread to avoid stalling the async executor.
                // Raw pointers are not `Send`, so the context and result pointers cross the
                // spawn_blocking boundary as `usize`; the owned `CString` moves in to stay alive.
                let callback = self.callback;
                let context = self.context as usize;
                let resp_addr = tokio::task::spawn_blocking(move || {
                    (callback)(context as *mut c_void, req_c_str.as_ptr()) as usize
                })
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                let resp_ptr = resp_addr as *mut c_char;

                if resp_ptr.is_null() {
                    return Err("C callback returned null response".into());
                }

                // SAFETY: resp_ptr was returned by the C callback and must be a null-terminated string.
                let resp_c_str = unsafe { CStr::from_ptr(resp_ptr) };
                let resp_json = resp_c_str.to_string_lossy();

                // Deserialize response from JSON
                let response: spikard::Response = serde_json::from_str(&resp_json)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                // Free the C-allocated response string. The host allocates it via malloc/strdup
                // and hands ownership to us; we release it with the C runtime's free.
                // SAFETY: resp_ptr is null-checked above and was produced by the host callback.
                unsafe {
                    extern "C" {
                        fn free(ptr: *mut c_void);
                    }
                    free(resp_ptr as *mut c_void);
                }

                Ok(response)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}
/// Opaque handle to a App service instance.
/// Allocated by spikard_app_new(), freed by spikard_app_free().
///
/// `inner` is `Option<Box<…>>` so a `Finalize` entrypoint can `.take()` the
/// owner out without invalidating the C pointer. Consumers must still call
/// `spikard_app_free()` after a `Finalize` returns to release the opaque
/// shell; the shell drops trivially when `inner` is `None`.
#[repr(C)]
pub struct AppOpaque {
    inner: Option<Box<spikard::App>>,
}

/// Allocate a new App instance.
///
/// # Safety
/// The returned pointer must be freed via spikard_app_free().
/// Never access the pointer after freeing it.
#[no_mangle]
pub extern "C" fn spikard_app_new() -> *mut AppOpaque {
    let owner = spikard::App::new();
    Box::into_raw(Box::new(AppOpaque {
        inner: Some(Box::new(owner)),
    }))
}

/// Free a App instance allocated by spikard_app_new().
///
/// # Safety
/// - `ptr` must have been allocated by spikard_app_new().
/// - After this call, `ptr` is invalid and must not be dereferenced.
/// - Calling this twice on the same pointer causes undefined behavior.
/// - Safe to call even after a `Finalize` entrypoint has emptied `inner`.
#[no_mangle]
pub extern "C" fn spikard_app_free(ptr: *mut AppOpaque) {
    if !ptr.is_null() {
        // SAFETY: ptr was allocated by into_raw above;
        // we are the sole owner and this is the final drop.
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}
/// Register a handler callback for method 'route'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_register_route(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    builder: *mut spikard::RouteBuilder,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if builder.is_null() {
        return 1; // Error: null pointer;
    }

    // SAFETY: pointer was produced by the matching opaque `_new`/builder export.
    // Borrow it as a reference; caller retains ownership and is responsible for freeing.
    let builder = unsafe { &*builder };

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder.clone(), handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a GET route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_get(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Get, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a POST route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_post(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Post, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a PUT route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_put(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Put, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a PATCH route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_patch(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Patch, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a DELETE route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_delete(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Delete, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a HEAD route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_head(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Head, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register an OPTIONS route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_options(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Options, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a CONNECT route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_connect(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Connect, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Register a TRACE route at the given path.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `callback` must be a valid function pointer that remains valid for the lifetime
///   of this service instance.
/// - `context` is an opaque pointer passed to the callback on each invocation.
///   The caller is responsible for keeping it valid.
/// Returns 0 on success, non-zero error code on failure.
#[no_mangle]
pub extern "C" fn spikard_app_trace(
    owner: *mut AppOpaque,
    callback: extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    context: *mut c_void,
    path: *const c_char,
) -> i32 {
    if owner.is_null() {
        return 1; // Error: null pointer
    }
    if path.is_null() {
        return 1; // Error: null pointer;
    }

    let path = if path.is_null() {
        String::new()
    } else {
        // SAFETY: caller guarantees a valid null-terminated C string.
        unsafe { CStr::from_ptr(path) }.to_string_lossy().into_owned()
    };

    let builder = spikard::RouteBuilder::new(spikard::Method::Trace, path);

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        match (*owner).inner.as_mut() {
            Some(owner_ref) => owner_ref.route(builder, handler),
            None => return 1, // Error: service already consumed
        }
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}
/// Configure the service via 'config'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - The same `owner` pointer is returned on success — the caller does **not**
///   need to swap the handle they hold. Returns `null` on failure (the original
///   handle is still valid in that case but should be inspected for usability).
#[no_mangle]
pub extern "C" fn spikard_app_config(owner: *mut AppOpaque, config: *mut spikard::ServerConfig) -> *mut AppOpaque {
    if owner.is_null() {
        return std::ptr::null_mut();
    }
    if config.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: pointer was produced by the matching opaque `_new`/builder export.
    // Borrow it as a reference; caller retains ownership and is responsible for freeing.
    let config = unsafe { &*config };

    // SAFETY: owner was allocated by _new() and is valid until freed.
    // Take the inner box out, transform it, and put the result back. The opaque
    // shell stays at the same address so the caller's handle remains valid.
    unsafe {
        let inner = match (*owner).inner.take() {
            Some(boxed) => *boxed,
            None => return std::ptr::null_mut(),
        };
        (*owner).inner = Some(Box::new(inner.config(config.clone())));
    }
    owner
}
/// Run the service entrypoint 'run'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - The inner owner value is moved out by this call and the opaque shell is left
///   with `inner = None`. The caller may still invoke `()`
///   afterwards to release the shell; subsequent registration/configurator calls
///   on the same pointer will fail with the null-return error code.
#[no_mangle]
pub extern "C" fn spikard_app_ep_run(owner: *mut AppOpaque) -> i32 {
    if owner.is_null() {
        return 1;
    }

    // SAFETY: owner was allocated by _new() and is valid until freed.
    // Move the inner owner out; leave `None` behind so the consumer's deferred
    // `_free` call drops the empty shell instead of touching a moved-from box.
    let inner = match unsafe { (*owner).inner.take() } {
        Some(boxed) => *boxed,
        None => return 1,
    };
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    match rt.block_on(inner.run()) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
/// Run the service entrypoint 'into_router'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - The inner owner value is moved out by this call and the opaque shell is left
///   with `inner = None`. The caller may still invoke `()`
///   afterwards to release the shell; subsequent registration/configurator calls
///   on the same pointer will fail with the null-return error code.
#[no_mangle]
pub extern "C" fn spikard_app_ep_into_router(owner: *mut AppOpaque) -> i32 {
    if owner.is_null() {
        return 1;
    }

    // SAFETY: owner was allocated by _new() and is valid until freed.
    // Move the inner owner out; leave `None` behind so the consumer's deferred
    // `_free` call drops the empty shell instead of touching a moved-from box.
    let inner = match unsafe { (*owner).inner.take() } {
        Some(boxed) => *boxed,
        None => return 1,
    };
    match inner.into_router() {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
