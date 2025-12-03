//! Ruby ↔ Rust type conversion utilities.
//!
//! This module provides functions for converting between Ruby and Rust types,
//! including JSON conversion, string conversion, and request/response building.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use bytes::Bytes;
use magnus::prelude::*;
use magnus::{Error, RArray, RHash, RString, Ruby, TryConvert, Value};
use serde_json::Value as JsonValue;
use spikard_core::problem::ProblemDetails;
use spikard_http::testing::MultipartFilePart;
use std::collections::HashMap;

use crate::test_client::{RequestBody, RequestConfig, TestResponseData};

/// Convert a Ruby value to JSON.
///
/// Uses Ruby's JSON.generate method to serialize the Ruby object
/// and then parses the result.
pub fn ruby_value_to_json(ruby: &Ruby, json_module: Value, value: Value) -> Result<JsonValue, Error> {
    if value.is_nil() {
        return Ok(JsonValue::Null);
    }

    let json_string: String = json_module.funcall("generate", (value,))?;
    serde_json::from_str(&json_string).map_err(|err| {
        Error::new(
            ruby.exception_runtime_error(),
            format!("Failed to convert Ruby value to JSON: {err}"),
        )
    })
}

/// Convert JSON to a Ruby value.
///
/// Recursively converts JSON types to native Ruby types:
/// - null → nil
/// - bool → true/false
/// - number → integer or float
/// - string → string
/// - array → array
/// - object → hash
pub fn json_to_ruby(ruby: &Ruby, value: &JsonValue) -> Result<Value, Error> {
    json_to_ruby_with_uploads(ruby, value, None::<&Value>)
}

/// Convert JSON to a Ruby value, optionally materialising UploadFile objects.
///
/// If `upload_file_class` is provided and the JSON object contains
/// file-metadata keys (`filename`, `content`), this will instantiate
/// `UploadFile` instead of returning a plain Hash.
pub fn json_to_ruby_with_uploads(
    ruby: &Ruby,
    value: &JsonValue,
    upload_file_class: Option<&Value>,
) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(ruby.qnil().as_value()),
        JsonValue::Bool(b) => Ok(if *b {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }),
        JsonValue::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(ruby.integer_from_i64(i).as_value())
            } else if let Some(f) = num.as_f64() {
                Ok(ruby.float_from_f64(f).as_value())
            } else {
                Ok(ruby.qnil().as_value())
            }
        }
        JsonValue::String(str_val) => Ok(ruby.str_new(str_val).as_value()),
        JsonValue::Array(items) => {
            let array = ruby.ary_new();
            for item in items {
                array.push(json_to_ruby_with_uploads(ruby, item, upload_file_class)?)?;
            }
            Ok(array.as_value())
        }
        JsonValue::Object(map) => {
            if let Some(upload_file) = upload_file_class
                && let Some(upload) = try_build_upload_file(ruby, upload_file, map)?
            {
                return Ok(upload);
            }

            let hash = ruby.hash_new();
            for (key, item) in map {
                hash.aset(
                    ruby.str_new(key),
                    json_to_ruby_with_uploads(ruby, item, upload_file_class)?,
                )?;
            }
            Ok(hash.as_value())
        }
    }
}

/// Convert a HashMap to a Ruby Hash.
pub fn map_to_ruby_hash(ruby: &Ruby, map: &HashMap<String, String>) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    for (key, value) in map {
        hash.aset(ruby.str_new(key), ruby.str_new(value))?;
    }
    Ok(hash.as_value())
}

/// Convert a HashMap of Vecs to a Ruby Hash with array values.
pub fn multimap_to_ruby_hash(ruby: &Ruby, map: &HashMap<String, Vec<String>>) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    for (key, values) in map {
        let array = ruby.ary_new();
        for value in values {
            array.push(ruby.str_new(value))?;
        }
        hash.aset(ruby.str_new(key), array)?;
    }
    Ok(hash.as_value())
}

fn try_build_upload_file(
    ruby: &Ruby,
    upload_file_class: &Value,
    map: &serde_json::Map<String, JsonValue>,
) -> Result<Option<Value>, Error> {
    let filename = match map.get("filename").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return Ok(None),
    };
    let content = match map.get("content") {
        Some(JsonValue::String(s)) => s.as_str(),
        _ => return Ok(None),
    };

    let content_type = map.get("content_type").and_then(|v| v.as_str());
    let size = map.get("size").and_then(|v| v.as_u64());
    let headers_value = map
        .get("headers")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();
    let headers = map_to_ruby_hash(ruby, &headers_value)?;
    let content_encoding = map.get("content_encoding").and_then(|v| v.as_str());

    let kwargs = magnus::kwargs!(
        "content_type" => content_type,
        "size" => size,
        "headers" => headers,
        "content_encoding" => content_encoding
    );

    let upload = upload_file_class.funcall("new", (filename, content, kwargs))?;
    Ok(Some(upload))
}

