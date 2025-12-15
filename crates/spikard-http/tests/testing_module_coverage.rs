use axum::body::Body;
use axum::http::{HeaderValue, Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use flate2::Compression;
use flate2::write::GzEncoder;
use spikard_http::testing::{SnapshotError, WebSocketMessage, call_test_server, connect_websocket, snapshot_response};
use std::io::Write;

#[tokio::test]
async fn call_test_server_preserves_method_headers_query_and_body() {
    let app = axum::Router::new().route(
        "/echo",
        get(|req: Request<Body>| async move {
            let method = req.method().to_string();
            let uri = req.uri().to_string();
            let header = req
                .headers()
                .get("x-test")
                .and_then(|v| v.to_str().ok())
                .map_or_else(|| "<missing>".to_string(), str::to_string);
            let bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
            (StatusCode::OK, format!("{method} {uri} {header} {}", bytes.len())).into_response()
        }),
    );

    let server = axum_test::TestServer::new(app).expect("server");
    let request = Request::builder()
        .method("GET")
        .uri("/echo?q=1")
        .header("x-test", "1")
        .body(Body::from("abc"))
        .expect("request");

    let response = call_test_server(&server, request).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let text = response.text();
    assert!(text.contains("GET"));
    assert!(text.contains("/echo"));
    assert!(text.contains("q=1"));
}

#[tokio::test]
async fn snapshot_response_reports_invalid_headers_and_decompression_errors() {
    let bad_header = HeaderValue::from_bytes(b"\xFF").expect("header value");
    let app = axum::Router::new()
        .route(
            "/bad-header",
            get(move || async move {
                (
                    StatusCode::OK,
                    [(axum::http::header::HeaderName::from_static("x-bad"), bad_header.clone())],
                    "ok",
                )
            }),
        )
        .route(
            "/bad-gzip",
            get(|| async move {
                (
                    StatusCode::OK,
                    [(axum::http::header::CONTENT_ENCODING, "gzip")],
                    vec![0_u8, 1, 2, 3],
                )
            }),
        );

    let server = axum_test::TestServer::new(app).expect("server");

    let err = snapshot_response(server.get("/bad-header").await)
        .await
        .expect_err("invalid header");
    assert!(matches!(err, SnapshotError::InvalidHeader(_)));

    let err = snapshot_response(server.get("/bad-gzip").await)
        .await
        .expect_err("bad gzip");
    assert!(matches!(err, SnapshotError::Decompression(_)));
}

#[tokio::test]
async fn websocket_testing_wrappers_roundtrip_and_message_helpers() {
    let app = axum::Router::new().route(
        "/ws",
        get(|ws: axum::extract::ws::WebSocketUpgrade| async move {
            ws.on_upgrade(|mut socket| async move {
                while let Some(msg) = socket.recv().await {
                    match msg {
                        Ok(axum::extract::ws::Message::Text(text)) => {
                            let _ = socket.send(axum::extract::ws::Message::Text(text)).await;
                        }
                        Ok(axum::extract::ws::Message::Binary(data)) => {
                            let _ = socket.send(axum::extract::ws::Message::Binary(data)).await;
                        }
                        Ok(axum::extract::ws::Message::Ping(data)) => {
                            let _ = socket.send(axum::extract::ws::Message::Pong(data)).await;
                        }
                        Ok(axum::extract::ws::Message::Close(_)) | Err(_) => break,
                        Ok(axum::extract::ws::Message::Pong(_)) => {}
                    }
                }
            })
        }),
    );

    let server = axum_test::TestServer::new_with_config(
        app,
        axum_test::TestServerConfig {
            transport: Some(axum_test::Transport::HttpRandomPort),
            ..axum_test::TestServerConfig::default()
        },
    )
    .expect("server");

    let mut ws = connect_websocket(&server, "/ws").await;

    ws.send_text("hi").await;
    let msg = ws.receive_message().await;
    assert_eq!(msg.as_text(), Some("hi"));
    assert!(msg.as_json().is_err());

    ws.send_message(axum_test::WsMessage::Binary(bytes::Bytes::from_static(b"bin")))
        .await;
    let msg = ws.receive_message().await;
    assert_eq!(msg.as_binary().expect("binary"), b"bin");
    assert!(msg.as_json().is_err());

    ws.send_message(axum_test::WsMessage::Ping(bytes::Bytes::from_static(b"ping")))
        .await;
    let msg = ws.receive_message().await;
    assert!(matches!(msg, WebSocketMessage::Pong(_)));

    ws.close().await;
}

#[tokio::test]
async fn snapshot_response_decodes_gzip_body() {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(b"hello gzip").expect("write");
    let gzipped = encoder.finish().expect("finish");

    let app = axum::Router::new().route(
        "/gzip",
        get(move || async move {
            (
                StatusCode::OK,
                [(axum::http::header::CONTENT_ENCODING, "gzip")],
                gzipped.clone(),
            )
        }),
    );

    let server = axum_test::TestServer::new(app).expect("server");
    let snapshot = snapshot_response(server.get("/gzip").await).await.expect("snapshot");
    assert_eq!(snapshot.text().expect("text"), "hello gzip");
}
