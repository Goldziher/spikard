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
#[repr(C)]
pub struct AppOpaque {
    inner: Box<spikard::App>,
}

/// Allocate a new App instance.
///
/// # Safety
/// The returned pointer must be freed via spikard_app_free().
/// Never access the pointer after freeing it.
#[no_mangle]
pub extern "C" fn spikard_app_new() -> *mut AppOpaque {
    let owner = spikard::App::new();
    Box::into_raw(Box::new(AppOpaque { inner: Box::new(owner) }))
}

/// Free a App instance allocated by spikard_app_new().
///
/// # Safety
/// - `ptr` must have been allocated by spikard_app_new().
/// - After this call, `ptr` is invalid and must not be dereferenced.
/// - Calling this twice on the same pointer causes undefined behavior.
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
        return 1; // Error: null pointer
    }

    // SAFETY: pointer was produced by the matching opaque `_new`/builder export and is consumed here.
    let builder = unsafe { *Box::from_raw(builder) };

    let bridge = FfiHandlerBridge { callback, context };
    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);

    // SAFETY: owner was allocated by _new() and is valid until freed.
    match unsafe {
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
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
        return 1; // Error: null pointer
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
        let owner_ref = &mut (*owner).inner;
        owner_ref.route(builder, handler)
    } {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Error
    }
}

/// Run the service entrypoint 'run'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `owner` is consumed by this call; it must not be used or freed afterwards.
#[no_mangle]
pub extern "C" fn spikard_app_ep_run(owner: *mut AppOpaque) -> i32 {
    if owner.is_null() {
        return 1;
    }

    // SAFETY: owner was allocated by _new() (Box::into_raw) and is consumed here.
    let owner = unsafe { Box::from_raw(owner) };
    let inner = *owner.inner;
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    match rt.block_on(inner.run()) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Opaque handle to a running background server.
/// Returned by `spikard_app_ep_start_background`, consumed by `spikard_app_ep_stop`.
pub struct ServerHandle {
    thread: Option<std::thread::JoinHandle<()>>,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

/// Start the HTTP server on a background OS thread and return immediately.
///
/// Unlike `spikard_app_ep_run`, this function does **not** block the calling
/// thread (safe for cgo / JNI / P-Invoke callers).  It spawns a dedicated OS
/// thread outside any managed thread pool, creates a Tokio runtime on that
/// thread, binds the TCP listener, and returns a non-null `*mut ServerHandle`
/// only after the socket is bound and the server is ready to accept connections.
///
/// The caller must eventually pass the returned handle to
/// `spikard_app_ep_stop` to shut the server down and release resources.
///
/// Returns null if the server fails to start (e.g. port already in use or
/// the 10-second bind timeout expires).
///
/// # Parameters
///
/// * `owner` – consumed by this call; must not be used or freed afterwards.
/// * `host`  – null-terminated bind address (e.g. `"127.0.0.1"`), or null to
///             use the address from the App's `ServerConfig`.
/// * `port`  – TCP port to bind, or 0 to use the port from `ServerConfig`.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not
///   yet freed.
/// - `host`, if non-null, must be a valid null-terminated UTF-8 string.
/// - The returned handle must be freed via `spikard_app_ep_stop`.
// <!-- go-ffi-nonblock -->
#[no_mangle]
pub extern "C" fn spikard_app_ep_start_background(
    owner: *mut AppOpaque,
    host: *const c_char,
    port: u16,
) -> *mut ServerHandle {
    if owner.is_null() {
        return std::ptr::null_mut();
    }

    // SAFETY: owner was allocated by _new() (Box::into_raw) and is consumed here.
    let owner = unsafe { Box::from_raw(owner) };
    let inner = *owner.inner;

    // Parse the optional host override from the C string.
    let host_override: Option<String> = if host.is_null() {
        None
    } else {
        // SAFETY: null-checked above; caller guarantees a valid null-terminated UTF-8 string.
        match unsafe { CStr::from_ptr(host) }.to_str() {
            Ok(s) => Some(s.to_owned()),
            Err(_) => return std::ptr::null_mut(),
        }
    };

    // Channel: spawned thread sends Ok(()) once the socket is bound, Err on failure.
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<(), String>>();

    // Oneshot used by the caller to request graceful shutdown.
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let thread = std::thread::spawn(move || {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => {
                let _ = ready_tx.send(Err(format!("failed to create tokio runtime: {e}")));
                return;
            }
        };

        rt.block_on(async move {
            // Decompose the App into its Axum router + server configuration.
            let (router, config) = match inner.into_router_and_config() {
                Ok(pair) => pair,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("failed to build router: {e}")));
                    return;
                }
            };

            // Apply caller-supplied overrides on top of the App's ServerConfig.
            let bind_host = host_override.as_deref().unwrap_or(&config.host);
            let bind_port = if port != 0 { port } else { config.port };
            let addr_str = format!("{bind_host}:{bind_port}");

            let addr: std::net::SocketAddr = match addr_str.parse() {
                Ok(a) => a,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("invalid bind address '{addr_str}': {e}")));
                    return;
                }
            };

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    let _ = ready_tx.send(Err(format!("failed to bind {addr}: {e}")));
                    return;
                }
            };

            // Signal readiness *after* bind, *before* serve — caller can now
            // dial the port.
            let _ = ready_tx.send(Ok(()));

            // Serve until the shutdown signal arrives.
            let _ = axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await;
        });
    });

    // Wait for the server to bind (or fail). 10-second timeout so callers
    // don't block forever when the port is permanently unavailable.
    match ready_rx.recv_timeout(std::time::Duration::from_secs(10)) {
        Ok(Ok(())) => {
            let handle = Box::new(ServerHandle {
                thread: Some(thread),
                shutdown_tx: Some(shutdown_tx),
            });
            Box::into_raw(handle)
        }
        Ok(Err(_msg)) => {
            // Server reported a startup error; thread has already exited.
            let _ = thread.join();
            std::ptr::null_mut()
        }
        Err(_timeout) => {
            // Timed out — request shutdown so the thread can exit cleanly.
            let _ = shutdown_tx.send(());
            let _ = thread.join();
            std::ptr::null_mut()
        }
    }
}

