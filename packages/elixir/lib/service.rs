#![allow(clippy::too_many_arguments, clippy::unused_async)]

use rustler::Error as NifError;
use rustler::{Encoder, LocalPid, NifResult, OwnedEnv, ResourceArc, types::atom::Atom};
use spikard::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

/// Atom constants used by the service NIFs.
mod atoms {
    rustler::atoms! {
        ok,
        error,
        trait_call,
    }
}

static REPLY_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
type TraitReplySender = tokio::sync::oneshot::Sender<String>;
type TraitReplyMap = Mutex<HashMap<u64, TraitReplySender>>;

static TRAIT_REPLY_MAP: OnceLock<TraitReplyMap> = OnceLock::new();

fn trait_reply_map() -> &'static TraitReplyMap {
    TRAIT_REPLY_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Complete a pending trait call with the JSON response from Elixir.
#[rustler::nif]
pub fn complete_trait_call(reply_id: u64, response_json: String) -> Atom {
    if let Some(tx) = trait_reply_map().lock().unwrap().remove(&reply_id) {
        let _ = tx.send(response_json);
    }
    atoms::ok()
}
/// Generated rustler bridge for the `Handler` contract.
///
/// Wraps an Elixir GenServer pid so it can be used
/// as `Arc<dyn Handler>` from Rust async code.
/// Uses message-passing to avoid blocking the BEAM scheduler.
/// Pending replies are stored in the module-level `TRAIT_REPLY_MAP`
/// keyed by `reply_id`; the GenServer completes them via the
/// `complete_trait_call` NIF.
pub struct ElixirHandlerBridge {
    pid: LocalPid,
}

impl ElixirHandlerBridge {
    /// Create a bridge from an Elixir GenServer pid.
    pub fn new(pid: LocalPid) -> Self {
        Self { pid }
    }
}

// SAFETY: LocalPid is Send+Sync as guaranteed by Rustler.
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

                let reply_id = REPLY_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                let (tx, rx) = tokio::sync::oneshot::channel();
                trait_reply_map().lock().unwrap().insert(reply_id, tx);

                // Send trait_call message to Elixir GenServer
                {
                    let pid = self.pid;
                    let method_name = "call";
                    let request_json_clone = request_json.clone();
                    tokio::task::spawn_blocking(move || {
                        let mut env = OwnedEnv::new();
                        let _ = env.send_and_clear(&pid, |env| {
                            (
                                Atom::from_str(env, "trait_call").unwrap(),
                                method_name,
                                request_json_clone.as_str(),
                                reply_id,
                            )
                                .encode(env)
                        });
                    })
                    .await
                    .ok();
                }

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
/// `:ok` or `{:error, reason}` after the entrypoint completes.
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
                if let Ok((builder,)) = metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                    let builder: spikard::RouteBuilder =
                        (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                    let bridge = ElixirHandlerBridge::new(handler_pid);
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let _ = owner.route(builder, handler);
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
/// `:ok` or `{:error, reason}` after the entrypoint completes.
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
                if let Ok((builder,)) = metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                    let builder: spikard::RouteBuilder =
                        (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                    let bridge = ElixirHandlerBridge::new(handler_pid);
                    let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                    let _ = owner.route(builder, handler);
                }
            }
        }
    }

    // Call the entrypoint method
    match owner.into_router() {
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
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
            if let Ok((builder,)) = _metadata.decode::<(rustler::ResourceArc<super::RouteBuilder>,)>() {
                let builder: spikard::RouteBuilder = (*builder).inner.read().unwrap_or_else(|e| e.into_inner()).clone();
                let bridge = ElixirHandlerBridge::new(handler_pid);
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let _ = owner.route(builder, handler);
            }
        }
    }

    Ok(atoms::ok())
}
