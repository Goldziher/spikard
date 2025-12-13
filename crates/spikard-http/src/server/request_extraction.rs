//! Request parsing and data extraction utilities

use crate::handler_trait::RequestData;
use crate::query_parser::parse_query_string_to_json;
use axum::body::Body;
use http_body_util::BodyExt;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Extract and parse query parameters from request URI
pub fn extract_query_params(uri: &axum::http::Uri) -> Value {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        parse_query_string_to_json(query_string.as_bytes(), true)
    }
}

/// Extract raw query parameters as strings (no type conversion)
/// Used for validation error messages to show the actual input values
pub fn extract_raw_query_params(uri: &axum::http::Uri) -> HashMap<String, Vec<String>> {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        HashMap::new()
    } else {
        crate::query_parser::parse_query_string(query_string.as_bytes(), '&')
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(k).or_insert_with(Vec::new).push(v);
                acc
            })
    }
}

/// Extract headers from request
pub fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            map.insert(name.as_str().to_lowercase(), val_str.to_string());
        }
    }
    map
}

/// Extract cookies from request headers
pub fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    if let Some(cookie_str) = headers.get(axum::http::header::COOKIE).and_then(|h| h.to_str().ok()) {
        for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());
        }
    }

    cookies
}

/// Create RequestData from request parts (for requests without body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
pub fn create_request_data_without_body(
    uri: &axum::http::Uri,
    method: &axum::http::Method,
    headers: &axum::http::HeaderMap,
    path_params: HashMap<String, String>,
) -> RequestData {
    RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(uri),
        raw_query_params: Arc::new(extract_raw_query_params(uri)),
        headers: Arc::new(extract_headers(headers)),
        cookies: Arc::new(extract_cookies(headers)),
        body: Value::Null,
        raw_body: None,
        method: method.as_str().to_string(),
        path: uri.path().to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    }
}

