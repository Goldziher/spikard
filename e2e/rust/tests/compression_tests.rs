//! Tests for compression fixtures
//! Generated from: testing_data/compression

#[cfg(test)]
mod compression {

    #[tokio::test]
    async fn test_compression_compression_brotli_applied() {
        // Fixture: Compression - brotli applied
        // Description: Tests brotli compression when client supports it via Accept-Encoding header
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/02_compression_brotli_applied.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___brotli_applied();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_client_prefers_brotli() {
        // Fixture: Compression - client prefers brotli
        // Description: Tests content negotiation when client prefers brotli over gzip using quality values
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/05_compression_prefer_brotli.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___client_prefers_brotli();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_client_prefers_gzip() {
        // Fixture: Compression - client prefers gzip
        // Description: Tests content negotiation when client prefers gzip over brotli using quality values
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/06_compression_prefer_gzip.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___client_prefers_gzip();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_gzip_applied() {
        // Fixture: Compression - gzip applied
        // Description: Tests gzip compression when client supports it via Accept-Encoding header
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/01_compression_gzip_applied.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___gzip_applied();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_high_compression_level() {
        // Fixture: Compression - high compression level
        // Description: Tests high compression level (9) for maximum size reduction at cost of CPU time
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/09_compression_high_level.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___high_compression_level();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_large_json_response() {
        // Fixture: Compression - large JSON response
        // Description: Tests compression of large JSON response with high compression ratio due to text redundancy
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/15_compression_large_json.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___large_JSON_response();

        // Build request
        let mut uri = "/api/records".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_low_compression_level() {
        // Fixture: Compression - low compression level
        // Description: Tests low compression level (1) for faster processing with modest size reduction
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json = std::fs::read_to_string("../../testing_data/compression/10_compression_low_level.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___low_compression_level();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_minimum_size_threshold() {
        // Fixture: Compression - minimum size threshold
        // Description: Tests that compression is only applied when response size exceeds configured threshold
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/compression/11_compression_min_size_threshold.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___minimum_size_threshold();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_multiple_acceptable_encodings() {
        // Fixture: Compression - multiple acceptable encodings
        // Description: Tests that server selects best available algorithm when client accepts multiple encodings
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/compression/07_compression_multiple_encodings.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___multiple_acceptable_encodings();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }

    #[tokio::test]
    async fn test_compression_compression_wildcard_encoding() {
        // Fixture: Compression - wildcard encoding
        // Description: Tests that server applies compression when client sends Accept-Encoding: * (any encoding acceptable)
        // Expected status: 200

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use serde_json::Value;
        use tower::ServiceExt;

        // Load fixture
        let fixture_json =
            std::fs::read_to_string("../../testing_data/compression/08_compression_wildcard_encoding.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        // Create app for this specific fixture
        let app = spikard_e2e_app::create_app_compression_Compression___wildcard_encoding();

        // Build request
        let mut uri = "/api/data".to_string();

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
            StatusCode::from_u16(200).unwrap(),
            "Expected status 200, got {:?}",
            response.status()
        );
    }
}
