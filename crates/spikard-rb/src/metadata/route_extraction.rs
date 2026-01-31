//! Route metadata extraction and building from Ruby objects.
//!
//! This module handles converting Ruby route definitions into structured
//! RouteMetadata that can be used by the Rust HTTP server.

use magnus::prelude::*;
use magnus::{Error, RArray, RHash, RString, Ruby, TryConvert, Value, r_hash::ForEach};
use serde_json::{Map as JsonMap, Value as JsonValue};
use spikard_http::{Route, RouteMetadata, SchemaRegistry};

/// Build route metadata from Ruby parameters
#[allow(clippy::too_many_arguments)]
pub fn build_route_metadata(
    ruby: &Ruby,
    method: String,
    path: String,
    handler_name: Option<String>,
    request_schema_value: Value,
    response_schema_value: Value,
    parameter_schema_value: Value,
    file_params_value: Value,
    is_async: bool,
    cors_value: Value,
    body_param_name: Option<String>,
    jsonrpc_method_value: Value,
    handler_value: Value,
) -> Result<Value, Error> {
    let normalized_path = normalize_path_for_route(&path);
    let final_handler_name = handler_name.unwrap_or_else(|| default_handler_name(&method, &normalized_path));

    let json_module = ruby
        .class_object()
        .const_get("JSON")
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

    let mut request_schema = if request_schema_value.is_nil() {
        None
    } else {
        Some(ruby_value_to_json(ruby, json_module, request_schema_value)?)
    };
    let response_schema = if response_schema_value.is_nil() {
        None
    } else {
        Some(ruby_value_to_json(ruby, json_module, response_schema_value)?)
    };
    let mut parameter_schema = if parameter_schema_value.is_nil() {
        None
    } else {
        Some(ruby_value_to_json(ruby, json_module, parameter_schema_value)?)
    };
    let file_params = if file_params_value.is_nil() {
        None
    } else {
        Some(ruby_value_to_json(ruby, json_module, file_params_value)?)
    };

    if parameter_schema.is_none()
        && let Some(derived) = derive_parameter_schema_from_request(&mut request_schema)
    {
        parameter_schema = Some(derived);
    }

    let cors = parse_cors_config(ruby, cors_value)?;
    let handler_dependencies = extract_handler_dependencies_from_ruby(ruby, handler_value)?;

    #[cfg(feature = "di")]
    let handler_deps_option = if handler_dependencies.is_empty() {
        None
    } else {
        Some(handler_dependencies.clone())
    };

    let jsonrpc_method = if jsonrpc_method_value.is_nil() {
        None
    } else {
        Some(ruby_value_to_json(ruby, json_module, jsonrpc_method_value)?)
    };

    #[cfg(feature = "di")]
    let mut metadata = RouteMetadata {
        method,
        path: normalized_path,
        handler_name: final_handler_name,
        request_schema,
        response_schema,
        parameter_schema,
        file_params,
        is_async,
        cors,
        body_param_name,
        handler_dependencies: handler_deps_option,
        jsonrpc_method,
        static_response: None,
    };

    #[cfg(not(feature = "di"))]
    let mut metadata = RouteMetadata {
        method,
        path: normalized_path,
        handler_name: final_handler_name,
        request_schema,
        response_schema,
        parameter_schema,
        file_params,
        is_async,
        cors,
        body_param_name,
        jsonrpc_method,
        static_response: None,
    };

    let registry = SchemaRegistry::new();
    let route = Route::from_metadata(metadata.clone(), &registry).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to build route metadata: {err}"),
        )
    })?;

    if let Some(validator) = route.parameter_validator.as_ref() {
        metadata.parameter_schema = Some(validator.schema().clone());
    }

    route_metadata_to_ruby(ruby, &metadata)
}

