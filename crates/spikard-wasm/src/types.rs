use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value, json};
use std::collections::HashMap;
use url::form_urlencoded;
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
    #[serde(default)]
    pub jsonrpc_method: Option<Value>,
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BodyKind {
    None,
    Json,
    Form,
    Multipart,
    Binary,
    Text,
}

impl Default for BodyKind {
    fn default() -> Self {
        BodyKind::None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BodyMetadata {
    #[serde(default)]
    pub kind: BodyKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<MultipartFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_base64: Option<String>,
}

impl BodyMetadata {
    pub fn from_body_value(body: Option<&Value>) -> Self {
        match body {
            None => BodyMetadata {
                kind: BodyKind::None,
                ..Default::default()
            },
            Some(Value::String(text)) => BodyMetadata {
                kind: BodyKind::Text,
                text: Some(text.clone()),
                raw_base64: Some(BASE64_STANDARD.encode(text.as_bytes())),
                ..Default::default()
            },
            Some(value @ Value::Number(_)) | Some(value @ Value::Bool(_)) => {
                let mut metadata = BodyMetadata {
                    kind: BodyKind::Json,
                    json: Some((*value).clone()),
                    ..Default::default()
                };
                metadata.raw_base64 = Some(encode_json_base64(value));
                metadata
            }
            Some(Value::Object(map)) => {
                if let Some(Value::String(encoded)) = map.get("__spikard_base64__") {
                    BodyMetadata {
                        kind: BodyKind::Binary,
                        raw_base64: Some(encoded.clone()),
                        ..Default::default()
                    }
                } else if let Some(Value::Object(form_fields)) = map.get("__spikard_form__") {
                    let form = normalize_form_values(form_fields);
                    BodyMetadata {
                        kind: BodyKind::Form,
                        form: Some(form.clone()),
                        raw_base64: Some(encode_form_bytes(&form)),
                        ..Default::default()
                    }
                } else if let Some(Value::Object(multipart)) = map.get("__spikard_multipart__") {
                    let (form, files) = normalize_multipart_payload(multipart);
                    BodyMetadata {
                        kind: BodyKind::Multipart,
                        form: Some(form.clone()),
                        files: Some(files),
                        raw_base64: Some(encode_form_bytes(&form)),
                        ..Default::default()
                    }
                } else {
                    let json_value = Value::Object(map.clone());
                    let mut metadata = BodyMetadata {
                        kind: BodyKind::Json,
                        json: Some(json_value.clone()),
                        ..Default::default()
                    };
                    metadata.raw_base64 = Some(encode_json_base64(&json_value));
                    metadata
                }
            }
            Some(other) => {
                let mut metadata = BodyMetadata {
                    kind: BodyKind::Json,
                    json: Some(other.clone()),
                    ..Default::default()
                };
                metadata.raw_base64 = Some(encode_json_base64(other));
                metadata
            }
        }
    }
}

pub struct BodyPayload {
    pub value: Option<Value>,
    pub metadata: BodyMetadata,
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
    #[serde(rename = "binary")]
    pub binary: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MultipartOptions {
    pub fields: HashMap<String, Value>,
    pub files: Vec<MultipartFile>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
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

    pub fn body_payload(&self) -> BodyPayload {
        let value = if let Some(multipart) = &self.multipart {
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
            Some(json!({
                "__spikard_multipart__": {
                    "fields": multipart.fields,
                    "files": files
                }
            }))
        } else if let Some(form) = &self.form {
            Some(json!({
                "__spikard_form__": form
            }))
        } else if let Some(raw) = &self.form_raw
            && let Some(parsed) = parse_form_raw(raw)
        {
            Some(parsed)
        } else if self.json.is_some() {
            self.json.clone()
        } else if let Some(binary) = &self.binary {
            Some(json!({ "__spikard_base64__": binary }))
        } else {
            None
        };

        let metadata = BodyMetadata::from_body_value(value.as_ref());
        BodyPayload { value, metadata }
    }
}

fn parse_form_raw(raw: &str) -> Option<Value> {
    serde_qs::from_str::<Value>(raw)
        .map(|value| json!({ "__spikard_form__": value }))
        .ok()
}

fn encode_json_base64(value: &Value) -> String {
    let json_text = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());
    BASE64_STANDARD.encode(json_text.as_bytes())
}

fn normalize_form_values(source: &JsonMap<String, Value>) -> HashMap<String, String> {
    let mut form = HashMap::new();
    for (key, value) in source {
        form.insert(key.clone(), value_to_string(value));
    }
    form
}

fn encode_form_bytes(form: &HashMap<String, String>) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for (key, value) in form {
        serializer.append_pair(key, value);
    }
    let encoded = serializer.finish();
    BASE64_STANDARD.encode(encoded.as_bytes())
}

fn normalize_multipart_payload(map: &JsonMap<String, Value>) -> (HashMap<String, String>, Vec<MultipartFile>) {
    let fields = map
        .get("fields")
        .and_then(|value| value.as_object())
        .cloned()
        .unwrap_or_default();
    let form = normalize_form_values(&fields);
    let files = map
        .get("files")
        .and_then(|value| serde_json::from_value::<Vec<MultipartFile>>(value.clone()).ok())
        .unwrap_or_default();
    (form, files)
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        other => serde_json::to_string(other).unwrap_or_else(|_| String::new()),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestPayload {
    pub(crate) method: String,
    pub(crate) path: String,
    #[serde(rename = "pathParams", default)]
    pub(crate) path_params: HashMap<String, String>,
    #[serde(default)]
    pub(crate) query: HashMap<String, Value>,
    #[serde(rename = "rawQuery", default)]
    pub(crate) raw_query: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub(crate) headers: HashMap<String, String>,
    #[serde(default)]
    pub(crate) cookies: HashMap<String, String>,
    #[serde(default)]
    pub(crate) params: HashMap<String, Value>,
    pub(crate) body: Option<Value>,
    #[serde(rename = "__spikard_body_metadata__", skip_serializing_if = "Option::is_none")]
    pub(crate) body_metadata: Option<BodyMetadata>,
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
        body_metadata: BodyMetadata,
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
            body_metadata: Some(body_metadata),
        }
    }

    pub fn ensure_body_metadata(&mut self) {
        if self.body_metadata.is_none() {
            let metadata = BodyMetadata::from_body_value(self.body.as_ref());
            self.body_metadata = Some(metadata);
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
