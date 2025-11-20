//! Generated route handlers - one handler per fixture for complete isolation

use axum::body::Body;
use axum::http::{HeaderName, HeaderValue, Response, StatusCode};
use bytes::Bytes;
use futures::stream;
use serde_json::{Value, json};
use spikard::{
    App, AppError, CompressionConfig, CorsConfig, HandlerResponse, HandlerResult, HookResult, LifecycleHook,
    LifecycleHooks, LifecycleHooksBuilder, Method, RateLimitConfig, RequestContext, RouteBuilder, ServerConfig,
    SseEvent, SseEventProducer, StaticFilesConfig, WebSocketHandler, add_cors_headers, delete, get, handle_preflight,
    patch, post, put, request_hook, response_hook, validate_cors_request,
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
type HttpResponse = Response<Body>;

fn apply_expected_headers(mut response: HttpResponse, headers: &[(&str, &str)]) -> HttpResponse {
    for &(name, value) in headers {
        if let Ok(header_name) = HeaderName::from_lowercase(name.as_bytes()) {
            if let Ok(header_value) = HeaderValue::from_str(value) {
                response.headers_mut().insert(header_name, header_value);
            }
        }
    }
    response
}

pub fn create_app() -> Result<App, AppError> {
    Ok(App::new())
}

/// App for fixture: API key authentication - invalid key
pub fn create_app_auth_api_key_authentication_invalid_key() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("auth_api_key_authentication_invalid_key_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), auth_api_key_authentication_invalid_key_handler)?;
    Ok(app)
}

/// App for fixture: API key authentication - missing header
pub fn create_app_auth_api_key_authentication_missing_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/data")
            .handler_name("auth_api_key_authentication_missing_header_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        auth_api_key_authentication_missing_header_handler,
    )?;
    Ok(app)
}

/// App for fixture: API key authentication - valid key
pub fn create_app_auth_api_key_authentication_valid_key() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("auth_api_key_authentication_valid_key_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"description\":\"API key for authentication\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), auth_api_key_authentication_valid_key_handler)?;
    Ok(app)
}

/// App for fixture: API key in query parameter
pub fn create_app_auth_api_key_in_query_parameter() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/data")
            .handler_name("auth_api_key_in_query_parameter_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        auth_api_key_in_query_parameter_handler,
    )?;
    Ok(app)
}

/// App for fixture: API key rotation - old key still valid
pub fn create_app_auth_api_key_rotation_old_key_still_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("auth_api_key_rotation_old_key_still_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"description\":\"API key for authentication\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), auth_api_key_rotation_old_key_still_valid_handler)?;
    Ok(app)
}

/// App for fixture: API key with custom header name
pub fn create_app_auth_api_key_with_custom_header_name() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("auth_api_key_with_custom_header_name_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Token\":{\"description\":\"API token for authentication\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Token\"],\"type\":\"object\"}").unwrap()), auth_api_key_with_custom_header_name_handler)?;
    Ok(app)
}

/// App for fixture: Bearer token without prefix
pub fn create_app_auth_bearer_token_without_prefix() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/protected").handler_name("auth_bearer_token_without_prefix_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_bearer_token_without_prefix_handler)?;
    Ok(app)
}

/// App for fixture: JWT authentication - expired token
pub fn create_app_auth_jwt_authentication_expired_token() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected/user").handler_name("auth_jwt_authentication_expired_token_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_authentication_expired_token_handler)?;
    Ok(app)
}

/// App for fixture: JWT authentication - invalid audience
pub fn create_app_auth_jwt_authentication_invalid_audience() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected/user").handler_name("auth_jwt_authentication_invalid_audience_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_authentication_invalid_audience_handler)?;
    Ok(app)
}

/// App for fixture: JWT authentication - invalid signature
pub fn create_app_auth_jwt_authentication_invalid_signature() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected/user").handler_name("auth_jwt_authentication_invalid_signature_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_authentication_invalid_signature_handler)?;
    Ok(app)
}

/// App for fixture: JWT authentication - missing Authorization header
pub fn create_app_auth_jwt_authentication_missing_authorization_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/protected/user")
            .handler_name("auth_jwt_authentication_missing_authorization_header_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        auth_jwt_authentication_missing_authorization_header_handler,
    )?;
    Ok(app)
}

/// App for fixture: JWT authentication - valid token
pub fn create_app_auth_jwt_authentication_valid_token() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected/user").handler_name("auth_jwt_authentication_valid_token_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_authentication_valid_token_handler)?;
    Ok(app)
}

/// App for fixture: JWT invalid issuer
pub fn create_app_auth_jwt_invalid_issuer() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/protected").handler_name("auth_jwt_invalid_issuer_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_invalid_issuer_handler)?;
    Ok(app)
}

/// App for fixture: JWT malformed token format
pub fn create_app_auth_jwt_malformed_token_format() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/protected").handler_name("auth_jwt_malformed_token_format_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_malformed_token_format_handler)?;
    Ok(app)
}

/// App for fixture: JWT missing required custom claims
pub fn create_app_auth_jwt_missing_required_custom_claims() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/admin").handler_name("auth_jwt_missing_required_custom_claims_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_missing_required_custom_claims_handler)?;
    Ok(app)
}

/// App for fixture: JWT not before claim in future
pub fn create_app_auth_jwt_not_before_claim_in_future() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/protected").handler_name("auth_jwt_not_before_claim_in_future_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_not_before_claim_in_future_handler)?;
    Ok(app)
}

/// App for fixture: JWT with multiple audiences
pub fn create_app_auth_jwt_with_multiple_audiences() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/protected").handler_name("auth_jwt_with_multiple_audiences_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), auth_jwt_with_multiple_audiences_handler)?;
    Ok(app)
}

/// App for fixture: Multiple authentication schemes - JWT precedence
pub fn create_app_auth_multiple_authentication_schemes_jwt_precedence() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("auth_multiple_authentication_schemes_jwt_precedence_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"description\":\"JWT token in Bearer format\",\"source\":\"header\",\"type\":\"string\"},\"X-API-Key\":{\"description\":\"API key for authentication\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\",\"X-API-Key\"],\"type\":\"object\"}").unwrap()), auth_multiple_authentication_schemes_jwt_precedence_handler)?;
    Ok(app)
}

/// App for fixture: Background event logging
pub fn create_app_background_background_event_logging() -> Result<App, AppError> {
    let state: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let mut app = App::new();
    {
        let handler_state = Arc::clone(&state);
        app.route(post("/background/events").handler_name("background_background_event_logging_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"event\":{\"type\":\"string\"}},\"required\":[\"event\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), move |ctx: RequestContext| {
            let handler_state = Arc::clone(&handler_state);
            async move { background_background_event_logging_handler(ctx, handler_state).await }
        })?;
    }
    {
        let state_clone = Arc::clone(&state);
        app.route(
            get("/background/events").handler_name("background_background_event_logging_handler_background_state"),
            move |ctx: RequestContext| {
                let state_clone = Arc::clone(&state_clone);
                async move { background_background_event_logging_handler_background_state(ctx, state_clone).await }
            },
        )?;
    }

    Ok(app)
}

/// App for fixture: Background event logging - second payload
pub fn create_app_background_background_event_logging_second_payload() -> Result<App, AppError> {
    let state: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let mut app = App::new();
    {
        let handler_state = Arc::clone(&state);
        app.route(post("/background/events").handler_name("background_background_event_logging_second_payload_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"event\":{\"type\":\"string\"}},\"required\":[\"event\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), move |ctx: RequestContext| {
            let handler_state = Arc::clone(&handler_state);
            async move { background_background_event_logging_second_payload_handler(ctx, handler_state).await }
        })?;
    }
    {
        let state_clone = Arc::clone(&state);
        app.route(
            get("/background/events")
                .handler_name("background_background_event_logging_second_payload_handler_background_state"),
            move |ctx: RequestContext| {
                let state_clone = Arc::clone(&state_clone);
                async move {
                    background_background_event_logging_second_payload_handler_background_state(ctx, state_clone).await
                }
            },
        )?;
    }

    Ok(app)
}

/// App for fixture: Body over limit returns 413
pub fn create_app_body_limits_body_over_limit_returns_413() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.max_body_size = Some(64);
    let mut app = App::new().config(config);
    app.route(post("/body-limit/over").handler_name("body_limits_body_over_limit_returns_413_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"note\":{\"type\":\"string\"}},\"required\":[\"note\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), body_limits_body_over_limit_returns_413_handler)?;
    Ok(app)
}

/// App for fixture: Body under limit succeeds
pub fn create_app_body_limits_body_under_limit_succeeds() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.max_body_size = Some(64);
    let mut app = App::new().config(config);
    app.route(post("/body-limit/under").handler_name("body_limits_body_under_limit_succeeds_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"note\":{\"type\":\"string\"}},\"required\":[\"note\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), body_limits_body_under_limit_succeeds_handler)?;
    Ok(app)
}

/// App for fixture: Compression - gzip applied
pub fn create_app_compression_compression_gzip_applied() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.compression = Some(CompressionConfig {
        gzip: true,
        brotli: false,
        min_size: 0,
        quality: 4,
    });
    let mut app = App::new().config(config);
    app.route(
        get("/compression/gzip")
            .handler_name("compression_compression_gzip_applied_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        compression_compression_gzip_applied_handler,
    )?;
    Ok(app)
}

/// App for fixture: Compression - payload below min_size is not compressed
pub fn create_app_compression_compression_payload_below_min_size_is_not_compressed() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.compression = Some(CompressionConfig {
        gzip: true,
        brotli: false,
        min_size: 4096,
        quality: 6,
    });
    let mut app = App::new().config(config);
    app.route(
        get("/compression/skip")
            .handler_name("compression_compression_payload_below_min_size_is_not_compressed_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        compression_compression_payload_below_min_size_is_not_compressed_handler,
    )?;
    Ok(app)
}

/// App for fixture: 13_json_with_charset_utf16
pub fn create_app_content_types_13_json_with_charset_utf16() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("content_types_13_json_with_charset_utf16_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_13_json_with_charset_utf16_handler,
    )?;
    Ok(app)
}

/// App for fixture: 14_content_type_case_insensitive
pub fn create_app_content_types_14_content_type_case_insensitive() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("content_types_14_content_type_case_insensitive_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_14_content_type_case_insensitive_handler,
    )?;
    Ok(app)
}

/// App for fixture: 15_multipart_boundary_required
pub fn create_app_content_types_15_multipart_boundary_required() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("content_types_15_multipart_boundary_required_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(serde_json::from_str::<Value>("{\"document\":{\"required\":true}}").unwrap()),
        content_types_15_multipart_boundary_required_handler,
    )?;
    Ok(app)
}

/// App for fixture: 16_text_plain_not_accepted
pub fn create_app_content_types_16_text_plain_not_accepted() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("content_types_16_text_plain_not_accepted_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_16_text_plain_not_accepted_handler,
    )?;
    Ok(app)
}

/// App for fixture: 17_vendor_json_accepted
pub fn create_app_content_types_17_vendor_json_accepted() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/api/v1/resource")
            .handler_name("content_types_17_vendor_json_accepted_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_17_vendor_json_accepted_handler,
    )?;
    Ok(app)
}

/// App for fixture: 18_content_type_with_multiple_params
pub fn create_app_content_types_18_content_type_with_multiple_params() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("content_types_18_content_type_with_multiple_params_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_18_content_type_with_multiple_params_handler,
    )?;
    Ok(app)
}

/// App for fixture: 19_missing_content_type_default_json
pub fn create_app_content_types_19_missing_content_type_default_json() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("content_types_19_missing_content_type_default_json_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_19_missing_content_type_default_json_handler,
    )?;
    Ok(app)
}

/// App for fixture: 20_content_length_mismatch
pub fn create_app_content_types_20_content_length_mismatch() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/data").handler_name("content_types_20_content_length_mismatch_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Content-Length\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), content_types_20_content_length_mismatch_handler)?;
    Ok(app)
}

/// App for fixture: 415 Unsupported Media Type
pub fn create_app_content_types_415_unsupported_media_type() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/items/")
            .handler_name("content_types_415_unsupported_media_type_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"type\":\"string\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_415_unsupported_media_type_handler,
    )?;
    Ok(app)
}

/// App for fixture: Binary response - application/octet-stream
pub fn create_app_content_types_binary_response_application_octet_stream() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/download/file.bin")
            .handler_name("content_types_binary_response_application_octet_stream_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_binary_response_application_octet_stream_handler,
    )?;
    Ok(app)
}

/// App for fixture: CSV response - text/csv
pub fn create_app_content_types_csv_response_text_csv() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/export/data.csv")
            .handler_name("content_types_csv_response_text_csv_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_csv_response_text_csv_handler,
    )?;
    Ok(app)
}

/// App for fixture: Content negotiation - Accept header
pub fn create_app_content_types_content_negotiation_accept_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/accept-test/{id}").handler_name("content_types_content_negotiation_accept_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), content_types_content_negotiation_accept_header_handler)?;
    Ok(app)
}

/// App for fixture: HTML response - text/html
pub fn create_app_content_types_html_response_text_html() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/html")
            .handler_name("content_types_html_response_text_html_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_html_response_text_html_handler,
    )?;
    Ok(app)
}

/// App for fixture: JPEG image response - image/jpeg
pub fn create_app_content_types_jpeg_image_response_image_jpeg() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/images/photo.jpg")
            .handler_name("content_types_jpeg_image_response_image_jpeg_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_jpeg_image_response_image_jpeg_handler,
    )?;
    Ok(app)
}

/// App for fixture: JSON response - application/json
pub fn create_app_content_types_json_response_application_json() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/items/json")
            .handler_name("content_types_json_response_application_json_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_json_response_application_json_handler,
    )?;
    Ok(app)
}

/// App for fixture: JSON with UTF-8 charset
pub fn create_app_content_types_json_with_utf_8_charset() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/items/unicode")
            .handler_name("content_types_json_with_utf_8_charset_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_json_with_utf_8_charset_handler,
    )?;
    Ok(app)
}

/// App for fixture: PDF response - application/pdf
pub fn create_app_content_types_pdf_response_application_pdf() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/download/document.pdf")
            .handler_name("content_types_pdf_response_application_pdf_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_pdf_response_application_pdf_handler,
    )?;
    Ok(app)
}

/// App for fixture: PNG image response - image/png
pub fn create_app_content_types_png_image_response_image_png() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/images/logo.png")
            .handler_name("content_types_png_image_response_image_png_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_png_image_response_image_png_handler,
    )?;
    Ok(app)
}

/// App for fixture: Plain text response - text/plain
pub fn create_app_content_types_plain_text_response_text_plain() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/text")
            .handler_name("content_types_plain_text_response_text_plain_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_plain_text_response_text_plain_handler,
    )?;
    Ok(app)
}

/// App for fixture: XML response - application/xml
pub fn create_app_content_types_xml_response_application_xml() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/xml")
            .handler_name("content_types_xml_response_application_xml_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        content_types_xml_response_application_xml_handler,
    )?;
    Ok(app)
}

/// App for fixture: 24_cookie_samesite_strict
pub fn create_app_cookies_24_cookie_samesite_strict() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/secure").handler_name("cookies_24_cookie_samesite_strict_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"session_id\":{\"samesite\":\"Strict\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap()), cookies_24_cookie_samesite_strict_handler)?;
    Ok(app)
}

/// App for fixture: 25_cookie_samesite_lax
pub fn create_app_cookies_25_cookie_samesite_lax() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/data").handler_name("cookies_25_cookie_samesite_lax_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tracking\":{\"samesite\":\"Lax\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking\"],\"type\":\"object\"}").unwrap()), cookies_25_cookie_samesite_lax_handler)?;
    Ok(app)
}

/// App for fixture: 26_cookie_secure_flag
pub fn create_app_cookies_26_cookie_secure_flag() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/secure").handler_name("cookies_26_cookie_secure_flag_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"auth_token\":{\"secure\":true,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"auth_token\"],\"type\":\"object\"}").unwrap()), cookies_26_cookie_secure_flag_handler)?;
    Ok(app)
}

/// App for fixture: 27_cookie_httponly_flag
pub fn create_app_cookies_27_cookie_httponly_flag() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/secure").handler_name("cookies_27_cookie_httponly_flag_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"session\":{\"httponly\":true,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session\"],\"type\":\"object\"}").unwrap()), cookies_27_cookie_httponly_flag_handler)?;
    Ok(app)
}

/// App for fixture: APIKey cookie authentication - missing
pub fn create_app_cookies_apikey_cookie_authentication_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me/auth").handler_name("cookies_apikey_cookie_authentication_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"key\"],\"type\":\"object\"}").unwrap()), cookies_apikey_cookie_authentication_missing_handler)?;
    Ok(app)
}

/// App for fixture: APIKey cookie authentication - success
pub fn create_app_cookies_apikey_cookie_authentication_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("cookies_apikey_cookie_authentication_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_apikey_cookie_authentication_success_handler)?;
    Ok(app)
}

/// App for fixture: Cookie regex pattern validation - fail
pub fn create_app_cookies_cookie_regex_pattern_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/cookies/pattern").handler_name("cookies_cookie_regex_pattern_validation_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tracking_id\":{\"pattern\":\"^[A-Z0-9]{8}$\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap()), cookies_cookie_regex_pattern_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: Cookie regex pattern validation - success
pub fn create_app_cookies_cookie_regex_pattern_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/cookies/pattern").handler_name("cookies_cookie_regex_pattern_validation_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tracking_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_cookie_regex_pattern_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: Cookie validation - max_length constraint fail
pub fn create_app_cookies_cookie_validation_max_length_constraint_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/cookies/validated").handler_name("cookies_cookie_validation_max_length_constraint_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"session_id\":{\"maxLength\":20,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap()), cookies_cookie_validation_max_length_constraint_fail_handler)?;
    Ok(app)
}

/// App for fixture: Cookie validation - min_length constraint success
pub fn create_app_cookies_cookie_validation_min_length_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/cookies/min-length").handler_name("cookies_cookie_validation_min_length_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"token\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_cookie_validation_min_length_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Cookie validation - min_length failure
pub fn create_app_cookies_cookie_validation_min_length_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("cookies_cookie_validation_min_length_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tracking_id\":{\"minLength\":3,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap()), cookies_cookie_validation_min_length_failure_handler)?;
    Ok(app)
}

/// App for fixture: Multiple cookies - success
pub fn create_app_cookies_multiple_cookies_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("cookies_multiple_cookies_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"fatebook_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"googall_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"session_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_multiple_cookies_success_handler)?;
    Ok(app)
}

/// App for fixture: Optional APIKey cookie - missing
pub fn create_app_cookies_optional_apikey_cookie_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("cookies_optional_apikey_cookie_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_optional_apikey_cookie_missing_handler)?;
    Ok(app)
}

/// App for fixture: Optional cookie parameter - missing
pub fn create_app_cookies_optional_cookie_parameter_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("cookies_optional_cookie_parameter_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ads_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_optional_cookie_parameter_missing_handler)?;
    Ok(app)
}

/// App for fixture: Optional cookie parameter - success
pub fn create_app_cookies_optional_cookie_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("cookies_optional_cookie_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ads_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_optional_cookie_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Required cookie - missing
pub fn create_app_cookies_required_cookie_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/cookies").handler_name("cookies_required_cookie_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"fatebook_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"session_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap()), cookies_required_cookie_missing_handler)?;
    Ok(app)
}

/// App for fixture: Response - delete cookie
pub fn create_app_cookies_response_delete_cookie() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/delete").handler_name("cookies_response_delete_cookie_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"session\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_delete_cookie_handler)?;
    Ok(app)
}

/// App for fixture: Response - multiple cookies
pub fn create_app_cookies_response_multiple_cookies() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/multiple").handler_name("cookies_response_multiple_cookies_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"session\":{\"type\":\"string\"},\"user\":{\"type\":\"string\"}},\"required\":[\"user\",\"session\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_multiple_cookies_handler)?;
    Ok(app)
}

/// App for fixture: Response - session cookie (no max_age)
pub fn create_app_cookies_response_session_cookie_no_max_age() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/session").handler_name("cookies_response_session_cookie_no_max_age_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_session_cookie_no_max_age_handler)?;
    Ok(app)
}

/// App for fixture: Response cookie with SameSite=Lax
pub fn create_app_cookies_response_cookie_with_samesite_lax() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/samesite-lax").handler_name("cookies_response_cookie_with_samesite_lax_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_cookie_with_samesite_lax_handler)?;
    Ok(app)
}

/// App for fixture: Response cookie with SameSite=None
pub fn create_app_cookies_response_cookie_with_samesite_none() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/samesite-none").handler_name("cookies_response_cookie_with_samesite_none_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_cookie_with_samesite_none_handler)?;
    Ok(app)
}

/// App for fixture: Response cookie with SameSite=Strict
pub fn create_app_cookies_response_cookie_with_samesite_strict() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/samesite-strict").handler_name("cookies_response_cookie_with_samesite_strict_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_cookie_with_samesite_strict_handler)?;
    Ok(app)
}

/// App for fixture: Response cookie with attributes
pub fn create_app_cookies_response_cookie_with_attributes() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/cookie/set")
            .handler_name("cookies_response_cookie_with_attributes_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cookies_response_cookie_with_attributes_handler,
    )?;
    Ok(app)
}

/// App for fixture: Response cookie with domain attribute
pub fn create_app_cookies_response_cookie_with_domain_attribute() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/set-with-domain").handler_name("cookies_response_cookie_with_domain_attribute_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_cookie_with_domain_attribute_handler)?;
    Ok(app)
}

/// App for fixture: Response cookie with path attribute
pub fn create_app_cookies_response_cookie_with_path_attribute() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/cookies/set-with-path").handler_name("cookies_response_cookie_with_path_attribute_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), cookies_response_cookie_with_path_attribute_handler)?;
    Ok(app)
}

/// App for fixture: Response set cookie - basic
pub fn create_app_cookies_response_set_cookie_basic() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/cookie/")
            .handler_name("cookies_response_set_cookie_basic_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cookies_response_set_cookie_basic_handler,
    )?;
    Ok(app)
}

