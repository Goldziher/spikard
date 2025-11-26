use ext_php_rs::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible HTTP request mirroring `RequestData`.
#[php_class(name = "Spikard\\Internal\\Request")]
pub struct PhpRequest {
    method: String,
    path: String,
    path_params: HashMap<String, String>,
    body: Value,
    raw_body: Option<Vec<u8>>,
    raw_query: HashMap<String, Vec<String>>,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
}

#[php_impl]
impl PhpRequest {
    #[constructor]
    pub fn __construct(
        method: String,
        path: String,
        body: Option<Value>,
        raw_body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        raw_query: Option<HashMap<String, Vec<String>>>,
        path_params: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            method,
            path,
            path_params: path_params.unwrap_or_default(),
            body: body.unwrap_or(Value::Null),
            raw_body,
            raw_query: raw_query.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
            cookies: cookies.unwrap_or_default(),
        }
    }

    #[getter]
    pub fn method(&self) -> &str {
        &self.method
    }

    #[getter]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[getter]
    pub fn body(&self) -> &Value {
        &self.body
    }

    /// Raw body bytes (if present).
    #[getter(name = "rawBody")]
    pub fn raw_body(&self) -> Option<&Vec<u8>> {
        self.raw_body.as_ref()
    }

    /// Lowercase header map for case-insensitive lookup.
    #[getter]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[getter]
    pub fn cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }

    /// Raw query parameters (multi-valued).
    #[getter(name = "queryParams")]
    pub fn raw_query(&self) -> &HashMap<String, Vec<String>> {
        &self.raw_query
    }

    /// Path parameters extracted by the router.
    #[getter(name = "pathParams")]
    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.path_params
    }
}
