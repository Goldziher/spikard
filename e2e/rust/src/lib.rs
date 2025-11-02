//! Generated route handlers - one handler per fixture for complete isolation

use axum::response::IntoResponse;
use axum::{
    Json, Router, middleware, routing,
    routing::{delete, get, head, options, patch, post, put, trace},
};
use form_urlencoded;
use serde_json::{Value, json};
use spikard_http::parameters::ParameterValidator;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct CorsConfig {
    allow_origin: &'static str,
    allow_methods: &'static str,
    allow_headers: &'static str,
}

// Default app for backwards compatibility (empty)
pub fn create_app() -> Router {
    Router::new()
}

// Per-fixture app functions
/// App for fixture: Simple form submission - success
pub fn create_app_url_encoded_Simple_form_submission___success() -> Router {
    Router::new()
        .route("/login/", post(url_encoded_Simple_form_submission___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 15_special_characters_field_names
pub fn create_app_url_encoded_15_special_characters_field_names() -> Router {
    Router::new()
        .route("/data", post(url_encoded_15_special_characters_field_names_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Pattern validation - fail
pub fn create_app_url_encoded_Pattern_validation___fail() -> Router {
    Router::new()
        .route("/form/validated", post(url_encoded_Pattern_validation___fail_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 22_additional_properties_strict_failure
pub fn create_app_url_encoded_22_additional_properties_strict_failure() -> Router {
    Router::new()
        .route(
            "/settings",
            post(url_encoded_22_additional_properties_strict_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 17_pattern_validation_failure
pub fn create_app_url_encoded_17_pattern_validation_failure() -> Router {
    Router::new()
        .route("/accounts", post(url_encoded_17_pattern_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_format_email_validation_failure
pub fn create_app_url_encoded_20_format_email_validation_failure() -> Router {
    Router::new()
        .route(
            "/subscribe",
            post(url_encoded_20_format_email_validation_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple values for same field
pub fn create_app_url_encoded_Multiple_values_for_same_field() -> Router {
    Router::new()
        .route("/form/tags", post(url_encoded_Multiple_values_for_same_field_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required field missing - validation error
pub fn create_app_url_encoded_Required_field_missing___validation_error() -> Router {
    Router::new()
        .route(
            "/login/",
            post(url_encoded_Required_field_missing___validation_error_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 13_array_field_success
pub fn create_app_url_encoded_13_array_field_success() -> Router {
    Router::new()
        .route("/register", post(url_encoded_13_array_field_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Numeric field type conversion
pub fn create_app_url_encoded_Numeric_field_type_conversion() -> Router {
    Router::new()
        .route("/form/", post(url_encoded_Numeric_field_type_conversion_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Special characters encoding
pub fn create_app_url_encoded_Special_characters_encoding() -> Router {
    Router::new()
        .route("/form/", post(url_encoded_Special_characters_encoding_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean field conversion
pub fn create_app_url_encoded_Boolean_field_conversion() -> Router {
    Router::new()
        .route("/form/", post(url_encoded_Boolean_field_conversion_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Empty string value
pub fn create_app_url_encoded_Empty_string_value() -> Router {
    Router::new()
        .route("/form/", post(url_encoded_Empty_string_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: OAuth2 password grant flow
pub fn create_app_url_encoded_OAuth2_password_grant_flow() -> Router {
    Router::new()
        .route("/token", post(url_encoded_OAuth2_password_grant_flow_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 19_array_minitems_validation_failure
pub fn create_app_url_encoded_19_array_minitems_validation_failure() -> Router {
    Router::new()
        .route("/tags", post(url_encoded_19_array_minitems_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional field missing - success
pub fn create_app_url_encoded_Optional_field_missing___success() -> Router {
    Router::new()
        .route("/register/", post(url_encoded_Optional_field_missing___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 14_nested_object_bracket_notation
pub fn create_app_url_encoded_14_nested_object_bracket_notation() -> Router {
    Router::new()
        .route("/profile", post(url_encoded_14_nested_object_bracket_notation_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String max_length validation - fail
pub fn create_app_url_encoded_String_max_length_validation___fail() -> Router {
    Router::new()
        .route(
            "/form/validated",
            post(url_encoded_String_max_length_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 18_integer_minimum_validation_failure
pub fn create_app_url_encoded_18_integer_minimum_validation_failure() -> Router {
    Router::new()
        .route(
            "/products",
            post(url_encoded_18_integer_minimum_validation_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 21_integer_type_coercion_failure
pub fn create_app_url_encoded_21_integer_type_coercion_failure() -> Router {
    Router::new()
        .route("/products", post(url_encoded_21_integer_type_coercion_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 16_minlength_validation_failure
pub fn create_app_url_encoded_16_minlength_validation_failure() -> Router {
    Router::new()
        .route("/users", post(url_encoded_16_minlength_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String min_length validation - fail
pub fn create_app_url_encoded_String_min_length_validation___fail() -> Router {
    Router::new()
        .route(
            "/form/validated",
            post(url_encoded_String_min_length_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 408 Request Timeout
pub fn create_app_status_codes_408_Request_Timeout() -> Router {
    Router::new()
        .route("/slow-endpoint", post(status_codes_408_Request_Timeout_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 404 Not Found - Resource not found
pub fn create_app_status_codes_404_Not_Found___Resource_not_found() -> Router {
    Router::new()
        .route(
            "/status-test/{code}",
            get(status_codes_404_Not_Found___Resource_not_found_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 503 Service Unavailable - Server overload
pub fn create_app_status_codes_503_Service_Unavailable___Server_overload() -> Router {
    Router::new()
        .route(
            "/health",
            get(status_codes_503_Service_Unavailable___Server_overload_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 422 Unprocessable Entity - Validation error
pub fn create_app_status_codes_422_Unprocessable_Entity___Validation_error() -> Router {
    Router::new()
        .route(
            "/items/",
            post(status_codes_422_Unprocessable_Entity___Validation_error_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 302 Found - Temporary redirect
pub fn create_app_status_codes_302_Found___Temporary_redirect() -> Router {
    Router::new()
        .route(
            "/temp-redirect",
            get(status_codes_302_Found___Temporary_redirect_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 304 Not Modified - Cached content valid
pub fn create_app_status_codes_304_Not_Modified___Cached_content_valid() -> Router {
    Router::new()
        .route(
            "/status-test/{code}",
            get(status_codes_304_Not_Modified___Cached_content_valid_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 400 Bad Request - Invalid request
pub fn create_app_status_codes_400_Bad_Request___Invalid_request() -> Router {
    Router::new()
        .route("/items/", post(status_codes_400_Bad_Request___Invalid_request_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 22_501_not_implemented
pub fn create_app_status_codes_22_501_not_implemented() -> Router {
    Router::new()
        .route("/data", trace(status_codes_22_501_not_implemented_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 204 No Content - Success with no body
pub fn create_app_status_codes_204_No_Content___Success_with_no_body() -> Router {
    Router::new()
        .route(
            "/status-test/{code}",
            delete(status_codes_204_No_Content___Success_with_no_body_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 301 Moved Permanently - Permanent redirect
pub fn create_app_status_codes_301_Moved_Permanently___Permanent_redirect() -> Router {
    Router::new()
        .route(
            "/old-path",
            get(status_codes_301_Moved_Permanently___Permanent_redirect_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 201 Created - Resource created
pub fn create_app_status_codes_201_Created___Resource_created() -> Router {
    Router::new()
        .route("/items/", post(status_codes_201_Created___Resource_created_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 202 Accepted - Request accepted for processing
pub fn create_app_status_codes_202_Accepted___Request_accepted_for_processing() -> Router {
    Router::new()
        .route(
            "/tasks/",
            post(status_codes_202_Accepted___Request_accepted_for_processing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 307 Temporary Redirect - Method preserved
pub fn create_app_status_codes_307_Temporary_Redirect___Method_preserved() -> Router {
    Router::new()
        .route(
            "/redirect-post",
            post(status_codes_307_Temporary_Redirect___Method_preserved_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 500 Internal Server Error - Server error
pub fn create_app_status_codes_500_Internal_Server_Error___Server_error() -> Router {
    Router::new()
        .route(
            "/error",
            get(status_codes_500_Internal_Server_Error___Server_error_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_414_uri_too_long
pub fn create_app_status_codes_20_414_uri_too_long() -> Router {
    Router::new()
        .route("/data", get(status_codes_20_414_uri_too_long_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 401 Unauthorized - Missing authentication
pub fn create_app_status_codes_401_Unauthorized___Missing_authentication() -> Router {
    Router::new()
        .route(
            "/users/me",
            get(status_codes_401_Unauthorized___Missing_authentication_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 23_503_service_unavailable
pub fn create_app_status_codes_23_503_service_unavailable() -> Router {
    Router::new()
        .route("/data", get(status_codes_23_503_service_unavailable_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 19_413_payload_too_large
pub fn create_app_status_codes_19_413_payload_too_large() -> Router {
    Router::new()
        .route("/upload", post(status_codes_19_413_payload_too_large_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 403 Forbidden - Insufficient permissions
pub fn create_app_status_codes_403_Forbidden___Insufficient_permissions() -> Router {
    Router::new()
        .route(
            "/admin/users",
            get(status_codes_403_Forbidden___Insufficient_permissions_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 21_431_request_header_fields_too_large
pub fn create_app_status_codes_21_431_request_header_fields_too_large() -> Router {
    Router::new()
        .route(
            "/data",
            get(status_codes_21_431_request_header_fields_too_large_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 429 Too Many Requests
pub fn create_app_status_codes_429_Too_Many_Requests() -> Router {
    Router::new()
        .route("/api/resource", get(status_codes_429_Too_Many_Requests_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 200 OK - Success
pub fn create_app_status_codes_200_OK___Success() -> Router {
    Router::new()
        .route("/status-test/{code}", get(status_codes_200_OK___Success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 206 Partial Content
pub fn create_app_status_codes_206_Partial_Content() -> Router {
    Router::new()
        .route("/files/document.pdf", get(status_codes_206_Partial_Content_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header regex validation - success
pub fn create_app_headers_Header_regex_validation___success() -> Router {
    Router::new()
        .route(
            "/headers/pattern",
            get(headers_Header_regex_validation___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 33_api_key_header_valid
pub fn create_app_headers_33_api_key_header_valid() -> Router {
    Router::new()
        .route("/api/data", get(headers_33_api_key_header_valid_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Content-Type header - application/json
pub fn create_app_headers_Content_Type_header___application_json() -> Router {
    Router::new()
        .route(
            "/headers/content-type",
            get(headers_Content_Type_header___application_json_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Accept-Language header
pub fn create_app_headers_Accept_Language_header() -> Router {
    Router::new()
        .route("/headers/accept-language", get(headers_Accept_Language_header_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: X-API-Key required header - success
pub fn create_app_headers_X_API_Key_required_header___success() -> Router {
    Router::new()
        .route("/users/me", get(headers_X_API_Key_required_header___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header validation - max_length constraint fail
pub fn create_app_headers_Header_validation___max_length_constraint_fail() -> Router {
    Router::new()
        .route(
            "/headers/max-length",
            get(headers_Header_validation___max_length_constraint_fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: X-API-Key required header - missing
pub fn create_app_headers_X_API_Key_required_header___missing() -> Router {
    Router::new()
        .route("/users/me", get(headers_X_API_Key_required_header___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Origin header
pub fn create_app_headers_Origin_header() -> Router {
    Router::new()
        .route("/headers/origin", get(headers_Origin_header_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: User-Agent header - default value
pub fn create_app_headers_User_Agent_header___default_value() -> Router {
    Router::new()
        .route("/items/", get(headers_User_Agent_header___default_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 32_bearer_token_missing_prefix
pub fn create_app_headers_32_bearer_token_missing_prefix() -> Router {
    Router::new()
        .route("/protected", get(headers_32_bearer_token_missing_prefix_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional header with None default - missing
pub fn create_app_headers_Optional_header_with_None_default___missing() -> Router {
    Router::new()
        .route(
            "/items/",
            get(headers_Optional_header_with_None_default___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header regex validation - fail
pub fn create_app_headers_Header_regex_validation___fail() -> Router {
    Router::new()
        .route("/headers/pattern", get(headers_Header_regex_validation___fail_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 31_bearer_token_format_invalid
pub fn create_app_headers_31_bearer_token_format_invalid() -> Router {
    Router::new()
        .route("/protected", get(headers_31_bearer_token_format_invalid_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: X-API-Key optional header - success
pub fn create_app_headers_X_API_Key_optional_header___success() -> Router {
    Router::new()
        .route("/users/me", get(headers_X_API_Key_optional_header___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Authorization header - success
pub fn create_app_headers_Authorization_header___success() -> Router {
    Router::new()
        .route("/users/me", get(headers_Authorization_header___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 30_bearer_token_format_valid
pub fn create_app_headers_30_bearer_token_format_valid() -> Router {
    Router::new()
        .route("/protected", get(headers_30_bearer_token_format_valid_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Authorization header - missing
pub fn create_app_headers_Authorization_header___missing() -> Router {
    Router::new()
        .route("/users/me", get(headers_Authorization_header___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Accept header - JSON
pub fn create_app_headers_Accept_header___JSON() -> Router {
    Router::new()
        .route("/headers/accept", get(headers_Accept_header___JSON_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Accept-Encoding header
pub fn create_app_headers_Accept_Encoding_header() -> Router {
    Router::new()
        .route("/headers/accept-encoding", get(headers_Accept_Encoding_header_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Authorization header - wrong scheme
pub fn create_app_headers_Authorization_header___wrong_scheme() -> Router {
    Router::new()
        .route("/users/me", get(headers_Authorization_header___wrong_scheme_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header validation - min_length constraint
pub fn create_app_headers_Header_validation___min_length_constraint() -> Router {
    Router::new()
        .route(
            "/headers/validated",
            get(headers_Header_validation___min_length_constraint_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Basic authentication - success
pub fn create_app_headers_Basic_authentication___success() -> Router {
    Router::new()
        .route(
            "/headers/basic-auth",
            get(headers_Basic_authentication___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Bearer token authentication - missing
pub fn create_app_headers_Bearer_token_authentication___missing() -> Router {
    Router::new()
        .route(
            "/headers/bearer-auth",
            get(headers_Bearer_token_authentication___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: X-API-Key optional header - missing
pub fn create_app_headers_X_API_Key_optional_header___missing() -> Router {
    Router::new()
        .route("/users/me", get(headers_X_API_Key_optional_header___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple custom headers
pub fn create_app_headers_Multiple_custom_headers() -> Router {
    Router::new()
        .route("/headers/multiple", get(headers_Multiple_custom_headers_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 34_api_key_header_invalid
pub fn create_app_headers_34_api_key_header_invalid() -> Router {
    Router::new()
        .route("/api/data", get(headers_34_api_key_header_invalid_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Bearer token authentication - success
pub fn create_app_headers_Bearer_token_authentication___success() -> Router {
    Router::new()
        .route(
            "/headers/bearer-auth",
            get(headers_Bearer_token_authentication___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Host header
pub fn create_app_headers_Host_header() -> Router {
    Router::new()
        .route("/headers/host", get(headers_Host_header_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Referer header
pub fn create_app_headers_Referer_header() -> Router {
    Router::new()
        .route("/headers/referer", get(headers_Referer_header_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header with underscore conversion - explicit
pub fn create_app_headers_Header_with_underscore_conversion___explicit() -> Router {
    Router::new()
        .route(
            "/headers/underscore",
            get(headers_Header_with_underscore_conversion___explicit_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header case insensitivity - access
pub fn create_app_headers_Header_case_insensitivity___access() -> Router {
    Router::new()
        .route("/echo", post(headers_Header_case_insensitivity___access_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: User-Agent header - custom value
pub fn create_app_headers_User_Agent_header___custom_value() -> Router {
    Router::new()
        .route("/items/", get(headers_User_Agent_header___custom_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Invalid UUID format
pub fn create_app_validation_errors_Invalid_UUID_format() -> Router {
    Router::new()
        .route("/items/{item_id}", get(validation_errors_Invalid_UUID_format_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Invalid boolean value
pub fn create_app_validation_errors_Invalid_boolean_value() -> Router {
    Router::new()
        .route("/items/", get(validation_errors_Invalid_boolean_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Missing required query parameter
pub fn create_app_validation_errors_Missing_required_query_parameter() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_Missing_required_query_parameter_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array max_items constraint violation
pub fn create_app_validation_errors_Array_max_items_constraint_violation() -> Router {
    Router::new()
        .route(
            "/items/",
            post(validation_errors_Array_max_items_constraint_violation_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Numeric constraint violation - gt (greater than)
pub fn create_app_validation_errors_Numeric_constraint_violation___gt__greater_than() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_Numeric_constraint_violation___gt__greater_than_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String regex pattern mismatch
pub fn create_app_validation_errors_String_regex_pattern_mismatch() -> Router {
    Router::new()
        .route("/items/", get(validation_errors_String_regex_pattern_mismatch_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Invalid enum value
pub fn create_app_validation_errors_Invalid_enum_value() -> Router {
    Router::new()
        .route(
            "/models/{model_name}",
            get(validation_errors_Invalid_enum_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String min_length constraint violation
pub fn create_app_validation_errors_String_min_length_constraint_violation() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_String_min_length_constraint_violation_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple validation errors
pub fn create_app_validation_errors_Multiple_validation_errors() -> Router {
    Router::new()
        .route("/items/", post(validation_errors_Multiple_validation_errors_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String max_length constraint violation
pub fn create_app_validation_errors_String_max_length_constraint_violation() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_String_max_length_constraint_violation_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Nested object validation error
pub fn create_app_validation_errors_Nested_object_validation_error() -> Router {
    Router::new()
        .route(
            "/items/",
            post(validation_errors_Nested_object_validation_error_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 10_nested_error_path
pub fn create_app_validation_errors_10_nested_error_path() -> Router {
    Router::new()
        .route("/profiles", post(validation_errors_10_nested_error_path_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Invalid datetime format
pub fn create_app_validation_errors_Invalid_datetime_format() -> Router {
    Router::new()
        .route("/items/", post(validation_errors_Invalid_datetime_format_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array item validation error
pub fn create_app_validation_errors_Array_item_validation_error() -> Router {
    Router::new()
        .route("/items/", post(validation_errors_Array_item_validation_error_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Missing required body field
pub fn create_app_validation_errors_Missing_required_body_field() -> Router {
    Router::new()
        .route("/items/", post(validation_errors_Missing_required_body_field_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Body field type error - string for float
pub fn create_app_validation_errors_Body_field_type_error___string_for_float() -> Router {
    Router::new()
        .route(
            "/items/",
            post(validation_errors_Body_field_type_error___string_for_float_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Malformed JSON body
pub fn create_app_validation_errors_Malformed_JSON_body() -> Router {
    Router::new()
        .route("/items/", post(validation_errors_Malformed_JSON_body_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Query param type error - string provided for int
pub fn create_app_validation_errors_Query_param_type_error___string_provided_for_int() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_Query_param_type_error___string_provided_for_int_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Header validation error
pub fn create_app_validation_errors_Header_validation_error() -> Router {
    Router::new()
        .route("/items/", get(validation_errors_Header_validation_error_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 09_multiple_validation_errors
pub fn create_app_validation_errors_09_multiple_validation_errors() -> Router {
    Router::new()
        .route("/users", post(validation_errors_09_multiple_validation_errors_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Numeric constraint violation - le (less than or equal)
pub fn create_app_validation_errors_Numeric_constraint_violation___le__less_than_or_equal() -> Router {
    Router::new()
        .route(
            "/items/",
            get(validation_errors_Numeric_constraint_violation___le__less_than_or_equal_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array min_items constraint violation
pub fn create_app_validation_errors_Array_min_items_constraint_violation() -> Router {
    Router::new()
        .route(
            "/items/",
            post(validation_errors_Array_min_items_constraint_violation_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple values for same field name
pub fn create_app_multipart_Multiple_values_for_same_field_name() -> Router {
    Router::new()
        .route("/", post(multipart_Multiple_values_for_same_field_name_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 19_file_mime_spoofing_png_as_jpeg
pub fn create_app_multipart_19_file_mime_spoofing_png_as_jpeg() -> Router {
    Router::new()
        .route("/upload", post(multipart_19_file_mime_spoofing_png_as_jpeg_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_file_mime_spoofing_jpeg_as_png
pub fn create_app_multipart_20_file_mime_spoofing_jpeg_as_png() -> Router {
    Router::new()
        .route("/upload", post(multipart_20_file_mime_spoofing_jpeg_as_png_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 21_file_pdf_magic_number_success
pub fn create_app_multipart_21_file_pdf_magic_number_success() -> Router {
    Router::new()
        .route("/upload", post(multipart_21_file_pdf_magic_number_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Content-Type validation - invalid type
pub fn create_app_multipart_Content_Type_validation___invalid_type() -> Router {
    Router::new()
        .route(
            "/files/images-only",
            post(multipart_Content_Type_validation___invalid_type_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PDF file upload
pub fn create_app_multipart_PDF_file_upload() -> Router {
    Router::new()
        .route("/files/document", post(multipart_PDF_file_upload_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: File list upload (array of files)
pub fn create_app_multipart_File_list_upload__array_of_files() -> Router {
    Router::new()
        .route("/files/list", post(multipart_File_list_upload__array_of_files_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional file upload - provided
pub fn create_app_multipart_Optional_file_upload___provided() -> Router {
    Router::new()
        .route(
            "/files/optional",
            post(multipart_Optional_file_upload___provided_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: File size validation - too large
pub fn create_app_multipart_File_size_validation___too_large() -> Router {
    Router::new()
        .route(
            "/files/validated",
            post(multipart_File_size_validation___too_large_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Mixed files and form data
pub fn create_app_multipart_Mixed_files_and_form_data() -> Router {
    Router::new()
        .route("/", post(multipart_Mixed_files_and_form_data_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Simple file upload
pub fn create_app_multipart_Simple_file_upload() -> Router {
    Router::new()
        .route("/", post(multipart_Simple_file_upload_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Empty file upload
pub fn create_app_multipart_Empty_file_upload() -> Router {
    Router::new()
        .route("/files/upload", post(multipart_Empty_file_upload_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional file upload - missing
pub fn create_app_multipart_Optional_file_upload___missing() -> Router {
    Router::new()
        .route(
            "/files/optional",
            post(multipart_Optional_file_upload___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: File upload without filename
pub fn create_app_multipart_File_upload_without_filename() -> Router {
    Router::new()
        .route("/", post(multipart_File_upload_without_filename_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 18_file_magic_number_jpeg_success
pub fn create_app_multipart_18_file_magic_number_jpeg_success() -> Router {
    Router::new()
        .route("/upload", post(multipart_18_file_magic_number_jpeg_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 22_file_empty_buffer
pub fn create_app_multipart_22_file_empty_buffer() -> Router {
    Router::new()
        .route("/upload", post(multipart_22_file_empty_buffer_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 17_file_magic_number_png_success
pub fn create_app_multipart_17_file_magic_number_png_success() -> Router {
    Router::new()
        .route("/upload", post(multipart_17_file_magic_number_png_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Form data without files
pub fn create_app_multipart_Form_data_without_files() -> Router {
    Router::new()
        .route("/", post(multipart_Form_data_without_files_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple file uploads
pub fn create_app_multipart_Multiple_file_uploads() -> Router {
    Router::new()
        .route("/", post(multipart_Multiple_file_uploads_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: File upload with custom headers
pub fn create_app_multipart_File_upload_with_custom_headers() -> Router {
    Router::new()
        .route("/", post(multipart_File_upload_with_custom_headers_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required file upload - missing
pub fn create_app_multipart_Required_file_upload___missing() -> Router {
    Router::new()
        .route(
            "/files/required",
            post(multipart_Required_file_upload___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Image file upload
pub fn create_app_multipart_Image_file_upload() -> Router {
    Router::new()
        .route("/files/image", post(multipart_Image_file_upload_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: UUID field - invalid format
pub fn create_app_json_bodies_UUID_field___invalid_format() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_UUID_field___invalid_format_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 44_const_validation_failure
pub fn create_app_json_bodies_44_const_validation_failure() -> Router {
    Router::new()
        .route("/api/v1/data", post(json_bodies_44_const_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean field - success
pub fn create_app_json_bodies_Boolean_field___success() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Boolean_field___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Numeric le validation - success
pub fn create_app_json_bodies_Numeric_le_validation___success() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_Numeric_le_validation___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Deeply nested objects
pub fn create_app_json_bodies_Deeply_nested_objects() -> Router {
    Router::new()
        .route("/items/nested", post(json_bodies_Deeply_nested_objects_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional fields - omitted
pub fn create_app_json_bodies_Optional_fields___omitted() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Optional_fields___omitted_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: UUID field - success
pub fn create_app_json_bodies_UUID_field___success() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_UUID_field___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Date field - success
pub fn create_app_json_bodies_Date_field___success() -> Router {
    Router::new()
        .route("/events/", post(json_bodies_Date_field___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 47_maxproperties_validation_failure
pub fn create_app_json_bodies_47_maxproperties_validation_failure() -> Router {
    Router::new()
        .route("/config", post(json_bodies_47_maxproperties_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 46_minproperties_validation_failure
pub fn create_app_json_bodies_46_minproperties_validation_failure() -> Router {
    Router::new()
        .route("/config", post(json_bodies_46_minproperties_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String min_length validation - fail
pub fn create_app_json_bodies_String_min_length_validation___fail() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_String_min_length_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Field type validation - invalid type
pub fn create_app_json_bodies_Field_type_validation___invalid_type() -> Router {
    Router::new()
        .route(
            "/items/",
            post(json_bodies_Field_type_validation___invalid_type_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 36_oneof_schema_multiple_match_failure
pub fn create_app_json_bodies_36_oneof_schema_multiple_match_failure() -> Router {
    Router::new()
        .route(
            "/payment",
            post(json_bodies_36_oneof_schema_multiple_match_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Nested object - success
pub fn create_app_json_bodies_Nested_object___success() -> Router {
    Router::new()
        .route("/items/nested", post(json_bodies_Nested_object___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 41_not_schema_success
pub fn create_app_json_bodies_41_not_schema_success() -> Router {
    Router::new()
        .route("/users", post(json_bodies_41_not_schema_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String max_length validation - fail
pub fn create_app_json_bodies_String_max_length_validation___fail() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_String_max_length_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 50_deep_nesting_4_levels
pub fn create_app_json_bodies_50_deep_nesting_4_levels() -> Router {
    Router::new()
        .route("/data", post(json_bodies_50_deep_nesting_4_levels_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 48_dependencies_validation_success
pub fn create_app_json_bodies_48_dependencies_validation_success() -> Router {
    Router::new()
        .route("/billing", post(json_bodies_48_dependencies_validation_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PATCH partial update
pub fn create_app_json_bodies_PATCH_partial_update() -> Router {
    Router::new()
        .route("/items/{id}", patch(json_bodies_PATCH_partial_update_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 30_nested_object_missing_field
pub fn create_app_json_bodies_30_nested_object_missing_field() -> Router {
    Router::new()
        .route("/users", post(json_bodies_30_nested_object_missing_field_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Datetime field - success
pub fn create_app_json_bodies_Datetime_field___success() -> Router {
    Router::new()
        .route("/events/", post(json_bodies_Datetime_field___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String pattern validation - success
pub fn create_app_json_bodies_String_pattern_validation___success() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_String_pattern_validation___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Extra fields ignored (no additionalProperties)
pub fn create_app_json_bodies_Extra_fields_ignored__no_additionalProperties() -> Router {
    Router::new()
        .route(
            "/items/",
            post(json_bodies_Extra_fields_ignored__no_additionalProperties_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 40_anyof_schema_failure
pub fn create_app_json_bodies_40_anyof_schema_failure() -> Router {
    Router::new()
        .route("/contact", post(json_bodies_40_anyof_schema_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 39_anyof_schema_multiple_match_success
pub fn create_app_json_bodies_39_anyof_schema_multiple_match_success() -> Router {
    Router::new()
        .route(
            "/contact",
            post(json_bodies_39_anyof_schema_multiple_match_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array of primitive values
pub fn create_app_json_bodies_Array_of_primitive_values() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Array_of_primitive_values_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Numeric ge validation - fail
pub fn create_app_json_bodies_Numeric_ge_validation___fail() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_Numeric_ge_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 37_oneof_schema_no_match_failure
pub fn create_app_json_bodies_37_oneof_schema_no_match_failure() -> Router {
    Router::new()
        .route("/payment", post(json_bodies_37_oneof_schema_no_match_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Empty array validation - fail
pub fn create_app_json_bodies_Empty_array_validation___fail() -> Router {
    Router::new()
        .route(
            "/items/list-validated",
            post(json_bodies_Empty_array_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 38_anyof_schema_success
pub fn create_app_json_bodies_38_anyof_schema_success() -> Router {
    Router::new()
        .route("/contact", post(json_bodies_38_anyof_schema_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Empty JSON object
pub fn create_app_json_bodies_Empty_JSON_object() -> Router {
    Router::new()
        .route("/items/optional-all", post(json_bodies_Empty_JSON_object_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String pattern validation - fail
pub fn create_app_json_bodies_String_pattern_validation___fail() -> Router {
    Router::new()
        .route(
            "/items/validated",
            post(json_bodies_String_pattern_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 49_dependencies_validation_failure
pub fn create_app_json_bodies_49_dependencies_validation_failure() -> Router {
    Router::new()
        .route("/billing", post(json_bodies_49_dependencies_validation_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Simple JSON object - success
pub fn create_app_json_bodies_Simple_JSON_object___success() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Simple_JSON_object___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required field missing - validation error
pub fn create_app_json_bodies_Required_field_missing___validation_error() -> Router {
    Router::new()
        .route(
            "/items/",
            post(json_bodies_Required_field_missing___validation_error_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 35_oneof_schema_success
pub fn create_app_json_bodies_35_oneof_schema_success() -> Router {
    Router::new()
        .route("/payment", post(json_bodies_35_oneof_schema_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum field - invalid value
pub fn create_app_json_bodies_Enum_field___invalid_value() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Enum_field___invalid_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum field - success
pub fn create_app_json_bodies_Enum_field___success() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Enum_field___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 33_allof_schema_composition
pub fn create_app_json_bodies_33_allof_schema_composition() -> Router {
    Router::new()
        .route("/items", post(json_bodies_33_allof_schema_composition_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 45_minproperties_validation_success
pub fn create_app_json_bodies_45_minproperties_validation_success() -> Router {
    Router::new()
        .route("/config", post(json_bodies_45_minproperties_validation_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Body with query parameters
pub fn create_app_json_bodies_Body_with_query_parameters() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Body_with_query_parameters_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 42_not_schema_failure
pub fn create_app_json_bodies_42_not_schema_failure() -> Router {
    Router::new()
        .route("/users", post(json_bodies_42_not_schema_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 43_const_validation_success
pub fn create_app_json_bodies_43_const_validation_success() -> Router {
    Router::new()
        .route("/api/v1/data", post(json_bodies_43_const_validation_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 32_schema_ref_definitions
pub fn create_app_json_bodies_32_schema_ref_definitions() -> Router {
    Router::new()
        .route("/products", post(json_bodies_32_schema_ref_definitions_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 29_nested_object_validation_success
pub fn create_app_json_bodies_29_nested_object_validation_success() -> Router {
    Router::new()
        .route("/users", post(json_bodies_29_nested_object_validation_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 34_additional_properties_false
pub fn create_app_json_bodies_34_additional_properties_false() -> Router {
    Router::new()
        .route("/users", post(json_bodies_34_additional_properties_false_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Null value for optional field
pub fn create_app_json_bodies_Null_value_for_optional_field() -> Router {
    Router::new()
        .route("/items/", post(json_bodies_Null_value_for_optional_field_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 31_nullable_property_null_value
pub fn create_app_json_bodies_31_nullable_property_null_value() -> Router {
    Router::new()
        .route("/users", post(json_bodies_31_nullable_property_null_value_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array of objects - success
pub fn create_app_json_bodies_Array_of_objects___success() -> Router {
    Router::new()
        .route("/items/list", post(json_bodies_Array_of_objects___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 415 Unsupported Media Type
pub fn create_app_content_types_415_Unsupported_Media_Type() -> Router {
    Router::new()
        .route("/items/", post(content_types_415_Unsupported_Media_Type_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: XML response - application/xml
pub fn create_app_content_types_XML_response___application_xml() -> Router {
    Router::new()
        .route("/xml", get(content_types_XML_response___application_xml_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 14_content_type_case_insensitive
pub fn create_app_content_types_14_content_type_case_insensitive() -> Router {
    Router::new()
        .route("/data", post(content_types_14_content_type_case_insensitive_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: JSON with UTF-8 charset
pub fn create_app_content_types_JSON_with_UTF_8_charset() -> Router {
    Router::new()
        .route("/items/unicode", get(content_types_JSON_with_UTF_8_charset_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 16_text_plain_not_accepted
pub fn create_app_content_types_16_text_plain_not_accepted() -> Router {
    Router::new()
        .route("/data", post(content_types_16_text_plain_not_accepted_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PDF response - application/pdf
pub fn create_app_content_types_PDF_response___application_pdf() -> Router {
    Router::new()
        .route(
            "/download/document.pdf",
            get(content_types_PDF_response___application_pdf_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_content_length_mismatch
pub fn create_app_content_types_20_content_length_mismatch() -> Router {
    Router::new()
        .route("/data", post(content_types_20_content_length_mismatch_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 17_vendor_json_accepted
pub fn create_app_content_types_17_vendor_json_accepted() -> Router {
    Router::new()
        .route("/api/v1/resource", post(content_types_17_vendor_json_accepted_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 13_json_with_charset_utf16
pub fn create_app_content_types_13_json_with_charset_utf16() -> Router {
    Router::new()
        .route("/data", post(content_types_13_json_with_charset_utf16_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: JSON response - application/json
pub fn create_app_content_types_JSON_response___application_json() -> Router {
    Router::new()
        .route(
            "/items/json",
            get(content_types_JSON_response___application_json_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 15_multipart_boundary_required
pub fn create_app_content_types_15_multipart_boundary_required() -> Router {
    Router::new()
        .route("/upload", post(content_types_15_multipart_boundary_required_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Content negotiation - Accept header
pub fn create_app_content_types_Content_negotiation___Accept_header() -> Router {
    Router::new()
        .route(
            "/accept-test/{id}",
            get(content_types_Content_negotiation___Accept_header_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: HTML response - text/html
pub fn create_app_content_types_HTML_response___text_html() -> Router {
    Router::new()
        .route("/html", get(content_types_HTML_response___text_html_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: JPEG image response - image/jpeg
pub fn create_app_content_types_JPEG_image_response___image_jpeg() -> Router {
    Router::new()
        .route(
            "/images/photo.jpg",
            get(content_types_JPEG_image_response___image_jpeg_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 19_missing_content_type_default_json
pub fn create_app_content_types_19_missing_content_type_default_json() -> Router {
    Router::new()
        .route(
            "/data",
            post(content_types_19_missing_content_type_default_json_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PNG image response - image/png
pub fn create_app_content_types_PNG_image_response___image_png() -> Router {
    Router::new()
        .route(
            "/images/logo.png",
            get(content_types_PNG_image_response___image_png_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Plain text response - text/plain
pub fn create_app_content_types_Plain_text_response___text_plain() -> Router {
    Router::new()
        .route("/text", get(content_types_Plain_text_response___text_plain_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 18_content_type_with_multiple_params
pub fn create_app_content_types_18_content_type_with_multiple_params() -> Router {
    Router::new()
        .route(
            "/data",
            post(content_types_18_content_type_with_multiple_params_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: CSV response - text/csv
pub fn create_app_content_types_CSV_response___text_csv() -> Router {
    Router::new()
        .route("/export/data.csv", get(content_types_CSV_response___text_csv_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Binary response - application/octet-stream
pub fn create_app_content_types_Binary_response___application_octet_stream() -> Router {
    Router::new()
        .route(
            "/download/file.bin",
            get(content_types_Binary_response___application_octet_stream_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean path parameter - True
pub fn create_app_path_params_Boolean_path_parameter___True() -> Router {
    Router::new()
        .route(
            "/path/bool/{item_id}",
            get(path_params_Boolean_path_parameter___True_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 29_decimal_path_param_success
pub fn create_app_path_params_29_decimal_path_param_success() -> Router {
    Router::new()
        .route(
            "/prices/{amount}",
            get(path_params_29_decimal_path_param_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with combined lt and gt constraints - success
pub fn create_app_path_params_Integer_path_parameter_with_combined_lt_and_gt_constraints___success() -> Router {
    Router::new()
        .route(
            "/path/param-lt-gt/{item_id}",
            get(path_params_Integer_path_parameter_with_combined_lt_and_gt_constraints___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 33_string_pattern_path_success
pub fn create_app_path_params_33_string_pattern_path_success() -> Router {
    Router::new()
        .route(
            "/repos/{owner}/{repo}",
            get(path_params_33_string_pattern_path_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 31_string_minlength_path_failure
pub fn create_app_path_params_31_string_minlength_path_failure() -> Router {
    Router::new()
        .route(
            "/users/{username}",
            get(path_params_31_string_minlength_path_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 35_negative_integer_path_param
pub fn create_app_path_params_35_negative_integer_path_param() -> Router {
    Router::new()
        .route(
            "/offset/{value}",
            get(path_params_35_negative_integer_path_param_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum path parameter - invalid value
pub fn create_app_path_params_Enum_path_parameter___invalid_value() -> Router {
    Router::new()
        .route(
            "/models/{model_name}",
            get(path_params_Enum_path_parameter___invalid_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 27_datetime_format_path_param_success
pub fn create_app_path_params_27_datetime_format_path_param_success() -> Router {
    Router::new()
        .route(
            "/bookings/{timestamp}",
            get(path_params_27_datetime_format_path_param_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 25_date_format_invalid_failure
pub fn create_app_path_params_25_date_format_invalid_failure() -> Router {
    Router::new()
        .route(
            "/events/{date}",
            get(path_params_25_date_format_invalid_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with lt constraint - success
pub fn create_app_path_params_Integer_path_parameter_with_lt_constraint___success() -> Router {
    Router::new()
        .route(
            "/path/param-lt/{item_id}",
            get(path_params_Integer_path_parameter_with_lt_constraint___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with gt constraint - success
pub fn create_app_path_params_Integer_path_parameter_with_gt_constraint___success() -> Router {
    Router::new()
        .route(
            "/path/param-gt/{item_id}",
            get(path_params_Integer_path_parameter_with_gt_constraint___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 28_duration_format_path_param_success
pub fn create_app_path_params_28_duration_format_path_param_success() -> Router {
    Router::new()
        .route(
            "/delays/{duration}",
            get(path_params_28_duration_format_path_param_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Path parameter type syntax with override
pub fn create_app_path_params_Path_parameter_type_syntax_with_override() -> Router {
    Router::new()
        .route(
            "/type-syntax/items-count/{count}",
            get(path_params_Path_parameter_type_syntax_with_override_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_uuid_v3_path_param_success
pub fn create_app_path_params_20_uuid_v3_path_param_success() -> Router {
    Router::new()
        .route("/items/{id}", get(path_params_20_uuid_v3_path_param_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter - invalid string
pub fn create_app_path_params_Integer_path_parameter___invalid_string() -> Router {
    Router::new()
        .route(
            "/path/int/{item_id}",
            get(path_params_Integer_path_parameter___invalid_string_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 30_string_minlength_path_success
pub fn create_app_path_params_30_string_minlength_path_success() -> Router {
    Router::new()
        .route(
            "/users/{username}",
            get(path_params_30_string_minlength_path_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with le constraint - success
pub fn create_app_path_params_Integer_path_parameter_with_le_constraint___success() -> Router {
    Router::new()
        .route(
            "/path/param-le/{item_id}",
            get(path_params_Integer_path_parameter_with_le_constraint___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Path parameter type syntax - invalid UUID
pub fn create_app_path_params_Path_parameter_type_syntax___invalid_UUID() -> Router {
    Router::new()
        .route(
            "/type-syntax/items/{id}",
            get(path_params_Path_parameter_type_syntax___invalid_UUID_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Path type parameter - file path
pub fn create_app_path_params_Path_type_parameter___file_path() -> Router {
    Router::new()
        .route(
            "/files/{*file_path}",
            get(path_params_Path_type_parameter___file_path_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Path parameter with type syntax - UUID
pub fn create_app_path_params_Path_parameter_with_type_syntax___UUID() -> Router {
    Router::new()
        .route(
            "/type-syntax/items/{id}",
            get(path_params_Path_parameter_with_type_syntax___UUID_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 32_string_maxlength_path_failure
pub fn create_app_path_params_32_string_maxlength_path_failure() -> Router {
    Router::new()
        .route(
            "/users/{username}",
            get(path_params_32_string_maxlength_path_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter - success
pub fn create_app_path_params_Integer_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/path/int/{item_id}",
            get(path_params_Integer_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 34_string_pattern_path_failure
pub fn create_app_path_params_34_string_pattern_path_failure() -> Router {
    Router::new()
        .route(
            "/repos/{owner}",
            get(path_params_34_string_pattern_path_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 21_uuid_v5_path_param_success
pub fn create_app_path_params_21_uuid_v5_path_param_success() -> Router {
    Router::new()
        .route("/items/{id}", get(path_params_21_uuid_v5_path_param_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String path parameter with max_length - failure
pub fn create_app_path_params_String_path_parameter_with_max_length___failure() -> Router {
    Router::new()
        .route(
            "/path/param-maxlength/{item_id}",
            get(path_params_String_path_parameter_with_max_length___failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String path parameter with min_length - failure
pub fn create_app_path_params_String_path_parameter_with_min_length___failure() -> Router {
    Router::new()
        .route(
            "/path/param-minlength/{item_id}",
            get(path_params_String_path_parameter_with_min_length___failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple path parameters - success
pub fn create_app_path_params_Multiple_path_parameters___success() -> Router {
    Router::new()
        .route(
            "/{version}/{service_id}/{user_id}/{order_id}",
            get(path_params_Multiple_path_parameters___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Date path parameter - success
pub fn create_app_path_params_Date_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/date/{date_param}",
            get(path_params_Date_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with gt constraint - failure
pub fn create_app_path_params_Integer_path_parameter_with_gt_constraint___failure() -> Router {
    Router::new()
        .route(
            "/path/param-gt/{item_id}",
            get(path_params_Integer_path_parameter_with_gt_constraint___failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 24_date_format_path_param_success
pub fn create_app_path_params_24_date_format_path_param_success() -> Router {
    Router::new()
        .route(
            "/events/{date}",
            get(path_params_24_date_format_path_param_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Float path parameter - success
pub fn create_app_path_params_Float_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/path/float/{item_id}",
            get(path_params_Float_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Path parameter with type syntax - integer
pub fn create_app_path_params_Path_parameter_with_type_syntax___integer() -> Router {
    Router::new()
        .route(
            "/type-syntax/users/{user_id}",
            get(path_params_Path_parameter_with_type_syntax___integer_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String path parameter - success
pub fn create_app_path_params_String_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/path/str/{item_id}",
            get(path_params_String_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: UUID path parameter - success
pub fn create_app_path_params_UUID_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/items/{item_id}",
            get(path_params_UUID_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer path parameter with ge constraint - success
pub fn create_app_path_params_Integer_path_parameter_with_ge_constraint___success() -> Router {
    Router::new()
        .route(
            "/path/param-ge/{item_id}",
            get(path_params_Integer_path_parameter_with_ge_constraint___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum path parameter - success
pub fn create_app_path_params_Enum_path_parameter___success() -> Router {
    Router::new()
        .route(
            "/models/{model_name}",
            get(path_params_Enum_path_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean path parameter - numeric 1
pub fn create_app_path_params_Boolean_path_parameter___numeric_1() -> Router {
    Router::new()
        .route(
            "/path/bool/{item_id}",
            get(path_params_Boolean_path_parameter___numeric_1_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 07_cors_preflight_header_not_allowed
pub fn create_app_cors_07_cors_preflight_header_not_allowed() -> Router {
    Router::new()
        .route("/api/data", options(cors_07_cors_preflight_header_not_allowed_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: CORS preflight request
pub fn create_app_cors_CORS_preflight_request() -> Router {
    Router::new()
        .route("/items/", options(cors_CORS_preflight_request_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: CORS with credentials
pub fn create_app_cors_CORS_with_credentials() -> Router {
    Router::new()
        .route("/api/user/profile", get(cors_CORS_with_credentials_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 08_cors_max_age
pub fn create_app_cors_08_cors_max_age() -> Router {
    Router::new()
        .route("/api/data", options(cors_08_cors_max_age_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 10_cors_origin_null
pub fn create_app_cors_10_cors_origin_null() -> Router {
    Router::new()
        .route("/api/data", get(cors_10_cors_origin_null_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: CORS wildcard origin
pub fn create_app_cors_CORS_wildcard_origin() -> Router {
    Router::new()
        .route("/public/data", get(cors_CORS_wildcard_origin_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: CORS request blocked
pub fn create_app_cors_CORS_request_blocked() -> Router {
    Router::new()
        .route("/items/", get(cors_CORS_request_blocked_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Simple CORS request
pub fn create_app_cors_Simple_CORS_request() -> Router {
    Router::new()
        .route("/items/", get(cors_Simple_CORS_request_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 09_cors_expose_headers
pub fn create_app_cors_09_cors_expose_headers() -> Router {
    Router::new()
        .route("/api/data", get(cors_09_cors_expose_headers_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 06_cors_preflight_method_not_allowed
pub fn create_app_cors_06_cors_preflight_method_not_allowed() -> Router {
    Router::new()
        .route("/api/data", options(cors_06_cors_preflight_method_not_allowed_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: OPTIONS - CORS preflight request
pub fn create_app_http_methods_OPTIONS___CORS_preflight_request() -> Router {
    Router::new()
        .route(
            "/items/",
            options(http_methods_OPTIONS___CORS_preflight_request_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: DELETE - Remove resource
pub fn create_app_http_methods_DELETE___Remove_resource() -> Router {
    Router::new()
        .route("/items/{id}", delete(http_methods_DELETE___Remove_resource_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PUT - Create resource if doesn't exist
pub fn create_app_http_methods_PUT___Create_resource_if_doesn_t_exist() -> Router {
    Router::new()
        .route(
            "/items/{id}",
            put(http_methods_PUT___Create_resource_if_doesn_t_exist_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PATCH - Update multiple fields
pub fn create_app_http_methods_PATCH___Update_multiple_fields() -> Router {
    Router::new()
        .route(
            "/items/{id}",
            patch(http_methods_PATCH___Update_multiple_fields_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PUT - Validation error
pub fn create_app_http_methods_PUT___Validation_error() -> Router {
    Router::new()
        .route("/items/{id}", put(http_methods_PUT___Validation_error_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: HEAD - Get metadata without body
pub fn create_app_http_methods_HEAD___Get_metadata_without_body() -> Router {
    Router::new()
        .route(
            "/items/{id}",
            head(http_methods_HEAD___Get_metadata_without_body_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: DELETE - With response body
pub fn create_app_http_methods_DELETE___With_response_body() -> Router {
    Router::new()
        .route("/items/{id}", delete(http_methods_DELETE___With_response_body_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PUT - Missing required field
pub fn create_app_http_methods_PUT___Missing_required_field() -> Router {
    Router::new()
        .route("/items/{id}", put(http_methods_PUT___Missing_required_field_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PATCH - Partial update
pub fn create_app_http_methods_PATCH___Partial_update() -> Router {
    Router::new()
        .route("/items/{id}", patch(http_methods_PATCH___Partial_update_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: DELETE - Resource not found
pub fn create_app_http_methods_DELETE___Resource_not_found() -> Router {
    Router::new()
        .route("/items/{id}", delete(http_methods_DELETE___Resource_not_found_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PUT - Idempotent operation
pub fn create_app_http_methods_PUT___Idempotent_operation() -> Router {
    Router::new()
        .route("/items/{id}", put(http_methods_PUT___Idempotent_operation_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: PUT - Complete resource replacement
pub fn create_app_http_methods_PUT___Complete_resource_replacement() -> Router {
    Router::new()
        .route(
            "/items/{id}",
            put(http_methods_PUT___Complete_resource_replacement_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String validation with regex - success
pub fn create_app_query_params_String_validation_with_regex___success() -> Router {
    Router::new()
        .route(
            "/items/",
            get(query_params_String_validation_with_regex___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 49_integer_gt_constraint_success
pub fn create_app_query_params_49_integer_gt_constraint_success() -> Router {
    Router::new()
        .route("/items", get(query_params_49_integer_gt_constraint_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum query parameter - invalid value
pub fn create_app_query_params_Enum_query_parameter___invalid_value() -> Router {
    Router::new()
        .route(
            "/query/enum",
            get(query_params_Enum_query_parameter___invalid_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 68_array_uniqueitems_success
pub fn create_app_query_params_68_array_uniqueitems_success() -> Router {
    Router::new()
        .route("/items", get(query_params_68_array_uniqueitems_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 47_pattern_validation_email_success
pub fn create_app_query_params_47_pattern_validation_email_success() -> Router {
    Router::new()
        .route(
            "/subscribe",
            get(query_params_47_pattern_validation_email_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required integer query parameter - success
pub fn create_app_query_params_Required_integer_query_parameter___success() -> Router {
    Router::new()
        .route(
            "/query/int",
            get(query_params_Required_integer_query_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required string query parameter - missing
pub fn create_app_query_params_Required_string_query_parameter___missing() -> Router {
    Router::new()
        .route(
            "/query",
            get(query_params_Required_string_query_parameter___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 57_boolean_empty_string_coercion
pub fn create_app_query_params_57_boolean_empty_string_coercion() -> Router {
    Router::new()
        .route("/items", get(query_params_57_boolean_empty_string_coercion_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 52_integer_le_constraint_boundary
pub fn create_app_query_params_52_integer_le_constraint_boundary() -> Router {
    Router::new()
        .route("/items", get(query_params_52_integer_le_constraint_boundary_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: List with default empty array - no values provided
pub fn create_app_query_params_List_with_default_empty_array___no_values_provided() -> Router {
    Router::new()
        .route(
            "/query/list-default",
            get(query_params_List_with_default_empty_array___no_values_provided_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Date query parameter - success
pub fn create_app_query_params_Date_query_parameter___success() -> Router {
    Router::new()
        .route("/query/date", get(query_params_Date_query_parameter___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String query param with max_length constraint - fail
pub fn create_app_query_params_String_query_param_with_max_length_constraint___fail() -> Router {
    Router::new()
        .route(
            "/query/str-max-length",
            get(query_params_String_query_param_with_max_length_constraint___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 45_string_minlength_validation_failure
pub fn create_app_query_params_45_string_minlength_validation_failure() -> Router {
    Router::new()
        .route(
            "/search",
            get(query_params_45_string_minlength_validation_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer with default value - override
pub fn create_app_query_params_Integer_with_default_value___override() -> Router {
    Router::new()
        .route(
            "/query/int/default",
            get(query_params_Integer_with_default_value___override_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 67_multipleof_constraint_failure
pub fn create_app_query_params_67_multipleof_constraint_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_67_multipleof_constraint_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 58_format_email_success
pub fn create_app_query_params_58_format_email_success() -> Router {
    Router::new()
        .route("/subscribe", get(query_params_58_format_email_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer query param with ge constraint - boundary
pub fn create_app_query_params_Integer_query_param_with_ge_constraint___boundary() -> Router {
    Router::new()
        .route(
            "/query/int-ge",
            get(query_params_Integer_query_param_with_ge_constraint___boundary_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer query param with gt constraint - valid
pub fn create_app_query_params_Integer_query_param_with_gt_constraint___valid() -> Router {
    Router::new()
        .route(
            "/query/int-gt",
            get(query_params_Integer_query_param_with_gt_constraint___valid_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required integer query parameter - invalid type
pub fn create_app_query_params_Required_integer_query_parameter___invalid_type() -> Router {
    Router::new()
        .route(
            "/query/int",
            get(query_params_Required_integer_query_parameter___invalid_type_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required integer query parameter - float value
pub fn create_app_query_params_Required_integer_query_parameter___float_value() -> Router {
    Router::new()
        .route(
            "/query/int",
            get(query_params_Required_integer_query_parameter___float_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Query parameter with URL encoded special characters
pub fn create_app_query_params_Query_parameter_with_URL_encoded_special_characters() -> Router {
    Router::new()
        .route(
            "/query/basic",
            get(query_params_Query_parameter_with_URL_encoded_special_characters_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 59_format_email_failure
pub fn create_app_query_params_59_format_email_failure() -> Router {
    Router::new()
        .route("/subscribe", get(query_params_59_format_email_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 43_scientific_notation_float
pub fn create_app_query_params_43_scientific_notation_float() -> Router {
    Router::new()
        .route("/stats", get(query_params_43_scientific_notation_float_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 63_format_uri_success
pub fn create_app_query_params_63_format_uri_success() -> Router {
    Router::new()
        .route("/redirect", get(query_params_63_format_uri_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean query parameter - numeric 1
pub fn create_app_query_params_Boolean_query_parameter___numeric_1() -> Router {
    Router::new()
        .route(
            "/query/bool",
            get(query_params_Boolean_query_parameter___numeric_1_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String query param with min_length constraint - fail
pub fn create_app_query_params_String_query_param_with_min_length_constraint___fail() -> Router {
    Router::new()
        .route(
            "/query/str-min-length",
            get(query_params_String_query_param_with_min_length_constraint___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional string query parameter - provided
pub fn create_app_query_params_Optional_string_query_parameter___provided() -> Router {
    Router::new()
        .route(
            "/query/optional",
            get(query_params_Optional_string_query_parameter___provided_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: List of integers - multiple values
pub fn create_app_query_params_List_of_integers___multiple_values() -> Router {
    Router::new()
        .route(
            "/query/list",
            get(query_params_List_of_integers___multiple_values_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer query param with lt constraint - valid
pub fn create_app_query_params_Integer_query_param_with_lt_constraint___valid() -> Router {
    Router::new()
        .route(
            "/query/int-lt",
            get(query_params_Integer_query_param_with_lt_constraint___valid_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 42_negative_integer_query_param
pub fn create_app_query_params_42_negative_integer_query_param() -> Router {
    Router::new()
        .route(
            "/items/negative",
            get(query_params_42_negative_integer_query_param_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 46_string_maxlength_validation_failure
pub fn create_app_query_params_46_string_maxlength_validation_failure() -> Router {
    Router::new()
        .route(
            "/search",
            get(query_params_46_string_maxlength_validation_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 56_array_maxitems_constraint_failure
pub fn create_app_query_params_56_array_maxitems_constraint_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_56_array_maxitems_constraint_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String query param with regex pattern - fail
pub fn create_app_query_params_String_query_param_with_regex_pattern___fail() -> Router {
    Router::new()
        .route(
            "/query/pattern",
            get(query_params_String_query_param_with_regex_pattern___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 44_string_minlength_validation_success
pub fn create_app_query_params_44_string_minlength_validation_success() -> Router {
    Router::new()
        .route(
            "/search",
            get(query_params_44_string_minlength_validation_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 61_format_ipv4_failure
pub fn create_app_query_params_61_format_ipv4_failure() -> Router {
    Router::new()
        .route("/network", get(query_params_61_format_ipv4_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 48_pattern_validation_email_failure
pub fn create_app_query_params_48_pattern_validation_email_failure() -> Router {
    Router::new()
        .route(
            "/subscribe",
            get(query_params_48_pattern_validation_email_failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required integer query parameter - missing
pub fn create_app_query_params_Required_integer_query_parameter___missing() -> Router {
    Router::new()
        .route(
            "/query/int",
            get(query_params_Required_integer_query_parameter___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Query parameter with special characters - URL encoding
pub fn create_app_query_params_Query_parameter_with_special_characters___URL_encoding() -> Router {
    Router::new()
        .route(
            "/test",
            get(query_params_Query_parameter_with_special_characters___URL_encoding_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: List query parameter - required but missing
pub fn create_app_query_params_List_query_parameter___required_but_missing() -> Router {
    Router::new()
        .route(
            "/query/list",
            get(query_params_List_query_parameter___required_but_missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required string query parameter - success
pub fn create_app_query_params_Required_string_query_parameter___success() -> Router {
    Router::new()
        .route(
            "/query",
            get(query_params_Required_string_query_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 66_multipleof_constraint_success
pub fn create_app_query_params_66_multipleof_constraint_success() -> Router {
    Router::new()
        .route("/items", get(query_params_66_multipleof_constraint_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 53_integer_le_constraint_failure
pub fn create_app_query_params_53_integer_le_constraint_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_53_integer_le_constraint_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple query parameters with different types
pub fn create_app_query_params_Multiple_query_parameters_with_different_types() -> Router {
    Router::new()
        .route(
            "/query/multi-type",
            get(query_params_Multiple_query_parameters_with_different_types_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 71_array_separator_semicolon
pub fn create_app_query_params_71_array_separator_semicolon() -> Router {
    Router::new()
        .route("/items", get(query_params_71_array_separator_semicolon_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 70_array_separator_pipe
pub fn create_app_query_params_70_array_separator_pipe() -> Router {
    Router::new()
        .route("/items", get(query_params_70_array_separator_pipe_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer with default value - not provided
pub fn create_app_query_params_Integer_with_default_value___not_provided() -> Router {
    Router::new()
        .route(
            "/query/int/default",
            get(query_params_Integer_with_default_value___not_provided_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Boolean query parameter - true
pub fn create_app_query_params_Boolean_query_parameter___true() -> Router {
    Router::new()
        .route("/query/bool", get(query_params_Boolean_query_parameter___true_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Integer query param with le constraint - boundary
pub fn create_app_query_params_Integer_query_param_with_le_constraint___boundary() -> Router {
    Router::new()
        .route(
            "/query/int-le",
            get(query_params_Integer_query_param_with_le_constraint___boundary_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Float query param with ge constraint - success
pub fn create_app_query_params_Float_query_param_with_ge_constraint___success() -> Router {
    Router::new()
        .route(
            "/query/float-ge",
            get(query_params_Float_query_param_with_ge_constraint___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 51_integer_ge_constraint_boundary
pub fn create_app_query_params_51_integer_ge_constraint_boundary() -> Router {
    Router::new()
        .route("/items", get(query_params_51_integer_ge_constraint_boundary_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional integer query parameter - missing
pub fn create_app_query_params_Optional_integer_query_parameter___missing() -> Router {
    Router::new()
        .route(
            "/query/int/optional",
            get(query_params_Optional_integer_query_parameter___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 69_array_uniqueitems_failure
pub fn create_app_query_params_69_array_uniqueitems_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_69_array_uniqueitems_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 72_array_separator_space
pub fn create_app_query_params_72_array_separator_space() -> Router {
    Router::new()
        .route("/search", get(query_params_72_array_separator_space_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: String validation with regex - failure
pub fn create_app_query_params_String_validation_with_regex___failure() -> Router {
    Router::new()
        .route(
            "/items/",
            get(query_params_String_validation_with_regex___failure_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 65_format_hostname_success
pub fn create_app_query_params_65_format_hostname_success() -> Router {
    Router::new()
        .route("/dns", get(query_params_65_format_hostname_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Query parameter with URL encoded space
pub fn create_app_query_params_Query_parameter_with_URL_encoded_space() -> Router {
    Router::new()
        .route(
            "/query/basic",
            get(query_params_Query_parameter_with_URL_encoded_space_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: List of strings - multiple values
pub fn create_app_query_params_List_of_strings___multiple_values() -> Router {
    Router::new()
        .route("/items/", get(query_params_List_of_strings___multiple_values_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional query parameter with default value
pub fn create_app_query_params_Optional_query_parameter_with_default_value() -> Router {
    Router::new()
        .route(
            "/query/optional-default",
            get(query_params_Optional_query_parameter_with_default_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 62_format_ipv6_success
pub fn create_app_query_params_62_format_ipv6_success() -> Router {
    Router::new()
        .route("/network/ipv6", get(query_params_62_format_ipv6_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array query parameter - single value
pub fn create_app_query_params_Array_query_parameter___single_value() -> Router {
    Router::new()
        .route(
            "/query/list-default",
            get(query_params_Array_query_parameter___single_value_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional string query parameter - missing
pub fn create_app_query_params_Optional_string_query_parameter___missing() -> Router {
    Router::new()
        .route(
            "/query/optional",
            get(query_params_Optional_string_query_parameter___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Datetime query parameter - success
pub fn create_app_query_params_Datetime_query_parameter___success() -> Router {
    Router::new()
        .route(
            "/query/datetime",
            get(query_params_Datetime_query_parameter___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: UUID query parameter - invalid format
pub fn create_app_query_params_UUID_query_parameter___invalid_format() -> Router {
    Router::new()
        .route(
            "/query/uuid",
            get(query_params_UUID_query_parameter___invalid_format_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Array query parameter - empty array
pub fn create_app_query_params_Array_query_parameter___empty_array() -> Router {
    Router::new()
        .route(
            "/query/list-default",
            get(query_params_Array_query_parameter___empty_array_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Enum query parameter - success
pub fn create_app_query_params_Enum_query_parameter___success() -> Router {
    Router::new()
        .route("/query/enum", get(query_params_Enum_query_parameter___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: UUID query parameter - success
pub fn create_app_query_params_UUID_query_parameter___success() -> Router {
    Router::new()
        .route("/query/uuid", get(query_params_UUID_query_parameter___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 50_integer_gt_constraint_failure
pub fn create_app_query_params_50_integer_gt_constraint_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_50_integer_gt_constraint_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 64_format_uri_failure
pub fn create_app_query_params_64_format_uri_failure() -> Router {
    Router::new()
        .route("/redirect", get(query_params_64_format_uri_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 54_array_minitems_constraint_success
pub fn create_app_query_params_54_array_minitems_constraint_success() -> Router {
    Router::new()
        .route("/items", get(query_params_54_array_minitems_constraint_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 55_array_minitems_constraint_failure
pub fn create_app_query_params_55_array_minitems_constraint_failure() -> Router {
    Router::new()
        .route("/items", get(query_params_55_array_minitems_constraint_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 60_format_ipv4_success
pub fn create_app_query_params_60_format_ipv4_success() -> Router {
    Router::new()
        .route("/network", get(query_params_60_format_ipv4_success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 25_cookie_samesite_lax
pub fn create_app_cookies_25_cookie_samesite_lax() -> Router {
    Router::new()
        .route("/data", get(cookies_25_cookie_samesite_lax_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional cookie parameter - success
pub fn create_app_cookies_Optional_cookie_parameter___success() -> Router {
    Router::new()
        .route("/items/", get(cookies_Optional_cookie_parameter___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Cookie regex pattern validation - fail
pub fn create_app_cookies_Cookie_regex_pattern_validation___fail() -> Router {
    Router::new()
        .route(
            "/cookies/pattern",
            get(cookies_Cookie_regex_pattern_validation___fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response - session cookie (no max_age)
pub fn create_app_cookies_Response___session_cookie__no_max_age() -> Router {
    Router::new()
        .route(
            "/cookies/session",
            post(cookies_Response___session_cookie__no_max_age_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 27_cookie_httponly_flag
pub fn create_app_cookies_27_cookie_httponly_flag() -> Router {
    Router::new()
        .route("/secure", get(cookies_27_cookie_httponly_flag_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with attributes
pub fn create_app_cookies_Response_cookie_with_attributes() -> Router {
    Router::new()
        .route("/cookie/set", get(cookies_Response_cookie_with_attributes_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 24_cookie_samesite_strict
pub fn create_app_cookies_24_cookie_samesite_strict() -> Router {
    Router::new()
        .route("/secure", get(cookies_24_cookie_samesite_strict_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: APIKey cookie authentication - success
pub fn create_app_cookies_APIKey_cookie_authentication___success() -> Router {
    Router::new()
        .route("/users/me", get(cookies_APIKey_cookie_authentication___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Cookie validation - min_length constraint success
pub fn create_app_cookies_Cookie_validation___min_length_constraint_success() -> Router {
    Router::new()
        .route(
            "/cookies/min-length",
            get(cookies_Cookie_validation___min_length_constraint_success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Cookie validation - min_length failure
pub fn create_app_cookies_Cookie_validation___min_length_failure() -> Router {
    Router::new()
        .route("/items/", get(cookies_Cookie_validation___min_length_failure_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Cookie validation - max_length constraint fail
pub fn create_app_cookies_Cookie_validation___max_length_constraint_fail() -> Router {
    Router::new()
        .route(
            "/cookies/validated",
            get(cookies_Cookie_validation___max_length_constraint_fail_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Required cookie - missing
pub fn create_app_cookies_Required_cookie___missing() -> Router {
    Router::new()
        .route("/items/cookies", get(cookies_Required_cookie___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional cookie parameter - missing
pub fn create_app_cookies_Optional_cookie_parameter___missing() -> Router {
    Router::new()
        .route("/items/", get(cookies_Optional_cookie_parameter___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: APIKey cookie authentication - missing
pub fn create_app_cookies_APIKey_cookie_authentication___missing() -> Router {
    Router::new()
        .route(
            "/users/me/auth",
            get(cookies_APIKey_cookie_authentication___missing_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response - multiple cookies
pub fn create_app_cookies_Response___multiple_cookies() -> Router {
    Router::new()
        .route("/cookies/multiple", post(cookies_Response___multiple_cookies_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with SameSite=Lax
pub fn create_app_cookies_Response_cookie_with_SameSite_Lax() -> Router {
    Router::new()
        .route(
            "/cookies/samesite-lax",
            post(cookies_Response_cookie_with_SameSite_Lax_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response - delete cookie
pub fn create_app_cookies_Response___delete_cookie() -> Router {
    Router::new()
        .route("/cookies/delete", post(cookies_Response___delete_cookie_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with path attribute
pub fn create_app_cookies_Response_cookie_with_path_attribute() -> Router {
    Router::new()
        .route(
            "/cookies/set-with-path",
            post(cookies_Response_cookie_with_path_attribute_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Optional APIKey cookie - missing
pub fn create_app_cookies_Optional_APIKey_cookie___missing() -> Router {
    Router::new()
        .route("/users/me", get(cookies_Optional_APIKey_cookie___missing_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with SameSite=Strict
pub fn create_app_cookies_Response_cookie_with_SameSite_Strict() -> Router {
    Router::new()
        .route(
            "/cookies/samesite-strict",
            post(cookies_Response_cookie_with_SameSite_Strict_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with SameSite=None
pub fn create_app_cookies_Response_cookie_with_SameSite_None() -> Router {
    Router::new()
        .route(
            "/cookies/samesite-none",
            post(cookies_Response_cookie_with_SameSite_None_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Cookie regex pattern validation - success
pub fn create_app_cookies_Cookie_regex_pattern_validation___success() -> Router {
    Router::new()
        .route(
            "/cookies/pattern",
            get(cookies_Cookie_regex_pattern_validation___success_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response set cookie - basic
pub fn create_app_cookies_Response_set_cookie___basic() -> Router {
    Router::new()
        .route("/cookie/", post(cookies_Response_set_cookie___basic_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Multiple cookies - success
pub fn create_app_cookies_Multiple_cookies___success() -> Router {
    Router::new()
        .route("/items/", get(cookies_Multiple_cookies___success_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 26_cookie_secure_flag
pub fn create_app_cookies_26_cookie_secure_flag() -> Router {
    Router::new()
        .route("/secure", get(cookies_26_cookie_secure_flag_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Response cookie with domain attribute
pub fn create_app_cookies_Response_cookie_with_domain_attribute() -> Router {
    Router::new()
        .route(
            "/cookies/set-with-domain",
            post(cookies_Response_cookie_with_domain_attribute_handler),
        )
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 19_emoji_in_strings
pub fn create_app_edge_cases_19_emoji_in_strings() -> Router {
    Router::new()
        .route("/messages", post(edge_cases_19_emoji_in_strings_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 12_percent_encoded_special_chars
pub fn create_app_edge_cases_12_percent_encoded_special_chars() -> Router {
    Router::new()
        .route("/search", get(edge_cases_12_percent_encoded_special_chars_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Special string values and escaping
pub fn create_app_edge_cases_Special_string_values_and_escaping() -> Router {
    Router::new()
        .route("/strings/", post(edge_cases_Special_string_values_and_escaping_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 15_float_precision_preservation
pub fn create_app_edge_cases_15_float_precision_preservation() -> Router {
    Router::new()
        .route("/calculate", post(edge_cases_15_float_precision_preservation_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 13_empty_string_query_param_preserved
pub fn create_app_edge_cases_13_empty_string_query_param_preserved() -> Router {
    Router::new()
        .route("/items", get(edge_cases_13_empty_string_query_param_preserved_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 24_array_with_holes
pub fn create_app_edge_cases_24_array_with_holes() -> Router {
    Router::new()
        .route("/items", post(edge_cases_24_array_with_holes_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 21_scientific_notation_number
pub fn create_app_edge_cases_21_scientific_notation_number() -> Router {
    Router::new()
        .route("/calculate", post(edge_cases_21_scientific_notation_number_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Float precision and rounding
pub fn create_app_edge_cases_Float_precision_and_rounding() -> Router {
    Router::new()
        .route("/calculations/", post(edge_cases_Float_precision_and_rounding_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Unicode and emoji handling
pub fn create_app_edge_cases_Unicode_and_emoji_handling() -> Router {
    Router::new()
        .route("/items/", post(edge_cases_Unicode_and_emoji_handling_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 17_extremely_long_string
pub fn create_app_edge_cases_17_extremely_long_string() -> Router {
    Router::new()
        .route("/text", post(edge_cases_17_extremely_long_string_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 11_utf8_query_parameter
pub fn create_app_edge_cases_11_utf8_query_parameter() -> Router {
    Router::new()
        .route("/search", get(edge_cases_11_utf8_query_parameter_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 18_unicode_normalization
pub fn create_app_edge_cases_18_unicode_normalization() -> Router {
    Router::new()
        .route("/users", post(edge_cases_18_unicode_normalization_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 20_null_byte_in_string
pub fn create_app_edge_cases_20_null_byte_in_string() -> Router {
    Router::new()
        .route("/files", post(edge_cases_20_null_byte_in_string_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 23_deeply_nested_json_limit
pub fn create_app_edge_cases_23_deeply_nested_json_limit() -> Router {
    Router::new()
        .route("/data", post(edge_cases_23_deeply_nested_json_limit_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 14_large_integer_boundary
pub fn create_app_edge_cases_14_large_integer_boundary() -> Router {
    Router::new()
        .route("/items", get(edge_cases_14_large_integer_boundary_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 22_leading_zeros_integer
pub fn create_app_edge_cases_22_leading_zeros_integer() -> Router {
    Router::new()
        .route("/data", get(edge_cases_22_leading_zeros_integer_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Large integer boundary values
pub fn create_app_edge_cases_Large_integer_boundary_values() -> Router {
    Router::new()
        .route("/numbers/", post(edge_cases_Large_integer_boundary_values_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Deeply nested structure (10+ levels)
pub fn create_app_edge_cases_Deeply_nested_structure__10__levels() -> Router {
    Router::new()
        .route("/nested/", post(edge_cases_Deeply_nested_structure__10__levels_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: Empty and null value handling
pub fn create_app_edge_cases_Empty_and_null_value_handling() -> Router {
    Router::new()
        .route("/nulls/", post(edge_cases_Empty_and_null_value_handling_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

/// App for fixture: 16_negative_zero_handling
pub fn create_app_edge_cases_16_negative_zero_handling() -> Router {
    Router::new()
        .route("/data", post(edge_cases_16_negative_zero_handling_handler))
        .layer(middleware::from_fn(
            spikard_http::middleware::validate_content_type_middleware,
        ))
}

// Handler functions
async fn url_encoded_Simple_form_submission___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"username\":\"johndoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_15_special_characters_field_names_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"contact.email\":{\"format\":\"email\",\"type\":\"string\"},\"user-name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"contact.email\":\"john@example.com\",\"user-name\":\"JohnDoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Pattern_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"pattern\":\"^[a-z0-9_]+$\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-z0-9_]+$\"},\"input\":\"john doe\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should match pattern '^[a-z0-9_]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_22_additional_properties_strict_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"theme\":{\"enum\":[\"light\",\"dark\"],\"type\":\"string\"}},\"required\":[\"theme\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"property\":\"unknown_field\"},\"loc\":[\"body\",\"unknown_field\"],\"msg\":\"Additional properties are not allowed\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_17_pattern_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"account_id\":{\"pattern\":\"^ACC-[0-9]{6}$\",\"type\":\"string\"}},\"required\":[\"account_id\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^ACC-[0-9]{6}$\",\"value\":\"INVALID123\"},\"loc\":[\"body\",\"account_id\"],\"msg\":\"String does not match pattern '^ACC-[0-9]{6}$'\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_20_format_email_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"email\",\"value\":\"not-an-email\"},\"loc\":[\"body\",\"email\"],\"msg\":\"Invalid email format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Multiple_values_for_same_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"tags\":[\"python\",\"fastapi\",\"web\"]}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Required_field_missing___validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"username\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_13_array_field_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":1,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"tags\":[\"python\",\"rust\",\"typescript\"]}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Numeric_field_type_conversion_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"age\":{\"type\":\"integer\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"age\":30,\"username\":\"johndoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Special_characters_encoding_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"description\":\"Test & Development\",\"name\":\"John Doe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Boolean_field_conversion_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"subscribe\":{\"type\":\"boolean\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"subscribe\":true,\"username\":\"johndoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Empty_string_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"description\":\"\",\"username\":\"johndoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_OAuth2_password_grant_flow_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"grant_type\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\",\"grant_type\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"access_token\":\"johndoe\",\"token_type\":\"bearer\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_19_array_minitems_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":2,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":1,\"min_items\":2},\"loc\":[\"body\",\"tags\"],\"msg\":\"Array must contain at least 2 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_Optional_field_missing___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":[\"string\",\"null\"]},\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"email\":null,\"username\":\"johndoe\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_14_nested_object_bracket_notation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"user\":{\"properties\":{\"age\":{\"minimum\":0,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"user\":{\"age\":30,\"email\":\"john@example.com\",\"name\":\"John Doe\"}}")
                    .unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_String_max_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"maxLength\":20,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_is_a_very_long_username_that_exceeds_limit\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_18_integer_minimum_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"quantity\":{\"minimum\":1,\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_value\":0,\"minimum\":1},\"loc\":[\"body\",\"quantity\"],\"msg\":\"Value must be at least 1\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_21_integer_type_coercion_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"price\":{\"type\":\"integer\"}},\"required\":[\"price\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"value\":\"not-a-number\"},\"loc\":[\"body\",\"price\"],\"msg\":\"Value is not a valid integer\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_16_minlength_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3,\"value\":\"ab\"},\"loc\":[\"body\",\"username\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn url_encoded_String_min_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"username\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_408_Request_Timeout_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Request timeout\"}").unwrap();
    (axum::http::StatusCode::from_u16(408).unwrap(), Json(expected_body))
}

async fn status_codes_404_Not_Found___Resource_not_found_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Item not found\"}").unwrap();
    (axum::http::StatusCode::from_u16(404).unwrap(), Json(expected_body))
}

async fn status_codes_503_Service_Unavailable___Server_overload_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Service temporarily unavailable\"}").unwrap();
    (axum::http::StatusCode::from_u16(503).unwrap(), Json(expected_body))
}

async fn status_codes_422_Unprocessable_Entity___Validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"price\",\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"name\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_302_Found___Temporary_redirect_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("null").unwrap();
    (axum::http::StatusCode::from_u16(302).unwrap(), Json(expected_body))
}

async fn status_codes_304_Not_Modified___Cached_content_valid_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("null").unwrap();
    (axum::http::StatusCode::from_u16(304).unwrap(), Json(expected_body))
}

async fn status_codes_400_Bad_Request___Invalid_request_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Invalid request format\"}").unwrap();
    (axum::http::StatusCode::from_u16(400).unwrap(), Json(expected_body))
}

async fn status_codes_22_501_not_implemented_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str(
        "{\"error\":\"Not Implemented\",\"message\":\"The TRACE method is not supported by this server\"}",
    )
    .unwrap();
    (axum::http::StatusCode::from_u16(501).unwrap(), Json(expected_body))
}

async fn status_codes_204_No_Content___Success_with_no_body_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 204;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_301_Moved_Permanently___Permanent_redirect_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("null").unwrap();
    (axum::http::StatusCode::from_u16(301).unwrap(), Json(expected_body))
}

async fn status_codes_201_Created___Resource_created_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"id\":1,\"name\":\"New Item\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_202_Accepted___Request_accepted_for_processing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"task\":{\"type\":\"string\"}},\"required\":[\"task\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"message\":\"Task accepted for processing\",\"task_id\":\"abc123\"}").unwrap();
            (axum::http::StatusCode::from_u16(202).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_307_Temporary_Redirect___Method_preserved_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    (axum::http::StatusCode::from_u16(307).unwrap(), Json(expected_body))
}

async fn status_codes_500_Internal_Server_Error___Server_error_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Internal server error\",\"status\":500,\"title\":\"Internal Server Error\",\"type\":\"https://spikard.dev/errors/internal-server-error\"}").unwrap();
    (axum::http::StatusCode::from_u16(500).unwrap(), Json(expected_body))
}

async fn status_codes_20_414_uri_too_long_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str(
        "{\"error\":\"URI Too Long\",\"message\":\"Request URI exceeds maximum allowed length of 2048 characters\"}",
    )
    .unwrap();
    (axum::http::StatusCode::from_u16(414).unwrap(), Json(expected_body))
}

async fn status_codes_401_Unauthorized___Missing_authentication_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Not authenticated\"}").unwrap();
    (axum::http::StatusCode::from_u16(401).unwrap(), Json(expected_body))
}

async fn status_codes_23_503_service_unavailable_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"error\":\"Service Unavailable\",\"message\":\"The service is temporarily unavailable. Please try again later.\"}").unwrap();
    (axum::http::StatusCode::from_u16(503).unwrap(), Json(expected_body))
}

async fn status_codes_19_413_payload_too_large_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"error\":\"Payload Too Large\",\"message\":\"Request body size exceeds maximum allowed size of 1024 bytes\"}").unwrap();
    (axum::http::StatusCode::from_u16(413).unwrap(), Json(expected_body))
}

async fn status_codes_403_Forbidden___Insufficient_permissions_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Not enough permissions\"}").unwrap();
    (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body))
}

async fn status_codes_21_431_request_header_fields_too_large_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"error\":\"Request Header Fields Too Large\",\"message\":\"Request headers exceed maximum allowed size of 8192 bytes\"}").unwrap();
    (axum::http::StatusCode::from_u16(431).unwrap(), Json(expected_body))
}

async fn status_codes_429_Too_Many_Requests_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"detail\":\"Rate limit exceeded. Try again in 60 seconds.\"}").unwrap();
    (axum::http::StatusCode::from_u16(429).unwrap(), Json(expected_body))
}

async fn status_codes_200_OK___Success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"id\":1,\"name\":\"Item 1\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_206_Partial_Content_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"binary_data_1024_bytes\"").unwrap();
    (axum::http::StatusCode::from_u16(206).unwrap(), Json(expected_body))
}

async fn headers_Header_regex_validation___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Request-Id\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Request-Id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"x_request_id\":\"12345\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_33_api_key_header_valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Content_Type_header___application_json_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Content-Type\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Content-Type\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"content_type\":\"application/json\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Accept_Language_header_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Accept-Language\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept-Language\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"accept_language\":\"en-US,en;q=0.9\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_X_API_Key_required_header___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"key\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"username\":\"secret\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Header_validation___max_length_constraint_fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Session-Id\":{\"maxLength\":20,\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Session-Id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_is_way_too_long_for_validation\",\"loc\":[\"header\",\"x-session-id\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_X_API_Key_required_header___missing_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Not authenticated\"}").unwrap();
    (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body))
}

async fn headers_Origin_header_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Origin\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"origin\":\"https://example.com\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_User_Agent_header___default_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"User-Agent\":{\"default\":\"testclient\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"User-Agent\":\"testclient\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_32_bearer_token_missing_prefix_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"value\":\"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\"},\"loc\":[\"headers\",\"Authorization\"],\"msg\":\"Invalid Bearer token format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Optional_header_with_None_default___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"strange-header\":{\"default\":null,\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"strange_header\":null}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Header_regex_validation___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Request-Id\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Request-Id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[0-9]{3,}$\"},\"input\":\"invalid-format\",\"loc\":[\"header\",\"x-request-id\"],\"msg\":\"String should match pattern '^[0-9]{3,}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_31_bearer_token_format_invalid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"value\":\"Bearer invalid token with spaces\"},\"loc\":[\"headers\",\"Authorization\"],\"msg\":\"Invalid Bearer token format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_X_API_Key_optional_header___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"msg\":\"Hello secret\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Authorization_header___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"credentials\":\"foobar\",\"scheme\":\"Digest\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_30_bearer_token_format_valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Authorization_header___missing_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Not authenticated\"}").unwrap();
    (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body))
}

async fn headers_Accept_header___JSON_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Accept\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"accept\":\"application/json\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Accept_Encoding_header_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Accept-Encoding\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Accept-Encoding\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"accept_encoding\":\"gzip, deflate, br\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Authorization_header___wrong_scheme_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Invalid authentication credentials\"}").unwrap();
    (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body))
}

async fn headers_Header_validation___min_length_constraint_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Token\":{\"minLength\":3,\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Token\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"header\",\"x-token\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Basic_authentication___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"password\":\"password\",\"username\":\"username\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Bearer_token_authentication___missing_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Not authenticated\"}").unwrap();
    (axum::http::StatusCode::from_u16(401).unwrap(), Json(expected_body))
}

async fn headers_X_API_Key_optional_header___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"key\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"msg\":\"Hello World\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Multiple_custom_headers_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Client-Version\":{\"source\":\"header\",\"type\":\"string\"},\"X-Request-Id\":{\"source\":\"header\",\"type\":\"string\"},\"X-Trace-Id\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Client-Version\",\"X-Request-Id\",\"X-Trace-Id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str(
                "{\"x_client_version\":\"1.2.3\",\"x_request_id\":\"req-12345\",\"x_trace_id\":\"trace-abc\"}",
            )
            .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_34_api_key_header_invalid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-f0-9]{32}$\",\"value\":\"invalid-key\"},\"loc\":[\"headers\",\"X-API-Key\"],\"msg\":\"Invalid API key format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Bearer_token_authentication___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"token\":\"valid_token_123\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Host_header_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Host\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Host\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"host\":\"example.com:8080\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Referer_header_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Referer\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"Referer\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"referer\":\"https://example.com/page\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Header_with_underscore_conversion___explicit_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Token\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"X-Token\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"x_token\":\"secret123\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Header_case_insensitivity___access_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test\":{\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"content_type_lower\":\"application/json\",\"content_type_mixed\":\"application/json\",\"content_type_upper\":\"application/json\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_User_Agent_header___custom_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"User-Agent\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"User-Agent\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"User-Agent\":\"Mozilla/5.0 Custom Browser\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Invalid_UUID_format_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Invalid_boolean_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"is_active\":{\"source\":\"query\",\"type\":\"boolean\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"is_active\",\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"maybe\",\"loc\":[\"query\",\"is_active\"],\"msg\":\"Input should be a valid boolean, unable to interpret input\",\"type\":\"bool_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Missing_required_query_parameter_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"q\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Array_max_items_constraint_violation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":[\"tag1\",\"tag2\",\"tag3\",\"tag4\",\"tag5\",\"tag6\",\"tag7\",\"tag8\",\"tag9\",\"tag10\",\"tag11\"],\"loc\":[\"body\",\"tags\"],\"msg\":\"[\\\"tag1\\\",\\\"tag2\\\",\\\"tag3\\\",\\\"tag4\\\",\\\"tag5\\\",\\\"tag6\\\",\\\"tag7\\\",\\\"tag8\\\",\\\"tag9\\\",\\\"tag10\\\",\\\"tag11\\\"] has more than 10 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Numeric_constraint_violation___gt__greater_than_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"price\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"number\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"price\",\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"gt\":0},\"input\":\"0\",\"loc\":[\"query\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_String_regex_pattern_mismatch_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"pattern\":\"^[a-zA-Z0-9_-]+$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_-]+$\"},\"input\":\"invalid!\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_-]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Invalid_enum_value_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"invalid_model\",\"loc\":[\"path\",\"model_name\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_String_min_length_constraint_violation_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Multiple_validation_errors_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\"},\"quantity\":{\"type\":\"integer\"}},\"required\":[\"name\",\"price\",\"quantity\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"X\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"gt\":0},\"input\":-10,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"},{\"input\":\"not_a_number\",\"loc\":[\"body\",\"quantity\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_String_max_length_constraint_violation_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"maxLength\":50,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":50},\"input\":\"this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter\",\"loc\":[\"query\",\"q\"],\"msg\":\"String should have at most 50 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Nested_object_validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"type\":\"string\"},\"zip_code\":{\"type\":\"string\"}},\"required\":[\"city\",\"zip_code\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"SF\",\"loc\":[\"body\",\"seller\",\"address\",\"city\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"min_length\":5},\"input\":\"123\",\"loc\":[\"body\",\"seller\",\"address\",\"zip_code\"],\"msg\":\"String should have at least 5 characters\",\"type\":\"string_too_short\"},{\"ctx\":{\"min_length\":3},\"input\":\"Jo\",\"loc\":[\"body\",\"seller\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_10_nested_error_path_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"},\"input\":\"invalid\",\"loc\":[\"body\",\"profile\",\"contact\",\"email\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Invalid_datetime_format_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"created_at\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-datetime\",\"loc\":[\"body\",\"created_at\"],\"msg\":\"Input should be a valid datetime\",\"type\":\"datetime_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Array_item_validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":123,\"loc\":[\"body\",\"tags\",\"2\"],\"msg\":\"Input should be a valid unknown\",\"type\":\"type_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Missing_required_body_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":{\"name\":\"Item\"},\"loc\":[\"body\",\"price\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Body_field_type_error___string_for_float_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not_a_float\",\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be a valid number, unable to parse string as a number\",\"type\":\"float_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Malformed_JSON_body_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Invalid request format\"}").unwrap();
    (axum::http::StatusCode::from_u16(400).unwrap(), Json(expected_body))
}

async fn validation_errors_Query_param_type_error___string_provided_for_int_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"},\"skip\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"q\",\"skip\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not_a_number\",\"loc\":[\"query\",\"skip\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Header_validation_error_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"source\":\"query\",\"type\":\"string\"},\"x-token\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[\"x-token\",\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"header\",\"x-token\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_09_multiple_validation_errors_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"age\":{\"minimum\":18,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"name\",\"email\",\"age\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"3 validation errors in request\",\"errors\":[{\"ctx\":{\"ge\":18},\"input\":15,\"loc\":[\"body\",\"age\"],\"msg\":\"Input should be greater than or equal to 18\",\"type\":\"greater_than_equal\"},{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"},\"input\":\"invalid-email\",\"loc\":[\"body\",\"email\"],\"msg\":\"String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$'\",\"type\":\"string_pattern_mismatch\"},{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Numeric_constraint_violation___le__less_than_or_equal_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"},\"q\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"limit\",\"q\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"le\":100},\"input\":\"101\",\"loc\":[\"query\",\"limit\"],\"msg\":\"Input should be less than or equal to 100\",\"type\":\"less_than_equal\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Array_min_items_constraint_violation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":[],\"loc\":[\"body\",\"tags\"],\"msg\":\"[] has less than 1 item\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Multiple_values_for_same_field_name_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"files\":[{\"content\":\"first file\",\"content_type\":\"text/plain\",\"filename\":\"file1.txt\",\"size\":10},{\"content\":\"second file\",\"content_type\":\"text/plain\",\"filename\":\"file2.txt\",\"size\":11}],\"tags\":[\"python\",\"rust\",\"web\"]}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_19_file_mime_spoofing_png_as_jpeg_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"declared_mime\":\"image/jpeg\",\"detected_type\":\"image/png\",\"magic_bytes\":\"89504e470d0a1a0a\"},\"loc\":[\"files\",\"image\"],\"msg\":\"File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_20_file_mime_spoofing_jpeg_as_png_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"declared_mime\":\"image/png\",\"detected_type\":\"image/jpeg\",\"magic_bytes\":\"ffd8ffe0\"},\"loc\":[\"files\",\"image\"],\"msg\":\"File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_21_file_pdf_magic_number_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Content_Type_validation___invalid_type_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str(
                "{\"detail\":\"Only image files are allowed (image/jpeg, image/png, image/gif)\"}",
            )
            .unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_PDF_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"document\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"document\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"content_type\":\"application/pdf\",\"filename\":\"report.pdf\",\"size\":16}")
                    .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_File_list_upload__array_of_files_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"filenames\":[\"file1.txt\",\"file2.txt\"],\"total_size\":35}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Optional_file_upload___provided_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"content_type\":\"text/plain\",\"filename\":\"optional.txt\",\"size\":21}")
                    .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_File_size_validation___too_large_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"File too large. Maximum size is 1MB\"}").unwrap();
    (axum::http::StatusCode::from_u16(413).unwrap(), Json(expected_body))
}

async fn multipart_Mixed_files_and_form_data_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"active\":{\"type\":\"string\"},\"age\":{\"type\":\"string\"},\"file\":{\"format\":\"binary\",\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"active\":\"true\",\"age\":\"25\",\"file\":{\"content\":\"file data here\",\"content_type\":\"text/plain\",\"filename\":\"upload.txt\",\"size\":14},\"username\":\"testuser\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Simple_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"test\":{\"content\":\"<file content>\",\"content_type\":\"text/plain\",\"filename\":\"test.txt\",\"size\":14}}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Empty_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"filename\":\"empty.txt\",\"size\":0}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Optional_file_upload___missing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"file\":null}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_File_upload_without_filename_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"test1\":\"<file1 content>\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_18_file_magic_number_jpeg_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_22_file_empty_buffer_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"buffer_size\":0},\"loc\":[\"files\",\"file\"],\"msg\":\"File buffer is empty\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_17_file_magic_number_png_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Form_data_without_files_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"additionalProperties\":false,\"properties\":{\"some\":{\"type\":\"string\"}},\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"some\":\"data\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Multiple_file_uploads_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"},\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\",\"test2\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"test1\":{\"content\":\"<file1 content>\",\"content_type\":\"text/plain\",\"filename\":\"test1.txt\",\"size\":15},\"test2\":{\"content\":\"<file2 content>\",\"content_type\":\"text/plain\",\"filename\":\"test2.txt\",\"size\":15}}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_File_upload_with_custom_headers_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test2\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"test2\":{\"content\":\"<file2 content>\",\"content_type\":\"text/plain\",\"filename\":\"test2.txt\",\"headers\":[[\"content-disposition\",\"form-data; name=\\\"test2\\\"; filename=\\\"test2.txt\\\"\"],[\"content-type\",\"text/plain\"],[\"x-custom\",\"f2\"]],\"size\":15}}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Required_file_upload___missing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"required\",\"loc\":[\"body\",\"file\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn multipart_Image_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"image\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"content_type\":\"image/jpeg\",\"filename\":\"photo.jpg\",\"size\":22}")
                    .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_UUID_field___invalid_format_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-valid-uuid\",\"loc\":[\"body\",\"item_id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_44_const_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"const\":\"1.0\",\"value\":\"2.0\"},\"loc\":[\"body\",\"version\"],\"msg\":\"Value must be exactly '1.0'\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Boolean_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"in_stock\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"in_stock\":true,\"name\":\"Item\",\"price\":42.0}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Numeric_le_validation___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":100.0}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Deeply_nested_objects_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"type\":\"string\"},\"country\":{\"additionalProperties\":false,\"properties\":{\"code\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"code\"],\"type\":\"object\"},\"street\":{\"type\":\"string\"}},\"required\":[\"street\",\"city\",\"country\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"Product\",\"price\":100.0,\"seller\":{\"address\":{\"city\":\"Springfield\",\"country\":{\"code\":\"US\",\"name\":\"USA\"},\"street\":\"123 Main St\"},\"name\":\"John Doe\"}}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Optional_fields___omitted_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"description\":null,\"name\":\"Foo\",\"price\":35.4,\"tax\":null}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_UUID_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"item_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\",\"name\":\"Item\"}")
                    .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Date_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"event_date\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"event_date\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"event_date\":\"2024-03-15\",\"name\":\"Conference\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_47_maxproperties_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"maxProperties\":3,\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_properties\":4,\"max_properties\":3},\"loc\":[\"body\"],\"msg\":\"Object must have at most 3 properties\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_46_minproperties_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"minProperties\":2,\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_properties\":1,\"min_properties\":2},\"loc\":[\"body\"],\"msg\":\"Object must have at least 2 properties\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_String_min_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Field_type_validation___invalid_type_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not a number\",\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be a valid number\",\"type\":\"float_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_36_oneof_schema_multiple_match_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":2},\"loc\":[\"body\"],\"msg\":\"Must match exactly one schema (oneOf), but matched 2\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Nested_object___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"image\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"image\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"image\":{\"name\":\"Product Image\",\"url\":\"https://example.com/image.jpg\"},\"name\":\"Foo\",\"price\":42.0}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_41_not_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_String_max_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":50},\"input\":\"This is a very long name that exceeds the maximum length\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at most 50 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_50_deep_nesting_4_levels_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"user\":{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"address\":{\"properties\":{\"street\":{\"type\":\"string\"}},\"required\":[\"street\"],\"type\":\"object\"}},\"required\":[\"address\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_48_dependencies_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_PATCH_partial_update_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"price\":{\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value = serde_json::from_str(
                "{\"description\":\"Original description\",\"name\":\"Original Item\",\"price\":45.0}",
            )
            .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_30_nested_object_missing_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"required\":true},\"loc\":[\"body\",\"profile\",\"email\"],\"msg\":\"Field required\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Datetime_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"created_at\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"created_at\":\"2024-03-15T10:30:00Z\",\"name\":\"Meeting\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_String_pattern_validation___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"Item\",\"sku\":\"ABC1234\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Extra_fields_ignored__no_additionalProperties_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"another_extra\":{\"type\":\"integer\"},\"extra_field\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"extra_field\",\"another_extra\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":42.0}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_40_anyof_schema_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":0},\"loc\":[\"body\"],\"msg\":\"Must match at least one schema (anyOf), but matched 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_39_anyof_schema_multiple_match_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Array_of_primitive_values_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"ratings\":{\"items\":{\"type\":\"number\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"ratings\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str(
                "{\"name\":\"Product\",\"ratings\":[4.5,4.8,5.0,4.2],\"tags\":[\"electronics\",\"gadget\",\"new\"]}",
            )
            .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Numeric_ge_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"ge\":1},\"input\":0.5,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than or equal to 1\",\"type\":\"greater_than_equal\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_37_oneof_schema_no_match_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"matched_schemas\":0},\"loc\":[\"body\"],\"msg\":\"Must match exactly one schema (oneOf), but matched 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Empty_array_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{},\"type\":\"array\"}},\"required\":[\"name\",\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":1},\"input\":[],\"loc\":[\"body\",\"tags\"],\"msg\":\"List should have at least 1 item after validation\",\"type\":\"too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_38_anyof_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Empty_JSON_object_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"description\":null,\"name\":null,\"price\":null,\"tax\":null}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_String_pattern_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[A-Z]{3}[0-9]{4}$\"},\"input\":\"ABC-123\",\"loc\":[\"body\",\"sku\"],\"msg\":\"String should match pattern '^[A-Z]{3}[0-9]{4}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_49_dependencies_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"dependency\":\"credit_card\",\"required_fields\":[\"billing_address\"]},\"loc\":[\"body\"],\"msg\":\"When 'credit_card' is present, 'billing_address' is required\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Simple_JSON_object___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str(
                "{\"description\":\"A very nice Item\",\"name\":\"Foo\",\"price\":35.4,\"tax\":3.2}",
            )
            .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Required_field_missing___validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"description\",\"price\",\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"body\",\"name\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_35_oneof_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Enum_field___invalid_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"category\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'electronics', 'clothing' or 'books'\"},\"input\":\"furniture\",\"loc\":[\"body\",\"category\"],\"msg\":\"Input should be 'electronics', 'clothing' or 'books'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Enum_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"category\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"category\":\"electronics\",\"name\":\"Item\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_33_allof_schema_composition_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"allOf\":[{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"},{\"properties\":{\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}]}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_45_minproperties_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"minProperties\":2,\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Body_with_query_parameters_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"item\":{\"name\":\"Item\",\"price\":42.0},\"limit\":10}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_42_not_schema_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"prohibited_value\":\"admin\"},\"loc\":[\"body\",\"username\"],\"msg\":\"Must not match the schema\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_43_const_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_32_schema_ref_definitions_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"definitions\":{\"Product\":{\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}},\"properties\":{\"product\":{\"$ref\":\"#/definitions/Product\"}},\"required\":[\"product\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_29_nested_object_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_34_additional_properties_false_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"email\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"additional_properties\":false,\"unexpected_field\":\"extra_field\"},\"loc\":[\"body\",\"extra_field\"],\"msg\":\"Additional properties are not allowed\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Null_value_for_optional_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"null\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"null\"}},\"required\":[\"name\",\"price\",\"description\",\"tax\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"description\":null,\"name\":\"Item\",\"price\":42.0,\"tax\":null}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_31_nullable_property_null_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":[\"string\",\"null\"]},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("null").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn json_bodies_Array_of_objects___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"images\":{\"items\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"type\":\"array\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"images\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"images\":[{\"name\":\"Front\",\"url\":\"https://example.com/img1.jpg\"},{\"name\":\"Back\",\"url\":\"https://example.com/img2.jpg\"}],\"name\":\"Product Bundle\",\"tags\":[\"electronics\",\"gadget\"]}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_415_Unsupported_Media_Type_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"detail\":\"Unsupported media type\"}").unwrap();
    (axum::http::StatusCode::from_u16(415).unwrap(), Json(expected_body))
}

async fn content_types_XML_response___application_xml_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("\"<?xml version=\\\"1.0\\\"?><item><name>Item</name><price>42.0</price></item>\"")
            .unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_14_content_type_case_insensitive_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"test\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_JSON_with_UTF_8_charset_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"emoji\":\"☕\",\"name\":\"Café\"}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_16_text_plain_not_accepted_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"error\":\"Unsupported Media Type. Expected application/json\"}").unwrap();
    (axum::http::StatusCode::from_u16(415).unwrap(), Json(expected_body))
}

async fn content_types_PDF_response___application_pdf_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"pdf_binary_data\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_20_content_length_mismatch_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"error\":\"Content-Length header does not match actual body size\"}").unwrap();
    (axum::http::StatusCode::from_u16(400).unwrap(), Json(expected_body))
}

async fn content_types_17_vendor_json_accepted_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"data\":\"value\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_13_json_with_charset_utf16_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"error\":\"Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported.\"}")
            .unwrap();
    (axum::http::StatusCode::from_u16(415).unwrap(), Json(expected_body))
}

async fn content_types_JSON_response___application_json_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"name\":\"Item\",\"price\":42.0}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_15_multipart_boundary_required_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"error\":\"multipart/form-data requires 'boundary' parameter\"}").unwrap();
    (axum::http::StatusCode::from_u16(400).unwrap(), Json(expected_body))
}

async fn content_types_Content_negotiation___Accept_header_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"id\":1,\"name\":\"Item\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_HTML_response___text_html_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"<html><body><h1>Hello</h1></body></html>\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_JPEG_image_response___image_jpeg_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"jpeg_binary_data\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_19_missing_content_type_default_json_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"test\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_PNG_image_response___image_png_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"png_binary_data\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_Plain_text_response___text_plain_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"Hello, World!\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_18_content_type_with_multiple_params_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"value\":\"test\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_CSV_response___text_csv_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"id,name,price\\n1,Item A,10.0\\n2,Item B,20.0\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn content_types_Binary_response___application_octet_stream_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("\"binary_data_placeholder\"").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn path_params_Boolean_path_parameter___True_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"boolean\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":true}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_29_decimal_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"amount\":{\"format\":\"decimal\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"amount\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"amount\":\"19.99\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_combined_lt_and_gt_constraints___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"exclusiveMaximum\":3,\"exclusiveMinimum\":1,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":2}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_33_string_pattern_path_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"owner\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"source\":\"path\",\"type\":\"string\"},\"repo\":{\"pattern\":\"^[a-zA-Z0-9-_]+$\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"owner\",\"repo\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"owner\":\"spikard-labs\",\"repo\":\"spikard-http\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_31_string_minlength_path_failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3},\"loc\":[\"path\",\"username\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_35_negative_integer_path_param_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":-100}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Enum_path_parameter___invalid_value_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"foo\",\"loc\":[\"path\",\"model_name\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_27_datetime_format_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"timestamp\":{\"format\":\"date-time\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"timestamp\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"timestamp\":\"2025-10-30T14:30:00Z\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_25_date_format_invalid_failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"date\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"date\",\"value\":\"2025-13-45\"},\"loc\":[\"path\",\"date\"],\"msg\":\"Invalid date format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_lt_constraint___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"exclusiveMaximum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":2}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_gt_constraint___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"exclusiveMinimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":42}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_28_duration_format_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"duration\":{\"format\":\"duration\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"duration\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"duration\":\"P1DT2H30M\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_parameter_type_syntax_with_override_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"count\":{\"maximum\":100,\"minimum\":1,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"count\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"count\":\"50\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_20_uuid_v3_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\",\"uuidVersion\":\"3\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":\"e8b5a51d-11c8-3310-a6ab-367563f20686\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter___invalid_string_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"foobar\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_30_string_minlength_path_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"username\":\"alice\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_le_constraint___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"maximum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":3}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_parameter_type_syntax___invalid_UUID_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"path\",\"id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_type_parameter___file_path_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"file_path\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"file_path\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"file_path\":\"home/johndoe/myfile.txt\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_parameter_with_type_syntax___UUID_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":\"550e8400-e29b-41d4-a716-446655440000\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_32_string_maxlength_path_failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"maxLength\":20,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":42,\"max_length\":20},\"loc\":[\"path\",\"username\"],\"msg\":\"String length must not exceed 20\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":42}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_34_string_pattern_path_failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"owner\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"owner\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9-]+$\",\"value\":\"invalid@owner\"},\"loc\":[\"path\",\"owner\"],\"msg\":\"String does not match pattern\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_21_uuid_v5_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\",\"uuidVersion\":\"5\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":\"630eb68f-e0fa-5ecc-887a-7c7a62614681\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_String_path_parameter_with_max_length___failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"maxLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":3},\"input\":\"foobar\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"String should have at most 3 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_String_path_parameter_with_min_length___failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"minLength\":3,\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"fo\",\"loc\":[\"path\",\"item_id\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Multiple_path_parameters___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"order_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"},\"service_id\":{\"source\":\"path\",\"type\":\"integer\"},\"user_id\":{\"source\":\"path\",\"type\":\"string\"},\"version\":{\"source\":\"path\",\"type\":\"number\"}},\"required\":[\"order_id\",\"service_id\",\"user_id\",\"version\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"order_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\",\"service_id\":1,\"user_id\":\"abc\",\"version\":1.0}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Date_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"date_param\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date_param\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"date_param\":\"2023-07-15\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_gt_constraint___failure_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"exclusiveMinimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"gt\":3},\"input\":2,\"loc\":[\"path\",\"item_id\"],\"msg\":\"Input should be greater than 3\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_24_date_format_path_param_success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"date\":{\"format\":\"date\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"date\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"date\":\"2025-10-30\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Float_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"number\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":42.5}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_parameter_with_type_syntax___integer_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"user_id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"user_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"user_id\":\"42\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_String_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":\"foobar\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_UUID_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"item_id\":\"ec38df32-ceda-4cfa-9b4a-1aeb94ad551a\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_ge_constraint___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"minimum\":3,\"source\":\"path\",\"type\":\"integer\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":3}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Enum_path_parameter___success_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"model_name\":{\"enum\":[\"alexnet\",\"lenet\",\"resnet\"],\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"model_name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"model_name\":\"alexnet\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Boolean_path_parameter___numeric_1_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"source\":\"path\",\"type\":\"boolean\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_id\":true}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cors_07_cors_preflight_header_not_allowed_handler(
    headers: axum::http::HeaderMap,
) -> axum::response::Result<axum::response::Response<axum::body::Body>, axum::response::Response<axum::body::Body>> {
    use spikard_http::CorsConfig;
    use spikard_http::cors::handle_preflight;

    // Parse CORS configuration
    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap();

    // Handle the preflight request
    handle_preflight(&headers, &cors_config)
}

async fn cors_CORS_preflight_request_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("null").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cors_CORS_with_credentials_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"username\":\"john\"}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cors_08_cors_max_age_handler(
    headers: axum::http::HeaderMap,
) -> axum::response::Result<axum::response::Response<axum::body::Body>, axum::response::Response<axum::body::Body>> {
    use spikard_http::CorsConfig;
    use spikard_http::cors::handle_preflight;

    // Parse CORS configuration
    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"POST\"],\"allowed_origins\":[\"https://example.com\"],\"max_age\":3600}").unwrap();

    // Handle the preflight request
    handle_preflight(&headers, &cors_config)
}

async fn cors_10_cors_origin_null_handler(headers: axum::http::HeaderMap) -> impl axum::response::IntoResponse {
    // CORS validation
    use spikard_http::CorsConfig;
    use spikard_http::cors::{add_cors_headers, validate_cors_request};

    let cors_config: CorsConfig =
        serde_json::from_str("{\"allowed_methods\":[\"GET\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {
        return err_response;
    }

    // Add CORS headers to response
    let expected_body: Value = serde_json::from_str("{\"error\":\"Origin 'null' is not allowed\"}").unwrap();
    let mut response = (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response
}

async fn cors_CORS_wildcard_origin_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"data\":\"public\"}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cors_CORS_request_blocked_handler(headers: axum::http::HeaderMap) -> impl axum::response::IntoResponse {
    // CORS validation
    use spikard_http::CorsConfig;
    use spikard_http::cors::{add_cors_headers, validate_cors_request};

    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"GET\",\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {
        return err_response;
    }

    // Add CORS headers to response
    let expected_body: Value =
        serde_json::from_str("{\"detail\":\"CORS request from origin 'https://malicious-site.com' not allowed\"}")
            .unwrap();
    let mut response = (axum::http::StatusCode::from_u16(403).unwrap(), Json(expected_body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response
}

async fn cors_Simple_CORS_request_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"items\":[]}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cors_09_cors_expose_headers_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // CORS validation
    use spikard_http::CorsConfig;
    use spikard_http::cors::{add_cors_headers, validate_cors_request};

    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_methods\":[\"GET\"],\"allowed_origins\":[\"https://example.com\"],\"expose_headers\":[\"X-Total-Count\",\"X-Request-Id\"]}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {
        return err_response;
    }
    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"Origin\":{\"source\":\"header\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            // Add CORS headers to response
            let expected_body: Value = serde_json::from_str("null").unwrap();
            let mut response = (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
                .into_response();
            response = add_cors_headers(response, origin, &cors_config);
            response
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()
        }
    }
}

async fn cors_06_cors_preflight_method_not_allowed_handler(
    headers: axum::http::HeaderMap,
) -> axum::response::Result<axum::response::Response<axum::body::Body>, axum::response::Response<axum::body::Body>> {
    use spikard_http::CorsConfig;
    use spikard_http::cors::handle_preflight;

    // Parse CORS configuration
    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"GET\",\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap();

    // Handle the preflight request
    handle_preflight(&headers, &cors_config)
}

async fn http_methods_OPTIONS___CORS_preflight_request_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("null").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn http_methods_DELETE___Remove_resource_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PUT___Create_resource_if_doesn_t_exist_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":999,\"name\":\"New Item\",\"price\":49.99}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PATCH___Update_multiple_fields_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"in_stock\",\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":1,\"in_stock\":false,\"name\":\"Updated Name\",\"price\":89.99}")
                    .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PUT___Validation_error_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\",\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"minLength\":3,\"type\":\"string\"},\"price\":{\"exclusiveMinimum\":0,\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"2 validation errors in request\",\"errors\":[{\"input\":\"X\",\"loc\":[\"body\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"},{\"input\":-10,\"loc\":[\"body\",\"price\"],\"msg\":\"Input should be greater than 0\",\"type\":\"greater_than\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_HEAD___Get_metadata_without_body_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_DELETE___With_response_body_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":1,\"message\":\"Item deleted successfully\",\"name\":\"Deleted Item\"}")
                    .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PUT___Missing_required_field_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"1\",\"loc\":[\"body\",\"price\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PATCH___Partial_update_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"price\":{\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":1,\"in_stock\":true,\"name\":\"Existing Item\",\"price\":79.99}")
                    .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_DELETE___Resource_not_found_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PUT___Idempotent_operation_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"type\":\"integer\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"id\",\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"id\":1,\"name\":\"Fixed Name\",\"price\":50.0}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn http_methods_PUT___Complete_resource_replacement_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"id\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"id\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\"},\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"description\",\"id\",\"in_stock\",\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(&query_params, &raw_query_params, &path_params, &headers_map, &cookies) {
        Ok(validated) => {
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }

            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"description\":\"Completely replaced\",\"id\":1,\"in_stock\":true,\"name\":\"Updated Item\",\"price\":99.99}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_String_validation_with_regex___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_query\":{\"pattern\":\"^fixedquery$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"item_query\":\"fixedquery\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_49_integer_gt_constraint_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"limit\":5}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Enum_query_parameter___invalid_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"model\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"model\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"expected\":\"'alexnet', 'resnet' or 'lenet'\"},\"input\":\"vgg16\",\"loc\":[\"query\",\"model\"],\"msg\":\"Input should be 'alexnet', 'resnet' or 'lenet'\",\"type\":\"enum\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_68_array_uniqueitems_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\",\"uniqueItems\":true}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"ids\":[1,2,3,4]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_47_pattern_validation_email_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"email\":\"user@example.com\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_integer_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar 42\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_string_query_parameter___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"query\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_57_boolean_empty_string_coercion_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"active\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"active\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"active\":false}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_52_integer_le_constraint_boundary_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"limit\":100}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_List_with_default_empty_array___no_values_provided_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("[]").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Date_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"event_date\":{\"format\":\"date\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"event_date\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"event_date\":\"2024-01-15\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_String_query_param_with_max_length_constraint___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"name\":{\"maxLength\":10,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":10},\"input\":\"this_is_way_too_long\",\"loc\":[\"query\",\"name\"],\"msg\":\"String should have at most 10 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_45_string_minlength_validation_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"term\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":2,\"min_length\":3},\"loc\":[\"query\",\"term\"],\"msg\":\"String length must be at least 3\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_with_default_value___override_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar 50\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_67_multipleof_constraint_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"quantity\":{\"multipleOf\":5,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"multiple_of\":5,\"value\":17},\"loc\":[\"query\",\"quantity\"],\"msg\":\"Value must be a multiple of 5\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_58_format_email_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"email\":\"user@example.com\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_query_param_with_ge_constraint___boundary_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"minimum\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":10}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_query_param_with_gt_constraint___valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":1}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_integer_query_parameter___invalid_type_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"baz\",\"loc\":[\"query\",\"query\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_integer_query_parameter___float_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":42.5,\"loc\":[\"query\",\"query\"],\"msg\":\"Input should be a valid integer, unable to parse string as an integer\",\"type\":\"int_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Query_parameter_with_URL_encoded_special_characters_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"name\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"name\":\"test&value=123\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_59_format_email_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"email\",\"value\":\"not-an-email\"},\"loc\":[\"query\",\"email\"],\"msg\":\"Invalid email format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_43_scientific_notation_float_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"threshold\":{\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"threshold\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"threshold\":0.0015}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_63_format_uri_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"url\":{\"format\":\"uri\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"url\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"url\":\"https://example.com/path?query=value\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Boolean_query_parameter___numeric_1_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"flag\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"flag\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"flag\":true}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_String_query_param_with_min_length_constraint___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"name\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"min_length\":3},\"input\":\"ab\",\"loc\":[\"query\",\"name\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Optional_string_query_parameter___provided_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar baz\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_List_of_integers___multiple_values_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"device_ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"device_ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("[1,2]").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_query_param_with_lt_constraint___valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"exclusiveMaximum\":50,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":49}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_42_negative_integer_query_param_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"offset\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"offset\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"offset\":-10}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_46_string_maxlength_validation_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"term\":{\"maxLength\":10,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":21,\"max_length\":10},\"loc\":[\"query\",\"term\"],\"msg\":\"String length must not exceed 10\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_56_array_maxitems_constraint_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"maxItems\":5,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":6,\"max_items\":5},\"loc\":[\"query\",\"tags\"],\"msg\":\"Array must not contain more than 5 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_String_query_param_with_regex_pattern___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"code\":{\"pattern\":\"^[0-9]{3,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[0-9]{3,}$\"},\"input\":\"abc123\",\"loc\":[\"query\",\"code\"],\"msg\":\"String should match pattern '^[0-9]{3,}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_44_string_minlength_validation_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"term\":{\"minLength\":3,\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"term\":\"foo\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_61_format_ipv4_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ip\":{\"format\":\"ipv4\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"ipv4\",\"value\":\"999.999.999.999\"},\"loc\":[\"query\",\"ip\"],\"msg\":\"Invalid IPv4 address format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_48_pattern_validation_email_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\\\.[a-zA-Z]{2,}$\",\"value\":\"invalid-email\"},\"loc\":[\"query\",\"email\"],\"msg\":\"String does not match pattern\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_integer_query_parameter___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"query\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Query_parameter_with_special_characters___URL_encoding_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"source\":\"query\",\"type\":\"string\"},\"special\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"email\",\"special\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"email\":\"x@test.com\",\"special\":\"&@A.ac\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_List_query_parameter___required_but_missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"device_ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"device_ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"query\",\"device_ids\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Required_string_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar baz\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_66_multipleof_constraint_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"quantity\":{\"multipleOf\":5,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"quantity\":15}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_53_integer_le_constraint_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"maximum\":100,\"value\":101},\"loc\":[\"query\",\"limit\"],\"msg\":\"Value must not exceed 100\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Multiple_query_parameters_with_different_types_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"active\":{\"source\":\"query\",\"type\":\"boolean\"},\"age\":{\"source\":\"query\",\"type\":\"integer\"},\"name\":{\"source\":\"query\",\"type\":\"string\"},\"score\":{\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"active\",\"age\",\"name\",\"score\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"active\":true,\"age\":30,\"name\":\"john\",\"score\":95.5}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_71_array_separator_semicolon_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"colors\":{\"items\":{\"type\":\"string\"},\"separator\":\";\",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"colors\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"colors\":[\"red\",\"green\",\"blue\"]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_70_array_separator_pipe_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"separator\":\"|\",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"tags\":[\"python\",\"rust\",\"typescript\"]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_with_default_value___not_provided_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"query\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar 10\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Boolean_query_parameter___true_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"flag\":{\"source\":\"query\",\"type\":\"boolean\"}},\"required\":[\"flag\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"flag\":true}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Integer_query_param_with_le_constraint___boundary_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"maximum\":100,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":100}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Float_query_param_with_ge_constraint___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"price\":{\"minimum\":0.01,\"source\":\"query\",\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"price\":0.01}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_51_integer_ge_constraint_boundary_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"offset\":{\"minimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"offset\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"offset\":0}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Optional_integer_query_parameter___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar None\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_69_array_uniqueitems_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"source\":\"query\",\"type\":\"array\",\"uniqueItems\":true}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"duplicate_index\":2,\"duplicate_value\":2,\"unique_items\":true},\"loc\":[\"query\",\"ids\"],\"msg\":\"Array items must be unique\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_72_array_separator_space_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"keywords\":{\"items\":{\"type\":\"string\"},\"separator\":\" \",\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"keywords\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"keywords\":[\"rust\",\"web\",\"framework\"]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_String_validation_with_regex___failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_query\":{\"pattern\":\"^fixedquery$\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_query\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^fixedquery$\"},\"input\":\"nonregexquery\",\"loc\":[\"query\",\"item_query\"],\"msg\":\"String should match pattern '^fixedquery$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_65_format_hostname_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"host\":{\"format\":\"hostname\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"host\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"host\":\"api.example.com\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Query_parameter_with_URL_encoded_space_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"name\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"name\":\"hello world\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_List_of_strings___multiple_values_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"q\":{\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"q\":[\"foo\",\"bar\"]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Optional_query_parameter_with_default_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"default\":10,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"limit\":10}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_62_format_ipv6_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ip\":{\"format\":\"ipv6\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"ip\":\"2001:0db8:85a3:0000:0000:8a2e:0370:7334\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Array_query_parameter___single_value_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("[\"apple\"]").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Optional_string_query_parameter___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"query\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("\"foo bar None\"").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Datetime_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"timestamp\":{\"format\":\"date-time\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"timestamp\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"timestamp\":\"2024-01-15T10:30:00Z\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_UUID_query_parameter___invalid_format_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"not-a-uuid\",\"loc\":[\"query\",\"item_id\"],\"msg\":\"Input should be a valid UUID\",\"type\":\"uuid_parsing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Array_query_parameter___empty_array_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"default\":[],\"items\":{\"type\":\"string\"},\"source\":\"query\",\"type\":\"array\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("[]").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_Enum_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"model\":{\"enum\":[\"alexnet\",\"resnet\",\"lenet\"],\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"model\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"model\":\"alexnet\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_UUID_query_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"item_id\":{\"format\":\"uuid\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"item_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value =
                serde_json::from_str("{\"item_id\":\"c892496f-b1fd-4b91-bdb8-b46f92df1716\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_50_integer_gt_constraint_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"limit\":{\"exclusiveMinimum\":0,\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"limit\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"exclusive_minimum\":0,\"value\":0},\"loc\":[\"query\",\"limit\"],\"msg\":\"Value must be greater than 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_64_format_uri_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"url\":{\"format\":\"uri\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"url\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"format\":\"uri\",\"value\":\"not a uri\"},\"loc\":[\"query\",\"url\"],\"msg\":\"Invalid URI format\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_54_array_minitems_constraint_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"minItems\":2,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"ids\":[1,2,3]}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_55_array_minitems_constraint_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ids\":{\"items\":{\"type\":\"integer\"},\"minItems\":2,\"source\":\"query\",\"type\":\"array\"}},\"required\":[\"ids\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_items\":1,\"min_items\":2},\"loc\":[\"query\",\"ids\"],\"msg\":\"Array must contain at least 2 items\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn query_params_60_format_ipv4_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"ip\":{\"format\":\"ipv4\",\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"ip\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"ip\":\"192.168.1.1\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_25_cookie_samesite_lax_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking\":{\"samesite\":\"Lax\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Optional_cookie_parameter___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"ads_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"ads_id\":\"abc123\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Cookie_regex_pattern_validation___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking_id\":{\"pattern\":\"^[A-Z0-9]{8}$\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"pattern\":\"^[A-Z0-9]{8}$\"},\"input\":\"invalid-format\",\"loc\":[\"cookie\",\"tracking_id\"],\"msg\":\"String should match pattern '^[A-Z0-9]{8}$'\",\"type\":\"string_pattern_mismatch\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response___session_cookie__no_max_age_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Session cookie set\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_27_cookie_httponly_flag_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session\":{\"httponly\":true,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_attributes_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie set\"}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cookies_24_cookie_samesite_strict_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session_id\":{\"samesite\":\"Strict\",\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_APIKey_cookie_authentication___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"username\":\"secret\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Cookie_validation___min_length_constraint_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"token\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"token\":\"abc\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Cookie_validation___min_length_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking_id\":{\"minLength\":3,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"cookie\",\"tracking_id\"],\"msg\":\"String should have at least 3 characters\",\"type\":\"string_too_short\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Cookie_validation___max_length_constraint_fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session_id\":{\"maxLength\":20,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"max_length\":20},\"input\":\"this_cookie_value_is_way_too_long\",\"loc\":[\"cookie\",\"session_id\"],\"msg\":\"String should have at most 20 characters\",\"type\":\"string_too_long\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Required_cookie___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"fatebook_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"session_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":\"\",\"loc\":[\"cookie\",\"session_id\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Optional_cookie_parameter___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"ads_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"ads_id\":null}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_APIKey_cookie_authentication___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"key\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 422;

            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"input\":null,\"loc\":[\"cookie\",\"key\"],\"msg\":\"Field required\",\"type\":\"missing\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response___multiple_cookies_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"session\":{\"type\":\"string\"},\"user\":{\"type\":\"string\"}},\"required\":[\"user\",\"session\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Multiple cookies set\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_SameSite_Lax_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie set with SameSite=Lax\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response___delete_cookie_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie deleted\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_path_attribute_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie set with path\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Optional_APIKey_cookie___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"key\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"msg\":\"Create an account first\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_SameSite_Strict_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value =
                serde_json::from_str("{\"message\":\"Cookie set with SameSite=Strict\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_SameSite_None_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie set with SameSite=None\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Cookie_regex_pattern_validation___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"tracking_id\":\"ABC12345\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_set_cookie___basic_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"message\":\"Come to the dark side, we have cookies\"}").unwrap();
    (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
}

async fn cookies_Multiple_cookies___success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"fatebook_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"googall_tracker\":{\"source\":\"cookie\",\"type\":\"string\"},\"session_id\":{\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str(
                "{\"fatebook_tracker\":\"tracker456\",\"googall_tracker\":\"ga789\",\"session_id\":\"session123\"}",
            )
            .unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_26_cookie_secure_flag_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"auth_token\":{\"secure\":true,\"source\":\"cookie\",\"type\":\"string\"}},\"required\":[\"auth_token\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("null").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_domain_attribute_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"message\":\"Cookie set with domain\"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_19_emoji_in_strings_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"text\":{\"maxLength\":100,\"minLength\":1,\"type\":\"string\"}},\"required\":[\"text\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"text\":\"Hello 👋 World 🌍\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_12_percent_encoded_special_chars_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"term\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"term\":\"hi there\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Special_string_values_and_escaping_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"backslashes\":{\"type\":\"string\"},\"empty_string\":{\"type\":\"string\"},\"quotes\":{\"type\":\"string\"},\"special_chars\":{\"type\":\"string\"},\"tabs_newlines\":{\"type\":\"string\"},\"unicode_escapes\":{\"type\":\"string\"},\"whitespace\":{\"type\":\"string\"}},\"required\":[\"empty_string\",\"whitespace\",\"tabs_newlines\",\"quotes\",\"backslashes\",\"unicode_escapes\",\"special_chars\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"backslashes\":\"C:\\\\\\\\Users\\\\\\\\Path\",\"empty_string\":\"\",\"quotes\":\"He said \\\"hello\\\" and 'goodbye'\",\"special_chars\":\"!@#$%^&*()_+-=[]{}|;':\\\",./<>?\",\"tabs_newlines\":\"line1\\n\\tline2\\r\\nline3\",\"unicode_escapes\":\"Hello\",\"whitespace\":\"   \"}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_15_float_precision_preservation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"value\":{\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"value\":3.141592653589793}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_13_empty_string_query_param_preserved_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"filter\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"filter\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"filter\":\"\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_24_array_with_holes_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"items\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"items\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"missing_indices\":[1,3,4]},\"loc\":[\"body\",\"items\"],\"msg\":\"Array indices must be consecutive starting from 0\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_21_scientific_notation_number_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"value\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"value\":123000}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Float_precision_and_rounding_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"expected_sum\":{\"type\":\"number\"},\"precise_value\":{\"type\":\"number\"},\"value1\":{\"type\":\"number\"},\"value2\":{\"type\":\"number\"},\"very_large\":{\"type\":\"number\"},\"very_small\":{\"type\":\"number\"}},\"required\":[\"value1\",\"value2\",\"expected_sum\",\"precise_value\",\"very_small\",\"very_large\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"precise_value\":3.141592653589793,\"sum\":0.30000000000000004,\"very_large\":1.7976931348623157e308,\"very_small\":1e-10}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Unicode_and_emoji_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"emoji_reactions\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"description\",\"tags\",\"emoji_reactions\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"description\":\"Best café in München 🇩🇪\",\"emoji_reactions\":\"👍❤️😂🎉\",\"id\":1,\"name\":\"Coffee Shop ☕\",\"tags\":[\"食べ物\",\"音楽\",\"💰\"]}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_17_extremely_long_string_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"content\":{\"maxLength\":10000,\"type\":\"string\"}},\"required\":[\"content\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"actual_length\":10001,\"max_length\":10000},\"loc\":[\"body\",\"content\"],\"msg\":\"String length must not exceed 10000\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_11_utf8_query_parameter_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"term\":{\"source\":\"query\",\"type\":\"string\"}},\"required\":[\"term\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"term\":\"café\"}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_18_unicode_normalization_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"name\":\"café\"}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_20_null_byte_in_string_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"filename\":{\"pattern\":\"^[^\\\\x00]+$\",\"type\":\"string\"}},\"required\":[\"filename\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"detail\":\"1 validation error in request\",\"errors\":[{\"ctx\":{\"value\":\"file\\\\u0000.txt\"},\"loc\":[\"body\",\"filename\"],\"msg\":\"String contains null byte character\",\"type\":\"validation_error\"}],\"status\":422,\"title\":\"Request Validation Failed\",\"type\":\"https://spikard.dev/errors/validation-error\"}").unwrap();
            (axum::http::StatusCode::from_u16(422).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_23_deeply_nested_json_limit_handler() -> impl axum::response::IntoResponse {
    let expected_body: Value =
        serde_json::from_str("{\"error\":\"Request body exceeds maximum nesting depth of 32\"}").unwrap();
    (axum::http::StatusCode::from_u16(400).unwrap(), Json(expected_body))
}

async fn edge_cases_14_large_integer_boundary_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"id\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"id\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"id\":9007199254740991}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_22_leading_zeros_integer_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"value\":{\"source\":\"query\",\"type\":\"integer\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
            raw_query_params.insert(key.to_string(), value.to_string());
        }
    }

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {
        if name != axum::http::header::COOKIE {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(name.to_string(), value_str.to_string());
            }
        }
    }

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for result in cookie::Cookie::split_parse(cookie_str) {
                if let Ok(cookie) = result {
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }
            }
        }
    }

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &HashMap::new(), //path_params,
        &headers_map,
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            let expected_body: Value = serde_json::from_str("{\"value\":123}").unwrap();
            (
                axum::http::StatusCode::from_u16(status_code).unwrap(),
                Json(expected_body),
            )
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Large_integer_boundary_values_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"large_int\":{\"type\":\"integer\"},\"max_safe_int\":{\"type\":\"integer\"},\"negative_large\":{\"type\":\"integer\"}},\"required\":[\"max_safe_int\",\"large_int\",\"negative_large\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"large_int\":9223372036854775807,\"max_safe_int\":9007199254740991,\"negative_large\":-9223372036854775808}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Deeply_nested_structure__10__levels_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"level1\":{\"additionalProperties\":false,\"properties\":{\"level2\":{\"additionalProperties\":false,\"properties\":{\"level3\":{\"additionalProperties\":false,\"properties\":{\"level4\":{\"additionalProperties\":false,\"properties\":{\"level5\":{\"additionalProperties\":false,\"properties\":{\"level6\":{\"additionalProperties\":false,\"properties\":{\"level7\":{\"additionalProperties\":false,\"properties\":{\"level8\":{\"additionalProperties\":false,\"properties\":{\"level9\":{\"additionalProperties\":false,\"properties\":{\"level10\":{\"additionalProperties\":false,\"properties\":{\"depth\":{\"type\":\"integer\"},\"value\":{\"type\":\"string\"}},\"required\":[\"value\",\"depth\"],\"type\":\"object\"}},\"required\":[\"level10\"],\"type\":\"object\"}},\"required\":[\"level9\"],\"type\":\"object\"}},\"required\":[\"level8\"],\"type\":\"object\"}},\"required\":[\"level7\"],\"type\":\"object\"}},\"required\":[\"level6\"],\"type\":\"object\"}},\"required\":[\"level5\"],\"type\":\"object\"}},\"required\":[\"level4\"],\"type\":\"object\"}},\"required\":[\"level3\"],\"type\":\"object\"}},\"required\":[\"level2\"],\"type\":\"object\"}},\"required\":[\"level1\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str(
                "{\"max_depth\":10,\"message\":\"Processed deeply nested structure\",\"value_found\":\"deep\"}",
            )
            .unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_Empty_and_null_value_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"empty_array\":{\"items\":{},\"type\":\"array\"},\"empty_object\":{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"},\"empty_string\":{\"type\":\"string\"},\"explicit_null\":{\"type\":\"null\"},\"false_boolean\":{\"type\":\"boolean\"},\"zero_number\":{\"type\":\"integer\"}},\"required\":[\"explicit_null\",\"empty_string\",\"empty_array\",\"empty_object\",\"zero_number\",\"false_boolean\"],\"type\":\"object\"}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"empty_array_length\":0,\"empty_object_keys\":0,\"empty_string_length\":0,\"explicit_null_is_null\":true,\"false_is_false\":true,\"zero_is_falsy\":true}").unwrap();
            (axum::http::StatusCode::from_u16(200).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn edge_cases_16_negative_zero_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::validation::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"offset\":{\"type\":\"number\"}},\"required\":[\"offset\"],\"type\":\"object\"}",
    )
    .unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {
        Ok(_) => {
            let expected_body: Value = serde_json::from_str("{\"offset\":0}").unwrap();
            (axum::http::StatusCode::from_u16(201).unwrap(), Json(expected_body))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}