/// App for fixture: 06_cors_preflight_method_not_allowed
pub fn create_app_cors_06_cors_preflight_method_not_allowed() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(RouteBuilder::new(Method::from_str("OPTIONS").expect("invalid method"), "/api/data").handler_name("cors_06_cors_preflight_method_not_allowed_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"GET\",\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Access-Control-Request-Headers\":{\"source\":\"header\",\"type\":\"string\"},\"Access-Control-Request-Method\":{\"source\":\"header\",\"type\":\"string\"},\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_06_cors_preflight_method_not_allowed_handler)?;
    Ok(app)
}

/// App for fixture: 07_cors_preflight_header_not_allowed
pub fn create_app_cors_07_cors_preflight_header_not_allowed() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(RouteBuilder::new(Method::from_str("OPTIONS").expect("invalid method"), "/api/data").handler_name("cors_07_cors_preflight_header_not_allowed_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Access-Control-Request-Headers\":{\"source\":\"header\",\"type\":\"string\"},\"Access-Control-Request-Method\":{\"source\":\"header\",\"type\":\"string\"},\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_07_cors_preflight_header_not_allowed_handler)?;
    Ok(app)
}

/// App for fixture: 08_cors_max_age
pub fn create_app_cors_08_cors_max_age() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(RouteBuilder::new(Method::from_str("OPTIONS").expect("invalid method"), "/api/data").handler_name("cors_08_cors_max_age_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"POST\"],\"allowed_origins\":[\"https://example.com\"],\"max_age\":3600}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Access-Control-Request-Headers\":{\"source\":\"header\",\"type\":\"string\"},\"Access-Control-Request-Method\":{\"source\":\"header\",\"type\":\"string\"},\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_08_cors_max_age_handler)?;
    Ok(app)
}

/// App for fixture: 09_cors_expose_headers
pub fn create_app_cors_09_cors_expose_headers() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("cors_09_cors_expose_headers_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_methods\":[\"GET\"],\"allowed_origins\":[\"https://example.com\"],\"expose_headers\":[\"X-Total-Count\",\"X-Request-Id\"]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_09_cors_expose_headers_handler)?;
    Ok(app)
}

/// App for fixture: 10_cors_origin_null
pub fn create_app_cors_10_cors_origin_null() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("cors_10_cors_origin_null_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_methods\":[\"GET\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_10_cors_origin_null_handler)?;
    Ok(app)
}

/// App for fixture: CORS Private Network Access
pub fn create_app_cors_cors_private_network_access() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(
            Method::from_str("OPTIONS").expect("invalid method"),
            "/api/local-resource",
        )
        .handler_name("cors_cors_private_network_access_handler")
        .params_schema_json(
            serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
        ),
        cors_cors_private_network_access_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS Vary header for proper caching
pub fn create_app_cors_cors_vary_header_for_proper_caching() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/cached-resource")
            .handler_name("cors_cors_vary_header_for_proper_caching_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_vary_header_for_proper_caching_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS multiple allowed origins
pub fn create_app_cors_cors_multiple_allowed_origins() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/data")
            .handler_name("cors_cors_multiple_allowed_origins_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_multiple_allowed_origins_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS origin case sensitivity
pub fn create_app_cors_cors_origin_case_sensitivity() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/data")
            .handler_name("cors_cors_origin_case_sensitivity_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_origin_case_sensitivity_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS preflight for DELETE method
pub fn create_app_cors_cors_preflight_for_delete_method() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(
            Method::from_str("OPTIONS").expect("invalid method"),
            "/api/resource/456",
        )
        .handler_name("cors_cors_preflight_for_delete_method_handler")
        .params_schema_json(
            serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
        ),
        cors_cors_preflight_for_delete_method_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS preflight for PUT method
pub fn create_app_cors_cors_preflight_for_put_method() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(
            Method::from_str("OPTIONS").expect("invalid method"),
            "/api/resource/123",
        )
        .handler_name("cors_cors_preflight_for_put_method_handler")
        .params_schema_json(
            serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
        ),
        cors_cors_preflight_for_put_method_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS preflight request
pub fn create_app_cors_cors_preflight_request() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(Method::from_str("OPTIONS").expect("invalid method"), "/items/")
            .handler_name("cors_cors_preflight_request_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_preflight_request_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS regex pattern matching for origins
pub fn create_app_cors_cors_regex_pattern_matching_for_origins() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/data")
            .handler_name("cors_cors_regex_pattern_matching_for_origins_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_regex_pattern_matching_for_origins_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS request blocked
pub fn create_app_cors_cors_request_blocked() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("cors_cors_request_blocked_handler").cors(serde_json::from_str::<CorsConfig>("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"GET\",\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), cors_cors_request_blocked_handler)?;
    Ok(app)
}

/// App for fixture: CORS safelisted headers without preflight
pub fn create_app_cors_cors_safelisted_headers_without_preflight() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/api/form")
            .handler_name("cors_cors_safelisted_headers_without_preflight_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_safelisted_headers_without_preflight_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS wildcard origin
pub fn create_app_cors_cors_wildcard_origin() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/public/data")
            .handler_name("cors_cors_wildcard_origin_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_wildcard_origin_handler,
    )?;
    Ok(app)
}

/// App for fixture: CORS with credentials
pub fn create_app_cors_cors_with_credentials() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/user/profile")
            .handler_name("cors_cors_with_credentials_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_cors_with_credentials_handler,
    )?;
    Ok(app)
}

/// App for fixture: Simple CORS request
pub fn create_app_cors_simple_cors_request() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/items/")
            .handler_name("cors_simple_cors_request_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        cors_simple_cors_request_handler,
    )?;
    Ok(app)
}

/// App for fixture: 11_utf8_query_parameter
pub fn create_app_edge_cases_11_utf8_query_parameter() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("edge_cases_11_utf8_query_parameter_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"term\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap()), edge_cases_11_utf8_query_parameter_handler)?;
    Ok(app)
}

/// App for fixture: 12_percent_encoded_special_chars
pub fn create_app_edge_cases_12_percent_encoded_special_chars() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("edge_cases_12_percent_encoded_special_chars_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"term\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap()), edge_cases_12_percent_encoded_special_chars_handler)?;
    Ok(app)
}

/// App for fixture: 13_empty_string_query_param_preserved
pub fn create_app_edge_cases_13_empty_string_query_param_preserved() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("edge_cases_13_empty_string_query_param_preserved_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"filter\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"filter\"],\"type\":\"object\"}").unwrap()), edge_cases_13_empty_string_query_param_preserved_handler)?;
    Ok(app)
}

/// App for fixture: 14_large_integer_boundary
pub fn create_app_edge_cases_14_large_integer_boundary() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("edge_cases_14_large_integer_boundary_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), edge_cases_14_large_integer_boundary_handler)?;
    Ok(app)
}

/// App for fixture: 15_float_precision_preservation
pub fn create_app_edge_cases_15_float_precision_preservation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/calculate")
            .handler_name("edge_cases_15_float_precision_preservation_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"value\":{\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        edge_cases_15_float_precision_preservation_handler,
    )?;
    Ok(app)
}

/// App for fixture: 16_negative_zero_handling
pub fn create_app_edge_cases_16_negative_zero_handling() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("edge_cases_16_negative_zero_handling_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"offset\":{\"type\":\"number\"}},\"required\":[\"offset\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        edge_cases_16_negative_zero_handling_handler,
    )?;
    Ok(app)
}

/// App for fixture: 17_extremely_long_string
pub fn create_app_edge_cases_17_extremely_long_string() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/text").handler_name("edge_cases_17_extremely_long_string_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"content\":{\"maxLength\":10000,\"type\":\"string\"}},\"required\":[\"content\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_17_extremely_long_string_handler)?;
    Ok(app)
}

/// App for fixture: 18_unicode_normalization
pub fn create_app_edge_cases_18_unicode_normalization() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("edge_cases_18_unicode_normalization_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_18_unicode_normalization_handler)?;
    Ok(app)
}

/// App for fixture: 19_emoji_in_strings
pub fn create_app_edge_cases_19_emoji_in_strings() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/messages").handler_name("edge_cases_19_emoji_in_strings_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"text\":{\"maxLength\":100,\"minLength\":1,\"type\":\"string\"}},\"required\":[\"text\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_19_emoji_in_strings_handler)?;
    Ok(app)
}

/// App for fixture: 20_null_byte_in_string
pub fn create_app_edge_cases_20_null_byte_in_string() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files").handler_name("edge_cases_20_null_byte_in_string_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"filename\":{\"pattern\":\"^[^\\\\x00]+$\",\"type\":\"string\"}},\"required\":[\"filename\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_20_null_byte_in_string_handler)?;
    Ok(app)
}

/// App for fixture: 21_scientific_notation_number
pub fn create_app_edge_cases_21_scientific_notation_number() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/calculate").handler_name("edge_cases_21_scientific_notation_number_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_21_scientific_notation_number_handler)?;
    Ok(app)
}

/// App for fixture: 22_leading_zeros_integer
pub fn create_app_edge_cases_22_leading_zeros_integer() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/data").handler_name("edge_cases_22_leading_zeros_integer_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), edge_cases_22_leading_zeros_integer_handler)?;
    Ok(app)
}

/// App for fixture: 23_deeply_nested_json_limit
pub fn create_app_edge_cases_23_deeply_nested_json_limit() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/data")
            .handler_name("edge_cases_23_deeply_nested_json_limit_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"type\":\"object\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        edge_cases_23_deeply_nested_json_limit_handler,
    )?;
    Ok(app)
}

/// App for fixture: 24_array_with_holes
pub fn create_app_edge_cases_24_array_with_holes() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items").handler_name("edge_cases_24_array_with_holes_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"items\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"items\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_24_array_with_holes_handler)?;
    Ok(app)
}

/// App for fixture: Deeply nested structure (10+ levels)
pub fn create_app_edge_cases_deeply_nested_structure_10_levels() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/nested/").handler_name("edge_cases_deeply_nested_structure_10_levels_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"level1\":{\"additionalProperties\":false,\"properties\":{\"level2\":{\"additionalProperties\":false,\"properties\":{\"level3\":{\"additionalProperties\":false,\"properties\":{\"level4\":{\"additionalProperties\":false,\"properties\":{\"level5\":{\"additionalProperties\":false,\"properties\":{\"level6\":{\"additionalProperties\":false,\"properties\":{\"level7\":{\"additionalProperties\":false,\"properties\":{\"level8\":{\"additionalProperties\":false,\"properties\":{\"level9\":{\"additionalProperties\":false,\"properties\":{\"level10\":{\"additionalProperties\":false,\"properties\":{\"depth\":{\"type\":\"integer\"},\"value\":{\"type\":\"string\"}},\"required\":[\"value\",\"depth\"],\"type\":\"object\"}},\"required\":[\"level10\"],\"type\":\"object\"}},\"required\":[\"level9\"],\"type\":\"object\"}},\"required\":[\"level8\"],\"type\":\"object\"}},\"required\":[\"level7\"],\"type\":\"object\"}},\"required\":[\"level6\"],\"type\":\"object\"}},\"required\":[\"level5\"],\"type\":\"object\"}},\"required\":[\"level4\"],\"type\":\"object\"}},\"required\":[\"level3\"],\"type\":\"object\"}},\"required\":[\"level2\"],\"type\":\"object\"}},\"required\":[\"level1\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_deeply_nested_structure_10_levels_handler)?;
    Ok(app)
}

/// App for fixture: Empty and null value handling
pub fn create_app_edge_cases_empty_and_null_value_handling() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/nulls/").handler_name("edge_cases_empty_and_null_value_handling_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"empty_array\":{\"items\":{},\"type\":\"array\"},\"empty_object\":{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"},\"empty_string\":{\"type\":\"string\"},\"explicit_null\":{\"type\":\"null\"},\"false_boolean\":{\"type\":\"boolean\"},\"zero_number\":{\"type\":\"integer\"}},\"required\":[\"explicit_null\",\"empty_string\",\"empty_array\",\"empty_object\",\"zero_number\",\"false_boolean\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_empty_and_null_value_handling_handler)?;
    Ok(app)
}

/// App for fixture: Float precision and rounding
pub fn create_app_edge_cases_float_precision_and_rounding() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/calculations/").handler_name("edge_cases_float_precision_and_rounding_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"expected_sum\":{\"type\":\"number\"},\"precise_value\":{\"type\":\"number\"},\"value1\":{\"type\":\"number\"},\"value2\":{\"type\":\"number\"},\"very_large\":{\"type\":\"number\"},\"very_small\":{\"type\":\"number\"}},\"required\":[\"value1\",\"value2\",\"expected_sum\",\"precise_value\",\"very_small\",\"very_large\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_float_precision_and_rounding_handler)?;
    Ok(app)
}

/// App for fixture: Large integer boundary values
pub fn create_app_edge_cases_large_integer_boundary_values() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/numbers/").handler_name("edge_cases_large_integer_boundary_values_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"large_int\":{\"type\":\"integer\"},\"max_safe_int\":{\"type\":\"integer\"},\"negative_large\":{\"type\":\"integer\"}},\"required\":[\"max_safe_int\",\"large_int\",\"negative_large\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_large_integer_boundary_values_handler)?;
    Ok(app)
}

/// App for fixture: Special string values and escaping
pub fn create_app_edge_cases_special_string_values_and_escaping() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/strings/").handler_name("edge_cases_special_string_values_and_escaping_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"backslashes\":{\"type\":\"string\"},\"empty_string\":{\"type\":\"string\"},\"quotes\":{\"type\":\"string\"},\"special_chars\":{\"type\":\"string\"},\"tabs_newlines\":{\"type\":\"string\"},\"unicode_escapes\":{\"type\":\"string\"},\"whitespace\":{\"type\":\"string\"}},\"required\":[\"empty_string\",\"whitespace\",\"tabs_newlines\",\"quotes\",\"backslashes\",\"unicode_escapes\",\"special_chars\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_special_string_values_and_escaping_handler)?;
    Ok(app)
}

/// App for fixture: Unicode and emoji handling
pub fn create_app_edge_cases_unicode_and_emoji_handling() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("edge_cases_unicode_and_emoji_handling_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"emoji_reactions\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"description\",\"tags\",\"emoji_reactions\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), edge_cases_unicode_and_emoji_handling_handler)?;
    Ok(app)
}

/// App for fixture: 30_bearer_token_format_valid
pub fn create_app_headers_30_bearer_token_format_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected").handler_name("headers_30_bearer_token_format_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_30_bearer_token_format_valid_handler)?;
    Ok(app)
}

/// App for fixture: 31_bearer_token_format_invalid
pub fn create_app_headers_31_bearer_token_format_invalid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected").handler_name("headers_31_bearer_token_format_invalid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_31_bearer_token_format_invalid_handler)?;
    Ok(app)
}

/// App for fixture: 32_bearer_token_missing_prefix
pub fn create_app_headers_32_bearer_token_missing_prefix() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/protected").handler_name("headers_32_bearer_token_missing_prefix_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_32_bearer_token_missing_prefix_handler)?;
    Ok(app)
}

/// App for fixture: 33_api_key_header_valid
pub fn create_app_headers_33_api_key_header_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("headers_33_api_key_header_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), headers_33_api_key_header_valid_handler)?;
    Ok(app)
}

/// App for fixture: 34_api_key_header_invalid
pub fn create_app_headers_34_api_key_header_invalid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/api/data").handler_name("headers_34_api_key_header_invalid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), headers_34_api_key_header_invalid_handler)?;
    Ok(app)
}

/// App for fixture: Accept header - JSON
pub fn create_app_headers_accept_header_json() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/accept").handler_name("headers_accept_header_json_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Accept\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept\"],\"type\":\"object\"}").unwrap()), headers_accept_header_json_handler)?;
    Ok(app)
}

/// App for fixture: Accept-Encoding header
pub fn create_app_headers_accept_encoding_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/accept-encoding").handler_name("headers_accept_encoding_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Accept-Encoding\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept-Encoding\"],\"type\":\"object\"}").unwrap()), headers_accept_encoding_header_handler)?;
    Ok(app)
}

/// App for fixture: Accept-Language header
pub fn create_app_headers_accept_language_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/accept-language").handler_name("headers_accept_language_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Accept-Language\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept-Language\"],\"type\":\"object\"}").unwrap()), headers_accept_language_header_handler)?;
    Ok(app)
}

/// App for fixture: Authorization header - missing
pub fn create_app_headers_authorization_header_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_authorization_header_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_authorization_header_missing_handler)?;
    Ok(app)
}

/// App for fixture: Authorization header - success
pub fn create_app_headers_authorization_header_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_authorization_header_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_authorization_header_success_handler)?;
    Ok(app)
}

/// App for fixture: Authorization header - wrong scheme
pub fn create_app_headers_authorization_header_wrong_scheme() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_authorization_header_wrong_scheme_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"pattern\":\"^Digest .+\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_authorization_header_wrong_scheme_handler)?;
    Ok(app)
}

/// App for fixture: Basic authentication - success
pub fn create_app_headers_basic_authentication_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/basic-auth").handler_name("headers_basic_authentication_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_basic_authentication_success_handler)?;
    Ok(app)
}

/// App for fixture: Bearer token authentication - missing
pub fn create_app_headers_bearer_token_authentication_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/bearer-auth").handler_name("headers_bearer_token_authentication_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer .+\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_bearer_token_authentication_missing_handler)?;
    Ok(app)
}

/// App for fixture: Bearer token authentication - success
pub fn create_app_headers_bearer_token_authentication_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/bearer-auth").handler_name("headers_bearer_token_authentication_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap()), headers_bearer_token_authentication_success_handler)?;
    Ok(app)
}

/// App for fixture: Content-Type header - application/json
pub fn create_app_headers_content_type_header_application_json() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/content-type").handler_name("headers_content_type_header_application_json_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Content-Type\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Content-Type\"],\"type\":\"object\"}").unwrap()), headers_content_type_header_application_json_handler)?;
    Ok(app)
}

/// App for fixture: Header case insensitivity - access
pub fn create_app_headers_header_case_insensitivity_access() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/echo").handler_name("headers_header_case_insensitivity_access_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"test\":{\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), headers_header_case_insensitivity_access_handler)?;
    Ok(app)
}

/// App for fixture: Header regex validation - fail
pub fn create_app_headers_header_regex_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/pattern").handler_name("headers_header_regex_validation_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Request-Id\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Request-Id\"],\"type\":\"object\"}").unwrap()), headers_header_regex_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: Header regex validation - success
pub fn create_app_headers_header_regex_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/pattern").handler_name("headers_header_regex_validation_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Request-Id\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Request-Id\"],\"type\":\"object\"}").unwrap()), headers_header_regex_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: Header validation - max_length constraint fail
pub fn create_app_headers_header_validation_max_length_constraint_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/max-length").handler_name("headers_header_validation_max_length_constraint_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Session-Id\":{\"maxLength\":20,\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Session-Id\"],\"type\":\"object\"}").unwrap()), headers_header_validation_max_length_constraint_fail_handler)?;
    Ok(app)
}

/// App for fixture: Header validation - min_length constraint
pub fn create_app_headers_header_validation_min_length_constraint() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/validated").handler_name("headers_header_validation_min_length_constraint_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Token\":{\"minLength\":3,\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Token\"],\"type\":\"object\"}").unwrap()), headers_header_validation_min_length_constraint_handler)?;
    Ok(app)
}

/// App for fixture: Header with underscore conversion - explicit
pub fn create_app_headers_header_with_underscore_conversion_explicit() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/underscore").handler_name("headers_header_with_underscore_conversion_explicit_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Token\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Token\"],\"type\":\"object\"}").unwrap()), headers_header_with_underscore_conversion_explicit_handler)?;
    Ok(app)
}

/// App for fixture: Host header
pub fn create_app_headers_host_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/host").handler_name("headers_host_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Host\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Host\"],\"type\":\"object\"}").unwrap()), headers_host_header_handler)?;
    Ok(app)
}

/// App for fixture: Multiple custom headers
pub fn create_app_headers_multiple_custom_headers() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/multiple").handler_name("headers_multiple_custom_headers_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Client-Version\":{\"source\":\"header\",\"type\":\"string\"},\"X-Request-Id\":{\"source\":\"header\",\"type\":\"string\"},\"X-Trace-Id\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Client-Version\",\"X-Request-Id\",\"X-Trace-Id\"],\"type\":\"object\"}").unwrap()), headers_multiple_custom_headers_handler)?;
    Ok(app)
}

/// App for fixture: Multiple header values - X-Token
pub fn create_app_headers_multiple_header_values_x_token() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("headers_multiple_header_values_x_token_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"x-token\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"x-token\"],\"type\":\"object\"}").unwrap()), headers_multiple_header_values_x_token_handler)?;
    Ok(app)
}

/// App for fixture: Optional header with None default - missing
pub fn create_app_headers_optional_header_with_none_default_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("headers_optional_header_with_none_default_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"strange-header\":{\"default\":null,\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), headers_optional_header_with_none_default_missing_handler)?;
    Ok(app)
}

/// App for fixture: Origin header
pub fn create_app_headers_origin_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/origin").handler_name("headers_origin_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Origin\"],\"type\":\"object\"}").unwrap()), headers_origin_header_handler)?;
    Ok(app)
}

/// App for fixture: Referer header
pub fn create_app_headers_referer_header() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/headers/referer").handler_name("headers_referer_header_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"Referer\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Referer\"],\"type\":\"object\"}").unwrap()), headers_referer_header_handler)?;
    Ok(app)
}

/// App for fixture: User-Agent header - custom value
pub fn create_app_headers_user_agent_header_custom_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("headers_user_agent_header_custom_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"User-Agent\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"User-Agent\"],\"type\":\"object\"}").unwrap()), headers_user_agent_header_custom_value_handler)?;
    Ok(app)
}

/// App for fixture: User-Agent header - default value
pub fn create_app_headers_user_agent_header_default_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("headers_user_agent_header_default_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"User-Agent\":{\"default\":\"testclient\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), headers_user_agent_header_default_value_handler)?;
    Ok(app)
}

/// App for fixture: X-API-Key optional header - missing
pub fn create_app_headers_x_api_key_optional_header_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_x_api_key_optional_header_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), headers_x_api_key_optional_header_missing_handler)?;
    Ok(app)
}

/// App for fixture: X-API-Key optional header - success
pub fn create_app_headers_x_api_key_optional_header_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_x_api_key_optional_header_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), headers_x_api_key_optional_header_success_handler)?;
    Ok(app)
}

/// App for fixture: X-API-Key required header - missing
pub fn create_app_headers_x_api_key_required_header_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_x_api_key_required_header_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-API-Key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap()), headers_x_api_key_required_header_missing_handler)?;
    Ok(app)
}

/// App for fixture: X-API-Key required header - success
pub fn create_app_headers_x_api_key_required_header_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/me").handler_name("headers_x_api_key_required_header_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"key\"],\"type\":\"object\"}").unwrap()), headers_x_api_key_required_header_success_handler)?;
    Ok(app)
}

