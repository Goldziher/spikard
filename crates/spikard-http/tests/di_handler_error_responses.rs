#![cfg(feature = "di")]

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use spikard_core::di::{Dependency, DependencyContainer, DependencyError, ResolvedDependencies};
use spikard_http::di_handler::DependencyInjectingHandler;
use spikard_http::handler_trait::{Handler, HandlerResult, RequestData};
use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move { Ok(axum::http::Response::new(Body::empty())) })
    }
}

fn minimal_request_data() -> RequestData {
    RequestData {
        path_params: Arc::new(HashMap::new()),
        query_params: Arc::new(serde_json::json!({})),
        validated_params: None,
        raw_query_params: Arc::new(HashMap::new()),
        body: Arc::new(serde_json::Value::Null),
        raw_body: None,
        headers: Arc::new(HashMap::new()),
        cookies: Arc::new(HashMap::new()),
        method: "GET".to_string(),
        path: "/".to_string(),
        #[cfg(feature = "di")]
        dependencies: None,
    }
}

async fn read_json_body(resp: axum::http::Response<Body>) -> serde_json::Value {
    let bytes = to_bytes(resp.into_body(), usize::MAX).await.expect("read body");
    serde_json::from_slice(&bytes).expect("parse json")
}

#[tokio::test]
async fn missing_dependency_returns_structured_500() {
    let container = Arc::new(DependencyContainer::new());
    let handler = DependencyInjectingHandler::new(Arc::new(OkHandler), container, vec!["missing".to_string()]);

    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let resp = handler.call(req, minimal_request_data()).await.expect("ok response");

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let json = read_json_body(resp).await;
    assert_eq!(json["title"], "Dependency Resolution Failed");
    assert_eq!(json["errors"][0]["type"], "missing_dependency");
    assert_eq!(json["errors"][0]["dependency_key"], "missing");
}

#[tokio::test]
async fn circular_dependency_returns_structured_500() {
    struct DependsOn {
        dep_key: String,
        deps: Vec<String>,
    }

    impl Dependency for DependsOn {
        fn resolve(
            &self,
            _request: &axum::http::Request<()>,
            _request_data: &spikard_core::RequestData,
            _resolved: &ResolvedDependencies,
        ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
            Box::pin(async move { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) })
        }

        fn key(&self) -> &str {
            &self.dep_key
        }

        fn depends_on(&self) -> Vec<String> {
            self.deps.clone()
        }
    }

    let mut container = DependencyContainer::new();
    container
        .register(
            "a".to_string(),
            Arc::new(DependsOn {
                dep_key: "a".to_string(),
                deps: vec!["b".to_string()],
            }),
        )
        .unwrap();
    container
        .register(
            "b".to_string(),
            Arc::new(DependsOn {
                dep_key: "b".to_string(),
                deps: vec!["a".to_string()],
            }),
        )
        .unwrap();

    let handler = DependencyInjectingHandler::new(Arc::new(OkHandler), Arc::new(container), vec!["a".to_string()]);
    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let resp = handler.call(req, minimal_request_data()).await.expect("ok response");

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let json = read_json_body(resp).await;
    assert_eq!(json["errors"][0]["type"], "circular_dependency");
    assert!(json["errors"][0]["cycle"].is_array());
}

#[tokio::test]
async fn resolution_failed_returns_structured_503() {
    struct FailingDependency;

    impl Dependency for FailingDependency {
        fn resolve(
            &self,
            _request: &axum::http::Request<()>,
            _request_data: &spikard_core::RequestData,
            _resolved: &ResolvedDependencies,
        ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>> {
            Box::pin(async move {
                Err(DependencyError::ResolutionFailed {
                    message: "boom".to_string(),
                })
            })
        }

        fn key(&self) -> &'static str {
            "failing"
        }

        fn depends_on(&self) -> Vec<String> {
            Vec::new()
        }
    }

    let mut container = DependencyContainer::new();
    container
        .register("failing".to_string(), Arc::new(FailingDependency))
        .unwrap();

    let handler =
        DependencyInjectingHandler::new(Arc::new(OkHandler), Arc::new(container), vec!["failing".to_string()]);

    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let resp = handler.call(req, minimal_request_data()).await.expect("ok response");

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    let json = read_json_body(resp).await;
    assert_eq!(json["title"], "Service Unavailable");
    assert_eq!(json["errors"][0]["type"], "resolution_failed");
    assert_eq!(json["errors"][0]["msg"], "boom");
}
