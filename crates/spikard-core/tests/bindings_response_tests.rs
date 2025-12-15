use brotli::Decompressor;
use flate2::read::GzDecoder;
use spikard_core::{CompressionConfig, RawResponse, StaticAsset};
use std::collections::HashMap;
use std::io::Read;

fn decode_gzip(payload: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(payload);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).expect("gzip decode failed");
    buf
}

fn decode_brotli(payload: &[u8]) -> Vec<u8> {
    let mut decoder = Decompressor::new(payload, 4096);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).expect("brotli decode failed");
    buf
}

#[test]
fn raw_response_does_not_compress_empty_or_partial_content() {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());

    let mut empty = RawResponse::new(200, headers.clone(), Vec::new());
    let cfg = CompressionConfig {
        min_size: 1,
        ..Default::default()
    };
    empty.apply_compression(&HashMap::new(), &cfg);
    assert!(!empty.headers.contains_key("content-encoding"));
    assert!(empty.body.is_empty());

    let mut partial = RawResponse::new(206, headers, b"{\"ok\":true}".to_vec());
    partial.apply_compression(&HashMap::new(), &cfg);
    assert!(!partial.headers.contains_key("content-encoding"));
}

#[test]
fn raw_response_respects_existing_content_encoding_and_min_size() {
    let mut headers = HashMap::new();
    headers.insert("content-encoding".to_string(), "gzip".to_string());

    let mut response = RawResponse::new(200, headers, b"x".repeat(4096));
    let cfg = CompressionConfig::default();
    response.apply_compression(&HashMap::new(), &cfg);
    assert_eq!(
        response.headers.get("content-encoding").map(String::as_str),
        Some("gzip")
    );

    let mut small = RawResponse::new(200, HashMap::new(), b"x".repeat(10));
    let cfg = CompressionConfig {
        min_size: 1024,
        ..Default::default()
    };
    small.apply_compression(&HashMap::new(), &cfg);
    assert!(!small.headers.contains_key("content-encoding"));
}

#[test]
fn raw_response_prefers_brotli_when_accepted() {
    let original = b"hello world ".repeat(256);
    let mut response = RawResponse::new(200, HashMap::new(), original.clone());

    let mut request_headers = HashMap::new();
    request_headers.insert("Accept-Encoding".to_string(), "br, gzip".to_string());

    let cfg = CompressionConfig {
        min_size: 1,
        quality: 6,
        gzip: true,
        brotli: true,
    };

    response.apply_compression(&request_headers, &cfg);

    assert_eq!(response.headers.get("content-encoding").map(String::as_str), Some("br"));
    assert_eq!(
        response.headers.get("vary").map(String::as_str),
        Some("Accept-Encoding")
    );
    assert_eq!(decode_brotli(&response.body), original);
}

#[test]
fn raw_response_falls_back_to_gzip() {
    let original = b"hello world ".repeat(256);
    let mut response = RawResponse::new(200, HashMap::new(), original.clone());

    let mut request_headers = HashMap::new();
    request_headers.insert("Accept-Encoding".to_string(), "gzip".to_string());

    let cfg = CompressionConfig {
        min_size: 1,
        quality: 6,
        gzip: true,
        brotli: true,
    };

    response.apply_compression(&request_headers, &cfg);

    assert_eq!(
        response.headers.get("content-encoding").map(String::as_str),
        Some("gzip")
    );
    assert_eq!(decode_gzip(&response.body), original);
}

#[test]
fn static_asset_serves_get_and_head_only() {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/plain".to_string());

    let asset = StaticAsset {
        route: "/static/hello.txt".to_string(),
        headers,
        body: b"hello".to_vec(),
    };

    let get = asset.serve("GET", "/static/hello.txt").expect("expected GET response");
    assert_eq!(get.status, 200);
    assert_eq!(get.body, b"hello");
    assert_eq!(get.headers.get("content-length").map(String::as_str), Some("5"));

    let head = asset
        .serve("HEAD", "/static/hello.txt")
        .expect("expected HEAD response");
    assert_eq!(head.status, 200);
    assert!(head.body.is_empty());
    assert_eq!(head.headers.get("content-length").map(String::as_str), Some("5"));

    assert!(asset.serve("POST", "/static/hello.txt").is_none());
    assert!(asset.serve("GET", "/static/other.txt").is_none());
}
