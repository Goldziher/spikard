//! File upload validation for multipart/form-data
//!
//! This module provides:
//! - Content-type validation against allowed MIME types
//! - Magic number validation for security (prevents MIME spoofing)
//! - Required/optional file field enforcement
//! - File size validation

use serde_json::Value;
use std::collections::HashMap;

/// Magic number signatures for common file types
/// Format: (mime_type, magic_bytes_hex)
const MAGIC_NUMBERS: &[(&str, &str)] = &[
    // Images
    ("image/png", "89504e470d0a1a0a"),
    ("image/jpeg", "ffd8ff"),
    ("image/gif", "474946383961"), // GIF89a
    ("image/gif", "474946383761"), // GIF87a
    ("image/webp", "52494646"),    // RIFF header
    // Documents
    ("application/pdf", "255044462d"),
    // Archives
    ("application/zip", "504b0304"),
    ("application/x-rar-compressed", "526172211a07"),
];

/// File validation parameters extracted from schema
#[derive(Debug, Clone)]
pub struct FileParameterSchema {
    pub required: bool,
    pub content_types: Option<Vec<String>>,
    pub validate_magic_numbers: bool,
    pub max_size: Option<usize>,
    pub min_size: Option<usize>,
}

impl FileParameterSchema {
    /// Parse file parameter schema from JSON
    pub fn from_json(schema: &Value) -> Self {
        Self {
            required: schema.get("required").and_then(|v| v.as_bool()).unwrap_or(false),
            content_types: schema.get("content_type").and_then(|v| {
                if let Some(arr) = v.as_array() {
                    Some(arr.iter().filter_map(|s| s.as_str().map(String::from)).collect())
                } else if let Some(s) = v.as_str() {
                    Some(vec![s.to_string()])
                } else {
                    None
                }
            }),
            validate_magic_numbers: schema
                .get("validate_magic_numbers")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            max_size: schema.get("max_size").and_then(|v| v.as_u64()).map(|n| n as usize),
            min_size: schema.get("min_size").and_then(|v| v.as_u64()).map(|n| n as usize),
        }
    }
}

/// File validation error
#[derive(Debug, Clone)]
pub struct FileValidationError {
    pub field_name: String,
    pub error_type: String,
    pub message: String,
    pub context: Option<Value>,
}

impl FileValidationError {
    /// Convert to JSON error detail format
    pub fn to_json(&self) -> Value {
        let mut error = serde_json::json!({
            "type": self.error_type,
            "loc": ["files", self.field_name],
            "msg": self.message,
        });

        if let Some(ctx) = &self.context {
            error["ctx"] = ctx.clone();
        }

        error
    }
}

