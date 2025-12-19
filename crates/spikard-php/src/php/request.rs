//! PHP-visible HTTP request struct.

#![allow(non_snake_case)]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use ext_php_rs::types::Zval;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use bytes::Bytes;

/// PHP-visible HTTP request mirroring `RequestData`.
#[php_class]
#[php(name = "Spikard\\Http\\Request")]
pub struct PhpRequest {
    #[php(prop)]
    pub(crate) method: String,
    #[php(prop)]
    pub(crate) path: String,
    #[php(prop, name = "pathParams")]
    pub(crate) path_params: HashMap<String, String>,
    pub(crate) body: Value,
    pub(crate) files: Value,
    pub(crate) raw_body: Option<Bytes>,
    #[php(prop, name = "queryParams")]
    pub(crate) raw_query: HashMap<String, Vec<String>>,
    #[php(prop)]
    pub(crate) headers: HashMap<String, String>,
    #[php(prop)]
    pub(crate) cookies: HashMap<String, String>,
}

#[php_impl]
impl PhpRequest {
    #[php(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn __construct(
        method: String,
        path: String,
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        queryParams: Option<HashMap<String, Vec<String>>>,
        pathParams: Option<HashMap<String, String>>,
        files: Option<&Zval>,
        dependencies: Option<&Zval>,
    ) -> PhpResult<Self> {
        Self::new(
            method,
            path,
            body,
            headers,
            cookies,
            queryParams,
            pathParams,
            files,
            dependencies,
        )
    }

    /// Create a new request.
    ///
    /// This intentionally matches `packages/php/src/Http/Request.php` so PHP code can use named
    /// arguments like `queryParams:` and `pathParams:` even when the native extension is loaded.
    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    pub fn new(
        method: String,
        path: String,
        body: Option<&Zval>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        queryParams: Option<HashMap<String, Vec<String>>>,
        pathParams: Option<HashMap<String, String>>,
        files: Option<&Zval>,
        dependencies: Option<&Zval>,
    ) -> PhpResult<Self> {
        let _ = dependencies;
        let body_value = match body {
            None => Value::Null,
            Some(v) => crate::php::zval_to_json(v).map_err(PhpException::default)?,
        };

        let files_value = match files {
            None => Value::Object(serde_json::Map::new()),
            Some(v) => crate::php::zval_to_json(v).map_err(PhpException::default)?,
        };

        Ok(Self {
            method,
            path,
            path_params: pathParams.unwrap_or_default(),
            body: body_value,
            files: files_value,
            raw_body: None,
            raw_query: queryParams.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
            cookies: cookies.unwrap_or_default(),
        })
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
        match &self.body {
            Value::String(s) => s.clone(),
            _ => serde_json::to_string(&self.body).unwrap_or_else(|_| "{}".to_string()),
        }
    }

    #[php(name = "__get")]
    pub fn __get(&self, name: String) -> PhpResult<Zval> {
        match name.as_str() {
            "body" => crate::php::json_to_zval(&self.body),
            "files" => crate::php::json_to_zval(&self.files),
            _ => Ok(Zval::new()),
        }
    }

    /// Get raw body bytes.
    #[php(name = "getRawBody")]
    pub fn get_raw_body(&self) -> Option<Vec<u8>> {
        self.raw_body.as_ref().map(|b| b.to_vec())
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
            files: Value::Object(serde_json::Map::new()),
            raw_body: data.raw_body.clone(),
            raw_query: (*data.raw_query_params).clone(),
            headers: (*data.headers).clone(),
            cookies: (*data.cookies).clone(),
        }
    }

    fn unwrap_arc_map<K: Clone, V: Clone>(map: Arc<HashMap<K, V>>) -> HashMap<K, V> {
        Arc::try_unwrap(map).unwrap_or_else(|map| (*map).clone())
    }

    fn unwrap_arc_multimap(map: Arc<HashMap<String, Vec<String>>>) -> HashMap<String, Vec<String>> {
        Arc::try_unwrap(map).unwrap_or_else(|map| (*map).clone())
    }

    /// Build from RequestData by-value to avoid unnecessary cloning when possible.
    pub fn from_request_data_owned(data: spikard_http::RequestData) -> Self {
        Self {
            method: data.method,
            path: data.path,
            path_params: Self::unwrap_arc_map(data.path_params),
            body: data.body,
            files: Value::Object(serde_json::Map::new()),
            raw_body: data.raw_body,
            raw_query: Self::unwrap_arc_multimap(data.raw_query_params),
            headers: Self::unwrap_arc_map(data.headers),
            cookies: Self::unwrap_arc_map(data.cookies),
        }
    }

    /// Internal constructor for Rust code (not exposed to PHP).
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        method: String,
        path: String,
        body: Value,
        files: Value,
        raw_body: Option<Bytes>,
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
            files,
            raw_body,
            raw_query,
            headers,
            cookies,
        }
    }
}