/// App for fixture: DELETE - Remove resource
pub fn create_app_http_methods_delete_remove_resource() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(delete("/items/{id}").handler_name("http_methods_delete_remove_resource_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_delete_remove_resource_handler)?;
    Ok(app)
}

/// App for fixture: DELETE - Resource not found
pub fn create_app_http_methods_delete_resource_not_found() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(delete("/items/{id}").handler_name("http_methods_delete_resource_not_found_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_delete_resource_not_found_handler)?;
    Ok(app)
}

/// App for fixture: DELETE - With response body
pub fn create_app_http_methods_delete_with_response_body() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(delete("/items/{id}").handler_name("http_methods_delete_with_response_body_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_delete_with_response_body_handler)?;
    Ok(app)
}

/// App for fixture: HEAD - Get metadata without body
pub fn create_app_http_methods_head_get_metadata_without_body() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(RouteBuilder::new(Method::from_str("HEAD").expect("invalid method"), "/items/{id}").handler_name("http_methods_head_get_metadata_without_body_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_head_get_metadata_without_body_handler)?;
    Ok(app)
}

/// App for fixture: OPTIONS - CORS preflight request
pub fn create_app_http_methods_options_cors_preflight_request() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(Method::from_str("OPTIONS").expect("invalid method"), "/items/")
            .handler_name("http_methods_options_cors_preflight_request_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        http_methods_options_cors_preflight_request_handler,
    )?;
    Ok(app)
}

/// App for fixture: PATCH - Partial update
pub fn create_app_http_methods_patch_partial_update() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(patch("/items/{id}").handler_name("http_methods_patch_partial_update_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"price\":{\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_patch_partial_update_handler)?;
    Ok(app)
}

/// App for fixture: PATCH - Update multiple fields
pub fn create_app_http_methods_patch_update_multiple_fields() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(patch("/items/{id}").handler_name("http_methods_patch_update_multiple_fields_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"in_stock\",\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_patch_update_multiple_fields_handler)?;
    Ok(app)
}

/// App for fixture: PUT - Complete resource replacement
pub fn create_app_http_methods_put_complete_resource_replacement() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(put("/items/{id}").handler_name("http_methods_put_complete_resource_replacement_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"description\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\"},\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"description\",\"id\",\"in_stock\",\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_put_complete_resource_replacement_handler)?;
    Ok(app)
}

/// App for fixture: PUT - Create resource if doesn't exist
pub fn create_app_http_methods_put_create_resource_if_doesn_t_exist() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(put("/items/{id}").handler_name("http_methods_put_create_resource_if_doesn_t_exist_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_put_create_resource_if_doesn_t_exist_handler)?;
    Ok(app)
}

/// App for fixture: PUT - Idempotent operation
pub fn create_app_http_methods_put_idempotent_operation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(put("/items/{id}").handler_name("http_methods_put_idempotent_operation_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_put_idempotent_operation_handler)?;
    Ok(app)
}

/// App for fixture: PUT - Missing required field
pub fn create_app_http_methods_put_missing_required_field() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(put("/items/{id}").handler_name("http_methods_put_missing_required_field_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_put_missing_required_field_handler)?;
    Ok(app)
}

/// App for fixture: PUT - Validation error
pub fn create_app_http_methods_put_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(put("/items/{id}").handler_name("http_methods_put_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\",\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"minLength\":3,\"type\":\"string\"},\"price\":{\"exclusiveMinimum\":0,\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), http_methods_put_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: 29_nested_object_validation_success
pub fn create_app_json_bodies_29_nested_object_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_29_nested_object_validation_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_29_nested_object_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: 30_nested_object_missing_field
pub fn create_app_json_bodies_30_nested_object_missing_field() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_30_nested_object_missing_field_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_30_nested_object_missing_field_handler)?;
    Ok(app)
}

/// App for fixture: 31_nullable_property_null_value
pub fn create_app_json_bodies_31_nullable_property_null_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_31_nullable_property_null_value_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"description\":{\"type\":[\"string\",\"null\"]},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_31_nullable_property_null_value_handler)?;
    Ok(app)
}

/// App for fixture: 32_schema_ref_definitions
pub fn create_app_json_bodies_32_schema_ref_definitions() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/products").handler_name("json_bodies_32_schema_ref_definitions_handler").request_schema_json(serde_json::from_str::<Value>("{\"definitions\":{\"Product\":{\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}},\"properties\":{\"product\":{\"$ref\":\"#/definitions/Product\"}},\"required\":[\"product\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_32_schema_ref_definitions_handler)?;
    Ok(app)
}

/// App for fixture: 33_allof_schema_composition
pub fn create_app_json_bodies_33_allof_schema_composition() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items").handler_name("json_bodies_33_allof_schema_composition_handler").request_schema_json(serde_json::from_str::<Value>("{\"allOf\":[{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"},{\"properties\":{\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_33_allof_schema_composition_handler)?;
    Ok(app)
}

/// App for fixture: 34_additional_properties_false
pub fn create_app_json_bodies_34_additional_properties_false() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_34_additional_properties_false_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"email\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_34_additional_properties_false_handler)?;
    Ok(app)
}

/// App for fixture: 35_oneof_schema_success
pub fn create_app_json_bodies_35_oneof_schema_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/payment").handler_name("json_bodies_35_oneof_schema_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_35_oneof_schema_success_handler)?;
    Ok(app)
}

/// App for fixture: 36_oneof_schema_multiple_match_failure
pub fn create_app_json_bodies_36_oneof_schema_multiple_match_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/payment").handler_name("json_bodies_36_oneof_schema_multiple_match_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_36_oneof_schema_multiple_match_failure_handler)?;
    Ok(app)
}

/// App for fixture: 37_oneof_schema_no_match_failure
pub fn create_app_json_bodies_37_oneof_schema_no_match_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/payment").handler_name("json_bodies_37_oneof_schema_no_match_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_37_oneof_schema_no_match_failure_handler)?;
    Ok(app)
}

/// App for fixture: 38_anyof_schema_success
pub fn create_app_json_bodies_38_anyof_schema_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/contact").handler_name("json_bodies_38_anyof_schema_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_38_anyof_schema_success_handler)?;
    Ok(app)
}

/// App for fixture: 39_anyof_schema_multiple_match_success
pub fn create_app_json_bodies_39_anyof_schema_multiple_match_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/contact").handler_name("json_bodies_39_anyof_schema_multiple_match_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_39_anyof_schema_multiple_match_success_handler)?;
    Ok(app)
}

/// App for fixture: 40_anyof_schema_failure
pub fn create_app_json_bodies_40_anyof_schema_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/contact").handler_name("json_bodies_40_anyof_schema_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_40_anyof_schema_failure_handler)?;
    Ok(app)
}

/// App for fixture: 41_not_schema_success
pub fn create_app_json_bodies_41_not_schema_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_41_not_schema_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_41_not_schema_success_handler)?;
    Ok(app)
}

/// App for fixture: 42_not_schema_failure
pub fn create_app_json_bodies_42_not_schema_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("json_bodies_42_not_schema_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_42_not_schema_failure_handler)?;
    Ok(app)
}

/// App for fixture: 43_const_validation_success
pub fn create_app_json_bodies_43_const_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/api/v1/data").handler_name("json_bodies_43_const_validation_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_43_const_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: 44_const_validation_failure
pub fn create_app_json_bodies_44_const_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/api/v1/data").handler_name("json_bodies_44_const_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_44_const_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 45_minproperties_validation_success
pub fn create_app_json_bodies_45_minproperties_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/config")
            .handler_name("json_bodies_45_minproperties_validation_success_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"minProperties\":2,\"type\":\"object\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        json_bodies_45_minproperties_validation_success_handler,
    )?;
    Ok(app)
}

/// App for fixture: 46_minproperties_validation_failure
pub fn create_app_json_bodies_46_minproperties_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/config")
            .handler_name("json_bodies_46_minproperties_validation_failure_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"minProperties\":2,\"type\":\"object\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        json_bodies_46_minproperties_validation_failure_handler,
    )?;
    Ok(app)
}

/// App for fixture: 47_maxproperties_validation_failure
pub fn create_app_json_bodies_47_maxproperties_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/config")
            .handler_name("json_bodies_47_maxproperties_validation_failure_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"maxProperties\":3,\"type\":\"object\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        json_bodies_47_maxproperties_validation_failure_handler,
    )?;
    Ok(app)
}

/// App for fixture: 48_dependencies_validation_success
pub fn create_app_json_bodies_48_dependencies_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/billing").handler_name("json_bodies_48_dependencies_validation_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_48_dependencies_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: 49_dependencies_validation_failure
pub fn create_app_json_bodies_49_dependencies_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/billing").handler_name("json_bodies_49_dependencies_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_49_dependencies_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 50_deep_nesting_4_levels
pub fn create_app_json_bodies_50_deep_nesting_4_levels() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/data").handler_name("json_bodies_50_deep_nesting_4_levels_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"user\":{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"address\":{\"properties\":{\"street\":{\"type\":\"string\"}},\"required\":[\"street\"],\"type\":\"object\"}},\"required\":[\"address\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_50_deep_nesting_4_levels_handler)?;
    Ok(app)
}

/// App for fixture: Array of objects - success
pub fn create_app_json_bodies_array_of_objects_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/list").handler_name("json_bodies_array_of_objects_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"images\":{\"items\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"type\":\"array\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"images\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_array_of_objects_success_handler)?;
    Ok(app)
}

/// App for fixture: Array of primitive values
pub fn create_app_json_bodies_array_of_primitive_values() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_array_of_primitive_values_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"ratings\":{\"items\":{\"type\":\"number\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"ratings\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_array_of_primitive_values_handler)?;
    Ok(app)
}

/// App for fixture: Body with query parameters
pub fn create_app_json_bodies_body_with_query_parameters() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_body_with_query_parameters_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap()), json_bodies_body_with_query_parameters_handler)?;
    Ok(app)
}

/// App for fixture: Boolean field - success
pub fn create_app_json_bodies_boolean_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_boolean_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"in_stock\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_boolean_field_success_handler)?;
    Ok(app)
}

/// App for fixture: Date field - success
pub fn create_app_json_bodies_date_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/events/").handler_name("json_bodies_date_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"event_date\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"event_date\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_date_field_success_handler)?;
    Ok(app)
}

/// App for fixture: Datetime field - success
pub fn create_app_json_bodies_datetime_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/events/").handler_name("json_bodies_datetime_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"created_at\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_datetime_field_success_handler)?;
    Ok(app)
}

/// App for fixture: Deeply nested objects
pub fn create_app_json_bodies_deeply_nested_objects() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/nested").handler_name("json_bodies_deeply_nested_objects_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"type\":\"string\"},\"country\":{\"additionalProperties\":false,\"properties\":{\"code\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"code\"],\"type\":\"object\"},\"street\":{\"type\":\"string\"}},\"required\":[\"street\",\"city\",\"country\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_deeply_nested_objects_handler)?;
    Ok(app)
}

/// App for fixture: Empty JSON object
pub fn create_app_json_bodies_empty_json_object() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/items/optional-all")
            .handler_name("json_bodies_empty_json_object_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        json_bodies_empty_json_object_handler,
    )?;
    Ok(app)
}

/// App for fixture: Empty array validation - fail
pub fn create_app_json_bodies_empty_array_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/list-validated").handler_name("json_bodies_empty_array_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{},\"minItems\":1,\"type\":\"array\"}},\"required\":[\"name\",\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_empty_array_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: Enum field - invalid value
pub fn create_app_json_bodies_enum_field_invalid_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_enum_field_invalid_value_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"category\":{\"enum\":[\"electronics\",\"clothing\",\"books\"],\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_enum_field_invalid_value_handler)?;
    Ok(app)
}

/// App for fixture: Enum field - success
pub fn create_app_json_bodies_enum_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_enum_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"category\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_enum_field_success_handler)?;
    Ok(app)
}

/// App for fixture: Extra fields ignored (no additionalProperties)
pub fn create_app_json_bodies_extra_fields_ignored_no_additionalproperties() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_extra_fields_ignored_no_additionalproperties_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"another_extra\":{\"type\":\"integer\"},\"extra_field\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"extra_field\",\"another_extra\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_extra_fields_ignored_no_additionalproperties_handler)?;
    Ok(app)
}

/// App for fixture: Field type validation - invalid type
pub fn create_app_json_bodies_field_type_validation_invalid_type() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_field_type_validation_invalid_type_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_field_type_validation_invalid_type_handler)?;
    Ok(app)
}

/// App for fixture: Nested object - success
pub fn create_app_json_bodies_nested_object_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/nested").handler_name("json_bodies_nested_object_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"image\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"image\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_nested_object_success_handler)?;
    Ok(app)
}

/// App for fixture: Null value for optional field
pub fn create_app_json_bodies_null_value_for_optional_field() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_null_value_for_optional_field_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"null\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"null\"}},\"required\":[\"name\",\"price\",\"description\",\"tax\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_null_value_for_optional_field_handler)?;
    Ok(app)
}

/// App for fixture: Numeric ge validation - fail
pub fn create_app_json_bodies_numeric_ge_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_numeric_ge_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"minimum\":1,\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_numeric_ge_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: Numeric le validation - success
pub fn create_app_json_bodies_numeric_le_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_numeric_le_validation_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_numeric_le_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: Optional fields - omitted
pub fn create_app_json_bodies_optional_fields_omitted() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_optional_fields_omitted_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_optional_fields_omitted_handler)?;
    Ok(app)
}

/// App for fixture: PATCH partial update
pub fn create_app_json_bodies_patch_partial_update() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(patch("/items/{id}").handler_name("json_bodies_patch_partial_update_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"price\":{\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), json_bodies_patch_partial_update_handler)?;
    Ok(app)
}

/// App for fixture: Required field missing - validation error
pub fn create_app_json_bodies_required_field_missing_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_required_field_missing_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"description\",\"price\",\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_required_field_missing_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: Simple JSON object - success
pub fn create_app_json_bodies_simple_json_object_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_simple_json_object_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_simple_json_object_success_handler)?;
    Ok(app)
}

/// App for fixture: String max_length validation - fail
pub fn create_app_json_bodies_string_max_length_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_string_max_length_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"maxLength\":50,\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_string_max_length_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: String min_length validation - fail
pub fn create_app_json_bodies_string_min_length_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_string_min_length_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"minLength\":3,\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_string_min_length_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: String pattern validation - fail
pub fn create_app_json_bodies_string_pattern_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_string_pattern_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"pattern\":\"^[A-Z]{3}[0-9]{4}$\",\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_string_pattern_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: String pattern validation - success
pub fn create_app_json_bodies_string_pattern_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/validated").handler_name("json_bodies_string_pattern_validation_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_string_pattern_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: UUID field - invalid format
pub fn create_app_json_bodies_uuid_field_invalid_format() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_uuid_field_invalid_format_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_uuid_field_invalid_format_handler)?;
    Ok(app)
}

/// App for fixture: UUID field - success
pub fn create_app_json_bodies_uuid_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("json_bodies_uuid_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), json_bodies_uuid_field_success_handler)?;
    Ok(app)
}

/// App for fixture: Hook Execution Order
pub fn create_app_lifecycle_hooks_hook_execution_order() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_request(spikard::request_hook(
                "first_hook",
                lifecycle_hooks_hook_execution_order_first_hook_on_request_0,
            ))
            .on_request(spikard::request_hook(
                "second_hook",
                lifecycle_hooks_hook_execution_order_second_hook_on_request_1,
            ))
            .on_request(spikard::request_hook(
                "third_hook",
                lifecycle_hooks_hook_execution_order_third_hook_on_request_2,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/test-hook-order")
            .handler_name("lifecycle_hooks_hook_execution_order_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_hook_execution_order_handler,
    )?;
    Ok(app)
}

/// App for fixture: Multiple Hooks - All Phases
pub fn create_app_lifecycle_hooks_multiple_hooks_all_phases() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_request(spikard::request_hook(
                "request_logger",
                lifecycle_hooks_multiple_hooks_all_phases_request_logger_on_request_0,
            ))
            .on_request(spikard::request_hook(
                "request_id_generator",
                lifecycle_hooks_multiple_hooks_all_phases_request_id_generator_on_request_1,
            ))
            .pre_validation(spikard::request_hook(
                "rate_limiter",
                lifecycle_hooks_multiple_hooks_all_phases_rate_limiter_pre_validation_0,
            ))
            .pre_handler(spikard::request_hook(
                "authenticator",
                lifecycle_hooks_multiple_hooks_all_phases_authenticator_pre_handler_0,
            ))
            .pre_handler(spikard::request_hook(
                "authorizer",
                lifecycle_hooks_multiple_hooks_all_phases_authorizer_pre_handler_1,
            ))
            .on_response(spikard::response_hook(
                "security_headers",
                lifecycle_hooks_multiple_hooks_all_phases_security_headers_on_response_0,
            ))
            .on_response(spikard::response_hook(
                "response_timer",
                lifecycle_hooks_multiple_hooks_all_phases_response_timer_on_response_1,
            ))
            .on_response(spikard::response_hook(
                "audit_logger",
                lifecycle_hooks_multiple_hooks_all_phases_audit_logger_on_response_2,
            ))
            .on_error(spikard::response_hook(
                "error_logger",
                lifecycle_hooks_multiple_hooks_all_phases_error_logger_on_error_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(post("/api/full-lifecycle").handler_name("lifecycle_hooks_multiple_hooks_all_phases_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"action\":{\"type\":\"string\"},\"user_id\":{\"type\":\"string\"}},\"required\":[\"user_id\",\"action\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), lifecycle_hooks_multiple_hooks_all_phases_handler)?;
    Ok(app)
}

/// App for fixture: onError - Error Logging
pub fn create_app_lifecycle_hooks_onerror_error_logging() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_error(spikard::response_hook(
                "error_logger",
                lifecycle_hooks_onerror_error_logging_error_logger_on_error_0,
            ))
            .on_error(spikard::response_hook(
                "error_formatter",
                lifecycle_hooks_onerror_error_logging_error_formatter_on_error_1,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/test-error")
            .handler_name("lifecycle_hooks_onerror_error_logging_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_onerror_error_logging_handler,
    )?;
    Ok(app)
}

/// App for fixture: onRequest - Request Logging
pub fn create_app_lifecycle_hooks_onrequest_request_logging() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_request(spikard::request_hook(
                "request_logger",
                lifecycle_hooks_onrequest_request_logging_request_logger_on_request_0,
            ))
            .on_request(spikard::request_hook(
                "request_id_generator",
                lifecycle_hooks_onrequest_request_logging_request_id_generator_on_request_1,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/test-on-request")
            .handler_name("lifecycle_hooks_onrequest_request_logging_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_onrequest_request_logging_handler,
    )?;
    Ok(app)
}

/// App for fixture: onResponse - Response Timing
pub fn create_app_lifecycle_hooks_onresponse_response_timing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_request(spikard::request_hook(
                "start_timer",
                lifecycle_hooks_onresponse_response_timing_start_timer_on_request_0,
            ))
            .on_response(spikard::response_hook(
                "response_timer",
                lifecycle_hooks_onresponse_response_timing_response_timer_on_response_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/test-timing")
            .handler_name("lifecycle_hooks_onresponse_response_timing_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_onresponse_response_timing_handler,
    )?;
    Ok(app)
}

/// App for fixture: onResponse - Security Headers
pub fn create_app_lifecycle_hooks_onresponse_security_headers() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .on_response(spikard::response_hook(
                "security_headers",
                lifecycle_hooks_onresponse_security_headers_security_headers_on_response_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/test-security-headers")
            .handler_name("lifecycle_hooks_onresponse_security_headers_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_onresponse_security_headers_handler,
    )?;
    Ok(app)
}

/// App for fixture: preHandler - Authentication Failed (Short Circuit)
pub fn create_app_lifecycle_hooks_prehandler_authentication_failed_short_circuit() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_handler(spikard::request_hook(
                "authenticator",
                lifecycle_hooks_prehandler_authentication_failed_short_circuit_authenticator_pre_handler_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/protected-resource-fail")
            .handler_name("lifecycle_hooks_prehandler_authentication_failed_short_circuit_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prehandler_authentication_failed_short_circuit_handler,
    )?;
    Ok(app)
}

/// App for fixture: preHandler - Authentication Success
pub fn create_app_lifecycle_hooks_prehandler_authentication_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_handler(spikard::request_hook(
                "authenticator",
                lifecycle_hooks_prehandler_authentication_success_authenticator_pre_handler_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/protected-resource")
            .handler_name("lifecycle_hooks_prehandler_authentication_success_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prehandler_authentication_success_handler,
    )?;
    Ok(app)
}

/// App for fixture: preHandler - Authorization Check
pub fn create_app_lifecycle_hooks_prehandler_authorization_check() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_handler(spikard::request_hook(
                "authenticator",
                lifecycle_hooks_prehandler_authorization_check_authenticator_pre_handler_0,
            ))
            .pre_handler(spikard::request_hook(
                "authorizer",
                lifecycle_hooks_prehandler_authorization_check_authorizer_pre_handler_1,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/admin-only")
            .handler_name("lifecycle_hooks_prehandler_authorization_check_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prehandler_authorization_check_handler,
    )?;
    Ok(app)
}

/// App for fixture: preHandler - Authorization Forbidden (Short Circuit)
pub fn create_app_lifecycle_hooks_prehandler_authorization_forbidden_short_circuit() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_handler(spikard::request_hook(
                "authenticator",
                lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authenticator_pre_handler_0,
            ))
            .pre_handler(spikard::request_hook(
                "authorizer",
                lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authorizer_pre_handler_1,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        get("/api/admin-only-forbidden")
            .handler_name("lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_handler,
    )?;
    Ok(app)
}

/// App for fixture: preValidation - Rate Limit Exceeded (Short Circuit)
pub fn create_app_lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_validation(spikard::request_hook(
                "rate_limiter",
                lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_rate_limiter_pre_validation_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        post("/api/test-rate-limit-exceeded")
            .handler_name("lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_handler,
    )?;
    Ok(app)
}

/// App for fixture: preValidation - Rate Limiting
pub fn create_app_lifecycle_hooks_prevalidation_rate_limiting() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.lifecycle_hooks = Some(
        spikard::LifecycleHooks::builder()
            .pre_validation(spikard::request_hook(
                "rate_limiter",
                lifecycle_hooks_prevalidation_rate_limiting_rate_limiter_pre_validation_0,
            ))
            .build(),
    );
    let mut app = App::new().config(config);
    app.route(
        post("/api/test-rate-limit")
            .handler_name("lifecycle_hooks_prevalidation_rate_limiting_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        lifecycle_hooks_prevalidation_rate_limiting_handler,
    )?;
    Ok(app)
}

/// App for fixture: 17_file_magic_number_png_success
pub fn create_app_multipart_17_file_magic_number_png_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("multipart_17_file_magic_number_png_success_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(
                serde_json::from_str::<Value>(
                    "{\"image\":{\"content_type\":[\"image/png\"],\"required\":true,\"validate_magic_numbers\":true}}",
                )
                .unwrap(),
            ),
        multipart_17_file_magic_number_png_success_handler,
    )?;
    Ok(app)
}

/// App for fixture: 18_file_magic_number_jpeg_success
pub fn create_app_multipart_18_file_magic_number_jpeg_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("multipart_18_file_magic_number_jpeg_success_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(
                serde_json::from_str::<Value>(
                    "{\"image\":{\"content_type\":[\"image/jpeg\"],\"required\":true,\"validate_magic_numbers\":true}}",
                )
                .unwrap(),
            ),
        multipart_18_file_magic_number_jpeg_success_handler,
    )?;
    Ok(app)
}

/// App for fixture: 19_file_mime_spoofing_png_as_jpeg
pub fn create_app_multipart_19_file_mime_spoofing_png_as_jpeg() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("multipart_19_file_mime_spoofing_png_as_jpeg_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(
                serde_json::from_str::<Value>(
                    "{\"image\":{\"content_type\":[\"image/jpeg\"],\"required\":true,\"validate_magic_numbers\":true}}",
                )
                .unwrap(),
            ),
        multipart_19_file_mime_spoofing_png_as_jpeg_handler,
    )?;
    Ok(app)
}

/// App for fixture: 20_file_mime_spoofing_jpeg_as_png
pub fn create_app_multipart_20_file_mime_spoofing_jpeg_as_png() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("multipart_20_file_mime_spoofing_jpeg_as_png_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(
                serde_json::from_str::<Value>(
                    "{\"image\":{\"content_type\":[\"image/png\"],\"required\":true,\"validate_magic_numbers\":true}}",
                )
                .unwrap(),
            ),
        multipart_20_file_mime_spoofing_jpeg_as_png_handler,
    )?;
    Ok(app)
}

/// App for fixture: 21_file_pdf_magic_number_success
pub fn create_app_multipart_21_file_pdf_magic_number_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/upload").handler_name("multipart_21_file_pdf_magic_number_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()).file_params_json(serde_json::from_str::<Value>("{\"document\":{\"content_type\":[\"application/pdf\"],\"required\":true,\"validate_magic_numbers\":true}}").unwrap()), multipart_21_file_pdf_magic_number_success_handler)?;
    Ok(app)
}

/// App for fixture: 22_file_empty_buffer
pub fn create_app_multipart_22_file_empty_buffer() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("multipart_22_file_empty_buffer_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            )
            .file_params_json(
                serde_json::from_str::<Value>("{\"file\":{\"required\":true,\"validate_magic_numbers\":true}}")
                    .unwrap(),
            ),
        multipart_22_file_empty_buffer_handler,
    )?;
    Ok(app)
}

/// App for fixture: Content-Type validation - invalid type
pub fn create_app_multipart_content_type_validation_invalid_type() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/images-only").handler_name("multipart_content_type_validation_invalid_type_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()).file_params_json(serde_json::from_str::<Value>("{\"file\":{\"content_type\":[\"image/jpeg\",\"image/png\",\"image/gif\"],\"required\":true}}").unwrap()), multipart_content_type_validation_invalid_type_handler)?;
    Ok(app)
}

