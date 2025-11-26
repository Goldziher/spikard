//! ext-php-rs implementation.

use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendCallable, Zval};
use serde_json::Value;
use spikard_http::{Handler, RequestData};
use std::collections::HashMap;
use std::sync::Arc;
use urlencoding::decode;

mod request;
mod response;

pub use request::PhpRequest;
pub use response::PhpResponse;

#[derive(Clone)]
struct PhpHandlerEntry {
    handler: Zval,
}

impl PhpHandlerEntry {
    fn matches(&self, req: &PhpRequest) -> Result<bool, ext_php_rs::error::Error> {
        self.handler
            .try_call_method("matches", vec![req])
            .map(|z| z.bool().unwrap_or(false))
    }

    fn handle(&self, req: &PhpRequest) -> Result<Zval, ext_php_rs::error::Error> {
        self.handler.try_call_method("handle", vec![req])
    }
}

#[derive(Clone)]
struct PhpRoute {
    method: String,
    path: String,
    handlers: Vec<PhpHandlerEntry>,
}

impl PhpRoute {
    fn from_php(value: &Zval) -> Result<Self, ext_php_rs::error::Error> {
        let array = ext_php_rs::types::array::Array::try_from(value)?;
        let method: String = array
            .get("method")?
            .and_then(|z| z.string())
            .ok_or(ext_php_rs::error::Error::Internal("missing method".into()))?
            .to_string();
        let path: String = array
            .get("path")?
            .and_then(|z| z.string())
            .ok_or(ext_php_rs::error::Error::Internal("missing path".into()))?
            .to_string();

        let mut handlers = Vec::new();
        if let Some(handler_arr) = array.get("handler")? {
            handlers.push(PhpHandlerEntry {
                handler: handler_arr.clone(),
            });
        } else if let Some(handlers_arr) = array.get("handlers")? {
            if let Ok(arr) = ext_php_rs::types::array::Array::try_from(handlers_arr) {
                for (_, handler_zval) in arr.iter() {
                    handlers.push(PhpHandlerEntry {
                        handler: handler_zval.clone(),
                    });
                }
            }
        }

        if handlers.is_empty() {
            return Err(ext_php_rs::error::Error::Internal(
                "route requires at least one handler".into(),
            ));
        }

        Ok(Self { method, path, handlers })
    }
}

impl Handler for PhpHandlerWrapper {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = spikard_http::HandlerResult> + Send + '_>> {
        Box::pin(async {
            Err((
                axum::http::StatusCode::NOT_IMPLEMENTED,
                "php handler bridge not implemented".into(),
            ))
        })
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function("spikard_version", spikard_version)
        .function("spikard_echo_response", spikard_echo_response)
        .class::<NativeTestClient>()
        .class::<PhpRequest>()
        .class::<PhpResponse>()
}

/// Return the crate version.
#[php_function]
pub fn spikard_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Placeholder entrypoint: echo a response for sanity checks.
#[php_function]
pub fn spikard_echo_response(body: &str) -> PhpResponse {
    PhpResponse::json(body.into(), Some(200))
}

/// Native TestClient for driving PHP handlers through Rust.
#[php_class(name = "Spikard\\Native\\TestClient")]
pub struct NativeTestClient {
    routes: Vec<PhpRoute>,
}

#[php_impl]
impl NativeTestClient {
    #[constructor]
    pub fn __construct(routes: Vec<Zval>) -> Result<Self, PhpException> {
        let mut parsed = Vec::new();
        for route in routes {
            parsed.push(PhpRoute::from_php(&route).map_err(|e| PhpException::default(e.to_string()))?);
        }
        Ok(Self { routes: parsed })
    }

