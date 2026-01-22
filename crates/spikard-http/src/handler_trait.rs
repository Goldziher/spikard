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
    pub query_params: std::sync::Arc<Value>,
    /// Validated parameters produced by ParameterValidator (query/path/header/cookie combined).
    pub validated_params: Option<std::sync::Arc<Value>>,
    pub raw_query_params: std::sync::Arc<HashMap<String, Vec<String>>>,
    pub body: std::sync::Arc<Value>,
    pub raw_body: Option<bytes::Bytes>,
    pub headers: std::sync::Arc<HashMap<String, String>>,
    pub cookies: std::sync::Arc<HashMap<String, String>>,
    pub method: String,
    pub path: String,
    /// Resolved dependencies for this request (populated by DependencyInjectingHandler)
    #[cfg(feature = "di")]
    pub dependencies: Option<std::sync::Arc<spikard_core::di::ResolvedDependencies>>,
}

impl Serialize for RequestData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        #[cfg(feature = "di")]
        let field_count = 11;
        #[cfg(not(feature = "di"))]
        let field_count = 10;

        let mut state = serializer.serialize_struct("RequestData", field_count)?;
        state.serialize_field("path_params", &*self.path_params)?;
        state.serialize_field("query_params", &*self.query_params)?;
        state.serialize_field("validated_params", &self.validated_params.as_deref())?;
        state.serialize_field("raw_query_params", &*self.raw_query_params)?;
        state.serialize_field("body", &*self.body)?;
        state.serialize_field("raw_body", &self.raw_body.as_ref().map(|b| b.as_ref()))?;
        state.serialize_field("headers", &*self.headers)?;
        state.serialize_field("cookies", &*self.cookies)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("path", &self.path)?;

        #[cfg(feature = "di")]
        {
            state.serialize_field("has_dependencies", &self.dependencies.is_some())?;
        }

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
            ValidatedParams,
            Body,
            RawBody,
            Headers,
            Cookies,
            Method,
            Path,
            #[cfg(feature = "di")]
            HasDependencies,
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
                let mut validated_params = None;
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
                            query_params = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::RawQueryParams => {
                            raw_query_params = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::ValidatedParams => {
                            validated_params = Some(std::sync::Arc::new(map.next_value()?));
                        }
                        Field::Body => {
                            body = Some(std::sync::Arc::new(map.next_value()?));
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
                        #[cfg(feature = "di")]
                        Field::HasDependencies => {
                            let _: bool = map.next_value()?;
                        }
                    }
                }

                Ok(RequestData {
                    path_params: path_params.ok_or_else(|| serde::de::Error::missing_field("path_params"))?,
                    query_params: query_params.ok_or_else(|| serde::de::Error::missing_field("query_params"))?,
                    raw_query_params: raw_query_params
                        .ok_or_else(|| serde::de::Error::missing_field("raw_query_params"))?,
                    validated_params,
                    body: body.ok_or_else(|| serde::de::Error::missing_field("body"))?,
                    raw_body,
                    headers: headers.ok_or_else(|| serde::de::Error::missing_field("headers"))?,
                    cookies: cookies.ok_or_else(|| serde::de::Error::missing_field("cookies"))?,
                    method: method.ok_or_else(|| serde::de::Error::missing_field("method"))?,
                    path: path.ok_or_else(|| serde::de::Error::missing_field("path"))?,
                    #[cfg(feature = "di")]
                    dependencies: None,
                })
            }
        }

        #[cfg(feature = "di")]
        const FIELDS: &[&str] = &[
            "path_params",
            "query_params",
            "validated_params",
            "raw_query_params",
            "body",
            "raw_body",
            "headers",
            "cookies",
            "method",
            "path",
            "has_dependencies",
        ];

        #[cfg(not(feature = "di"))]
        const FIELDS: &[&str] = &[
            "path_params",
            "query_params",
            "validated_params",
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

    /// Whether this handler prefers consuming `RequestData::raw_body` over the parsed
    /// `RequestData::body` for JSON requests.
    ///
    /// When `true`, the server may skip eager JSON parsing when there is no request-body
    /// schema validator attached to the route.
    fn prefers_raw_json_body(&self) -> bool {
        false
    }

    /// Whether this handler wants to perform its own parameter validation/extraction (path/query/header/cookie).
    ///
    /// When `true`, the server will skip `ParameterValidator::validate_and_extract` in `ValidatingHandler`.
    /// This is useful for language bindings which need to transform validated parameters into
    /// language-specific values (e.g., Python kwargs) without duplicating work. When `false`,
    /// the server stores validated output in `RequestData::validated_params`.
    fn prefers_parameter_extraction(&self) -> bool {
        false
    }

    /// Whether this handler needs the parsed headers map in `RequestData`.
    ///
    /// When `false`, the server may skip building `RequestData::headers` for requests without a body.
    /// (Requests with bodies still typically need `Content-Type` decisions.)
    fn wants_headers(&self) -> bool {
        true
    }

    /// Whether this handler needs the parsed cookies map in `RequestData`.
    ///
    /// When `false`, the server may skip parsing cookies for requests without a body.
    fn wants_cookies(&self) -> bool {
        true
    }

    /// Whether this handler needs `RequestData` stored in request extensions.
    ///
    /// When `false`, the server avoids inserting `RequestData` into extensions to
    /// skip cloning in hot paths.
    fn wants_request_extensions(&self) -> bool {
        false
    }
}

