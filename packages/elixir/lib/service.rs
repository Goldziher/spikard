#![allow(clippy::too_many_arguments, clippy::unused_async)]

use rustler::{LocalPid, ResourceArc};
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

/// Generated rustler bridge for the `Handler` contract.
///
/// Wraps an Elixir GenServer pid so it can be used
/// as `Arc<dyn Handler>` from Rust async code.
/// Uses message-passing to avoid blocking the BEAM scheduler.
pub struct ElixirHandlerBridge {
    pid: LocalPid,
    reply_map: Arc<TokioMutex<std::collections::HashMap<u64, tokio::sync::oneshot::Sender<String>>>>,
}

impl ElixirHandlerBridge {
    /// Create a bridge from an Elixir GenServer pid.
    pub fn new(pid: LocalPid) -> Self {
        Self {
            pid,
            reply_map: Arc::new(TokioMutex::new(std::collections::HashMap::new())),
        }
    }
}

// SAFETY: LocalPid is Send+Sync as guaranteed by Rustler.
// Arc<TokioMutex<HashMap>> is Send+Sync.
unsafe impl Send for ElixirHandlerBridge {}
unsafe impl Sync for ElixirHandlerBridge {}

impl spikard::Handler for ElixirHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard::HandlerResult> + Send + '_>> {
        Box::pin(async move {
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                let request_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let reply_id = crate::nif_support::next_request_id();
                let (tx, rx) = tokio::sync::oneshot::channel();

                {
                    let mut map = self.reply_map.lock().await;
                    map.insert(reply_id, tx);
                }

                // Send trait_call message to Elixir GenServer
                // Note: This requires a NIF that sends the message
                // crate::nif_support::send_trait_call(self.pid, "call", &request_json, reply_id)?;

                // Await response
                let response_json = rx.await.map_err(|e| {
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))
                        as Box<dyn std::error::Error + Send + Sync>
                })?;

                let response: spikard::Response = serde_json::from_str(&response_json)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(response)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}

/// Drive `spikard::App::run` from Elixir.
///
/// This NIF is scheduled on the dirty CPU scheduler to avoid blocking
/// the BEAM scheduler during the (potentially long) run operation.
///
/// # Arguments
///
/// - `registrations` — Elixir list of `{method_name, metadata, handler}` tuples
///   where `handler` is an Elixir function/closure that accepts request JSON and returns response JSON.
///
/// # Returns
/// `:ok` or `{{:error, reason}}` after the entrypoint completes.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_run(registrations: rustler::Term<'_>) -> NifResult<Atom> {
    // Parse registrations from Elixir term
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    // Build the service owner from its constructor
    let mut owner = spikard::App::new();

    // Register handlers from Elixir registrations
    // Each registration entry is a tuple: {method_name, metadata, handler_pid}
    for reg_entry in registration_list {
        if let Ok((method_name, metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if method_name == "route" {
                if let Ok((builder,)) = metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                    let builder: spikard::RouteBuilder = (*builder.inner).clone();
                    let bridge = ElixirHandlerBridge::new(handler_pid);
                    let _ = owner.route(builder, std::sync::Arc::new(bridge));
                }
            }
        }
    }

    // Call the entrypoint method
    let rt = tokio::runtime::Runtime::new().map_err(|_e| NifError::Atom("runtime_error"))?;

    let result = rt.block_on(owner.run());
    match result {
        Ok(_) => Ok(atoms::ok()),
        Err(_e) => Err(NifError::Atom("error")),
    }
}

/// Drive `spikard::App::into_router` from Elixir.
///
/// This NIF is scheduled on the dirty CPU scheduler to avoid blocking
/// the BEAM scheduler during the (potentially long) run operation.
///
/// # Arguments
///
/// - `registrations` — Elixir list of `{method_name, metadata, handler}` tuples
///   where `handler` is an Elixir function/closure that accepts request JSON and returns response JSON.
///
/// # Returns
/// `:ok` or `{{:error, reason}}` after the entrypoint completes.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_into_router(registrations: rustler::Term<'_>) -> NifResult<Atom> {
    // Parse registrations from Elixir term
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    // Build the service owner from its constructor
    let mut owner = spikard::App::new();

    // Register handlers from Elixir registrations
    // Each registration entry is a tuple: {method_name, metadata, handler_pid}
    for reg_entry in registration_list {
        if let Ok((method_name, metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if method_name == "route" {
                if let Ok((builder,)) = metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                    let builder: spikard::RouteBuilder = (*builder.inner).clone();
                    let bridge = ElixirHandlerBridge::new(handler_pid);
                    let _ = owner.route(builder, std::sync::Arc::new(bridge));
                }
            }
        }
    }

    // Call the entrypoint method
    match owner.finalize() {
        Ok(_) => Ok(atoms::ok()),
        Err(_e) => Err(NifError::Atom("error")),
    }
}

/// Registration variant `get` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_get(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Get, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `post` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_post(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Post, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `put` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_put(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Put, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `patch` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_patch(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Patch, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `delete` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_delete(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Delete, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `head` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_head(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Head, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `options` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_options(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Options, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `connect` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_connect(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Connect, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}

/// Registration variant `trace` for the `route` base method.
///
/// This NIF pre-builds the wrapper and delegates to the base registration.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn app_trace(registrations: rustler::Term<'_>, path: String, handler: rustler::LocalPid) -> NifResult<Atom> {
    let registration_list: Vec<rustler::Term<'_>> = registrations
        .decode::<Vec<rustler::Term<'_>>>()
        .unwrap_or_else(|_| vec![]);

    let mut owner = spikard::App::new();

    // Build RouteBuilder via spikard::RouteBuilder
    let wrapper = spikard::RouteBuilder::new(spikard::Method::Trace, path);

    // Register the handler with wrapper or direct metadata
    for reg_entry in registration_list {
        if let Ok((_method, _metadata, handler_pid)) =
            reg_entry.decode::<(String, rustler::Term<'_>, rustler::LocalPid)>()
        {
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder.inner).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let _ = owner.route(builder, wrapper, std::sync::Arc::new(bridge));
            }
        }
    }

    Ok(atoms::ok())
}
