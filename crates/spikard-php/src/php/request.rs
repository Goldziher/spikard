//! PHP-visible HTTP request struct.

#![allow(non_snake_case)]

use bytes::Bytes;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use ext_php_rs::types::Zval;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

/// PHP-visible HTTP request mirroring `RequestData`.
#[php_class]
#[php(name = "Spikard\\Http\\Request")]
pub struct PhpRequest {
    #[php(prop)]
    pub(crate) method: String,
    #[php(prop)]
    pub(crate) path: String,
    pub(crate) path_params: HashMap<String, String>,
    pub(crate) body: Value,
    pub(crate) files: Value,
    pub(crate) raw_body: Option<Bytes>,
    pub(crate) raw_query: HashMap<String, Vec<String>>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) cookies: HashMap<String, String>,
    pub(crate) validated_params: Option<Value>,

    // Cached PHP-side conversions (built on-demand via __get).
    cached_body: RefCell<Option<Zval>>,
    cached_files: RefCell<Option<Zval>>,
    cached_headers: RefCell<Option<Zval>>,
    cached_cookies: RefCell<Option<Zval>>,
    cached_query_params: RefCell<Option<Zval>>,
    cached_path_params: RefCell<Option<Zval>>,
    cached_validated_params: RefCell<Option<Zval>>,
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
        validatedParams: Option<&Zval>,
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
            validatedParams,
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
        validatedParams: Option<&Zval>,
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
        let validated_value = match validatedParams {
            None => None,
            Some(v) => Some(crate::php::zval_to_json(v).map_err(PhpException::default)?),
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
            validated_params: validated_value,
            cached_body: RefCell::new(None),
            cached_files: RefCell::new(None),
            cached_headers: RefCell::new(None),
            cached_cookies: RefCell::new(None),
            cached_query_params: RefCell::new(None),
            cached_path_params: RefCell::new(None),
            cached_validated_params: RefCell::new(None),
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
        if self.body.is_null()
            && let Some(raw) = self.raw_body.as_ref()
            && !raw.is_empty()
        {
            return String::from_utf8_lossy(raw.as_ref()).to_string();
        }

        match &self.body {
            Value::String(s) => s.clone(),
            _ => serde_json::to_string(&self.body).unwrap_or_else(|_| "{}".to_string()),
        }
    }

    #[php(name = "__get")]
    pub fn __get(&self, name: &str) -> PhpResult<Zval> {
        match name {
            "body" => self.get_cached_json_or_raw_body(),
            "files" => self.get_cached_json_field(&self.files, &self.cached_files),
            "headers" => self.get_cached_string_map(&self.headers, &self.cached_headers),
            "cookies" => self.get_cached_string_map(&self.cookies, &self.cached_cookies),
            "queryParams" => self.get_cached_multimap(&self.raw_query, &self.cached_query_params),
            "pathParams" => self.get_cached_string_map(&self.path_params, &self.cached_path_params),
            "validatedParams" => self.get_cached_validated_params(),
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
        self.build_string_map_table(&self.headers)
    }

    /// Get cookies as a PHP array.
    #[php(name = "getCookies")]
    pub fn get_cookies(&self) -> PhpResult<ZBox<ZendHashTable>> {
        self.build_string_map_table(&self.cookies)
    }

    /// Get query params as a PHP array.
    #[php(name = "getQueryParams")]
    pub fn get_query_params(&self) -> PhpResult<ZBox<ZendHashTable>> {
        self.build_multimap_table(&self.raw_query)
    }

    /// Get path params as a PHP array.
    #[php(name = "getPathParams")]
    pub fn get_path_params(&self) -> PhpResult<ZBox<ZendHashTable>> {
        self.build_string_map_table(&self.path_params)
    }
}

