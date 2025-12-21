use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use spikard_core::request_data::RequestData;

fn make_request_data() -> RequestData {
    RequestData {
        path_params: Arc::new(HashMap::from([("id".to_string(), "42".to_string())])),
        query_params: json!({"page": 3, "filter": "active"}),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::from([
            ("page".to_string(), vec!["3".to_string()]),
            ("filter".to_string(), vec!["active".to_string()]),
        ])),
        body: json!({"name": "spikard", "active": true}),
        #[cfg(feature = "di")]
        raw_body: Some(bytes::Bytes::from_static(b"{\"name\":\"spikard\",\"active\":true}")),
        #[cfg(not(feature = "di"))]
        raw_body: Some(b"{\"name\":\"spikard\",\"active\":true}".to_vec()),
        headers: Arc::new(HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("x-request-id".to_string(), "req-123".to_string()),
        ])),
        cookies: Arc::new(HashMap::from([("session".to_string(), "abc123".to_string())])),
        method: "POST".to_string(),
        path: "/widgets/42".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    }
}

#[test]
fn request_data_serialization_roundtrip() {
    let original = make_request_data();

    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: RequestData = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(original.path_params.as_ref(), decoded.path_params.as_ref());
    assert_eq!(original.query_params, decoded.query_params);
    assert_eq!(original.raw_query_params.as_ref(), decoded.raw_query_params.as_ref());
    assert_eq!(original.body, decoded.body);
    assert_eq!(original.raw_body, decoded.raw_body);
    assert_eq!(original.headers.as_ref(), decoded.headers.as_ref());
    assert_eq!(original.cookies.as_ref(), decoded.cookies.as_ref());
    assert_eq!(original.method, decoded.method);
    assert_eq!(original.path, decoded.path);
}

#[test]
fn request_data_clone_shares_arc_backing() {
    let request = make_request_data();

    let initial_path_refs = Arc::strong_count(&request.path_params);
    let initial_headers_refs = Arc::strong_count(&request.headers);
    let initial_cookies_refs = Arc::strong_count(&request.cookies);
    let initial_query_refs = Arc::strong_count(&request.raw_query_params);

    let clone = request.clone();

    assert_eq!(Arc::strong_count(&request.path_params), initial_path_refs + 1);
    assert_eq!(Arc::strong_count(&clone.path_params), initial_path_refs + 1);
    assert_eq!(Arc::strong_count(&request.headers), initial_headers_refs + 1);
    assert_eq!(Arc::strong_count(&request.cookies), initial_cookies_refs + 1);
    assert_eq!(Arc::strong_count(&request.raw_query_params), initial_query_refs + 1);
}
