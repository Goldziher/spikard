//! Tests for multipart fixtures
//! Generated from: testing_data/multipart

#[cfg(test)]
mod multipart {

    #[tokio::test]
    async fn test_multipart_multiple_values_for_same_field_name() {
        // Fixture: Multiple values for same field name
        // Description: Multiple files uploaded with the same field name (array-like behavior)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/07_multiple_values_same_field.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_19_file_mime_spoofing_png_as_jpeg() {
        // Fixture: 19_file_mime_spoofing_png_as_jpeg
        // Description: File with PNG magic number but JPEG MIME type should be rejected (spoofing detection)
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/19_file_mime_spoofing_png_as_jpeg.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_20_file_mime_spoofing_jpeg_as_png() {
        // Fixture: 20_file_mime_spoofing_jpeg_as_png
        // Description: File with JPEG magic number but PNG MIME type should be rejected (spoofing detection)
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/20_file_mime_spoofing_jpeg_as_png.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_21_file_pdf_magic_number_success() {
        // Fixture: 21_file_pdf_magic_number_success
        // Description: File with correct PDF magic number should be accepted
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/21_file_pdf_magic_number_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(201).unwrap(),
            "Expected status 201, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_content_type_validation_invalid_type() {
        // Fixture: Content-Type validation - invalid type
        // Description: Tests file upload with disallowed content type
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/15_content_type_validation_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/images-only".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_pdf_file_upload() {
        // Fixture: PDF file upload
        // Description: Tests uploading a PDF document
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/13_pdf_file_upload.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/document".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_file_list_upload_array_of_files() {
        // Fixture: File list upload (array of files)
        // Description: Tests uploading multiple files as a list parameter
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/10_file_list_upload.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/list".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_optional_file_upload_provided() {
        // Fixture: Optional file upload - provided
        // Description: Tests optional file parameter when file is provided
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/09_optional_file_upload_provided.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/optional".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_file_size_validation_too_large() {
        // Fixture: File size validation - too large
        // Description: Tests file upload exceeding max size limit
        // Expected status: 413

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/14_file_size_validation_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/validated".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(413).unwrap(),
            "Expected status 413, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_mixed_files_and_form_data() {
        // Fixture: Mixed files and form data
        // Description: Multipart request with both file uploads and regular form fields
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/04_mixed_files_and_data.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_simple_file_upload() {
        // Fixture: Simple file upload
        // Description: Single file upload with text/plain content type
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/01_simple_file_upload.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_empty_file_upload() {
        // Fixture: Empty file upload
        // Description: Tests uploading a file with zero bytes
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/11_empty_file_upload.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_optional_file_upload_missing() {
        // Fixture: Optional file upload - missing
        // Description: Tests optional file parameter when no file is provided
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/08_optional_file_upload_missing.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/optional".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_file_upload_without_filename() {
        // Fixture: File upload without filename
        // Description: Upload file content without providing a filename
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/06_file_without_filename.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_18_file_magic_number_jpeg_success() {
        // Fixture: 18_file_magic_number_jpeg_success
        // Description: File with correct JPEG magic number and matching MIME type should be accepted
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/18_file_magic_number_jpeg_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(201).unwrap(),
            "Expected status 201, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_22_file_empty_buffer() {
        // Fixture: 22_file_empty_buffer
        // Description: File with empty buffer should fail validation
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/22_file_empty_buffer.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_17_file_magic_number_png_success() {
        // Fixture: 17_file_magic_number_png_success
        // Description: File with correct PNG magic number and matching MIME type should be accepted
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/multipart/17_file_magic_number_png_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/upload".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(201).unwrap(),
            "Expected status 201, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_form_data_without_files() {
        // Fixture: Form data without files
        // Description: Multipart form with only text fields, no file uploads
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/02_form_data_only.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_multiple_file_uploads() {
        // Fixture: Multiple file uploads
        // Description: Upload multiple files in a single multipart request
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/03_multiple_files.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_file_upload_with_custom_headers() {
        // Fixture: File upload with custom headers
        // Description: File upload with additional custom headers in the multipart section
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/05_file_with_custom_headers.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_required_file_upload_missing() {
        // Fixture: Required file upload - missing
        // Description: Tests required file parameter when no file is provided
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/16_required_file_missing.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/required".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_multipart_image_file_upload() {
        // Fixture: Image file upload
        // Description: Tests uploading an image file (JPEG)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/multipart/12_image_file_upload.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/image".to_string();

        if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v.as_str().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("&");
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(&query_string);
            }
        }

        let request = Request::builder().method("POST").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }
}