/// Validate uploaded files against schema
pub fn validate_files(
    files_data: &Value,
    file_schemas: &HashMap<String, FileParameterSchema>,
) -> Result<(), Vec<FileValidationError>> {
    let mut errors = Vec::new();

    // Check required files are present
    for (field_name, schema) in file_schemas {
        if schema.required {
            let file_present = files_data.get(field_name).map(|v| !v.is_null()).unwrap_or(false);

            if !file_present {
                errors.push(FileValidationError {
                    field_name: field_name.clone(),
                    error_type: "missing".to_string(),
                    message: "Field required".to_string(),
                    context: Some(serde_json::json!({"input": "required"})),
                });
            }
        }
    }

    // Validate each uploaded file
    if let Some(files_obj) = files_data.as_object() {
        for (field_name, file_value) in files_obj {
            // Skip null values (optional files not provided)
            if file_value.is_null() {
                continue;
            }

            // Get schema for this field
            let schema = match file_schemas.get(field_name) {
                Some(s) => s,
                None => continue, // No schema = no validation
            };

            // Validate file object
            if let Err(mut field_errors) = validate_file_field(field_name, file_value, schema) {
                errors.append(&mut field_errors);
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Validate a single file field
fn validate_file_field(
    field_name: &str,
    file_value: &Value,
    schema: &FileParameterSchema,
) -> Result<(), Vec<FileValidationError>> {
    let mut errors = Vec::new();

    // Extract file metadata
    let file_obj = match file_value.as_object() {
        Some(obj) => obj,
        None => {
            errors.push(FileValidationError {
                field_name: field_name.to_string(),
                error_type: "validation_error".to_string(),
                message: "File field must be an object".to_string(),
                context: None,
            });
            return Err(errors);
        }
    };

    let content_type = file_obj
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream");

    let size = file_obj.get("size").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    let content = file_obj.get("content").and_then(|v| v.as_str()).unwrap_or("");

    // Validate content type
    if let Some(ref allowed_types) = schema.content_types
        && !allowed_types.iter().any(|t| t == content_type)
    {
        errors.push(FileValidationError {
            field_name: field_name.to_string(),
            error_type: "validation_error".to_string(),
            message: format!(
                "Invalid content type '{}'. Allowed types: {}",
                content_type,
                allowed_types.join(", ")
            ),
            context: Some(serde_json::json!({
                "allowed_types": allowed_types,
                "provided_type": content_type
            })),
        });
    }

    // Validate file size
    if let Some(max_size) = schema.max_size
        && size > max_size
    {
        errors.push(FileValidationError {
            field_name: field_name.to_string(),
            error_type: "validation_error".to_string(),
            message: format!("File too large. Maximum size is {} bytes", max_size),
            context: Some(serde_json::json!({
                "max_size": max_size,
                "file_size": size
            })),
        });
    }

    if let Some(min_size) = schema.min_size
        && size < min_size
    {
        errors.push(FileValidationError {
            field_name: field_name.to_string(),
            error_type: "validation_error".to_string(),
            message: format!("File too small. Minimum size is {} bytes", min_size),
            context: Some(serde_json::json!({
                "min_size": min_size,
                "file_size": size
            })),
        });
    }

    // Validate magic numbers if requested
    if schema.validate_magic_numbers
        && let Err(mut magic_errors) = validate_magic_numbers(field_name, content, content_type)
    {
        errors.append(&mut magic_errors);
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Validate file magic numbers against declared MIME type
fn validate_magic_numbers(
    field_name: &str,
    content: &str,
    declared_mime: &str,
) -> Result<(), Vec<FileValidationError>> {
    // Convert content to bytes (assuming UTF-8, lossy)
    let content_bytes = content.as_bytes();

    // Check if we have enough bytes to validate
    if content_bytes.is_empty() {
        return Ok(()); // Empty files pass magic number validation
    }

    // Convert first bytes to hex string for comparison
    let hex_prefix = content_bytes
        .iter()
        .take(16) // Check up to 16 bytes
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    // Find detected type by magic numbers
    let detected_type = MAGIC_NUMBERS
        .iter()
        .find(|(_, magic)| hex_prefix.starts_with(magic))
        .map(|(mime, _)| *mime);

    // If we detected a type, it must match the declared type
    if let Some(detected) = detected_type
        && detected != declared_mime
    {
        return Err(vec![FileValidationError {
            field_name: field_name.to_string(),
            error_type: "validation_error".to_string(),
            message: format!(
                "File type mismatch: MIME type is {} but magic numbers indicate {}",
                declared_mime, detected
            ),
            context: Some(serde_json::json!({
                "declared_mime": declared_mime,
                "detected_type": detected,
                "magic_bytes": &hex_prefix[..std::cmp::min(hex_prefix.len(), 16)]
            })),
        }]);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_required_file_missing() {
        let schema = FileParameterSchema {
            required: true,
            content_types: None,
            validate_magic_numbers: false,
            max_size: None,
            min_size: None,
        };

        let mut schemas = HashMap::new();
        schemas.insert("file".to_string(), schema);

        let files_data = json!({});

        let result = validate_files(&files_data, &schemas);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, "missing");
    }

    #[test]
    fn test_content_type_validation() {
        let schema = FileParameterSchema {
            required: true,
            content_types: Some(vec!["image/png".to_string()]),
            validate_magic_numbers: false,
            max_size: None,
            min_size: None,
        };

        let mut schemas = HashMap::new();
        schemas.insert("image".to_string(), schema);

        let files_data = json!({
            "image": {
                "filename": "test.jpg",
                "content_type": "image/jpeg",
                "size": 100,
                "content": "fake jpeg"
            }
        });

        let result = validate_files(&files_data, &schemas);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("Invalid content type")));
    }

    #[test]
    fn test_magic_number_validation_png() {
        let schema = FileParameterSchema {
            required: true,
            content_types: Some(vec!["image/png".to_string()]),
            validate_magic_numbers: true,
            max_size: None,
            min_size: None,
        };

        let mut schemas = HashMap::new();
        schemas.insert("image".to_string(), schema);

        // PNG magic bytes: 89 50 4e 47 0d 0a 1a 0a
        let png_bytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        let png_content = String::from_utf8_lossy(&png_bytes).to_string();

        let files_data = json!({
            "image": {
                "filename": "test.png",
                "content_type": "image/png",
                "size": png_bytes.len(),
                "content": png_content
            }
        });

        let result = validate_files(&files_data, &schemas);
        assert!(result.is_ok());
    }

    #[test]
    fn test_magic_number_spoofing_detection() {
        let schema = FileParameterSchema {
            required: true,
            content_types: Some(vec!["image/jpeg".to_string()]),
            validate_magic_numbers: true,
            max_size: None,
            min_size: None,
        };

        let mut schemas = HashMap::new();
        schemas.insert("image".to_string(), schema);

        // PNG magic bytes but declared as JPEG
        let png_bytes = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        let png_content = String::from_utf8_lossy(&png_bytes).to_string();

        let files_data = json!({
            "image": {
                "filename": "fake.jpg",
                "content_type": "image/jpeg",
                "size": png_bytes.len(),
                "content": png_content
            }
        });

        let result = validate_files(&files_data, &schemas);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("File type mismatch")));
    }
}
