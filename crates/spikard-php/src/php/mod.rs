//! ext-php-rs implementation bridging PHP handlers to the Rust core.
//!
//! This module exposes spikard functionality to PHP including:
//! - Request/Response objects
//! - Server configuration
//! - Test client for unit testing
//! - Handler infrastructure

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use serde::Serialize;
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
// Start module functions are wrapped above and don't need re-export
pub use streaming::{StreamingConfig, create_handler_response as create_streaming_response, register_generator};
pub use testing::{
    PhpHttpTestClient, PhpSseEvent, PhpSseStream, PhpTestClient, PhpTestResponse, PhpWebSocketTestConnection,
};
pub use websocket::{PhpWebSocketHandler, create_websocket_state};

#[derive(Debug, Serialize)]
struct DtoFieldDefinition {
    name: &'static str,
    rust_type: &'static str,
    php_doc: &'static str,
    optional: bool,
    description: &'static str,
}

#[derive(Debug, Serialize)]
struct DtoDefinition {
    name: &'static str,
    kind: &'static str,
    fields: &'static [DtoFieldDefinition],
}

/// Background task wrapper to make wrap_function! work
#[php_function]
#[php(name = "spikard_background_run")]
pub fn spikard_background_run_wrapper(callable: &Zval, args: &Zval) -> PhpResult<()> {
    background::spikard_background_run(callable, args)
}

/// Start server wrapper for PHP.
#[php_function]
#[php(name = "spikard_start_server")]
pub fn spikard_start_server(
    routes_zval: &Zval,
    config: &Zval,
    hooks: &Zval,
    dependencies: Option<&Zval>,
) -> PhpResult<u64> {
    let default_deps = Zval::new();
    let deps = dependencies.unwrap_or(&default_deps);
    start::spikard_start_server_impl(routes_zval, config, hooks, deps)
}

/// Stop server wrapper for PHP.
#[php_function]
#[php(name = "spikard_stop_server")]
pub fn spikard_stop_server(_handle: u64) -> PhpResult<()> {
    start::spikard_stop_server_impl(_handle)
}

/// Register the PHP module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .name("spikard")
        .version(env!("CARGO_PKG_VERSION"))
        // Functions
        .function(wrap_function!(spikard_version))
        .function(wrap_function!(spikard_echo_response))
        .function(wrap_function!(spikard_json_response))
        .function(wrap_function!(spikard_parse_json))
        .function(wrap_function!(spikard_start_server))
        .function(wrap_function!(spikard_stop_server))
        .function(wrap_function!(spikard_background_run_wrapper))
        // Core classes
        .class::<PhpRequest>()
        .class::<PhpResponse>()
        // Server
        .class::<PhpServer>()
        // Testing
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

const REQUEST_FIELDS: &[DtoFieldDefinition] = &[
    DtoFieldDefinition {
        name: "method",
        rust_type: "String",
        php_doc: "string",
        optional: false,
        description: "HTTP method in uppercase form",
    },
    DtoFieldDefinition {
        name: "path",
        rust_type: "String",
        php_doc: "string",
        optional: false,
        description: "Route path with query stripped",
    },
    DtoFieldDefinition {
        name: "path_params",
        rust_type: "HashMap<String, String>",
        php_doc: "array<string, string>",
        optional: false,
        description: "Resolved path parameters",
    },
    DtoFieldDefinition {
        name: "query_params",
        rust_type: "serde_json::Value",
        php_doc: "mixed",
        optional: false,
        description: "Parsed query params preserving typed JSON",
    },
    DtoFieldDefinition {
        name: "raw_query_params",
        rust_type: "HashMap<String, Vec<String>>",
        php_doc: "array<string, array<int, string>>",
        optional: false,
        description: "Lossless multi-map query parameters",
    },
    DtoFieldDefinition {
        name: "body",
        rust_type: "serde_json::Value",
        php_doc: "mixed",
        optional: false,
        description: "Validated JSON body",
    },
    DtoFieldDefinition {
        name: "raw_body",
        rust_type: "Option<Vec<u8>>",
        php_doc: "string|null",
        optional: true,
        description: "Raw request body bytes when available",
    },
    DtoFieldDefinition {
        name: "headers",
        rust_type: "HashMap<String, String>",
        php_doc: "array<string, string>",
        optional: false,
        description: "Normalized header map (lowercase keys)",
    },
    DtoFieldDefinition {
        name: "cookies",
        rust_type: "HashMap<String, String>",
        php_doc: "array<string, string>",
        optional: false,
        description: "Incoming cookies",
    },
    DtoFieldDefinition {
        name: "files",
        rust_type: "HashMap<String, Value>",
        php_doc: "array<string, mixed>",
        optional: false,
        description: "Multipart form/file uploads",
    },
    DtoFieldDefinition {
        name: "dependencies",
        rust_type: "Option<ResolvedDependencies>",
        php_doc: "ResolvedDependencies|null",
        optional: true,
        description: "Dependency injection payload",
    },
];

