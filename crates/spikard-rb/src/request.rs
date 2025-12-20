//! Native Request object for Ruby handlers.
//!
//! Ruby benchmarks frequently access only a subset of request fields. Building a full
//! Ruby Hash for every request eagerly converts headers/cookies/query/etc even when
//! unused. This module provides a native `Spikard::Native::Request` that lazily
//! materialises Ruby values on demand and caches them for subsequent access.

#![deny(clippy::unwrap_used)]

use bytes::Bytes;
use magnus::prelude::*;
use magnus::value::InnerValue;
use magnus::value::Opaque;
use magnus::{Error, RHash, RString, Ruby, Symbol, Value, gc::Marker};
use serde_json::Value as JsonValue;
use spikard_http::RequestData;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::conversion::{map_to_ruby_hash, multimap_to_ruby_hash};
use crate::metadata::json_to_ruby;

#[derive(Default)]
struct RequestCache {
    method: Option<Opaque<Value>>,
    path: Option<Opaque<Value>>,
    path_params: Option<Opaque<Value>>,
    query: Option<Opaque<Value>>,
    raw_query: Option<Opaque<Value>>,
    headers: Option<Opaque<Value>>,
    cookies: Option<Opaque<Value>>,
    body: Option<Opaque<Value>>,
    raw_body: Option<Opaque<Value>>,
    params: Option<Opaque<Value>>,
    to_h: Option<Opaque<Value>>,
}

#[magnus::wrap(class = "Spikard::Native::Request", free_immediately, mark)]
pub(crate) struct NativeRequest {
    method: String,
    path: String,
    path_params: Arc<HashMap<String, String>>,
    query_params: JsonValue,
    raw_query_params: Arc<HashMap<String, Vec<String>>>,
    body: JsonValue,
    raw_body: Option<Bytes>,
    headers: Arc<HashMap<String, String>>,
    cookies: Arc<HashMap<String, String>>,
    validated_params: Option<JsonValue>,
    cache: RefCell<RequestCache>,
}

impl NativeRequest {
    pub(crate) fn from_request_data(request_data: RequestData, validated_params: Option<JsonValue>) -> Self {
        let RequestData {
            path_params,
            query_params,
            raw_query_params,
            body,
            raw_body,
            headers,
            cookies,
            method,
            path,
            ..
        } = request_data;

        Self {
            method,
            path,
            path_params,
            query_params,
            raw_query_params,
            body,
            raw_body,
            headers,
            cookies,
            validated_params,
            cache: RefCell::new(RequestCache::default()),
        }
    }

    fn cache_get(cache: &Option<Opaque<Value>>, ruby: &Ruby) -> Option<Value> {
        cache.as_ref().map(|v| v.get_inner_with(ruby))
    }

    fn cache_set(slot: &mut Option<Opaque<Value>>, value: Value) -> Value {
        *slot = Some(Opaque::from(value));
        value
    }