fn derive_parameter_schema_from_request(request_schema: &mut Option<JsonValue>) -> Option<JsonValue> {
    let schema = request_schema.as_ref()?;
    let schema_obj = schema.as_object()?;
    let properties = schema_obj.get("properties")?.as_object()?;

    let mut param_properties = JsonMap::new();
    let mut required = Vec::new();
    let mut has_params = false;

    let sources = [
        ("path", "path"),
        ("query", "query"),
        ("headers", "header"),
        ("cookies", "cookie"),
    ];

    for (section_key, source) in sources {
        let Some(section_schema) = properties.get(section_key) else {
            continue;
        };
        let Some(section_props) = section_schema.get("properties").and_then(|value| value.as_object()) else {
            continue;
        };

        has_params = true;
        for (name, schema_value) in section_props {
            let mut schema_obj = if let Some(obj) = schema_value.as_object() {
                obj.clone()
            } else {
                let mut wrapped = JsonMap::new();
                wrapped.insert("const".to_string(), schema_value.clone());
                wrapped
            };
            schema_obj.insert("source".to_string(), JsonValue::String(source.to_string()));
            param_properties.insert(name.clone(), JsonValue::Object(schema_obj));
        }

        if let Some(required_list) = section_schema.get("required").and_then(|value| value.as_array()) {
            for item in required_list {
                if let Some(name) = item.as_str() {
                    required.push(name.to_string());
                }
            }
        }
    }

    if !has_params {
        return None;
    }

    let mut derived = JsonMap::new();
    derived.insert("type".to_string(), JsonValue::String("object".to_string()));
    derived.insert("properties".to_string(), JsonValue::Object(param_properties));

    if !required.is_empty() {
        required.sort();
        required.dedup();
        derived.insert(
            "required".to_string(),
            JsonValue::Array(required.into_iter().map(JsonValue::String).collect()),
        );
    }

    if let Some(body_schema) = properties.get("body") {
        *request_schema = Some(body_schema.clone());
    } else {
        *request_schema = None;
    }

    Some(JsonValue::Object(derived))
}

/// Convert a RouteMetadata to a Ruby hash
pub fn route_metadata_to_ruby(ruby: &Ruby, metadata: &RouteMetadata) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(ruby.to_symbol("method"), ruby.str_new(&metadata.method))?;
    hash.aset(ruby.to_symbol("path"), ruby.str_new(&metadata.path))?;
    hash.aset(ruby.to_symbol("handler_name"), ruby.str_new(&metadata.handler_name))?;
    let is_async_val: Value = if metadata.is_async {
        ruby.qtrue().as_value()
    } else {
        ruby.qfalse().as_value()
    };
    hash.aset(ruby.to_symbol("is_async"), is_async_val)?;

    hash.aset(
        ruby.to_symbol("request_schema"),
        option_json_to_ruby(ruby, metadata.request_schema.as_ref())?,
    )?;
    hash.aset(
        ruby.to_symbol("response_schema"),
        option_json_to_ruby(ruby, metadata.response_schema.as_ref())?,
    )?;
    hash.aset(
        ruby.to_symbol("parameter_schema"),
        option_json_to_ruby(ruby, metadata.parameter_schema.as_ref())?,
    )?;
    hash.aset(
        ruby.to_symbol("file_params"),
        option_json_to_ruby(ruby, metadata.file_params.as_ref())?,
    )?;
    hash.aset(
        ruby.to_symbol("body_param_name"),
        metadata
            .body_param_name
            .as_ref()
            .map(|s| ruby.str_new(s).as_value())
            .unwrap_or_else(|| ruby.qnil().as_value()),
    )?;

    hash.aset(ruby.to_symbol("cors"), cors_to_ruby(ruby, metadata.cors.as_ref())?)?;

    #[cfg(feature = "di")]
    {
        if let Some(deps) = &metadata.handler_dependencies {
            let array = ruby.ary_new();
            for dep in deps {
                array.push(ruby.str_new(dep))?;
            }
            hash.aset(ruby.to_symbol("handler_dependencies"), array)?;
        } else {
            hash.aset(ruby.to_symbol("handler_dependencies"), ruby.qnil())?;
        }
    }

    hash.aset(
        ruby.to_symbol("jsonrpc_method"),
        option_json_to_ruby(ruby, metadata.jsonrpc_method.as_ref())?,
    )?;

    Ok(hash.as_value())
}