const RESPONSE_FIELDS: &[DtoFieldDefinition] = &[
    DtoFieldDefinition {
        name: "status",
        rust_type: "u16",
        php_doc: "int",
        optional: false,
        description: "HTTP status code",
    },
    DtoFieldDefinition {
        name: "body",
        rust_type: "serde_json::Value",
        php_doc: "mixed",
        optional: true,
        description: "Response body as structured JSON",
    },
    DtoFieldDefinition {
        name: "headers",
        rust_type: "HashMap<String, String>",
        php_doc: "array<string, string>",
        optional: false,
        description: "Outgoing headers",
    },
    DtoFieldDefinition {
        name: "cookies",
        rust_type: "HashMap<String, String>",
        php_doc: "array<string, string>",
        optional: false,
        description: "Outgoing cookies",
    },
];

const REQUEST_DTO: DtoDefinition = DtoDefinition {
    name: "Request",
    kind: "request",
    fields: REQUEST_FIELDS,
};

const RESPONSE_DTO: DtoDefinition = DtoDefinition {
    name: "Response",
    kind: "response",
    fields: RESPONSE_FIELDS,
};

/// Expose RequestData field layout for PHP fallbacks (authoritative Rust shape).
#[php_function]
#[php(name = "spikard_request_dto_shape")]
pub fn spikard_request_dto_shape() -> PhpResult<ZBox<ZendHashTable>> {
    let shape = serde_json::json!({
        "method": "string",
        "path": "string",
        "path_params": "map<string,string>",
        "query_params": "json",
        "body": "json",
        "raw_body": "bytes|null",
        "raw_query_params": "map<string,array<string>>",
        "headers": "map<string,string>",
        "cookies": "map<string,string>",
        "files": "map<string,mixed>",
        "dependencies": "map<string,mixed>|null"
    });
    json_to_php_table(&shape)
}

/// Expose Response DTO layout for PHP fallbacks (authoritative Rust shape).
#[php_function]
#[php(name = "spikard_response_dto_shape")]
pub fn spikard_response_dto_shape() -> PhpResult<ZBox<ZendHashTable>> {
    let shape = serde_json::json!({
        "status": "int",
        "body": "json",
        "headers": "map<string,string>",
        "cookies": "map<string,string>"
    });
    json_to_php_table(&shape)
}

/// Emit full DTO metadata (field names, types, descriptions) for generators.
#[php_function]
#[php(name = "spikard_dto_definitions")]
pub fn spikard_dto_definitions() -> PhpResult<ZBox<ZendHashTable>> {
    let definitions = [REQUEST_DTO, RESPONSE_DTO];
    let json = serde_json::to_value(&definitions)
        .map_err(|e| PhpException::default(format!("Failed to encode DTO metadata: {e}")))?;
    json_to_php_table(&json)
}

/// Return default ServerConfig as a PHP array (authoritative Rust source).
#[php_function]
#[php(name = "spikard_config_defaults")]
pub fn spikard_config_defaults() -> PhpResult<ZBox<ZendHashTable>> {
    let defaults = spikard_http::ServerConfig::default();
    let json = serde_json::json!({
        "host": defaults.host,
        "port": defaults.port,
        "workers": defaults.workers,
        "enable_request_id": defaults.enable_request_id,
        "max_body_size": defaults.max_body_size,
        "request_timeout": defaults.request_timeout,
        "compression": defaults.compression,
        "rate_limit": defaults.rate_limit,
        "jwt_auth": defaults.jwt_auth,
        "api_key_auth": defaults.api_key_auth,
        "static_files": defaults.static_files,
        "graceful_shutdown": defaults.graceful_shutdown,
        "shutdown_timeout": defaults.shutdown_timeout,
        "openapi": defaults.openapi,
        "background_tasks": serde_json::Value::Null,
        "lifecycle_hooks": serde_json::Value::Null,
        "di_container": serde_json::Value::Null,
    });
    json_to_php_table(&json)
}

