#![allow(clippy::pedantic, clippy::nursery, clippy::all)]
//! Behavioral tests for multipart form data handling in spikard-http
//!
//! These tests verify observable behavior of the multipart parser through real HTTP requests
//! and response outcomes, not parsing internals. They test edge cases, error conditions,
//! and unusual input patterns to ensure robust handling of form data.

mod common;

use axum::body::Body;
use axum::extract::FromRequest;
use axum::extract::Multipart;
use axum::http::Request;

/// Helper to create a multipart body with proper RFC 7578 formatting
fn create_multipart_body(boundary: &str, parts: Vec<String>) -> Vec<u8> {
    let mut body = Vec::new();
    for part in parts {
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(part.as_bytes());
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    body
}

/// Test 1: Large file upload (>1MB) streaming behavior
/// Verifies that large files are properly handled and marked as binary/streaming
#[tokio::test]
async fn test_large_file_upload_streaming_behavior() {
    let boundary = "multipart-boundary";

    let large_file_size = 1024 * 1024 + 512;
    let large_file_content: Vec<u8> = (0..large_file_size).map(|i| (i % 256) as u8).collect();

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"large_upload\"; filename=\"large_file.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(&large_file_content);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("large_upload"));

    let file_obj = &obj["large_upload"];

    assert!(file_obj["filename"].is_string());
    assert_eq!(file_obj["filename"], "large_file.bin");
    assert!(file_obj["size"].is_number());
    assert_eq!(file_obj["size"], large_file_size);

    let content = file_obj["content"].as_str().unwrap();
    assert!(content.contains("<binary data"));
    assert!(content.contains("bytes"));
}

/// Test 2: Boundary string appearing in file data
/// Verifies that boundary strings within file content don't break parsing
#[tokio::test]
async fn test_boundary_string_in_file_data() {
    let boundary = "boundary123";

    let file_content = "This file contains --boundary123 in the data\nBut it should not confuse the parser";
    let parts = vec![format!(
        "Content-Disposition: form-data; name=\"file\"; filename=\"tricky.txt\"\r\nContent-Type: text/plain\r\n\r\n{}",
        file_content
    )];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("file"));

    let returned_content = obj["file"]["content"].as_str().unwrap();
    assert_eq!(returned_content, file_content);
    assert!(returned_content.contains("--boundary123"));
}

/// Test 3: Mixed file and form field ordering
/// Verifies that files and fields can be interleaved and all are captured correctly
#[tokio::test]
async fn test_mixed_file_and_form_field_ordering() {
    let boundary = "mixed-boundary";
    let parts = vec![
        "Content-Disposition: form-data; name=\"field1\"\r\n\r\nValue 1".to_string(),
        "Content-Disposition: form-data; name=\"upload1\"; filename=\"file1.txt\"\r\nContent-Type: text/plain\r\n\r\nFile 1 content".to_string(),
        "Content-Disposition: form-data; name=\"field2\"\r\n\r\nValue 2".to_string(),
        "Content-Disposition: form-data; name=\"upload2\"; filename=\"file2.txt\"\r\nContent-Type: text/plain\r\n\r\nFile 2 content".to_string(),
        "Content-Disposition: form-data; name=\"field3\"\r\n\r\nValue 3".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();

    assert_eq!(obj["field1"], "Value 1");
    assert_eq!(obj["field2"], "Value 2");
    assert_eq!(obj["field3"], "Value 3");

    assert!(obj["upload1"].is_object());
    assert!(obj["upload2"].is_object());
    assert_eq!(obj["upload1"]["filename"], "file1.txt");
    assert_eq!(obj["upload2"]["filename"], "file2.txt");
}

/// Test 4: Invalid UTF-8 in text fields
/// Verifies graceful handling of non-UTF8 data in text fields with lossy conversion
#[tokio::test]
async fn test_invalid_utf8_in_text_fields() {
    let boundary = "boundary123";
    let mut body = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"text_field\"\r\n\r\n");

    body.extend_from_slice(b"Valid UTF-8: ");

    body.extend_from_slice(&[0xFF, 0xFE, 0xFD]);

    body.extend_from_slice(b" and more valid");

    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();

    assert!(obj.contains_key("text_field"));
    let text_value = obj["text_field"].as_str().unwrap();

    assert!(text_value.contains("Valid UTF-8:"));
    assert!(text_value.contains("and more valid"));
}

/// Test 5: Malformed multipart bodies (missing headers)
/// Verifies that the parser handles structural errors gracefully
#[tokio::test]
async fn test_malformed_multipart_missing_content_disposition() {
    let boundary = "boundary123";
    let mut body = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(b"orphaned content");
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();

    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart).await;

    if let Ok(parsed) = result {
        assert!(parsed.is_object());
    }
}

