//! Request parsing and data extraction utilities
//!
//! Performance optimizations in this module:
//! - Static singletons for empty collections avoid repeated allocations
//! - HashMap::with_capacity pre-allocates based on expected sizes
//! - Arc wrapping enables cheap cloning of RequestData

use crate::handler_trait::RequestData;
use crate::query_parser::{parse_query_pairs_to_json, parse_query_string};
use axum::body::Body;
use http_body_util::BodyExt;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

// Performance: Static singletons for empty collections.
// These avoid allocating new empty HashMaps/Values for every request that doesn't
// need them (e.g., requests without query params, headers disabled, etc.).

/// Static empty JSON object value, shared across all requests without query params.
fn empty_json_object() -> Arc<Value> {
    static EMPTY: OnceLock<Arc<Value>> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Arc::new(Value::Object(serde_json::Map::new()))))
}

/// Static null JSON value for requests without bodies.
fn null_json_value() -> Arc<Value> {
    static NULL: OnceLock<Arc<Value>> = OnceLock::new();
    Arc::clone(NULL.get_or_init(|| Arc::new(Value::Null)))
}

/// Static empty path params map.
fn empty_path_params() -> Arc<HashMap<String, String>> {
    static EMPTY: OnceLock<Arc<HashMap<String, String>>> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Arc::new(HashMap::new())))
}

#[derive(Debug, Clone, Copy)]
pub struct WithoutBodyExtractionOptions {
    pub include_raw_query_params: bool,
    pub include_query_params_json: bool,
    pub include_headers: bool,
    pub include_cookies: bool,
}

fn extract_query_params_and_raw(
    uri: &axum::http::Uri,
    include_raw_query_params: bool,
    include_query_params_json: bool,
) -> (Value, HashMap<String, Vec<String>>) {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        // Performance: Return empty object for empty query string.
        return (Value::Object(serde_json::Map::new()), HashMap::new());
    }

    match (include_raw_query_params, include_query_params_json) {
        (false, false) => (Value::Null, HashMap::new()),
        (false, true) => (
            crate::query_parser::parse_query_string_to_json(query_string.as_bytes(), true),
            HashMap::new(),
        ),
        (true, false) => {
            let pairs = parse_query_string(query_string.as_bytes(), '&');
            // Performance: Pre-allocate HashMap with estimated unique key count.
            // In practice, most keys are unique, so pairs.len() is a good estimate.
            let mut raw = HashMap::with_capacity(pairs.len());
            for (k, v) in pairs {
                raw.entry(k).or_insert_with(Vec::new).push(v);
            }
            (Value::Null, raw)
        }
        (true, true) => {
            let pairs = parse_query_string(query_string.as_bytes(), '&');
            let json = parse_query_pairs_to_json(&pairs, true);
            // Performance: Pre-allocate HashMap with estimated unique key count.
            let mut raw = HashMap::with_capacity(pairs.len());
            for (k, v) in pairs {
                raw.entry(k).or_insert_with(Vec::new).push(v);
            }
            (json, raw)
        }
    }
}

/// Extract and parse query parameters from request URI
pub fn extract_query_params(uri: &axum::http::Uri) -> Value {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        crate::query_parser::parse_query_string_to_json(query_string.as_bytes(), true)
    }
}

/// Extract raw query parameters as strings (no type conversion)
/// Used for validation error messages to show the actual input values
///
/// Performance: Pre-allocates HashMap based on parsed pair count.
pub fn extract_raw_query_params(uri: &axum::http::Uri) -> HashMap<String, Vec<String>> {
    let query_string = uri.query().unwrap_or("");
    if query_string.is_empty() {
        return HashMap::new();
    }

    let pairs = parse_query_string(query_string.as_bytes(), '&');
    // Performance: Pre-allocate with estimated unique key count.
    let mut map = HashMap::with_capacity(pairs.len());
    for (k, v) in pairs {
        map.entry(k).or_insert_with(Vec::new).push(v);
    }
    map
}

/// Extract headers from request
pub fn extract_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(headers.len());
    for (name, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            // `HeaderName::as_str()` is already normalized to lowercase.
            map.insert(name.as_str().to_string(), val_str.to_string());
        }
    }
    map
}

fn extract_content_type_header(headers: &axum::http::HeaderMap) -> Arc<HashMap<String, String>> {
    let Some(value) = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
    else {
        return empty_string_map();
    };

    let mut map = HashMap::with_capacity(1);
    map.insert("content-type".to_string(), value.to_string());
    Arc::new(map)
}

/// Extract cookies from request headers
///
/// Performance: Pre-allocates HashMap capacity based on estimated cookie count
/// by counting semicolons in the cookie string (each cookie separated by "; ").
pub fn extract_cookies(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
    let Some(cookie_str) = headers.get(axum::http::header::COOKIE).and_then(|h| h.to_str().ok()) else {
        return HashMap::new();
    };

    // Performance: Estimate cookie count by counting semicolons + 1.
    // Typical cookie string: "session=abc; user=123; theme=dark"
    let estimated_count = cookie_str.bytes().filter(|&b| b == b';').count() + 1;
    let mut cookies = HashMap::with_capacity(estimated_count);

    for cookie in cookie::Cookie::split_parse(cookie_str).flatten() {
        cookies.insert(cookie.name().to_string(), cookie.value().to_string());
    }

    cookies
}

