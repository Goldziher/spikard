use axum::body::Body;
use axum::http::{Request, StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use spikard_http::server::build_router_with_handlers_and_config;
use spikard_http::{ApiKeyConfig, Claims, Handler, HandlerResult, JwtConfig, Method, RequestData, Route, ServerConfig};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

struct OkHandler;

impl Handler for OkHandler {
    fn call(
        &self,
        _request: Request<Body>,
        _request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(async move {
            Ok(axum::http::Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("ok"))
                .expect("response"))
        })
    }
}

fn route(method: Method, path: &str, handler_name: &str) -> Route {
    Route {
        method,
        path: path.to_string(),
        handler_name: handler_name.to_string(),
        expects_json_body: false,
        cors: None,
        is_async: true,
        file_params: None,
        request_validator: None,
        response_validator: None,
        parameter_validator: None,
        jsonrpc_method: None,
        #[cfg(feature = "di")]
        handler_dependencies: Vec::new(),
    }
}

fn now_plus(seconds: u64) -> usize {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time");
    usize::try_from((now + Duration::from_secs(seconds)).as_secs()).expect("timestamp fits usize")
}

#[tokio::test]
async fn jwt_auth_layer_rejects_missing_authorization() {
    let config = ServerConfig {
        jwt_auth: Some(JwtConfig {
            secret: "secret".to_string(),
            algorithm: "HS256".to_string(),
            audience: None,
            issuer: None,
            leeway: 0,
        }),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/protected", "ok"),
            Arc::new(OkHandler) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn jwt_auth_layer_accepts_valid_bearer_token() {
    let secret = "secret";
    let config = ServerConfig {
        jwt_auth: Some(JwtConfig {
            secret: secret.to_string(),
            algorithm: "HS256".to_string(),
            audience: None,
            issuer: None,
            leeway: 0,
        }),
        ..Default::default()
    };

    let claims = Claims {
        sub: "user123".to_string(),
        exp: now_plus(60),
        iat: None,
        nbf: None,
        aud: None,
        iss: None,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("token");

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/protected", "ok"),
            Arc::new(OkHandler) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn api_key_auth_layer_rejects_missing_key() {
    let config = ServerConfig {
        api_key_auth: Some(ApiKeyConfig {
            keys: vec!["k1".to_string()],
            header_name: "X-API-Key".to_string(),
        }),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/protected", "ok"),
            Arc::new(OkHandler) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn api_key_auth_layer_accepts_valid_key_from_header() {
    let config = ServerConfig {
        api_key_auth: Some(ApiKeyConfig {
            keys: vec!["k1".to_string()],
            header_name: "X-API-Key".to_string(),
        }),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/protected", "ok"),
            Arc::new(OkHandler) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .header("x-api-key", "k1")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn api_key_auth_layer_accepts_valid_key_from_query_param() {
    let config = ServerConfig {
        api_key_auth: Some(ApiKeyConfig {
            keys: vec!["k1".to_string()],
            header_name: "X-API-Key".to_string(),
        }),
        ..Default::default()
    };

    let router = build_router_with_handlers_and_config(
        vec![(
            route(Method::Get, "/protected", "ok"),
            Arc::new(OkHandler) as Arc<dyn Handler>,
        )],
        config,
        Vec::new(),
    )
    .expect("router");

    let response = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected?api_key=k1")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
}