/// App for fixture: Empty file upload
pub fn create_app_multipart_empty_file_upload() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/upload").handler_name("multipart_empty_file_upload_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_empty_file_upload_handler)?;
    Ok(app)
}

/// App for fixture: File list upload (array of files)
pub fn create_app_multipart_file_list_upload_array_of_files() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/list").handler_name("multipart_file_list_upload_array_of_files_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_file_list_upload_array_of_files_handler)?;
    Ok(app)
}

/// App for fixture: File size validation - too large
pub fn create_app_multipart_file_size_validation_too_large() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/validated").handler_name("multipart_file_size_validation_too_large_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_file_size_validation_too_large_handler)?;
    Ok(app)
}

/// App for fixture: File upload with custom headers
pub fn create_app_multipart_file_upload_with_custom_headers() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_file_upload_with_custom_headers_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test2\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_file_upload_with_custom_headers_handler)?;
    Ok(app)
}

/// App for fixture: File upload without filename
pub fn create_app_multipart_file_upload_without_filename() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_file_upload_without_filename_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_file_upload_without_filename_handler)?;
    Ok(app)
}

/// App for fixture: Form data without files
pub fn create_app_multipart_form_data_without_files() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_form_data_without_files_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"some\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_form_data_without_files_handler)?;
    Ok(app)
}

/// App for fixture: Image file upload
pub fn create_app_multipart_image_file_upload() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/image").handler_name("multipart_image_file_upload_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"image\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_image_file_upload_handler)?;
    Ok(app)
}

/// App for fixture: Mixed files and form data
pub fn create_app_multipart_mixed_files_and_form_data() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_mixed_files_and_form_data_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"active\":{\"type\":\"string\"},\"age\":{\"type\":\"string\"},\"file\":{\"format\":\"binary\",\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_mixed_files_and_form_data_handler)?;
    Ok(app)
}

/// App for fixture: Multiple file uploads
pub fn create_app_multipart_multiple_file_uploads() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_multiple_file_uploads_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"},\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\",\"test2\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_multiple_file_uploads_handler)?;
    Ok(app)
}

/// App for fixture: Multiple values for same field name
pub fn create_app_multipart_multiple_values_for_same_field_name() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_multiple_values_for_same_field_name_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_multiple_values_for_same_field_name_handler)?;
    Ok(app)
}

/// App for fixture: Optional file upload - missing
pub fn create_app_multipart_optional_file_upload_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/files/optional")
            .handler_name("multipart_optional_file_upload_missing_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        multipart_optional_file_upload_missing_handler,
    )?;
    Ok(app)
}

/// App for fixture: Optional file upload - provided
pub fn create_app_multipart_optional_file_upload_provided() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/optional").handler_name("multipart_optional_file_upload_provided_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_optional_file_upload_provided_handler)?;
    Ok(app)
}

/// App for fixture: PDF file upload
pub fn create_app_multipart_pdf_file_upload() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/document").handler_name("multipart_pdf_file_upload_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"document\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"document\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_pdf_file_upload_handler)?;
    Ok(app)
}

/// App for fixture: Required file upload - missing
pub fn create_app_multipart_required_file_upload_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/files/required").handler_name("multipart_required_file_upload_missing_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_required_file_upload_missing_handler)?;
    Ok(app)
}

/// App for fixture: Simple file upload
pub fn create_app_multipart_simple_file_upload() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/").handler_name("multipart_simple_file_upload_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"test\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), multipart_simple_file_upload_handler)?;
    Ok(app)
}

/// App for fixture: 20_uuid_v3_path_param_success
pub fn create_app_path_params_20_uuid_v3_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/{id}").handler_name("path_params_20_uuid_v3_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\",\"uuidVersion\":\"3\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), path_params_20_uuid_v3_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 21_uuid_v5_path_param_success
pub fn create_app_path_params_21_uuid_v5_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/{id}").handler_name("path_params_21_uuid_v5_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\",\"uuidVersion\":\"5\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap()), path_params_21_uuid_v5_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 24_date_format_path_param_success
pub fn create_app_path_params_24_date_format_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/events/{date}").handler_name("path_params_24_date_format_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"date\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date\"],\"type\":\"object\"}").unwrap()), path_params_24_date_format_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 25_date_format_invalid_failure
pub fn create_app_path_params_25_date_format_invalid_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/events/{date}").handler_name("path_params_25_date_format_invalid_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"date\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date\"],\"type\":\"object\"}").unwrap()), path_params_25_date_format_invalid_failure_handler)?;
    Ok(app)
}

/// App for fixture: 27_datetime_format_path_param_success
pub fn create_app_path_params_27_datetime_format_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/bookings/{timestamp}").handler_name("path_params_27_datetime_format_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"timestamp\":{\"format\":\"date-time\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"timestamp\"],\"type\":\"object\"}").unwrap()), path_params_27_datetime_format_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 28_duration_format_path_param_success
pub fn create_app_path_params_28_duration_format_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/delays/{duration}").handler_name("path_params_28_duration_format_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"duration\":{\"format\":\"duration\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"duration\"],\"type\":\"object\"}").unwrap()), path_params_28_duration_format_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 29_decimal_path_param_success
pub fn create_app_path_params_29_decimal_path_param_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/prices/{amount}").handler_name("path_params_29_decimal_path_param_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"amount\":{\"format\":\"decimal\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"amount\"],\"type\":\"object\"}").unwrap()), path_params_29_decimal_path_param_success_handler)?;
    Ok(app)
}

/// App for fixture: 30_string_minlength_path_success
pub fn create_app_path_params_30_string_minlength_path_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/{username}").handler_name("path_params_30_string_minlength_path_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()), path_params_30_string_minlength_path_success_handler)?;
    Ok(app)
}

/// App for fixture: 31_string_minlength_path_failure
pub fn create_app_path_params_31_string_minlength_path_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/{username}").handler_name("path_params_31_string_minlength_path_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()), path_params_31_string_minlength_path_failure_handler)?;
    Ok(app)
}

/// App for fixture: 32_string_maxlength_path_failure
pub fn create_app_path_params_32_string_maxlength_path_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/users/{username}").handler_name("path_params_32_string_maxlength_path_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"maxLength\":20,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()), path_params_32_string_maxlength_path_failure_handler)?;
    Ok(app)
}

/// App for fixture: 33_string_pattern_path_success
pub fn create_app_path_params_33_string_pattern_path_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/repos/{owner}/{repo}").handler_name("path_params_33_string_pattern_path_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"owner\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"source\":\"path\",\"type\":\"string\"},\"repo\":{\"pattern\":\"^[a-zA-Z0-9-_]+$\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"owner\",\"repo\"],\"type\":\"object\"}").unwrap()), path_params_33_string_pattern_path_success_handler)?;
    Ok(app)
}

/// App for fixture: 34_string_pattern_path_failure
pub fn create_app_path_params_34_string_pattern_path_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/repos/{owner}").handler_name("path_params_34_string_pattern_path_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"owner\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"owner\"],\"type\":\"object\"}").unwrap()), path_params_34_string_pattern_path_failure_handler)?;
    Ok(app)
}

/// App for fixture: 35_negative_integer_path_param
pub fn create_app_path_params_35_negative_integer_path_param() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/offset/{value}").handler_name("path_params_35_negative_integer_path_param_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), path_params_35_negative_integer_path_param_handler)?;
    Ok(app)
}

/// App for fixture: Boolean path parameter - True
pub fn create_app_path_params_boolean_path_parameter_true() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/bool/{item_id}").handler_name("path_params_boolean_path_parameter_true_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"boolean\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_boolean_path_parameter_true_handler)?;
    Ok(app)
}

/// App for fixture: Boolean path parameter - numeric 1
pub fn create_app_path_params_boolean_path_parameter_numeric_1() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/bool/{item_id}").handler_name("path_params_boolean_path_parameter_numeric_1_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"boolean\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_boolean_path_parameter_numeric_1_handler)?;
    Ok(app)
}

/// App for fixture: Date path parameter - success
pub fn create_app_path_params_date_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/date/{date_param}").handler_name("path_params_date_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"date_param\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date_param\"],\"type\":\"object\"}").unwrap()), path_params_date_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Enum path parameter - invalid value
pub fn create_app_path_params_enum_path_parameter_invalid_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/models/{model_name}").handler_name("path_params_enum_path_parameter_invalid_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap()), path_params_enum_path_parameter_invalid_value_handler)?;
    Ok(app)
}

/// App for fixture: Enum path parameter - success
pub fn create_app_path_params_enum_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/models/{model_name}").handler_name("path_params_enum_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"lenet\",\"resnet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap()), path_params_enum_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Float path parameter - success
pub fn create_app_path_params_float_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/float/{item_id}").handler_name("path_params_float_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"number\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_float_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter - invalid string
pub fn create_app_path_params_integer_path_parameter_invalid_string() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/int/{item_id}").handler_name("path_params_integer_path_parameter_invalid_string_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_invalid_string_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter - success
pub fn create_app_path_params_integer_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/int/{item_id}").handler_name("path_params_integer_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with combined lt and gt constraints - success
pub fn create_app_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success()
-> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-lt-gt/{item_id}").handler_name("path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"exclusiveMaximum\":3,\"exclusiveMinimum\":1,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with ge constraint - success
pub fn create_app_path_params_integer_path_parameter_with_ge_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-ge/{item_id}").handler_name("path_params_integer_path_parameter_with_ge_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"minimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_ge_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with gt constraint - failure
pub fn create_app_path_params_integer_path_parameter_with_gt_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-gt/{item_id}").handler_name("path_params_integer_path_parameter_with_gt_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"exclusiveMinimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_gt_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with gt constraint - success
pub fn create_app_path_params_integer_path_parameter_with_gt_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-gt/{item_id}").handler_name("path_params_integer_path_parameter_with_gt_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"exclusiveMinimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_gt_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with le constraint - success
pub fn create_app_path_params_integer_path_parameter_with_le_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-le/{item_id}").handler_name("path_params_integer_path_parameter_with_le_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"maximum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_le_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer path parameter with lt constraint - success
pub fn create_app_path_params_integer_path_parameter_with_lt_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-lt/{item_id}").handler_name("path_params_integer_path_parameter_with_lt_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"exclusiveMaximum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_integer_path_parameter_with_lt_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Multiple path parameters - success
pub fn create_app_path_params_multiple_path_parameters_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/{version}/{service_id}/{user_id}/{order_id}").handler_name("path_params_multiple_path_parameters_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"order_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"},\"service_id\":{\"source\":\"path\",\"type\":\"integer\"},\"user_id\":{\"source\":\"path\",\"type\":\"string\"},\"version\":{\"source\":\"path\",\"type\":\"number\"}},\"required\":[\"order_id\",\"service_id\",\"user_id\",\"version\"],\"type\":\"object\"}").unwrap()), path_params_multiple_path_parameters_success_handler)?;
    Ok(app)
}

/// App for fixture: Path parameter type syntax - invalid UUID
pub fn create_app_path_params_path_parameter_type_syntax_invalid_uuid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/type-syntax/items/{id:uuid}")
            .handler_name("path_params_path_parameter_type_syntax_invalid_uuid_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        path_params_path_parameter_type_syntax_invalid_uuid_handler,
    )?;
    Ok(app)
}

/// App for fixture: Path parameter type syntax with override
pub fn create_app_path_params_path_parameter_type_syntax_with_override() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/type-syntax/items-count/{count:int}").handler_name("path_params_path_parameter_type_syntax_with_override_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"count\":{\"maximum\":100,\"minimum\":1,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"count\"],\"type\":\"object\"}").unwrap()), path_params_path_parameter_type_syntax_with_override_handler)?;
    Ok(app)
}

/// App for fixture: Path parameter with type syntax - UUID
pub fn create_app_path_params_path_parameter_with_type_syntax_uuid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/type-syntax/items/{id:uuid}")
            .handler_name("path_params_path_parameter_with_type_syntax_uuid_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        path_params_path_parameter_with_type_syntax_uuid_handler,
    )?;
    Ok(app)
}

/// App for fixture: Path parameter with type syntax - integer
pub fn create_app_path_params_path_parameter_with_type_syntax_integer() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/type-syntax/users/{user_id:int}")
            .handler_name("path_params_path_parameter_with_type_syntax_integer_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        path_params_path_parameter_with_type_syntax_integer_handler,
    )?;
    Ok(app)
}

/// App for fixture: Path type parameter - file path
pub fn create_app_path_params_path_type_parameter_file_path() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/files/{file_path:path}").handler_name("path_params_path_type_parameter_file_path_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"file_path\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"file_path\"],\"type\":\"object\"}").unwrap()), path_params_path_type_parameter_file_path_handler)?;
    Ok(app)
}

/// App for fixture: String path parameter - success
pub fn create_app_path_params_string_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/str/{item_id}").handler_name("path_params_string_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_string_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: String path parameter with max_length - failure
pub fn create_app_path_params_string_path_parameter_with_max_length_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-maxlength/{item_id}").handler_name("path_params_string_path_parameter_with_max_length_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"maxLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_string_path_parameter_with_max_length_failure_handler)?;
    Ok(app)
}

/// App for fixture: String path parameter with min_length - failure
pub fn create_app_path_params_string_path_parameter_with_min_length_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/path/param-minlength/{item_id}").handler_name("path_params_string_path_parameter_with_min_length_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_string_path_parameter_with_min_length_failure_handler)?;
    Ok(app)
}

/// App for fixture: UUID path parameter - success
pub fn create_app_path_params_uuid_path_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/{item_id}").handler_name("path_params_uuid_path_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), path_params_uuid_path_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: 42_negative_integer_query_param
pub fn create_app_query_params_42_negative_integer_query_param() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/negative").handler_name("query_params_42_negative_integer_query_param_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"offset\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"offset\"],\"type\":\"object\"}").unwrap()), query_params_42_negative_integer_query_param_handler)?;
    Ok(app)
}

/// App for fixture: 43_scientific_notation_float
pub fn create_app_query_params_43_scientific_notation_float() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/stats").handler_name("query_params_43_scientific_notation_float_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"threshold\":{\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"threshold\"],\"type\":\"object\"}").unwrap()), query_params_43_scientific_notation_float_handler)?;
    Ok(app)
}

/// App for fixture: 44_string_minlength_validation_success
pub fn create_app_query_params_44_string_minlength_validation_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("query_params_44_string_minlength_validation_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"term\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap()), query_params_44_string_minlength_validation_success_handler)?;
    Ok(app)
}

/// App for fixture: 45_string_minlength_validation_failure
pub fn create_app_query_params_45_string_minlength_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("query_params_45_string_minlength_validation_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"term\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap()), query_params_45_string_minlength_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 46_string_maxlength_validation_failure
pub fn create_app_query_params_46_string_maxlength_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("query_params_46_string_maxlength_validation_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"term\":{\"maxLength\":10,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap()), query_params_46_string_maxlength_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 47_pattern_validation_email_success
pub fn create_app_query_params_47_pattern_validation_email_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/subscribe").handler_name("query_params_47_pattern_validation_email_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap()), query_params_47_pattern_validation_email_success_handler)?;
    Ok(app)
}

/// App for fixture: 48_pattern_validation_email_failure
pub fn create_app_query_params_48_pattern_validation_email_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/subscribe").handler_name("query_params_48_pattern_validation_email_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap()), query_params_48_pattern_validation_email_failure_handler)?;
    Ok(app)
}

/// App for fixture: 49_integer_gt_constraint_success
pub fn create_app_query_params_49_integer_gt_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_49_integer_gt_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap()), query_params_49_integer_gt_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: 50_integer_gt_constraint_failure
pub fn create_app_query_params_50_integer_gt_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_50_integer_gt_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap()), query_params_50_integer_gt_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: 51_integer_ge_constraint_boundary
pub fn create_app_query_params_51_integer_ge_constraint_boundary() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_51_integer_ge_constraint_boundary_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"offset\":{\"minimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"offset\"],\"type\":\"object\"}").unwrap()), query_params_51_integer_ge_constraint_boundary_handler)?;
    Ok(app)
}

/// App for fixture: 52_integer_le_constraint_boundary
pub fn create_app_query_params_52_integer_le_constraint_boundary() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_52_integer_le_constraint_boundary_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap()), query_params_52_integer_le_constraint_boundary_handler)?;
    Ok(app)
}

/// App for fixture: 53_integer_le_constraint_failure
pub fn create_app_query_params_53_integer_le_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_53_integer_le_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap()), query_params_53_integer_le_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: 54_array_minitems_constraint_success
pub fn create_app_query_params_54_array_minitems_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_54_array_minitems_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"minItems\":2,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap()), query_params_54_array_minitems_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: 55_array_minitems_constraint_failure
pub fn create_app_query_params_55_array_minitems_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_55_array_minitems_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"minItems\":2,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap()), query_params_55_array_minitems_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: 56_array_maxitems_constraint_failure
pub fn create_app_query_params_56_array_maxitems_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_56_array_maxitems_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"maxItems\":5,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap()), query_params_56_array_maxitems_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: 57_boolean_empty_string_coercion
pub fn create_app_query_params_57_boolean_empty_string_coercion() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_57_boolean_empty_string_coercion_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"active\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"active\"],\"type\":\"object\"}").unwrap()), query_params_57_boolean_empty_string_coercion_handler)?;
    Ok(app)
}

/// App for fixture: 58_format_email_success
pub fn create_app_query_params_58_format_email_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/subscribe").handler_name("query_params_58_format_email_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"format\":\"email\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap()), query_params_58_format_email_success_handler)?;
    Ok(app)
}

/// App for fixture: 59_format_email_failure
pub fn create_app_query_params_59_format_email_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/subscribe").handler_name("query_params_59_format_email_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"format\":\"email\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap()), query_params_59_format_email_failure_handler)?;
    Ok(app)
}

/// App for fixture: 60_format_ipv4_success
pub fn create_app_query_params_60_format_ipv4_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/network").handler_name("query_params_60_format_ipv4_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ip\":{\"format\":\"ipv4\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap()), query_params_60_format_ipv4_success_handler)?;
    Ok(app)
}

/// App for fixture: 61_format_ipv4_failure
pub fn create_app_query_params_61_format_ipv4_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/network").handler_name("query_params_61_format_ipv4_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ip\":{\"format\":\"ipv4\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap()), query_params_61_format_ipv4_failure_handler)?;
    Ok(app)
}

/// App for fixture: 62_format_ipv6_success
pub fn create_app_query_params_62_format_ipv6_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/network/ipv6").handler_name("query_params_62_format_ipv6_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ip\":{\"format\":\"ipv6\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap()), query_params_62_format_ipv6_success_handler)?;
    Ok(app)
}

/// App for fixture: 63_format_uri_success
pub fn create_app_query_params_63_format_uri_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/redirect").handler_name("query_params_63_format_uri_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"url\":{\"format\":\"uri\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"url\"],\"type\":\"object\"}").unwrap()), query_params_63_format_uri_success_handler)?;
    Ok(app)
}

/// App for fixture: 64_format_uri_failure
pub fn create_app_query_params_64_format_uri_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/redirect").handler_name("query_params_64_format_uri_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"url\":{\"format\":\"uri\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"url\"],\"type\":\"object\"}").unwrap()), query_params_64_format_uri_failure_handler)?;
    Ok(app)
}

/// App for fixture: 65_format_hostname_success
pub fn create_app_query_params_65_format_hostname_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/dns").handler_name("query_params_65_format_hostname_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"host\":{\"format\":\"hostname\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"host\"],\"type\":\"object\"}").unwrap()), query_params_65_format_hostname_success_handler)?;
    Ok(app)
}

/// App for fixture: 66_multipleof_constraint_success
pub fn create_app_query_params_66_multipleof_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_66_multipleof_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"quantity\":{\"multipleOf\":5,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap()), query_params_66_multipleof_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: 67_multipleof_constraint_failure
pub fn create_app_query_params_67_multipleof_constraint_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_67_multipleof_constraint_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"quantity\":{\"multipleOf\":5,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap()), query_params_67_multipleof_constraint_failure_handler)?;
    Ok(app)
}

/// App for fixture: 68_array_uniqueitems_success
pub fn create_app_query_params_68_array_uniqueitems_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_68_array_uniqueitems_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\",\"uniqueItems\":true}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap()), query_params_68_array_uniqueitems_success_handler)?;
    Ok(app)
}

/// App for fixture: 69_array_uniqueitems_failure
pub fn create_app_query_params_69_array_uniqueitems_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_69_array_uniqueitems_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\",\"uniqueItems\":true}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap()), query_params_69_array_uniqueitems_failure_handler)?;
    Ok(app)
}

/// App for fixture: 70_array_separator_pipe
pub fn create_app_query_params_70_array_separator_pipe() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_70_array_separator_pipe_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"separator\":\"|\",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap()), query_params_70_array_separator_pipe_handler)?;
    Ok(app)
}

/// App for fixture: 71_array_separator_semicolon
pub fn create_app_query_params_71_array_separator_semicolon() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items").handler_name("query_params_71_array_separator_semicolon_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"colors\":{\"items\":{\"type\":\"string\"},\"separator\":\";\",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"colors\"],\"type\":\"object\"}").unwrap()), query_params_71_array_separator_semicolon_handler)?;
    Ok(app)
}

/// App for fixture: 72_array_separator_space
pub fn create_app_query_params_72_array_separator_space() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/search").handler_name("query_params_72_array_separator_space_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"keywords\":{\"items\":{\"type\":\"string\"},\"separator\":\" \",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"keywords\"],\"type\":\"object\"}").unwrap()), query_params_72_array_separator_space_handler)?;
    Ok(app)
}