    /// Minimal request dispatcher used by PHP TestClient.
    pub fn request(
        &self,
        method: String,
        path: String,
        options: Option<HashMap<String, Zval>>,
    ) -> Result<PhpResponse, PhpException> {
        let uppercase_method = method.to_ascii_uppercase();
        let (path_only, query) = split_path_and_query(&path);

        // Build PhpRequest to feed into handler::matches/handle
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut cookies: HashMap<String, String> = HashMap::new();
        let mut files: HashMap<String, Value> = HashMap::new();
        let mut body: Value = Value::Null;

        if let Some(opts) = options {
            if let Some(h) = opts.get("headers") {
                if let Ok(arr) = ext_php_rs::types::array::Array::try_from(h) {
                    for (k, v) in arr.iter() {
                        if let (Some(key), Some(val)) = (k.string(), v.string()) {
                            headers.insert(key.to_string(), val.to_string());
                        }
                    }
                }
            }
            if let Some(c) = opts.get("cookies") {
                if let Ok(arr) = ext_php_rs::types::array::Array::try_from(c) {
                    for (k, v) in arr.iter() {
                        if let (Some(key), Some(val)) = (k.string(), v.string()) {
                            cookies.insert(key.to_string(), val.to_string());
                        }
                    }
                }
            }
            if let Some(f) = opts.get("files") {
                if let Ok(arr) = ext_php_rs::types::array::Array::try_from(f) {
                    for (k, v) in arr.iter() {
                        if let Some(key) = k.string() {
                            if let Ok(json_val) = serde_json::to_value(v.to_string()) {
                                files.insert(key.to_string(), json_val);
                            }
                        }
                    }
                }
            }
            if let Some(b) = opts.get("body") {
                body = zval_to_json(b).unwrap_or(Value::Null);
            } else if !files.is_empty() {
                body = serde_json::to_value(files.clone()).unwrap_or(Value::Null);
            }
        }

        let req = PhpRequest::new(
            uppercase_method.clone(),
            path_only.clone(),
            Some(body),
            Some(headers.clone()),
            Some(cookies.clone()),
            Some(query.clone()),
        );

        for route in &self.routes {
            if route.method == uppercase_method && route.path == path_only {
                for handler in &route.handlers {
                    let matches = handler
                        .matches(&req)
                        .map_err(|e| PhpException::default(e.to_string()))?;
                    if matches {
                        let zval_resp = handler.handle(&req).map_err(|e| PhpException::default(e.to_string()))?;
                        let response = response_from_zval(&zval_resp)?;
                        return Ok(response);
                    }
                }
            }
        }

        Err(PhpException::default(format!(
            "No handler registered for {} {}",
            uppercase_method, path
        )))
    }
}

fn split_path_and_query(path: &str) -> (String, HashMap<String, Vec<String>>) {
    if let Some((p, q)) = path.split_once('?') {
        let mut query_map = HashMap::new();
        for pair in q.split('&') {
            if pair.is_empty() {
                continue;
            }
            let mut iter = pair.splitn(2, '=');
            let key = iter.next().unwrap_or("");
            let val = iter.next().unwrap_or("");
            let decoded_key = decode(key).unwrap_or_else(|_| key.into()).to_string();
            let decoded_val = decode(val).unwrap_or_else(|_| val.into()).to_string();
            query_map.entry(decoded_key).or_insert_with(Vec::new).push(decoded_val);
        }
        (p.to_string(), query_map)
    } else {
        (path.to_string(), HashMap::new())
    }
}

fn zval_to_json(value: &Zval) -> Result<Value, serde_json::Error> {
    if let Some(arr) = ext_php_rs::types::array::Array::try_from(value).ok() {
        let mut map = serde_json::Map::new();
        for (k, v) in arr.iter() {
            let key = k.string().unwrap_or_else(|| "".into()).to_string();
            map.insert(key, Value::String(v.to_string()));
        }
        Ok(Value::Object(map))
    } else if let Some(str_val) = value.string() {
        Ok(Value::String(str_val.to_string()))
    } else if let Some(long) = value.long() {
        Ok(Value::from(long))
    } else if let Some(b) = value.bool() {
        Ok(Value::Bool(b))
    } else {
        Ok(Value::Null)
    }
}

fn response_from_zval(z: &Zval) -> Result<PhpResponse, PhpException> {
    // Attempt to extract properties (body, statusCode, headers)
    let status = z.get_property("statusCode").ok().and_then(|v| v.long()).unwrap_or(200) as i64;

    let mut headers = HashMap::new();
    if let Ok(h) = z.get_property("headers") {
        if let Ok(arr) = ext_php_rs::types::array::Array::try_from(&h) {
            for (k, v) in arr.iter() {
                if let (Some(key), Some(val)) = (k.string(), v.string()) {
                    headers.insert(key.to_string().to_ascii_lowercase(), val.to_string());
                }
            }
        }
    }

    let body_zval = z.get_property("body").unwrap_or_else(|_| Zval::new());
    let body_val = zval_to_json(&body_zval).unwrap_or(Value::Null);

    Ok(PhpResponse::json(body_val, Some(status)).with_headers(headers))
}