/// Convert a Ruby value to Bytes.
///
/// Accepts either String or Array of bytes.
pub fn ruby_value_to_bytes(value: Value) -> Result<Bytes, std::io::Error> {
    if let Ok(str_value) = RString::try_convert(value) {
        // SAFETY: Magnus guarantees RString::as_slice() returns valid UTF-8 (or binary)
        // bytes for the lifetime of the RString. The slice is only used within this
        // function scope to copy into a Bytes buffer, and does not outlive the RString
        // reference. The copy_from_slice operation is safe for the borrowed data.
        let slice = unsafe { str_value.as_slice() };
        return Ok(Bytes::copy_from_slice(slice));
    }

    if let Ok(vec_bytes) = Vec::<u8>::try_convert(value) {
        return Ok(Bytes::from(vec_bytes));
    }

    Err(std::io::Error::other(
        "Streaming chunks must be Strings or Arrays of bytes",
    ))
}

/// Convert a response to a Ruby Hash.
pub fn response_to_ruby(ruby: &Ruby, response: TestResponseData) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(
        ruby.intern("status_code"),
        ruby.integer_from_i64(response.status as i64),
    )?;

    let headers_hash = ruby.hash_new();
    for (key, value) in response.headers {
        headers_hash.aset(ruby.str_new(&key), ruby.str_new(&value))?;
    }
    hash.aset(ruby.intern("headers"), headers_hash)?;

    if let Some(body) = response.body_text {
        let body_value = ruby.str_new(&body);
        hash.aset(ruby.intern("body"), body_value)?;
        hash.aset(ruby.intern("body_text"), body_value)?;
    } else {
        hash.aset(ruby.intern("body"), ruby.qnil())?;
        hash.aset(ruby.intern("body_text"), ruby.qnil())?;
    }

    Ok(hash.as_value())
}

/// Convert a ProblemDetails to a JSON string.
pub fn problem_to_json(problem: &ProblemDetails) -> String {
    problem
        .to_json_pretty()
        .unwrap_or_else(|err| format!("Failed to serialise problem details: {err}"))
}

/// Fetch a handler from a Ruby Hash by name.
///
/// Tries both symbol and string keys.
pub fn fetch_handler(ruby: &Ruby, handlers: &RHash, name: &str) -> Result<Value, Error> {
    let symbol_key = ruby.intern(name);
    if let Some(value) = handlers.get(symbol_key) {
        return Ok(value);
    }

    let string_key = ruby.str_new(name);
    if let Some(value) = handlers.get(string_key) {
        return Ok(value);
    }

    Err(Error::new(
        ruby.exception_name_error(),
        format!("Handler '{name}' not provided"),
    ))
}

/// Get an optional keyword argument from a Ruby Hash.
pub fn get_kw(ruby: &Ruby, hash: RHash, name: &str) -> Option<Value> {
    let sym = ruby.intern(name);
    hash.get(sym).or_else(|| hash.get(name))
}

