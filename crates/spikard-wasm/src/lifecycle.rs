#![cfg(target_arch = "wasm32")]

//! WASM lifecycle hook adapters bridging JavaScript hooks and Rust core.

use http::{Request, Response};
use js_sys::{Array, Function, Object, Promise, Reflect};
use serde::Deserialize;
use serde_json::Value;
use spikard_core::lifecycle::{HookResult, LifecycleHook, LifecycleHooks};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::types::RequestPayload;

type LifecycleRequest = Request<()>;
type LifecycleResponse = Response<()>;
pub type WasmLifecycleHooks = LifecycleHooks<LifecycleRequest, LifecycleResponse>;
type WasmHook = dyn LifecycleHook<LifecycleRequest, LifecycleResponse>;
type HookFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, String>> + 'a>>;

/// Bridge struct wrapping a JS lifecycle function.
pub struct WasmLifecycleHook {
    name: String,
    func: Function,
}

impl WasmLifecycleHook {
    pub fn new(name: String, func: Function) -> Self {
        Self { name, func }
    }
}

#[derive(Clone)]
struct RequestState(RequestPayload);

#[derive(Clone)]
struct ResponseState(Value);

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum JsHookBridgeResult {
    Request { payload: RequestPayload },
    Response { payload: Value },
}

impl LifecycleHook<LifecycleRequest, LifecycleResponse> for WasmLifecycleHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &self,
        req: LifecycleRequest,
    ) -> HookFuture<'a, HookResult<LifecycleRequest, LifecycleResponse>> {
        let func = self.func.clone();
        let name = self.name.clone();

        Box::pin(async move {
            let mut req = req;
            let payload = take_request_state(&mut req)?;
            let input = serde_wasm_bindgen::to_value(&payload)
                .map_err(|err| format!("Failed to serialize request for hook '{}': {}", name, err))?;
            let result = call_hook(&func, input, &name).await?;

            match result {
                JsHookBridgeResult::Request { payload } => {
                    insert_request_state(&mut req, payload);
                    Ok(HookResult::Continue(req))
                }
                JsHookBridgeResult::Response { payload } => {
                    let response = response_from_value(payload)?;
                    Ok(HookResult::ShortCircuit(response))
                }
            }
        })
    }

    fn execute_response<'a>(
        &self,
        resp: LifecycleResponse,
    ) -> HookFuture<'a, HookResult<LifecycleResponse, LifecycleResponse>> {
        let func = self.func.clone();
        let name = self.name.clone();

        Box::pin(async move {
            let mut resp = resp;
            let payload = take_response_state(&mut resp)?;
            let input = serde_wasm_bindgen::to_value(&payload)
                .map_err(|err| format!("Failed to serialize response for hook '{}': {}", name, err))?;
            let result = call_hook(&func, input, &name).await?;

            match result {
                JsHookBridgeResult::Response { payload } => {
                    insert_response_state(&mut resp, payload);
                    Ok(HookResult::Continue(resp))
                }
                JsHookBridgeResult::Request { .. } => {
                    Err(format!("Hook '{}' returned a request during response phase", name))
                }
            }
        })
    }
}

async fn call_hook(func: &Function, input: JsValue, name: &str) -> Result<JsHookBridgeResult, String> {
    let output = func
        .call1(&JsValue::NULL, &input)
        .map_err(|err| format!("Hook '{}' invocation failed: {:?}", name, err))?;

    let promise: Promise = if output.is_instance_of::<Promise>() {
        output
            .dyn_into()
            .map_err(|_| format!("Hook '{}' returned an invalid Promise", name))?
    } else {
        Promise::resolve(&output)
    };

    let value = JsFuture::from(promise)
        .await
        .map_err(|err| format!("Hook '{}' promise rejected: {:?}", name, err))?;

    serde_wasm_bindgen::from_value(value).map_err(|err| format!("Hook '{}' returned invalid payload: {}", name, err))
}

fn insert_request_state(req: &mut LifecycleRequest, mut payload: RequestPayload) {
    payload.ensure_body_metadata();
    req.extensions_mut().insert(RequestState(payload));
}

