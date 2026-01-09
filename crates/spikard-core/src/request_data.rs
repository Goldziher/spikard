//! Request data structures for HTTP handlers
//!
//! This module provides the `RequestData` type which represents extracted
//! HTTP request data in a language-agnostic format.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "di")]
use crate::di::ResolvedDependencies;
#[cfg(feature = "di")]
use bytes::Bytes;

/// Request data extracted from HTTP request
///
/// This is the language-agnostic representation passed to handlers.
///
/// Uses `Arc` for `HashMap`s to enable cheap cloning without duplicating data.
/// When `RequestData` is cloned, only the `Arc` pointers are cloned, not the underlying data.
///
/// Performance optimization: `raw_body` stores the unparsed request body bytes.
/// Language bindings should use `raw_body` when possible to avoid double-parsing.
/// The `body` field is lazily parsed only when needed for validation.
#[derive(Debug, Clone)]
pub struct RequestData {
    /// Path parameters extracted from the URL path
    pub path_params: Arc<HashMap<String, String>>,
    /// Query parameters parsed as JSON
    pub query_params: Value,
    /// Validated parameters produced by `ParameterValidator` (query/path/header/cookie combined).
    pub validated_params: Option<Value>,
    /// Raw query parameters as key-value pairs
    pub raw_query_params: Arc<HashMap<String, Vec<String>>>,
    /// Parsed request body as JSON
    pub body: Value,
    /// Raw request body bytes (optional, for zero-copy access)
    #[cfg(feature = "di")]
    pub raw_body: Option<Bytes>,
    #[cfg(not(feature = "di"))]
    pub raw_body: Option<Vec<u8>>,
    /// Request headers
    pub headers: Arc<HashMap<String, String>>,
    /// Request cookies
    pub cookies: Arc<HashMap<String, String>>,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request path
    pub path: String,
    /// Resolved dependencies for this request (populated by DI handlers)
    #[cfg(feature = "di")]
    pub dependencies: Option<Arc<ResolvedDependencies>>,
}

