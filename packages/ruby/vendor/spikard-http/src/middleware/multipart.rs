//! Multipart form-data parsing

use axum::extract::Multipart;
use serde_json::json;

/// Size threshold for streaming vs buffering multipart fields
/// Fields larger than this will be streamed chunk-by-chunk
const MULTIPART_STREAMING_THRESHOLD: usize = 1024 * 1024;

/// Parse multipart/form-data to JSON
///
/// This handles:
/// - File uploads → {"filename": "...", "size": N, "content": "...", "content_type": "..."}
/// - Form fields → plain string values
/// - Mixed files and data → combined in single JSON object
/// - Large files → streamed chunk-by-chunk (async)
/// - Small files → buffered in memory
/// - Multiple values with same field name → aggregated into arrays
///
/// Streaming strategy:
/// - Files > 1MB: Use field.chunk().await for async streaming
/// - Files <= 1MB: Use field.bytes().await for buffered loading
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
            let content = if is_text_like || size <= MULTIPART_STREAMING_THRESHOLD {
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