    pub(crate) fn method(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.method, ruby) {
            return Ok(value);
        }
        Ok(Self::cache_set(
            &mut cache.method,
            ruby.str_new(&this.method).as_value(),
        ))
    }

    pub(crate) fn path(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.path, ruby) {
            return Ok(value);
        }
        Ok(Self::cache_set(&mut cache.path, ruby.str_new(&this.path).as_value()))
    }

    pub(crate) fn path_params(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.path_params, ruby) {
            return Ok(value);
        }
        let value = map_to_ruby_hash(ruby, this.path_params.as_ref())?;
        Ok(Self::cache_set(&mut cache.path_params, value))
    }

    pub(crate) fn query(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.query, ruby) {
            return Ok(value);
        }
        let value = json_to_ruby(ruby, &this.query_params)?;
        Ok(Self::cache_set(&mut cache.query, value))
    }

    pub(crate) fn raw_query(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.raw_query, ruby) {
            return Ok(value);
        }
        let value = multimap_to_ruby_hash(ruby, this.raw_query_params.as_ref())?;
        Ok(Self::cache_set(&mut cache.raw_query, value))
    }

    pub(crate) fn headers(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.headers, ruby) {
            return Ok(value);
        }
        let value = map_to_ruby_hash(ruby, this.headers.as_ref())?;
        Ok(Self::cache_set(&mut cache.headers, value))
    }

    pub(crate) fn cookies(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.cookies, ruby) {
            return Ok(value);
        }
        let value = map_to_ruby_hash(ruby, this.cookies.as_ref())?;
        Ok(Self::cache_set(&mut cache.cookies, value))
    }

    pub(crate) fn body(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.body, ruby) {
            return Ok(value);
        }
        let value = json_to_ruby(ruby, &this.body)?;
        Ok(Self::cache_set(&mut cache.body, value))
    }

    pub(crate) fn raw_body(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.raw_body, ruby) {
            return Ok(value);
        }
        let value = match &this.raw_body {
            Some(bytes) => ruby.str_from_slice(bytes.as_ref()).as_value(),
            None => ruby.qnil().as_value(),
        };
        Ok(Self::cache_set(&mut cache.raw_body, value))
    }

    pub(crate) fn params(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.params, ruby) {
            return Ok(value);
        }

        let value = if let Some(validated) = &this.validated_params {
            json_to_ruby(ruby, validated)?
        } else {
            let params = ruby.hash_new();
            if let Some(hash) = RHash::from_value(Self::path_params(ruby, this)?) {
                let _: Value = params.funcall("merge!", (hash,))?;
            }
            if let Some(hash) = RHash::from_value(Self::query(ruby, this)?) {
                let _: Value = params.funcall("merge!", (hash,))?;
            }
            if let Some(hash) = RHash::from_value(Self::headers(ruby, this)?) {
                let _: Value = params.funcall("merge!", (hash,))?;
            }
            if let Some(hash) = RHash::from_value(Self::cookies(ruby, this)?) {
                let _: Value = params.funcall("merge!", (hash,))?;
            }
            params.as_value()
        };

        Ok(Self::cache_set(&mut cache.params, value))
    }

    pub(crate) fn to_h(ruby: &Ruby, this: &Self) -> Result<Value, Error> {
        let mut cache = this.cache.borrow_mut();
        if let Some(value) = Self::cache_get(&cache.to_h, ruby) {
            return Ok(value);
        }

        let hash = ruby.hash_new_capa(10);
        hash.aset(ruby.to_symbol("method"), Self::method(ruby, this)?)?;
        hash.aset(ruby.to_symbol("path"), Self::path(ruby, this)?)?;
        hash.aset(ruby.to_symbol("path_params"), Self::path_params(ruby, this)?)?;
        hash.aset(ruby.to_symbol("query"), Self::query(ruby, this)?)?;
        hash.aset(ruby.to_symbol("raw_query"), Self::raw_query(ruby, this)?)?;
        hash.aset(ruby.to_symbol("headers"), Self::headers(ruby, this)?)?;
        hash.aset(ruby.to_symbol("cookies"), Self::cookies(ruby, this)?)?;
        hash.aset(ruby.to_symbol("body"), Self::body(ruby, this)?)?;
        hash.aset(ruby.to_symbol("raw_body"), Self::raw_body(ruby, this)?)?;
        hash.aset(ruby.to_symbol("params"), Self::params(ruby, this)?)?;

        Ok(Self::cache_set(&mut cache.to_h, hash.as_value()))
    }

    pub(crate) fn index(ruby: &Ruby, this: &Self, key: Value) -> Result<Value, Error> {
        let key_str = if let Ok(sym) = Symbol::try_convert(key) {
            sym.name()?.into_owned()
        } else if let Ok(text) = RString::try_convert(key) {
            text.to_string()?
        } else {
            return Ok(ruby.qnil().as_value());
        };

        match key_str.as_str() {
            "method" => Self::method(ruby, this),
            "path" => Self::path(ruby, this),
            "path_params" => Self::path_params(ruby, this),
            "query" => Self::query(ruby, this),
            "raw_query" => Self::raw_query(ruby, this),
            "headers" => Self::headers(ruby, this),
            "cookies" => Self::cookies(ruby, this),
            "body" => Self::body(ruby, this),
            "raw_body" => Self::raw_body(ruby, this),
            "params" => Self::params(ruby, this),
            _ => Ok(ruby.qnil().as_value()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn mark(&self, marker: &Marker) {
        if let Ok(ruby) = Ruby::get() {
            let cache = self.cache.borrow();
            for handle in [
                &cache.method,
                &cache.path,
                &cache.path_params,
                &cache.query,
                &cache.raw_query,
                &cache.headers,
                &cache.cookies,
                &cache.body,
                &cache.raw_body,
                &cache.params,
                &cache.to_h,
            ]
            .into_iter()
            .filter_map(|value| value.as_ref())
            {
                marker.mark(handle.get_inner_with(&ruby));
            }
        }
    }
}
