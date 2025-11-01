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

/// App for fixture: Boolean path parameter - True
pub fn create_app_path_params_Boolean_path_parameter___True() -> Router {
    Router::new()
        .route(
            "/path/bool/True",
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
            "/path/param-lt-gt/2",
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
            "/path/param-lt/2",
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
            "/path/param-gt/42",
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
            "/path/param-le/3",
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
            "/files/home/johndoe/myfile.txt",
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
            "/path/int/42",
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
            "/1.0/1/abc/c892496f-b1fd-4b91-bdb8-b46f92df1716",
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
            "/date/2023-07-15",
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
            "/path/float/42.5",
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
            "/path/str/foobar",
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
            "/items/ec38df32-ceda-4cfa-9b4a-1aeb94ad551a",
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
            "/path/param-ge/3",
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
            "/models/alexnet",
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
            "/path/bool/1",
            get(path_params_Boolean_path_parameter___numeric_1_handler),
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

/// App for fixture: Invalid UUID format
pub fn create_app_validation_errors_Invalid_UUID_format() -> Router {
    Router::new()
        .route("/items/not-a-uuid", get(validation_errors_Invalid_UUID_format_handler))
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
            "/models/invalid_model",
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

// Handler functions
async fn multipart_Multiple_values_for_same_field_name_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_19_file_mime_spoofing_png_as_jpeg_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"image\":{\"content_type\":[\"image/jpeg\"],\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"image\":{\"content_type\":[\"image/png\"],\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"document\":{\"content_type\":[\"application/pdf\"],\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"document\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_PDF_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"document\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"document\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_File_list_upload__array_of_files_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"files\":{\"items\":{\"format\":\"binary\",\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"files\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Optional_file_upload___provided_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_File_size_validation___too_large_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Mixed_files_and_form_data_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"active\":{\"type\":\"string\"},\"age\":{\"type\":\"string\"},\"file\":{\"format\":\"binary\",\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Simple_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Empty_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Optional_file_upload___missing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_File_upload_without_filename_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_18_file_magic_number_jpeg_success_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"image\":{\"content_type\":[\"image/jpeg\"],\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"file\":{\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"image\":{\"content_type\":[\"image/png\"],\"source\":\"files\",\"validate_magic_numbers\":true}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"additionalProperties\":false,\"properties\":{\"some\":{\"type\":\"string\"}},\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Multiple_file_uploads_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test1\":{\"format\":\"binary\",\"type\":\"string\"},\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test1\",\"test2\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_File_upload_with_custom_headers_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test2\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"test2\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Required_file_upload___missing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"file\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"file\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn multipart_Image_file_upload_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"image\":{\"format\":\"binary\",\"type\":\"string\"}},\"required\":[\"image\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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

async fn cors_CORS_preflight_request_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cors_CORS_with_credentials_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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

async fn cors_10_cors_origin_null_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

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
    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Origin\":{\"source\":\"headers\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            // Add CORS headers to response
            let mut response =
                (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated)).into_response();
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

async fn cors_CORS_wildcard_origin_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cors_CORS_request_blocked_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // CORS validation
    use spikard_http::CorsConfig;
    use spikard_http::cors::{add_cors_headers, validate_cors_request};

    let cors_config: CorsConfig = serde_json::from_str("{\"allowed_headers\":[\"Content-Type\"],\"allowed_methods\":[\"GET\",\"POST\"],\"allowed_origins\":[\"https://example.com\"]}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {
        return err_response;
    }
    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Origin\":{\"source\":\"headers\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            // Add CORS headers to response
            let mut response =
                (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated)).into_response();
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

async fn cors_Simple_CORS_request_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
    let schema: Value = serde_json::from_str("{\"properties\":{\"Origin\":{\"source\":\"headers\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;

            // Add CORS headers to response
            let mut response =
                (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated)).into_response();
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

async fn url_encoded_Simple_form_submission___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_15_special_characters_field_names_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"contact.email\":{\"format\":\"email\",\"type\":\"string\"},\"user-name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Pattern_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"pattern\":\"^[a-z0-9_]+$\",\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_22_additional_properties_strict_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"theme\":{\"enum\":[\"light\",\"dark\"],\"type\":\"string\"}},\"required\":[\"theme\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_17_pattern_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"account_id\":{\"pattern\":\"^ACC-[0-9]{6}$\",\"type\":\"string\"}},\"required\":[\"account_id\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_20_format_email_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Multiple_values_for_same_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Required_field_missing___validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_13_array_field_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":1,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Numeric_field_type_conversion_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"age\":{\"type\":\"integer\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Special_characters_encoding_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Boolean_field_conversion_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"subscribe\":{\"type\":\"boolean\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Empty_string_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_OAuth2_password_grant_flow_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"grant_type\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"scope\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\",\"grant_type\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_19_array_minitems_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"tags\":{\"items\":{\"type\":\"string\"},\"minItems\":2,\"type\":\"array\"}},\"required\":[\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_Optional_field_missing___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"email\":{\"format\":\"email\",\"type\":[\"string\",\"null\"]},\"password\":{\"type\":\"string\"},\"username\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_14_nested_object_bracket_notation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"user\":{\"properties\":{\"age\":{\"minimum\":0,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_String_max_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"maxLength\":20,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_18_integer_minimum_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"quantity\":{\"minimum\":1,\"type\":\"integer\"}},\"required\":[\"quantity\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_21_integer_type_coercion_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"price\":{\"type\":\"integer\"}},\"required\":[\"price\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_16_minlength_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn url_encoded_String_min_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_UUID_field___invalid_format_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_44_const_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Boolean_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"in_stock\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"in_stock\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Numeric_le_validation___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Deeply_nested_objects_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"type\":\"string\"},\"country\":{\"additionalProperties\":false,\"properties\":{\"code\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"code\"],\"type\":\"object\"},\"street\":{\"type\":\"string\"}},\"required\":[\"street\",\"city\",\"country\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Optional_fields___omitted_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_UUID_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"item_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"item_id\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Date_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"event_date\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"event_date\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_47_maxproperties_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"maxProperties\":3,\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_46_minproperties_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"minProperties\":2,\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_String_min_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Field_type_validation___invalid_type_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_36_oneof_schema_multiple_match_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Nested_object___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"image\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"image\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_41_not_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_String_max_length_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_50_deep_nesting_4_levels_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"user\":{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"address\":{\"properties\":{\"street\":{\"type\":\"string\"}},\"required\":[\"street\"],\"type\":\"object\"}},\"required\":[\"address\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}},\"required\":[\"user\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_48_dependencies_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_PATCH_partial_update_handler(
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Datetime_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"created_at\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_String_pattern_validation___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Extra_fields_ignored__no_additionalProperties_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"another_extra\":{\"type\":\"integer\"},\"extra_field\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"extra_field\",\"another_extra\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_40_anyof_schema_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_39_anyof_schema_multiple_match_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"phone\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Array_of_primitive_values_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"ratings\":{\"items\":{\"type\":\"number\"},\"type\":\"array\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"ratings\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Numeric_ge_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_37_oneof_schema_no_match_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Empty_array_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{},\"type\":\"array\"}},\"required\":[\"name\",\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_38_anyof_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"anyOf\":[{\"required\":[\"email\"]},{\"required\":[\"phone\"]}],\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Empty_JSON_object_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_String_pattern_validation___fail_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"sku\":{\"type\":\"string\"}},\"required\":[\"name\",\"sku\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_49_dependencies_validation_failure_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"dependencies\":{\"credit_card\":[\"billing_address\"]},\"properties\":{\"billing_address\":{\"type\":\"string\"},\"credit_card\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Simple_JSON_object___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"number\"}},\"required\":[\"name\",\"description\",\"price\",\"tax\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Required_field_missing___validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"description\",\"price\",\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_35_oneof_schema_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"oneOf\":[{\"properties\":{\"credit_card\":{\"pattern\":\"^[0-9]{16}$\",\"type\":\"string\"}},\"required\":[\"credit_card\"],\"type\":\"object\"},{\"properties\":{\"paypal_email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"paypal_email\"],\"type\":\"object\"}]}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Enum_field___invalid_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"category\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Enum_field___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"category\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"category\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_33_allof_schema_composition_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"allOf\":[{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"},{\"properties\":{\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"price\"],\"type\":\"object\"}]}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_45_minproperties_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"minProperties\":2,\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Body_with_query_parameters_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"username\":{\"not\":{\"enum\":[\"admin\",\"root\",\"system\"]},\"type\":\"string\"}},\"required\":[\"username\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_43_const_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"data\":{\"type\":\"string\"},\"version\":{\"const\":\"1.0\",\"type\":\"string\"}},\"required\":[\"version\",\"data\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_32_schema_ref_definitions_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"definitions\":{\"Product\":{\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}},\"properties\":{\"product\":{\"$ref\":\"#/definitions/Product\"}},\"required\":[\"product\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_29_nested_object_validation_success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\",\"email\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_34_additional_properties_false_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"email\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Null_value_for_optional_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"null\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tax\":{\"type\":\"null\"}},\"required\":[\"name\",\"price\",\"description\",\"tax\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_31_nullable_property_null_value_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"description\":{\"type\":[\"string\",\"null\"]},\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn json_bodies_Array_of_objects___success_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"images\":{\"items\":{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"url\":{\"type\":\"string\"}},\"required\":[\"url\",\"name\"],\"type\":\"object\"},\"type\":\"array\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"tags\",\"images\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn path_params_Boolean_path_parameter___True_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    uri: axum::http::Uri,
) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_lt_constraint___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_Integer_path_parameter_with_gt_constraint___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter_with_le_constraint___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Path_type_parameter___file_path_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Integer_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Multiple_path_parameters___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_Date_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_Float_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn path_params_String_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_UUID_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_Integer_path_parameter_with_ge_constraint___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_Enum_path_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn path_params_Boolean_path_parameter___numeric_1_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_408_Request_Timeout_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_404_Not_Found___Resource_not_found_handler(
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_503_Service_Unavailable___Server_overload_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_422_Unprocessable_Entity___Validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"price\",\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_302_Found___Temporary_redirect_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_304_Not_Modified___Cached_content_valid_handler(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"If-None-Match\":{\"source\":\"headers\",\"type\":\"string\"},\"code\":{\"source\":\"path\",\"type\":\"string\"}},\"required\":[\"code\"],\"type\":\"object\"}").unwrap();
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_400_Bad_Request___Invalid_request_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"type\":\"string\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_22_501_not_implemented_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 204;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_301_Moved_Permanently___Permanent_redirect_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_201_Created___Resource_created_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_202_Accepted___Request_accepted_for_processing_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"task\":{\"type\":\"string\"}},\"required\":[\"task\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 202;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_307_Temporary_Redirect___Method_preserved_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_500_Internal_Server_Error___Server_error_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_20_414_uri_too_long_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_401_Unauthorized___Missing_authentication_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_23_503_service_unavailable_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_19_413_payload_too_large_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"properties\":{\"data\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn status_codes_403_Forbidden___Insufficient_permissions_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn status_codes_21_431_request_header_fields_too_large_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-Large-Header\":{\"source\":\"headers\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_429_Too_Many_Requests_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn status_codes_206_Partial_Content_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_regex_validation___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_33_api_key_header_valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"headers\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Content_Type_header___application_json_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Accept_Language_header_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_X_API_Key_required_header___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_validation___max_length_constraint_fail_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_X_API_Key_required_header___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Origin_header_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_User_Agent_header___default_value_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_32_bearer_token_missing_prefix_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"headers\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Optional_header_with_None_default___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_regex_validation___fail_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_31_bearer_token_format_invalid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"headers\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_X_API_Key_optional_header___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Authorization_header___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_30_bearer_token_format_valid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Authorization\":{\"pattern\":\"^Bearer [A-Za-z0-9-._~+/]+=*$\",\"source\":\"headers\",\"type\":\"string\"}},\"required\":[\"Authorization\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Authorization_header___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Accept_header___JSON_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Accept_Encoding_header_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Authorization_header___wrong_scheme_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_validation___min_length_constraint_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Basic_authentication___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Bearer_token_authentication___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_X_API_Key_optional_header___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Multiple_custom_headers_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_34_api_key_header_invalid_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"X-API-Key\":{\"pattern\":\"^[a-f0-9]{32}$\",\"source\":\"headers\",\"type\":\"string\"}},\"required\":[\"X-API-Key\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn headers_Bearer_token_authentication___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Host_header_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Referer_header_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_with_underscore_conversion___explicit_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn headers_Header_case_insensitivity___access_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"test\":{\"type\":\"string\"}},\"required\":[\"test\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn headers_User_Agent_header___custom_value_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_25_cookie_samesite_lax_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking\":{\"samesite\":\"Lax\",\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"tracking\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Optional_cookie_parameter___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_Cookie_regex_pattern_validation___fail_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking_id\":{\"pattern\":\"^[A-Z0-9]{8}$\",\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_27_cookie_httponly_flag_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session\":{\"httponly\":true,\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"session\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Response_cookie_with_attributes_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_24_cookie_samesite_strict_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"session_id\":{\"samesite\":\"Strict\",\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_APIKey_cookie_authentication___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_Cookie_validation___min_length_constraint_success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_Cookie_validation___min_length_failure_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"tracking_id\":{\"minLength\":3,\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"tracking_id\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    let schema: Value = serde_json::from_str("{\"properties\":{\"session_id\":{\"maxLength\":20,\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    let schema: Value = serde_json::from_str("{\"properties\":{\"fatebook_tracker\":{\"source\":\"cookies\",\"type\":\"string\"},\"session_id\":{\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"session_id\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn cookies_Optional_cookie_parameter___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_APIKey_cookie_authentication___missing_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"key\":{\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"key\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"session\":{\"type\":\"string\"},\"user\":{\"type\":\"string\"}},\"required\":[\"user\",\"session\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Response_cookie_with_SameSite_Lax_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Response___delete_cookie_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Response_cookie_with_path_attribute_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Optional_APIKey_cookie___missing_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_Response_cookie_with_SameSite_Strict_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Response_cookie_with_SameSite_None_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Cookie_regex_pattern_validation___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_Response_set_cookie___basic_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn cookies_Multiple_cookies___success_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn cookies_26_cookie_secure_flag_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"auth_token\":{\"secure\":true,\"source\":\"cookies\",\"type\":\"string\"}},\"required\":[\"auth_token\"],\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"value\":{\"type\":\"string\"}},\"required\":[\"value\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_415_Unsupported_Media_Type_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"type\":\"string\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_XML_response___application_xml_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_14_content_type_case_insensitive_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_JSON_with_UTF_8_charset_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_16_text_plain_not_accepted_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_PDF_response___application_pdf_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_20_content_length_mismatch_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{\"properties\":{\"Content-Length\":{\"source\":\"headers\",\"type\":\"string\"}},\"required\":[],\"type\":\"object\"}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();
    // Parse body schema and create body validator
    let body_schema: Value =
        serde_json::from_str("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_17_vendor_json_accepted_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"data\":{\"type\":\"string\"}},\"required\":[\"data\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_13_json_with_charset_utf16_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_JSON_response___application_json_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_15_multipart_boundary_required_handler(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse {
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str(
        "{\"properties\":{\"document\":{\"source\":\"files\"}},\"required\":[\"document\"],\"type\":\"object\"}",
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn content_types_HTML_response___text_html_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_JPEG_image_response___image_jpeg_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_19_missing_content_type_default_json_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_PNG_image_response___image_png_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_Plain_text_response___text_plain_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_18_content_type_with_multiple_params_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value =
        serde_json::from_str("{\"properties\":{\"value\":{\"type\":\"string\"}},\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn content_types_CSV_response___text_csv_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn content_types_Binary_response___application_octet_stream_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn http_methods_OPTIONS___CORS_preflight_request_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 201;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &path_params,
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"text\":{\"maxLength\":100,\"minLength\":1,\"type\":\"string\"}},\"required\":[\"text\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"backslashes\":{\"type\":\"string\"},\"empty_string\":{\"type\":\"string\"},\"quotes\":{\"type\":\"string\"},\"special_chars\":{\"format\":\"email\",\"type\":\"string\"},\"tabs_newlines\":{\"type\":\"string\"},\"unicode_escapes\":{\"type\":\"string\"},\"whitespace\":{\"type\":\"string\"}},\"required\":[\"empty_string\",\"whitespace\",\"tabs_newlines\",\"quotes\",\"backslashes\",\"unicode_escapes\",\"special_chars\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_15_float_precision_preservation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"value\":{\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"items\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"items\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_21_scientific_notation_number_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"value\":{\"minimum\":0,\"type\":\"number\"}},\"required\":[\"value\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_Float_precision_and_rounding_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"expected_sum\":{\"type\":\"number\"},\"precise_value\":{\"type\":\"number\"},\"value1\":{\"type\":\"number\"},\"value2\":{\"type\":\"number\"},\"very_large\":{\"type\":\"number\"},\"very_small\":{\"type\":\"number\"}},\"required\":[\"value1\",\"value2\",\"expected_sum\",\"precise_value\",\"very_small\",\"very_large\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_Unicode_and_emoji_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"description\":{\"type\":\"string\"},\"emoji_reactions\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"description\",\"tags\",\"emoji_reactions\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_17_extremely_long_string_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"content\":{\"maxLength\":10000,\"type\":\"string\"}},\"required\":[\"content\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"name\":{\"minLength\":1,\"type\":\"string\"}},\"required\":[\"name\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_20_null_byte_in_string_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"filename\":{\"pattern\":\"^[^\\\\x00]+$\",\"type\":\"string\"}},\"required\":[\"filename\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_23_deeply_nested_json_limit_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"large_int\":{\"type\":\"integer\"},\"max_safe_int\":{\"type\":\"integer\"},\"negative_large\":{\"type\":\"integer\"}},\"required\":[\"max_safe_int\",\"large_int\",\"negative_large\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_Deeply_nested_structure__10__levels_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"level1\":{\"additionalProperties\":false,\"properties\":{\"level2\":{\"additionalProperties\":false,\"properties\":{\"level3\":{\"additionalProperties\":false,\"properties\":{\"level4\":{\"additionalProperties\":false,\"properties\":{\"level5\":{\"additionalProperties\":false,\"properties\":{\"level6\":{\"additionalProperties\":false,\"properties\":{\"level7\":{\"additionalProperties\":false,\"properties\":{\"level8\":{\"additionalProperties\":false,\"properties\":{\"level9\":{\"additionalProperties\":false,\"properties\":{\"level10\":{\"additionalProperties\":false,\"properties\":{\"depth\":{\"type\":\"integer\"},\"value\":{\"type\":\"string\"}},\"required\":[\"value\",\"depth\"],\"type\":\"object\"}},\"required\":[\"level10\"],\"type\":\"object\"}},\"required\":[\"level9\"],\"type\":\"object\"}},\"required\":[\"level8\"],\"type\":\"object\"}},\"required\":[\"level7\"],\"type\":\"object\"}},\"required\":[\"level6\"],\"type\":\"object\"}},\"required\":[\"level5\"],\"type\":\"object\"}},\"required\":[\"level4\"],\"type\":\"object\"}},\"required\":[\"level3\"],\"type\":\"object\"}},\"required\":[\"level2\"],\"type\":\"object\"}},\"required\":[\"level1\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_Empty_and_null_value_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"empty_array\":{\"items\":{},\"type\":\"array\"},\"empty_object\":{\"additionalProperties\":false,\"properties\":{},\"type\":\"object\"},\"empty_string\":{\"type\":\"string\"},\"explicit_null\":{\"type\":\"null\"},\"false_boolean\":{\"type\":\"boolean\"},\"zero_number\":{\"type\":\"integer\"}},\"required\":[\"explicit_null\",\"empty_string\",\"empty_array\",\"empty_object\",\"zero_number\",\"false_boolean\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 200;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn edge_cases_16_negative_zero_handling_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str(
        "{\"properties\":{\"offset\":{\"type\":\"number\"}},\"required\":[\"offset\"],\"type\":\"object\"}",
    )
    .unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
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
        &HashMap::new(),
        &cookies,
    ) {
        Ok(validated) => {
            let status_code = 200;
            (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }
    }
}

async fn validation_errors_Invalid_UUID_format_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Invalid_boolean_value_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Missing_required_query_parameter_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Array_max_items_constraint_violation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Numeric_constraint_violation___gt__greater_than_handler(
    uri: axum::http::Uri,
) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_String_regex_pattern_mismatch_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Invalid_enum_value_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_String_min_length_constraint_violation_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Multiple_validation_errors_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\"},\"quantity\":{\"type\":\"integer\"}},\"required\":[\"name\",\"price\",\"quantity\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_String_max_length_constraint_violation_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Nested_object_validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"seller\":{\"additionalProperties\":false,\"properties\":{\"address\":{\"additionalProperties\":false,\"properties\":{\"city\":{\"type\":\"string\"},\"zip_code\":{\"type\":\"string\"}},\"required\":[\"city\",\"zip_code\"],\"type\":\"object\"},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"address\"],\"type\":\"object\"}},\"required\":[\"name\",\"price\",\"seller\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_10_nested_error_path_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"profile\":{\"properties\":{\"contact\":{\"properties\":{\"email\":{\"format\":\"email\",\"type\":\"string\"}},\"required\":[\"email\"],\"type\":\"object\"}},\"required\":[\"contact\"],\"type\":\"object\"}},\"required\":[\"profile\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Invalid_datetime_format_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"created_at\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\",\"created_at\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Array_item_validation_error_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{\"type\":\"string\"},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Missing_required_body_field_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Body_field_type_error___string_for_float_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"}},\"required\":[\"name\",\"price\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Malformed_JSON_body_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"type\":\"string\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Query_param_type_error___string_provided_for_int_handler(
    uri: axum::http::Uri,
) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Header_validation_error_handler(uri: axum::http::Uri) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_09_multiple_validation_errors_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"properties\":{\"age\":{\"minimum\":18,\"type\":\"integer\"},\"email\":{\"format\":\"email\",\"type\":\"string\"},\"name\":{\"minLength\":3,\"type\":\"string\"}},\"required\":[\"name\",\"email\",\"age\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}

async fn validation_errors_Numeric_constraint_violation___le__less_than_or_equal_handler(
    uri: axum::http::Uri,
) -> Json<Value> {
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {
        parse_query_string_to_json(query_str.as_bytes(), true)
    } else {
        Value::Object(serde_json::Map::new())
    };

    Json(params)
}

async fn validation_errors_Array_min_items_constraint_violation_handler(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{\"additionalProperties\":false,\"properties\":{\"name\":{\"type\":\"string\"},\"price\":{\"type\":\"number\"},\"tags\":{\"items\":{},\"type\":\"array\"}},\"required\":[\"name\",\"price\",\"tags\"],\"type\":\"object\"}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }

    let status_code = 201;
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))
}
