//! Shared lifecycle hook executor infrastructure
//!
//! This module provides a language-agnostic abstraction for executing lifecycle hooks.
//! It extracts ~960 lines of duplicated logic from Python, Node.js, Ruby, and PHP bindings
//! into reusable trait-based components.
//!
//! # Design
//!
//! The executor uses a trait-based pattern where language bindings implement
//! `LanguageLifecycleHook` to provide language-specific hook invocation, while
//! `LifecycleExecutor` handles the common logic:
//!
//! - Hook result type handling (Continue vs `ShortCircuit`)
//! - Response/Request building from hook results
//! - Error handling and conversion
//!
//! # Example
//!
//! ```ignore
//! struct MyLanguageHook { ... }
//!
//! impl LanguageLifecycleHook for MyLanguageHook {
//!     fn prepare_hook_data(&self, req: &Request<Body>) -> Result<Self::HookData, String> {
//!         // Convert Request<Body> to language-specific representation
//!     }
//!
//!     async fn invoke_hook(&self, data: Self::HookData) -> Result<HookResultData, String> {
//!         // Call language function and return structured result
//!     }
//! }
//! ```

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Data returned from language-specific hook invocation
///
/// This is a normalized representation of hook results that abstracts away
/// language-specific details. Bindings convert their native hook results
/// into this common format.
#[derive(Debug, Clone)]
pub struct HookResultData {
    /// Whether to continue execution (true) or short-circuit (false)
    pub continue_execution: bool,
    /// Optional status code for short-circuit responses
    pub status_code: Option<u16>,
    /// Optional headers to include in response
    pub headers: Option<HashMap<String, String>>,
    /// Optional body bytes for response
    pub body: Option<Vec<u8>>,
    /// Optional request modifications (method, path, headers, body)
    pub request_modifications: Option<RequestModifications>,
}

/// Modifications to apply to a request
#[derive(Debug, Clone)]
pub struct RequestModifications {
    /// New HTTP method (e.g., "GET", "POST")
    pub method: Option<String>,
    /// New request path
    pub path: Option<String>,
    /// New or updated headers
    pub headers: Option<HashMap<String, String>>,
    /// New request body
    pub body: Option<Vec<u8>>,
}

impl HookResultData {
    /// Create a Continue result (pass through)
    #[must_use]
    pub const fn continue_execution() -> Self {
        Self {
            continue_execution: true,
            status_code: None,
            headers: None,
            body: None,
            request_modifications: None,
        }
    }

    /// Create a short-circuit response result
    #[must_use]
    pub const fn short_circuit(status_code: u16, body: Vec<u8>, headers: Option<HashMap<String, String>>) -> Self {
        Self {
            continue_execution: false,
            status_code: Some(status_code),
            headers,
            body: Some(body),
            request_modifications: None,
        }
    }

    /// Create a request modification result
    #[must_use]
    pub const fn modify_request(modifications: RequestModifications) -> Self {
        Self {
            continue_execution: true,
            status_code: None,
            headers: None,
            body: None,
            request_modifications: Some(modifications),
        }
    }
}

/// Trait for language-specific lifecycle hook implementations
///
/// Each language binding implements this trait to provide language-specific
/// hook invocation while delegating common logic to `LifecycleExecutor`.
pub trait LanguageLifecycleHook: Send + Sync {
    /// Language-specific hook data type
    type HookData: Send;

    /// Prepare hook data from the incoming request/response
    ///
    /// This should convert axum HTTP types to language-specific representations.
    ///
    /// # Errors
    ///
    /// Returns an error if hook data preparation fails.
    fn prepare_hook_data(&self, req: &Request<Body>) -> Result<Self::HookData, String>;

    /// Invoke the language hook and return normalized result data
    ///
    /// This should call the language function and convert its result to `HookResultData`.
    fn invoke_hook(&self, data: Self::HookData)
    -> Pin<Box<dyn Future<Output = Result<HookResultData, String>> + Send>>;
}

/// Executor that handles common lifecycle hook logic
///
/// This executor is generic over any language binding that implements
/// `LanguageLifecycleHook`. It provides common logic for:
/// - Executing request hooks and handling short-circuits
/// - Executing response hooks and building modified responses
/// - Converting hook results to axum Request/Response types
pub struct LifecycleExecutor<L: LanguageLifecycleHook> {
    hook: Arc<L>,
}