impl Serialize for RequestData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RequestData", 10)?;
        state.serialize_field("path_params", &*self.path_params)?;
        state.serialize_field("query_params", &self.query_params)?;
        state.serialize_field("validated_params", &self.validated_params)?;
        state.serialize_field("raw_query_params", &*self.raw_query_params)?;
        state.serialize_field("body", &self.body)?;
        #[cfg(feature = "di")]
        state.serialize_field("raw_body", &self.raw_body.as_ref().map(AsRef::as_ref))?;
        #[cfg(not(feature = "di"))]
        state.serialize_field("raw_body", &self.raw_body)?;
        state.serialize_field("headers", &*self.headers)?;
        state.serialize_field("cookies", &*self.cookies)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("path", &self.path)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RequestData {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            PathParams,
            QueryParams,
            ValidatedParams,
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
                let mut validated_params = None;
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
                            path_params = Some(Arc::new(map.next_value()?));
                        }
                        Field::QueryParams => {
                            query_params = Some(map.next_value()?);
                        }
                        Field::ValidatedParams => {
                            validated_params = Some(map.next_value()?);
                        }
                        Field::RawQueryParams => {
                            raw_query_params = Some(Arc::new(map.next_value()?));
                        }
                        Field::Body => {
                            body = Some(map.next_value()?);
                        }
                        Field::RawBody => {
                            let bytes_vec: Option<Vec<u8>> = map.next_value()?;
                            #[cfg(feature = "di")]
                            {
                                raw_body = bytes_vec.map(Bytes::from);
                            }
                            #[cfg(not(feature = "di"))]
                            {
                                raw_body = bytes_vec;
                            }
                        }
                        Field::Headers => {
                            headers = Some(Arc::new(map.next_value()?));
                        }
                        Field::Cookies => {
                            cookies = Some(Arc::new(map.next_value()?));
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
                    validated_params,
                    raw_query_params: raw_query_params
                        .ok_or_else(|| serde::de::Error::missing_field("raw_query_params"))?,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[derive(Default)]
    struct RequestDataCollections {
        raw_query_params: HashMap<String, Vec<String>>,
        headers: HashMap<String, String>,
        cookies: HashMap<String, String>,
        path_params: HashMap<String, String>,
    }

    fn create_request_data(
        path: &str,
        method: &str,
        body: Value,
        query_params: Value,
        collections: RequestDataCollections,
    ) -> RequestData {
        RequestData {
            path_params: Arc::new(collections.path_params),
            query_params,
            validated_params: None,
            raw_query_params: Arc::new(collections.raw_query_params),
            body,
            raw_body: None,
            headers: Arc::new(collections.headers),
            cookies: Arc::new(collections.cookies),
            method: method.to_string(),
            path: path.to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        }
    }

    #[test]
    fn test_request_data_minimal() {
        let data = create_request_data(
            "/api/users",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        assert_eq!(data.path, "/api/users");
        assert_eq!(data.method, "GET");
        assert_eq!(data.body, json!(null));
    }

    #[test]
    fn test_request_data_with_json_body() {
        let body = json!({
            "name": "test_user",
            "email": "test@example.com",
            "age": 30
        });

        let data = create_request_data(
            "/api/users",
            "POST",
            body.clone(),
            json!({}),
            RequestDataCollections::default(),
        );

        assert_eq!(data.body, body);
        assert_eq!(data.body["name"], "test_user");
        assert_eq!(data.body["email"], "test@example.com");
        assert_eq!(data.body["age"], 30);
    }

    #[test]
    fn test_request_data_with_query_params() {
        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("page".to_string(), vec!["1".to_string()]);
        raw_query_params.insert("limit".to_string(), vec!["10".to_string()]);
        raw_query_params.insert("tags".to_string(), vec!["rust".to_string(), "web".to_string()]);

        let query_params = json!({
            "page": "1",
            "limit": "10",
            "tags": ["rust", "web"]
        });

        let data = create_request_data(
            "/api/users",
            "GET",
            json!(null),
            query_params,
            RequestDataCollections {
                raw_query_params,
                ..Default::default()
            },
        );

        assert_eq!(data.raw_query_params.get("page"), Some(&vec!["1".to_string()]));
        assert_eq!(data.raw_query_params.get("limit"), Some(&vec!["10".to_string()]));
        assert_eq!(
            data.raw_query_params.get("tags"),
            Some(&vec!["rust".to_string(), "web".to_string()])
        );
    }

    #[test]
    fn test_request_data_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("authorization".to_string(), "Bearer token123".to_string());
        headers.insert("user-agent".to_string(), "Mozilla/5.0".to_string());

        let data = create_request_data(
            "/api/users",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                headers,
                ..Default::default()
            },
        );

        assert_eq!(data.headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(data.headers.get("authorization"), Some(&"Bearer token123".to_string()));
        assert_eq!(data.headers.get("user-agent"), Some(&"Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_request_data_with_cookies() {
        let mut cookies = HashMap::new();
        cookies.insert("session_id".to_string(), "abc123xyz".to_string());
        cookies.insert("user_id".to_string(), "user_42".to_string());
        cookies.insert("theme".to_string(), "dark".to_string());

        let data = create_request_data(
            "/api/users",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                cookies,
                ..Default::default()
            },
        );

        assert_eq!(data.cookies.get("session_id"), Some(&"abc123xyz".to_string()));
        assert_eq!(data.cookies.get("user_id"), Some(&"user_42".to_string()));
        assert_eq!(data.cookies.get("theme"), Some(&"dark".to_string()));
    }

    #[test]
    fn test_request_data_with_path_params() {
        let mut path_params = HashMap::new();
        path_params.insert("user_id".to_string(), "123".to_string());
        path_params.insert("post_id".to_string(), "456".to_string());

        let data = create_request_data(
            "/api/users/123/posts/456",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                path_params,
                ..Default::default()
            },
        );

        assert_eq!(data.path_params.get("user_id"), Some(&"123".to_string()));
        assert_eq!(data.path_params.get("post_id"), Some(&"456".to_string()));
    }

    #[test]
    fn test_request_data_all_fields_populated() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "user_99".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("filter".to_string(), vec!["active".to_string()]);

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "xyz789".to_string());

        let body = json!({
            "name": "John",
            "email": "john@example.com"
        });

        let query_params = json!({
            "filter": "active"
        });

        let data = create_request_data(
            "/api/users/user_99",
            "PUT",
            body,
            query_params,
            RequestDataCollections {
                raw_query_params,
                headers,
                cookies,
                path_params,
            },
        );

        assert_eq!(data.path, "/api/users/user_99");
        assert_eq!(data.method, "PUT");
        assert_eq!(data.path_params.get("id"), Some(&"user_99".to_string()));
        assert_eq!(data.body["name"], "John");
        assert_eq!(data.headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(data.cookies.get("session"), Some(&"xyz789".to_string()));
    }

    #[test]
    fn test_request_data_empty_collections() {
        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        assert!(data.path_params.is_empty());
        assert!(data.raw_query_params.is_empty());
        assert!(data.headers.is_empty());
        assert!(data.cookies.is_empty());
    }

    #[test]
    fn test_request_data_clone() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());

        let data1 = create_request_data(
            "/api/users/123",
            "GET",
            json!({"name": "test"}),
            json!({}),
            RequestDataCollections {
                path_params,
                ..Default::default()
            },
        );

        let data2 = data1.clone();

        assert_eq!(data1.path, data2.path);
        assert_eq!(data1.method, data2.method);
        assert_eq!(data1.body, data2.body);
        assert_eq!(data1.path_params, data2.path_params);
    }

    #[test]
    fn test_request_data_various_http_methods() {
        let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];

        for method in methods {
            let data = create_request_data(
                "/api/test",
                method,
                json!(null),
                json!({}),
                RequestDataCollections::default(),
            );

            assert_eq!(data.method, method);
        }
    }

    #[test]
    fn test_request_data_raw_body_none() {
        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        assert!(data.raw_body.is_none());
    }

    #[cfg(not(feature = "di"))]
    #[test]
    fn test_request_data_raw_body_some() {
        let raw_body_bytes = vec![1, 2, 3, 4, 5];
        let mut data = create_request_data(
            "/api/test",
            "POST",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        data.raw_body = Some(raw_body_bytes.clone());
        assert_eq!(data.raw_body, Some(raw_body_bytes));
    }

    #[cfg(not(feature = "di"))]
    #[test]
    fn request_data_serializes_and_deserializes() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "abc".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("tags".to_string(), vec!["rust".to_string(), "http".to_string()]);

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());

        let mut data = create_request_data(
            "/api/items/123",
            "POST",
            json!({"name": "demo"}),
            json!({"tags": ["rust", "http"]}),
            RequestDataCollections {
                raw_query_params,
                headers,
                cookies,
                path_params,
            },
        );
        data.raw_body = Some(vec![9, 8, 7]);

        let encoded = serde_json::to_string(&data).expect("serialize RequestData");
        let decoded: RequestData = serde_json::from_str(&encoded).expect("deserialize RequestData");

        assert_eq!(decoded.path, "/api/items/123");
        assert_eq!(decoded.method, "POST");
        assert_eq!(decoded.body["name"], "demo");
        assert_eq!(decoded.query_params["tags"][0], "rust");
        assert_eq!(
            decoded.raw_query_params.get("tags").unwrap(),
            &vec!["rust".to_string(), "http".to_string()]
        );
        assert_eq!(decoded.headers.get("content-type").unwrap(), "application/json");
        assert_eq!(decoded.cookies.get("session").unwrap(), "abc");
        assert_eq!(decoded.path_params.get("id").unwrap(), "123");
        assert_eq!(decoded.raw_body, Some(vec![9, 8, 7]));
    }

    #[test]
    fn test_request_data_multiple_query_param_values() {
        let mut raw_query_params = HashMap::new();
        raw_query_params.insert(
            "colors".to_string(),
            vec!["red".to_string(), "green".to_string(), "blue".to_string()],
        );

        let data = create_request_data(
            "/api/items",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                raw_query_params,
                ..Default::default()
            },
        );

        let colors = data.raw_query_params.get("colors").unwrap();
        assert_eq!(colors.len(), 3);
        assert!(colors.contains(&"red".to_string()));
        assert!(colors.contains(&"green".to_string()));
        assert!(colors.contains(&"blue".to_string()));
    }

    #[test]
    fn test_request_data_complex_json_body() {
        let body = json!({
            "user": {
                "name": "Alice",
                "profile": {
                    "age": 28,
                    "location": "San Francisco"
                }
            },
            "tags": ["admin", "active"],
            "metadata": {
                "created_at": "2024-01-01",
                "updated_at": "2024-12-01"
            }
        });

        let data = create_request_data("/api/users", "POST", body, json!({}), RequestDataCollections::default());

        assert_eq!(data.body["user"]["name"], "Alice");
        assert_eq!(data.body["user"]["profile"]["age"], 28);
        assert_eq!(data.body["tags"][0], "admin");
        assert_eq!(data.body["metadata"]["created_at"], "2024-01-01");
    }

    #[test]
    fn test_request_data_special_characters_in_paths() {
        let data = create_request_data(
            "/api/users/john@example.com/posts",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        assert_eq!(data.path, "/api/users/john@example.com/posts");
    }

    #[test]
    fn test_request_data_special_characters_in_params() {
        let mut path_params = HashMap::new();
        path_params.insert("email".to_string(), "test@example.com".to_string());
        path_params.insert("slug".to_string(), "my-post-title".to_string());

        let data = create_request_data(
            "/api/users/test@example.com",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                path_params,
                ..Default::default()
            },
        );

        assert_eq!(data.path_params.get("email"), Some(&"test@example.com".to_string()));
        assert_eq!(data.path_params.get("slug"), Some(&"my-post-title".to_string()));
    }

    #[test]
    fn test_request_data_null_and_empty_values() {
        let body = json!({
            "name": null,
            "email": "",
            "age": 0,
            "active": false
        });

        let data = create_request_data("/api/users", "POST", body, json!({}), RequestDataCollections::default());

        assert!(data.body["name"].is_null());
        assert_eq!(data.body["email"], "");
        assert_eq!(data.body["age"], 0);
        assert_eq!(data.body["active"], false);
    }

    #[test]
    fn test_request_data_serialization() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let data = create_request_data(
            "/api/users/123",
            "GET",
            json!({"name": "test"}),
            json!({"page": "1"}),
            RequestDataCollections {
                headers,
                path_params,
                ..Default::default()
            },
        );

        let serialized = serde_json::to_string(&data);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_request_data_delete_request() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "999".to_string());

        let data = create_request_data(
            "/api/users/999",
            "DELETE",
            json!(null),
            json!({}),
            RequestDataCollections {
                path_params,
                ..Default::default()
            },
        );

        assert_eq!(data.method, "DELETE");
        assert_eq!(data.path_params.get("id"), Some(&"999".to_string()));
    }

    #[test]
    fn test_request_data_array_body() {
        let body = json!([
            {"id": 1, "name": "item1"},
            {"id": 2, "name": "item2"},
            {"id": 3, "name": "item3"}
        ]);

        let data = create_request_data("/api/items", "POST", body, json!({}), RequestDataCollections::default());

        assert!(data.body.is_array());
        assert_eq!(data.body.as_array().unwrap().len(), 3);
        assert_eq!(data.body[0]["id"], 1);
        assert_eq!(data.body[2]["name"], "item3");
    }

    #[test]
    fn test_request_data_case_sensitive_keys() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("content-type".to_string(), "text/plain".to_string());

        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                headers,
                ..Default::default()
            },
        );

        assert_eq!(data.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(data.headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_request_data_serialization_with_all_fields() {
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "42".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("page".to_string(), vec!["2".to_string()]);

        let mut headers = HashMap::new();
        headers.insert("authorization".to_string(), "Bearer xyz".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("token".to_string(), "abc123".to_string());

        let data = create_request_data(
            "/api/resource/42",
            "PUT",
            json!({"status": "updated"}),
            json!({"page": "2"}),
            RequestDataCollections {
                raw_query_params,
                headers,
                cookies,
                path_params,
            },
        );

        let serialized = serde_json::to_string(&data).expect("serialization failed");
        assert!(!serialized.is_empty());
        assert!(serialized.contains("\"id\":\"42\""));
        assert!(serialized.contains("PUT"));
    }

    #[test]
    fn test_request_data_deserialization() {
        let json_str = r#"{
            "path_params": {"user_id": "123"},
            "query_params": {"page": "1"},
            "raw_query_params": {"page": ["1"]},
            "body": {"name": "test"},
            "raw_body": null,
            "headers": {"content-type": "application/json"},
            "cookies": {"session": "abc"},
            "method": "POST",
            "path": "/api/users"
        }"#;

        let deserialized: RequestData = serde_json::from_str(json_str).expect("deserialization failed");

        assert_eq!(deserialized.method, "POST");
        assert_eq!(deserialized.path, "/api/users");
        assert_eq!(deserialized.path_params.get("user_id"), Some(&"123".to_string()));
        assert_eq!(deserialized.body["name"], "test");
        assert_eq!(
            deserialized.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(deserialized.cookies.get("session"), Some(&"abc".to_string()));
    }

    #[test]
    fn test_request_data_roundtrip_serialization() {
        let mut path_params = HashMap::new();
        path_params.insert("item_id".to_string(), "789".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("sort".to_string(), vec!["desc".to_string()]);

        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "value123".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("prefer".to_string(), "dark_mode".to_string());

        let original = create_request_data(
            "/api/items/789",
            "DELETE",
            json!({"reason": "archived"}),
            json!({"sort": "desc"}),
            RequestDataCollections {
                raw_query_params,
                headers,
                cookies,
                path_params,
            },
        );

        let serialized = serde_json::to_string(&original).expect("serialization failed");
        let deserialized: RequestData = serde_json::from_str(&serialized).expect("deserialization failed");

        assert_eq!(deserialized.method, original.method);
        assert_eq!(deserialized.path, original.path);
        assert_eq!(deserialized.body, original.body);
        assert_eq!(deserialized.path_params, original.path_params);
    }

    #[test]
    fn test_request_data_large_json_body() {
        let mut large_obj = serde_json::Map::new();
        for i in 0..100 {
            large_obj.insert(format!("field_{i}"), json!({"value": i, "name": format!("item_{}", i)}));
        }
        let body = json!(large_obj);

        let data = create_request_data("/api/batch", "POST", body, json!({}), RequestDataCollections::default());

        assert!(data.body.is_object());
        assert_eq!(data.body.as_object().unwrap().len(), 100);
    }

    #[test]
    fn test_request_data_deeply_nested_json() {
        let body = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "level5": {
                                "value": "deep"
                            }
                        }
                    }
                }
            }
        });

        let data = create_request_data("/api/nested", "GET", body, json!({}), RequestDataCollections::default());

        assert_eq!(
            data.body["level1"]["level2"]["level3"]["level4"]["level5"]["value"],
            "deep"
        );
    }

    #[test]
    fn test_request_data_unicode_in_fields() {
        let mut path_params = HashMap::new();
        path_params.insert("name".to_string(), "用户".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("msg".to_string(), "こんにちは".to_string());

        let body = json!({
            "greeting": "مرحبا",
            "emoji": "🚀"
        });

        let data = create_request_data(
            "/api/users/用户",
            "POST",
            body,
            json!({}),
            RequestDataCollections {
                cookies,
                path_params,
                ..Default::default()
            },
        );

        assert_eq!(data.path_params.get("name"), Some(&"用户".to_string()));
        assert_eq!(data.cookies.get("msg"), Some(&"こんにちは".to_string()));
        assert_eq!(data.body["emoji"], "🚀");
    }

    #[test]
    fn test_request_data_multiple_headers_with_same_prefix() {
        let mut headers = HashMap::new();
        headers.insert("x-custom-1".to_string(), "value1".to_string());
        headers.insert("x-custom-2".to_string(), "value2".to_string());
        headers.insert("x-custom-3".to_string(), "value3".to_string());

        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                headers,
                ..Default::default()
            },
        );

        assert_eq!(data.headers.get("x-custom-1"), Some(&"value1".to_string()));
        assert_eq!(data.headers.get("x-custom-2"), Some(&"value2".to_string()));
        assert_eq!(data.headers.get("x-custom-3"), Some(&"value3".to_string()));
        assert_eq!(data.headers.len(), 3);
    }

    #[test]
    fn test_request_data_numeric_values_in_json() {
        let body = json!({
            "int": 42,
            "float": 3.2,
            "negative": -100,
            "zero": 0,
            "large": 9_223_372_036_854_775_807i64
        });

        let data = create_request_data(
            "/api/numbers",
            "POST",
            body,
            json!({}),
            RequestDataCollections::default(),
        );

        assert_eq!(data.body["int"], 42);
        assert_eq!(data.body["float"], 3.2);
        assert_eq!(data.body["negative"], -100);
        assert_eq!(data.body["zero"], 0);
    }

    #[test]
    fn test_request_data_boolean_values_in_json() {
        let body = json!({
            "is_active": true,
            "is_admin": false
        });

        let data = create_request_data("/api/config", "GET", body, json!({}), RequestDataCollections::default());

        assert_eq!(data.body["is_active"], true);
        assert_eq!(data.body["is_admin"], false);
    }

    #[test]
    fn test_request_data_missing_optional_raw_body() {
        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections::default(),
        );

        assert!(data.raw_body.is_none());
    }

    #[test]
    fn test_request_data_path_with_query_string_format() {
        let data = create_request_data(
            "/api/search?q=test&limit=10",
            "GET",
            json!(null),
            json!({"q": "test", "limit": "10"}),
            RequestDataCollections::default(),
        );

        assert_eq!(data.path, "/api/search?q=test&limit=10");
    }

    #[test]
    fn test_request_data_root_path() {
        let data = create_request_data("/", "GET", json!(null), json!({}), RequestDataCollections::default());

        assert_eq!(data.path, "/");
    }

    #[test]
    fn test_request_data_empty_string_values() {
        let mut headers = HashMap::new();
        headers.insert("empty-header".to_string(), String::new());

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), String::new());

        let data = create_request_data(
            "/api/test",
            "GET",
            json!(null),
            json!({}),
            RequestDataCollections {
                headers,
                path_params,
                ..Default::default()
            },
        );

        assert_eq!(data.headers.get("empty-header"), Some(&String::new()));
        assert_eq!(data.path_params.get("id"), Some(&String::new()));
    }

    #[test]
    fn test_request_data_deserialization_missing_field() {
        let json_str = r#"{
            "path_params": {"user_id": "123"},
            "query_params": {"page": "1"},
            "raw_query_params": {"page": ["1"]},
            "body": {"name": "test"},
            "raw_body": null,
            "headers": {"content-type": "application/json"},
            "cookies": {"session": "abc"}
        }"#;

        let result: Result<RequestData, _> = serde_json::from_str(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_data_deserialization_extra_fields_rejected() {
        let json_str = r#"{
            "path_params": {"user_id": "123"},
            "query_params": {"page": "1"},
            "raw_query_params": {"page": ["1"]},
            "body": {"name": "test"},
            "raw_body": null,
            "headers": {"content-type": "application/json"},
            "cookies": {"session": "abc"},
            "method": "GET",
            "path": "/api/test",
            "extra_field": "not allowed"
        }"#;

        let result: Result<RequestData, _> = serde_json::from_str(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_data_multiple_raw_body_values() {
        let mut raw_query_params = HashMap::new();
        raw_query_params.insert(
            "data".to_string(),
            vec!["val1".to_string(), "val2".to_string(), "val3".to_string()],
        );

        let data = create_request_data(
            "/api/test",
            "POST",
            json!(null),
            json!({}),
            RequestDataCollections {
                raw_query_params,
                ..Default::default()
            },
        );

        let values = data.raw_query_params.get("data").unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], "val1");
        assert_eq!(values[1], "val2");
        assert_eq!(values[2], "val3");
    }

    #[test]
    fn test_request_data_debug_output() {
        let data = create_request_data(
            "/api/test",
            "GET",
            json!({"key": "value"}),
            json!({}),
            RequestDataCollections::default(),
        );

        let debug_str = format!("{data:?}");
        assert!(debug_str.contains("RequestData"));
        assert!(debug_str.contains("/api/test"));
    }

    #[test]
    fn test_request_data_clone_independence() {
        let mut path_params1 = HashMap::new();
        path_params1.insert("id".to_string(), "original".to_string());

        let data1 = create_request_data(
            "/api/users/1",
            "GET",
            json!({"name": "original"}),
            json!({}),
            RequestDataCollections {
                path_params: path_params1,
                ..Default::default()
            },
        );

        let data2 = data1.clone();

        assert_eq!(data1.path_params.get("id"), data2.path_params.get("id"));
        assert_eq!(data1.body["name"], data2.body["name"]);
    }

    #[test]
    fn deserialize_errors_on_missing_required_field() {
        let value = json!({
            "path_params": {},
            "query_params": {},
            "raw_query_params": {},
            "body": null,
            "raw_body": null,
            "headers": {},
            "cookies": {},
            "method": "GET"
        });

        let err = serde_json::from_value::<RequestData>(value).unwrap_err();
        assert!(err.to_string().contains("missing field"));
    }

    #[test]
    fn deserialize_errors_on_invalid_raw_body_type() {
        let value = json!({
            "path_params": {},
            "query_params": {},
            "raw_query_params": {},
            "body": null,
            "raw_body": "not-bytes",
            "headers": {},
            "cookies": {},
            "method": "GET",
            "path": "/"
        });

        assert!(serde_json::from_value::<RequestData>(value).is_err());
    }
}
