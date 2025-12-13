//! Tests for status_codes fixtures
//! Generated from: testing_data/status_codes

#[cfg(test)]
mod status_codes {
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use spikard::testing::TestServer;

    #[tokio::test]
    async fn test_status_codes_19_413_payload_too_large() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/19_413_payload_too_large.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app =
            spikard_e2e_app::create_app_status_codes_19_413_payload_too_large().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/upload".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 413, "Expected status 413, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_200_ok_success() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/01_200_ok_success.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_200_ok_success().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/status-test/200".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_201_created_resource_created() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/02_201_created.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_201_created_resource_created()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/items/".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 201, "Expected status 201, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_202_accepted_request_accepted_for_processing() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/03_202_accepted.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_202_accepted_request_accepted_for_processing()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/tasks/".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 202, "Expected status 202, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_204_no_content_success_with_no_body() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/04_204_no_content.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_204_no_content_success_with_no_body()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/status-test/204".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("DELETE").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 204, "Expected status 204, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_206_partial_content() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/16_206_partial_content.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_206_partial_content().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/files/document.pdf".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 206, "Expected status 206, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("accept-ranges") {
            assert_eq!(actual, "bytes", "Mismatched header 'Accept-Ranges'");
        } else {
            panic!("Expected header 'Accept-Ranges' to be present");
        }
        if let Some(actual) = headers.get("content-length") {
            assert_eq!(actual, "1024", "Mismatched header 'Content-Length'");
        } else {
            panic!("Expected header 'Content-Length' to be present");
        }
        if let Some(actual) = headers.get("content-type") {
            assert_eq!(actual, "application/pdf", "Mismatched header 'Content-Type'");
        } else {
            panic!("Expected header 'Content-Type' to be present");
        }
        if let Some(actual) = headers.get("content-range") {
            assert_eq!(actual, "bytes 0-1023/5000", "Mismatched header 'Content-Range'");
        } else {
            panic!("Expected header 'Content-Range' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_20_414_uri_too_long() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/20_414_uri_too_long.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_20_414_uri_too_long().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/data".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 200, "Expected status 200, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_21_431_request_header_fields_too_large() {

        let fixture_json =
            std::fs::read_to_string("../../testing_data/status_codes/21_431_request_header_fields_too_large.json")
                .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_21_431_request_header_fields_too_large()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/data".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 431, "Expected status 431, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_22_501_not_implemented() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/22_501_not_implemented.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app =
            spikard_e2e_app::create_app_status_codes_22_501_not_implemented().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/data".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("TRACE").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 405, "Expected status 405, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_23_503_service_unavailable() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/23_503_service_unavailable.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app =
            spikard_e2e_app::create_app_status_codes_23_503_service_unavailable().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/data".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 503, "Expected status 503, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("retry-after") {
            assert_eq!(actual, "60", "Mismatched header 'Retry-After'");
        } else {
            panic!("Expected header 'Retry-After' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_301_moved_permanently_permanent_redirect() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/05_301_moved_permanently.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_301_moved_permanently_permanent_redirect()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/old-path".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 301, "Expected status 301, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("location") {
            assert_eq!(actual, "/new-path", "Mismatched header 'location'");
        } else {
            panic!("Expected header 'location' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_302_found_temporary_redirect() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/06_302_found_temporary.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_302_found_temporary_redirect()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/temp-redirect".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 302, "Expected status 302, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("location") {
            assert_eq!(actual, "/target-path", "Mismatched header 'location'");
        } else {
            panic!("Expected header 'location' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_304_not_modified_cached_content_valid() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/07_304_not_modified.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_304_not_modified_cached_content_valid()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/status-test/304".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 304, "Expected status 304, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_307_temporary_redirect_method_preserved() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/08_307_temporary_redirect.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_307_temporary_redirect_method_preserved()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/redirect-post".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 307, "Expected status 307, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("location") {
            assert_eq!(actual, "/target-post", "Mismatched header 'location'");
        } else {
            panic!("Expected header 'location' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_400_bad_request_invalid_request() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/09_400_bad_request.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_400_bad_request_invalid_request()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/items/".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 400, "Expected status 400, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_401_unauthorized_missing_authentication() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/10_401_unauthorized.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_401_unauthorized_missing_authentication()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/users/me".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 401, "Expected status 401, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("www-authenticate") {
            assert_eq!(actual, "Bearer", "Mismatched header 'www-authenticate'");
        } else {
            panic!("Expected header 'www-authenticate' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_403_forbidden_insufficient_permissions() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/11_403_forbidden.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_403_forbidden_insufficient_permissions()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/admin/users".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 403, "Expected status 403, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_404_not_found_resource_not_found() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/12_404_not_found.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_404_not_found_resource_not_found()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/status-test/404".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 404, "Expected status 404, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_408_request_timeout() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/17_408_request_timeout.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_408_request_timeout().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/slow-endpoint".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 408, "Expected status 408, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("connection") {
            assert_eq!(actual, "close", "Mismatched header 'Connection'");
        } else {
            panic!("Expected header 'Connection' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_422_unprocessable_entity_validation_error() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/13_422_validation_error.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_422_unprocessable_entity_validation_error()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/items/".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("POST").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 422, "Expected status 422, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_429_too_many_requests() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/18_429_too_many_requests.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app =
            spikard_e2e_app::create_app_status_codes_429_too_many_requests().expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/api/resource".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 429, "Expected status 429, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("retry-after") {
            assert_eq!(actual, "60", "Mismatched header 'Retry-After'");
        } else {
            panic!("Expected header 'Retry-After' to be present");
        }
        if let Some(actual) = headers.get("x-ratelimit-limit") {
            assert_eq!(actual, "100", "Mismatched header 'X-RateLimit-Limit'");
        } else {
            panic!("Expected header 'X-RateLimit-Limit' to be present");
        }
        if let Some(actual) = headers.get("x-ratelimit-remaining") {
            assert_eq!(actual, "0", "Mismatched header 'X-RateLimit-Remaining'");
        } else {
            panic!("Expected header 'X-RateLimit-Remaining' to be present");
        }
        if let Some(actual) = headers.get("x-ratelimit-reset") {
            assert_eq!(actual, "1609459200", "Mismatched header 'X-RateLimit-Reset'");
        } else {
            panic!("Expected header 'X-RateLimit-Reset' to be present");
        }
    }

    #[tokio::test]
    async fn test_status_codes_500_internal_server_error_server_error() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/14_500_internal_server_error.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_500_internal_server_error_server_error()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/error".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 500, "Expected status 500, got {}", snapshot.status);
    }

    #[tokio::test]
    async fn test_status_codes_503_service_unavailable_server_overload() {

        let fixture_json = std::fs::read_to_string("../../testing_data/status_codes/15_503_service_unavailable.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_status_codes_503_service_unavailable_server_overload()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/health".to_string();

        if let Some(query_string) = fixture["request"]["query_string"].as_str() {
            if !query_string.is_empty() {
                uri.push_str("?");
                uri.push_str(query_string);
            }
        } else if let Some(query_params) = fixture["request"]["query_params"].as_object() {
            use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

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

        let mut request_builder = Request::builder().method("GET").uri(uri);

        if let Some(headers) = fixture["request"]["headers"].as_object() {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }
        }

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

        let body = if let Some(files) = fixture["request"]["files"].as_array() {
            let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
            let mut multipart_body = String::new();

            for file in files {
                let field_name = file["field_name"].as_str().unwrap();

                let content_str = if let Some(content) = file["content"].as_str() {
                    content.to_string()
                } else if let Some(magic_bytes) = file["magic_bytes"].as_str() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(data) = fixture["request"]["data"].as_object() {
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

            request_builder =
                request_builder.header("content-type", format!("multipart/form-data; boundary={}", boundary));

            Body::from(multipart_body)
        } else if let Some(request_body) = fixture["request"]["body"].as_str() {
            Body::from(request_body.to_string())
        } else if let Some(request_body) = fixture["request"]["body"].as_object() {
            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/json");
            }
            let body_str = serde_json::to_string(request_body).unwrap();
            Body::from(body_str)
        } else if let Some(form_data) = fixture["request"]["form_data"].as_object() {
            use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

            let form_params: Vec<String> = form_data
                .iter()
                .flat_map(|(key, value)| {
                    match value {
                        serde_json::Value::Array(arr) => {
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

            if fixture["request"]["headers"]["Content-Type"].is_null() {
                request_builder = request_builder.header("content-type", "application/x-www-form-urlencoded");
            }

            Body::from(body_str)
        } else {
            Body::empty()
        };

        let request = request_builder.body(body).unwrap();

        let snapshot = server.call(request).await.unwrap();

        assert_eq!(snapshot.status, 503, "Expected status 503, got {}", snapshot.status);
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("retry-after") {
            assert_eq!(actual, "120", "Mismatched header 'retry-after'");
        } else {
            panic!("Expected header 'retry-after' to be present");
        }
    }
}