/// App for fixture: Array query parameter - empty array
pub fn create_app_query_params_array_query_parameter_empty_array() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/list-default").handler_name("query_params_array_query_parameter_empty_array_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_array_query_parameter_empty_array_handler)?;
    Ok(app)
}

/// App for fixture: Array query parameter - single value
pub fn create_app_query_params_array_query_parameter_single_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/list-default").handler_name("query_params_array_query_parameter_single_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_array_query_parameter_single_value_handler)?;
    Ok(app)
}

/// App for fixture: Boolean query parameter - numeric 1
pub fn create_app_query_params_boolean_query_parameter_numeric_1() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/bool").handler_name("query_params_boolean_query_parameter_numeric_1_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"flag\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"flag\"],\"type\":\"object\"}").unwrap()), query_params_boolean_query_parameter_numeric_1_handler)?;
    Ok(app)
}

/// App for fixture: Boolean query parameter - true
pub fn create_app_query_params_boolean_query_parameter_true() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/bool").handler_name("query_params_boolean_query_parameter_true_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"flag\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"flag\"],\"type\":\"object\"}").unwrap()), query_params_boolean_query_parameter_true_handler)?;
    Ok(app)
}

/// App for fixture: Date query parameter - success
pub fn create_app_query_params_date_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/date").handler_name("query_params_date_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"event_date\":{\"format\":\"date\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"event_date\"],\"type\":\"object\"}").unwrap()), query_params_date_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Datetime query parameter - success
pub fn create_app_query_params_datetime_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/datetime").handler_name("query_params_datetime_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"timestamp\":{\"format\":\"date-time\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"timestamp\"],\"type\":\"object\"}").unwrap()), query_params_datetime_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Enum query parameter - invalid value
pub fn create_app_query_params_enum_query_parameter_invalid_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/enum").handler_name("query_params_enum_query_parameter_invalid_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"model\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"model\"],\"type\":\"object\"}").unwrap()), query_params_enum_query_parameter_invalid_value_handler)?;
    Ok(app)
}

/// App for fixture: Enum query parameter - success
pub fn create_app_query_params_enum_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/enum").handler_name("query_params_enum_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"model\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"model\"],\"type\":\"object\"}").unwrap()), query_params_enum_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Float query param with ge constraint - success
pub fn create_app_query_params_float_query_param_with_ge_constraint_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/float-ge").handler_name("query_params_float_query_param_with_ge_constraint_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"price\":{\"minimum\":0.01,\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap()), query_params_float_query_param_with_ge_constraint_success_handler)?;
    Ok(app)
}

/// App for fixture: Integer query param with ge constraint - boundary
pub fn create_app_query_params_integer_query_param_with_ge_constraint_boundary() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int-ge").handler_name("query_params_integer_query_param_with_ge_constraint_boundary_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"minimum\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), query_params_integer_query_param_with_ge_constraint_boundary_handler)?;
    Ok(app)
}

/// App for fixture: Integer query param with gt constraint - valid
pub fn create_app_query_params_integer_query_param_with_gt_constraint_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int-gt").handler_name("query_params_integer_query_param_with_gt_constraint_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), query_params_integer_query_param_with_gt_constraint_valid_handler)?;
    Ok(app)
}

/// App for fixture: Integer query param with le constraint - boundary
pub fn create_app_query_params_integer_query_param_with_le_constraint_boundary() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int-le").handler_name("query_params_integer_query_param_with_le_constraint_boundary_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), query_params_integer_query_param_with_le_constraint_boundary_handler)?;
    Ok(app)
}

/// App for fixture: Integer query param with lt constraint - valid
pub fn create_app_query_params_integer_query_param_with_lt_constraint_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int-lt").handler_name("query_params_integer_query_param_with_lt_constraint_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"value\":{\"exclusiveMaximum\":50,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap()), query_params_integer_query_param_with_lt_constraint_valid_handler)?;
    Ok(app)
}

/// App for fixture: Integer with default value - not provided
pub fn create_app_query_params_integer_with_default_value_not_provided() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int/default").handler_name("query_params_integer_with_default_value_not_provided_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_integer_with_default_value_not_provided_handler)?;
    Ok(app)
}

/// App for fixture: Integer with default value - override
pub fn create_app_query_params_integer_with_default_value_override() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int/default").handler_name("query_params_integer_with_default_value_override_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_integer_with_default_value_override_handler)?;
    Ok(app)
}

/// App for fixture: List of integers - multiple values
pub fn create_app_query_params_list_of_integers_multiple_values() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/list").handler_name("query_params_list_of_integers_multiple_values_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"device_ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"device_ids\"],\"type\":\"object\"}").unwrap()), query_params_list_of_integers_multiple_values_handler)?;
    Ok(app)
}

/// App for fixture: List of strings - multiple values
pub fn create_app_query_params_list_of_strings_multiple_values() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("query_params_list_of_strings_multiple_values_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_list_of_strings_multiple_values_handler)?;
    Ok(app)
}

/// App for fixture: List query parameter - required but missing
pub fn create_app_query_params_list_query_parameter_required_but_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/list").handler_name("query_params_list_query_parameter_required_but_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"device_ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"device_ids\"],\"type\":\"object\"}").unwrap()), query_params_list_query_parameter_required_but_missing_handler)?;
    Ok(app)
}

/// App for fixture: List with default empty array - no values provided
pub fn create_app_query_params_list_with_default_empty_array_no_values_provided() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/list-default").handler_name("query_params_list_with_default_empty_array_no_values_provided_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_list_with_default_empty_array_no_values_provided_handler)?;
    Ok(app)
}

/// App for fixture: Multiple query parameters with different types
pub fn create_app_query_params_multiple_query_parameters_with_different_types() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/multi-type").handler_name("query_params_multiple_query_parameters_with_different_types_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"active\":{\"source\":\"query\",\"type\":\"boolean\"},\"age\":{\"source\":\"query\",\"type\":\"integer\"},\"name\":{\"source\":\"query\",\"type\":\"string\"},\"score\":{\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"active\",\"age\",\"name\",\"score\"],\"type\":\"object\"}").unwrap()), query_params_multiple_query_parameters_with_different_types_handler)?;
    Ok(app)
}

/// App for fixture: Optional integer query parameter - missing
pub fn create_app_query_params_optional_integer_query_parameter_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int/optional").handler_name("query_params_optional_integer_query_parameter_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_optional_integer_query_parameter_missing_handler)?;
    Ok(app)
}

/// App for fixture: Optional query parameter with default value
pub fn create_app_query_params_optional_query_parameter_with_default_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/optional-default").handler_name("query_params_optional_query_parameter_with_default_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_optional_query_parameter_with_default_value_handler)?;
    Ok(app)
}

/// App for fixture: Optional string query parameter - missing
pub fn create_app_query_params_optional_string_query_parameter_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/optional").handler_name("query_params_optional_string_query_parameter_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_optional_string_query_parameter_missing_handler)?;
    Ok(app)
}

/// App for fixture: Optional string query parameter - provided
pub fn create_app_query_params_optional_string_query_parameter_provided() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/optional").handler_name("query_params_optional_string_query_parameter_provided_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), query_params_optional_string_query_parameter_provided_handler)?;
    Ok(app)
}

/// App for fixture: Query parameter with URL encoded space
pub fn create_app_query_params_query_parameter_with_url_encoded_space() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/basic").handler_name("query_params_query_parameter_with_url_encoded_space_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"name\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()), query_params_query_parameter_with_url_encoded_space_handler)?;
    Ok(app)
}

/// App for fixture: Query parameter with URL encoded special characters
pub fn create_app_query_params_query_parameter_with_url_encoded_special_characters() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/basic").handler_name("query_params_query_parameter_with_url_encoded_special_characters_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"name\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()), query_params_query_parameter_with_url_encoded_special_characters_handler)?;
    Ok(app)
}

/// App for fixture: Query parameter with special characters - URL encoding
pub fn create_app_query_params_query_parameter_with_special_characters_url_encoding() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/test").handler_name("query_params_query_parameter_with_special_characters_url_encoding_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"source\":\"query\",\"type\":\"string\"},\"special\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\",\"special\"],\"type\":\"object\"}").unwrap()), query_params_query_parameter_with_special_characters_url_encoding_handler)?;
    Ok(app)
}

/// App for fixture: Required integer query parameter - float value
pub fn create_app_query_params_required_integer_query_parameter_float_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int").handler_name("query_params_required_integer_query_parameter_float_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_integer_query_parameter_float_value_handler)?;
    Ok(app)
}

/// App for fixture: Required integer query parameter - invalid type
pub fn create_app_query_params_required_integer_query_parameter_invalid_type() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int").handler_name("query_params_required_integer_query_parameter_invalid_type_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_integer_query_parameter_invalid_type_handler)?;
    Ok(app)
}

/// App for fixture: Required integer query parameter - missing
pub fn create_app_query_params_required_integer_query_parameter_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int").handler_name("query_params_required_integer_query_parameter_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_integer_query_parameter_missing_handler)?;
    Ok(app)
}

/// App for fixture: Required integer query parameter - success
pub fn create_app_query_params_required_integer_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/int").handler_name("query_params_required_integer_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_integer_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Required string query parameter - missing
pub fn create_app_query_params_required_string_query_parameter_missing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query").handler_name("query_params_required_string_query_parameter_missing_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_string_query_parameter_missing_handler)?;
    Ok(app)
}

/// App for fixture: Required string query parameter - success
pub fn create_app_query_params_required_string_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query").handler_name("query_params_required_string_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap()), query_params_required_string_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: String query param with max_length constraint - fail
pub fn create_app_query_params_string_query_param_with_max_length_constraint_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/str-max-length").handler_name("query_params_string_query_param_with_max_length_constraint_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"name\":{\"maxLength\":10,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()), query_params_string_query_param_with_max_length_constraint_fail_handler)?;
    Ok(app)
}

/// App for fixture: String query param with min_length constraint - fail
pub fn create_app_query_params_string_query_param_with_min_length_constraint_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/str-min-length").handler_name("query_params_string_query_param_with_min_length_constraint_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"name\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()), query_params_string_query_param_with_min_length_constraint_fail_handler)?;
    Ok(app)
}

/// App for fixture: String query param with regex pattern - fail
pub fn create_app_query_params_string_query_param_with_regex_pattern_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/pattern").handler_name("query_params_string_query_param_with_regex_pattern_fail_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"code\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap()), query_params_string_query_param_with_regex_pattern_fail_handler)?;
    Ok(app)
}

/// App for fixture: String validation with regex - failure
pub fn create_app_query_params_string_validation_with_regex_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("query_params_string_validation_with_regex_failure_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_query\":{\"pattern\":\"^fixedquery$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_query\"],\"type\":\"object\"}").unwrap()), query_params_string_validation_with_regex_failure_handler)?;
    Ok(app)
}

/// App for fixture: String validation with regex - success
pub fn create_app_query_params_string_validation_with_regex_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("query_params_string_validation_with_regex_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_query\":{\"pattern\":\"^fixedquery$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_query\"],\"type\":\"object\"}").unwrap()), query_params_string_validation_with_regex_success_handler)?;
    Ok(app)
}

/// App for fixture: UUID query parameter - invalid format
pub fn create_app_query_params_uuid_query_parameter_invalid_format() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/uuid").handler_name("query_params_uuid_query_parameter_invalid_format_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), query_params_uuid_query_parameter_invalid_format_handler)?;
    Ok(app)
}

/// App for fixture: UUID query parameter - success
pub fn create_app_query_params_uuid_query_parameter_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/query/uuid").handler_name("query_params_uuid_query_parameter_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), query_params_uuid_query_parameter_success_handler)?;
    Ok(app)
}

/// App for fixture: Rate limit below threshold succeeds
pub fn create_app_rate_limit_rate_limit_below_threshold_succeeds() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.rate_limit = Some(RateLimitConfig {
        per_second: 5,
        burst: 5,
        ip_based: false,
    });
    let mut app = App::new().config(config);
    app.route(
        get("/rate-limit/basic")
            .handler_name("rate_limit_rate_limit_below_threshold_succeeds_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        rate_limit_rate_limit_below_threshold_succeeds_handler,
    )?;
    Ok(app)
}

/// App for fixture: Rate limit exceeded returns 429
pub fn create_app_rate_limit_rate_limit_exceeded_returns_429() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.rate_limit = Some(RateLimitConfig {
        per_second: 1,
        burst: 1,
        ip_based: false,
    });
    let mut app = App::new().config(config);
    app.route(
        get("/rate-limit/exceeded")
            .handler_name("rate_limit_rate_limit_exceeded_returns_429_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        rate_limit_rate_limit_exceeded_returns_429_handler,
    )?;
    Ok(app)
}

/// App for fixture: Request ID header is preserved
pub fn create_app_request_id_request_id_header_is_preserved() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/request-id/preserved")
            .handler_name("request_id_request_id_header_is_preserved_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        request_id_request_id_header_is_preserved_handler,
    )?;
    Ok(app)
}

/// App for fixture: Request ID is generated when not provided
pub fn create_app_request_id_request_id_is_generated_when_not_provided() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.enable_request_id = true;
    let mut app = App::new().config(config);
    app.route(
        get("/request-id/generated")
            .handler_name("request_id_request_id_is_generated_when_not_provided_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        request_id_request_id_is_generated_when_not_provided_handler,
    )?;
    Ok(app)
}

/// App for fixture: Request ID middleware can be disabled
pub fn create_app_request_id_request_id_middleware_can_be_disabled() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.enable_request_id = false;
    let mut app = App::new().config(config);
    app.route(
        get("/request-id/disabled")
            .handler_name("request_id_request_id_middleware_can_be_disabled_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        request_id_request_id_middleware_can_be_disabled_handler,
    )?;
    Ok(app)
}

/// App for fixture: Request completes before timeout
pub fn create_app_request_timeout_request_completes_before_timeout() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.request_timeout = Some(2);
    let mut app = App::new().config(config);
    app.route(
        get("/timeouts/fast")
            .handler_name("request_timeout_request_completes_before_timeout_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        request_timeout_request_completes_before_timeout_handler,
    )?;
    Ok(app)
}

/// App for fixture: Request exceeds timeout
pub fn create_app_request_timeout_request_exceeds_timeout() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.request_timeout = Some(1);
    let mut app = App::new().config(config);
    app.route(
        get("/timeouts/slow")
            .handler_name("request_timeout_request_exceeds_timeout_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        request_timeout_request_exceeds_timeout_handler,
    )?;
    Ok(app)
}

/// App for fixture: Static file server returns text file
pub fn create_app_static_files_static_file_server_returns_text_file() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.static_files.push(StaticFilesConfig {
        directory: "static_assets/static_files_static_file_server_returns_text_file/public_0".to_string(),
        route_prefix: "/public".to_string(),
        index_file: true,
        cache_control: Some("public, max-age=60".to_string()),
    });
    let mut app = App::new().config(config);
    app.route(
        get("/public/hello.txt")
            .handler_name("static_files_static_file_server_returns_text_file_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        static_files_static_file_server_returns_text_file_handler,
    )?;
    Ok(app)
}

/// App for fixture: Static server returns index.html for directory
pub fn create_app_static_files_static_server_returns_index_html_for_directory() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    config.static_files.push(StaticFilesConfig {
        directory: "static_assets/static_files_static_server_returns_index_html_for_directory/app_0".to_string(),
        route_prefix: "/app".to_string(),
        index_file: true,
        cache_control: None,
    });
    let mut app = App::new().config(config);
    Ok(app)
}

/// App for fixture: 19_413_payload_too_large
pub fn create_app_status_codes_19_413_payload_too_large() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/upload")
            .handler_name("status_codes_19_413_payload_too_large_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{\"data\":{\"type\":\"string\"}},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_19_413_payload_too_large_handler,
    )?;
    Ok(app)
}

/// App for fixture: 200 OK - Success
pub fn create_app_status_codes_200_ok_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/status-test/{code}").handler_name("status_codes_200_ok_success_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap()), status_codes_200_ok_success_handler)?;
    Ok(app)
}

/// App for fixture: 201 Created - Resource created
pub fn create_app_status_codes_201_created_resource_created() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("status_codes_201_created_resource_created_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), status_codes_201_created_resource_created_handler)?;
    Ok(app)
}

/// App for fixture: 202 Accepted - Request accepted for processing
pub fn create_app_status_codes_202_accepted_request_accepted_for_processing() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/tasks/").handler_name("status_codes_202_accepted_request_accepted_for_processing_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"task\":{\"type\":\"string\"}},\"required\":[\"task\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), status_codes_202_accepted_request_accepted_for_processing_handler)?;
    Ok(app)
}

/// App for fixture: 204 No Content - Success with no body
pub fn create_app_status_codes_204_no_content_success_with_no_body() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(delete("/status-test/{code}").handler_name("status_codes_204_no_content_success_with_no_body_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap()), status_codes_204_no_content_success_with_no_body_handler)?;
    Ok(app)
}

/// App for fixture: 206 Partial Content
pub fn create_app_status_codes_206_partial_content() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/files/document.pdf")
            .handler_name("status_codes_206_partial_content_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_206_partial_content_handler,
    )?;
    Ok(app)
}

/// App for fixture: 20_414_uri_too_long
pub fn create_app_status_codes_20_414_uri_too_long() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/data")
            .handler_name("status_codes_20_414_uri_too_long_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_20_414_uri_too_long_handler,
    )?;
    Ok(app)
}

/// App for fixture: 21_431_request_header_fields_too_large
pub fn create_app_status_codes_21_431_request_header_fields_too_large() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/data").handler_name("status_codes_21_431_request_header_fields_too_large_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"X-Large-Header\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap()), status_codes_21_431_request_header_fields_too_large_handler)?;
    Ok(app)
}

/// App for fixture: 22_501_not_implemented
pub fn create_app_status_codes_22_501_not_implemented() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        RouteBuilder::new(Method::from_str("TRACE").expect("invalid method"), "/data")
            .handler_name("status_codes_22_501_not_implemented_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_22_501_not_implemented_handler,
    )?;
    Ok(app)
}

/// App for fixture: 23_503_service_unavailable
pub fn create_app_status_codes_23_503_service_unavailable() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/data")
            .handler_name("status_codes_23_503_service_unavailable_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_23_503_service_unavailable_handler,
    )?;
    Ok(app)
}

/// App for fixture: 301 Moved Permanently - Permanent redirect
pub fn create_app_status_codes_301_moved_permanently_permanent_redirect() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/old-path")
            .handler_name("status_codes_301_moved_permanently_permanent_redirect_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_301_moved_permanently_permanent_redirect_handler,
    )?;
    Ok(app)
}

/// App for fixture: 302 Found - Temporary redirect
pub fn create_app_status_codes_302_found_temporary_redirect() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/temp-redirect")
            .handler_name("status_codes_302_found_temporary_redirect_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_302_found_temporary_redirect_handler,
    )?;
    Ok(app)
}

/// App for fixture: 304 Not Modified - Cached content valid
pub fn create_app_status_codes_304_not_modified_cached_content_valid() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/status-test/{code}").handler_name("status_codes_304_not_modified_cached_content_valid_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"If-None-Match\":{\"source\":\"header\",\"type\":\"string\"},\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap()), status_codes_304_not_modified_cached_content_valid_handler)?;
    Ok(app)
}

/// App for fixture: 307 Temporary Redirect - Method preserved
pub fn create_app_status_codes_307_temporary_redirect_method_preserved() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/redirect-post")
            .handler_name("status_codes_307_temporary_redirect_method_preserved_handler")
            .request_schema_json(
                serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}")
                    .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_307_temporary_redirect_method_preserved_handler,
    )?;
    Ok(app)
}

/// App for fixture: 400 Bad Request - Invalid request
pub fn create_app_status_codes_400_bad_request_invalid_request() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/items/")
            .handler_name("status_codes_400_bad_request_invalid_request_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"type\":\"string\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_400_bad_request_invalid_request_handler,
    )?;
    Ok(app)
}

/// App for fixture: 401 Unauthorized - Missing authentication
pub fn create_app_status_codes_401_unauthorized_missing_authentication() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/users/me")
            .handler_name("status_codes_401_unauthorized_missing_authentication_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_401_unauthorized_missing_authentication_handler,
    )?;
    Ok(app)
}

/// App for fixture: 403 Forbidden - Insufficient permissions
pub fn create_app_status_codes_403_forbidden_insufficient_permissions() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/admin/users")
            .handler_name("status_codes_403_forbidden_insufficient_permissions_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_403_forbidden_insufficient_permissions_handler,
    )?;
    Ok(app)
}

/// App for fixture: 404 Not Found - Resource not found
pub fn create_app_status_codes_404_not_found_resource_not_found() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/status-test/{code}").handler_name("status_codes_404_not_found_resource_not_found_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap()), status_codes_404_not_found_resource_not_found_handler)?;
    Ok(app)
}

/// App for fixture: 408 Request Timeout
pub fn create_app_status_codes_408_request_timeout() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/slow-endpoint").handler_name("status_codes_408_request_timeout_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), status_codes_408_request_timeout_handler)?;
    Ok(app)
}

/// App for fixture: 422 Unprocessable Entity - Validation error
pub fn create_app_status_codes_422_unprocessable_entity_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("status_codes_422_unprocessable_entity_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"price\",\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), status_codes_422_unprocessable_entity_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: 429 Too Many Requests
pub fn create_app_status_codes_429_too_many_requests() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/api/resource")
            .handler_name("status_codes_429_too_many_requests_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_429_too_many_requests_handler,
    )?;
    Ok(app)
}

/// App for fixture: 500 Internal Server Error - Server error
pub fn create_app_status_codes_500_internal_server_error_server_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/error")
            .handler_name("status_codes_500_internal_server_error_server_error_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_500_internal_server_error_server_error_handler,
    )?;
    Ok(app)
}

/// App for fixture: 503 Service Unavailable - Server overload
pub fn create_app_status_codes_503_service_unavailable_server_overload() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/health")
            .handler_name("status_codes_503_service_unavailable_server_overload_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        status_codes_503_service_unavailable_server_overload_handler,
    )?;
    Ok(app)
}

/// App for fixture: Binary log download
pub fn create_app_streaming_binary_log_download() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/stream/logfile")
            .handler_name("streaming_binary_log_download_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        streaming_binary_log_download_handler,
    )?;
    Ok(app)
}

/// App for fixture: Chunked CSV export
pub fn create_app_streaming_chunked_csv_export() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/stream/csv-report")
            .handler_name("streaming_chunked_csv_export_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        streaming_chunked_csv_export_handler,
    )?;
    Ok(app)
}

/// App for fixture: Stream JSON lines
pub fn create_app_streaming_stream_json_lines() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        get("/stream/json-lines")
            .handler_name("streaming_stream_json_lines_handler")
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        streaming_stream_json_lines_handler,
    )?;
    Ok(app)
}

/// App for fixture: 13_array_field_success
pub fn create_app_url_encoded_13_array_field_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/register").handler_name("url_encoded_13_array_field_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":1,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_13_array_field_success_handler)?;
    Ok(app)
}

/// App for fixture: 14_nested_object_bracket_notation
pub fn create_app_url_encoded_14_nested_object_bracket_notation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/profile").handler_name("url_encoded_14_nested_object_bracket_notation_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"user\":{\"properties\":{\"age\":{\"minimum\":0,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_14_nested_object_bracket_notation_handler)?;
    Ok(app)
}