/// Parse request configuration from a Ruby options Hash.
///
/// Supports: query, headers, cookies, json, data, raw_body, files
pub fn parse_request_config(ruby: &Ruby, options: Value) -> Result<RequestConfig, Error> {
    if options.is_nil() {
        return Ok(RequestConfig {
            query: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: None,
        });
    }

    let hash = RHash::from_value(options)
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "request options must be a Hash"))?;

    let json_module = ruby
        .class_object()
        .const_get("JSON")
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "JSON module not available"))?;

    let query = if let Some(value) = get_kw(ruby, hash, "query") {
        if value.is_nil() {
            None
        } else {
            Some(ruby_value_to_json(ruby, json_module, value)?)
        }
    } else {
        None
    };

    let headers = if let Some(value) = get_kw(ruby, hash, "headers") {
        if value.is_nil() {
            HashMap::new()
        } else {
            let hash = RHash::try_convert(value)?;
            hash.to_hash_map::<String, String>()?
        }
    } else {
        HashMap::new()
    };

    let cookies = if let Some(value) = get_kw(ruby, hash, "cookies") {
        if value.is_nil() {
            HashMap::new()
        } else {
            let hash = RHash::try_convert(value)?;
            hash.to_hash_map::<String, String>()?
        }
    } else {
        HashMap::new()
    };

    let files_opt = get_kw(ruby, hash, "files");
    let has_files = files_opt.as_ref().is_some_and(|f| !f.is_nil());

    let body = if has_files {
        let files_value = files_opt.ok_or_else(|| {
            Error::new(
                ruby.exception_runtime_error(),
                "Files option should be Some if has_files is true",
            )
        })?;
        let files = extract_files(ruby, files_value)?;

        let mut form_data = Vec::new();
        if let Some(data_value) = get_kw(ruby, hash, "data")
            && !data_value.is_nil()
        {
            let data_hash = RHash::try_convert(data_value)?;

            let keys_array: RArray = data_hash.funcall("keys", ())?;

            for i in 0..keys_array.len() {
                let key_val = keys_array.entry::<Value>(i as isize)?;
                let field_name = String::try_convert(key_val)?;
                let value = data_hash
                    .get(key_val)
                    .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

                if let Some(array) = RArray::from_value(value) {
                    for j in 0..array.len() {
                        let item = array.entry::<Value>(j as isize)?;
                        let item_str = String::try_convert(item)?;
                        form_data.push((field_name.clone(), item_str));
                    }
                } else {
                    let value_str = String::try_convert(value)?;
                    form_data.push((field_name, value_str));
                }
            }
        }

        Some(RequestBody::Multipart { form_data, files })
    } else if let Some(value) = get_kw(ruby, hash, "json") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Json(ruby_value_to_json(ruby, json_module, value)?))
        }
    } else if let Some(value) = get_kw(ruby, hash, "data") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Form(ruby_value_to_json(ruby, json_module, value)?))
        }
    } else if let Some(value) = get_kw(ruby, hash, "raw_body") {
        if value.is_nil() {
            None
        } else {
            Some(RequestBody::Raw(String::try_convert(value)?))
        }
    } else {
        None
    };

    Ok(RequestConfig {
        query,
        headers,
        cookies,
        body,
    })
}

/// Extract files from a Ruby Hash.
///
/// Files can be provided as [filename, content] or [filename, content, content_type]
pub fn extract_files(ruby: &Ruby, files_value: Value) -> Result<Vec<MultipartFilePart>, Error> {
    let files_hash = RHash::try_convert(files_value)?;

    let keys_array: RArray = files_hash.funcall("keys", ())?;
    let mut result = Vec::new();

    for i in 0..keys_array.len() {
        let key_val = keys_array.entry::<Value>(i as isize)?;
        let field_name = String::try_convert(key_val)?;
        let value = files_hash
            .get(key_val)
            .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "Failed to get hash value"))?;

        if let Some(outer_array) = RArray::from_value(value) {
            if outer_array.is_empty() {
                continue;
            }

            let first_elem = outer_array.entry::<Value>(0)?;

            if RArray::from_value(first_elem).is_some() {
                for j in 0..outer_array.len() {
                    let file_array = outer_array.entry::<Value>(j as isize)?;
                    let file_data = extract_single_file(ruby, &field_name, file_array)?;
                    result.push(file_data);
                }
            } else {
                let file_data = extract_single_file(ruby, &field_name, value)?;
                result.push(file_data);
            }
        }
    }

    Ok(result)
}

/// Extract a single file from a Ruby array [filename, content, content_type (optional)].
pub fn extract_single_file(ruby: &Ruby, field_name: &str, array_value: Value) -> Result<MultipartFilePart, Error> {
    let array = RArray::from_value(array_value)
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), "file must be an Array [filename, content]"))?;

    if array.len() < 2 {
        return Err(Error::new(
            ruby.exception_arg_error(),
            "file Array must have at least 2 elements: [filename, content]",
        ));
    }

    let filename: String = String::try_convert(array.shift()?)?;
    let content_str: String = String::try_convert(array.shift()?)?;
    let content = content_str.into_bytes();

    let content_type: Option<String> = if !array.is_empty() {
        String::try_convert(array.shift()?).ok()
    } else {
        None
    };

    Ok(MultipartFilePart {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}

/// Extract an optional string from a Ruby Hash.
pub fn get_optional_string_from_hash(hash: RHash, key: &str) -> Result<Option<String>, Error> {
    match hash.get(String::from(key)) {
        Some(v) if !v.is_nil() => Ok(Some(String::try_convert(v)?)),
        _ => Ok(None),
    }
}

/// Extract a required string from a Ruby Hash.
pub fn get_required_string_from_hash(hash: RHash, key: &str, ruby: &Ruby) -> Result<String, Error> {
    let value = hash
        .get(String::from(key))
        .ok_or_else(|| Error::new(ruby.exception_arg_error(), format!("missing required key '{}'", key)))?;
    if value.is_nil() {
        return Err(Error::new(
            ruby.exception_arg_error(),
            format!("key '{}' cannot be nil", key),
        ));
    }
    String::try_convert(value)
}