/// Stop a server started by `spikard_app_ep_start_background` and free its handle.
///
/// Sends a graceful-shutdown signal to the background server, waits for the
/// server thread to exit, and releases all resources.
///
/// Passing null is a safe no-op. After this call `handle` must not be used again.
///
/// # Safety
/// - `handle` must have been returned by `spikard_app_ep_start_background` and
///   not yet freed.
/// - Calling this twice on the same pointer causes undefined behavior.
#[no_mangle]
pub extern "C" fn spikard_app_ep_stop(handle: *mut ServerHandle) {
    if handle.is_null() {
        return;
    }

    // SAFETY: handle was allocated by Box::into_raw in start_background; sole owner.
    let mut handle = unsafe { Box::from_raw(handle) };

    // Signal shutdown (best-effort; receiver may already be gone if server crashed).
    if let Some(tx) = handle.shutdown_tx.take() {
        let _ = tx.send(());
    }

    // Join the background thread to ensure it exits before we return.
    if let Some(thread) = handle.thread.take() {
        let _ = thread.join();
    }
    // `handle` is dropped here, freeing the Box.
}

/// Run the service entrypoint 'into_router'.
///
/// # Safety
/// - `owner` must be a valid pointer returned by `spikard_app_new()` and not yet freed.
/// - `owner` is consumed by this call; it must not be used or freed afterwards.
#[no_mangle]
pub extern "C" fn spikard_app_ep_into_router(owner: *mut AppOpaque) -> i32 {
    if owner.is_null() {
        return 1;
    }

    // SAFETY: owner was allocated by _new() (Box::into_raw) and is consumed here.
    let owner = unsafe { Box::from_raw(owner) };
    let inner = *owner.inner;
    match inner.into_router() {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
