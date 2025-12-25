//! JSON schema validation middleware

use axum::http::HeaderValue;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use spikard_core::problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};

/// Check if a media type is JSON or has a +json suffix
pub fn is_json_content_type(mime: &mime::Mime) -> bool {
    (mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON) || mime.suffix() == Some(mime::JSON)
}

fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let mut start = 0usize;
    let mut end = bytes.len();
    while start < end && (bytes[start] == b' ' || bytes[start] == b'\t') {
        start += 1;
    }
    while end > start && (bytes[end - 1] == b' ' || bytes[end - 1] == b'\t') {
        end -= 1;
    }
    &bytes[start..end]
}

fn token_before_semicolon(bytes: &[u8]) -> &[u8] {
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b';' {
            break;
        }
        i += 1;
    }
    trim_ascii_whitespace(&bytes[..i])
}

#[inline]
fn is_json_like_token(token: &[u8]) -> bool {
    if token.eq_ignore_ascii_case(b"application/json") {
        return true;
    }
    // vendor JSON: application/vnd.foo+json
    token.len() >= 5 && token[token.len() - 5..].eq_ignore_ascii_case(b"+json")
}

#[inline]
fn is_multipart_form_data_token(token: &[u8]) -> bool {
    token.eq_ignore_ascii_case(b"multipart/form-data")
}

#[inline]
fn is_form_urlencoded_token(token: &[u8]) -> bool {
    token.eq_ignore_ascii_case(b"application/x-www-form-urlencoded")
}

fn is_valid_content_type_token(token: &[u8]) -> bool {
    // Minimal fast validation:
    // - exactly one '/' separating type and subtype
    // - no whitespace
    // - type and subtype are non-empty
    if token.is_empty() {
        return false;
    }
    let mut slash_pos: Option<usize> = None;
    for (idx, &b) in token.iter().enumerate() {
        if b == b' ' || b == b'\t' {
            return false;
        }
        if b == b'/' {
            if slash_pos.is_some() {
                return false;
            }
            slash_pos = Some(idx);
        }
    }
    match slash_pos {
        None => false,
        Some(0) => false,
        Some(pos) if pos + 1 >= token.len() => false,
        Some(_) => true,
    }
}

fn ascii_contains_ignore_case(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle))
}

/// Fast classification: does this Content-Type represent JSON (application/json or +json)?
pub fn is_json_like(content_type: &HeaderValue) -> bool {
    let token = token_before_semicolon(content_type.as_bytes());
    is_json_like_token(token)
}

/// Fast classification for already-extracted header strings.
///
/// This is used in hot paths where headers are stored as `String` values in `RequestData`.
pub fn is_json_like_str(content_type: &str) -> bool {
    let token = token_before_semicolon(content_type.as_bytes());
    is_json_like_token(token)
}

/// Fast classification: is this Content-Type multipart/form-data?
pub fn is_multipart_form_data(content_type: &HeaderValue) -> bool {
    let token = token_before_semicolon(content_type.as_bytes());
    is_multipart_form_data_token(token)
}

/// Fast classification: is this Content-Type application/x-www-form-urlencoded?
pub fn is_form_urlencoded(content_type: &HeaderValue) -> bool {
    let token = token_before_semicolon(content_type.as_bytes());
    is_form_urlencoded_token(token)
}

/// Classify Content-Type header values after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentTypeKind {
    Json,
    Multipart,
    FormUrlencoded,
    Other,
}

fn multipart_has_boundary(content_type: &HeaderValue) -> bool {
    ascii_contains_ignore_case(content_type.as_bytes(), b"boundary=")
}

fn json_charset_value(content_type: &HeaderValue) -> Option<&[u8]> {
    let bytes = content_type.as_bytes();
    if !ascii_contains_ignore_case(bytes, b"charset=") {
        return None;
    }

    // Extract first charset token after "charset=" up to ';' or whitespace.
    let mut i = 0usize;
    while i + 8 <= bytes.len() {
        if bytes[i..i + 8].eq_ignore_ascii_case(b"charset=") {
            let mut j = i + 8;
            while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\t') {
                j += 1;
            }
            let start = j;
            while j < bytes.len() {
                let b = bytes[j];
                if b == b';' || b == b' ' || b == b'\t' {
                    break;
                }
                j += 1;
            }
            return Some(&bytes[start..j]);
        }
        i += 1;
    }
    None
}

/// Validate that Content-Type is JSON-compatible when route expects JSON
#[allow(clippy::result_large_err)]
pub fn validate_json_content_type(headers: &HeaderMap) -> Result<(), Response> {
    if let Some(content_type_header) = headers.get(axum::http::header::CONTENT_TYPE) {
        if content_type_header.to_str().is_err() {
            return Ok(());
        }

        let token = token_before_semicolon(content_type_header.as_bytes());
        let is_json = is_json_like_token(token);
        let is_form = is_form_urlencoded_token(token) || is_multipart_form_data_token(token);

        if !is_json && !is_form {
            let problem = ProblemDetails::new(
                "https://spikard.dev/errors/unsupported-media-type",
                "Unsupported Media Type",
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
            )
            .with_detail("Unsupported media type");
            let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
            return Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                body,
            )
                .into_response());
        }
    }
    Ok(())
}

