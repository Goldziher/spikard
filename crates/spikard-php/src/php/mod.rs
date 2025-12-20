//! ext-php-rs implementation bridging PHP handlers to the Rust core.
//!
//! This module exposes spikard functionality to PHP including:
//! - Request/Response objects
//! - Server configuration
//! - Test client for unit testing
//! - Handler infrastructure

use ext_php_rs::boxed::ZBox;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::error::Error as ExtPhpError;
use ext_php_rs::ffi::zend_hash_str_update;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use serde_json::Value;

pub mod background;
mod di;
mod handler;
mod hooks;
mod request;
mod response;
mod server;
mod sse;
pub(crate) mod start;
mod streaming;
mod testing;
mod websocket;

pub use background::{clear_handle, install_handle, process_pending_tasks};
pub use di::{PhpFactoryDependency, PhpValueDependency, extract_di_container_from_php};
pub use handler::PhpHandler;
pub use handler::get_runtime;
pub use hooks::{PhpHookResult, PhpLifecycleHooks};
pub use request::PhpRequest;
pub use response::PhpResponse;
pub use server::PhpServer;
pub use sse::{PhpSseEventProducer, create_sse_state};
pub use streaming::{StreamingConfig, create_handler_response as create_streaming_response, register_generator};
pub use testing::{
    PhpHttpTestClient, PhpSseEvent, PhpSseStream, PhpTestClient, PhpTestResponse, PhpWebSocketTestConnection,
};
pub use websocket::{PhpWebSocketHandler, create_websocket_state};

fn map_ext_php_err(e: ExtPhpError) -> PhpException {
    PhpException::default(e.to_string())
}

fn php_table_with_capacity(len: usize) -> ZBox<ZendHashTable> {
    if len == 0 {
        ZendHashTable::new()
    } else if len > (u32::MAX as usize) {
        ZendHashTable::new()
    } else {
        ZendHashTable::with_capacity(len as u32)
    }
}

fn table_insert_str_fast<V: IntoZval>(table: &mut ZendHashTable, key: &str, value: V) -> PhpResult<()> {
    // SAFETY:
    // - `zend_hash_str_update` uses (ptr, len) and does not require NUL termination.
    // - `key` lives for the duration of this call.
    // - `val` is a valid Zval owned by Rust; the table takes over after update and we release it.
    let mut val = value.into_zval(false).map_err(map_ext_php_err)?;
    unsafe { zend_hash_str_update(table, key.as_ptr().cast::<i8>(), key.len(), &raw mut val) };
    val.release();
    Ok(())
}

/// Background task wrapper to make wrap_function! work
#[php_function]
#[php(name = "spikard_background_run")]
pub fn spikard_background_run_wrapper(callable: &Zval, args: &Zval) -> PhpResult<()> {
    background::spikard_background_run(callable, args)
}

/// Start server wrapper for PHP.
/// Returns i64 (signed) because PHP integers are signed, not unsigned.
#[php_function]
#[php(name = "spikard_start_server")]
pub fn spikard_start_server(
    routes_zval: &Zval,
    config: &Zval,
    hooks: &Zval,
    dependencies: Option<&Zval>,
) -> PhpResult<i64> {
    let default_deps = Zval::new();
    let deps = dependencies.unwrap_or(&default_deps);
    start::spikard_start_server_impl(routes_zval, config, hooks, deps)
}

/// Stop server wrapper for PHP.
/// Accepts i64 (signed) because PHP integers are signed, converts to u64 internally.
#[php_function]
#[php(name = "spikard_stop_server")]
pub fn spikard_stop_server(_handle: i64) -> PhpResult<()> {
    start::spikard_stop_server_impl(_handle)
}

/// Register the PHP module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .name("spikard")
        .version(env!("CARGO_PKG_VERSION"))
        .function(wrap_function!(spikard_version))
        .function(wrap_function!(spikard_echo_response))
        .function(wrap_function!(spikard_json_response))
        .function(wrap_function!(spikard_parse_json))
        .function(wrap_function!(spikard_start_server))
        .function(wrap_function!(spikard_stop_server))
        .function(wrap_function!(spikard_background_run_wrapper))
        .class::<PhpRequest>()
        .class::<PhpResponse>()
        .class::<PhpServer>()
        .class::<PhpTestClient>()
        .class::<PhpTestResponse>()
        .class::<PhpHttpTestClient>()
        .class::<PhpWebSocketTestConnection>()
        .class::<PhpSseStream>()
        .class::<PhpSseEvent>()
}

/// Return the crate version.
#[php_function]
#[php(name = "spikard_version")]
pub fn spikard_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Echo a response for sanity checks.
#[php_function]
#[php(name = "spikard_echo_response")]
pub fn spikard_echo_response(body: String) -> PhpResponse {
    PhpResponse::text(body, Some(200))
}

/// Create a JSON response.
#[php_function]
#[php(name = "spikard_json_response")]
pub fn spikard_json_response(body: String, status: Option<i64>) -> PhpResult<PhpResponse> {
    let value: Value = serde_json::from_str(&body).map_err(|e| PhpException::default(format!("Invalid JSON: {e}")))?;
    Ok(PhpResponse::json(value, status))
}

