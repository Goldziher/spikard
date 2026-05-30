#![allow(clippy::too_many_arguments, clippy::unused_async)]

use magnus::{Opaque, RArray, RHash, Value, method, prelude::*};
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
            // Call the Ruby proc with the GVL.
            // Ruby procs are synchronous, so we block_on in a spawn_blocking.
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                // Serialize the request to JSON
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let resp_json = tokio::task::spawn_blocking({
                    let proc_handle = self.proc_handle.clone();
                    let req_json = req_json.clone();
                    move || {
                        Ruby::with_gvl(|ruby| {
                            let proc_value = proc_handle.get_inner_with(&ruby);

                            // Parse request JSON into a Ruby Hash
                            let json_mod = ruby
                                .eval::<_, Value>("JSON")
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                            let req_hash = json_mod
                                .funcall::<_, _, Value>("parse", (&req_json,))
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                            // Call the proc with the request hash
                            let result = proc_value
                                .funcall::<_, _, Value>("call", (req_hash,))
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                            // Serialize result back to JSON
                            let resp_json_str = json_mod
                                .funcall::<_, _, String>("generate", (result,))
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                            Ok::<String, Box<dyn std::error::Error + Send + Sync>>(resp_json_str)
                        })
                    }
                })
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)??
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

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

/// Drive `spikard::App::run` from Ruby.
///
/// Each entry in `registrations` is a `[method_name, metadata_array, proc]` triple
/// produced by the Ruby service class. Constructs an owned service instance,
/// registers all handlers (acquiring GVL for each Ruby proc call), then invokes
/// the entrypoint.
#[magnus::function]
pub fn app_run(registrations: &Opaque<Value>) -> magnus::error::Result<()> {
    let mut owner = spikard::App::new();

    Ruby::with_gvl(|ruby| {
        let regs_value = registrations.get_inner_with(&ruby);
        let regs_array = RArray::try_convert(regs_value)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

        for entry in regs_array.iter() {
            let entry_array = RArray::try_convert(entry)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
            let method_name: String = entry_array
                .get::<String>(0)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
            let proc_value = entry_array
                .get::<Value>(2)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

            match method_name.as_str() {
                "route" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = meta_array
                        .get::<Value>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "get" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Get, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "post" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Post, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "put" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Put, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "patch" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Patch, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "delete" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Delete, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "head" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Head, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "options" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Options, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "connect" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Connect, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "trace" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Trace, path);
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
        Ok::<(), magnus::Error>(())
    })
    .map_err(|e| e)?;

    tokio::runtime::Handle::current()
        .block_on(owner.run())
        .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), e.to_string()))?;
    Ok(())
}

/// Drive `spikard::App::into_router` from Ruby.
///
/// Each entry in `registrations` is a `[method_name, metadata_array, proc]` triple
/// produced by the Ruby service class. Constructs an owned service instance,
/// registers all handlers (acquiring GVL for each Ruby proc call), then invokes
/// the entrypoint.
#[magnus::function]
pub fn app_into_router(registrations: &Opaque<Value>) -> magnus::error::Result<()> {
    let mut owner = spikard::App::new();

    Ruby::with_gvl(|ruby| {
        let regs_value = registrations.get_inner_with(&ruby);
        let regs_array = RArray::try_convert(regs_value)
            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

        for entry in regs_array.iter() {
            let entry_array = RArray::try_convert(entry)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
            let method_name: String = entry_array
                .get::<String>(0)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
            let proc_value = entry_array
                .get::<Value>(2)
                .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

            match method_name.as_str() {
                "route" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = meta_array
                        .get::<Value>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "get" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Get, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "post" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Post, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "put" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Put, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "patch" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Patch, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "delete" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Delete, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "head" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Head, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "options" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Options, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "connect" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Connect, path);
                    owner
                        .route(builder, handler)
                        .map_err(|e| magnus::Error::new(ruby.exception_runtime_error(), e.to_string()))?;
                }
                "trace" => {
                    let bridge = RbHandlerBridge::new(Opaque::new(proc_value));
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let meta_array = RArray::try_convert(
                        entry_array
                            .get::<Value>(1)
                            .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?,
                    )
                    .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;

                    let path: String = meta_array
                        .get::<String>(0)
                        .map_err(|e| magnus::Error::new(ruby.exception_type_error(), e.to_string()))?;
                    let builder: spikard::RouteBuilder = spikard::RouteBuilder(spikard::Method::Trace, path);
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
        Ok::<(), magnus::Error>(())
    })
    .map_err(|e| e)?;

    owner
        .into_router()
        .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), e.to_string()))?;
    Ok(())
}