fn empty_string_map() -> Arc<HashMap<String, String>> {
    static EMPTY: OnceLock<Arc<HashMap<String, String>>> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Arc::new(HashMap::new())))
}

fn empty_raw_query_map() -> Arc<HashMap<String, Vec<String>>> {
    static EMPTY: OnceLock<Arc<HashMap<String, Vec<String>>>> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Arc::new(HashMap::new())))
}

/// Create RequestData from request parts (for requests without body)
///
/// Wraps HashMaps in Arc to enable cheap cloning without duplicating data.
///
/// Performance optimizations:
/// - Uses static singletons for empty path params, body, headers, cookies
/// - Only allocates when data is present
pub fn create_request_data_without_body(
    uri: &axum::http::Uri,
    method: &axum::http::Method,
    headers: &axum::http::HeaderMap,
    path_params: HashMap<String, String>,
    options: WithoutBodyExtractionOptions,
) -> RequestData {
    let (query_params, raw_query_params) =
        extract_query_params_and_raw(uri, options.include_raw_query_params, options.include_query_params_json);

    // Performance: Use static singleton for empty path params (common for routes without params).
    let path_params_arc = if path_params.is_empty() {
        empty_path_params()
    } else {
        Arc::new(path_params)
    };

    // Performance: Use static singleton for empty query params.
    let query_params_arc = if matches!(query_params, Value::Object(ref m) if m.is_empty()) {
        empty_json_object()
    } else if matches!(query_params, Value::Null) {
        null_json_value()
    } else {
        Arc::new(query_params)
    };

    RequestData {
        path_params: path_params_arc,
        query_params: query_params_arc,
        raw_query_params: if raw_query_params.is_empty() {
            empty_raw_query_map()
        } else {
            Arc::new(raw_query_params)
        },
        validated_params: None,
        headers: if options.include_headers {
            Arc::new(extract_headers(headers))
        } else {
            empty_string_map()
        },
        cookies: if options.include_cookies {
            Arc::new(extract_cookies(headers))
        } else {
            empty_string_map()
        },
        // Performance: Use static null value singleton.
        body: null_json_value(),
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
///
/// Performance optimizations:
/// - Stores raw body bytes without parsing JSON (deferred parsing)
/// - Uses static singletons for empty collections
/// - Pre-read body bytes are reused if available in extensions
pub async fn create_request_data_with_body(
    parts: &axum::http::request::Parts,
    path_params: HashMap<String, String>,
    body: Body,
    include_raw_query_params: bool,
    include_query_params_json: bool,
    include_headers: bool,
    include_cookies: bool,
) -> Result<RequestData, (axum::http::StatusCode, String)> {
    let body_bytes = if let Some(pre_read) = parts.extensions.get::<crate::middleware::PreReadBody>() {
        pre_read.0.clone()
    } else {
        body.collect()
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("Failed to read body: {}", e),
                )
            })?
            .to_bytes()
    };

    let (query_params, raw_query_params) =
        extract_query_params_and_raw(&parts.uri, include_raw_query_params, include_query_params_json);

    // Performance: Use static singleton for empty path params.
    let path_params_arc = if path_params.is_empty() {
        empty_path_params()
    } else {
        Arc::new(path_params)
    };

    // Performance: Use static singleton for empty/null query params.
    let query_params_arc = if matches!(query_params, Value::Object(ref m) if m.is_empty()) {
        empty_json_object()
    } else if matches!(query_params, Value::Null) {
        null_json_value()
    } else {
        Arc::new(query_params)
    };

    // Performance: Reuse pre-parsed JSON if available, otherwise use static null.
    let body_arc = if let Some(parsed) = parts.extensions.get::<crate::middleware::PreParsedJson>() {
        Arc::new(parsed.0.clone())
    } else {
        null_json_value()
    };

    Ok(RequestData {
        path_params: path_params_arc,
        query_params: query_params_arc,
        raw_query_params: if raw_query_params.is_empty() {
            empty_raw_query_map()
        } else {
            Arc::new(raw_query_params)
        },
        validated_params: None,
        headers: if include_headers {
            Arc::new(extract_headers(&parts.headers))
        } else {
            extract_content_type_header(&parts.headers)
        },
        cookies: if include_cookies {
            Arc::new(extract_cookies(&parts.headers))
        } else {
            empty_string_map()
        },
        body: body_arc,
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
    use futures_util::stream;
    use serde_json::json;

    const OPTIONS_ALL: WithoutBodyExtractionOptions = WithoutBodyExtractionOptions {
        include_raw_query_params: true,
        include_query_params_json: true,
        include_headers: true,
        include_cookies: true,
    };

    const OPTIONS_SKIP_HEADERS: WithoutBodyExtractionOptions = WithoutBodyExtractionOptions {
        include_raw_query_params: true,
        include_query_params_json: true,
        include_headers: false,
        include_cookies: true,
    };

    const OPTIONS_SKIP_COOKIES: WithoutBodyExtractionOptions = WithoutBodyExtractionOptions {
        include_raw_query_params: true,
        include_query_params_json: true,
        include_headers: true,
        include_cookies: false,
    };

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

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_ALL);

        assert_eq!(result.method, "GET");
        assert_eq!(result.path, "/test");
        assert!(result.path_params.is_empty());
        assert_eq!(*result.query_params, json!({}));
        assert!(result.raw_query_params.is_empty());
        assert!(result.headers.is_empty());
        assert!(result.cookies.is_empty());
        assert_eq!(*result.body, Value::Null);
        assert!(result.raw_body.is_none());
    }

    #[test]
    fn test_create_request_data_without_body_with_path_params() {
        let uri: Uri = "/users/42".parse().unwrap();
        let method = Method::GET;
        let headers = HeaderMap::new();
        let mut path_params = HashMap::new();
        path_params.insert("user_id".to_string(), "42".to_string());

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_ALL);

        assert_eq!(result.path_params.get("user_id"), Some(&"42".to_string()));
    }

    #[test]
    fn test_create_request_data_without_body_with_query_params() {
        let uri: Uri = "/search?q=rust&limit=10".parse().unwrap();
        let method = Method::GET;
        let headers = HeaderMap::new();
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_ALL);

        assert_eq!(*result.query_params, json!({"q": "rust", "limit": 10}));
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

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_ALL);

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

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_ALL);

        assert_eq!(result.cookies.get("session"), Some(&"xyz".to_string()));
        assert_eq!(result.cookies.get("theme"), Some(&"dark".to_string()));
    }

    #[test]
    fn test_create_request_data_without_body_skip_headers() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer token"));
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_SKIP_HEADERS);

        assert!(result.headers.is_empty());
    }

    #[test]
    fn test_create_request_data_without_body_skip_cookies() {
        let uri: Uri = "/test".parse().unwrap();
        let method = Method::GET;
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::COOKIE, HeaderValue::from_static("session=xyz"));
        let path_params = HashMap::new();

        let result = create_request_data_without_body(&uri, &method, &headers, path_params, OPTIONS_SKIP_COOKIES);

        assert!(result.cookies.is_empty());
    }

    #[test]
    fn test_create_request_data_without_body_different_methods() {
        let uri: Uri = "/resource".parse().unwrap();
        let headers = HeaderMap::new();
        let path_params = HashMap::new();

        for method in &[Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH] {
            let result = create_request_data_without_body(&uri, method, &headers, path_params.clone(), OPTIONS_ALL);
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

        let result = create_request_data_with_body(&parts.0, path_params, body, true, true, true, true)
            .await
            .unwrap();

        assert_eq!(result.method, "POST");
        assert_eq!(result.path, "/test");
        assert_eq!(*result.body, Value::Null);
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
            .await
            .unwrap();

        assert_eq!(result.method, "POST");
        assert_eq!(*result.body, Value::Null);
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
            .await
            .unwrap();

        assert_eq!(*result.query_params, json!({"foo": "bar", "baz": "qux"}));
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
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

        let result = create_request_data_with_body(&parts, path_params, request_body, true, true, true, true)
            .await
            .unwrap();

        assert_eq!(result.method, "PUT");
        assert_eq!(result.path, "/users/42");
        assert_eq!(result.path_params.get("user_id"), Some(&"42".to_string()));
        assert_eq!(*result.query_params, json!({"action": "update"}));
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

        let request_data = create_request_data_without_body(&uri, &method, &headers, path_params.clone(), OPTIONS_ALL);

        let cloned = request_data.clone();

        assert!(Arc::ptr_eq(&request_data.path_params, &cloned.path_params));
        assert!(Arc::ptr_eq(&request_data.headers, &cloned.headers));
        assert!(Arc::ptr_eq(&request_data.cookies, &cloned.cookies));
        assert!(Arc::ptr_eq(&request_data.raw_query_params, &cloned.raw_query_params));
    }

    #[tokio::test]
    async fn create_request_data_with_body_returns_bad_request_when_body_stream_errors() {
        let request = axum::http::Request::builder()
            .method(Method::POST)
            .uri("/path")
            .body(Body::empty())
            .unwrap();
        let (parts, _) = request.into_parts();

        let stream = stream::once(async move {
            Err::<bytes::Bytes, std::io::Error>(std::io::Error::new(std::io::ErrorKind::Other, "boom"))
        });
        let body = Body::from_stream(stream);

        let err = create_request_data_with_body(&parts, HashMap::new(), body, true, true, true, true)
            .await
            .unwrap_err();
        assert_eq!(err.0, axum::http::StatusCode::BAD_REQUEST);
        assert!(err.1.contains("Failed to read body:"));
    }
}