/// Parse a JSON string into a PHP array.
#[php_function]
#[php(name = "spikard_parse_json")]
pub fn spikard_parse_json(json: String) -> PhpResult<ZBox<ZendHashTable>> {
    let value: Value = serde_json::from_str(&json).map_err(|e| PhpException::default(format!("Invalid JSON: {e}")))?;
    json_to_php_table(&value)
}
/// Convert a serde_json Value to a ZendHashTable.
pub fn json_to_php_table(value: &Value) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = match value {
        Value::Object(map) => php_table_with_capacity(map.len()),
        Value::Array(arr) => php_table_with_capacity(arr.len()),
        _ => ZendHashTable::new(),
    };

    match value {
        Value::Object(map) => {
            for (k, v) in map {
                match v {
                    Value::Null => table_insert_str_fast(&mut table, k.as_str(), ())?,
                    Value::Bool(b) => table_insert_str_fast(&mut table, k.as_str(), *b)?,
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            table_insert_str_fast(&mut table, k.as_str(), i)?;
                        } else if let Some(f) = n.as_f64() {
                            table_insert_str_fast(&mut table, k.as_str(), f)?;
                        }
                    }
                    Value::String(s) => table_insert_str_fast(&mut table, k.as_str(), s.as_str())?,
                    Value::Array(arr) => {
                        let inner = json_array_to_php(arr)?;
                        table_insert_str_fast(&mut table, k.as_str(), inner)?;
                    }
                    Value::Object(_) => {
                        let inner = json_to_php_table(v)?;
                        table_insert_str_fast(&mut table, k.as_str(), inner)?;
                    }
                };
            }
        }
        Value::Array(arr) => {
            return json_array_to_php(arr);
        }
        _ => {}
    }

    Ok(table)
}

/// Convert a serde_json Value into a PHP Zval.
pub fn json_to_zval(value: &Value) -> PhpResult<Zval> {
    match value {
        Value::Null => Ok(Zval::new()),
        Value::Bool(b) => b.into_zval(false).map_err(map_ext_php_err),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_zval(false).map_err(map_ext_php_err)
            } else if let Some(f) = n.as_f64() {
                f.into_zval(false).map_err(map_ext_php_err)
            } else {
                Ok(Zval::new())
            }
        }
        Value::String(s) => s.as_str().into_zval(false).map_err(map_ext_php_err),
        Value::Array(_) | Value::Object(_) => {
            let table = json_to_php_table(value)?;
            table.into_zval(false).map_err(map_ext_php_err)
        }
    }
}

/// Convert a JSON array to a ZendHashTable.
fn json_array_to_php(arr: &[Value]) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = php_table_with_capacity(arr.len());

    for v in arr {
        match v {
            Value::Null => table.push(()).map_err(map_ext_php_err)?,
            Value::Bool(b) => table.push(*b).map_err(map_ext_php_err)?,
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    table.push(i).map_err(map_ext_php_err)?;
                } else if let Some(f) = n.as_f64() {
                    table.push(f).map_err(map_ext_php_err)?;
                }
            }
            Value::String(s) => table.push(s.as_str()).map_err(map_ext_php_err)?,
            Value::Array(inner_arr) => {
                let inner = json_array_to_php(inner_arr)?;
                table.push(inner).map_err(map_ext_php_err)?;
            }
            Value::Object(_) => {
                let inner = json_to_php_table(v)?;
                table.push(inner).map_err(map_ext_php_err)?;
            }
        };
    }

    Ok(table)
}

/// Helper to convert Zval to JSON Value.
pub fn zval_to_json(value: &ext_php_rs::types::Zval) -> Result<Value, String> {
    if value.is_null() {
        return Ok(Value::Null);
    }

    if let Some(b) = value.bool() {
        return Ok(Value::Bool(b));
    }

    if let Some(l) = value.long() {
        return Ok(Value::from(l));
    }

    if let Some(d) = value.double() {
        return Ok(serde_json::Number::from_f64(d)
            .map(Value::Number)
            .unwrap_or(Value::Null));
    }

    if let Some(s) = value.string() {
        return Ok(Value::String(s.to_string()));
    }

    if let Some(arr) = value.array() {
        let mut map = serde_json::Map::new();
        let mut is_sequential = true;
        let mut expected_idx = 0i64;

        for (key, val) in arr.iter() {
            let key_str = match key {
                ext_php_rs::types::ArrayKey::Long(i) => {
                    if i != expected_idx {
                        is_sequential = false;
                    }
                    expected_idx += 1;
                    i.to_string()
                }
                ext_php_rs::types::ArrayKey::String(s) => {
                    is_sequential = false;
                    s.to_string()
                }
                ext_php_rs::types::ArrayKey::Str(s) => {
                    is_sequential = false;
                    s.to_string()
                }
            };

            let json_val = zval_to_json(val)?;
            map.insert(key_str, json_val);
        }

        if is_sequential && !map.is_empty() {
            let arr: Vec<Value> = map.into_iter().map(|(_, v)| v).collect();
            return Ok(Value::Array(arr));
        }

        return Ok(Value::Object(map));
    }

    Ok(Value::Null)
}

/// Helper to read a string from Zval.
pub fn read_string(value: &ext_php_rs::types::Zval, key: &str) -> PhpResult<String> {
    value
        .string()
        .map(|s| s.to_string())
        .ok_or_else(|| PhpException::default(format!("'{key}' must be a string")))
}

/// Helper to read a bool from Zval.
pub fn read_bool(value: &ext_php_rs::types::Zval, key: &str) -> PhpResult<bool> {
    value
        .bool()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be a boolean")))
}

/// Helper to read an i64 from Zval.
pub fn read_long(value: &ext_php_rs::types::Zval, key: &str) -> PhpResult<i64> {
    value
        .long()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be an integer")))
}