impl<L: LanguageLifecycleHook> LifecycleExecutor<L> {
    /// Create a new executor for the given hook
    pub const fn new(hook: Arc<L>) -> Self {
        Self { hook }
    }

    /// Execute a request hook, handling Continue/ShortCircuit semantics
    ///
    /// Returns either the modified request or a short-circuit response.
    ///
    /// # Errors
    ///
    /// Returns an error if hook execution or modification fails.
    pub async fn execute_request_hook(
        &self,
        req: Request<Body>,
    ) -> Result<Result<Request<Body>, Response<Body>>, String> {
        let hook_data = self.hook.prepare_hook_data(&req)?;
        let result = self.hook.invoke_hook(hook_data).await?;

        if !result.continue_execution {
            let response = Self::build_response_from_hook_result(&result)?;
            return Ok(Err(response));
        }

        if let Some(modifications) = result.request_modifications {
            let modified_req = Self::apply_request_modifications(req, modifications)?;
            Ok(Ok(modified_req))
        } else {
            Ok(Ok(req))
        }
    }

    /// Execute a response hook, handling response modification
    ///
    /// Response hooks can only continue or modify the response,
    /// never short-circuit.
    /// Execute the lifecycle hook on an outgoing response
    ///
    /// # Errors
    ///
    /// Returns an error if hook execution or response building fails.
    pub async fn execute_response_hook(&self, resp: Response<Body>) -> Result<Response<Body>, String> {
        let (parts, body) = resp.into_parts();
        let body_bytes = extract_body(body).await?;

        let dummy_req = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::empty())
            .map_err(|e| format!("Failed to build dummy request: {e}"))?;

        let hook_data = self.hook.prepare_hook_data(&dummy_req)?;
        let result = self.hook.invoke_hook(hook_data).await?;

        if let Some(modifications) = result.request_modifications {
            let mut builder = Response::builder().status(parts.status);

            let header_mod_keys: Vec<String> = modifications
                .headers
                .as_ref()
                .map(|mods| mods.keys().map(|k| k.to_lowercase()).collect())
                .unwrap_or_default();

            if let Some(header_mods) = modifications.headers {
                for (key, value) in header_mods {
                    builder = builder.header(&key, &value);
                }
            }

            for (name, value) in &parts.headers {
                let key_str = name.as_str().to_lowercase();
                if !header_mod_keys.contains(&key_str) {
                    builder = builder.header(name, value);
                }
            }

            let body = modifications.body.unwrap_or(body_bytes);
            return builder
                .body(Body::from(body))
                .map_err(|e| format!("Failed to build modified response: {e}"));
        }

        let mut builder = Response::builder().status(parts.status);
        for (name, value) in parts.headers {
            if let Some(name) = name {
                builder = builder.header(name, value);
            }
        }
        builder
            .body(Body::from(body_bytes))
            .map_err(|e| format!("Failed to rebuild response: {e}"))
    }

    /// Build an axum Response from hook result data
    fn build_response_from_hook_result(result: &HookResultData) -> Result<Response<Body>, String> {
        let status_code = result.status_code.unwrap_or(200);
        let status =
            StatusCode::from_u16(status_code).map_err(|e| format!("Invalid status code {status_code}: {e}"))?;

        let mut builder = Response::builder().status(status);

        if let Some(ref headers) = result.headers {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        if !builder.headers_ref().is_some_and(|h| h.contains_key("content-type")) {
            builder = builder.header("content-type", "application/json");
        }

        let body = result.body.clone().unwrap_or_else(|| b"{}".to_vec());

        builder
            .body(Body::from(body))
            .map_err(|e| format!("Failed to build response: {e}"))
    }

    /// Apply request modifications to a request
    fn apply_request_modifications(req: Request<Body>, mods: RequestModifications) -> Result<Request<Body>, String> {
        let (mut parts, body) = req.into_parts();

        if let Some(method) = &mods.method {
            parts.method = method.parse().map_err(|e| format!("Invalid method '{method}': {e}"))?;
        }

        if let Some(path) = &mods.path {
            parts.uri = path.parse().map_err(|e| format!("Invalid path '{path}': {e}"))?;
        }

        if let Some(new_headers) = &mods.headers {
            for (key, value) in new_headers {
                let header_name: http::header::HeaderName =
                    key.parse().map_err(|_| format!("Invalid header name: {key}"))?;
                let header_value: http::header::HeaderValue = value
                    .parse()
                    .map_err(|_| format!("Invalid header value for {key}: {value}"))?;
                parts.headers.insert(header_name, header_value);
            }
        }

        let body = mods.body.map_or(body, Body::from);

        Ok(Request::from_parts(parts, body))
    }
}

