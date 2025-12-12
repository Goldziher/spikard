//! Tests for static_files fixtures
//! Generated from: testing_data/static_files

#[cfg(test)]
mod static_files {
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::Value;
    use spikard::testing::TestServer;

    #[tokio::test]
    async fn test_static_files_static_file_server_returns_text_file() {

        let fixture_json = std::fs::read_to_string("../../testing_data/static_files/01_serves_plain_text.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_static_files_static_file_server_returns_text_file()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/public/hello.txt".to_string();

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
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("cache-control") {
            assert_eq!(actual, "public, max-age=60", "Mismatched header 'cache-control'");
        } else {
            panic!("Expected header 'cache-control' to be present");
        }
        if let Some(actual) = headers.get("content-type") {
            assert_eq!(actual, "text/plain", "Mismatched header 'content-type'");
        } else {
            panic!("Expected header 'content-type' to be present");
        }
    }

    #[tokio::test]
    async fn test_static_files_static_server_returns_index_html_for_directory() {

        let fixture_json = std::fs::read_to_string("../../testing_data/static_files/02_index_html_fallback.json")
            .expect("Failed to read fixture file");
        let fixture: Value = serde_json::from_str(&fixture_json).expect("Failed to parse fixture JSON");

        let app = spikard_e2e_app::create_app_static_files_static_server_returns_index_html_for_directory()
            .expect("Failed to build fixture app");
        let server = TestServer::from_app(app).expect("Failed to build server");

        let mut uri = "/app/".to_string();

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
        let headers = &snapshot.headers;
        if let Some(actual) = headers.get("content-type") {
            assert_eq!(actual, "text/html", "Mismatched header 'content-type'");
        } else {
            panic!("Expected header 'content-type' to be present");
        }
    }
}
