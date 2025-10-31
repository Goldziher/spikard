//! Tests for path_params fixtures
//! Generated from: testing_data/path_params

#[cfg(test)]
mod path_params {

    #[tokio::test]
    async fn test_path_params_22_uuid_v4_rejects_v3() {
        // Fixture: 22_uuid_v4_rejects_v3
        // Description: Path parameter expecting UUID v4 should reject UUID v3
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/22_uuid_v4_rejects_v3.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/e8b5a51d-11c8-3310-a6ab-367563f20686".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_boolean_path_parameter_true() {
        // Fixture: Boolean path parameter - True
        // Description: Tests boolean path parameter with 'True' string value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/13_bool_path_param_true.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/bool/True".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_29_decimal_path_param_success() {
        // Fixture: 29_decimal_path_param_success
        // Description: Path parameter with decimal/money value should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/29_decimal_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/prices/19.99".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success() {
        // Fixture: Integer path parameter with combined lt and gt constraints - success
        // Description: Tests integer path parameter with both lt and gt validation (range check)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/18_int_validation_combined_lt_gt.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-lt-gt/2".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_33_string_pattern_path_success() {
        // Fixture: 33_string_pattern_path_success
        // Description: Path parameter matching regex pattern should succeed
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/33_string_pattern_path_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/repos/spikard-labs/spikard-http".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_31_string_minlength_path_failure() {
        // Fixture: 31_string_minlength_path_failure
        // Description: Path parameter with string below minLength constraint should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/31_string_minlength_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/users/ab".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_35_negative_integer_path_param() {
        // Fixture: 35_negative_integer_path_param
        // Description: Path parameter with negative integer should be parsed correctly
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/35_negative_integer_path_param.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/offset/-100".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_enum_path_parameter_invalid_value() {
        // Fixture: Enum path parameter - invalid value
        // Description: Tests enum path parameter with invalid enum value returns 422
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/10_enum_path_param_invalid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/models/foo".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_27_datetime_format_path_param_success() {
        // Fixture: 27_datetime_format_path_param_success
        // Description: Path parameter with valid ISO 8601 datetime should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/27_datetime_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/bookings/2025-10-30T14:30:00Z".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_25_date_format_invalid_failure() {
        // Fixture: 25_date_format_invalid_failure
        // Description: Path parameter with invalid date format should fail validation
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/25_date_format_invalid_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/events/2025-13-45".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_lt_constraint_success() {
        // Fixture: Integer path parameter with lt constraint - success
        // Description: Tests integer path parameter with lt (less than) validation
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/17_int_validation_lt_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-lt/2".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_with_gt_constraint_success() {
        // Fixture: Integer path parameter with gt constraint - success
        // Description: Tests integer path parameter with gt (greater than) validation succeeds
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/06_int_validation_gt_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-gt/42".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_28_duration_format_path_param_success() {
        // Fixture: 28_duration_format_path_param_success
        // Description: Path parameter with valid ISO 8601 duration should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/28_duration_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/delays/P1DT2H30M".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_20_uuid_v3_path_param_success() {
        // Fixture: 20_uuid_v3_path_param_success
        // Description: Path parameter with valid UUID v3 should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/20_uuid_v3_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/e8b5a51d-11c8-3310-a6ab-367563f20686".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_invalid_string() {
        // Fixture: Integer path parameter - invalid string
        // Description: Tests integer path parameter with non-numeric string returns 422
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/03_int_path_param_invalid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/int/foobar".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_30_string_minlength_path_success() {
        // Fixture: 30_string_minlength_path_success
        // Description: Path parameter with string meeting minLength constraint should succeed
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/30_string_minlength_path_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/users/alice".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_with_le_constraint_success() {
        // Fixture: Integer path parameter with le constraint - success
        // Description: Tests integer path parameter with le (less than or equal) validation at boundary
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/16_int_validation_le_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-le/3".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_path_type_parameter_file_path() {
        // Fixture: Path type parameter - file path
        // Description: Tests path type parameter that captures remaining path segments
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/11_path_type_file_path.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/files/home/johndoe/myfile.txt".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_32_string_maxlength_path_failure() {
        // Fixture: 32_string_maxlength_path_failure
        // Description: Path parameter with string exceeding maxLength constraint should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/32_string_maxlength_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/users/this_username_is_way_too_long_to_be_valid".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_success() {
        // Fixture: Integer path parameter - success
        // Description: Tests integer path parameter with valid value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/02_int_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/int/42".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_34_string_pattern_path_failure() {
        // Fixture: 34_string_pattern_path_failure
        // Description: Path parameter not matching regex pattern should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/34_string_pattern_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/repos/invalid@owner".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_21_uuid_v5_path_param_success() {
        // Fixture: 21_uuid_v5_path_param_success
        // Description: Path parameter with valid UUID v5 should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/21_uuid_v5_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/630eb68f-e0fa-5ecc-887a-7c7a62614681".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_string_path_parameter_with_max_length_failure() {
        // Fixture: String path parameter with max_length - failure
        // Description: Tests string path parameter with max_length validation fails when too long
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/19_string_maxlength_validation_fail.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-maxlength/foobar".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_string_path_parameter_with_min_length_failure() {
        // Fixture: String path parameter with min_length - failure
        // Description: Tests string path parameter with min_length validation fails
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/08_string_minlength_validation_fail.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-minlength/fo".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_multiple_path_parameters_success() {
        // Fixture: Multiple path parameters - success
        // Description: Tests multiple path parameters in single route
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/12_multiple_path_params.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_date_path_parameter_success() {
        // Fixture: Date path parameter - success
        // Description: Tests date path parameter with ISO format date
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/20_date_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/date/2023-07-15".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_with_gt_constraint_failure() {
        // Fixture: Integer path parameter with gt constraint - failure
        // Description: Tests integer path parameter with gt validation fails when value too small
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/07_int_validation_gt_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-gt/2".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_24_date_format_path_param_success() {
        // Fixture: 24_date_format_path_param_success
        // Description: Path parameter with valid ISO date format should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/24_date_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/events/2025-10-30".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_float_path_parameter_success() {
        // Fixture: Float path parameter - success
        // Description: Tests float path parameter with valid value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/04_float_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/float/42.5".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_string_path_parameter_success() {
        // Fixture: String path parameter - success
        // Description: Tests string path parameter with valid value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/01_string_path_param.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/str/foobar".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_26_time_format_path_param_success() {
        // Fixture: 26_time_format_path_param_success
        // Description: Path parameter with valid time format should be accepted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/26_time_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/schedule/14:30:00".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_uuid_path_parameter_success() {
        // Fixture: UUID path parameter - success
        // Description: Tests UUID path parameter with valid UUID format
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/05_uuid_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_integer_path_parameter_with_ge_constraint_success() {
        // Fixture: Integer path parameter with ge constraint - success
        // Description: Tests integer path parameter with ge (greater than or equal) validation at boundary
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/15_int_validation_ge_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/param-ge/3".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_enum_path_parameter_success() {
        // Fixture: Enum path parameter - success
        // Description: Tests enum path parameter with valid enum value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/09_enum_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/models/alexnet".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
    async fn test_path_params_23_uuid_v4_rejects_v5() {
        // Fixture: 23_uuid_v4_rejects_v5
        // Description: Path parameter expecting UUID v4 should reject UUID v5
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/23_uuid_v4_rejects_v5.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/items/630eb68f-e0fa-5ecc-887a-7c7a62614681".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
            StatusCode::from_u16(422).unwrap(),
            "Expected status 422, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_path_params_boolean_path_parameter_numeric_1() {
        // Fixture: Boolean path parameter - numeric 1
        // Description: Tests boolean path parameter with '1' converts to true
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/14_bool_path_param_1.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app
        let app = spikard_e2e_app::create_app();

        // Build request
        let mut uri = "/path/bool/1".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

            // Define the query component encoding set (RFC 3986)
            // Encode: space, ", #, <, >, %, {, }, |, \\, ^, `, and control characters
            // Plus query-specific: &, =, +
            const QUERY: &AsciiSet = &CONTROLS
                .add(b' ')
                .add(b'"')
                .add(b'#')
                .add(b'<')
                .add(b'>')
                .add(b'%')
                .add(b'{')
                .add(b'}')
                .add(b'|')
                .add(b'\\')
                .add(b'^')
                .add(b'`')
                .add(b'&')
                .add(b'=')
                .add(b'+');

            let query_string = query_params
                .iter()
                .flat_map(|(k, v)| {
                    let key = percent_encode(k.as_bytes(), QUERY).to_string();
                    match v {
                        Value::String(s) => {
                            let value = percent_encode(s.as_bytes(), QUERY).to_string();
                            vec![format!("{}={}", key, value)]
                        }
                        Value::Number(n) => vec![format!("{}={}", key, n)],
                        Value::Bool(b) => vec![format!("{}={}", key, b)],
                        Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .filter_map(|item| match item {
                                    Value::String(s) => {
                                        Some(format!("{}={}", key, percent_encode(s.as_bytes(), QUERY)))
                                    }
                                    Value::Number(n) => Some(format!("{}={}", key, n)),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        }
                        _ => vec![],
                    }
                })
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
}