fn take_request_state(req: &mut LifecycleRequest) -> Result<RequestPayload, String> {
    req.extensions_mut()
        .remove::<RequestState>()
        .map(|state| state.0)
        .ok_or_else(|| "Missing request payload for lifecycle execution".to_string())
}

fn insert_response_state(resp: &mut LifecycleResponse, payload: Value) {
    resp.extensions_mut().insert(ResponseState(payload));
}

fn take_response_state(resp: &mut LifecycleResponse) -> Result<Value, String> {
    resp.extensions_mut()
        .remove::<ResponseState>()
        .map(|state| state.0)
        .ok_or_else(|| "Missing response payload for lifecycle execution".to_string())
}

/// Create a request object carrying the serialized payload for hook execution.
pub fn request_from_payload(payload: RequestPayload) -> Result<LifecycleRequest, String> {
    let builder = Request::builder()
        .method(payload.method.as_str())
        .uri(payload.path.as_str());
    let mut request = builder
        .body(())
        .map_err(|err| format!("Failed to build request for lifecycle hooks: {}", err))?;
    insert_request_state(&mut request, payload);
    Ok(request)
}

/// Extract the request payload from the lifecycle carrier.
pub fn request_into_payload(mut request: LifecycleRequest) -> Result<RequestPayload, String> {
    take_request_state(&mut request)
}

/// Create a response object carrying the structured response payload.
pub fn response_from_value(payload: Value) -> Result<LifecycleResponse, String> {
    let mut response = Response::builder()
        .status(200)
        .body(())
        .map_err(|err| format!("Failed to build response for lifecycle hooks: {}", err))?;
    insert_response_state(&mut response, payload);
    Ok(response)
}

/// Extract the structured response payload from the lifecycle carrier.
pub fn response_into_value(mut response: LifecycleResponse) -> Result<Value, String> {
    take_response_state(&mut response)
}

/// Build lifecycle hooks from the serialized JS representation.
pub fn parse_hooks(hooks_value: &JsValue) -> Result<Option<WasmLifecycleHooks>, JsValue> {
    if hooks_value.is_undefined() || hooks_value.is_null() {
        return Ok(None);
    }

    let hooks_obj: Object = hooks_value
        .clone()
        .dyn_into()
        .map_err(|_| JsValue::from_str("Lifecycle hooks must be an object"))?;

    let mut hooks = WasmLifecycleHooks::new();
    register_hooks(&mut hooks, &hooks_obj, "onRequest", WasmLifecycleHooks::add_on_request)?;
    register_hooks(
        &mut hooks,
        &hooks_obj,
        "preValidation",
        WasmLifecycleHooks::add_pre_validation,
    )?;
    register_hooks(
        &mut hooks,
        &hooks_obj,
        "preHandler",
        WasmLifecycleHooks::add_pre_handler,
    )?;
    register_hooks(
        &mut hooks,
        &hooks_obj,
        "onResponse",
        WasmLifecycleHooks::add_on_response,
    )?;
    register_hooks(&mut hooks, &hooks_obj, "onError", WasmLifecycleHooks::add_on_error)?;

    if hooks.is_empty() { Ok(None) } else { Ok(Some(hooks)) }
}

fn register_hooks<F>(hooks: &mut WasmLifecycleHooks, obj: &Object, key: &str, mut adder: F) -> Result<(), JsValue>
where
    F: FnMut(&mut WasmLifecycleHooks, Arc<WasmHook>),
{
    let value =
        Reflect::get(obj, &JsValue::from_str(key)).map_err(|_| JsValue::from_str("Failed to read lifecycle hooks"))?;
    if value.is_null() || value.is_undefined() {
        return Ok(());
    }

    let array = Array::from(&value);
    for idx in 0..array.length() {
        let func_value = array.get(idx);
        let func: Function = func_value
            .dyn_into()
            .map_err(|_| JsValue::from_str(&format!("Hook '{}' must be a function", key)))?;
        let hook = Arc::new(WasmLifecycleHook::new(format!("{}_{}", key, idx), func)) as Arc<WasmHook>;
        adder(hooks, hook);
    }

    Ok(())
}
