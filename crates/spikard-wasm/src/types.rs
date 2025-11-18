use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value, json};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize)]
pub struct RouteDefinition {
    pub method: String,
    pub path: String,
    pub handler_name: String,
    #[serde(default)]
    pub request_schema: Option<Value>,
    #[serde(default)]
    pub response_schema: Option<Value>,
    #[serde(default)]
    pub parameter_schema: Option<Value>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    #[serde(default)]
    pub compression: Option<CompressionConfig>,
    #[serde(default)]
    pub rate_limit: Option<RateLimitConfig>,
    #[serde(rename = "__wasmStaticManifest", default)]
    pub wasm_static_manifest: Vec<StaticManifestEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CompressionConfig {
    #[serde(default = "default_true")]
    pub gzip: bool,
    #[serde(default)]
    pub brotli: bool,
    #[serde(rename = "minSize", default)]
    pub min_size: usize,
    #[serde(default = "default_quality")]
    pub quality: u8,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig {
            gzip: true,
            brotli: true,
            min_size: 1024,
            quality: 6,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RateLimitConfig {
    #[serde(rename = "perSecond")]
    pub per_second: u64,
    pub burst: u64,
    #[serde(rename = "ipBased", default = "default_true")]
    pub ip_based: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StaticManifestEntry {
    pub route: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_base64_bytes")]
    pub body: Vec<u8>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct RequestOptions {
    pub headers: HashMap<String, String>,
    pub json: Option<Value>,
    pub form: Option<HashMap<String, Value>>,
    #[serde(rename = "formRaw")]
    pub form_raw: Option<String>,
    pub multipart: Option<MultipartOptions>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MultipartOptions {
    pub fields: HashMap<String, Value>,
    pub files: Vec<MultipartFile>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MultipartFile {
    pub name: String,
    pub filename: Option<String>,
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}

impl RequestOptions {
    pub fn from_js(value: JsValue) -> Result<Self, JsValue> {
        if value.is_null() || value.is_undefined() {
            return Ok(RequestOptions::default());
        }
        serde_wasm_bindgen::from_value(value).map_err(|err| JsValue::from_str(&err.to_string()))
    }

    pub fn body_payload(&self) -> Option<Value> {
        if let Some(multipart) = &self.multipart {
            let files = multipart
                .files
                .iter()
                .map(|file| {
                    json!({
                        "name": file.name,
                        "filename": file.filename,
                        "content": file.content,
                        "contentType": file.content_type,
                    })
                })
                .collect::<Vec<_>>();
            return Some(json!({
                "__spikard_multipart__": {
                    "fields": multipart.fields,
                    "files": files
                }
            }));
        }

        if let Some(form) = &self.form {
            return Some(json!({
                "__spikard_form__": form
            }));
        }

        if let Some(raw) = &self.form_raw
            && let Some(parsed) = parse_form_raw(raw)
        {
            return Some(parsed);
        }

        if self.json.is_some() {
            return self.json.clone();
        }

        None
    }
}

fn parse_form_raw(raw: &str) -> Option<Value> {
    serde_qs::from_str::<Value>(raw)
        .map(|value| json!({ "__spikard_form__": value }))
        .ok()
}

#[derive(Serialize)]
pub struct RequestPayload {
    method: String,
    path: String,
    #[serde(rename = "pathParams")]
    path_params: HashMap<String, String>,
    query: HashMap<String, Value>,
    #[serde(rename = "rawQuery")]
    raw_query: HashMap<String, Vec<String>>,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    params: HashMap<String, Value>,
    body: Option<Value>,
}

impl RequestPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        method: String,
        path: &str,
        path_params: HashMap<String, String>,
        headers: &HashMap<String, String>,
        query: crate::matching::QueryParams,
        params: HashMap<String, Value>,
        body: Option<Value>,
    ) -> Self {
        RequestPayload {
            method,
            path: path.to_string(),
            path_params,
            query: query.normalized,
            raw_query: query.raw,
            headers: headers.clone(),
            cookies: HashMap::new(),
            params,
            body,
        }
    }
}

pub fn build_params(
    path: &HashMap<String, String>,
    query: &HashMap<String, Value>,
    headers: &HashMap<String, String>,
) -> HashMap<String, Value> {
    let mut params = HashMap::new();
    for (key, value) in path {
        params.insert(key.clone(), Value::String(value.clone()));
    }
    for (key, value) in query {
        params.insert(key.clone(), value.clone());
    }
    for (key, value) in headers {
        params.insert(key.clone(), Value::String(value.clone()));
    }
    params
}

#[derive(Default)]
pub struct HandlerResponsePayload {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body_bytes: Vec<u8>,
}

impl HandlerResponsePayload {
    pub fn from_value(value: Value) -> Result<Self, JsValue> {
        if let Value::Object(mut map) = value {
            let status = extract_status(&mut map);
            let headers = extract_headers(&mut map);
            let body = extract_body(&mut map);
            let body_bytes = encode_body(body);
            Ok(HandlerResponsePayload {
                status,
                headers,
                body_bytes,
            })
        } else {
            let body_bytes = encode_body(Some(value));
            Ok(HandlerResponsePayload {
                status: 200,
                headers: HashMap::new(),
                body_bytes,
            })
        }
    }
}

fn extract_status(map: &mut JsonMap<String, Value>) -> u16 {
    map.remove("status")
        .or_else(|| map.remove("statusCode"))
        .and_then(|value| value.as_u64())
        .map(|num| num as u16)
        .unwrap_or(200)
}

fn extract_headers(map: &mut JsonMap<String, Value>) -> HashMap<String, String> {
    map.remove("headers")
        .and_then(|value| value.as_object().cloned())
        .map(|object| {
            object
                .into_iter()
                .filter_map(|(key, value)| match value {
                    Value::String(s) => Some((key, s)),
                    Value::Number(num) => Some((key, num.to_string())),
                    Value::Bool(boolean) => Some((key, boolean.to_string())),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn extract_body(map: &mut JsonMap<String, Value>) -> Option<Value> {
    if let Some(value) = map.remove("body") {
        match value {
            Value::Null => None,
            other => Some(other),
        }
    } else if map.is_empty() {
        None
    } else {
        Some(Value::Object(std::mem::take(map)))
    }
}

fn encode_body(body: Option<Value>) -> Vec<u8> {
    match body {
        None => Vec::new(),
        Some(Value::Object(mut object)) => {
            if let Some(Value::String(encoded)) = object.remove("__spikard_base64__") {
                return BASE64_STANDARD.decode(encoded).unwrap_or_else(|_| Vec::new());
            }
            serde_json::to_vec(&Value::Object(object)).unwrap_or_else(|_| Vec::new())
        }
        Some(Value::String(s)) => s.into_bytes(),
        Some(other) => serde_json::to_vec(&other).unwrap_or_else(|_| Vec::new()),
    }
}

const fn default_true() -> bool {
    true
}

const fn default_quality() -> u8 {
    6
}

fn deserialize_base64_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    BASE64_STANDARD
        .decode(encoded.as_bytes())
        .map_err(|err| de::Error::custom(format!("Invalid base64 payload: {err}")))
}