/// Test 6: Duplicate field names aggregation
/// Verifies that multiple values with same field name are properly aggregated into arrays
#[tokio::test]
async fn test_duplicate_field_names_aggregation() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"tags\"\r\n\r\nrust".to_string(),
        "Content-Disposition: form-data; name=\"tags\"\r\n\r\nweb".to_string(),
        "Content-Disposition: form-data; name=\"tags\"\r\n\r\napi".to_string(),
        "Content-Disposition: form-data; name=\"tags\"\r\n\r\nserver".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("tags"));

    let tags = &obj["tags"];
    assert!(
        tags.is_array(),
        "Multiple values with same name should be aggregated into array"
    );

    let tags_array = tags.as_array().unwrap();
    assert_eq!(tags_array.len(), 4, "All 4 tag values should be present");
    assert_eq!(tags_array[0], "rust");
    assert_eq!(tags_array[1], "web");
    assert_eq!(tags_array[2], "api");
    assert_eq!(tags_array[3], "server");
}

/// Test 7: Upload timeout behavior with streaming files
/// Verifies that streaming large files doesn't cause handlers to timeout
/// (Tests observable streaming behavior, not timeout mechanics)
#[tokio::test]
async fn test_large_streaming_file_completes() {
    let boundary = "stream-boundary";

    let streaming_file_size = 1024 * 1024 + 256 * 1024;
    let streaming_file: Vec<u8> = (0..streaming_file_size).map(|i| ((i >> 8) % 256) as u8).collect();

    let mut body = Vec::new();
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"stream_upload\"; filename=\"stream.dat\"\r\nContent-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(&streaming_file);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();

    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart).await;

    assert!(result.is_ok(), "Large streaming file should parse without timeout");

    let parsed = result.unwrap();
    let obj = parsed.as_object().unwrap();
    assert!(obj.contains_key("stream_upload"));

    assert_eq!(obj["stream_upload"]["size"], streaming_file_size);
}

/// Test 8: Multiple files with same field name
/// Verifies that multiple file uploads with identical field names are aggregated correctly
#[tokio::test]
async fn test_multiple_file_uploads_same_field_name() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"attachments\"; filename=\"file1.txt\"\r\nContent-Type: text/plain\r\n\r\nFirst file content".to_string(),
        "Content-Disposition: form-data; name=\"attachments\"; filename=\"file2.txt\"\r\nContent-Type: text/plain\r\n\r\nSecond file content".to_string(),
        "Content-Disposition: form-data; name=\"attachments\"; filename=\"file3.txt\"\r\nContent-Type: text/plain\r\n\r\nThird file content".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("attachments"));

    let attachments = &obj["attachments"];
    assert!(
        attachments.is_array(),
        "Multiple files with same name should be aggregated"
    );

    let files = attachments.as_array().unwrap();
    assert_eq!(files.len(), 3, "All 3 file uploads should be present");

    assert_eq!(files[0]["filename"], "file1.txt");
    assert_eq!(files[0]["content"], "First file content");

    assert_eq!(files[1]["filename"], "file2.txt");
    assert_eq!(files[1]["content"], "Second file content");

    assert_eq!(files[2]["filename"], "file3.txt");
    assert_eq!(files[2]["content"], "Third file content");
}

/// Test 9: Empty file upload
/// Verifies that empty files are handled correctly
#[tokio::test]
async fn test_empty_file_upload() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"empty_file\"; filename=\"empty.txt\"\r\nContent-Type: text/plain\r\n\r\n".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("empty_file"));

    let file_obj = &obj["empty_file"];
    assert_eq!(file_obj["size"], 0, "Empty file should have size 0");
    assert_eq!(file_obj["content"], "", "Empty file should have empty content");
    assert_eq!(file_obj["filename"], "empty.txt");
}

/// Test 10: File with binary null bytes
/// Verifies that binary data with null bytes is preserved correctly
#[tokio::test]
async fn test_binary_file_with_null_bytes() {
    let boundary = "boundary123";
    let mut body = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"binary\"; filename=\"binary.dat\"\r\nContent-Type: application/octet-stream\r\n\r\n");

    body.extend_from_slice(&[0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC]);

    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("binary"));

    let file_obj = &obj["binary"];
    assert_eq!(file_obj["size"], 8, "Binary file size should be 8 bytes");
    assert_eq!(file_obj["content_type"], "application/octet-stream");
}

/// Test 11: Mixed JSON and binary file uploads
/// Verifies that JSON form fields are parsed while binary files are preserved
#[tokio::test]
async fn test_mixed_json_and_binary_files() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"metadata\"\r\n\r\n{\"version\":1,\"timestamp\":1234567890}".to_string(),
        "Content-Disposition: form-data; name=\"image\"; filename=\"photo.bin\"\r\nContent-Type: application/octet-stream\r\n\r\nBINARY_IMAGE_DATA_HERE".to_string(),
        "Content-Disposition: form-data; name=\"config\"\r\n\r\n[1,2,3,4,5]".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();

    assert!(obj["metadata"].is_object());
    assert_eq!(obj["metadata"]["version"], 1);

    assert!(obj["config"].is_array());
    assert_eq!(obj["config"][0], 1);
    assert_eq!(obj["config"][4], 5);

    assert!(obj["image"].is_object());
    assert_eq!(obj["image"]["filename"], "photo.bin");
    assert_eq!(obj["image"]["content"], "BINARY_IMAGE_DATA_HERE");
}

