//! Multipart form-data parsing

use axum::extract::Multipart;
use serde_json::json;

/// Max bytes of file content to inline into the JSON "content" field for non-text uploads.
///
/// Keeping this small avoids turning file uploads into large JSON strings (CPU + memory),
/// while still supporting fixtures that assert on small binary payloads.
const MULTIPART_INLINE_CONTENT_LIMIT: usize = 8 * 1024;

/// Parse multipart/form-data to JSON
///
/// This handles:
/// - File uploads → {"filename": "...", "size": N, "content": "...", "content_type": "..."}
/// - Form fields → plain string values
/// - Mixed files and data → combined in single JSON object
/// - Large binary files → placeholder content
/// - Small files → content inlined into JSON
/// - Multiple values with same field name → aggregated into arrays
pub async fn parse_multipart_to_json(
    mut multipart: Multipart,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    use rustc_hash::FxHashMap;

    let mut field_values: FxHashMap<String, Vec<serde_json::Value>> = FxHashMap::default();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().ok_or("Field missing name")?.to_string();

        let field_value = if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let content_type = field
                .content_type()
                .map(|ct| ct.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let bytes = field.bytes().await?;
            let size = bytes.len();

            let is_text_like = content_type.starts_with("text/") || content_type == "application/json";
            let content = if is_text_like || size <= MULTIPART_INLINE_CONTENT_LIMIT {
                String::from_utf8_lossy(&bytes).to_string()
            } else {
                format!("<binary data, {} bytes>", size)
            };

            json!({
                "filename": filename,
                "size": size,
                "content": content,
                "content_type": content_type
            })
        } else {
            let value = field.text().await?;

            if (value.starts_with('[') && value.ends_with(']')) || (value.starts_with('{') && value.ends_with('}')) {
                if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&value) {
                    parsed_json
                } else {
                    json!(value)
                }
            } else {
                json!(value)
            }
        };

        field_values.entry(name).or_default().push(field_value);
    }

    let result: serde_json::Map<String, serde_json::Value> = field_values
        .into_iter()
        .map(|(key, mut values)| {
            if values.len() == 1 {
                let val = values.pop().unwrap_or(serde_json::Value::Null);
                (key, val)
            } else {
                (key, serde_json::Value::Array(values))
            }
        })
        .collect();

    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::FromRequest;
    use axum::extract::Multipart;

    /// Helper function to create a multipart boundary string
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

    #[tokio::test]
    async fn test_single_file_upload() {
        let boundary = "boundary123";
        let file_content = "Hello, World!";
        let parts = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\nContent-Type: text/plain\r\n\r\nHello, World!".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        assert!(result.is_object());
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("file"));

        let file_obj = &obj["file"];
        assert_eq!(file_obj["filename"], "test.txt");
        assert_eq!(file_obj["size"], 13);
        assert_eq!(file_obj["content"], file_content);
        assert_eq!(file_obj["content_type"], "text/plain");
    }

    #[tokio::test]
    async fn test_multiple_files() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"file1\"; filename=\"file1.txt\"\r\nContent-Type: text/plain\r\n\r\nContent 1".to_string(),
            "Content-Disposition: form-data; name=\"file2\"; filename=\"file2.txt\"\r\nContent-Type: text/plain\r\n\r\nContent 2".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("file1"));
        assert!(obj.contains_key("file2"));
        assert_eq!(obj["file1"]["filename"], "file1.txt");
        assert_eq!(obj["file2"]["filename"], "file2.txt");
    }

    #[tokio::test]
    async fn test_mixed_form_data_and_files() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"username\"\r\n\r\njohn_doe".to_string(),
            "Content-Disposition: form-data; name=\"avatar\"; filename=\"avatar.png\"\r\nContent-Type: image/png\r\n\r\nPNG_BINARY_DATA".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["username"], "john_doe");
        assert_eq!(obj["avatar"]["filename"], "avatar.png");
        assert_eq!(obj["avatar"]["content_type"], "image/png");
    }

    #[tokio::test]
    async fn test_content_type_validation() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"data\"; filename=\"data.json\"\r\nContent-Type: application/json\r\n\r\n{}".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["data"]["content_type"], "application/json");
    }

    #[tokio::test]
    async fn test_text_file_handling() {
        let boundary = "boundary123";
        let text_content = "This is a plain text file\nwith multiple lines";
        let parts = vec![
            "Content-Disposition: form-data; name=\"document\"; filename=\"document.txt\"\r\nContent-Type: text/plain\r\n\r\nThis is a plain text file\nwith multiple lines".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["document"]["content"], text_content);
        assert!(obj["document"]["content"].is_string());
    }

    #[tokio::test]
    async fn test_binary_file_handling() {
        let boundary = "boundary123";
        let binary_data: Vec<u8> = vec![0xFF; 1024 * 1024 + 1];
        let mut body = Vec::new();
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"binary\"; filename=\"binary.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(&binary_data);
        body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        let content = obj["binary"]["content"].as_str().unwrap();
        assert!(content.contains("<binary data"));
        assert!(content.contains("1048577 bytes"));
    }

    #[tokio::test]
    async fn test_json_field_parsing() {
        let boundary = "boundary123";
        let json_value = r#"{"key":"value","nested":{"inner":"data"}}"#;
        let parts = vec![format!(
            "Content-Disposition: form-data; name=\"metadata\"\r\n\r\n{}",
            json_value
        )];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        let metadata = &obj["metadata"];
        assert!(metadata.is_object());
        assert_eq!(metadata["key"], "value");
        assert_eq!(metadata["nested"]["inner"], "data");
    }

    #[tokio::test]
    async fn test_multiple_values_same_name() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"tags\"\r\n\r\nrust".to_string(),
            "Content-Disposition: form-data; name=\"tags\"\r\n\r\nweb".to_string(),
            "Content-Disposition: form-data; name=\"tags\"\r\n\r\nserver".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        let tags = &obj["tags"];
        assert!(tags.is_array());
        let tags_array = tags.as_array().unwrap();
        assert_eq!(tags_array.len(), 3);
        assert_eq!(tags_array[0], "rust");
        assert_eq!(tags_array[1], "web");
        assert_eq!(tags_array[2], "server");
    }

    #[tokio::test]
    async fn test_empty_field() {
        let boundary = "boundary123";
        let parts = vec!["Content-Disposition: form-data; name=\"empty_field\"\r\n\r\n".to_string()];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["empty_field"], "");
    }

    #[tokio::test]
    async fn test_default_content_type_when_missing() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"no_type.bin\"\r\n\r\nbinary content".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["file"]["content_type"], "application/octet-stream");
    }

    #[tokio::test]
    async fn test_json_array_field_parsing() {
        let boundary = "boundary123";
        let json_array = r#"[1,2,3,4,5]"#;
        let parts = vec![format!(
            "Content-Disposition: form-data; name=\"numbers\"\r\n\r\n{}",
            json_array
        )];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        let numbers = &obj["numbers"];
        assert!(numbers.is_array());
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[4], 5);
    }

    #[tokio::test]
    async fn test_mixed_json_and_text_files() {
        let boundary = "boundary123";
        let parts = vec![
            "Content-Disposition: form-data; name=\"config\"; filename=\"config.json\"\r\nContent-Type: application/json\r\n\r\n{\"enabled\":true}".to_string(),
            "Content-Disposition: form-data; name=\"readme\"; filename=\"README.txt\"\r\nContent-Type: text/plain\r\n\r\nThis is a readme".to_string(),
        ];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["config"]["content_type"], "application/json");
        assert_eq!(obj["readme"]["content_type"], "text/plain");
        assert!(obj["config"]["content"].is_string());
    }

    #[tokio::test]
    async fn test_file_size_calculation() {
        let boundary = "boundary123";
        let file_content = "Exact size test";
        let expected_size = file_content.len();
        let parts = vec![format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"size.txt\"\r\nContent-Type: text/plain\r\n\r\n{}",
            file_content
        )];
        let body = create_multipart_body(boundary, parts);

        let request = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart = Multipart::from_request(request, &()).await.unwrap();
        let result = parse_multipart_to_json(multipart).await.unwrap();

        let obj = result.as_object().unwrap();
        assert_eq!(obj["file"]["size"], expected_size);
    }

    #[tokio::test]
    async fn test_boundary_with_special_characters() {
        let boundary: &str = "boundary-with-dashes_and_underscores.123";
        let parts: Vec<String> = vec!["Content-Disposition: form-data; name=\"field\"\r\n\r\nvalue".to_string()];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert_eq!(obj["field"], "value");
    }

    #[tokio::test]
    async fn test_boundary_parsing_with_crlf() {
        let boundary: &str = "boundary123";
        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"field\"\r\n");
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(b"value");
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert!(obj.contains_key("field"));
        assert_eq!(obj["field"], "value");
    }

    #[tokio::test]
    async fn test_file_upload_with_no_content_type_header() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"nocontenttype.bin\"\r\n\r\nbinary_content_here"
                .to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let file_obj: &serde_json::Value = &obj["file"];

        assert_eq!(file_obj["content_type"], "application/octet-stream");
        assert_eq!(file_obj["filename"], "nocontenttype.bin");
    }

    #[tokio::test]
    async fn test_empty_file_upload() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"emptyfile\"; filename=\"empty.txt\"\r\nContent-Type: text/plain\r\n\r\n".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let file_obj: &serde_json::Value = &obj["emptyfile"];

        assert_eq!(file_obj["size"], 0);
        assert_eq!(file_obj["content"], "");
    }

    #[tokio::test]
    async fn test_very_large_field_name() {
        let boundary: &str = "boundary123";
        let long_field_name: String = "field_".repeat(100);
        let parts: Vec<String> = vec![format!(
            "Content-Disposition: form-data; name=\"{}\"\r\n\r\nvalue",
            long_field_name
        )];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert!(obj.contains_key(&long_field_name));
        assert_eq!(obj[&long_field_name], "value");
    }

    #[tokio::test]
    async fn test_very_large_field_value() {
        let boundary: &str = "boundary123";
        let large_value: String = "x".repeat(10_000);
        let parts: Vec<String> = vec![format!(
            "Content-Disposition: form-data; name=\"large\"\r\n\r\n{}",
            large_value
        )];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let retrieved_value: &str = obj["large"].as_str().unwrap();
        assert_eq!(retrieved_value.len(), 10_000);
    }

    #[tokio::test]
    async fn test_multiple_files_same_name() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"document\"; filename=\"doc1.txt\"\r\nContent-Type: text/plain\r\n\r\nContent 1".to_string(),
            "Content-Disposition: form-data; name=\"document\"; filename=\"doc2.txt\"\r\nContent-Type: text/plain\r\n\r\nContent 2".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let docs: &serde_json::Value = &obj["document"];
        assert!(docs.is_array());

        let docs_array: &Vec<serde_json::Value> = docs.as_array().unwrap();
        assert_eq!(docs_array.len(), 2);
        assert_eq!(docs_array[0]["filename"], "doc1.txt");
        assert_eq!(docs_array[1]["filename"], "doc2.txt");
    }

    #[tokio::test]
    async fn test_unicode_in_filename() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"файл_文件_🎯.txt\"\r\nContent-Type: text/plain\r\n\r\nUnicode content".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let filename: &str = obj["file"]["filename"].as_str().unwrap();
        assert!(filename.contains("файл"));
        assert!(filename.contains("文件"));
    }

    #[tokio::test]
    async fn test_file_with_crlf_in_content() {
        let boundary: &str = "boundary123";
        let content_with_crlf: &str = "Line 1\r\nLine 2\r\nLine 3";
        let parts: Vec<String> = vec![format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"crlf.txt\"\r\nContent-Type: text/plain\r\n\r\n{}",
            content_with_crlf
        )];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let retrieved_content: &str = obj["file"]["content"].as_str().unwrap();
        assert!(retrieved_content.contains("Line 1"));
        assert!(retrieved_content.contains("Line 2"));
    }

    #[tokio::test]
    async fn test_boundary_on_line_boundary() {
        let boundary: &str = "boundary";
        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"field1\"\r\n\r\nvalue1\r\n");
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"field2\"\r\n\r\nvalue2\r\n");
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert_eq!(obj["field1"], "value1");
        assert_eq!(obj["field2"], "value2");
    }

    #[tokio::test]
    async fn test_content_type_with_charset_parameter() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"utf8.txt\"\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nUTF-8 content".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let content_type: &str = obj["file"]["content_type"].as_str().unwrap();
        assert!(content_type.contains("text/plain"));
    }

    #[tokio::test]
    async fn test_null_bytes_in_binary_file() {
        let boundary: &str = "boundary123";
        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"binary\"; filename=\"nullbytes.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(&[0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD]);
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let file_obj: &serde_json::Value = &obj["binary"];
        assert_eq!(file_obj["size"], 6);
    }

    #[tokio::test]
    async fn test_form_field_with_malformed_json_stays_string() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> =
            vec!["Content-Disposition: form-data; name=\"malformed\"\r\n\r\n{invalid json}".to_string()];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert!(obj["malformed"].is_string());
        assert_eq!(obj["malformed"], "{invalid json}");
    }

    #[tokio::test]
    async fn test_multiple_files_different_sizes_around_threshold() {
        let boundary: &str = "boundary123";
        let small_content: Vec<u8> = "small".repeat(100).into_bytes();
        let large_content: Vec<u8> = vec![0x42; 1024 * 1024 + 1];

        let mut body: Vec<u8> = Vec::new();

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"small\"; filename=\"small.txt\"\r\nContent-Type: text/plain\r\n\r\n");
        body.extend_from_slice(&small_content);
        body.extend_from_slice(b"\r\n");

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"large\"; filename=\"large.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(&large_content);
        body.extend_from_slice(b"\r\n");

        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();

        assert!(obj.contains_key("small"));
        assert!(obj["small"]["content"].is_string());
        let small_str = obj["small"]["content"].as_str().unwrap();
        assert_eq!(small_str.len(), 500);

        assert!(obj.contains_key("large"));
        let large_content_str = obj["large"]["content"].as_str().unwrap();
        assert!(large_content_str.contains("<binary data"));
        assert!(large_content_str.contains("1048577 bytes"));
    }

    #[tokio::test]
    async fn test_header_case_insensitivity() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> =
            vec!["content-disposition: form-data; name=\"field\"\r\nContent-TYPE: text/plain\r\n\r\nvalue".to_string()];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert_eq!(obj["field"], "value");
    }

    #[tokio::test]
    async fn test_field_value_containing_boundary_string() {
        let boundary: &str = "boundary123";
        let value_with_boundary: &str = "This contains --boundary123 but is not a real boundary";
        let parts: Vec<String> = vec![format!(
            "Content-Disposition: form-data; name=\"field\"\r\n\r\n{}",
            value_with_boundary
        )];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert_eq!(obj["field"], value_with_boundary);
    }

    #[tokio::test]
    async fn test_json_object_and_array_aggregation_with_files() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"config\"\r\n\r\n{\"debug\":true}".to_string(),
            "Content-Disposition: form-data; name=\"file\"; filename=\"data.txt\"\r\nContent-Type: text/plain\r\n\r\nfile content".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert!(obj["config"].is_object());
        assert!(obj["file"].is_object());
        assert_eq!(obj["file"]["filename"], "data.txt");
    }

    #[tokio::test]
    async fn test_consecutive_empty_fields() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"field1\"\r\n\r\n".to_string(),
            "Content-Disposition: form-data; name=\"field2\"\r\n\r\n".to_string(),
            "Content-Disposition: form-data; name=\"field3\"\r\n\r\nactual_value".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        assert_eq!(obj["field1"], "");
        assert_eq!(obj["field2"], "");
        assert_eq!(obj["field3"], "actual_value");
    }

    #[tokio::test]
    async fn test_filename_with_quotes_and_special_chars() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"file\"; filename=\"file[2024].txt\"\r\nContent-Type: text/plain\r\n\r\nContent".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let filename: &str = obj["file"]["filename"].as_str().unwrap();
        assert_eq!(filename, "file[2024].txt");
    }

    #[tokio::test]
    async fn test_text_like_content_type_variant_detection() {
        let boundary: &str = "boundary123";
        let parts: Vec<String> = vec![
            "Content-Disposition: form-data; name=\"html\"; filename=\"page.html\"\r\nContent-Type: text/html\r\n\r\n<html></html>".to_string(),
            "Content-Disposition: form-data; name=\"xml\"; filename=\"data.xml\"\r\nContent-Type: application/xml\r\n\r\n<data></data>".to_string(),
        ];
        let body: Vec<u8> = create_multipart_body(boundary, parts);

        let request: axum::http::Request<axum::body::Body> = axum::http::Request::builder()
            .method("POST")
            .header("content-type", format!("multipart/form-data; boundary={}", boundary))
            .body(axum::body::Body::from(body))
            .unwrap();

        let multipart: Multipart = Multipart::from_request(request, &()).await.unwrap();
        let result: serde_json::Value = parse_multipart_to_json(multipart).await.unwrap();

        let obj: &serde_json::Map<String, serde_json::Value> = result.as_object().unwrap();
        let html_content: &str = obj["html"]["content"].as_str().unwrap();
        let xml_content: &str = obj["xml"]["content"].as_str().unwrap();

        assert_eq!(html_content, "<html></html>");
        assert_eq!(xml_content, "<data></data>");
    }
}