/// Normalize path for routes (convert :param to {param})
pub fn normalize_path_for_route(path: &str) -> String {
    let has_trailing_slash = path.ends_with('/');
    let segments = path.split('/').map(|segment| {
        if let Some(stripped) = segment.strip_prefix(':') {
            format!("{{{}}}", stripped)
        } else {
            segment.to_string()
        }
    });

    let normalized = segments.collect::<Vec<_>>().join("/");
    if has_trailing_slash && !normalized.ends_with('/') {
        format!("{normalized}/")
    } else {
        normalized
    }
}

/// Generate default handler name from method and path
pub fn default_handler_name(method: &str, path: &str) -> String {
    let normalized_path: String = path
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect();
    let trimmed = normalized_path.trim_matches('_');
    let final_segment = if trimmed.is_empty() { "root" } else { trimmed };
    format!("{}_{}", method.to_ascii_lowercase(), final_segment)
}

/// Extract handler dependencies from a Ruby handler callable
pub fn extract_handler_dependencies_from_ruby(_ruby: &Ruby, handler_value: Value) -> Result<Vec<String>, Error> {
    if handler_value.is_nil() {
        return Ok(Vec::new());
    }

    let params_value: Value = handler_value.funcall("parameters", ())?;
    let params = RArray::try_convert(params_value)?;

    let mut dependencies = Vec::new();
    for i in 0..params.len() {
        let entry: Value = params.entry(i as isize)?;
        if let Some(pair) = RArray::from_value(entry) {
            if pair.len() < 2 {
                continue;
            }

            let kind_val: Value = pair.entry(0)?;
            let name_val: Value = pair.entry(1)?;

            let kind_symbol: magnus::Symbol = magnus::Symbol::try_convert(kind_val)?;
            let kind_name = kind_symbol.name().unwrap_or_default();

            if kind_name == "key" || kind_name == "keyreq" {
                if let Ok(sym) = magnus::Symbol::try_convert(name_val) {
                    if let Ok(name) = sym.name() {
                        dependencies.push(name.to_string());
                    }
                } else {
                    dependencies.push(String::try_convert(name_val)?);
                }
            }
        }
    }

    Ok(dependencies)
}

/// Parse CORS configuration from Ruby value
pub fn parse_cors_config(ruby: &Ruby, value: Value) -> Result<Option<spikard_http::CorsConfig>, Error> {
    if value.is_nil() {
        return Ok(None);
    }

    let hash = RHash::try_convert(value)?;
    let lookup = |key: &str| -> Option<Value> { hash.get(ruby.to_symbol(key)).or_else(|| hash.get(ruby.str_new(key))) };

    let allowed_origins = lookup("allowed_origins")
        .and_then(|v| Vec::<String>::try_convert(v).ok())
        .unwrap_or_default();
    let allowed_methods = lookup("allowed_methods")
        .and_then(|v| Vec::<String>::try_convert(v).ok())
        .unwrap_or_default();
    let allowed_headers = lookup("allowed_headers")
        .and_then(|v| Vec::<String>::try_convert(v).ok())
        .unwrap_or_default();
    let expose_headers = lookup("expose_headers").and_then(|v| Vec::<String>::try_convert(v).ok());
    let max_age = lookup("max_age")
        .and_then(|v| i64::try_convert(v).ok())
        .map(|v| v as u32);
    let allow_credentials = lookup("allow_credentials").and_then(|v| bool::try_convert(v).ok());

    Ok(Some(spikard_http::CorsConfig {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        expose_headers,
        max_age,
        allow_credentials,
        ..Default::default()
    }))
}

/// Convert an optional JSON value to Ruby
pub fn option_json_to_ruby(ruby: &Ruby, value: Option<&JsonValue>) -> Result<Value, Error> {
    if let Some(json) = value {
        json_to_ruby(ruby, json)
    } else {
        Ok(ruby.qnil().as_value())
    }
}