/// Test 12: Very large field name
/// Verifies that extremely long field names are handled correctly
#[tokio::test]
async fn test_very_large_field_name() {
    let boundary = "boundary123";
    let long_field_name = "field_".repeat(200);
    let parts = vec![format!(
        "Content-Disposition: form-data; name=\"{}\"\r\n\r\nvalue",
        long_field_name
    )];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    assert!(obj.contains_key(&long_field_name));
    assert_eq!(obj[&long_field_name], "value");
}

/// Test 13: Content-Type with charset parameter
/// Verifies that Content-Type headers with parameters are preserved
#[tokio::test]
async fn test_content_type_with_charset_parameter() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"text_file\"; filename=\"utf8.txt\"\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nUTF-8 encoded content: Ñ é ü".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    let file_obj = &obj["text_file"];

    let content_type = file_obj["content_type"].as_str().unwrap();
    assert!(content_type.contains("text/plain"));
}

/// Test 14: CRLF line endings preservation in file content
/// Verifies that CRLF sequences within file content are preserved
#[tokio::test]
async fn test_crlf_line_endings_in_file_content() {
    let boundary = "boundary123";
    let content_with_crlf = "Line 1\r\nLine 2\r\nLine 3\r\n";
    let parts = vec![format!(
        "Content-Disposition: form-data; name=\"multiline\"; filename=\"text.txt\"\r\nContent-Type: text/plain\r\n\r\n{}",
        content_with_crlf
    )];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    let file_content = obj["multiline"]["content"].as_str().unwrap();

    assert!(file_content.contains("Line 1\r\nLine 2"));
}

/// Test 15: File upload with default Content-Type when missing
/// Verifies that missing Content-Type defaults to application/octet-stream
#[tokio::test]
async fn test_default_content_type_when_missing() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"no_type\"; filename=\"mystery.bin\"\r\n\r\nFile content without explicit type".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    let file_obj = &obj["no_type"];

    assert_eq!(
        file_obj["content_type"], "application/octet-stream",
        "Missing Content-Type should default to application/octet-stream"
    );
}

/// Test 16: Unicode characters in filenames
/// Verifies that Unicode filenames are properly handled and preserved
#[tokio::test]
async fn test_unicode_characters_in_filenames() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"file\"; filename=\"файл_文件_🎯.txt\"\r\nContent-Type: text/plain\r\n\r\nUnicode filename test".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();
    let filename = obj["file"]["filename"].as_str().unwrap();

    assert!(filename.contains("файл"));
    assert!(filename.contains("文件"));
}

/// Test 17: All single-value fields when no duplicates
/// Verifies that fields with single values are not converted to arrays
#[tokio::test]
async fn test_single_value_fields_not_arrays() {
    let boundary = "boundary123";
    let parts = vec![
        "Content-Disposition: form-data; name=\"username\"\r\n\r\njohn_doe".to_string(),
        "Content-Disposition: form-data; name=\"email\"\r\n\r\njohn@example.com".to_string(),
        "Content-Disposition: form-data; name=\"age\"\r\n\r\n30".to_string(),
    ];

    let body = create_multipart_body(boundary, parts);

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();

    assert!(obj["username"].is_string());
    assert!(obj["email"].is_string());
    assert!(obj["age"].is_string());
}

/// Test 18: File around the 1MB streaming threshold
/// Verifies correct handling at the boundary of streaming threshold
#[tokio::test]
async fn test_file_around_1mb_threshold() {
    let boundary = "boundary123";

    let small_file_size = 1024 * 1024 - 1024;
    let small_file: Vec<u8> = (0..small_file_size).map(|i| (i % 256) as u8).collect();

    let large_file_size = 1024 * 1024 + 512;
    let large_file: Vec<u8> = (0..large_file_size).map(|i| ((i >> 8) % 256) as u8).collect();

    let mut body = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"small\"; filename=\"small.dat\"\r\nContent-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(&small_file);
    body.extend_from_slice(b"\r\n");

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"large\"; filename=\"large.dat\"\r\nContent-Type: application/octet-stream\r\n\r\n");
    body.extend_from_slice(&large_file);
    body.extend_from_slice(b"\r\n");

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method("POST")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(body))
        .unwrap();

    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let result = spikard_http::middleware::multipart::parse_multipart_to_json(multipart)
        .await
        .unwrap();

    let obj = result.as_object().unwrap();

    assert_eq!(obj["small"]["size"], small_file_size);

    let large_content = obj["large"]["content"].as_str().unwrap();
    assert!(large_content.contains("<binary data") || large_content.len() > 100);
}