/// App for fixture: 15_special_characters_field_names
pub fn create_app_url_encoded_15_special_characters_field_names() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/data").handler_name("url_encoded_15_special_characters_field_names_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"contact.email\":{\"format\":\"email\",\"type\":\"string\"},\"user-name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_15_special_characters_field_names_handler)?;
    Ok(app)
}

/// App for fixture: 16_minlength_validation_failure
pub fn create_app_url_encoded_16_minlength_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("url_encoded_16_minlength_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_16_minlength_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 17_pattern_validation_failure
pub fn create_app_url_encoded_17_pattern_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/accounts").handler_name("url_encoded_17_pattern_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"account_id\":{\"pattern\":\"^ACC-[0-9]{6}$\",\"type\":\"string\"}},\"required\":[\"account_id\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_17_pattern_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 18_integer_minimum_validation_failure
pub fn create_app_url_encoded_18_integer_minimum_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/products").handler_name("url_encoded_18_integer_minimum_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"quantity\":{\"minimum\":1,\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_18_integer_minimum_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 19_array_minitems_validation_failure
pub fn create_app_url_encoded_19_array_minitems_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/tags").handler_name("url_encoded_19_array_minitems_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":2,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_19_array_minitems_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 20_format_email_validation_failure
pub fn create_app_url_encoded_20_format_email_validation_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/subscribe").handler_name("url_encoded_20_format_email_validation_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_20_format_email_validation_failure_handler)?;
    Ok(app)
}

/// App for fixture: 21_integer_type_coercion_failure
pub fn create_app_url_encoded_21_integer_type_coercion_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/products")
            .handler_name("url_encoded_21_integer_type_coercion_failure_handler")
            .request_schema_json(
                serde_json::from_str::<Value>(
                    "{\"properties\":{\"price\":{\"type\":\"integer\"}},\"required\":[\"price\"],\"type\":\"object\"}",
                )
                .unwrap(),
            )
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        url_encoded_21_integer_type_coercion_failure_handler,
    )?;
    Ok(app)
}

/// App for fixture: 22_additional_properties_strict_failure
pub fn create_app_url_encoded_22_additional_properties_strict_failure() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/settings").handler_name("url_encoded_22_additional_properties_strict_failure_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"theme\":{\"enum\":[\"light\",\"dark\"],\"type\":\"string\"}},\"required\":[\"theme\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_22_additional_properties_strict_failure_handler)?;
    Ok(app)
}

/// App for fixture: Boolean field conversion
pub fn create_app_url_encoded_boolean_field_conversion() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/").handler_name("url_encoded_boolean_field_conversion_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"subscribe\":{\"type\":\"boolean\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_boolean_field_conversion_handler)?;
    Ok(app)
}

/// App for fixture: Empty string value
pub fn create_app_url_encoded_empty_string_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/").handler_name("url_encoded_empty_string_value_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"description\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_empty_string_value_handler)?;
    Ok(app)
}

/// App for fixture: Multiple values for same field
pub fn create_app_url_encoded_multiple_values_for_same_field() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/tags").handler_name("url_encoded_multiple_values_for_same_field_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_multiple_values_for_same_field_handler)?;
    Ok(app)
}

/// App for fixture: Numeric field type conversion
pub fn create_app_url_encoded_numeric_field_type_conversion() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/").handler_name("url_encoded_numeric_field_type_conversion_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"age\":{\"type\":\"integer\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_numeric_field_type_conversion_handler)?;
    Ok(app)
}

/// App for fixture: OAuth2 password grant flow
pub fn create_app_url_encoded_oauth2_password_grant_flow() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/token").handler_name("url_encoded_oauth2_password_grant_flow_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"grant_type\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\",\"grant_type\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_oauth2_password_grant_flow_handler)?;
    Ok(app)
}

/// App for fixture: Optional field missing - success
pub fn create_app_url_encoded_optional_field_missing_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/register/").handler_name("url_encoded_optional_field_missing_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":[\"string\",\"null\"]},\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_optional_field_missing_success_handler)?;
    Ok(app)
}

/// App for fixture: Pattern validation - fail
pub fn create_app_url_encoded_pattern_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/validated").handler_name("url_encoded_pattern_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"pattern\":\"^[a-z0-9_]+$\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_pattern_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: Required field missing - validation error
pub fn create_app_url_encoded_required_field_missing_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/login/").handler_name("url_encoded_required_field_missing_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_required_field_missing_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: Simple form submission - success
pub fn create_app_url_encoded_simple_form_submission_success() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/login/").handler_name("url_encoded_simple_form_submission_success_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_simple_form_submission_success_handler)?;
    Ok(app)
}

/// App for fixture: Special characters encoding
pub fn create_app_url_encoded_special_characters_encoding() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/").handler_name("url_encoded_special_characters_encoding_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_special_characters_encoding_handler)?;
    Ok(app)
}

/// App for fixture: String max_length validation - fail
pub fn create_app_url_encoded_string_max_length_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/validated").handler_name("url_encoded_string_max_length_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"maxLength\":20,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_string_max_length_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: String min_length validation - fail
pub fn create_app_url_encoded_string_min_length_validation_fail() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/form/validated").handler_name("url_encoded_string_min_length_validation_fail_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), url_encoded_string_min_length_validation_fail_handler)?;
    Ok(app)
}

/// App for fixture: 09_multiple_validation_errors
pub fn create_app_validation_errors_09_multiple_validation_errors() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/users").handler_name("validation_errors_09_multiple_validation_errors_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"age\":{\"minimum\":18,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"name\",\"email\",\"age\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_09_multiple_validation_errors_handler)?;
    Ok(app)
}

/// App for fixture: 10_nested_error_path
pub fn create_app_validation_errors_10_nested_error_path() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/profiles").handler_name("validation_errors_10_nested_error_path_handler").request_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_10_nested_error_path_handler)?;
    Ok(app)
}

/// App for fixture: Array item validation error
pub fn create_app_validation_errors_array_item_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_array_item_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_array_item_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: Array max_items constraint violation
pub fn create_app_validation_errors_array_max_items_constraint_violation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_array_max_items_constraint_violation_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"maxItems\":10,\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_array_max_items_constraint_violation_handler)?;
    Ok(app)
}

/// App for fixture: Array min_items constraint violation
pub fn create_app_validation_errors_array_min_items_constraint_violation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_array_min_items_constraint_violation_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{},\"minItems\":1,\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_array_min_items_constraint_violation_handler)?;
    Ok(app)
}

/// App for fixture: Body field type error - string for float
pub fn create_app_validation_errors_body_field_type_error_string_for_float() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_body_field_type_error_string_for_float_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_body_field_type_error_string_for_float_handler)?;
    Ok(app)
}

/// App for fixture: Header validation error
pub fn create_app_validation_errors_header_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_header_validation_error_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"},\"x-token\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"x-token\",\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_header_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: Invalid UUID format
pub fn create_app_validation_errors_invalid_uuid_format() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/{item_id}").handler_name("validation_errors_invalid_uuid_format_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap()), validation_errors_invalid_uuid_format_handler)?;
    Ok(app)
}

/// App for fixture: Invalid boolean value
pub fn create_app_validation_errors_invalid_boolean_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_invalid_boolean_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"is_active\":{\"source\":\"query\",\"type\":\"boolean\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"is_active\",\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_invalid_boolean_value_handler)?;
    Ok(app)
}

/// App for fixture: Invalid datetime format
pub fn create_app_validation_errors_invalid_datetime_format() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_invalid_datetime_format_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"created_at\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_invalid_datetime_format_handler)?;
    Ok(app)
}

/// App for fixture: Invalid enum value
pub fn create_app_validation_errors_invalid_enum_value() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/models/{model_name}").handler_name("validation_errors_invalid_enum_value_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap()), validation_errors_invalid_enum_value_handler)?;
    Ok(app)
}

/// App for fixture: Malformed JSON body
pub fn create_app_validation_errors_malformed_json_body() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(
        post("/items/")
            .handler_name("validation_errors_malformed_json_body_handler")
            .request_schema_json(serde_json::from_str::<Value>("{\"type\":\"string\"}").unwrap())
            .params_schema_json(
                serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap(),
            ),
        validation_errors_malformed_json_body_handler,
    )?;
    Ok(app)
}

/// App for fixture: Missing required body field
pub fn create_app_validation_errors_missing_required_body_field() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_missing_required_body_field_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_missing_required_body_field_handler)?;
    Ok(app)
}

/// App for fixture: Missing required query parameter
pub fn create_app_validation_errors_missing_required_query_parameter() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_missing_required_query_parameter_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_missing_required_query_parameter_handler)?;
    Ok(app)
}

/// App for fixture: Multiple validation errors
pub fn create_app_validation_errors_multiple_validation_errors() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_multiple_validation_errors_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"minLength\":3,\"type\":\"string\"},\"price\":{\"exclusiveMinimum\":0,\"type\":\"integer\"},\"quantity\":{\"type\":\"integer\"}},\"required\":[\"name\",\"price\",\"quantity\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_multiple_validation_errors_handler)?;
    Ok(app)
}

/// App for fixture: Nested object validation error
pub fn create_app_validation_errors_nested_object_validation_error() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(post("/items/").handler_name("validation_errors_nested_object_validation_error_handler").request_schema_json(serde_json::from_str::<Value>("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"minLength\":3,\"type\":\"string\"},\"zip_code\":{\"minLength\":5,\"type\":\"string\"}},\"required\":[\"city\",\"zip_code\"],\"type\":\"object\"},\"name\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap()).params_schema_json(serde_json::from_str::<Value>("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap()), validation_errors_nested_object_validation_error_handler)?;
    Ok(app)
}

/// App for fixture: Numeric constraint violation - gt (greater than)
pub fn create_app_validation_errors_numeric_constraint_violation_gt_greater_than() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_numeric_constraint_violation_gt_greater_than_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"price\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"number\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"price\",\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_numeric_constraint_violation_gt_greater_than_handler)?;
    Ok(app)
}

/// App for fixture: Numeric constraint violation - le (less than or equal)
pub fn create_app_validation_errors_numeric_constraint_violation_le_less_than_or_equal() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_numeric_constraint_violation_le_less_than_or_equal_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"limit\",\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_numeric_constraint_violation_le_less_than_or_equal_handler)?;
    Ok(app)
}

/// App for fixture: Query param type error - string provided for int
pub fn create_app_validation_errors_query_param_type_error_string_provided_for_int() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_query_param_type_error_string_provided_for_int_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"},\"skip\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"q\",\"skip\"],\"type\":\"object\"}").unwrap()), validation_errors_query_param_type_error_string_provided_for_int_handler)?;
    Ok(app)
}

/// App for fixture: String max_length constraint violation
pub fn create_app_validation_errors_string_max_length_constraint_violation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_string_max_length_constraint_violation_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"maxLength\":50,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_string_max_length_constraint_violation_handler)?;
    Ok(app)
}

/// App for fixture: String min_length constraint violation
pub fn create_app_validation_errors_string_min_length_constraint_violation() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_string_min_length_constraint_violation_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_string_min_length_constraint_violation_handler)?;
    Ok(app)
}

/// App for fixture: String regex pattern mismatch
pub fn create_app_validation_errors_string_regex_pattern_mismatch() -> Result<App, AppError> {
    let mut config = ServerConfig::default();
    let mut app = App::new().config(config);
    app.route(get("/items/").handler_name("validation_errors_string_regex_pattern_mismatch_handler").params_schema_json(serde_json::from_str::<Value>("{\"properties\":{\"q\":{\"pattern\":\"^[a-zA-Z0-9_-]+$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap()), validation_errors_string_regex_pattern_mismatch_handler)?;
    Ok(app)
}

/// App for SSE channel: /notifications
pub fn create_app_sse_notifications() -> Result<App, AppError> {
    let mut app = App::new();
    app.route(
        get("/notifications").handler_name("sse_notifications_handler"),
        sse_notifications_handler,
    )?;
    Ok(app)
}

/// App for WebSocket channel: /chat
pub fn create_app_websocket_chat() -> Result<App, AppError> {
    let mut app = App::new();
    app.websocket("/chat", ChatWebSocketHandler);
    Ok(app)
}

async fn auth_api_key_authentication_invalid_key_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"The provided API key is not valid\",\"status\":401,\"title\":\"Invalid API key\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_api_key_authentication_missing_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Expected 'X-API-Key' header or 'api_key' query parameter with valid API key\",\"status\":401,\"title\":\"Missing API key\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_api_key_authentication_valid_key_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"data\":\"sensitive information\",\"message\":\"Access granted\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_api_key_in_query_parameter_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"data\":\"sensitive information\",\"message\":\"Access granted\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_api_key_rotation_old_key_still_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"data\":\"sensitive information\",\"message\":\"Access granted\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("x-api-key-deprecated", "true")]);
    Ok(response)
}

async fn auth_api_key_with_custom_header_name_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"data\":\"sensitive information\",\"message\":\"Access granted\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_bearer_token_without_prefix_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Authorization header must use Bearer scheme: 'Bearer <token>'\",\"status\":401,\"title\":\"Invalid Authorization header format\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_authentication_expired_token_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Token has expired\",\"status\":401,\"title\":\"JWT validation failed\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_authentication_invalid_audience_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Token audience is invalid\",\"status\":401,\"title\":\"JWT validation failed\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_authentication_invalid_signature_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Token signature is invalid\",\"status\":401,\"title\":\"JWT validation failed\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_authentication_missing_authorization_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Expected 'Authorization: Bearer <token>'\",\"status\":401,\"title\":\"Missing or invalid Authorization header\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_authentication_valid_token_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Access granted\",\"user_id\":\"user123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_invalid_issuer_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Token issuer is invalid, expected 'https://auth.example.com'\",\"status\":401,\"title\":\"JWT validation failed\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_malformed_token_format_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Malformed JWT token: expected 3 parts separated by dots, found 2\",\"status\":401,\"title\":\"Malformed JWT token\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_missing_required_custom_claims_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Required claims 'role' and 'permissions' missing from JWT\",\"status\":403,\"title\":\"Forbidden\",\"type\":\"https://spikard.dev/errors/forbidden\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_not_before_claim_in_future_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"JWT not valid yet, not before claim is in the future\",\"status\":401,\"title\":\"JWT validation failed\",\"type\":\"https://spikard.dev/errors/unauthorized\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_jwt_with_multiple_audiences_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Access granted\",\"user_id\":\"user123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn auth_multiple_authentication_schemes_jwt_precedence_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"auth_method\":\"jwt\",\"message\":\"Access granted\",\"user_id\":\"user123\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn background_background_event_logging_handler(
    ctx: RequestContext,
    state: Arc<Mutex<Vec<Value>>>,
) -> HandlerResult {
    let body = ctx.body_value();
    let value = body.get("event").cloned();
    let value = match value {
        Some(val) => val,
        None => {
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": "missing background value"}).to_string()))
                .unwrap();
            let response = apply_expected_headers(response, &[("content-type", "application/json")]);

            return Ok(response);
        }
    };

    {
        let mut guard = state.lock().await;
        guard.push(value);
    }

    let response = Response::builder()
        .status(StatusCode::from_u16(202).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json")]);

    Ok(response)
}

async fn background_background_event_logging_handler_background_state(
    _ctx: RequestContext,
    state: Arc<Mutex<Vec<Value>>>,
) -> HandlerResult {
    let values = {
        let guard = state.lock().await;
        guard.clone()
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "events": values }).to_string()))
        .unwrap();
    Ok(response)
}

async fn background_background_event_logging_second_payload_handler(
    ctx: RequestContext,
    state: Arc<Mutex<Vec<Value>>>,
) -> HandlerResult {
    let body = ctx.body_value();
    let value = body.get("event").cloned();
    let value = match value {
        Some(val) => val,
        None => {
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json!({"error": "missing background value"}).to_string()))
                .unwrap();
            let response = apply_expected_headers(response, &[("content-type", "application/json")]);

            return Ok(response);
        }
    };

    {
        let mut guard = state.lock().await;
        guard.push(value);
    }

    let response = Response::builder()
        .status(StatusCode::from_u16(202).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json")]);

    Ok(response)
}

async fn background_background_event_logging_second_payload_handler_background_state(
    _ctx: RequestContext,
    state: Arc<Mutex<Vec<Value>>>,
) -> HandlerResult {
    let values = {
        let guard = state.lock().await;
        guard.clone()
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({ "events": values }).to_string()))
        .unwrap();
    Ok(response)
}

async fn body_limits_body_over_limit_returns_413_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(413).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn body_limits_body_under_limit_succeeds_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"accepted\":true,\"note\":\"small\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn compression_compression_gzip_applied_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Compressed payload\",\"payload\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("vary", "Accept-Encoding")]);
    Ok(response)
}

async fn compression_compression_payload_below_min_size_is_not_compressed_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Small payload\",\"payload\":\"tiny\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_13_json_with_charset_utf16_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported.\",\"status\":415,\"title\":\"Unsupported Charset\",\"type\":\"https://spikard.dev/errors/unsupported-charset\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(415).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_14_content_type_case_insensitive_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"test\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_15_multipart_boundary_required_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"error\":\"multipart/form-data requires 'boundary' parameter\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(400).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_16_text_plain_not_accepted_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Unsupported media type\",\"status\":415,\"title\":\"Unsupported Media Type\",\"type\":\"https://spikard.dev/errors/unsupported-media-type\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(415).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_17_vendor_json_accepted_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"data\":\"value\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_18_content_type_with_multiple_params_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":\"test\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_19_missing_content_type_default_json_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"test\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_20_content_length_mismatch_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"error\":\"Content-Length header does not match actual body size\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(400).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_415_unsupported_media_type_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Unsupported media type\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(415).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn content_types_binary_response_application_octet_stream_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"binary_data_placeholder\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("content-type", "application/octet-stream"),
            ("content-disposition", "attachment; filename=file.bin"),
        ],
    );
    Ok(response)
}

async fn content_types_csv_response_text_csv_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"id,name,price\\n1,Item A,10.0\\n2,Item B,20.0\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("content-type", "text/csv; charset=utf-8"),
            ("content-disposition", "attachment; filename=data.csv"),
        ],
    );
    Ok(response)
}

async fn content_types_content_negotiation_accept_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":1,\"name\":\"Item\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json")]);
    Ok(response)
}

async fn content_types_html_response_text_html_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"<html><body><h1>Hello</h1></body></html>\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "text/html; charset=utf-8")]);
    Ok(response)
}

async fn content_types_jpeg_image_response_image_jpeg_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"jpeg_binary_data\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "image/jpeg")]);
    Ok(response)
}

async fn content_types_json_response_application_json_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":42.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json")]);
    Ok(response)
}

async fn content_types_json_with_utf_8_charset_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"emoji\":\"☕\",\"name\":\"Café\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json; charset=utf-8")]);
    Ok(response)
}

async fn content_types_pdf_response_application_pdf_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"pdf_binary_data\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("content-type", "application/pdf"),
            ("content-disposition", "attachment; filename=document.pdf"),
        ],
    );
    Ok(response)
}

async fn content_types_png_image_response_image_png_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"png_binary_data\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "image/png")]);
    Ok(response)
}

async fn content_types_plain_text_response_text_plain_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"Hello, World!\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "text/plain; charset=utf-8")]);
    Ok(response)
}

async fn content_types_xml_response_application_xml_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("\"<?xml version=\\\"1.0\\\"?><item><name>Item</name><price>42.0</price></item>\"")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/xml")]);
    Ok(response)
}

async fn cookies_24_cookie_samesite_strict_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cookies_25_cookie_samesite_lax_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cookies_26_cookie_secure_flag_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cookies_27_cookie_httponly_flag_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cookies_apikey_cookie_authentication_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"cookie\",\"key\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_apikey_cookie_authentication_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"username\":\"secret\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_cookie_regex_pattern_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[A-Z0-9]{8}$\"},\"input\":\"invalid-format\",\"loc\":[\"cookie\",\"tracking_id\"],\"msg\":\"String should match pattern '^[A-Z0-9]{8}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_cookie_regex_pattern_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"tracking_id\":\"ABC12345\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_cookie_validation_max_length_constraint_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_cookie_value_is_way_too_long\",\"loc\":[\"cookie\",\"session_id\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_cookie_validation_min_length_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"token\":\"abc\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_cookie_validation_min_length_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"cookie\",\"tracking_id\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_multiple_cookies_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"fatebook_tracker\":\"tracker456\",\"googall_tracker\":\"ga789\",\"session_id\":\"session123\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_optional_apikey_cookie_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"msg\":\"Create an account first\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_optional_cookie_parameter_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ads_id\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_optional_cookie_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ads_id\":\"abc123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_required_cookie_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"cookie\",\"session_id\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_delete_cookie_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie deleted\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_multiple_cookies_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Multiple cookies set\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_session_cookie_no_max_age_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Session cookie set\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_samesite_lax_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set with SameSite=Lax\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_samesite_none_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set with SameSite=None\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_samesite_strict_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set with SameSite=Strict\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_attributes_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_domain_attribute_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set with domain\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_cookie_with_path_attribute_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Cookie set with path\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cookies_response_set_cookie_basic_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Come to the dark side, we have cookies\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cors_06_cors_preflight_method_not_allowed_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cors_07_cors_preflight_header_not_allowed_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn cors_08_cors_max_age_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(204).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-methods", "POST"),
            ("access-control-allow-headers", "Content-Type"),
            ("access-control-allow-origin", "https://example.com"),
            ("access-control-max-age", "3600"),
        ],
    );
    Ok(response)
}

async fn cors_09_cors_expose_headers_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("x-request-id", "abc123"),
            ("access-control-expose-headers", "X-Total-Count, X-Request-Id"),
            ("access-control-allow-origin", "https://example.com"),
            ("x-total-count", "42"),
        ],
    );
    Ok(response)
}

async fn cors_10_cors_origin_null_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"error\":\"Origin 'null' is not allowed\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cors_cors_private_network_access_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(204).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-origin", "https://public.example.com"),
            ("vary", "Origin"),
            ("access-control-allow-methods", "GET, POST"),
            ("access-control-allow-private-network", "true"),
        ],
    );
    Ok(response)
}

async fn cors_cors_vary_header_for_proper_caching_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"data\":\"cacheable resource\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-origin", "https://app.example.com"),
            ("cache-control", "public, max-age=3600"),
            ("vary", "Origin"),
        ],
    );
    Ok(response)
}

async fn cors_cors_multiple_allowed_origins_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"data\":\"resource data\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("vary", "Origin"),
            ("access-control-allow-origin", "https://admin.example.com"),
        ],
    );
    Ok(response)
}

async fn cors_cors_origin_case_sensitivity_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(response, &[("vary", "Origin")]);
    Ok(response)
}

async fn cors_cors_preflight_for_delete_method_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(204).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("vary", "Origin"),
            ("access-control-allow-origin", "https://app.example.com"),
            ("access-control-allow-methods", "GET, POST, PUT, PATCH, DELETE"),
            ("access-control-max-age", "3600"),
        ],
    );
    Ok(response)
}

