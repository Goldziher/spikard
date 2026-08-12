//! Per-route lifecycle hooks: `Route.lifecycle_hooks` names hooks by string, resolved against
//! `ServerConfig.lifecycle_hooks` (the server's registered hook set) by `name()`. This is
//! opt-in, not a refinement of a server-wide default: a hook registered on the server has no
//! effect on a route unless that route names it — the old behavior threaded the same registered
//! hook set into every route unconditionally, which is exactly the bug this covers.

use axum::http::{HeaderValue, StatusCode};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{
    Handler, HandlerResult, HookResult, LifecycleHookRef, LifecycleHooks, LifecycleHooksConfig, RequestData, Route,
    ServerConfig, response_hook,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: axum::http::Request<axum::body::Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(axum::body::Body::from("ok"))
                .unwrap())
        })
    }
}

const HOOK_NAME: &str = "add-response-header";

fn registered_hooks() -> Arc<LifecycleHooks> {
    let hook = response_hook(HOOK_NAME, |mut resp| async move {
        resp.headers_mut()
            .insert("x-response-hook", HeaderValue::from_static("yes"));
        Ok(HookResult::Continue(resp))
    });
    Arc::new(LifecycleHooks::builder().on_response(hook).build())
}

fn hook_selecting_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/with-hook".to_string(),
        handler_name: "ok".to_string(),
        lifecycle_hooks: Some(LifecycleHooksConfig {
            on_response: vec![LifecycleHookRef {
                name: HOOK_NAME.to_string(),
                handler: "run_add_response_header".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn plain_route() -> Route {
    Route {
        method: "GET".parse().unwrap(),
        path: "/without-hook".to_string(),
        handler_name: "ok".to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn route_naming_registered_hook_gets_its_effect() {
    let config = ServerConfig {
        lifecycle_hooks: Some(registered_hooks()),
        ..ServerConfig::default()
    };
    let router = build_router_with_handlers_and_config(
        vec![(hook_selecting_route(), Arc::new(OkHandler) as Arc<dyn Handler>)],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let response = server.get("/with-hook").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.header("x-response-hook").to_str().unwrap(), "yes");
}

#[tokio::test]
async fn route_without_lifecycle_hooks_config_does_not_run_registered_hook() {
    let config = ServerConfig {
        lifecycle_hooks: Some(registered_hooks()),
        ..ServerConfig::default()
    };
    let router = build_router_with_handlers_and_config(
        vec![
            (hook_selecting_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
            (plain_route(), Arc::new(OkHandler) as Arc<dyn Handler>),
        ],
        config,
        Vec::new(),
    )
    .expect("router");

    let server = axum_test::TestServer::new(router);
    let response = server.get("/without-hook").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert!(
        !response.contains_header("x-response-hook"),
        "a route with no `lifecycle_hooks` config must not run a hook that is registered on the \
         server but never named by this route — hooks are opt-in per route, not global"
    );
}

#[tokio::test]
async fn route_naming_an_unregistered_hook_fails_router_build_loudly() {
    let route = Route {
        method: "GET".parse().unwrap(),
        path: "/broken-hook".to_string(),
        handler_name: "ok".to_string(),
        lifecycle_hooks: Some(LifecycleHooksConfig {
            on_request: vec![LifecycleHookRef {
                name: "does-not-exist".to_string(),
                handler: "run_does_not_exist".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    // No hooks registered on the server at all: the route's reference can never resolve.
    let config = ServerConfig::default();
    let result = build_router_with_handlers_and_config(
        vec![(route, Arc::new(OkHandler) as Arc<dyn Handler>)],
        config,
        Vec::new(),
    );

    let err = result.expect_err("a route naming an unregistered lifecycle hook must fail to build, not silently skip");
    assert!(
        err.contains("does-not-exist"),
        "build error should name the unresolved hook, got: {err}"
    );
}

#[tokio::test]
async fn route_naming_a_hook_registered_for_a_different_phase_fails_router_build() {
    let route = Route {
        method: "GET".parse().unwrap(),
        path: "/wrong-phase".to_string(),
        handler_name: "ok".to_string(),
        lifecycle_hooks: Some(LifecycleHooksConfig {
            // `HOOK_NAME` is registered for `on_response`, not `on_request`.
            on_request: vec![LifecycleHookRef {
                name: HOOK_NAME.to_string(),
                handler: "run_add_response_header".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let config = ServerConfig {
        lifecycle_hooks: Some(registered_hooks()),
        ..ServerConfig::default()
    };
    let result = build_router_with_handlers_and_config(
        vec![(route, Arc::new(OkHandler) as Arc<dyn Handler>)],
        config,
        Vec::new(),
    );

    result.expect_err("a hook registered for one phase must not be resolvable from a different phase");
}