/// Convert CORS config to Ruby hash
pub fn cors_to_ruby(ruby: &Ruby, cors: Option<&spikard_http::CorsConfig>) -> Result<Value, Error> {
    if let Some(cors_config) = cors {
        let hash = ruby.hash_new();
        let origins = cors_config
            .allowed_origins
            .iter()
            .map(|s| JsonValue::String(s.clone()))
            .collect();
        hash.aset(
            ruby.to_symbol("allowed_origins"),
            json_to_ruby(ruby, &JsonValue::Array(origins))?,
        )?;
        let methods = cors_config
            .allowed_methods
            .iter()
            .map(|s| JsonValue::String(s.clone()))
            .collect();
        hash.aset(
            ruby.to_symbol("allowed_methods"),
            json_to_ruby(ruby, &JsonValue::Array(methods))?,
        )?;
        let headers = cors_config
            .allowed_headers
            .iter()
            .map(|s| JsonValue::String(s.clone()))
            .collect();
        hash.aset(
            ruby.to_symbol("allowed_headers"),
            json_to_ruby(ruby, &JsonValue::Array(headers))?,
        )?;
        hash.aset(
            ruby.to_symbol("expose_headers"),
            if let Some(expose_headers) = &cors_config.expose_headers {
                let expose = expose_headers.iter().map(|s| JsonValue::String(s.clone())).collect();
                json_to_ruby(ruby, &JsonValue::Array(expose))?
            } else {
                ruby.qnil().as_value()
            },
        )?;
        hash.aset(
            ruby.to_symbol("max_age"),
            if let Some(max_age) = cors_config.max_age {
                ruby.integer_from_i64(max_age as i64).as_value()
            } else {
                ruby.qnil().as_value()
            },
        )?;
        hash.aset(
            ruby.to_symbol("allow_credentials"),
            if let Some(allow_creds) = cors_config.allow_credentials {
                if allow_creds {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                }
            } else {
                ruby.qnil().as_value()
            },
        )?;
        Ok(hash.as_value())
    } else {
        Ok(ruby.qnil().as_value())
    }
}

/// Convert Ruby value to JSON
pub fn ruby_value_to_json(ruby: &Ruby, _json_module: Value, value: Value) -> Result<JsonValue, Error> {
    if value.is_nil() {
        return Ok(JsonValue::Null);
    }

    if value.is_kind_of(ruby.class_true_class()) {
        return Ok(JsonValue::Bool(true));
    }

    if value.is_kind_of(ruby.class_false_class()) {
        return Ok(JsonValue::Bool(false));
    }

    if value.is_kind_of(ruby.class_float()) {
        let float_val = f64::try_convert(value)?;
        if let Some(num) = serde_json::Number::from_f64(float_val) {
            return Ok(JsonValue::Number(num));
        }
    }

    if value.is_kind_of(ruby.class_integer()) {
        if let Ok(int_val) = i64::try_convert(value) {
            return Ok(JsonValue::Number(int_val.into()));
        }
        if let Ok(int_val) = u64::try_convert(value) {
            return Ok(JsonValue::Number(int_val.into()));
        }
    }

    if let Ok(str_val) = RString::try_convert(value) {
        let slice = str_val.to_string()?;
        return Ok(JsonValue::String(slice));
    }

    if let Some(array) = RArray::from_value(value) {
        let mut items = Vec::with_capacity(array.len());
        let slice = unsafe { array.as_slice() };
        for elem in slice {
            items.push(ruby_value_to_json(ruby, _json_module, *elem)?);
        }
        return Ok(JsonValue::Array(items));
    }

    if let Some(hash) = RHash::from_value(value) {
        let mut map = JsonMap::new();
        hash.foreach(|key: Value, val: Value| -> Result<ForEach, Error> {
            let key_str: String = if let Ok(sym) = magnus::Symbol::try_convert(key) {
                sym.name().map(|c| c.to_string()).unwrap_or_default()
            } else {
                String::try_convert(key)?
            };
            let json_val = ruby_value_to_json(ruby, _json_module, val)?;
            map.insert(key_str, json_val);
            Ok(ForEach::Continue)
        })?;
        return Ok(JsonValue::Object(map));
    }

    Err(Error::new(
        ruby.exception_arg_error(),
        "Unsupported Ruby value type for JSON conversion",
    ))
}

/// Convert JSON to Ruby value
pub fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    crate::conversion::json_to_ruby(ruby, value)
}