async fn cors_cors_preflight_for_put_method_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(204).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-origin", "https://app.example.com"),
            ("access-control-max-age", "3600"),
            ("vary", "Origin"),
            ("access-control-allow-headers", "Content-Type, X-Custom-Header"),
            ("access-control-allow-methods", "GET, POST, PUT, PATCH, DELETE"),
        ],
    );
    Ok(response)
}

async fn cors_cors_preflight_request_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-headers", "Content-Type, X-Custom-Header"),
            ("access-control-max-age", "600"),
            ("access-control-allow-origin", "https://example.com"),
            ("access-control-allow-methods", "GET, POST, PUT, DELETE, OPTIONS"),
        ],
    );
    Ok(response)
}

async fn cors_cors_regex_pattern_matching_for_origins_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"data\":\"resource data\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-origin", "https://subdomain.example.com"),
            ("vary", "Origin"),
        ],
    );
    Ok(response)
}

async fn cors_cors_request_blocked_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"detail\":\"CORS request from origin 'https://malicious-site.com' not allowed\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn cors_cors_safelisted_headers_without_preflight_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Success\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("vary", "Origin"),
            ("access-control-allow-origin", "https://app.example.com"),
        ],
    );
    Ok(response)
}

async fn cors_cors_wildcard_origin_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"data\":\"public\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("access-control-allow-origin", "*")]);
    Ok(response)
}

async fn cors_cors_with_credentials_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"username\":\"john\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-credentials", "true"),
            ("vary", "Origin"),
            ("access-control-allow-origin", "https://app.example.com"),
        ],
    );
    Ok(response)
}

async fn cors_simple_cors_request_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"items\":[]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("vary", "Origin"),
            ("access-control-allow-origin", "https://example.com"),
        ],
    );
    Ok(response)
}

async fn edge_cases_11_utf8_query_parameter_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"term\":\"café\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_12_percent_encoded_special_chars_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"term\":\"hi there\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_13_empty_string_query_param_preserved_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"filter\":\"\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_14_large_integer_boundary_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":9007199254740991}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_15_float_precision_preservation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":3.141592653589793}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_16_negative_zero_handling_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"offset\":0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_17_extremely_long_string_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":10001,\"max_length\":10000},\"loc\":[\"body\",\"content\"],\"msg\":\"String length must not exceed 10000\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_18_unicode_normalization_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"café\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_19_emoji_in_strings_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"text\":\"Hello 👋 World 🌍\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_20_null_byte_in_string_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"value\":\"file\\\\u0000.txt\"},\"loc\":[\"body\",\"filename\"],\"msg\":\"String contains null byte character\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_21_scientific_notation_number_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":123000}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_22_leading_zeros_integer_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":123}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_23_deeply_nested_json_limit_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"error\":\"Request body exceeds maximum nesting depth of 32\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(400).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_24_array_with_holes_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"items\":[\"first\",\"third\",\"sixth\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_deeply_nested_structure_10_levels_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"max_depth\":10,\"message\":\"Processed deeply nested structure\",\"value_found\":\"deep\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_empty_and_null_value_handling_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"empty_array_length\":0,\"empty_object_keys\":0,\"empty_string_length\":0,\"explicit_null_is_null\":true,\"false_is_false\":true,\"zero_is_falsy\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_float_precision_and_rounding_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"precise_value\":3.141592653589793,\"sum\":0.30000000000000004,\"very_large\":1.7976931348623157e308,\"very_small\":1e-10}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_large_integer_boundary_values_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"large_int\":9223372036854775807,\"max_safe_int\":9007199254740991,\"negative_large\":-9223372036854775808}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_special_string_values_and_escaping_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"backslashes\":\"C:\\\\\\\\Users\\\\\\\\Path\",\"empty_string\":\"\",\"quotes\":\"He said \\\"hello\\\" and 'goodbye'\",\"special_chars\":\"!@#$%^&*()_+-=[]{}|;':\\\",./<>?\",\"tabs_newlines\":\"line1\\n\\tline2\\r\\nline3\",\"unicode_escapes\":\"Hello\",\"whitespace\":\"   \"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn edge_cases_unicode_and_emoji_handling_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"description\":\"Best café in München 🇩🇪\",\"emoji_reactions\":\"👍❤️😂🎉\",\"id\":1,\"name\":\"Coffee Shop ☕\",\"tags\":[\"食べ物\",\"音楽\",\"💰\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_30_bearer_token_format_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn headers_31_bearer_token_format_invalid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"value\":\"Bearer invalid token with spaces\"},\"loc\":[\"headers\",\"authorization\"],\"msg\":\"Invalid Bearer token format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_32_bearer_token_missing_prefix_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"value\":\"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\"},\"loc\":[\"headers\",\"authorization\"],\"msg\":\"Invalid Bearer token format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_33_api_key_header_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn headers_34_api_key_header_invalid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-f0-9]{32}$\",\"value\":\"invalid-key\"},\"loc\":[\"headers\",\"x-api-key\"],\"msg\":\"Invalid API key format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_accept_header_json_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"accept\":\"application/json\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_accept_encoding_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"accept_encoding\":\"gzip, deflate, br\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_accept_language_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"accept_language\":\"en-US,en;q=0.9\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_authorization_header_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"headers\",\"authorization\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_authorization_header_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"credentials\":\"foobar\",\"scheme\":\"Digest\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_authorization_header_wrong_scheme_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"Other invalidauthorization\",\"loc\":[\"headers\",\"authorization\"],\"msg\":\"String should match pattern '^Digest .+'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_basic_authentication_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"password\":\"password\",\"username\":\"username\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_bearer_token_authentication_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"headers\",\"authorization\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_bearer_token_authentication_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"token\":\"valid_token_123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_content_type_header_application_json_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"content_type\":\"application/json\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_case_insensitivity_access_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"content_type_lower\":\"application/json\",\"content_type_mixed\":\"application/json\",\"content_type_upper\":\"application/json\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_regex_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[0-9]{3,}$\"},\"input\":\"invalid-format\",\"loc\":[\"headers\",\"x-request-id\"],\"msg\":\"String should match pattern '^[0-9]{3,}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_regex_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"x_request_id\":\"12345\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_validation_max_length_constraint_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_is_way_too_long_for_validation\",\"loc\":[\"headers\",\"x-session-id\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_validation_min_length_constraint_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"headers\",\"x-token\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_header_with_underscore_conversion_explicit_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"x_token\":\"secret123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_host_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"host\":\"example.com:8080\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_multiple_custom_headers_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"x_client_version\":\"1.2.3\",\"x_request_id\":\"req-12345\",\"x_trace_id\":\"trace-abc\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_multiple_header_values_x_token_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"X-Token values\":[\"foo\",\"bar\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_optional_header_with_none_default_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"strange_header\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_origin_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"origin\":\"https://example.com\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_referer_header_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"referer\":\"https://example.com/page\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_user_agent_header_custom_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"User-Agent\":\"Mozilla/5.0 Custom Browser\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_user_agent_header_default_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"User-Agent\":\"testclient\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_x_api_key_optional_header_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"msg\":\"Hello World\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_x_api_key_optional_header_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"msg\":\"Hello secret\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_x_api_key_required_header_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"headers\",\"x-api-key\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn headers_x_api_key_required_header_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"username\":\"secret\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_delete_remove_resource_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_delete_resource_not_found_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_delete_with_response_body_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"id\":1,\"message\":\"Item deleted successfully\",\"name\":\"Deleted Item\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_head_get_metadata_without_body_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[("content-type", "application/json"), ("content-length", "85")],
    );
    Ok(response)
}

async fn http_methods_options_cors_preflight_request_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("access-control-allow-headers", "Content-Type"),
            ("access-control-max-age", "86400"),
            ("access-control-allow-origin", "https://example.com"),
            ("access-control-allow-methods", "GET, POST, PUT, DELETE, OPTIONS"),
        ],
    );
    Ok(response)
}

async fn http_methods_patch_partial_update_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"id\":1,\"in_stock\":true,\"name\":\"Existing Item\",\"price\":79.99}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_patch_update_multiple_fields_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"id\":1,\"in_stock\":false,\"name\":\"Updated Name\",\"price\":89.99}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_put_complete_resource_replacement_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"description\":\"Completely replaced\",\"id\":1,\"in_stock\":true,\"name\":\"Updated Item\",\"price\":99.99}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_put_create_resource_if_doesn_t_exist_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":999,\"name\":\"New Item\",\"price\":49.99}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_put_idempotent_operation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":1,\"name\":\"Fixed Name\",\"price\":50.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_put_missing_required_field_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"1\",\"loc\":[\"body\",\"price\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn http_methods_put_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"2 validation errors in request\",\"errors\":[{\"input\":\"X\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"input\":-10,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_29_nested_object_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_30_nested_object_missing_field_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"required\":true},\"loc\":[\"body\",\"profile\",\"email\"],\"msg\":\"Field required\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_31_nullable_property_null_value_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_32_schema_ref_definitions_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_33_allof_schema_composition_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_34_additional_properties_false_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"additional_properties\":false,\"unexpected_field\":\"extra_field\"},\"loc\":[\"body\",\"extra_field\"],\"msg\":\"Additional properties are not allowed\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_35_oneof_schema_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_36_oneof_schema_multiple_match_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":2},\"loc\":[\"body\"],\"msg\":\"Must match exactly one schema (oneOf), but matched 2\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_37_oneof_schema_no_match_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":0},\"loc\":[\"body\"],\"msg\":\"Must match exactly one schema (oneOf), but matched 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_38_anyof_schema_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_39_anyof_schema_multiple_match_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_40_anyof_schema_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":0},\"loc\":[\"body\"],\"msg\":\"Must match at least one schema (anyOf), but matched 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_41_not_schema_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_42_not_schema_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"prohibited_value\":\"admin\"},\"loc\":[\"body\",\"username\"],\"msg\":\"Must not match the schema\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_43_const_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_44_const_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"const\":\"1.0\",\"value\":\"2.0\"},\"loc\":[\"body\",\"version\"],\"msg\":\"Value must be exactly '1.0'\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_45_minproperties_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_46_minproperties_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_properties\":1,\"min_properties\":2},\"loc\":[\"body\"],\"msg\":\"Object must have at least 2 properties\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_47_maxproperties_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_properties\":4,\"max_properties\":3},\"loc\":[\"body\"],\"msg\":\"Object must have at most 3 properties\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_48_dependencies_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_49_dependencies_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"dependency\":\"credit_card\",\"required_fields\":[\"billing_address\"]},\"loc\":[\"body\"],\"msg\":\"When 'credit_card' is present, 'billing_address' is required\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_50_deep_nesting_4_levels_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn json_bodies_array_of_objects_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"images\":[{\"name\":\"Front\",\"url\":\"https://example.com/img1.jpg\"},{\"name\":\"Back\",\"url\":\"https://example.com/img2.jpg\"}],\"name\":\"Product Bundle\",\"tags\":[\"electronics\",\"gadget\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_array_of_primitive_values_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"name\":\"Product\",\"ratings\":[4.5,4.8,5.0,4.2],\"tags\":[\"electronics\",\"gadget\",\"new\"]}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_body_with_query_parameters_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item\":{\"name\":\"Item\",\"price\":42.0},\"limit\":10}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_boolean_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"in_stock\":true,\"name\":\"Item\",\"price\":42.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_date_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"event_date\":\"2024-03-15\",\"name\":\"Conference\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_datetime_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"created_at\":\"2024-03-15T10:30:00Z\",\"name\":\"Meeting\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_deeply_nested_objects_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"Product\",\"price\":100.0,\"seller\":{\"address\":{\"city\":\"Springfield\",\"country\":{\"code\":\"US\",\"name\":\"USA\"},\"street\":\"123 Main St\"},\"name\":\"John Doe\"}}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_empty_json_object_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":null,\"name\":null,\"price\":null,\"tax\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_empty_array_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":1},\"input\":[],\"loc\":[\"body\",\"tags\"],\"msg\":\"List should have at least 1 item after validation\",\"type\":\"too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_enum_field_invalid_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'electronics', 'clothing' or 'books'\"},\"input\":\"furniture\",\"loc\":[\"body\",\"category\"],\"msg\":\"Input should be 'electronics', 'clothing' or 'books'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_enum_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"category\":\"electronics\",\"name\":\"Item\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_extra_fields_ignored_no_additionalproperties_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":42.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_field_type_validation_invalid_type_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not a number\",\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be a valid number\",\"type\":\"float_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_nested_object_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"image\":{\"name\":\"Product Image\",\"url\":\"https://example.com/image.jpg\"},\"name\":\"Foo\",\"price\":42.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_null_value_for_optional_field_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":null,\"name\":\"Item\",\"price\":42.0,\"tax\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_numeric_ge_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"ge\":1},\"input\":0.5,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than or equal to 1\",\"type\":\"greater_than_equal\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_numeric_le_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":100.0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_optional_fields_omitted_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":null,\"name\":\"Foo\",\"price\":35.4,\"tax\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_patch_partial_update_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":\"Original description\",\"name\":\"Original Item\",\"price\":45.0}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_required_field_missing_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"name\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_simple_json_object_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":\"A very nice Item\",\"name\":\"Foo\",\"price\":35.4,\"tax\":3.2}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_string_max_length_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":50},\"input\":\"This is a very long name that exceeds the maximum length\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at most 50 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_string_min_length_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_string_pattern_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[A-Z]{3}[0-9]{4}$\"},\"input\":\"ABC-123\",\"loc\":[\"body\",\"sku\"],\"msg\":\"String should match pattern '^[A-Z]{3}[0-9]{4}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_string_pattern_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"Item\",\"sku\":\"ABC1234\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_uuid_field_invalid_format_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-valid-uuid\",\"loc\":[\"body\",\"item_id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn json_bodies_uuid_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"item_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\",\"name\":\"Item\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_hook_execution_order_first_hook_on_request_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_hook_execution_order_second_hook_on_request_1(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_hook_execution_order_third_hook_on_request_2(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_hook_execution_order_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"execution_order\":[\"first_hook\",\"second_hook\",\"third_hook\"],\"message\":\"Hooks executed in order\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_multiple_hooks_all_phases_request_logger_on_request_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_request_id_generator_on_request_1(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_rate_limiter_pre_validation_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_authenticator_pre_handler_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_authorizer_pre_handler_1(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_security_headers_on_response_0(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut()
        .insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    resp.headers_mut().insert("X-Frame-Options", "DENY".parse().unwrap());
    resp.headers_mut()
        .insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    resp.headers_mut().insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_response_timer_on_response_1(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut().insert("X-Response-Time", ".*ms".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_audit_logger_on_response_2(
    resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_error_logger_on_error_0(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_multiple_hooks_all_phases_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"action\":\"update_profile\",\"message\":\"Action completed successfully\",\"request_id\":\".*\",\"user_id\":\"user-123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("x-request-id", ".*"),
            ("x-response-time", ".*ms"),
            ("x-content-type-options", "nosniff"),
            ("x-frame-options", "DENY"),
        ],
    );
    Ok(response)
}

async fn lifecycle_hooks_onerror_error_logging_error_logger_on_error_0(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_onerror_error_logging_error_formatter_on_error_1(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_onerror_error_logging_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"error\":\"Internal Server Error\",\"error_id\":\".*\",\"message\":\"An unexpected error occurred\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(500).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("content-type", "application/json")]);
    Ok(response)
}

async fn lifecycle_hooks_onrequest_request_logging_request_logger_on_request_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_onrequest_request_logging_request_id_generator_on_request_1(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_onrequest_request_logging_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"has_request_id\":true,\"message\":\"onRequest hooks executed\",\"request_logged\":true}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("x-request-id", ".*")]);
    Ok(response)
}

async fn lifecycle_hooks_onresponse_response_timing_start_timer_on_request_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_onresponse_response_timing_response_timer_on_response_0(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut().insert("X-Response-Time", ".*ms".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_onresponse_response_timing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Response with timing info\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("x-response-time", ".*ms")]);
    Ok(response)
}

async fn lifecycle_hooks_onresponse_security_headers_security_headers_on_response_0(
    mut resp: axum::http::Response<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>>, String> {
    resp.headers_mut()
        .insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    resp.headers_mut().insert("X-Frame-Options", "DENY".parse().unwrap());
    resp.headers_mut()
        .insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    resp.headers_mut().insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    Ok(spikard::HookResult::Continue(resp))
}

async fn lifecycle_hooks_onresponse_security_headers_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"message\":\"Response with security headers\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("x-content-type-options", "nosniff"),
            ("x-frame-options", "DENY"),
            ("strict-transport-security", "max-age=31536000; includeSubDomains"),
            ("x-xss-protection", "1; mode=block"),
        ],
    );
    Ok(response)
}

async fn lifecycle_hooks_prehandler_authentication_failed_short_circuit_authenticator_pre_handler_0(
    _req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    use axum::response::IntoResponse;
    let response = (
        axum::http::StatusCode::UNAUTHORIZED,
        axum::Json(serde_json::json!({
            "error": "Unauthorized",
            "message": "Invalid or expired authentication token"
        })),
    )
        .into_response();
    Ok(spikard::HookResult::ShortCircuit(response))
}

async fn lifecycle_hooks_prehandler_authentication_failed_short_circuit_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"error\":\"Unauthorized\",\"message\":\"Invalid or expired authentication token\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_prehandler_authentication_success_authenticator_pre_handler_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_prehandler_authentication_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"authenticated\":true,\"message\":\"Access granted\",\"user_id\":\"user-123\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_prehandler_authorization_check_authenticator_pre_handler_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_prehandler_authorization_check_authorizer_pre_handler_1(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_prehandler_authorization_check_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"message\":\"Admin access granted\",\"role\":\"admin\",\"user_id\":\"admin-456\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authenticator_pre_handler_0(
    _req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    use axum::response::IntoResponse;
    let response = (
        axum::http::StatusCode::FORBIDDEN,
        axum::Json(serde_json::json!({
            "error": "Forbidden",
            "message": "Admin role required for this endpoint"
        })),
    )
        .into_response();
    Ok(spikard::HookResult::ShortCircuit(response))
}

async fn lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_authorizer_pre_handler_1(
    _req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    use axum::response::IntoResponse;
    let response = (
        axum::http::StatusCode::FORBIDDEN,
        axum::Json(serde_json::json!({
            "error": "Forbidden",
            "message": "Admin role required for this endpoint"
        })),
    )
        .into_response();
    Ok(spikard::HookResult::ShortCircuit(response))
}

async fn lifecycle_hooks_prehandler_authorization_forbidden_short_circuit_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"error\":\"Forbidden\",\"message\":\"Admin role required for this endpoint\"}")
            .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_rate_limiter_pre_validation_0(
    _req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    use axum::response::IntoResponse;
    let mut response = (
        axum::http::StatusCode::TOO_MANY_REQUESTS,
        axum::Json(serde_json::json!({
            "error": "Rate limit exceeded",
            "message": "Too many requests, please try again later"
        })),
    )
        .into_response();
    response.headers_mut().insert("Retry-After", "60".parse().unwrap());
    Ok(spikard::HookResult::ShortCircuit(response))
}

async fn lifecycle_hooks_prevalidation_rate_limit_exceeded_short_circuit_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"error\":\"Rate limit exceeded\",\"message\":\"Too many requests, please try again later\"}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(429).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("retry-after", "60")]);
    Ok(response)
}

async fn lifecycle_hooks_prevalidation_rate_limiting_rate_limiter_pre_validation_0(
    req: axum::http::Request<axum::body::Body>,
) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>>, String> {
    Ok(spikard::HookResult::Continue(req))
}

