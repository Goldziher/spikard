#![allow(clippy::too_many_arguments, clippy::unused_async)]

use magnus::{
    RArray, RHash, Ruby, Value, method,
    prelude::*,
    value::{InnerValue, Opaque},
};
use std::sync::Arc;

/// Generated Magnus bridge for the `Handler` contract.
///
/// Wraps a Ruby proc so it can be used as `Arc<dyn Handler>`
/// from Rust async code. Calls the proc with GVL acquired.
pub struct RbHandlerBridge {
    proc_handle: Opaque<Value>,
}

impl RbHandlerBridge {
    /// Create a bridge from a Ruby proc.
    pub fn new(proc_handle: Opaque<Value>) -> Self {
        Self { proc_handle }
    }
}

// SAFETY: Opaque<Value> is Send+Sync; calls acquire the GVL.
unsafe impl Send for RbHandlerBridge {}
unsafe impl Sync for RbHandlerBridge {}

impl spikard::Handler for RbHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            // Call the Ruby proc with the GVL via spawn_blocking.
            // Ruby procs are synchronous, so we block on the spawned task.
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                // Serialize the request to JSON
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let resp_json = tokio::task::spawn_blocking({
                    let proc_handle = self.proc_handle.clone();
                    let req_json = req_json.clone();
                    move || {
                        // SAFETY: rb_sys::rb_thread_call_with_gvl acquires the GVL.
                        // We pass a callback that will be invoked with the GVL held.
                        call_ruby_proc_with_gvl(&proc_handle, &req_json)
                    }
                })
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)??;

                // Deserialize the JSON result back into the wire response DTO.
                let response: spikard::Response = serde_json::from_str(&resp_json)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(response)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}

/// Call a Ruby proc with the GVL acquired via rb_sys.
/// This function is called from a tokio spawn_blocking task (non-GVL context).
fn call_ruby_proc_with_gvl(
    proc_handle: &Opaque<Value>,
    req_json: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let box_err = |e: Box<dyn std::error::Error + Send + Sync>| e;

    // SAFETY: rb_thread_call_with_gvl is safe to call from any thread.
    // It acquires the GVL and calls the callback with it held.
    // We use a helper extern fn to bridge the gap.
    unsafe {
        let mut state = RubyProcCallState {
            proc_handle: proc_handle.clone(),
            req_json: req_json.to_string(),
            result: None,
        };
        rb_sys::rb_thread_call_with_gvl(
            Some(ruby_proc_gvl_callback),
            &mut state as *mut _ as *mut std::ffi::c_void,
        );
        state.result.unwrap_or_else(|| {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "GVL callback failed to set result",
            )) as Box<dyn std::error::Error + Send + Sync>)
        })
    }
}

struct RubyProcCallState {
    proc_handle: Opaque<Value>,
    req_json: String,
    result: Option<Result<String, Box<dyn std::error::Error + Send + Sync>>>,
}

// SAFETY: RubyProcCallState is only accessed from within the GVL callback.
unsafe impl Send for RubyProcCallState {}
unsafe impl Sync for RubyProcCallState {}

// Callback invoked by rb_thread_call_with_gvl with the GVL held.
extern "C" fn ruby_proc_gvl_callback(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    // SAFETY: data is a pointer to our RubyProcCallState, guaranteed valid for the duration of the callback.
    unsafe {
        let state = &mut *(data as *mut RubyProcCallState);
        let box_err = |e: Box<dyn std::error::Error + Send + Sync>| e;

        // We are now on a Ruby thread with the GVL held. Safe to call Magnus APIs.
        let ruby = match Ruby::get() {
            Ok(r) => r,
            Err(_) => {
                state.result = Some(Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not obtain Ruby handle within GVL callback",
                )) as Box<dyn std::error::Error + Send + Sync>));
                return std::ptr::null_mut();
            }
        };

        let proc_value = state.proc_handle.get_inner_with(&ruby);

        // Parse request JSON into a Ruby Hash
        let json_mod = match ruby.eval::<Value>("JSON") {
            Ok(m) => m,
            Err(e) => {
                state.result = Some(Err(
                    Box::new(std::io::Error::other(e.to_string())) as Box<dyn std::error::Error + Send + Sync>
                ));
                return std::ptr::null_mut();
            }
        };

        let req_hash = match json_mod.funcall::<_, _, Value>("parse", (state.req_json.as_str(),)) {
            Ok(h) => h,
            Err(e) => {
                state.result = Some(Err(
                    Box::new(std::io::Error::other(e.to_string())) as Box<dyn std::error::Error + Send + Sync>
                ));
                return std::ptr::null_mut();
            }
        };

        // Call the proc with the request hash
        let result = match proc_value.funcall::<_, _, Value>("call", (req_hash,)) {
            Ok(r) => r,
            Err(e) => {
                state.result = Some(Err(
                    Box::new(std::io::Error::other(e.to_string())) as Box<dyn std::error::Error + Send + Sync>
                ));
                return std::ptr::null_mut();
            }
        };

        // Serialize result back to JSON
        match json_mod.funcall::<_, _, String>("generate", (result,)) {
            Ok(resp_json_str) => {
                state.result = Some(Ok(resp_json_str));
            }
            Err(e) => {
                state.result = Some(Err(
                    Box::new(std::io::Error::other(e.to_string())) as Box<dyn std::error::Error + Send + Sync>
                ));
            }
        }
    }
    std::ptr::null_mut()
}