/// Normalize route metadata using Rust schema (method/path/handler_name/schemas).
#[php_function]
#[php(name = "spikard_normalize_routes")]
pub fn spikard_normalize_routes(routes: &Zval) -> PhpResult<ZBox<ZendHashTable>> {
    let arr = routes
        .array()
        .ok_or_else(|| PhpException::default("Routes must be an array".to_string()))?;

    let mut table = ZendHashTable::new();
    for (idx, route_val) in arr.iter().enumerate() {
        let route_arr = route_val
            .1
            .array()
            .ok_or_else(|| PhpException::default("Each route must be an array/object".to_string()))?;

        let method = route_arr
            .get("method")
            .and_then(|v| v.string())
            .map(|s| s.to_uppercase())
            .unwrap_or_else(|| "GET".to_string());
        let path = route_arr
            .get("path")
            .and_then(|v| v.string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "/".to_string());
        let handler_name = route_arr
            .get("handler_name")
            .and_then(|v| v.string())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "php_handler".to_string());

        let mut route_table = ZendHashTable::new();
        route_table.insert("method", method.as_str())?;
        route_table.insert("path", path.as_str())?;
        route_table.insert("handler_name", handler_name.as_str())?;

        if let Some(v) = route_arr.get("request_schema") {
            let value = crate::php::zval_to_json(v).map_err(|e| PhpException::default(e))?;
            route_table.insert("request_schema", json_to_php_table(&value)?)?;
        }
        if let Some(v) = route_arr.get("response_schema") {
            let value = crate::php::zval_to_json(v).map_err(|e| PhpException::default(e))?;
            route_table.insert("response_schema", json_to_php_table(&value)?)?;
        }
        if let Some(v) = route_arr.get("parameter_schema") {
            let value = crate::php::zval_to_json(v).map_err(|e| PhpException::default(e))?;
            route_table.insert("parameter_schema", json_to_php_table(&value)?)?;
        }
        if let Some(v) = route_arr.get("cors") {
            let value = crate::php::zval_to_json(v).map_err(|e| PhpException::default(e))?;
            route_table.insert("cors", json_to_php_table(&value)?)?;
        }
        if let Some(v) = route_arr.get("middleware") {
            let value = crate::php::zval_to_json(v).map_err(|e| PhpException::default(e))?;
            route_table.insert("middleware", json_to_php_table(&value)?)?;
        }

        route_table.insert(
            "websocket",
            route_arr.get("websocket").and_then(|v| v.bool()).unwrap_or(false),
        )?;
        route_table.insert("sse", route_arr.get("sse").and_then(|v| v.bool()).unwrap_or(false))?;

        table.insert(idx as i64, route_table)?;
    }

    Ok(table)
}
/// Convert a serde_json Value to a ZendHashTable.
pub fn json_to_php_table(value: &Value) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();

    match value {
        Value::Object(map) => {
            for (k, v) in map {
                match v {
                    Value::Null => table.insert(k.as_str(), ())?,
                    Value::Bool(b) => table.insert(k.as_str(), *b)?,
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            table.insert(k.as_str(), i)?;
                        } else if let Some(f) = n.as_f64() {
                            table.insert(k.as_str(), f)?;
                        }
                    }
                    Value::String(s) => table.insert(k.as_str(), s.as_str())?,
                    Value::Array(arr) => {
                        let inner = json_array_to_php(arr)?;
                        table.insert(k.as_str(), inner)?;
                    }
                    Value::Object(_) => {
                        let inner = json_to_php_table(v)?;
                        table.insert(k.as_str(), inner)?;
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

/// Convert a JSON array to a ZendHashTable.
fn json_array_to_php(arr: &[Value]) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();

    for v in arr {
        match v {
            Value::Null => table.push(())?,
            Value::Bool(b) => table.push(*b)?,
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    table.push(i)?;
                } else if let Some(f) = n.as_f64() {
                    table.push(f)?;
                }
            }
            Value::String(s) => table.push(s.as_str())?,
            Value::Array(inner_arr) => {
                let inner = json_array_to_php(inner_arr)?;
                table.push(inner)?;
            }
            Value::Object(_) => {
                let inner = json_to_php_table(v)?;
                table.push(inner)?;
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
            // Return as array
            let arr: Vec<Value> = map.into_iter().map(|(_, v)| v).collect();
            return Ok(Value::Array(arr));
        }

        return Ok(Value::Object(map));
    }

    Ok(Value::Null)
}

/// Best-effort string extraction from a Zval.
pub fn try_zval_string(value: &ext_php_rs::types::Zval) -> Option<String> {
    value
        .string()
        .map(|s| s.to_string())
        .or_else(|| value.long().map(|l| l.to_string()))
        .or_else(|| value.double().map(|d| d.to_string()))
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
