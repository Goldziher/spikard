use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::{Router, routing::any};
use spikard_http::testing::{MultipartFilePart, TestClient};

async fn echo(req: Request<Body>) -> axum::response::Response {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let headers = req
        .headers()
        .iter()
        .fold(serde_json::Map::new(), |mut map, (key, value)| {
            map.insert(
                key.to_string(),
                serde_json::Value::String(value.to_str().unwrap_or("").to_string()),
            );
            map
        });
    let bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8_lossy(&bytes).to_string();

    let payload = serde_json::json!({
        "method": method,
        "uri": uri,
        "headers": headers,
        "body": body_text,
    });

    (StatusCode::OK, axum::Json(payload)).into_response()
}

#[tokio::test]
async fn test_client_sends_query_headers_and_bodies() {
    let app = Router::new().route("/{*path}", any(echo));
    let client = TestClient::from_router(app).expect("client");

    let snapshot = client
        .get(
            "/items",
            Some(vec![("q".to_string(), "a b".to_string())]),
            Some(vec![("x-test".to_string(), "1".to_string())]),
        )
        .await
        .expect("get");
    assert_eq!(snapshot.status, 200);
    let json = snapshot.json().expect("json");
    assert_eq!(json["method"], "GET");
    assert!(json["uri"].as_str().unwrap().contains("/items?q=a%20b"));
    assert_eq!(json["headers"]["x-test"], "1");

    let snapshot = client
        .post(
            "/json",
            Some(serde_json::json!({"hello":"world"})),
            None,
            None,
            None,
            None,
        )
        .await
        .expect("post");
    let json = snapshot.json().expect("json");
    assert_eq!(json["method"], "POST");
    assert!(json["body"].as_str().unwrap().contains("\"hello\":\"world\""));

    let snapshot = client
        .post(
            "/form",
            None,
            Some(vec![("a".to_string(), "b".to_string())]),
            None,
            None,
            None,
        )
        .await
        .expect("post");
    let json = snapshot.json().expect("json");
    let body = json["body"].as_str().unwrap();
    assert!(body.contains('a'));
    assert!(body.contains('b'));

    let snapshot = client
        .post(
            "/multipart",
            None,
            None,
            Some((
                vec![("field".to_string(), "value".to_string())],
                vec![MultipartFilePart {
                    field_name: "file".to_string(),
                    filename: "hello.txt".to_string(),
                    content_type: Some("text/plain".to_string()),
                    content: b"hello".to_vec(),
                }],
            )),
            None,
            None,
        )
        .await
        .expect("post");
    let json = snapshot.json().expect("json");
    assert!(
        json["headers"]["content-type"]
            .as_str()
            .unwrap()
            .contains("multipart/form-data")
    );
    assert!(json["body"].as_str().unwrap().contains("hello"));
}

#[tokio::test]
async fn test_client_supports_other_http_methods_and_query_merging() {
    let app = Router::new().route("/{*path}", any(echo));
    let client = TestClient::from_router(app).expect("client");

    let snapshot = client
        .put(
            "/put",
            Some(serde_json::json!({"name":"spikard"})),
            None,
            Some(vec![("x-test".to_string(), "2".to_string())]),
        )
        .await
        .expect("put");
    let json = snapshot.json().expect("json");
    assert_eq!(json["method"], "PUT");
    assert_eq!(json["headers"]["x-test"], "2");
    assert!(json["body"].as_str().unwrap().contains("\"name\":\"spikard\""));

    let snapshot = client.delete("/delete", None, None).await.expect("delete");
    assert_eq!(snapshot.json().expect("json")["method"], "DELETE");

    let snapshot = client.options("/options", None, None).await.expect("options");
    assert_eq!(snapshot.json().expect("json")["method"], "OPTIONS");

    let snapshot = client.head("/head", None, None).await.expect("head");
    assert_eq!(snapshot.status, 200);
    assert!(snapshot.body.is_empty());

    let snapshot = client.trace("/trace", None, None).await.expect("trace");
    assert_eq!(snapshot.json().expect("json")["method"], "TRACE");

    let snapshot = client
        .get("/items?x=1", Some(vec![("y".to_string(), "2".to_string())]), None)
        .await
        .expect("get");
    assert!(
        snapshot.json().expect("json")["uri"]
            .as_str()
            .unwrap()
            .contains("x=1&y=2")
    );
}

#[tokio::test]
async fn test_client_rejects_invalid_header_names() {
    let app = Router::new().route("/{*path}", any(echo));
    let client = TestClient::from_router(app).expect("client");

    let error = client
        .get("/items", None, Some(vec![("bad\n".to_string(), "1".to_string())]))
        .await
        .expect_err("invalid header");
    let message = error.to_string();
    assert!(message.contains("Invalid header name"));
}