/// Extract body bytes from an axum Body
///
/// This is a helper used by lifecycle executors to read response bodies.
///
/// # Errors
///
/// Returns an error if body collection fails.
pub async fn extract_body(body: Body) -> Result<Vec<u8>, String> {
    use http_body_util::BodyExt;

    let bytes = body
        .collect()
        .await
        .map_err(|e| format!("Failed to read body: {e}"))?
        .to_bytes();
    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_result_data_continue() {
        let result = HookResultData::continue_execution();
        assert!(result.continue_execution);
        assert_eq!(result.status_code, None);
        assert_eq!(result.body, None);
    }

    #[test]
    fn test_hook_result_data_short_circuit() {
        let body = b"error".to_vec();
        let mut headers = HashMap::new();
        headers.insert("x-error".to_string(), "true".to_string());

        let result = HookResultData::short_circuit(400, body.clone(), Some(headers.clone()));
        assert!(!result.continue_execution);
        assert_eq!(result.status_code, Some(400));
        assert_eq!(result.body, Some(body));
        assert_eq!(result.headers, Some(headers));
    }

    #[test]
    fn test_hook_result_data_modify_request() {
        let mods = RequestModifications {
            method: Some("POST".to_string()),
            path: Some("/new-path".to_string()),
            headers: None,
            body: None,
        };

        let result = HookResultData::modify_request(mods);
        assert!(result.continue_execution);
        assert_eq!(result.status_code, None);
        assert_eq!(
            result.request_modifications.as_ref().unwrap().method,
            Some("POST".to_string())
        );
        assert_eq!(
            result.request_modifications.as_ref().unwrap().path,
            Some("/new-path".to_string())
        );
    }

    #[test]
    fn test_request_modifications_creation() {
        let mods = RequestModifications {
            method: Some("PUT".to_string()),
            path: Some("/api/resource".to_string()),
            headers: None,
            body: Some(b"data".to_vec()),
        };

        assert_eq!(mods.method, Some("PUT".to_string()));
        assert_eq!(mods.path, Some("/api/resource".to_string()));
        assert_eq!(mods.body, Some(b"data".to_vec()));
    }

    struct MockHook {
        result: HookResultData,
    }

    impl LanguageLifecycleHook for MockHook {
        type HookData = ();

        fn prepare_hook_data(&self, _req: &Request<Body>) -> Result<Self::HookData, String> {
            Ok(())
        }

        fn invoke_hook(
            &self,
            _data: Self::HookData,
        ) -> Pin<Box<dyn Future<Output = Result<HookResultData, String>> + Send>> {
            let result = self.result.clone();
            Box::pin(async move { Ok(result) })
        }
    }

    #[tokio::test]
    async fn test_execute_request_hook_continue() {
        let hook = Arc::new(MockHook {
            result: HookResultData::continue_execution(),
        });
        let executor = LifecycleExecutor::new(hook);

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = executor.execute_request_hook(req).await.unwrap();

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_request_hook_short_circuit() {
        let hook = Arc::new(MockHook {
            result: HookResultData::short_circuit(403, b"Forbidden".to_vec(), None),
        });
        let executor = LifecycleExecutor::new(hook);

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = executor.execute_request_hook(req).await.unwrap();

        assert!(result.is_err());
        let response = result.unwrap_err();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_execute_request_hook_modify_request() {
        let mods = RequestModifications {
            method: Some("POST".to_string()),
            path: Some("/new-path".to_string()),
            headers: None,
            body: Some(b"new body".to_vec()),
        };
        let hook = Arc::new(MockHook {
            result: HookResultData::modify_request(mods),
        });
        let executor = LifecycleExecutor::new(hook);

        let req = Request::builder()
            .method("GET")
            .uri("/old-path")
            .body(Body::empty())
            .unwrap();
        let result = executor.execute_request_hook(req).await.unwrap();

        assert!(result.is_ok());
        let modified_req = result.unwrap();
        assert_eq!(modified_req.method(), "POST");
        assert_eq!(modified_req.uri().path(), "/new-path");
    }

    #[tokio::test]
    async fn test_execute_response_hook_with_modifications() {
        let mods = RequestModifications {
            method: None,
            path: None,
            headers: Some({
                let mut h = HashMap::new();
                h.insert("X-Modified".to_string(), "true".to_string());
                h
            }),
            body: Some(b"modified response".to_vec()),
        };
        let hook = Arc::new(MockHook {
            result: HookResultData::modify_request(mods),
        });
        let executor = LifecycleExecutor::new(hook);

        let resp = Response::builder().status(200).body(Body::from("original")).unwrap();
        let result = executor.execute_response_hook(resp).await.unwrap();

        assert_eq!(result.status(), StatusCode::OK);
        assert_eq!(result.headers().get("X-Modified").unwrap().to_str().unwrap(), "true");
    }

    #[tokio::test]
    async fn test_build_response_from_hook_result_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom".to_string(), "value".to_string());

        let result = HookResultData::short_circuit(201, b"Created".to_vec(), Some(headers));

        let response = LifecycleExecutor::<MockHook>::build_response_from_hook_result(&result).unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(response.headers().get("X-Custom").unwrap().to_str().unwrap(), "value");
    }

    #[tokio::test]
    async fn test_build_response_from_hook_result_default_content_type() {
        let result = HookResultData::short_circuit(200, b"{}".to_vec(), None);

        let response = LifecycleExecutor::<MockHook>::build_response_from_hook_result(&result).unwrap();
        assert_eq!(
            response.headers().get("content-type").unwrap().to_str().unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_apply_request_modifications_method() {
        let mods = RequestModifications {
            method: Some("PATCH".to_string()),
            path: None,
            headers: None,
            body: None,
        };

        let req = Request::builder().method("GET").body(Body::empty()).unwrap();
        let modified = LifecycleExecutor::<MockHook>::apply_request_modifications(req, mods).unwrap();

        assert_eq!(modified.method(), "PATCH");
    }

    #[tokio::test]
    async fn test_apply_request_modifications_path() {
        let mods = RequestModifications {
            method: None,
            path: Some("/api/v2/users".to_string()),
            headers: None,
            body: None,
        };

        let req = Request::builder().uri("/api/v1/users").body(Body::empty()).unwrap();
        let modified = LifecycleExecutor::<MockHook>::apply_request_modifications(req, mods).unwrap();

        assert_eq!(modified.uri().path(), "/api/v2/users");
    }

    #[tokio::test]
    async fn test_apply_request_modifications_headers() {
        let mut new_headers = HashMap::new();
        new_headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let mods = RequestModifications {
            method: None,
            path: None,
            headers: Some(new_headers),
            body: None,
        };

        let req = Request::builder().body(Body::empty()).unwrap();
        let modified = LifecycleExecutor::<MockHook>::apply_request_modifications(req, mods).unwrap();

        assert_eq!(
            modified.headers().get("Authorization").unwrap().to_str().unwrap(),
            "Bearer token"
        );
    }

    #[tokio::test]
    async fn test_apply_request_modifications_body() {
        let new_body = b"modified body".to_vec();
        let mods = RequestModifications {
            method: None,
            path: None,
            headers: None,
            body: Some(new_body.clone()),
        };

        let req = Request::builder().body(Body::from("original body")).unwrap();
        let modified = LifecycleExecutor::<MockHook>::apply_request_modifications(req, mods).unwrap();

        let body_bytes = extract_body(modified.into_body()).await.unwrap();
        assert_eq!(body_bytes, new_body);
    }

    #[tokio::test]
    async fn test_apply_request_modifications_invalid_method() {
        let mods = RequestModifications {
            method: Some(String::new()),
            path: None,
            headers: None,
            body: None,
        };

        let req = Request::builder().body(Body::empty()).unwrap();
        let result = LifecycleExecutor::<MockHook>::apply_request_modifications(req, mods);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid method"));
    }

    #[tokio::test]
    async fn test_extract_body_helper() {
        let body = Body::from("test data");
        let bytes = extract_body(body).await.unwrap();

        assert_eq!(bytes, b"test data");
    }

    #[tokio::test]
    async fn test_extract_body_empty() {
        let body = Body::empty();
        let bytes = extract_body(body).await.unwrap();

        assert_eq!(bytes.len(), 0);
    }
}
