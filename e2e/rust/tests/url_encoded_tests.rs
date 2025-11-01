//! Tests for url_encoded fixtures
//! Generated from: testing_data/url_encoded

#[cfg(test)]
mod url_encoded {

    #[tokio::test]
    async fn test_url_encoded_simple_form_submission_success() {
        // Fixture: Simple form submission - success
        // Description: Tests basic URL-encoded form with username and password
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/01_simple_form_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Simple_form_submission___success();

        // Build request
        let mut uri = "/login/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_15_special_characters_field_names() {
        // Fixture: 15_special_characters_field_names
        // Description: URL-encoded form with special characters in field names should be handled correctly
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/15_special_characters_field_names.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_15_special_characters_field_names();

        // Build request
        let mut uri = "/data".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_pattern_validation_fail() {
        // Fixture: Pattern validation - fail
        // Description: Tests form field with regex pattern constraint failure
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/07_pattern_validation_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Pattern_validation___fail();

        // Build request
        let mut uri = "/form/validated".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_22_additional_properties_strict_failure() {
        // Fixture: 22_additional_properties_strict_failure
        // Description: URL-encoded form with extra fields when additionalProperties is false should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/22_additional_properties_strict_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_22_additional_properties_strict_failure();

        // Build request
        let mut uri = "/settings".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_17_pattern_validation_failure() {
        // Fixture: 17_pattern_validation_failure
        // Description: URL-encoded form field violating regex pattern should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/17_pattern_validation_failure.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_17_pattern_validation_failure();

        // Build request
        let mut uri = "/accounts".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_20_format_email_validation_failure() {
        // Fixture: 20_format_email_validation_failure
        // Description: URL-encoded form with invalid email format should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/20_format_email_validation_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_20_format_email_validation_failure();

        // Build request
        let mut uri = "/subscribe".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_multiple_values_for_same_field() {
        // Fixture: Multiple values for same field
        // Description: Tests form field with multiple values (array)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/08_multiple_values_same_field.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Multiple_values_for_same_field();

        // Build request
        let mut uri = "/form/tags".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_required_field_missing_validation_error() {
        // Fixture: Required field missing - validation error
        // Description: Tests validation error when required form field is missing
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/02_required_field_missing.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Required_field_missing___validation_error();

        // Build request
        let mut uri = "/login/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_13_array_field_success() {
        // Fixture: 13_array_field_success
        // Description: URL-encoded form with array field using bracket notation should parse correctly
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/13_array_field_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_13_array_field_success();

        // Build request
        let mut uri = "/register".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_numeric_field_type_conversion() {
        // Fixture: Numeric field type conversion
        // Description: Tests conversion of form string value to numeric type
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/11_numeric_field_conversion.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Numeric_field_type_conversion();

        // Build request
        let mut uri = "/form/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_special_characters_encoding() {
        // Fixture: Special characters encoding
        // Description: Tests URL encoding of special characters in form data
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/04_special_chars_encoding.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Special_characters_encoding();

        // Build request
        let mut uri = "/form/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_boolean_field_conversion() {
        // Fixture: Boolean field conversion
        // Description: Tests conversion of form string value to boolean
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/12_boolean_field_conversion.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Boolean_field_conversion();

        // Build request
        let mut uri = "/form/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_empty_string_value() {
        // Fixture: Empty string value
        // Description: Tests form field with empty string value
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/09_empty_string_value.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Empty_string_value();

        // Build request
        let mut uri = "/form/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_oauth2_password_grant_flow() {
        // Fixture: OAuth2 password grant flow
        // Description: Tests OAuth2 password grant with form data
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/10_oauth_password_grant.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_OAuth2_password_grant_flow();

        // Build request
        let mut uri = "/token".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_19_array_minitems_validation_failure() {
        // Fixture: 19_array_minitems_validation_failure
        // Description: URL-encoded array with fewer items than minItems should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/19_array_minitems_validation_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_19_array_minitems_validation_failure();

        // Build request
        let mut uri = "/tags".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_optional_field_missing_success() {
        // Fixture: Optional field missing - success
        // Description: Tests form with optional field omitted
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/03_optional_field_missing.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_Optional_field_missing___success();

        // Build request
        let mut uri = "/register/".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_14_nested_object_bracket_notation() {
        // Fixture: 14_nested_object_bracket_notation
        // Description: URL-encoded form with nested object using bracket notation should parse correctly
        // Expected status: 201

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/14_nested_object_bracket_notation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_14_nested_object_bracket_notation();

        // Build request
        let mut uri = "/profile".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_string_max_length_validation_fail() {
        // Fixture: String max_length validation - fail
        // Description: Tests form field with max_length constraint failure
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/06_maxlength_validation_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_String_max_length_validation___fail();

        // Build request
        let mut uri = "/form/validated".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_18_integer_minimum_validation_failure() {
        // Fixture: 18_integer_minimum_validation_failure
        // Description: URL-encoded integer field below minimum should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/18_integer_minimum_validation_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_18_integer_minimum_validation_failure();

        // Build request
        let mut uri = "/products".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_21_integer_type_coercion_failure() {
        // Fixture: 21_integer_type_coercion_failure
        // Description: URL-encoded form with non-numeric value for integer field should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/21_integer_type_coercion_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_21_integer_type_coercion_failure();

        // Build request
        let mut uri = "/products".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_16_minlength_validation_failure() {
        // Fixture: 16_minlength_validation_failure
        // Description: URL-encoded form field violating minLength constraint should fail
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/url_encoded/16_minlength_validation_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_16_minlength_validation_failure();

        // Build request
        let mut uri = "/users".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
    async fn test_url_encoded_string_min_length_validation_fail() {
        // Fixture: String min_length validation - fail
        // Description: Tests form field with min_length constraint failure
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/url_encoded/05_minlength_validation_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_url_encoded_String_min_length_validation___fail();

        // Build request
        let mut uri = "/form/validated".to_string();

        // Use query_string if provided (for exact encoding control), otherwise build from query_params
        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("POST").uri(uri);

        // Add headers from fixture if present
        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

        // Add cookies from fixture if present
        if let Some(cookies) = fixture["request"]["cookies"].as_object() {
            let cookie_header: Vec<String> = cookies
                .iter()
                .map(|(name, value)| {
                    if let Some(value_str) = value.as_str() {
                        format!("{}={}", name, value_str)
                    } else {
                        format!("{}={}", name, value)
                    }
                })
                .collect();
            if !cookie_header.is_empty() {
                request_builder = request_builder.header("cookie", cookie_header.join("; "));
            }
        }

        // Add body if present in fixture
        let body = if let Some(request_body) = fixture["request"]["body"].as_str() {
            // Body is already encoded as a string (e.g., URL-encoded form data)
            // Don't override Content-Type if already set
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            // Body is a JSON object, encode it
            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            // Handle URL-encoded form data
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
                            // For arrays, repeat the key for each value
                            arr.iter()
                                .map(|item| {
                                    let encoded_value = match item {
                                        serde_json::Value::String(s) => {
                                            percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string()
                                        }
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::Bool(b) => b.to_string(),
                                        _ => percent_encode(item.to_string().as_bytes(), NON_ALPHANUMERIC).to_string(),
                                    };
                                    format!("{}={}", key, encoded_value)
                                })
                                .collect::<Vec<_>>()
                        }
                        serde_json::Value::String(s) => {
                            let encoded_value = percent_encode(s.as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                        serde_json::Value::Number(n) => {
                            vec![format!("{}={}", key, n)]
                        }
                        serde_json::Value::Bool(b) => {
                            vec![format!("{}={}", key, b)]
                        }
                        _ => {
                            let encoded_value =
                                percent_encode(value.to_string().as_bytes(), NON_ALPHANUMERIC).to_string();
                            vec![format!("{}={}", key, encoded_value)]
                        }
                    }
                })
                .collect();

            let body_str = form_params.join("&");

            // Only add content-type header if not already set by fixture headers
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

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
}
