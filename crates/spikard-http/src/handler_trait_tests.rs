//! Unit tests for the Handler trait and related functionality
//!
//! These tests verify the Handler trait works correctly and can be implemented
//! by different language bindings without requiring PyO3 or other FFI dependencies.

#[cfg(test)]
mod tests {
    use crate::handler_trait::{Handler, HandlerResult, RequestData};
    use axum::body::Body;
    use axum::http::{Method, Request, Response, StatusCode};
    use serde_json::{Value, json};
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;

    /// Mock handler for testing - always returns 200 OK with echoed request data
    struct EchoHandler;

    impl Handler for EchoHandler {
        fn call(
            &self,
            _request: Request<Body>,
            request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move {
                let response_body = json!({
                    "path_params": &*request_data.path_params,
                    "query_params": request_data.query_params,
                    "body": request_data.body,
                    "headers_count": request_data.headers.len(),
                    "cookies_count": request_data.cookies.len(),
                    "method": request_data.method,
                    "path": request_data.path,
                });

                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&response_body).unwrap()))
                    .unwrap();

                Ok(response)
            })
        }
    }

    /// Mock handler for testing - always returns errors
    struct ErrorHandler;

    impl Handler for ErrorHandler {
        fn call(
            &self,
            _request: Request<Body>,
            _request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            Box::pin(async move { Err((StatusCode::INTERNAL_SERVER_ERROR, "Handler error".to_string())) })
        }
    }

    /// Mock handler that checks for specific query parameters
    struct QueryParamHandler {
        required_param: String,
    }

    impl Handler for QueryParamHandler {
        fn call(
            &self,
            _request: Request<Body>,
            request_data: RequestData,
        ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
            let required = self.required_param.clone();
            Box::pin(async move {
                if request_data.raw_query_params.contains_key(&required) {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from("OK"))
                        .unwrap();
                    Ok(response)
                } else {
                    Err((
                        StatusCode::BAD_REQUEST,
                        format!("Missing required parameter: {}", required),
                    ))
                }
            })
        }
    }

    #[tokio::test]
    async fn test_handler_trait_echo() {
        let handler = Arc::new(EchoHandler);

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123".to_string());

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let request_data = RequestData {
            path_params: Arc::new(path_params),
            query_params: Arc::new(json!({"page": 1})),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(json!({"test": "data"})),
            raw_body: None,
            headers: Arc::new(headers),
            cookies: Arc::new(HashMap::new()),
            method: "POST".to_string(),
            path: "/items/123".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let request = Request::builder()
            .method(Method::POST)
            .uri("/items/123")
            .body(Body::empty())
            .unwrap();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_trait_error() {
        let handler = Arc::new(ErrorHandler);

        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(Value::Null),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/error".to_string(),

            #[cfg(feature = "di")]
            dependencies: None,
        };

        let request = Request::builder()
            .method(Method::GET)
            .uri("/error")
            .body(Body::empty())
            .unwrap();

        let result = handler.call(request, request_data).await;
        assert!(result.is_err());

        let (status, message) = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(message, "Handler error");
    }

    #[tokio::test]
    async fn test_handler_trait_query_params() {
        let handler = Arc::new(QueryParamHandler {
            required_param: "api_key".to_string(),
        });

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("api_key".to_string(), vec!["secret123".to_string()]);

        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(json!({"api_key": "secret123"})),
            validated_params: None,
            raw_query_params: Arc::new(raw_query_params.clone()),
            body: Arc::new(Value::Null),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/api/data".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/data?api_key=secret123")
            .body(Body::empty())
            .unwrap();

        let result = handler.call(request, request_data).await;
        assert!(result.is_ok());

        let request_data_no_param = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(Value::Null),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/api/data".to_string(),

            #[cfg(feature = "di")]
            dependencies: None,
        };

        let request_no_param = Request::builder()
            .method(Method::GET)
            .uri("/api/data")
            .body(Body::empty())
            .unwrap();

        let result_err = handler.call(request_no_param, request_data_no_param).await;
        assert!(result_err.is_err());

        let (status, message) = result_err.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(message.contains("api_key"));
    }

    #[tokio::test]
    async fn test_request_data_serialization() {
        let mut path_params = HashMap::new();
        path_params.insert("user_id".to_string(), "42".to_string());

        let mut raw_query_params = HashMap::new();
        raw_query_params.insert("filter".to_string(), vec!["active".to_string()]);

        let request_data = RequestData {
            path_params: Arc::new(path_params),
            query_params: Arc::new(json!({"filter": "active"})),
            validated_params: None,
            raw_query_params: Arc::new(raw_query_params),
            body: Arc::new(json!({"name": "test"})),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "PUT".to_string(),
            path: "/users/42".to_string(),
            #[cfg(feature = "di")]
            dependencies: None,
        };

        let json_str = serde_json::to_string(&request_data).unwrap();
        let deserialized: RequestData = serde_json::from_str(&json_str).unwrap();

        assert_eq!(deserialized.method, "PUT");
        assert_eq!(deserialized.path, "/users/42");
        assert_eq!(deserialized.path_params.get("user_id").unwrap(), "42");
        assert_eq!(*deserialized.body, json!({"name": "test"}));
    }

    #[test]
    fn test_request_data_default_values() {
        let request_data = RequestData {
            path_params: Arc::new(HashMap::new()),
            query_params: Arc::new(Value::Null),
            validated_params: None,
            raw_query_params: Arc::new(HashMap::new()),
            body: Arc::new(Value::Null),
            raw_body: None,
            headers: Arc::new(HashMap::new()),
            cookies: Arc::new(HashMap::new()),
            method: "GET".to_string(),
            path: "/".to_string(),

            #[cfg(feature = "di")]
            dependencies: None,
        };

        assert_eq!(request_data.method, "GET");
        assert_eq!(request_data.path, "/");
        assert!(request_data.path_params.is_empty());
        assert!(request_data.raw_query_params.is_empty());
        assert!(request_data.headers.is_empty());
        assert!(request_data.cookies.is_empty());
        assert_eq!(*request_data.body, Value::Null);
        assert_eq!(*request_data.query_params, Value::Null);
    }

    #[test]
    fn test_handler_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Arc<dyn Handler>>();
    }
}