/// Validated parameters from request (path, query, headers, cookies)
#[derive(Debug, Clone)]
pub struct ValidatedParams {
    pub params: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn minimal_request_data() -> RequestData {
        RequestData {
            path_params: std::sync::Arc::new(HashMap::new()),
            query_params: std::sync::Arc::new(Value::Object(serde_json::Map::new())),
            validated_params: None,
            raw_query_params: std::sync::Arc::new(HashMap::new()),
            body: std::sync::Arc::new(Value::Null),
            raw_body: None,
            headers: std::sync::Arc::new(HashMap::new()),
            cookies: std::sync::Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    #[test]
    fn test_request_data_serialization_minimal() {
        let data = minimal_request_data();

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["path_params"].is_object());
        assert!(json["query_params"].is_object());
        assert!(json["raw_query_params"].is_object());
        assert!(json["body"].is_null());
        assert!(json["headers"].is_object());
        assert!(json["cookies"].is_object());
        assert_eq!(json["method"], "GET");
        assert_eq!(json["path"], "/");
    }

    #[test]
    fn test_request_data_serialization_with_path_params() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());
        path_params.insert("username".to_string(), "alice".to_string());

        let data = RequestData {
            path_params: std::sync::Arc::new(path_params),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["path_params"]["id"], "123");
        assert_eq!(json["path_params"]["username"], "alice");
    }

    #[test]
    fn test_request_data_serialization_with_query_params() {
        let query_params = serde_json::json!({
            "filter": "active",
            "limit": 10,
            "sort": "name"
        });

        let data = RequestData {
            query_params: std::sync::Arc::new(query_params),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["query_params"]["filter"], "active");
        assert_eq!(json["query_params"]["limit"], 10);
        assert_eq!(json["query_params"]["sort"], "name");
    }

    #[test]
    fn test_request_data_serialization_with_raw_query_params() {
        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("tags".to_string(), vec!["rust".to_string(), "web".to_string()]);
        raw_query_params.insert("category".to_string(), vec!["backend".to_string()]);

        let data = RequestData {
            raw_query_params: std::sync::Arc::new(raw_query_params),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["raw_query_params"]["tags"].is_array());
        assert_eq!(json["raw_query_params"]["tags"][0], "rust");
        assert_eq!(json["raw_query_params"]["tags"][1], "web");
    }

    #[test]
    fn test_request_data_serialization_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("authorization".to_string(), "Bearer token123".to_string());
        headers.insert("user-agent".to_string(), "test-client/1.0".to_string());

        let data = RequestData {
            headers: std::sync::Arc::new(headers),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["headers"]["content-type"], "application/json");
        assert_eq!(json["headers"]["authorization"], "Bearer token123");
        assert_eq!(json["headers"]["user-agent"], "test-client/1.0");
    }

    #[test]
    fn test_request_data_serialization_with_cookies() {
        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "abc123def456".to_string());
        cookies.insert("preferences".to_string(), "dark_mode=true".to_string());

        let data = RequestData {
            cookies: std::sync::Arc::new(cookies),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["cookies"]["session_id"], "abc123def456");
        assert_eq!(json["cookies"]["preferences"], "dark_mode=true");
    }

    #[test]
    fn test_request_data_serialization_with_json_body() {
        let body = serde_json::json!({
            "name": "test",
            "age": 30,
            "active": true,
            "tags": ["a", "b"]
        });

        let data = RequestData {
            body: std::sync::Arc::new(body),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["body"]["name"], "test");
        assert_eq!(json["body"]["age"], 30);
        assert_eq!(json["body"]["active"], true);
        assert!(json["body"]["tags"].is_array());
    }

    #[test]
    fn test_request_data_serialization_with_raw_body() {
        let raw_body = bytes::Bytes::from("raw body content");
        let data = RequestData {
            raw_body: Some(raw_body),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["raw_body"].is_array());
        let serialized_bytes: Vec<u8> =
            serde_json::from_value(json["raw_body"].clone()).expect("failed to deserialize bytes");
        assert_eq!(serialized_bytes, b"raw body content".to_vec());
    }

    #[test]
    fn test_request_data_serialization_with_empty_strings() {
        let mut headers = HashMap::new();
        headers.insert("x-empty".to_string(), "".to_string());

        let data = RequestData {
            method: "".to_string(),
            path: "".to_string(),
            headers: std::sync::Arc::new(headers),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["method"], "");
        assert_eq!(json["path"], "");
        assert_eq!(json["headers"]["x-empty"], "");
    }

    #[test]
    fn test_request_data_serialization_with_nested_json_body() {
        let body = serde_json::json!({
            "user": {
                "profile": {
                    "name": "Alice",
                    "contact": {
                        "email": "alice@example.com",
                        "phone": null
                    }
                }
            },
            "settings": {
                "notifications": [true, false, true]
            }
        });

        let data = RequestData {
            body: std::sync::Arc::new(body),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["body"]["user"]["profile"]["name"], "Alice");
        assert_eq!(json["body"]["user"]["profile"]["contact"]["email"], "alice@example.com");
        assert!(json["body"]["user"]["profile"]["contact"]["phone"].is_null());
        assert_eq!(json["body"]["settings"]["notifications"][0], true);
    }

    #[test]
    fn test_request_data_serialization_all_fields_complete() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "42".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("filter".to_string(), vec!["active".to_string()]);

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "xyz789".to_string());

        let body = serde_json::json!({"action": "create"});
        let raw_body = bytes::Bytes::from("body bytes");

        let data = RequestData {
            path_params: std::sync::Arc::new(path_params),
            query_params: std::sync::Arc::new(serde_json::json!({"page": 1})),
            validated_params: None,
            raw_query_params: std::sync::Arc::new(raw_query_params),
            body: std::sync::Arc::new(body),
            raw_body: Some(raw_body),
            headers: std::sync::Arc::new(headers),
            cookies: std::sync::Arc::new(cookies),
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["path_params"]["id"], "42");
        assert_eq!(json["query_params"]["page"], 1);
        assert_eq!(json["raw_query_params"]["filter"][0], "active");
        assert_eq!(json["body"]["action"], "create");
        assert!(json["raw_body"].is_array());
        assert_eq!(json["headers"]["content-type"], "application/json");
        assert_eq!(json["cookies"]["session"], "xyz789");
        assert_eq!(json["method"], "POST");
        assert_eq!(json["path"], "/api/users");
    }

    #[test]
    fn test_request_data_clone_shares_arc_data() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "original".to_string());

        let data1 = RequestData {
            path_params: std::sync::Arc::new(path_params),
            ..minimal_request_data()
        };

        let data2 = data1.clone();

        assert!(std::sync::Arc::ptr_eq(&data1.path_params, &data2.path_params));
    }

    #[test]
    fn test_request_data_deserialization_complete() {
        let json = serde_json::json!({
            "path_params": {"id": "123"},
            "query_params": {"filter": "active"},
            "raw_query_params": {"tags": ["rust", "web"]},
            "body": {"name": "test"},
            "raw_body": null,
            "headers": {"content-type": "application/json"},
            "cookies": {"session": "abc"},
            "method": "POST",
            "path": "/api/test"
        });

        let data: RequestData = serde_json::from_value(json).expect("deserialization failed");

        assert_eq!(data.path_params.get("id").unwrap(), "123");
        assert_eq!(data.query_params["filter"], "active");
        assert_eq!(data.raw_query_params.get("tags").unwrap()[0], "rust");
        assert_eq!(data.body["name"], "test");
        assert!(data.raw_body.is_none());
        assert_eq!(data.headers.get("content-type").unwrap(), "application/json");
        assert_eq!(data.cookies.get("session").unwrap(), "abc");
        assert_eq!(data.method, "POST");
        assert_eq!(data.path, "/api/test");
    }

    #[test]
    fn test_request_data_deserialization_with_raw_body_bytes() {
        let json = serde_json::json!({
            "path_params": {},
            "query_params": {},
            "raw_query_params": {},
            "body": null,
            "raw_body": [72, 101, 108, 108, 111],
            "headers": {},
            "cookies": {},
            "method": "GET",
            "path": "/"
        });

        let data: RequestData = serde_json::from_value(json).expect("deserialization failed");

        assert!(data.raw_body.is_some());
        assert_eq!(data.raw_body.unwrap().as_ref(), b"Hello");
    }

    #[test]
    fn test_request_data_deserialization_missing_required_field_path_params() {
        let json = serde_json::json!({
            "query_params": {},
            "raw_query_params": {},
            "body": null,
            "headers": {},
            "cookies": {},
            "method": "GET",
            "path": "/"
        });

        let result: Result<RequestData, _> = serde_json::from_value(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("path_params"));
    }

    #[test]
    fn test_request_data_deserialization_missing_required_field_method() {
        let json = serde_json::json!({
            "path_params": {},
            "query_params": {},
            "raw_query_params": {},
            "body": null,
            "headers": {},
            "cookies": {},
            "path": "/"
        });

        let result: Result<RequestData, _> = serde_json::from_value(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("method"));
    }

    #[test]
    fn test_request_data_serialization_roundtrip() {
        let original = RequestData {
            path_params: std::sync::Arc::new({
                let mut map = HashMap::new();
                map.insert("id".to_string(), "999".to_string());
                map
            }),
            query_params: std::sync::Arc::new(serde_json::json!({"limit": 50, "offset": 10})),
            validated_params: None,
            raw_query_params: std::sync::Arc::new({
                let mut map = HashMap::new();
                map.insert("sort".to_string(), vec!["name".to_string(), "date".to_string()]);
                map
            }),
            body: std::sync::Arc::new(serde_json::json!({"title": "New Post", "content": "Hello World"})),
            raw_body: None,
            headers: std::sync::Arc::new({
                let mut map = HashMap::new();
                map.insert("accept".to_string(), "application/json".to_string());
                map
            }),
            cookies: std::sync::Arc::new({
                let mut map = HashMap::new();
                map.insert("user_id".to_string(), "user42".to_string());
                map
            }),
            method: "PUT".to_string(),
            path: "/blog/posts/999".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let json = serde_json::to_value(&original).expect("serialization failed");
        let restored: RequestData = serde_json::from_value(json).expect("deserialization failed");

        assert_eq!(*original.path_params, *restored.path_params);
        assert_eq!(original.query_params, restored.query_params);
        assert_eq!(*original.raw_query_params, *restored.raw_query_params);
        assert_eq!(original.body, restored.body);
        assert_eq!(original.raw_body, restored.raw_body);
        assert_eq!(*original.headers, *restored.headers);
        assert_eq!(*original.cookies, *restored.cookies);
        assert_eq!(original.method, restored.method);
        assert_eq!(original.path, restored.path);
    }

    #[test]
    fn test_request_data_serialization_large_body() {
        let mut large_object = serde_json::Map::new();
        for i in 0..100 {
            large_object.insert(format!("key_{}", i), serde_json::Value::String(format!("value_{}", i)));
        }

        let data = RequestData {
            body: std::sync::Arc::new(Value::Object(large_object)),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["body"].is_object());
        assert_eq!(json["body"].get("key_0").unwrap(), "value_0");
        assert_eq!(json["body"].get("key_99").unwrap(), "value_99");
    }

    #[test]
    fn test_request_data_empty_collections() {
        let data = RequestData {
            path_params: std::sync::Arc::new(HashMap::new()),
            query_params: std::sync::Arc::new(Value::Object(serde_json::Map::new())),
            validated_params: None,
            raw_query_params: std::sync::Arc::new(HashMap::new()),
            body: std::sync::Arc::new(Value::Object(serde_json::Map::new())),
            raw_body: None,
            headers: std::sync::Arc::new(HashMap::new()),
            cookies: std::sync::Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["path_params"].as_object().unwrap().len(), 0);
        assert_eq!(json["query_params"].as_object().unwrap().len(), 0);
        assert_eq!(json["raw_query_params"].as_object().unwrap().len(), 0);
        assert_eq!(json["body"].as_object().unwrap().len(), 0);
        assert!(json["raw_body"].is_null());
        assert_eq!(json["headers"].as_object().unwrap().len(), 0);
        assert_eq!(json["cookies"].as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_request_data_special_characters_in_strings() {
        let mut headers = HashMap::new();
        headers.insert("x-custom".to_string(), "value with \"quotes\"".to_string());
        headers.insert("x-unicode".to_string(), "Café ☕ 🚀".to_string());

        let data = RequestData {
            method: "POST".to_string(),
            path: "/api/v1/users\\test".to_string(),
            headers: std::sync::Arc::new(headers),
            body: std::sync::Arc::new(serde_json::json!({"note": "Contains\nnewline"})),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["headers"]["x-custom"], "value with \"quotes\"");
        assert_eq!(json["headers"]["x-unicode"], "Café ☕ 🚀");
        assert_eq!(json["path"], "/api/v1/users\\test");
        assert_eq!(json["body"]["note"], "Contains\nnewline");
    }

    #[test]
    #[cfg(feature = "di")]
    fn test_request_data_serialization_with_di_feature_no_dependencies() {
        let data = minimal_request_data();

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["has_dependencies"], false);
    }

    #[test]
    fn test_request_data_method_variants() {
        let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];

        for method in methods {
            let data = RequestData {
                method: method.to_string(),
                ..minimal_request_data()
            };

            let json = serde_json::to_value(&data).expect("serialization failed");

            assert_eq!(json["method"], method);
        }
    }

    #[test]
    fn test_request_data_serialization_null_body() {
        let data = RequestData {
            body: std::sync::Arc::new(Value::Null),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["body"].is_null());
    }

    #[test]
    fn test_request_data_serialization_array_body() {
        let data = RequestData {
            body: std::sync::Arc::new(serde_json::json!([1, 2, 3, "four", {"five": 5}])),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert!(json["body"].is_array());
        assert_eq!(json["body"][0], 1);
        assert_eq!(json["body"][1], 2);
        assert_eq!(json["body"][3], "four");
        assert_eq!(json["body"][4]["five"], 5);
    }

    #[test]
    fn test_request_data_serialization_numeric_edge_cases() {
        let data = RequestData {
            body: std::sync::Arc::new(serde_json::json!({
                "zero": 0,
                "negative": -42,
                "large": 9223372036854775807i64,
                "float": 3.14159
            })),
            ..minimal_request_data()
        };

        let json = serde_json::to_value(&data).expect("serialization failed");

        assert_eq!(json["body"]["zero"], 0);
        assert_eq!(json["body"]["negative"], -42);
        assert_eq!(json["body"]["large"], 9223372036854775807i64);
        assert_eq!(json["body"]["float"], 3.14159);
    }

    #[test]
    fn test_validated_params_basic_creation() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), Value::String("123".to_string()));
        params.insert("active".to_string(), Value::Bool(true));

        let validated = ValidatedParams { params };

        assert_eq!(validated.params.get("id").unwrap(), &Value::String("123".to_string()));
        assert_eq!(validated.params.get("active").unwrap(), &Value::Bool(true));
    }
}
