//! JSON schema validation middleware

use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use spikard_core::problem::{CONTENT_TYPE_PROBLEM_JSON, ProblemDetails};

/// Check if a media type is JSON or has a +json suffix
pub fn is_json_content_type(mime: &mime::Mime) -> bool {
    (mime.type_() == mime::APPLICATION && mime.subtype() == mime::JSON) || mime.suffix() == Some(mime::JSON)
}

/// Validate that Content-Type is JSON-compatible when route expects JSON
#[allow(clippy::result_large_err)]
pub fn validate_json_content_type(headers: &HeaderMap) -> Result<(), Response> {
    if let Some(content_type_header) = headers.get(axum::http::header::CONTENT_TYPE)
        && let Ok(content_type_str) = content_type_header.to_str()
        && let Ok(parsed_mime) = content_type_str.parse::<mime::Mime>()
    {
        let is_json = (parsed_mime.type_() == mime::APPLICATION && parsed_mime.subtype() == mime::JSON)
            || parsed_mime.suffix() == Some(mime::JSON);

        let is_form = (parsed_mime.type_() == mime::APPLICATION && parsed_mime.subtype() == "x-www-form-urlencoded")
            || (parsed_mime.type_() == mime::MULTIPART && parsed_mime.subtype() == "form-data");

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
        if let Ok(content_length_str) = content_length_header.to_str() {
            if let Ok(declared_length) = content_length_str.parse::<usize>() {
                if declared_length != actual_size {
                    let problem = ProblemDetails::bad_request(format!(
                        "Content-Length header ({}) does not match actual body size ({})",
                        declared_length, actual_size
                    ));

                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((
                        StatusCode::BAD_REQUEST,
                        [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                        body,
                    )
                        .into_response());
                }
            }
        }
    }
    Ok(())
}

/// Validate Content-Type header and related requirements
#[allow(clippy::result_large_err)]
pub fn validate_content_type_headers(headers: &HeaderMap, _declared_body_size: usize) -> Result<(), Response> {
    if let Some(content_type_str) = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
    {
        let parsed_mime = match content_type_str.parse::<mime::Mime>() {
            Ok(m) => m,
            Err(_) => {
                let error_body = json!({
                    "error": format!("Invalid Content-Type header: {}", content_type_str)
                });
                return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
            }
        };

        let is_json = is_json_content_type(&parsed_mime);
        let is_multipart = parsed_mime.type_() == mime::MULTIPART && parsed_mime.subtype() == "form-data";

        if is_multipart && parsed_mime.get_param(mime::BOUNDARY).is_none() {
            let error_body = json!({
                "error": "multipart/form-data requires 'boundary' parameter"
            });
            return Err((StatusCode::BAD_REQUEST, axum::Json(error_body)).into_response());
        }

        #[allow(clippy::collapsible_if)]
        if is_json {
            if let Some(charset) = parsed_mime.get_param(mime::CHARSET).map(|c| c.as_str()) {
                if !charset.eq_ignore_ascii_case("utf-8") && !charset.eq_ignore_ascii_case("utf8") {
                    let problem = ProblemDetails::new(
                        "https://spikard.dev/errors/unsupported-charset",
                        "Unsupported Charset",
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    )
                    .with_detail(format!(
                        "Unsupported charset '{}' for JSON. Only UTF-8 is supported.",
                        charset
                    ));

                    let body = problem.to_json().unwrap_or_else(|_| "{}".to_string());
                    return Err((
                        StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        [(axum::http::header::CONTENT_TYPE, CONTENT_TYPE_PROBLEM_JSON)],
                        body,
                    )
                        .into_response());
                }
            }
        }
    }

    Ok(())
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