/// Create RequestData from request parts (for requests with body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
/// Performance optimization: stores raw body bytes without parsing JSON.
/// JSON parsing is deferred until actually needed (e.g., for validation).
pub async fn create_request_data_with_body(
    parts: &axum::http::request::Parts,
    path_params: HashMap<String, String>,
    body: Body,
) -> Result<RequestData, (axum::http::StatusCode, String)> {
    let body_bytes = body
        .collect()
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
        })?
        .to_bytes();

    Ok(RequestData {
        path_params: Arc::new(path_params),
        query_params: extract_query_params(&parts.uri),
        raw_query_params: Arc::new(extract_raw_query_params(&parts.uri)),
        headers: Arc::new(extract_headers(&parts.headers)),
        cookies: Arc::new(extract_cookies(&parts.headers)),
        body: Value::Null,
        raw_body: if body_bytes.is_empty() { None } else { Some(body_bytes) },
        method: parts.method.as_str().to_string(),
        path: parts.uri.path().to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, HeaderValue, Method, Uri};
    use serde_json::json;

    #[test]
    fn test_extract_query_params_empty() {
        let uri: Uri = "/path".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({}));
    }

    #[test]
    fn test_extract_query_params_single_param() {
        let uri: Uri = "/path?name=value".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"name": "value"}));
    }

    #[test]
    fn test_extract_query_params_multiple_params() {
        let uri: Uri = "/path?foo=1&bar=2".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"foo": 1, "bar": 2}));
    }

    #[test]
    fn test_extract_query_params_array_params() {
        let uri: Uri = "/path?tags=rust&tags=http".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"tags": ["rust", "http"]}));
    }

    #[test]
    fn test_extract_query_params_mixed_array_and_single() {
        let uri: Uri = "/path?tags=rust&tags=web&id=123".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"tags": ["rust", "web"], "id": 123}));
    }

    #[test]
    fn test_extract_query_params_url_encoded() {
        let uri: Uri = "/path?email=test%40example.com&name=john+doe".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"email": "test@example.com", "name": "john doe"}));
    }

    #[test]
    fn test_extract_query_params_boolean_values() {
        let uri: Uri = "/path?active=true&enabled=false".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"active": true, "enabled": false}));
    }

    #[test]
    fn test_extract_query_params_null_value() {
        let uri: Uri = "/path?value=null".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"value": null}));
    }

    #[test]
    fn test_extract_query_params_empty_string_value() {
        let uri: Uri = "/path?key=".parse().unwrap();
        let result = extract_query_params(&uri);
        assert_eq!(result, json!({"key": ""}));
    }

    #[test]
    fn test_extract_raw_query_params_empty() {
        let uri: Uri = "/path".parse().unwrap();
        let result = extract_raw_query_params(&uri);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_raw_query_params_single() {
        let uri: Uri = "/path?name=value".parse().unwrap();
        let result = extract_raw_query_params(&uri);
        assert_eq!(result.get("name"), Some(&vec!["value".to_string()]));
    }

    #[test]
    fn test_extract_raw_query_params_multiple_values() {
        let uri: Uri = "/path?tag=rust&tag=http".parse().unwrap();
        let result = extract_raw_query_params(&uri);
        assert_eq!(result.get("tag"), Some(&vec!["rust".to_string(), "http".to_string()]));
    }

    #[test]
    fn test_extract_raw_query_params_mixed() {
        let uri: Uri = "/path?id=123&tags=rust&tags=web&active=true".parse().unwrap();
        let result = extract_raw_query_params(&uri);
        assert_eq!(result.get("id"), Some(&vec!["123".to_string()]));
        assert_eq!(result.get("tags"), Some(&vec!["rust".to_string(), "web".to_string()]));
        assert_eq!(result.get("active"), Some(&vec!["true".to_string()]));
    }

    #[test]
    fn test_extract_raw_query_params_url_encoded() {
        let uri: Uri = "/path?email=test%40example.com".parse().unwrap();
        let result = extract_raw_query_params(&uri);
        assert_eq!(result.get("email"), Some(&vec!["test@example.com".to_string()]));
    }

    #[test]
    fn test_extract_headers_empty() {
        let headers = HeaderMap::new();
        let result = extract_headers(&headers);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_headers_single() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        let result = extract_headers(&headers);
        assert_eq!(result.get("content-type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_extract_headers_multiple() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("user-agent", HeaderValue::from_static("test-agent"));
        headers.insert("authorization", HeaderValue::from_static("Bearer token123"));

        let result = extract_headers(&headers);
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(result.get("user-agent"), Some(&"test-agent".to_string()));
        assert_eq!(result.get("authorization"), Some(&"Bearer token123".to_string()));
    }

    #[test]
    fn test_extract_headers_case_insensitive() {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("text/html"));
        headers.insert("USER-Agent", HeaderValue::from_static("chrome"));

        let result = extract_headers(&headers);
        assert_eq!(result.get("content-type"), Some(&"text/html".to_string()));
        assert_eq!(result.get("user-agent"), Some(&"chrome".to_string()));
    }

    #[test]
    fn test_extract_headers_with_dashes() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom-header", HeaderValue::from_static("custom-value"));
        headers.insert("x-request-id", HeaderValue::from_static("req-12345"));

        let result = extract_headers(&headers);
        assert_eq!(result.get("x-custom-header"), Some(&"custom-value".to_string()));
        assert_eq!(result.get("x-request-id"), Some(&"req-12345".to_string()));
    }

    #[test]
    fn test_extract_cookies_no_cookie_header() {
        let headers = HeaderMap::new();
        let result = extract_cookies(&headers);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_cookies_single() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::COOKIE, HeaderValue::from_static("session=abc123"));

        let result = extract_cookies(&headers);
        assert_eq!(result.get("session"), Some(&"abc123".to_string()));
    }

    #[test]
    fn test_extract_cookies_multiple() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::COOKIE,
            HeaderValue::from_static("session=abc123; user_id=42; theme=dark"),
        );

        let result = extract_cookies(&headers);
        assert_eq!(result.len(), 3);
        assert_eq!(result.get("session"), Some(&"abc123".to_string()));
        assert_eq!(result.get("user_id"), Some(&"42".to_string()));
        assert_eq!(result.get("theme"), Some(&"dark".to_string()));
    }

    #[test]
    fn test_extract_cookies_with_spaces() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::COOKIE,
            HeaderValue::from_static("session = abc123 ; theme = light"),
        );

        let result = extract_cookies(&headers);
        assert!(result.len() >= 1);
    }

    #[test]
    fn test_extract_cookies_empty_value() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::COOKIE, HeaderValue::from_static("empty="));

        let result = extract_cookies(&headers);
        assert_eq!(result.get("empty"), Some(&String::new()));
    }

    #[test]
    fn test_create_request_data_without_body_minimal() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::GET;
        let headers = HeaderMap::new();
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params);

        assert_eq!(result.method, "GET");
        assert_eq!(result.path, "/test");
        assert!(result.path_params.is_empty());
        assert_eq!(result.query_params, json!({}));
        assert!(result.raw_query_params.is_empty());
        assert!(result.headers.is_empty());
        assert!(result.cookies.is_empty());
        assert_eq!(result.body, Value::Null);
        assert!(result.raw_body.is_none());
    }

    #[test]
    fn test_create_request_data_without_body_with_path_params() {
        let uri: Uri = "/users/42".parse().unwrap();
        let method = Method::GET;
        let headers = HeaderMap::new();
        let mut path_params = HashMap::new();
        path_params.insert("user_id".to_string(), "42".to_string());

        let result = create_request_data_without_body(&uri, &method, &headers, path_params);

        assert_eq!(result.path_params.get("user_id"), Some(&"42".to_string()));
    }

    #[test]
    fn test_create_request_data_without_body_with_query_params() {
        let uri: Uri = "/search?q=rust&limit=10".parse().unwrap();
        let method = Method::GET;
        let headers = HeaderMap::new();
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params);

        assert_eq!(result.query_params, json!({"q": "rust", "limit": 10}));
        assert_eq!(result.raw_query_params.get("q"), Some(&vec!["rust".to_string()]));
        assert_eq!(result.raw_query_params.get("limit"), Some(&vec!["10".to_string()]));
    }

    #[test]
    fn test_create_request_data_without_body_with_headers() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::POST;
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("authorization", HeaderValue::from_static("Bearer token"));
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params);

        assert_eq!(
            result.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(result.headers.get("authorization"), Some(&"Bearer token".to_string()));
    }

    #[test]
    fn test_create_request_data_without_body_with_cookies() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::COOKIE,
            HeaderValue::from_static("session=xyz; theme=dark"),
        );
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params);

        assert_eq!(result.cookies.get("session"), Some(&"xyz".to_string()));
        assert_eq!(result.cookies.get("theme"), Some(&"dark".to_string()));
    }

    #[test]
    fn test_create_request_data_without_body_different_methods() {
        let uri: Uri = "/resource".parse().unwrap();
        let headers = HeaderMap::new();
        let path_params = HashMap::new();

        for method in &[Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH] {
            let result = create_request_data_without_body(&uri, method, &headers, path_params.clone());
            assert_eq!(result.method, method.as_str());
        }
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_empty() {
        let parts = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test")
            .body(Body::empty())
            .unwrap()
            .into_parts();

        let body = Body::empty();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts.0, path_params, body)
            .await
            .unwrap();

        assert_eq!(result.method, "POST");
        assert_eq!(result.path, "/test");
        assert_eq!(result.body, Value::Null);
        assert!(result.raw_body.is_none());
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_json() {
        let request_body = Body::from(r#"{"key":"value"}"#);
        let request = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert_eq!(result.method, "POST");
        assert_eq!(result.body, Value::Null);
        assert!(result.raw_body.is_some());
        assert_eq!(result.raw_body.as_ref().unwrap().as_ref(), br#"{"key":"value"}"#);
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_with_query_params() {
        let request_body = Body::from("test");
        let request = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test?foo=bar&baz=qux")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert_eq!(result.query_params, json!({"foo": "bar", "baz": "qux"}));
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_with_headers() {
        let request_body = Body::from("test");
        let request = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test")
            .header("content-type", "application/json")
            .header("x-request-id", "req123")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert_eq!(
            result.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(result.headers.get("x-request-id"), Some(&"req123".to_string()));
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_with_cookies() {
        let request_body = Body::from("test");
        let request = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test")
            .header("cookie", "session=xyz; user=123")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert_eq!(result.cookies.get("session"), Some(&"xyz".to_string()));
        assert_eq!(result.cookies.get("user"), Some(&"123".to_string()));
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_large_payload() {
        let large_json = json!({
            "data": (0..100).map(|i| json!({"id": i, "value": format!("item-{}", i)})).collect::<Vec<_>>()
        });
        let json_str = serde_json::to_string(&large_json).unwrap();
        let request_body = Body::from(json_str.clone());

        let request = axum::http::request::Request::builder()
            .method(Method::POST)
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let path_params = HashMap::new();

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert!(result.raw_body.is_some());
        assert_eq!(result.raw_body.as_ref().unwrap().as_ref(), json_str.as_bytes());
    }

    #[tokio::test]
    async fn test_create_request_data_with_body_preserves_all_fields() {
        let request_body = Body::from("request data");
        let request = axum::http::request::Request::builder()
            .method(Method::PUT)
            .uri("/users/42?action=update")
            .header("authorization", "Bearer token")
            .header("cookie", "session=abc")
            .body(Body::empty())
            .unwrap();

        let (parts, _) = request.into_parts();
        let mut path_params = HashMap::new();
        path_params.insert("user_id".to_string(), "42".to_string());

        let result = create_request_data_with_body(&parts, path_params, request_body)
            .await
            .unwrap();

        assert_eq!(result.method, "PUT");
        assert_eq!(result.path, "/users/42");
        assert_eq!(result.path_params.get("user_id"), Some(&"42".to_string()));
        assert_eq!(result.query_params, json!({"action": "update"}));
        assert!(result.headers.contains_key("authorization"));
        assert!(result.cookies.contains_key("session"));
        assert!(result.raw_body.is_some());
    }

    #[test]
    fn test_arc_wrapping_for_cheap_cloning() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::COOKIE, HeaderValue::from_static("session=abc"));
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "1".to_string());

        let request_data = create_request_data_without_body(&uri, &method, &headers, path_params.clone());

        let cloned = request_data.clone();

        assert!(Arc::ptr_eq(&request_data.path_params, &cloned.path_params));
        assert!(Arc::ptr_eq(&request_data.headers, &cloned.headers));
        assert!(Arc::ptr_eq(&request_data.cookies, &cloned.cookies));
        assert!(Arc::ptr_eq(&request_data.raw_query_params, &cloned.raw_query_params));
    }
}
