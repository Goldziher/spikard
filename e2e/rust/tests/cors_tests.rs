//! Tests for cors fixtures
//! Generated from: testing_data/cors

#[cfg(test)]
mod cors {

    #[tokio::test]
    async fn test_cors_07_cors_preflight_header_not_allowed() {
        // Fixture: 07_cors_preflight_header_not_allowed
        // Description: CORS preflight request with non-allowed header should be rejected
        // Expected status: 403

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/07_cors_preflight_header_not_allowed.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/data".to_string();

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

        let request = Request::builder()
            .method("OPTIONS")
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(403).unwrap(),
            "Expected status 403, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_cors_cors_preflight_request() {
        // Fixture: CORS preflight request
        // Description: Tests OPTIONS preflight request for CORS
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/02_preflight_request.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/".to_string();

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

        let request = Request::builder()
            .method("OPTIONS")
            .uri(uri)
            .body(Body::empty())
            .unwrap();

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
    async fn test_cors_cors_with_credentials() {
        // Fixture: CORS with credentials
        // Description: Tests CORS request with credentials (cookies, auth headers)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/03_cors_with_credentials.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/user/profile".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

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
    async fn test_cors_08_cors_max_age() {
        // Fixture: 08_cors_max_age
        // Description: CORS preflight response should include Access-Control-Max-Age
        // Expected status: 204

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/08_cors_max_age.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/data".to_string();

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

        let request = Request::builder()
            .method("OPTIONS")
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(204).unwrap(),
            "Expected status 204, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_cors_10_cors_origin_null() {
        // Fixture: 10_cors_origin_null
        // Description: CORS request with 'null' origin should be handled according to policy
        // Expected status: 403

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/10_cors_origin_null.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/data".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(403).unwrap(),
            "Expected status 403, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_cors_cors_wildcard_origin() {
        // Fixture: CORS wildcard origin
        // Description: Tests CORS with wildcard allowing all origins
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/04_cors_wildcard.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/public/data".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

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
    async fn test_cors_cors_request_blocked() {
        // Fixture: CORS request blocked
        // Description: Tests CORS request from disallowed origin
        // Expected status: 403

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/05_cors_blocked.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(403).unwrap(),
            "Expected status 403, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_cors_simple_cors_request() {
        // Fixture: Simple CORS request
        // Description: Tests simple CORS request with Origin header
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/01_simple_cors_request.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

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
    async fn test_cors_09_cors_expose_headers() {
        // Fixture: 09_cors_expose_headers
        // Description: CORS response should include Access-Control-Expose-Headers for custom headers
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/09_cors_expose_headers.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/data".to_string();

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

        let request = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();

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
    async fn test_cors_06_cors_preflight_method_not_allowed() {
        // Fixture: 06_cors_preflight_method_not_allowed
        // Description: CORS preflight request for non-allowed method should be rejected
        // Expected status: 403

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/cors/06_cors_preflight_method_not_allowed.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/api/data".to_string();

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

        let request = Request::builder()
            .method("OPTIONS")
            .uri(uri)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();

        // Assert status code
        assert_eq!(
            response.status(),
            StatusCode::from_u16(403).unwrap(),
            "Expected status 403, got {:?}",
            response.status()
        );
    }
}
