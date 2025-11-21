//! Handler trait for language-agnostic request handling
//!
//! This module defines the core trait that all language bindings must implement.
//! It's completely language-agnostic - no Python, Node, or WASM knowledge.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Request data extracted from HTTP request
/// This is the language-agnostic representation passed to handlers
///
/// Uses Arc for HashMaps to enable cheap cloning without duplicating data.
/// When RequestData is cloned, only the Arc pointers are cloned, not the underlying data.
///
/// Performance optimization: raw_body stores the unparsed request body bytes.
/// Language bindings should use raw_body when possible to avoid double-parsing.
/// The body field is lazily parsed only when needed for validation.
#[derive(Debug, Clone)]
pub struct RequestData {
    pub path_params: std::sync::Arc<HashMap<String, String>>,
    pub query_params: Value,
    pub raw_query_params: std::sync::Arc<HashMap<String, Vec<String>>>,
    pub body: Value,
    pub raw_body: Option<bytes::Bytes>,
    pub headers: std::sync::Arc<HashMap<String, String>>,
    pub cookies: std::sync::Arc<HashMap<String, String>>,
    pub method: String,
    pub path: String,
}

impl Serialize for RequestData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RequestData", 9)?;
        state.serialize_field("path_params", &*self.path_params)?;
        state.serialize_field("query_params", &self.query_params)?;
        state.serialize_field("raw_query_params", &*self.raw_query_params)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("raw_body", &self.raw_body.as_ref().map(|b| b.as_ref()))?;
        state.serialize_field("headers", &*self.headers)?;
        state.serialize_field("cookies", &*self.cookies)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            PathParams,
            QueryParams,
            RawQueryParams,
            Body,
            RawBody,
            Headers,
            Cookies,
            Method,
            Path,
        }

        struct RequestDataVisitor;

        impl<'de> serde::de::Visitor<'de> for RequestDataVisitor {
            type Value = RequestData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct RequestData")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RequestData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path_params = None;
                let mut query_params = None;
                let mut raw_query_params = None;
                let mut body = None;
                let mut raw_body = None;
                let mut headers = None;
                let mut cookies = None;
                let mut method = None;
                let mut path = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::PathParams => {
                            path_params = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::QueryParams => {
                            query_params = Some(map.next_value()?);
                        }
                        Field::RawQueryParams => {
                            raw_query_params = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::Body => {
                            body = Some(map.next_value()?);
                        }
                        Field::RawBody => {
                            let bytes_vec: Option<Vec<u8>> = map.next_value()?;
                            raw_body = bytes_vec.map(bytes::Bytes::from);
                        }
                        Field::Headers => {
                            headers = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::Cookies => {
                            cookies = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::Method => {
                            method = Some(map.next_value()?);
                        }
                        Field::Path => {
                            path = Some(map.next_value()?);
                        }
                    }
                }

                Ok(RequestData {
                    path_params: path_params.ok_or_else(|| serde::de::Error::missing_field("path_params"))?,
                    query_params: query_params.ok_or_else(|| serde::de::Error::missing_field("query_params"))?,
                    raw_query_params: raw_query_params
                        .ok_or_else(|| serde::de::Error::missing_field("raw_query_params"))?,
                    body: body.ok_or_else(|| serde::de::Error::missing_field("body"))?,
                    raw_body,
                    headers: headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    cookies: cookies.ok_or_else(|| serde::de::Error::missing_field("cookies"))?,
                    method: method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    path: path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "path_params",
            "query_params",
            "raw_query_params",
            "body",
            "raw_body",
            "headers",
            "cookies",
            "method",
            "path",
        ];
        deserializer.deserialize_struct("RequestData", FIELDS, RequestDataVisitor)
    }
}

/// Result type for handlers
pub type HandlerResult = Result<Response<Body>, (StatusCode, String)>;

/// Handler trait that all language bindings must implement
///
/// This trait is completely language-agnostic. Each binding (Python, Node, WASM)
/// implements this trait to bridge their runtime to our HTTP server.
pub trait Handler: Send + Sync {
    /// Handle an HTTP request
    ///
    /// Takes the extracted request data and returns a future that resolves to either:
    /// - Ok(Response): A successful HTTP response
    /// - Err((StatusCode, String)): An error with status code and message
    fn call(
        &self,
        request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>>;
}

/// Validated parameters from request (path, query, headers, cookies)
#[derive(Debug, Clone)]
pub struct ValidatedParams {
    pub params: HashMap<String, Value>,
}
