//! Tests for path_params fixtures
//! Generated from: testing_data/path_params

#[cfg(test)]
mod path_params {
    use axum::body::Body;
    use axum::http::Request;
    use axum_test::TestServer;
    use serde_json::Value;
    use spikard_http::testing::{call_test_server, snapshot_response};

    #[tokio::test]
    async fn test_path_params_20_uuid_v3_path_param_success() {
        // Fixture: 20_uuid_v3_path_param_success
        // Description: Path parameter with valid UUID v3 should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/20_uuid_v3_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_20_uuid_v3_path_param_success();

        // Build request
        let mut uri = "/items/e8b5a51d-11c8-3310-a6ab-367563f20686".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_21_uuid_v5_path_param_success() {
        // Fixture: 21_uuid_v5_path_param_success
        // Description: Path parameter with valid UUID v5 should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/21_uuid_v5_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_21_uuid_v5_path_param_success();

        // Build request
        let mut uri = "/items/630eb68f-e0fa-5ecc-887a-7c7a62614681".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_24_date_format_path_param_success() {
        // Fixture: 24_date_format_path_param_success
        // Description: Path parameter with valid ISO date format should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/24_date_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_24_date_format_path_param_success();

        // Build request
        let mut uri = "/events/2025-10-30".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_25_date_format_invalid_failure() {
        // Fixture: 25_date_format_invalid_failure
        // Description: Path parameter with invalid date format should fail validation
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/25_date_format_invalid_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_25_date_format_invalid_failure();

        // Build request
        let mut uri = "/events/2025-13-45".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_27_datetime_format_path_param_success() {
        // Fixture: 27_datetime_format_path_param_success
        // Description: Path parameter with valid ISO 8601 datetime should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/27_datetime_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_27_datetime_format_path_param_success();

        // Build request
        let mut uri = "/bookings/2025-10-30T14:30:00Z".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_28_duration_format_path_param_success() {
        // Fixture: 28_duration_format_path_param_success
        // Description: Path parameter with valid ISO 8601 duration should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/28_duration_format_path_param_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_28_duration_format_path_param_success();

        // Build request
        let mut uri = "/delays/P1DT2H30M".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_29_decimal_path_param_success() {
        // Fixture: 29_decimal_path_param_success
        // Description: Path parameter with decimal/money value should be accepted
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/29_decimal_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_29_decimal_path_param_success();

        // Build request
        let mut uri = "/prices/19.99".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_30_string_minlength_path_success() {
        // Fixture: 30_string_minlength_path_success
        // Description: Path parameter with string meeting minLength constraint should succeed
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/30_string_minlength_path_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_30_string_minlength_path_success();

        // Build request
        let mut uri = "/users/alice".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_31_string_minlength_path_failure() {
        // Fixture: 31_string_minlength_path_failure
        // Description: Path parameter with string below minLength constraint should fail
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/31_string_minlength_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_31_string_minlength_path_failure();

        // Build request
        let mut uri = "/users/ab".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_32_string_maxlength_path_failure() {
        // Fixture: 32_string_maxlength_path_failure
        // Description: Path parameter with string exceeding maxLength constraint should fail
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/32_string_maxlength_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_32_string_maxlength_path_failure();

        // Build request
        let mut uri = "/users/this_username_is_way_too_long_to_be_valid".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_33_string_pattern_path_success() {
        // Fixture: 33_string_pattern_path_success
        // Description: Path parameter matching regex pattern should succeed
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/33_string_pattern_path_success.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_33_string_pattern_path_success();

        // Build request
        let mut uri = "/repos/spikard-labs/spikard-http".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_34_string_pattern_path_failure() {
        // Fixture: 34_string_pattern_path_failure
        // Description: Path parameter not matching regex pattern should fail
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/34_string_pattern_path_failure.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_34_string_pattern_path_failure();

        // Build request
        let mut uri = "/repos/invalid@owner".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_35_negative_integer_path_param() {
        // Fixture: 35_negative_integer_path_param
        // Description: Path parameter with negative integer should be parsed correctly
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/35_negative_integer_path_param.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_35_negative_integer_path_param();

        // Build request
        let mut uri = "/offset/-100".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_boolean_path_parameter_true() {
        // Fixture: Boolean path parameter - True
        // Description: Tests boolean path parameter with 'True' string value
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/13_bool_path_param_true.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_boolean_path_parameter_true();

        // Build request
        let mut uri = "/path/bool/True".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_boolean_path_parameter_numeric_1() {
        // Fixture: Boolean path parameter - numeric 1
        // Description: Tests boolean path parameter with '1' converts to true
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/14_bool_path_param_1.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_boolean_path_parameter_numeric_1();

        // Build request
        let mut uri = "/path/bool/1".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_date_path_parameter_success() {
        // Fixture: Date path parameter - success
        // Description: Tests date path parameter with ISO format date
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/20_date_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_date_path_parameter_success();

        // Build request
        let mut uri = "/date/2023-07-15".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_enum_path_parameter_invalid_value() {
        // Fixture: Enum path parameter - invalid value
        // Description: Tests enum path parameter with invalid enum value returns 422
        // Expected status: 422

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/10_enum_path_param_invalid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_enum_path_parameter_invalid_value();

        // Build request
        let mut uri = "/models/foo".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_enum_path_parameter_success() {
        // Fixture: Enum path parameter - success
        // Description: Tests enum path parameter with valid enum value
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/09_enum_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_enum_path_parameter_success();

        // Build request
        let mut uri = "/models/alexnet".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_float_path_parameter_success() {
        // Fixture: Float path parameter - success
        // Description: Tests float path parameter with valid value
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/04_float_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_float_path_parameter_success();

        // Build request
        let mut uri = "/path/float/42.5".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_invalid_string() {
        // Fixture: Integer path parameter - invalid string
        // Description: Tests integer path parameter with non-numeric string returns 422
        // Expected status: 422

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/03_int_path_param_invalid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_invalid_string();

        // Build request
        let mut uri = "/path/int/foobar".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_success() {
        // Fixture: Integer path parameter - success
        // Description: Tests integer path parameter with valid value
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/02_int_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_success();

        // Build request
        let mut uri = "/path/int/42".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success() {
        // Fixture: Integer path parameter with combined lt and gt constraints - success
        // Description: Tests integer path parameter with both lt and gt validation (range check)
        // Expected status: 200

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/18_int_validation_combined_lt_gt.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app =
            spikard_e2e_app::create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success(
            );

        // Build request
        let mut uri = "/path/param-lt-gt/2".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_ge_constraint_success() {
        // Fixture: Integer path parameter with ge constraint - success
        // Description: Tests integer path parameter with ge (greater than or equal) validation at boundary
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/15_int_validation_ge_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_with_ge_constraint_success();

        // Build request
        let mut uri = "/path/param-ge/3".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_gt_constraint_failure() {
        // Fixture: Integer path parameter with gt constraint - failure
        // Description: Tests integer path parameter with gt validation fails when value too small
        // Expected status: 422

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/07_int_validation_gt_fail.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_with_gt_constraint_failure();

        // Build request
        let mut uri = "/path/param-gt/2".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_gt_constraint_success() {
        // Fixture: Integer path parameter with gt constraint - success
        // Description: Tests integer path parameter with gt (greater than) validation succeeds
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/06_int_validation_gt_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_with_gt_constraint_success();

        // Build request
        let mut uri = "/path/param-gt/42".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_le_constraint_success() {
        // Fixture: Integer path parameter with le constraint - success
        // Description: Tests integer path parameter with le (less than or equal) validation at boundary
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/16_int_validation_le_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_with_le_constraint_success();

        // Build request
        let mut uri = "/path/param-le/3".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_integer_path_parameter_with_lt_constraint_success() {
        // Fixture: Integer path parameter with lt constraint - success
        // Description: Tests integer path parameter with lt (less than) validation
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/17_int_validation_lt_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_integer_path_parameter_with_lt_constraint_success();

        // Build request
        let mut uri = "/path/param-lt/2".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_multiple_path_parameters_success() {
        // Fixture: Multiple path parameters - success
        // Description: Tests multiple path parameters in single route
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/12_multiple_path_params.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_multiple_path_parameters_success();

        // Build request
        let mut uri = "/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_path_parameter_type_syntax_invalid_uuid() {
        // Fixture: Path parameter type syntax - invalid UUID
        // Description: Tests that :uuid type syntax auto-generates validation that rejects invalid UUIDs
        // Expected status: 422

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/39_type_syntax_invalid_uuid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_path_parameter_type_syntax_invalid_uuid();

        // Build request
        let mut uri = "/type-syntax/items/not-a-uuid".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_path_parameter_type_syntax_with_override() {
        // Fixture: Path parameter type syntax with override
        // Description: Tests that explicit parameter schema merges with auto-generated type syntax schema
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/38_type_syntax_override.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_path_parameter_type_syntax_with_override();

        // Build request
        let mut uri = "/type-syntax/items-count/50".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_path_parameter_with_type_syntax_uuid() {
        // Fixture: Path parameter with type syntax - UUID
        // Description: Tests path parameter with :uuid type syntax auto-generates UUID validation
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/36_type_syntax_uuid.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_path_parameter_with_type_syntax_uuid();

        // Build request
        let mut uri = "/type-syntax/items/550e8400-e29b-41d4-a716-446655440000".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_path_parameter_with_type_syntax_integer() {
        // Fixture: Path parameter with type syntax - integer
        // Description: Tests path parameter with :int type syntax auto-generates integer type
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/37_type_syntax_int.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_path_parameter_with_type_syntax_integer();

        // Build request
        let mut uri = "/type-syntax/users/42".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_path_type_parameter_file_path() {
        // Fixture: Path type parameter - file path
        // Description: Tests path type parameter that captures remaining path segments
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/11_path_type_file_path.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_path_type_parameter_file_path();

        // Build request
        let mut uri = "/files/home/johndoe/myfile.txt".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_string_path_parameter_success() {
        // Fixture: String path parameter - success
        // Description: Tests string path parameter with valid value
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/01_string_path_param.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_string_path_parameter_success();

        // Build request
        let mut uri = "/path/str/foobar".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_string_path_parameter_with_max_length_failure() {
        // Fixture: String path parameter with max_length - failure
        // Description: Tests string path parameter with max_length validation fails when too long
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/19_string_maxlength_validation_fail.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_string_path_parameter_with_max_length_failure();

        // Build request
        let mut uri = "/path/param-maxlength/foobar".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_string_path_parameter_with_min_length_failure() {
        // Fixture: String path parameter with min_length - failure
        // Description: Tests string path parameter with min_length validation fails
        // Expected status: 422

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/path_params/08_string_minlength_validation_fail.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_string_path_parameter_with_min_length_failure();

        // Build request
        let mut uri = "/path/param-minlength/fo".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_path_params_uuid_path_parameter_success() {
        // Fixture: UUID path parameter - success
        // Description: Tests UUID path parameter with valid UUID format
        // Expected status: 200

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/path_params/05_uuid_path_param_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_path_params_uuid_path_parameter_success();

        // Build request
        let mut uri = "/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a".to_string();

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
        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            // Handle multipart/form-data with files
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            // Add files
            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                // Handle both regular content and magic_bytes (hex-encoded binary)
                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
                    // Decode hex string to bytes, then to string
                    // For binary data, we'll use the hex representation as placeholder
                    format!("<binary data: {}>", magic_bytes)
                } else {
                    String::new()
                };

                let filename = file["filename"].as_str();
                let content_type = file["content_type"].as_str();

                multipart_body.push_str(&format!("--{}\r\n", boundary));
                if let Some(fname) = filename {
                    multipart_body.push_str(&format!(
                        "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                        field_name, fname
                    ));
                    if let Some(ct) = content_type {
                        multipart_body.push_str(&format!("Content-Type: {}\r\n", ct));
                    }
                } else {
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n", field_name));
                }
                multipart_body.push_str("\r\n");
                multipart_body.push_str(&content_str);
                multipart_body.push_str("\r\n");
            }

            // Add form data fields if present
            if let Some(data) = fixture["request"]["data"].as_object() {
                for (key, value) in data {
                    multipart_body.push_str(&format!("--{}\r\n", boundary));
                    multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                    if let Some(s) = value.as_str() {
                        multipart_body.push_str(s);
                    } else {
                        multipart_body.push_str(&value.to_string());
                    }
                    multipart_body.push_str("\r\n");
                }
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
            // Multipart with only form data (no files)
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for (key, value) in data {
                multipart_body.push_str(&format!("--{}\r\n", boundary));
                multipart_body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
                if let Some(s) = value.as_str() {
                    multipart_body.push_str(s);
                } else {
                    multipart_body.push_str(&value.to_string());
                }
                multipart_body.push_str("\r\n");
            }

            multipart_body.push_str(&format!("--{}--\r\n", boundary));

            // Set Content-Type header with boundary
            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
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

        let server = TestServer::new(app).unwrap();
        let response = call_test_server(&server, request).await;
        let snapshot = snapshot_response(response).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }
}