async fn lifecycle_hooks_prevalidation_rate_limiting_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"message\":\"Request accepted\",\"rate_limit_checked\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_17_file_magic_number_png_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn multipart_18_file_magic_number_jpeg_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn multipart_19_file_mime_spoofing_png_as_jpeg_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"declared_mime\":\"image/jpeg\",\"detected_type\":\"image/png\",\"magic_bytes\":\"89504e470d0a1a0a\"},\"loc\":[\"files\",\"image\"],\"msg\":\"File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_20_file_mime_spoofing_jpeg_as_png_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"declared_mime\":\"image/png\",\"detected_type\":\"image/jpeg\",\"magic_bytes\":\"ffd8ffe0\"},\"loc\":[\"files\",\"image\"],\"msg\":\"File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_21_file_pdf_magic_number_success_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn multipart_22_file_empty_buffer_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"buffer_size\":0},\"loc\":[\"files\",\"file\"],\"msg\":\"File buffer is empty\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_content_type_validation_invalid_type_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn multipart_empty_file_upload_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"filename\":\"empty.txt\",\"size\":0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_file_list_upload_array_of_files_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"filenames\":[\"file1.txt\",\"file2.txt\"],\"total_size\":35}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_file_size_validation_too_large_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"File too large. Maximum size is 1MB\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(413).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_file_upload_with_custom_headers_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"test2\":{\"content\":\"<file2 content>\",\"content_type\":\"text/plain\",\"filename\":\"test2.txt\",\"headers\":[[\"content-disposition\",\"form-data; name=\\\"test2\\\"; filename=\\\"test2.txt\\\"\"],[\"content-type\",\"text/plain\"],[\"x-custom\",\"f2\"]],\"size\":15}}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_file_upload_without_filename_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"test1\":\"<file1 content>\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_form_data_without_files_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"some\":\"data\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_image_file_upload_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"content_type\":\"image/jpeg\",\"filename\":\"photo.jpg\",\"size\":22}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_mixed_files_and_form_data_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"active\":\"true\",\"age\":\"25\",\"file\":{\"content\":\"file data here\",\"content_type\":\"text/plain\",\"filename\":\"upload.txt\",\"size\":14},\"username\":\"testuser\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_multiple_file_uploads_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"test1\":{\"content\":\"<file1 content>\",\"content_type\":\"text/plain\",\"filename\":\"test1.txt\",\"size\":15},\"test2\":{\"content\":\"<file2 content>\",\"content_type\":\"text/plain\",\"filename\":\"test2.txt\",\"size\":15}}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_multiple_values_for_same_field_name_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"files\":[{\"content\":\"first file\",\"content_type\":\"text/plain\",\"filename\":\"file1.txt\",\"size\":10},{\"content\":\"second file\",\"content_type\":\"text/plain\",\"filename\":\"file2.txt\",\"size\":11}],\"tags\":[\"python\",\"rust\",\"web\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_optional_file_upload_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"file\":null}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_optional_file_upload_provided_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"content_type\":\"text/plain\",\"filename\":\"optional.txt\",\"size\":21}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_pdf_file_upload_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"content_type\":\"application/pdf\",\"filename\":\"report.pdf\",\"size\":16}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_required_file_upload_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"required\",\"loc\":[\"body\",\"file\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn multipart_simple_file_upload_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"test\":{\"content\":\"<file content>\",\"content_type\":\"text/plain\",\"filename\":\"test.txt\",\"size\":14}}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_20_uuid_v3_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":\"e8b5a51d-11c8-3310-a6ab-367563f20686\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_21_uuid_v5_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":\"630eb68f-e0fa-5ecc-887a-7c7a62614681\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_24_date_format_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"date\":\"2025-10-30\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_25_date_format_invalid_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"date\",\"value\":\"2025-13-45\"},\"loc\":[\"path\",\"date\"],\"msg\":\"Invalid date format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_27_datetime_format_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"timestamp\":\"2025-10-30T14:30:00Z\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_28_duration_format_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"duration\":\"P1DT2H30M\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_29_decimal_path_param_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"amount\":\"19.99\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_30_string_minlength_path_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"username\":\"alice\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_31_string_minlength_path_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3},\"loc\":[\"path\",\"username\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_32_string_maxlength_path_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":42,\"max_length\":20},\"loc\":[\"path\",\"username\"],\"msg\":\"String length must not exceed 20\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_33_string_pattern_path_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"owner\":\"spikard-labs\",\"repo\":\"spikard-http\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_34_string_pattern_path_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"value\":\"invalid@owner\"},\"loc\":[\"path\",\"owner\"],\"msg\":\"String does not match pattern\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_35_negative_integer_path_param_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":-100}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_boolean_path_parameter_true_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_boolean_path_parameter_numeric_1_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_date_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"date_param\":\"2023-07-15\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_enum_path_parameter_invalid_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"foo\",\"loc\":[\"path\",\"model_name\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_enum_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"model_name\":\"alexnet\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_float_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":42.5}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_invalid_string_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"foobar\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":42}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_combined_lt_and_gt_constraints_success_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":2}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_ge_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":3}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_gt_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"gt\":3},\"input\":2,\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be greater than 3\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_gt_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":42}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_le_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":3}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_integer_path_parameter_with_lt_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":2}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_multiple_path_parameters_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str(
        "{\"order_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\",\"service_id\":1,\"user_id\":\"abc\",\"version\":1.0}",
    )
    .unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_path_parameter_type_syntax_invalid_uuid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"path\",\"id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_path_parameter_type_syntax_with_override_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"count\":\"50\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_path_parameter_with_type_syntax_uuid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":\"550e8400-e29b-41d4-a716-446655440000\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_path_parameter_with_type_syntax_integer_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"user_id\":\"42\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_path_type_parameter_file_path_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"file_path\":\"home/johndoe/myfile.txt\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_string_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":\"foobar\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_string_path_parameter_with_max_length_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":3},\"input\":\"foobar\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"String should have at most 3 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_string_path_parameter_with_min_length_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"fo\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn path_params_uuid_path_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":\"ec38df32-ceda-4cfa-9b4a-1aeb94ad551a\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_42_negative_integer_query_param_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"offset\":-10}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_43_scientific_notation_float_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"threshold\":0.0015}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_44_string_minlength_validation_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"term\":\"foo\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_45_string_minlength_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3},\"loc\":[\"query\",\"term\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_46_string_maxlength_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":21,\"max_length\":10},\"loc\":[\"query\",\"term\"],\"msg\":\"String length must not exceed 10\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_47_pattern_validation_email_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"email\":\"user@example.com\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_48_pattern_validation_email_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"value\":\"invalid-email\"},\"loc\":[\"query\",\"email\"],\"msg\":\"String does not match pattern\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_49_integer_gt_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"limit\":5}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_50_integer_gt_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"exclusive_minimum\":0,\"value\":0},\"loc\":[\"query\",\"limit\"],\"msg\":\"Value must be greater than 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_51_integer_ge_constraint_boundary_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"offset\":0}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_52_integer_le_constraint_boundary_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"limit\":100}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_53_integer_le_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"maximum\":100,\"value\":101},\"loc\":[\"query\",\"limit\"],\"msg\":\"Value must not exceed 100\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_54_array_minitems_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ids\":[1,2,3]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_55_array_minitems_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":1,\"min_items\":2},\"loc\":[\"query\",\"ids\"],\"msg\":\"Array must contain at least 2 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_56_array_maxitems_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":6,\"max_items\":5},\"loc\":[\"query\",\"tags\"],\"msg\":\"Array must not contain more than 5 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_57_boolean_empty_string_coercion_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"active\":false}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_58_format_email_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"email\":\"user@example.com\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_59_format_email_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"email\",\"value\":\"not-an-email\"},\"loc\":[\"query\",\"email\"],\"msg\":\"Invalid email format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_60_format_ipv4_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ip\":\"192.168.1.1\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_61_format_ipv4_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"ipv4\",\"value\":\"999.999.999.999\"},\"loc\":[\"query\",\"ip\"],\"msg\":\"Invalid IPv4 address format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_62_format_ipv6_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ip\":\"2001:0db8:85a3:0000:0000:8a2e:0370:7334\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_63_format_uri_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"url\":\"https://example.com/path?query=value\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_64_format_uri_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"uri\",\"value\":\"not a uri\"},\"loc\":[\"query\",\"url\"],\"msg\":\"Invalid URI format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_65_format_hostname_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"host\":\"api.example.com\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_66_multipleof_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"quantity\":15}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_67_multipleof_constraint_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"multiple_of\":5,\"value\":17},\"loc\":[\"query\",\"quantity\"],\"msg\":\"Value must be a multiple of 5\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_68_array_uniqueitems_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"ids\":[1,2,3,4]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_69_array_uniqueitems_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"duplicate_index\":2,\"duplicate_value\":2,\"unique_items\":true},\"loc\":[\"query\",\"ids\"],\"msg\":\"Array items must be unique\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_70_array_separator_pipe_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"tags\":[\"python\",\"rust\",\"typescript\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_71_array_separator_semicolon_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"colors\":[\"red\",\"green\",\"blue\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_72_array_separator_space_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"keywords\":[\"rust\",\"web\",\"framework\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_array_query_parameter_empty_array_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("[]").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_array_query_parameter_single_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("[\"apple\"]").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_boolean_query_parameter_numeric_1_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"flag\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_boolean_query_parameter_true_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"flag\":true}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_date_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"event_date\":\"2024-01-15\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_datetime_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"timestamp\":\"2024-01-15T10:30:00Z\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_enum_query_parameter_invalid_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"vgg16\",\"loc\":[\"query\",\"model\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_enum_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"model\":\"alexnet\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_float_query_param_with_ge_constraint_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"price\":0.01}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_query_param_with_ge_constraint_boundary_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":10}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_query_param_with_gt_constraint_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":1}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_query_param_with_le_constraint_boundary_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":100}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_query_param_with_lt_constraint_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"value\":49}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_with_default_value_not_provided_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar 10\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_integer_with_default_value_override_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar 50\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_list_of_integers_multiple_values_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("[1,2]").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_list_of_strings_multiple_values_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"q\":[\"foo\",\"bar\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_list_query_parameter_required_but_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"device_ids\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_list_with_default_empty_array_no_values_provided_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("[]").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_multiple_query_parameters_with_different_types_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"active\":true,\"age\":30,\"name\":\"john\",\"score\":95.5}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_optional_integer_query_parameter_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar None\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_optional_query_parameter_with_default_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"limit\":10}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_optional_string_query_parameter_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar None\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_optional_string_query_parameter_provided_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar baz\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_query_parameter_with_url_encoded_space_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"hello world\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_query_parameter_with_url_encoded_special_characters_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"name\":\"test&value=123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_query_parameter_with_special_characters_url_encoding_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"email\":\"x@test.com\",\"special\":\"&@A.ac\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_integer_query_parameter_float_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":42.5,\"loc\":[\"query\",\"query\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_integer_query_parameter_invalid_type_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"baz\",\"loc\":[\"query\",\"query\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_integer_query_parameter_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"query\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_integer_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar 42\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_string_query_parameter_missing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"query\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_required_string_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"foo bar baz\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_string_query_param_with_max_length_constraint_fail_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":10},\"input\":\"this_is_way_too_long\",\"loc\":[\"query\",\"name\"],\"msg\":\"String should have at most 10 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_string_query_param_with_min_length_constraint_fail_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"query\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_string_query_param_with_regex_pattern_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[0-9]{3,}$\"},\"input\":\"abc123\",\"loc\":[\"query\",\"code\"],\"msg\":\"String should match pattern '^[0-9]{3,}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_string_validation_with_regex_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^fixedquery$\"},\"input\":\"nonregexquery\",\"loc\":[\"query\",\"item_query\"],\"msg\":\"String should match pattern '^fixedquery$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_string_validation_with_regex_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_query\":\"fixedquery\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_uuid_query_parameter_invalid_format_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"query\",\"item_id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn query_params_uuid_query_parameter_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"item_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn rate_limit_rate_limit_below_threshold_succeeds_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"request\":\"under-limit\",\"status\":\"ok\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn rate_limit_rate_limit_exceeded_returns_429_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(429).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn request_id_request_id_header_is_preserved_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"echo\":\"trace-123\",\"status\":\"preserved\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("x-request-id", "trace-123")]);
    Ok(response)
}

async fn request_id_request_id_is_generated_when_not_provided_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"status\":\"generated\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("x-request-id", "00000000-0000-4000-8000-000000000000")]);
    Ok(response)
}

async fn request_id_request_id_middleware_can_be_disabled_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"status\":\"no-request-id\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn request_timeout_request_completes_before_timeout_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"duration\":\"fast\",\"status\":\"ok\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn request_timeout_request_exceeds_timeout_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(408).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn static_files_static_file_server_returns_text_file_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"Hello from static storage\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[("cache-control", "public, max-age=60"), ("content-type", "text/plain")],
    );
    Ok(response)
}

async fn status_codes_19_413_payload_too_large_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"error\":\"Payload Too Large\",\"message\":\"Request body size exceeds maximum allowed size of 1024 bytes\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(413).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_200_ok_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":1,\"name\":\"Item 1\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_201_created_resource_created_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"id\":1,\"name\":\"New Item\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_202_accepted_request_accepted_for_processing_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"message\":\"Task accepted for processing\",\"task_id\":\"abc123\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(202).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_204_no_content_success_with_no_body_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(204).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn status_codes_206_partial_content_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("\"binary_data_1024_bytes\"").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(206).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("accept-ranges", "bytes"),
            ("content-range", "bytes 0-1023/5000"),
            ("content-type", "application/pdf"),
            ("content-length", "1024"),
        ],
    );
    Ok(response)
}

async fn status_codes_20_414_uri_too_long_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_21_431_request_header_fields_too_large_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"error\":\"Request Header Fields Too Large\",\"message\":\"Request headers exceed maximum allowed size of 8192 bytes\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(431).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_22_501_not_implemented_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(405).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn status_codes_23_503_service_unavailable_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"error\":\"Service Unavailable\",\"message\":\"The service is temporarily unavailable. Please try again later.\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(503).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("retry-after", "60")]);
    Ok(response)
}

async fn status_codes_301_moved_permanently_permanent_redirect_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(301).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(response, &[("location", "/new-path")]);
    Ok(response)
}

async fn status_codes_302_found_temporary_redirect_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(302).unwrap())
        .body(Body::empty())
        .unwrap();
    let response = apply_expected_headers(response, &[("location", "/target-path")]);
    Ok(response)
}

async fn status_codes_304_not_modified_cached_content_valid_handler(_ctx: RequestContext) -> HandlerResult {
    let response = Response::builder()
        .status(StatusCode::from_u16(304).unwrap())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}

async fn status_codes_307_temporary_redirect_method_preserved_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(307).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("location", "/target-post")]);
    Ok(response)
}

async fn status_codes_400_bad_request_invalid_request_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Invalid request format\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(400).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_401_unauthorized_missing_authentication_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Not authenticated\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(401).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("www-authenticate", "Bearer")]);
    Ok(response)
}

async fn status_codes_403_forbidden_insufficient_permissions_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Not enough permissions\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(403).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_404_not_found_resource_not_found_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Item not found\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(404).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_408_request_timeout_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Request timeout\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(408).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("connection", "close")]);
    Ok(response)
}

async fn status_codes_422_unprocessable_entity_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"name\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_429_too_many_requests_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"detail\":\"Rate limit exceeded. Try again in 60 seconds.\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(429).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(
        response,
        &[
            ("x-ratelimit-limit", "100"),
            ("x-ratelimit-reset", "1609459200"),
            ("x-ratelimit-remaining", "0"),
            ("retry-after", "60"),
        ],
    );
    Ok(response)
}

async fn status_codes_500_internal_server_error_server_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Internal server error\",\"status\":500,\"title\":\"Internal Server Error\",\"type\":\"https://spikard.dev/errors/internal-server-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(500).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn status_codes_503_service_unavailable_server_overload_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Service temporarily unavailable\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(503).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    let response = apply_expected_headers(response, &[("retry-after", "120")]);
    Ok(response)
}

async fn streaming_binary_log_download_handler(_ctx: RequestContext) -> HandlerResult {
    let stream = stream::iter(vec![
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[0x4cu8, 0x4fu8, 0x47u8, 0x3au8])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[0x00u8, 0x01u8, 0x02u8, 0x03u8])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[0x7cu8, 0x54u8, 0x41u8, 0x49u8, 0x4cu8, 0x7cu8])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[0x07u8])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[0x5cu8, 0x6eu8])),
    ]);

    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::from_u16(200).unwrap())
        .into_response();
    let response = apply_expected_headers(response, &[("content-type", "application/octet-stream")]);

    Ok(response)
}

async fn streaming_chunked_csv_export_handler(_ctx: RequestContext) -> HandlerResult {
    let stream = stream::iter(vec![
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x69u8, 0x64u8, 0x2cu8, 0x6eu8, 0x61u8, 0x6du8, 0x65u8, 0x2cu8, 0x76u8, 0x61u8, 0x6cu8, 0x75u8, 0x65u8,
            0x5cu8, 0x6eu8,
        ])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x31u8, 0x2cu8, 0x41u8, 0x6cu8, 0x69u8, 0x63u8, 0x65u8, 0x2cu8, 0x34u8, 0x32u8, 0x5cu8, 0x6eu8,
        ])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x32u8, 0x2cu8, 0x42u8, 0x6fu8, 0x62u8, 0x2cu8, 0x37u8, 0x5cu8, 0x6eu8,
        ])),
    ]);

    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::from_u16(200).unwrap())
        .into_response();
    let response = apply_expected_headers(response, &[("content-type", "text/csv")]);

    Ok(response)
}

async fn streaming_stream_json_lines_handler(_ctx: RequestContext) -> HandlerResult {
    let stream = stream::iter(vec![
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x7bu8, 0x22u8, 0x69u8, 0x6eu8, 0x64u8, 0x65u8, 0x78u8, 0x22u8, 0x3au8, 0x30u8, 0x2cu8, 0x22u8, 0x70u8,
            0x61u8, 0x79u8, 0x6cu8, 0x6fu8, 0x61u8, 0x64u8, 0x22u8, 0x3au8, 0x22u8, 0x61u8, 0x6cu8, 0x70u8, 0x68u8,
            0x61u8, 0x22u8, 0x7du8, 0x5cu8, 0x6eu8,
        ])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x7bu8, 0x22u8, 0x69u8, 0x6eu8, 0x64u8, 0x65u8, 0x78u8, 0x22u8, 0x3au8, 0x31u8, 0x2cu8, 0x22u8, 0x70u8,
            0x61u8, 0x79u8, 0x6cu8, 0x6fu8, 0x61u8, 0x64u8, 0x22u8, 0x3au8, 0x22u8, 0x62u8, 0x65u8, 0x74u8, 0x61u8,
            0x22u8, 0x7du8, 0x5cu8, 0x6eu8,
        ])),
        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[
            0x7bu8, 0x22u8, 0x69u8, 0x6eu8, 0x64u8, 0x65u8, 0x78u8, 0x22u8, 0x3au8, 0x32u8, 0x2cu8, 0x22u8, 0x70u8,
            0x61u8, 0x79u8, 0x6cu8, 0x6fu8, 0x61u8, 0x64u8, 0x22u8, 0x3au8, 0x22u8, 0x67u8, 0x61u8, 0x6du8, 0x6du8,
            0x61u8, 0x22u8, 0x7du8, 0x5cu8, 0x6eu8,
        ])),
    ]);

    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::from_u16(200).unwrap())
        .into_response();
    let response = apply_expected_headers(response, &[("content-type", "application/x-ndjson")]);

    Ok(response)
}

async fn url_encoded_13_array_field_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"tags\":[\"python\",\"rust\",\"typescript\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_14_nested_object_bracket_notation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"user\":{\"age\":30,\"email\":\"john@example.com\",\"name\":\"John Doe\"}}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_15_special_characters_field_names_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"contact.email\":\"john@example.com\",\"user-name\":\"JohnDoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(201).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_16_minlength_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3,\"value\":\"ab\"},\"loc\":[\"body\",\"username\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_17_pattern_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^ACC-[0-9]{6}$\",\"value\":\"INVALID123\"},\"loc\":[\"body\",\"account_id\"],\"msg\":\"String does not match pattern '^ACC-[0-9]{6}$'\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_18_integer_minimum_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_value\":0,\"minimum\":1},\"loc\":[\"body\",\"quantity\"],\"msg\":\"Value must be at least 1\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_19_array_minitems_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":1,\"min_items\":2},\"loc\":[\"body\",\"tags\"],\"msg\":\"Array must contain at least 2 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_20_format_email_validation_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"email\",\"value\":\"not-an-email\"},\"loc\":[\"body\",\"email\"],\"msg\":\"Invalid email format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_21_integer_type_coercion_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"value\":\"not-a-number\"},\"loc\":[\"body\",\"price\"],\"msg\":\"Value is not a valid integer\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_22_additional_properties_strict_failure_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"property\":\"unknown_field\"},\"loc\":[\"body\",\"unknown_field\"],\"msg\":\"Additional properties are not allowed\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_boolean_field_conversion_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"subscribe\":true,\"username\":\"johndoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_empty_string_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"description\":\"\",\"username\":\"johndoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_multiple_values_for_same_field_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"tags\":[\"python\",\"fastapi\",\"web\"]}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_numeric_field_type_conversion_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"age\":30,\"username\":\"johndoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_oauth2_password_grant_flow_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"access_token\":\"johndoe\",\"token_type\":\"bearer\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_optional_field_missing_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"email\":null,\"username\":\"johndoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_pattern_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-z0-9_]+$\"},\"input\":\"john doe\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should match pattern '^[a-z0-9_]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_required_field_missing_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"username\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_simple_form_submission_success_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"username\":\"johndoe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_special_characters_encoding_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value =
        serde_json::from_str("{\"description\":\"Test & Development\",\"name\":\"John Doe\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(200).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_string_max_length_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_is_a_very_long_username_that_exceeds_limit\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn url_encoded_string_min_length_validation_fail_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_09_multiple_validation_errors_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"ge\":18},\"input\":15,\"loc\":[\"body\",\"age\"],\"msg\":\"Input should be greater than or equal to 18\",\"type\":\"greater_than_equal\"},{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"},\"input\":\"invalid-email\",\"loc\":[\"body\",\"email\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$'\",\"type\":\"string_pattern_mismatch\"},{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_10_nested_error_path_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"},\"input\":\"invalid\",\"loc\":[\"body\",\"profile\",\"contact\",\"email\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_array_item_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":123,\"loc\":[\"body\",\"tags\",\"2\"],\"msg\":\"Input should be a valid unknown\",\"type\":\"type_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_array_max_items_constraint_violation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":[\"tag1\",\"tag2\",\"tag3\",\"tag4\",\"tag5\",\"tag6\",\"tag7\",\"tag8\",\"tag9\",\"tag10\",\"tag11\"],\"loc\":[\"body\",\"tags\"],\"msg\":\"[\\\"tag1\\\",\\\"tag2\\\",\\\"tag3\\\",\\\"tag4\\\",\\\"tag5\\\",\\\"tag6\\\",\\\"tag7\\\",\\\"tag8\\\",\\\"tag9\\\",\\\"tag10\\\",\\\"tag11\\\"] has more than 10 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_array_min_items_constraint_violation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":[],\"loc\":[\"body\",\"tags\"],\"msg\":\"[] has less than 1 item\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_body_field_type_error_string_for_float_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not_a_float\",\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be a valid number, unable to parse string as a number\",\"type\":\"float_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_header_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"headers\",\"x-token\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_invalid_uuid_format_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_invalid_boolean_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"maybe\",\"loc\":[\"query\",\"is_active\"],\"msg\":\"Input should be a valid boolean, unable to interpret input\",\"type\":\"bool_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_invalid_datetime_format_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-datetime\",\"loc\":[\"body\",\"created_at\"],\"msg\":\"Input should be a valid datetime\",\"type\":\"datetime_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_invalid_enum_value_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"invalid_model\",\"loc\":[\"path\",\"model_name\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_malformed_json_body_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"Invalid request format\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(400).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_missing_required_body_field_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":{\"name\":\"Item\"},\"loc\":[\"body\",\"price\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_missing_required_query_parameter_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"q\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_multiple_validation_errors_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"X\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"gt\":0},\"input\":-10,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"},{\"input\":\"not_a_number\",\"loc\":[\"body\",\"quantity\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_nested_object_validation_error_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"SF\",\"loc\":[\"body\",\"seller\",\"address\",\"city\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"min_length\":5},\"input\":\"123\",\"loc\":[\"body\",\"seller\",\"address\",\"zip_code\"],\"msg\":\"String should have at least 5 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"min_length\":3},\"input\":\"Jo\",\"loc\":[\"body\",\"seller\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_numeric_constraint_violation_gt_greater_than_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"gt\":0},\"input\":\"0\",\"loc\":[\"query\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_numeric_constraint_violation_le_less_than_or_equal_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"le\":100},\"input\":\"101\",\"loc\":[\"query\",\"limit\"],\"msg\":\"Input should be less than or equal to 100\",\"type\":\"less_than_equal\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_query_param_type_error_string_provided_for_int_handler(
    _ctx: RequestContext,
) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not_a_number\",\"loc\":[\"query\",\"skip\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_string_max_length_constraint_violation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":50},\"input\":\"this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should have at most 50 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_string_min_length_constraint_violation_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn validation_errors_string_regex_pattern_mismatch_handler(_ctx: RequestContext) -> HandlerResult {
    let body_value: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_-]+$\"},\"input\":\"invalid!\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_-]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
    Ok(response)
}

async fn sse_notifications_handler(_ctx: RequestContext) -> HandlerResult {
    let events: Vec<String> = vec!["data: {\"level\":\"example_level\",\"message\":\"example_message\",\"source\":\"example_source\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}

", "data: {\"body\":\"example_body\",\"priority\":\"example_priority\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"example_title\",\"type\":\"user_notification\",\"userId\":\"example_userId\"}

", "data: {\"message\":\"example_message\",\"metadata\":{},\"service\":\"example_service\",\"status\":\"example_status\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}

"].into_iter().map(String::from).collect::<Vec<_>>();
    let stream = stream::iter(
        events
            .into_iter()
            .map(|chunk| Ok::<Bytes, std::io::Error>(Bytes::from(chunk))),
    );
    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::OK)
        .with_header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/event-stream"),
        )
        .with_header(
            HeaderName::from_static("cache-control"),
            HeaderValue::from_static("no-cache"),
        )
        .into_response();
    Ok(response)
}

struct ChatWebSocketHandler;

impl WebSocketHandler for ChatWebSocketHandler {
    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send {
        async move {
            let mut data = message;
            if let Some(obj) = data.as_object_mut() {
                obj.insert("validated".to_string(), Value::Bool(true));
            }
            Some(data)
        }
    }
}