/// Drive `spikard::App::run` from Ruby.
///
/// Each entry in `registrations` is a `[method_name, metadata_array, proc]` triple
/// produced by the Ruby service class. Constructs an owned service instance,
/// registers all handlers (acquiring GVL for each Ruby proc call), then invokes
/// the entrypoint.
///
/// This function runs on a Ruby thread (entered via function! macro from init), so the GVL is already held.
pub fn app_run(registrations: Value) -> magnus::error::Result<()> {
    let mut owner = spikard::App::new();

    let ruby = Ruby::get().expect("function! macro callbacks run on a Ruby thread");

    let regs_array = RArray::try_convert(registrations)
        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

    for i in 0..regs_array.len() {
        let entry = regs_array
            .entry::<Value>(i as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let entry_array =
            RArray::try_convert(entry).map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let method_name: String = entry_array
            .entry::<String>(0 as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let proc_value = entry_array
            .entry::<Value>(2 as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

        match method_name.as_str() {
            "route" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: &crate::RouteBuilder = magnus::TryConvert::try_convert(
                    meta_array
                        .entry::<Value>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder = builder.inner.as_ref().clone();
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "get" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Get, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "post" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Post, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "put" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Put, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "patch" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Patch, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "delete" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Delete, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "head" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Head, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "options" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Options, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "connect" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Connect, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "trace" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Trace, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            _ => {
                return Err(magnus::Error::new(
                    ruby.exception_arg_error(),
                    format!("unknown registration method: {method_name}"),
                ));
            }
        }
    }

    tokio::runtime::Handle::current()
        .block_on(owner.run())
        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
    Ok(())
}

/// Drive `spikard::App::into_router` from Ruby.
///
/// Each entry in `registrations` is a `[method_name, metadata_array, proc]` triple
/// produced by the Ruby service class. Constructs an owned service instance,
/// registers all handlers (acquiring GVL for each Ruby proc call), then invokes
/// the entrypoint.
///
/// This function runs on a Ruby thread (entered via function! macro from init), so the GVL is already held.
pub fn app_into_router(registrations: Value) -> magnus::error::Result<()> {
    let mut owner = spikard::App::new();

    let ruby = Ruby::get().expect("function! macro callbacks run on a Ruby thread");

    let regs_array = RArray::try_convert(registrations)
        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

    for i in 0..regs_array.len() {
        let entry = regs_array
            .entry::<Value>(i as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let entry_array =
            RArray::try_convert(entry).map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let method_name: String = entry_array
            .entry::<String>(0 as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
        let proc_value = entry_array
            .entry::<Value>(2 as isize)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

        match method_name.as_str() {
            "route" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: &crate::RouteBuilder = magnus::TryConvert::try_convert(
                    meta_array
                        .entry::<Value>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder = builder.inner.as_ref().clone();
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "get" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Get, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "post" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Post, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "put" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Put, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "patch" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Patch, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "delete" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Delete, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "head" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Head, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "options" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Options, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "connect" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Connect, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            "trace" => {
                let bridge = RbHandlerBridge::new(proc_value.into());
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_array = RArray::try_convert(
                    entry_array
                        .entry::<Value>(1 as isize)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                )
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                let path: String = meta_array
                    .entry::<String>(0)
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                let builder: spikard::RouteBuilder = spikard::RouteBuilder::new(spikard::Method::Trace, path);
                owner
                    .route(builder, handler)
                    .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
            }
            _ => {
                return Err(magnus::Error::new(
                    ruby.exception_arg_error(),
                    format!("unknown registration method: {method_name}"),
                ));
            }
        }
    }

    owner
        .into_router()
        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
    Ok(())
}
