use axum::http::HeaderValue;
use axum::{Router, response::IntoResponse, routing::get};
use brotli::CompressorWriter;
use flate2::Compression;
use flate2::write::GzEncoder;
use spikard_http::testing::{MultipartFilePart, build_multipart_body, encode_urlencoded_body, snapshot_response};
use std::io::Write;

#[test]
fn urlencoded_encoding_handles_scalars_and_objects() {
    let s = serde_json::Value::String("a=b&c=d".to_string());
    assert_eq!(encode_urlencoded_body(&s).unwrap(), b"a=b&c=d".to_vec());

    let mut obj = serde_json::Map::new();
    obj.insert("name".to_string(), serde_json::Value::String("Alice".to_string()));
    obj.insert("tags".to_string(), serde_json::json!(["a", "b"]));
    let value = serde_json::Value::Object(obj);
    let encoded = String::from_utf8(encode_urlencoded_body(&value).unwrap()).unwrap();
    assert!(encoded.contains("name=Alice"));
    assert!(encoded.contains("tags"));
}

#[test]
fn multipart_body_contains_fields_and_files() {
    let (body, boundary) = build_multipart_body(
        &[("field".to_string(), "value".to_string())],
        &[MultipartFilePart {
            field_name: "file".to_string(),
            filename: "hello.txt".to_string(),
            content_type: Some("text/plain".to_string()),
            content: b"hello".to_vec(),
        }],
    );

    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains(&format!("--{boundary}")));
    assert!(body_str.contains("name=\"field\""));
    assert!(body_str.contains("value"));
    assert!(body_str.contains("name=\"file\"; filename=\"hello.txt\""));
    assert!(body_str.contains("Content-Type: text/plain"));
    assert!(body_str.contains("hello"));
}

#[tokio::test]
async fn snapshot_response_decodes_gzip_body() {
    let app = Router::new().route(
        "/gzip",
        get(|| async move {
            let raw = b"hello gzip".to_vec();
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&raw).unwrap();
            let compressed = encoder.finish().unwrap();

            ([("content-encoding", HeaderValue::from_static("gzip"))], compressed).into_response()
        }),
    );

    let server = axum_test::TestServer::new(app).unwrap();
    let response = server.get("/gzip").await;

    let snapshot = snapshot_response(response).await.expect("snapshot failed");
    assert_eq!(snapshot.status, 200);
    assert_eq!(snapshot.text().unwrap(), "hello gzip");
}

#[tokio::test]
async fn snapshot_response_decodes_brotli_body() {
    let app = Router::new().route(
        "/br",
        get(|| async move {
            let raw = b"hello br".to_vec();
            let mut writer = CompressorWriter::new(Vec::new(), 4096, 6, 22);
            writer.write_all(&raw).unwrap();
            writer.flush().unwrap();
            let compressed = writer.into_inner();

            ([("content-encoding", HeaderValue::from_static("br"))], compressed).into_response()
        }),
    );

    let server = axum_test::TestServer::new(app).unwrap();
    let response = server.get("/br").await;

    let snapshot = snapshot_response(response).await.expect("snapshot failed");
    assert_eq!(snapshot.status, 200);
    assert_eq!(snapshot.text().unwrap(), "hello br");
}
