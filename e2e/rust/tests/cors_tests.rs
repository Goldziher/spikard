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

        // Build request with optional body
        let mut request_builder = Request::builder().method("OPTIONS").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("OPTIONS").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("OPTIONS").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("GET").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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

        // Build request with optional body
        let mut request_builder = Request::builder().method("OPTIONS").uri(uri);

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
            use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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
            StatusCode::from_u16(403).unwrap(),
            "Expected status 403, got {:?}",
            response.status()
        );
    }
}
