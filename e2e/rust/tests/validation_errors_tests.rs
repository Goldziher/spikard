//! Tests for validation_errors fixtures
//! Generated from: testing_data/validation_errors

#[cfg(test)]
mod validation_errors {

    #[tokio::test]
    async fn test_validation_errors_invalid_uuid_format() {
        // Fixture: Invalid UUID format
        // Description: Tests validation error when UUID format is invalid
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/validation_errors/11_uuid_invalid_format.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Invalid_UUID_format();

        // Build request
        let mut uri = "/items/not-a-uuid".to_string();

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
    async fn test_validation_errors_invalid_boolean_value() {
        // Fixture: Invalid boolean value
        // Description: Tests validation error when boolean value is invalid
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/validation_errors/13_bool_invalid_value.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Invalid_boolean_value();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_missing_required_query_parameter() {
        // Fixture: Missing required query parameter
        // Description: Tests validation error when required query param is missing
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/02_query_param_missing_required.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Missing_required_query_parameter();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_array_max_items_constraint_violation() {
        // Fixture: Array max_items constraint violation
        // Description: Tests validation error when array has more items than max_items
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/17_array_max_items_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Array_max_items_constraint_violation();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_numeric_constraint_violation_gt_greater_than() {
        // Fixture: Numeric constraint violation - gt (greater than)
        // Description: Tests validation error when value violates gt constraint
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/05_numeric_constraint_gt_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Numeric_constraint_violation___gt__greater_than();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_string_regex_pattern_mismatch() {
        // Fixture: String regex pattern mismatch
        // Description: Tests validation error when string doesn't match regex pattern
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/09_string_regex_pattern_mismatch.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_String_regex_pattern_mismatch();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_invalid_enum_value() {
        // Fixture: Invalid enum value
        // Description: Tests validation error when value is not in allowed enum values
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/validation_errors/10_enum_invalid_value.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Invalid_enum_value();

        // Build request
        let mut uri = "/models/invalid_model".to_string();

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
    async fn test_validation_errors_string_min_length_constraint_violation() {
        // Fixture: String min_length constraint violation
        // Description: Tests validation error when string is shorter than min_length
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/07_string_min_length_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_String_min_length_constraint_violation();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_multiple_validation_errors() {
        // Fixture: Multiple validation errors
        // Description: Tests multiple validation errors returned in single response
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/14_multiple_validation_errors.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Multiple_validation_errors();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_string_max_length_constraint_violation() {
        // Fixture: String max_length constraint violation
        // Description: Tests validation error when string exceeds max_length
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/08_string_max_length_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_String_max_length_constraint_violation();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_nested_object_validation_error() {
        // Fixture: Nested object validation error
        // Description: Tests validation error in nested object field
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/15_nested_object_validation_error.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Nested_object_validation_error();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_10_nested_error_path() {
        // Fixture: 10_nested_error_path
        // Description: Validation error in nested object should have correct path in loc
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/validation_errors/10_nested_error_path.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_10_nested_error_path();

        // Build request
        let mut uri = "/profiles".to_string();

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
    async fn test_validation_errors_invalid_datetime_format() {
        // Fixture: Invalid datetime format
        // Description: Tests validation error when datetime format is invalid
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/12_datetime_invalid_format.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Invalid_datetime_format();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_array_item_validation_error() {
        // Fixture: Array item validation error
        // Description: Tests validation error for invalid item within array
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/18_array_item_validation_error.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Array_item_validation_error();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_missing_required_body_field() {
        // Fixture: Missing required body field
        // Description: Tests validation error when required body field is missing
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/04_body_missing_required_field.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Missing_required_body_field();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_body_field_type_error_string_for_float() {
        // Fixture: Body field type error - string for float
        // Description: Tests validation error when body field has wrong type
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/03_body_field_type_error.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Body_field_type_error___string_for_float();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_malformed_json_body() {
        // Fixture: Malformed JSON body
        // Description: Tests validation error when request body contains malformed JSON
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/validation_errors/20_malformed_json.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Malformed_JSON_body();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_query_param_type_error_string_provided_for_int() {
        // Fixture: Query param type error - string provided for int
        // Description: Tests validation error when string is provided for integer query param
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string(
            "../../testing_data/validation_errors/01_query_param_type_error_string_to_int.json",
        )
        .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Query_param_type_error___string_provided_for_int();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_header_validation_error() {
        // Fixture: Header validation error
        // Description: Tests validation error when required header is missing
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/19_header_validation_error.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Header_validation_error();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_09_multiple_validation_errors() {
        // Fixture: 09_multiple_validation_errors
        // Description: Multiple validation errors should be returned together in batch
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/09_multiple_validation_errors.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_09_multiple_validation_errors();

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
    async fn test_validation_errors_numeric_constraint_violation_le_less_than_or_equal() {
        // Fixture: Numeric constraint violation - le (less than or equal)
        // Description: Tests validation error when value violates le constraint
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/06_numeric_constraint_le_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Numeric_constraint_violation___le__less_than_or_equal();

        // Build request
        let mut uri = "/items/".to_string();

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
    async fn test_validation_errors_array_min_items_constraint_violation() {
        // Fixture: Array min_items constraint violation
        // Description: Tests validation error when array has fewer items than min_items
        // Expected status: 422

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/validation_errors/16_array_min_items_violation.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_validation_errors_Array_min_items_constraint_violation();

        // Build request
        let mut uri = "/items/".to_string();

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