/// Validate Content-Length header matches actual body size
#[allow(clippy::result_large_err, clippy::collapsible_if)]
pub fn validate_content_length(headers: &HeaderMap, actual_size: usize) -> Result<(), Response> {
    if let Some(content_length_header) = headers.get(axum::http::header::CONTENT_LENGTH) {
        let Some(declared_length) = parse_ascii_usize(content_length_header.as_bytes()) else {
            return Ok(());
        };
        if declared_length != actual_size {
            let problem = ProblemDetails::new(
                "https://spikard.dev/errors/content-length-mismatch",
                "Content-Length header mismatch",
                StatusCode::BAD_REQUEST,
            )
            .with_detail("Content-Length header does not match actual body size");
            let body = problem.to_json().unwrap_or_else(|_| {
                json!({"error": "Content-Length header does not match actual body size"}).to_string()
            });
            return Err((
                StatusCode::BAD_REQUEST,
                [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                body,
            )
                .into_response());
        }
    }
    Ok(())
}

fn parse_ascii_usize(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() {
        return None;
    }
    let mut value: usize = 0;
    for &b in bytes {
        if !b.is_ascii_digit() {
            return None;
        }
        value = value.saturating_mul(10).saturating_add((b - b'0') as usize);
    }
    Some(value)
}

/// Validate Content-Type header and related requirements
#[allow(clippy::result_large_err)]
pub fn validate_content_type_headers(headers: &HeaderMap, _declared_body_size: usize) -> Result<(), Response> {
    validate_content_type_headers_and_classify(headers, _declared_body_size).map(|_| ())
}

/// Validate Content-Type and return its classification (if present).
#[allow(clippy::result_large_err)]
pub fn validate_content_type_headers_and_classify(
    headers: &HeaderMap,
    _declared_body_size: usize,
) -> Result<Option<ContentTypeKind>, Response> {
    let Some(content_type) = headers.get(axum::http::header::CONTENT_TYPE) else {
        return Ok(None);
    };

    if !content_type.as_bytes().is_ascii() && content_type.to_str().is_err() {
        // Keep legacy behavior: invalid bytes should fail fast.
        let error_body = json!({
            "error": "Invalid Content-Type header"
        });
        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
    }

    let token = token_before_semicolon(content_type.as_bytes());
    if !is_valid_content_type_token(token) {
        let error_body = json!({
            "error": "Invalid Content-Type header"
        });
        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
    }

    let is_json = is_json_like_token(token);
    let is_multipart = is_multipart_form_data_token(token);
    let is_form = is_form_urlencoded_token(token);

    if is_multipart && !multipart_has_boundary(content_type) {
        let error_body = json!({
            "error": "multipart/form-data requires 'boundary' parameter"
        });
        return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
    }

    if is_json
        && let Some(charset) = json_charset_value(content_type)
        && !charset.eq_ignore_ascii_case(b"utf-8")
        && !charset.eq_ignore_ascii_case(b"utf8")
    {
        let charset_str = String::from_utf8_lossy(charset);
        let problem = ProblemDetails::new(
            "https://spikard.dev/errors/unsupported-charset",
            "Unsupported Charset",
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        )
        .with_detail(format!(
            "Unsupported charset '{}' for JSON. Only UTF-8 is supported.",
            charset_str
        ));

        let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
        return Err((
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
            body,
        )
            .into_response());
    }

    let kind = if is_json {
        ContentTypeKind::Json
    } else if is_multipart {
        ContentTypeKind::Multipart
    } else if is_form {
        ContentTypeKind::FormUrlencoded
    } else {
        ContentTypeKind::Other
    };

    Ok(Some(kind))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn validate_content_length_accepts_matching_sizes() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_LENGTH, HeaderValue::from_static("5"));

        assert!(validate_content_length(&headers, 5).is_ok());
    }

    #[test]
    fn validate_content_length_rejects_mismatched_sizes() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_LENGTH, HeaderValue::from_static("10"));

        let err = validate_content_length(&headers, 4).expect_err("expected mismatch");
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            err.headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some(CONTENT_TYPE_PROBLEM_JSON)
        );
    }

    #[test]
    fn test_multipart_without_boundary() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_multipart_with_boundary() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data; boundary=----WebKitFormBoundary"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_with_utf16_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-16"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_with_utf8_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_without_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vendor_json_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_problem_json_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vendor_json_with_utf16_charset_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json; charset=utf-16"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_vendor_json_with_utf8_charset_accepted() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/vnd.api+json; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_json_content_type() {
        let mime = "application/json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/vnd.api+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/problem+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "application/hal+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime));

        let mime = "text/plain".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/xml".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));

        let mime = "application/x-www-form-urlencoded".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime));
    }

    #[test]
    fn test_json_with_utf8_uppercase_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=UTF-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "UTF-8 in uppercase should be accepted");
    }

    #[test]
    fn test_json_with_utf8_no_hyphen_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "utf8 without hyphen should be accepted");
    }

    #[test]
    fn test_json_with_iso88591_charset_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=iso-8859-1"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err(), "iso-8859-1 should be rejected for JSON");
    }

    #[test]
    fn test_json_with_utf32_charset_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-32"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err(), "UTF-32 should be rejected for JSON");
    }

    #[test]
    fn test_multipart_with_boundary_and_charset() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data; boundary=abc123; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(
            result.is_ok(),
            "multipart with boundary should accept charset parameter"
        );
    }

    #[test]
    fn test_validate_content_length_no_header() {
        let headers = HeaderMap::new();

        let result = validate_content_length(&headers, 1024);
        assert!(result.is_ok(), "Missing Content-Length header should pass");
    }

    #[test]
    fn test_validate_content_length_zero_bytes() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_LENGTH, HeaderValue::from_static("0"));

        assert!(validate_content_length(&headers, 0).is_ok());
    }

    #[test]
    fn test_validate_content_length_large_body() {
        let mut headers = HeaderMap::new();
        let large_size = 1024 * 1024 * 100;
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            HeaderValue::from_str(&large_size.to_string()).unwrap(),
        );

        assert!(validate_content_length(&headers, large_size).is_ok());
    }

    #[test]
    fn test_validate_content_length_invalid_header_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_LENGTH,
            HeaderValue::from_static("not-a-number"),
        );

        let result = validate_content_length(&headers, 100);
        assert!(
            result.is_ok(),
            "Invalid Content-Length format should be skipped gracefully"
        );
    }

    #[test]
    fn test_invalid_content_type_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("not/a/valid/type"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_err(), "Invalid mime type format should be rejected");
    }

    #[test]
    fn test_unsupported_content_type_xml() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/xml"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(
            result.is_ok(),
            "XML should pass header validation (routing layer rejects if needed)"
        );
    }

    #[test]
    fn test_unsupported_content_type_plain_text() {
        let mut headers = HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Plain text should pass header validation");
    }

    #[test]
    fn test_content_type_with_boundary_missing_boundary_param() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data; charset=utf-8"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(
            result.is_err(),
            "multipart/form-data without boundary parameter should be rejected"
        );
    }

    #[test]
    fn test_content_type_form_urlencoded() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "form-urlencoded should be accepted");
    }

    #[test]
    fn test_is_json_content_type_with_hal_json() {
        let mime = "application/hal+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime), "HAL+JSON should be recognized as JSON");
    }

    #[test]
    fn test_is_json_content_type_with_ld_json() {
        let mime = "application/ld+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime), "LD+JSON should be recognized as JSON");
    }

    #[test]
    fn test_is_json_content_type_rejects_json_patch() {
        let mime = "application/json-patch+json".parse::<mime::Mime>().unwrap();
        assert!(is_json_content_type(&mime), "JSON-Patch should be recognized as JSON");
    }

    #[test]
    fn test_is_json_content_type_rejects_html() {
        let mime = "text/html".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime), "HTML should not be JSON");
    }

    #[test]
    fn test_is_json_content_type_rejects_csv() {
        let mime = "text/csv".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime), "CSV should not be JSON");
    }

    #[test]
    fn test_is_json_content_type_rejects_image_png() {
        let mime = "image/png".parse::<mime::Mime>().unwrap();
        assert!(!is_json_content_type(&mime), "PNG should not be JSON");
    }

    #[test]
    fn test_validate_json_content_type_missing_header() {
        let headers = HeaderMap::new();
        let result = validate_json_content_type(&headers);
        assert!(
            result.is_ok(),
            "Missing Content-Type for JSON route should be OK (routing layer handles)"
        );
    }

    #[test]
    fn test_validate_json_content_type_accepts_form_urlencoded() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let result = validate_json_content_type(&headers);
        assert!(result.is_ok(), "Form-urlencoded should be accepted for JSON routes");
    }

    #[test]
    fn test_validate_json_content_type_accepts_multipart() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("multipart/form-data; boundary=abc123"),
        );

        let result = validate_json_content_type(&headers);
        assert!(result.is_ok(), "Multipart should be accepted for JSON routes");
    }

    #[test]
    fn test_validate_json_content_type_rejects_xml() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/xml"),
        );

        let result = validate_json_content_type(&headers);
        assert!(result.is_err(), "XML should be rejected for JSON-expecting routes");
        assert_eq!(
            result.unwrap_err().status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Should return 415 Unsupported Media Type"
        );
    }

    #[test]
    fn test_content_type_with_multiple_parameters() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8; boundary=xyz"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Multiple parameters should be parsed correctly");
    }

    #[test]
    fn test_content_type_with_quoted_parameter() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static(r#"multipart/form-data; boundary="----WebKitFormBoundary""#),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Quoted boundary parameter should be handled");
    }

    #[test]
    fn test_content_type_case_insensitive_type() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("Application/JSON"),
        );

        let result = validate_content_type_headers(&headers, 0);
        assert!(result.is_ok(), "Content-Type type/subtype should be case-insensitive");
    }
}