impl PhpRequest {
    fn build_string_map_table(&self, map: &HashMap<String, String>) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = crate::php::php_table_with_capacity(map.len());
        for (k, v) in map {
            crate::php::table_insert_str_fast(&mut table, k.as_str(), v.as_str())?;
        }
        Ok(table)
    }

    fn build_multimap_table(&self, map: &HashMap<String, Vec<String>>) -> PhpResult<ZBox<ZendHashTable>> {
        let mut table = crate::php::php_table_with_capacity(map.len());
        for (k, values) in map {
            let mut inner = crate::php::php_table_with_capacity(values.len());
            for v in values {
                inner.push(v.as_str()).map_err(crate::php::map_ext_php_err)?;
            }
            crate::php::table_insert_str_fast(&mut table, k.as_str(), inner)?;
        }
        Ok(table)
    }

    fn get_cached_json_field(&self, value: &Value, cache: &RefCell<Option<Zval>>) -> PhpResult<Zval> {
        if let Some(zv) = cache.borrow().as_ref() {
            return Ok(zv.shallow_clone());
        }
        let zv = crate::php::json_to_zval(value)?;
        *cache.borrow_mut() = Some(zv.shallow_clone());
        Ok(zv)
    }

    fn get_cached_string_map(&self, map: &HashMap<String, String>, cache: &RefCell<Option<Zval>>) -> PhpResult<Zval> {
        if let Some(zv) = cache.borrow().as_ref() {
            return Ok(zv.shallow_clone());
        }
        let table = self.build_string_map_table(map)?;
        let zv = table.into_zval(false).map_err(crate::php::map_ext_php_err)?;
        *cache.borrow_mut() = Some(zv.shallow_clone());
        Ok(zv)
    }

    fn get_cached_multimap(
        &self,
        map: &HashMap<String, Vec<String>>,
        cache: &RefCell<Option<Zval>>,
    ) -> PhpResult<Zval> {
        if let Some(zv) = cache.borrow().as_ref() {
            return Ok(zv.shallow_clone());
        }
        let table = self.build_multimap_table(map)?;
        let zv = table.into_zval(false).map_err(crate::php::map_ext_php_err)?;
        *cache.borrow_mut() = Some(zv.shallow_clone());
        Ok(zv)
    }

    fn get_cached_json_or_raw_body(&self) -> PhpResult<Zval> {
        if let Some(zv) = self.cached_body.borrow().as_ref() {
            return Ok(zv.shallow_clone());
        }

        let value = if !self.body.is_null() { Some(&self.body) } else { None };

        let zv = if let Some(value) = value {
            crate::php::json_to_zval(value)?
        } else if let Some(raw) = self.raw_body.as_ref()
            && !raw.is_empty()
        {
            match serde_json::from_slice::<Value>(raw.as_ref()) {
                Ok(parsed) => crate::php::json_to_zval(&parsed)?,
                Err(_) => String::from_utf8_lossy(raw.as_ref())
                    .as_ref()
                    .into_zval(false)
                    .map_err(crate::php::map_ext_php_err)?,
            }
        } else {
            Zval::new()
        };

        *self.cached_body.borrow_mut() = Some(zv.shallow_clone());
        Ok(zv)
    }

    fn get_cached_validated_params(&self) -> PhpResult<Zval> {
        if let Some(zv) = self.cached_validated_params.borrow().as_ref() {
            return Ok(zv.shallow_clone());
        }

        let zv = match self.validated_params.as_ref() {
            Some(value) => crate::php::json_to_zval(value)?,
            None => Zval::new(),
        };

        *self.cached_validated_params.borrow_mut() = Some(zv.shallow_clone());
        Ok(zv)
    }

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
            validated_params: data.validated_params.clone(),
            cached_body: RefCell::new(None),
            cached_files: RefCell::new(None),
            cached_headers: RefCell::new(None),
            cached_cookies: RefCell::new(None),
            cached_query_params: RefCell::new(None),
            cached_path_params: RefCell::new(None),
            cached_validated_params: RefCell::new(None),
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
            validated_params: data.validated_params,
            cached_body: RefCell::new(None),
            cached_files: RefCell::new(None),
            cached_headers: RefCell::new(None),
            cached_cookies: RefCell::new(None),
            cached_query_params: RefCell::new(None),
            cached_path_params: RefCell::new(None),
            cached_validated_params: RefCell::new(None),
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
        validated_params: Option<Value>,
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
            validated_params,
            cached_body: RefCell::new(None),
            cached_files: RefCell::new(None),
            cached_headers: RefCell::new(None),
            cached_cookies: RefCell::new(None),
            cached_query_params: RefCell::new(None),
            cached_path_params: RefCell::new(None),
            cached_validated_params: RefCell::new(None),
        }
    }
}
