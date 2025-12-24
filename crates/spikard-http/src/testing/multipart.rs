use std::time::{SystemTime, UNIX_EPOCH};

/// File part metadata for multipart/form-data payloads.
#[derive(Debug, Clone)]
pub struct MultipartFilePart {
    pub field_name: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
}

/// Build a multipart/form-data body from fields and files.
pub fn build_multipart_body(form_fields: &[(String, String)], files: &[MultipartFilePart]) -> (Vec<u8>, String) {
    let boundary = generate_boundary();
    let mut body = Vec::new();

    for (name, value) in form_fields {
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(name.as_bytes());
        body.extend_from_slice(b"\"\r\n\r\n");
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }

    for file in files {
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(file.field_name.as_bytes());
        body.extend_from_slice(b"\"");
        if !file.filename.is_empty() {
            body.extend_from_slice(b"; filename=\"");
            body.extend_from_slice(file.filename.as_bytes());
            body.extend_from_slice(b"\"");
        }
        body.extend_from_slice(b"\r\n");
        if let Some(content_type) = &file.content_type {
            body.extend_from_slice(b"Content-Type: ");
            body.extend_from_slice(content_type.as_bytes());
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(b"\r\n");
        body.extend_from_slice(&file.content);
        body.extend_from_slice(b"\r\n");
    }

    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--\r\n");

    (body, boundary)
}

fn generate_boundary() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("spikard-boundary-{nanos}")
}
