//! PHP-visible HTTP request struct.

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible HTTP request mirroring `RequestData`.
#[php_class]
#[php(name = "Spikard\\Internal\\Request")]
pub struct PhpRequest {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) path_params: HashMap<String, String>,
    pub(crate) body: Value,
    pub(crate) raw_body: Option<Vec<u8>>,
    pub(crate) raw_query: HashMap<String, Vec<String>>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) cookies: HashMap<String, String>,
}

#[php_impl]
impl PhpRequest {
    /// Create a new request from JSON body string.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        method: String,
        path: String,
        body: Option<String>,
        raw_body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        raw_query: Option<HashMap<String, Vec<String>>>,
        path_params: Option<HashMap<String, String>>,
    ) -> Self {
        let body_value = body
            .as_ref()
            .map(|b| serde_json::from_str(b).unwrap_or(Value::String(b.clone())))
            .unwrap_or(Value::Null);
        Self {
            method,
            path,
            path_params: path_params.unwrap_or_default(),
            body: body_value,
            raw_body,
            raw_query: raw_query.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
            cookies: cookies.unwrap_or_default(),
        }
    }

    /// Get the HTTP method.
    #[php(name = "getMethod")]
    pub fn get_method(&self) -> String {
        self.method.clone()
    }

    /// Get the request path.
    #[php(name = "getPath")]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Get the body as a JSON string.
    #[php(name = "getBody")]
    pub fn get_body(&self) -> String {
        serde_json::to_string(&self.body).unwrap_or_else(|_| "{}".to_string())
    }

    /// Get raw body bytes.
    #[php(name = "getRawBody")]
    pub fn get_raw_body(&self) -> Option<Vec<u8>> {
        self.raw_body.clone()
    }

    /// Get headers as a PHP array.
    #[php(name = "getHeaders")]
    pub fn get_headers(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.headers {
            table.insert(k.as_str(), v.as_str())?;
        }
        Ok(table)
    }

    /// Get cookies as a PHP array.
    #[php(name = "getCookies")]
    pub fn get_cookies(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.cookies {
            table.insert(k.as_str(), v.as_str())?;
        }
        Ok(table)
    }

    /// Get query params as a PHP array.
    #[php(name = "getQueryParams")]
    pub fn get_query_params(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, values) in &self.raw_query {
            let mut inner = ZendHashTable::new();
            for v in values {
                inner.push(v.as_str())?;
            }
            table.insert(k.as_str(), inner)?;
        }
        Ok(table)
    }

    /// Get path params as a PHP array.
    #[php(name = "getPathParams")]
    pub fn get_path_params(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = ZendHashTable::new();
        for (k, v) in &self.path_params {
            table.insert(k.as_str(), v.as_str())?;
        }
        Ok(table)
    }
}

impl PhpRequest {
    /// Build from RequestData (used by Handler bridge).
    pub fn from_request_data(data: &spikard_http::RequestData) -> Self {
        Self {
            method: data.method.clone(),
            path: data.path.clone(),
            path_params: (*data.path_params).clone(),
            body: data.body.clone(),
            raw_body: data.raw_body.as_ref().map(|b| b.to_vec()),
            raw_query: (*data.raw_query_params).clone(),
            headers: (*data.headers).clone(),
            cookies: (*data.cookies).clone(),
        }
    }

    /// Internal constructor for Rust code (not exposed to PHP).
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        method: String,
        path: String,
        body: Value,
        raw_body: Option<Vec<u8>>,
        headers: HashMap<String, String>,
        cookies: HashMap<String, String>,
        raw_query: HashMap<String, Vec<String>>,
        path_params: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            path,
            path_params,
            body,
            raw_body,
            raw_query,
            headers,
            cookies,
        }
    }
}
