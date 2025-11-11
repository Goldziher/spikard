# frozen_string_literal: true

require 'json'
require 'spikard'

module E2ERubyApp
  module_function

  def build_response(content:, status:, headers: nil)
    headers ||= {}
    Spikard::Response.new(content: content, status_code: status, headers: headers)
  end

  def create_app_auth_1_api_key_authentication_invalid_key
    app = Spikard::App.new
    app.get("/api/data", handler_name: "auth_1_api_key_authentication_invalid_key", parameter_schema: {"properties" => {"X-API-Key" => {"source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "The provided API key is not valid", "status" => 401, "title" => "Invalid API key", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_2_api_key_authentication_missing_header
    app = Spikard::App.new
    app.get("/api/data", handler_name: "auth_2_api_key_authentication_missing_header") do |_request|
      build_response(content: {"detail" => "Expected \'X-API-Key\' header with valid API key", "status" => 401, "title" => "Missing API key", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_3_api_key_authentication_valid_key
    app = Spikard::App.new
    app.get("/api/data", handler_name: "auth_3_api_key_authentication_valid_key", parameter_schema: {"properties" => {"X-API-Key" => {"description" => "API key for authentication", "source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: {"data" => "sensitive information", "message" => "Access granted"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_auth_4_jwt_authentication_expired_token
    app = Spikard::App.new
    app.get("/protected/user", handler_name: "auth_4_jwt_authentication_expired_token", parameter_schema: {"properties" => {"Authorization" => {"source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "Token has expired", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_5_jwt_authentication_invalid_audience
    app = Spikard::App.new
    app.get("/protected/user", handler_name: "auth_5_jwt_authentication_invalid_audience", parameter_schema: {"properties" => {"Authorization" => {"source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "Token audience is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_6_jwt_authentication_invalid_signature
    app = Spikard::App.new
    app.get("/protected/user", handler_name: "auth_6_jwt_authentication_invalid_signature", parameter_schema: {"properties" => {"Authorization" => {"source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "Token signature is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_7_jwt_authentication_missing_authorization_header
    app = Spikard::App.new
    app.get("/protected/user", handler_name: "auth_7_jwt_authentication_missing_authorization_header") do |_request|
      build_response(content: {"detail" => "Expected \'Authorization: Bearer <token>\'", "status" => 401, "title" => "Missing or invalid Authorization header", "type" => "https://spikard.dev/errors/unauthorized"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_auth_8_jwt_authentication_valid_token
    app = Spikard::App.new
    app.get("/protected/user", handler_name: "auth_8_jwt_authentication_valid_token", parameter_schema: {"properties" => {"Authorization" => {"description" => "JWT token in Bearer format", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Access granted", "user_id" => "user123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_1_13_json_with_charset_utf16
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_1_13_json_with_charset_utf16", request_schema: {"properties" => {"value" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"error" => "Unsupported charset \'utf-16\' for JSON. Only UTF-8 is supported."}, status: 415, headers: nil)
    end
    app
  end

  def create_app_content_types_2_14_content_type_case_insensitive
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_2_14_content_type_case_insensitive", request_schema: {"properties" => {"name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "test"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_content_types_3_15_multipart_boundary_required
    app = Spikard::App.new
    app.post("/upload", handler_name: "content_types_3_15_multipart_boundary_required", file_params: {"document" => {"required" => true}}) do |_request|
      build_response(content: {"error" => "multipart/form-data requires \'boundary\' parameter"}, status: 400, headers: nil)
    end
    app
  end

  def create_app_content_types_4_16_text_plain_not_accepted
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_4_16_text_plain_not_accepted", request_schema: {"properties" => {"data" => {"type" => "string"}}, "required" => ["data"], "type" => "object"}) do |_request|
      build_response(content: {"error" => "Unsupported Media Type. Expected application/json"}, status: 415, headers: nil)
    end
    app
  end

  def create_app_content_types_5_17_vendor_json_accepted
    app = Spikard::App.new
    app.post("/api/v1/resource", handler_name: "content_types_5_17_vendor_json_accepted", request_schema: {"properties" => {"data" => {"type" => "string"}}, "required" => ["data"], "type" => "object"}) do |_request|
      build_response(content: {"data" => "value"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_content_types_6_18_content_type_with_multiple_params
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_6_18_content_type_with_multiple_params", request_schema: {"properties" => {"value" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"value" => "test"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_content_types_7_19_missing_content_type_default_json
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_7_19_missing_content_type_default_json", request_schema: {"properties" => {"name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "test"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_content_types_8_20_content_length_mismatch
    app = Spikard::App.new
    app.post("/data", handler_name: "content_types_8_20_content_length_mismatch", parameter_schema: {"properties" => {"Content-Length" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}, request_schema: {"properties" => {"value" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"error" => "Content-Length header does not match actual body size"}, status: 400, headers: nil)
    end
    app
  end

  def create_app_content_types_9_415_unsupported_media_type
    app = Spikard::App.new
    app.post("/items/", handler_name: "content_types_9_415_unsupported_media_type", request_schema: {"type" => "string"}) do |_request|
      build_response(content: {"detail" => "Unsupported media type"}, status: 415, headers: nil)
    end
    app
  end

  def create_app_content_types_10_binary_response_application_octet_stream
    app = Spikard::App.new
    app.get("/download/file.bin", handler_name: "content_types_10_binary_response_application_octet_stream") do |_request|
      build_response(content: "binary_data_placeholder", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_11_csv_response_text_csv
    app = Spikard::App.new
    app.get("/export/data.csv", handler_name: "content_types_11_csv_response_text_csv") do |_request|
      build_response(content: "id,name,price\n1,Item A,10.0\n2,Item B,20.0", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_12_content_negotiation_accept_header
    app = Spikard::App.new
    app.get("/accept-test/{id}", handler_name: "content_types_12_content_negotiation_accept_header", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "name" => "Item"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_13_html_response_text_html
    app = Spikard::App.new
    app.get("/html", handler_name: "content_types_13_html_response_text_html") do |_request|
      build_response(content: "<html><body><h1>Hello</h1></body></html>", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_14_jpeg_image_response_image_jpeg
    app = Spikard::App.new
    app.get("/images/photo.jpg", handler_name: "content_types_14_jpeg_image_response_image_jpeg") do |_request|
      build_response(content: "jpeg_binary_data", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_15_json_response_application_json
    app = Spikard::App.new
    app.get("/items/json", handler_name: "content_types_15_json_response_application_json") do |_request|
      build_response(content: {"name" => "Item", "price" => 42.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_16_json_with_utf_8_charset
    app = Spikard::App.new
    app.get("/items/unicode", handler_name: "content_types_16_json_with_utf_8_charset") do |_request|
      build_response(content: {"emoji" => "\u{2615}", "name" => "Caf\u{e9}"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_17_pdf_response_application_pdf
    app = Spikard::App.new
    app.get("/download/document.pdf", handler_name: "content_types_17_pdf_response_application_pdf") do |_request|
      build_response(content: "pdf_binary_data", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_18_png_image_response_image_png
    app = Spikard::App.new
    app.get("/images/logo.png", handler_name: "content_types_18_png_image_response_image_png") do |_request|
      build_response(content: "png_binary_data", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_19_plain_text_response_text_plain
    app = Spikard::App.new
    app.get("/text", handler_name: "content_types_19_plain_text_response_text_plain") do |_request|
      build_response(content: "Hello, World!", status: 200, headers: nil)
    end
    app
  end

  def create_app_content_types_20_xml_response_application_xml
    app = Spikard::App.new
    app.get("/xml", handler_name: "content_types_20_xml_response_application_xml") do |_request|
      build_response(content: "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>", status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_1_24_cookie_samesite_strict
    app = Spikard::App.new
    app.get("/secure", handler_name: "cookies_1_24_cookie_samesite_strict", parameter_schema: {"properties" => {"session_id" => {"samesite" => "Strict", "source" => "cookie", "type" => "string"}}, "required" => ["session_id"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_2_25_cookie_samesite_lax
    app = Spikard::App.new
    app.get("/data", handler_name: "cookies_2_25_cookie_samesite_lax", parameter_schema: {"properties" => {"tracking" => {"samesite" => "Lax", "source" => "cookie", "type" => "string"}}, "required" => ["tracking"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_3_26_cookie_secure_flag
    app = Spikard::App.new
    app.get("/secure", handler_name: "cookies_3_26_cookie_secure_flag", parameter_schema: {"properties" => {"auth_token" => {"secure" => true, "source" => "cookie", "type" => "string"}}, "required" => ["auth_token"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_4_27_cookie_httponly_flag
    app = Spikard::App.new
    app.get("/secure", handler_name: "cookies_4_27_cookie_httponly_flag", parameter_schema: {"properties" => {"session" => {"httponly" => true, "source" => "cookie", "type" => "string"}}, "required" => ["session"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_5_apikey_cookie_authentication_missing
    app = Spikard::App.new
    app.get("/users/me/auth", handler_name: "cookies_5_apikey_cookie_authentication_missing", parameter_schema: {"properties" => {"key" => {"source" => "cookie", "type" => "string"}}, "required" => ["key"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["cookie", "key"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_cookies_6_apikey_cookie_authentication_success
    app = Spikard::App.new
    app.get("/users/me", handler_name: "cookies_6_apikey_cookie_authentication_success", parameter_schema: {"properties" => {"key" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"username" => "secret"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_7_cookie_regex_pattern_validation_fail
    app = Spikard::App.new
    app.get("/cookies/pattern", handler_name: "cookies_7_cookie_regex_pattern_validation_fail", parameter_schema: {"properties" => {"tracking_id" => {"pattern" => "^[A-Z0-9]{8}$", "source" => "cookie", "type" => "string"}}, "required" => ["tracking_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[A-Z0-9]{8}$"}, "input" => "invalid-format", "loc" => ["cookie", "tracking_id"], "msg" => "String should match pattern \'^[A-Z0-9]{8}$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_cookies_8_cookie_regex_pattern_validation_success
    app = Spikard::App.new
    app.get("/cookies/pattern", handler_name: "cookies_8_cookie_regex_pattern_validation_success", parameter_schema: {"properties" => {"tracking_id" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"tracking_id" => "ABC12345"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_9_cookie_validation_max_length_constraint_fail
    app = Spikard::App.new
    app.get("/cookies/validated", handler_name: "cookies_9_cookie_validation_max_length_constraint_fail", parameter_schema: {"properties" => {"session_id" => {"maxLength" => 20, "source" => "cookie", "type" => "string"}}, "required" => ["session_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 20}, "input" => "this_cookie_value_is_way_too_long", "loc" => ["cookie", "session_id"], "msg" => "String should have at most 20 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_cookies_10_cookie_validation_min_length_constraint_success
    app = Spikard::App.new
    app.get("/cookies/min-length", handler_name: "cookies_10_cookie_validation_min_length_constraint_success", parameter_schema: {"properties" => {"token" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"token" => "abc"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_11_cookie_validation_min_length_failure
    app = Spikard::App.new
    app.get("/items/", handler_name: "cookies_11_cookie_validation_min_length_failure", parameter_schema: {"properties" => {"tracking_id" => {"minLength" => 3, "source" => "cookie", "type" => "string"}}, "required" => ["tracking_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "", "loc" => ["cookie", "tracking_id"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_cookies_12_multiple_cookies_success
    app = Spikard::App.new
    app.get("/items/", handler_name: "cookies_12_multiple_cookies_success", parameter_schema: {"properties" => {"fatebook_tracker" => {"source" => "cookie", "type" => "string"}, "googall_tracker" => {"source" => "cookie", "type" => "string"}, "session_id" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"fatebook_tracker" => "tracker456", "googall_tracker" => "ga789", "session_id" => "session123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_13_optional_apikey_cookie_missing
    app = Spikard::App.new
    app.get("/users/me", handler_name: "cookies_13_optional_apikey_cookie_missing", parameter_schema: {"properties" => {"key" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"msg" => "Create an account first"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_14_optional_cookie_parameter_missing
    app = Spikard::App.new
    app.get("/items/", handler_name: "cookies_14_optional_cookie_parameter_missing", parameter_schema: {"properties" => {"ads_id" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"ads_id" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_15_optional_cookie_parameter_success
    app = Spikard::App.new
    app.get("/items/", handler_name: "cookies_15_optional_cookie_parameter_success", parameter_schema: {"properties" => {"ads_id" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"ads_id" => "abc123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_16_required_cookie_missing
    app = Spikard::App.new
    app.get("/items/cookies", handler_name: "cookies_16_required_cookie_missing", parameter_schema: {"properties" => {"fatebook_tracker" => {"source" => "cookie", "type" => "string"}, "session_id" => {"source" => "cookie", "type" => "string"}}, "required" => ["session_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "", "loc" => ["cookie", "session_id"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_cookies_17_response_delete_cookie
    app = Spikard::App.new
    app.post("/cookies/delete", handler_name: "cookies_17_response_delete_cookie", parameter_schema: {"properties" => {"session" => {"source" => "cookie", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie deleted"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_18_response_multiple_cookies
    app = Spikard::App.new
    app.post("/cookies/multiple", handler_name: "cookies_18_response_multiple_cookies", request_schema: {"additionalProperties" => false, "properties" => {"session" => {"type" => "string"}, "user" => {"type" => "string"}}, "required" => ["user", "session"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Multiple cookies set"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_19_response_session_cookie_no_max_age
    app = Spikard::App.new
    app.post("/cookies/session", handler_name: "cookies_19_response_session_cookie_no_max_age", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Session cookie set"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_20_response_cookie_with_samesite_lax
    app = Spikard::App.new
    app.post("/cookies/samesite-lax", handler_name: "cookies_20_response_cookie_with_samesite_lax", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie set with SameSite=Lax"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_21_response_cookie_with_samesite_none
    app = Spikard::App.new
    app.post("/cookies/samesite-none", handler_name: "cookies_21_response_cookie_with_samesite_none", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie set with SameSite=None"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_22_response_cookie_with_samesite_strict
    app = Spikard::App.new
    app.post("/cookies/samesite-strict", handler_name: "cookies_22_response_cookie_with_samesite_strict", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie set with SameSite=Strict"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_23_response_cookie_with_attributes
    app = Spikard::App.new
    app.get("/cookie/set", handler_name: "cookies_23_response_cookie_with_attributes") do |_request|
      build_response(content: {"message" => "Cookie set"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_24_response_cookie_with_domain_attribute
    app = Spikard::App.new
    app.post("/cookies/set-with-domain", handler_name: "cookies_24_response_cookie_with_domain_attribute", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie set with domain"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_25_response_cookie_with_path_attribute
    app = Spikard::App.new
    app.post("/cookies/set-with-path", handler_name: "cookies_25_response_cookie_with_path_attribute", request_schema: {"additionalProperties" => false, "properties" => {"value" => {"type" => "string"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Cookie set with path"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cookies_26_response_set_cookie_basic
    app = Spikard::App.new
    app.post("/cookie/", handler_name: "cookies_26_response_set_cookie_basic") do |_request|
      build_response(content: {"message" => "Come to the dark side, we have cookies"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cors_1_08_cors_max_age
    app = Spikard::App.new
    app.post("/api/data", handler_name: "cors_1_08_cors_max_age", parameter_schema: {"properties" => {"Access-Control-Request-Headers" => {"source" => "header", "type" => "string"}, "Access-Control-Request-Method" => {"source" => "header", "type" => "string"}, "Origin" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}, cors: {"allowed_headers" => ["Content-Type"], "allowed_methods" => ["POST"], "allowed_origins" => ["https://example.com"], "max_age" => 3600}) do |_request|
      build_response(content: nil, status: 204, headers: nil)
    end
    app
  end

  def create_app_cors_2_09_cors_expose_headers
    app = Spikard::App.new
    app.get("/api/data", handler_name: "cors_2_09_cors_expose_headers", parameter_schema: {"properties" => {"Origin" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}, cors: {"allowed_methods" => ["GET"], "allowed_origins" => ["https://example.com"], "expose_headers" => ["X-Total-Count", "X-Request-Id"]}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cors_3_10_cors_origin_null
    app = Spikard::App.new
    app.get("/api/data", handler_name: "cors_3_10_cors_origin_null", parameter_schema: {"properties" => {"Origin" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}, cors: {"allowed_methods" => ["GET"], "allowed_origins" => ["https://example.com"]}) do |_request|
      build_response(content: {"error" => "Origin \'null\' is not allowed"}, status: 403, headers: nil)
    end
    app
  end

  def create_app_cors_4_cors_preflight_request
    app = Spikard::App.new
    app.options("/items/", handler_name: "cors_4_cors_preflight_request") do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_cors_5_cors_request_blocked
    app = Spikard::App.new
    app.get("/items/", handler_name: "cors_5_cors_request_blocked", parameter_schema: {"properties" => {"Origin" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}, cors: {"allowed_headers" => ["Content-Type"], "allowed_methods" => ["GET", "POST"], "allowed_origins" => ["https://example.com"]}) do |_request|
      build_response(content: {"detail" => "CORS request from origin \'https://malicious-site.com\' not allowed"}, status: 403, headers: nil)
    end
    app
  end

  def create_app_cors_6_cors_wildcard_origin
    app = Spikard::App.new
    app.get("/public/data", handler_name: "cors_6_cors_wildcard_origin") do |_request|
      build_response(content: {"data" => "public"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cors_7_cors_with_credentials
    app = Spikard::App.new
    app.get("/api/user/profile", handler_name: "cors_7_cors_with_credentials") do |_request|
      build_response(content: {"username" => "john"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_cors_8_simple_cors_request
    app = Spikard::App.new
    app.get("/items/", handler_name: "cors_8_simple_cors_request") do |_request|
      build_response(content: {"items" => []}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_1_11_utf8_query_parameter
    app = Spikard::App.new
    app.get("/search", handler_name: "edge_cases_1_11_utf8_query_parameter", parameter_schema: {"properties" => {"term" => {"source" => "query", "type" => "string"}}, "required" => ["term"], "type" => "object"}) do |_request|
      build_response(content: {"term" => "caf\u{e9}"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_2_12_percent_encoded_special_chars
    app = Spikard::App.new
    app.get("/search", handler_name: "edge_cases_2_12_percent_encoded_special_chars", parameter_schema: {"properties" => {"term" => {"source" => "query", "type" => "string"}}, "required" => ["term"], "type" => "object"}) do |_request|
      build_response(content: {"term" => "hi there"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_3_13_empty_string_query_param_preserved
    app = Spikard::App.new
    app.get("/items", handler_name: "edge_cases_3_13_empty_string_query_param_preserved", parameter_schema: {"properties" => {"filter" => {"source" => "query", "type" => "string"}}, "required" => ["filter"], "type" => "object"}) do |_request|
      build_response(content: {"filter" => ""}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_4_14_large_integer_boundary
    app = Spikard::App.new
    app.get("/items", handler_name: "edge_cases_4_14_large_integer_boundary", parameter_schema: {"properties" => {"id" => {"source" => "query", "type" => "integer"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 9007199254740991}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_5_15_float_precision_preservation
    app = Spikard::App.new
    app.post("/calculate", handler_name: "edge_cases_5_15_float_precision_preservation", request_schema: {"properties" => {"value" => {"type" => "number"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 3.141592653589793}, status: 201, headers: nil)
    end
    app
  end

  def create_app_edge_cases_6_16_negative_zero_handling
    app = Spikard::App.new
    app.post("/data", handler_name: "edge_cases_6_16_negative_zero_handling", request_schema: {"properties" => {"offset" => {"type" => "number"}}, "required" => ["offset"], "type" => "object"}) do |_request|
      build_response(content: {"offset" => 0}, status: 201, headers: nil)
    end
    app
  end

  def create_app_edge_cases_7_17_extremely_long_string
    app = Spikard::App.new
    app.post("/text", handler_name: "edge_cases_7_17_extremely_long_string", request_schema: {"properties" => {"content" => {"maxLength" => 10000, "type" => "string"}}, "required" => ["content"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 10001, "max_length" => 10000}, "loc" => ["body", "content"], "msg" => "String length must not exceed 10000", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_edge_cases_8_18_unicode_normalization
    app = Spikard::App.new
    app.post("/users", handler_name: "edge_cases_8_18_unicode_normalization", request_schema: {"properties" => {"name" => {"minLength" => 1, "type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "caf\u{e9}"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_edge_cases_9_19_emoji_in_strings
    app = Spikard::App.new
    app.post("/messages", handler_name: "edge_cases_9_19_emoji_in_strings", request_schema: {"properties" => {"text" => {"maxLength" => 100, "minLength" => 1, "type" => "string"}}, "required" => ["text"], "type" => "object"}) do |_request|
      build_response(content: {"text" => "Hello \u{1f44b} World \u{1f30d}"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_edge_cases_10_20_null_byte_in_string
    app = Spikard::App.new
    app.post("/files", handler_name: "edge_cases_10_20_null_byte_in_string", request_schema: {"properties" => {"filename" => {"pattern" => "^[^\\x00]+$", "type" => "string"}}, "required" => ["filename"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"value" => "file\\u0000.txt"}, "loc" => ["body", "filename"], "msg" => "String contains null byte character", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_edge_cases_11_21_scientific_notation_number
    app = Spikard::App.new
    app.post("/calculate", handler_name: "edge_cases_11_21_scientific_notation_number", request_schema: {"properties" => {"value" => {"minimum" => 0, "type" => "number"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 123000}, status: 201, headers: nil)
    end
    app
  end

  def create_app_edge_cases_12_22_leading_zeros_integer
    app = Spikard::App.new
    app.get("/data", handler_name: "edge_cases_12_22_leading_zeros_integer", parameter_schema: {"properties" => {"value" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 123}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_13_23_deeply_nested_json_limit
    app = Spikard::App.new
    app.post("/data", handler_name: "edge_cases_13_23_deeply_nested_json_limit", request_schema: {"type" => "object"}) do |_request|
      build_response(content: {"error" => "Request body exceeds maximum nesting depth of 32"}, status: 400, headers: nil)
    end
    app
  end

  def create_app_edge_cases_14_24_array_with_holes
    app = Spikard::App.new
    app.post("/items", handler_name: "edge_cases_14_24_array_with_holes", request_schema: {"properties" => {"items" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["items"], "type" => "object"}) do |_request|
      build_response(content: {"items" => ["first", "third", "sixth"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_15_deeply_nested_structure_10_levels
    app = Spikard::App.new
    app.post("/nested/", handler_name: "edge_cases_15_deeply_nested_structure_10_levels", request_schema: {"additionalProperties" => false, "properties" => {"level1" => {"additionalProperties" => false, "properties" => {"level2" => {"additionalProperties" => false, "properties" => {"level3" => {"additionalProperties" => false, "properties" => {"level4" => {"additionalProperties" => false, "properties" => {"level5" => {"additionalProperties" => false, "properties" => {"level6" => {"additionalProperties" => false, "properties" => {"level7" => {"additionalProperties" => false, "properties" => {"level8" => {"additionalProperties" => false, "properties" => {"level9" => {"additionalProperties" => false, "properties" => {"level10" => {"additionalProperties" => false, "properties" => {"depth" => {"type" => "integer"}, "value" => {"type" => "string"}}, "required" => ["value", "depth"], "type" => "object"}}, "required" => ["level10"], "type" => "object"}}, "required" => ["level9"], "type" => "object"}}, "required" => ["level8"], "type" => "object"}}, "required" => ["level7"], "type" => "object"}}, "required" => ["level6"], "type" => "object"}}, "required" => ["level5"], "type" => "object"}}, "required" => ["level4"], "type" => "object"}}, "required" => ["level3"], "type" => "object"}}, "required" => ["level2"], "type" => "object"}}, "required" => ["level1"], "type" => "object"}) do |_request|
      build_response(content: {"max_depth" => 10, "message" => "Processed deeply nested structure", "value_found" => "deep"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_16_empty_and_null_value_handling
    app = Spikard::App.new
    app.post("/nulls/", handler_name: "edge_cases_16_empty_and_null_value_handling", request_schema: {"additionalProperties" => false, "properties" => {"empty_array" => {"items" => {}, "type" => "array"}, "empty_object" => {"additionalProperties" => false, "properties" => {}, "type" => "object"}, "empty_string" => {"type" => "string"}, "explicit_null" => {"type" => "null"}, "false_boolean" => {"type" => "boolean"}, "zero_number" => {"type" => "integer"}}, "required" => ["explicit_null", "empty_string", "empty_array", "empty_object", "zero_number", "false_boolean"], "type" => "object"}) do |_request|
      build_response(content: {"empty_array_length" => 0, "empty_object_keys" => 0, "empty_string_length" => 0, "explicit_null_is_null" => true, "false_is_false" => true, "zero_is_falsy" => true}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_17_float_precision_and_rounding
    app = Spikard::App.new
    app.post("/calculations/", handler_name: "edge_cases_17_float_precision_and_rounding", request_schema: {"additionalProperties" => false, "properties" => {"expected_sum" => {"type" => "number"}, "precise_value" => {"type" => "number"}, "value1" => {"type" => "number"}, "value2" => {"type" => "number"}, "very_large" => {"type" => "number"}, "very_small" => {"type" => "number"}}, "required" => ["value1", "value2", "expected_sum", "precise_value", "very_small", "very_large"], "type" => "object"}) do |_request|
      build_response(content: {"precise_value" => 3.141592653589793, "sum" => 0.30000000000000004, "very_large" => 1.7976931348623157e308, "very_small" => 1e-10}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_18_large_integer_boundary_values
    app = Spikard::App.new
    app.post("/numbers/", handler_name: "edge_cases_18_large_integer_boundary_values", request_schema: {"additionalProperties" => false, "properties" => {"large_int" => {"type" => "integer"}, "max_safe_int" => {"type" => "integer"}, "negative_large" => {"type" => "integer"}}, "required" => ["max_safe_int", "large_int", "negative_large"], "type" => "object"}) do |_request|
      build_response(content: {"large_int" => 9223372036854775807, "max_safe_int" => 9007199254740991, "negative_large" => -9223372036854775808}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_19_special_string_values_and_escaping
    app = Spikard::App.new
    app.post("/strings/", handler_name: "edge_cases_19_special_string_values_and_escaping", request_schema: {"additionalProperties" => false, "properties" => {"backslashes" => {"type" => "string"}, "empty_string" => {"type" => "string"}, "quotes" => {"type" => "string"}, "special_chars" => {"type" => "string"}, "tabs_newlines" => {"type" => "string"}, "unicode_escapes" => {"type" => "string"}, "whitespace" => {"type" => "string"}}, "required" => ["empty_string", "whitespace", "tabs_newlines", "quotes", "backslashes", "unicode_escapes", "special_chars"], "type" => "object"}) do |_request|
      build_response(content: {"backslashes" => "C:\\\\Users\\\\Path", "empty_string" => "", "quotes" => "He said \"hello\" and \'goodbye\'", "special_chars" => "!@#$%^&*()_+-=[]{}|;\':\",./<>?", "tabs_newlines" => "line1\n\tline2\r\nline3", "unicode_escapes" => "Hello", "whitespace" => "   "}, status: 200, headers: nil)
    end
    app
  end

  def create_app_edge_cases_20_unicode_and_emoji_handling
    app = Spikard::App.new
    app.post("/items/", handler_name: "edge_cases_20_unicode_and_emoji_handling", request_schema: {"additionalProperties" => false, "properties" => {"description" => {"type" => "string"}, "emoji_reactions" => {"type" => "string"}, "name" => {"type" => "string"}, "tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["name", "description", "tags", "emoji_reactions"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "Best caf\u{e9} in M\u{fc}nchen \u{1f1e9}\u{1f1ea}", "emoji_reactions" => "\u{1f44d}\u{2764}\u{fe0f}\u{1f602}\u{1f389}", "id" => 1, "name" => "Coffee Shop \u{2615}", "tags" => ["\u{98df}\u{3079}\u{7269}", "\u{97f3}\u{697d}", "\u{1f4b0}"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_1_30_bearer_token_format_valid
    app = Spikard::App.new
    app.get("/protected", handler_name: "headers_1_30_bearer_token_format_valid", parameter_schema: {"properties" => {"Authorization" => {"pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_2_31_bearer_token_format_invalid
    app = Spikard::App.new
    app.get("/protected", handler_name: "headers_2_31_bearer_token_format_invalid", parameter_schema: {"properties" => {"Authorization" => {"pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$", "value" => "Bearer invalid token with spaces"}, "loc" => ["headers", "authorization"], "msg" => "Invalid Bearer token format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_3_32_bearer_token_missing_prefix
    app = Spikard::App.new
    app.get("/protected", handler_name: "headers_3_32_bearer_token_missing_prefix", parameter_schema: {"properties" => {"Authorization" => {"pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$", "value" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"}, "loc" => ["headers", "authorization"], "msg" => "Invalid Bearer token format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_4_33_api_key_header_valid
    app = Spikard::App.new
    app.get("/api/data", handler_name: "headers_4_33_api_key_header_valid", parameter_schema: {"properties" => {"X-API-Key" => {"pattern" => "^[a-f0-9]{32}$", "source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_5_34_api_key_header_invalid
    app = Spikard::App.new
    app.get("/api/data", handler_name: "headers_5_34_api_key_header_invalid", parameter_schema: {"properties" => {"X-API-Key" => {"pattern" => "^[a-f0-9]{32}$", "source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-f0-9]{32}$", "value" => "invalid-key"}, "loc" => ["headers", "x-api-key"], "msg" => "Invalid API key format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_6_accept_header_json
    app = Spikard::App.new
    app.get("/headers/accept", handler_name: "headers_6_accept_header_json", parameter_schema: {"properties" => {"Accept" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Accept"], "type" => "object"}) do |_request|
      build_response(content: {"accept" => "application/json"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_7_accept_encoding_header
    app = Spikard::App.new
    app.get("/headers/accept-encoding", handler_name: "headers_7_accept_encoding_header", parameter_schema: {"properties" => {"Accept-Encoding" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Accept-Encoding"], "type" => "object"}) do |_request|
      build_response(content: {"accept_encoding" => "gzip, deflate, br"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_8_accept_language_header
    app = Spikard::App.new
    app.get("/headers/accept-language", handler_name: "headers_8_accept_language_header", parameter_schema: {"properties" => {"Accept-Language" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Accept-Language"], "type" => "object"}) do |_request|
      build_response(content: {"accept_language" => "en-US,en;q=0.9"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_9_authorization_header_missing
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_9_authorization_header_missing", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["headers", "authorization"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_10_authorization_header_success
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_10_authorization_header_success", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"credentials" => "foobar", "scheme" => "Digest"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_11_authorization_header_wrong_scheme
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_11_authorization_header_wrong_scheme", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "pattern" => "^Digest .+", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "Other invalidauthorization", "loc" => ["headers", "authorization"], "msg" => "String should match pattern \'^Digest .+\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_12_basic_authentication_success
    app = Spikard::App.new
    app.get("/headers/basic-auth", handler_name: "headers_12_basic_authentication_success", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"password" => "password", "username" => "username"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_13_bearer_token_authentication_missing
    app = Spikard::App.new
    app.get("/headers/bearer-auth", handler_name: "headers_13_bearer_token_authentication_missing", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "pattern" => "^Bearer .+", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["headers", "authorization"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_14_bearer_token_authentication_success
    app = Spikard::App.new
    app.get("/headers/bearer-auth", handler_name: "headers_14_bearer_token_authentication_success", parameter_schema: {"properties" => {"Authorization" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: {"token" => "valid_token_123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_15_content_type_header_application_json
    app = Spikard::App.new
    app.get("/headers/content-type", handler_name: "headers_15_content_type_header_application_json", parameter_schema: {"properties" => {"Content-Type" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Content-Type"], "type" => "object"}) do |_request|
      build_response(content: {"content_type" => "application/json"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_16_header_case_insensitivity_access
    app = Spikard::App.new
    app.post("/echo", handler_name: "headers_16_header_case_insensitivity_access", request_schema: {"additionalProperties" => false, "properties" => {"test" => {"type" => "string"}}, "required" => ["test"], "type" => "object"}) do |_request|
      build_response(content: {"content_type_lower" => "application/json", "content_type_mixed" => "application/json", "content_type_upper" => "application/json"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_17_header_regex_validation_fail
    app = Spikard::App.new
    app.get("/headers/pattern", handler_name: "headers_17_header_regex_validation_fail", parameter_schema: {"properties" => {"X-Request-Id" => {"annotation" => "str", "pattern" => "^[0-9]{3,}$", "source" => "header", "type" => "string"}}, "required" => ["X-Request-Id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[0-9]{3,}$"}, "input" => "invalid-format", "loc" => ["headers", "x-request-id"], "msg" => "String should match pattern \'^[0-9]{3,}$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_18_header_regex_validation_success
    app = Spikard::App.new
    app.get("/headers/pattern", handler_name: "headers_18_header_regex_validation_success", parameter_schema: {"properties" => {"X-Request-Id" => {"annotation" => "str", "pattern" => "^[0-9]{3,}$", "source" => "header", "type" => "string"}}, "required" => ["X-Request-Id"], "type" => "object"}) do |_request|
      build_response(content: {"x_request_id" => "12345"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_19_header_validation_max_length_constraint_fail
    app = Spikard::App.new
    app.get("/headers/max-length", handler_name: "headers_19_header_validation_max_length_constraint_fail", parameter_schema: {"properties" => {"X-Session-Id" => {"annotation" => "str", "maxLength" => 20, "source" => "header", "type" => "string"}}, "required" => ["X-Session-Id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 20}, "input" => "this_is_way_too_long_for_validation", "loc" => ["headers", "x-session-id"], "msg" => "String should have at most 20 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_20_header_validation_min_length_constraint
    app = Spikard::App.new
    app.get("/headers/validated", handler_name: "headers_20_header_validation_min_length_constraint", parameter_schema: {"properties" => {"X-Token" => {"annotation" => "str", "minLength" => 3, "source" => "header", "type" => "string"}}, "required" => ["X-Token"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["headers", "x-token"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_21_header_with_underscore_conversion_explicit
    app = Spikard::App.new
    app.get("/headers/underscore", handler_name: "headers_21_header_with_underscore_conversion_explicit", parameter_schema: {"properties" => {"X-Token" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["X-Token"], "type" => "object"}) do |_request|
      build_response(content: {"x_token" => "secret123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_22_host_header
    app = Spikard::App.new
    app.get("/headers/host", handler_name: "headers_22_host_header", parameter_schema: {"properties" => {"Host" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Host"], "type" => "object"}) do |_request|
      build_response(content: {"host" => "example.com:8080"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_23_multiple_custom_headers
    app = Spikard::App.new
    app.get("/headers/multiple", handler_name: "headers_23_multiple_custom_headers", parameter_schema: {"properties" => {"X-Client-Version" => {"annotation" => "str", "source" => "header", "type" => "string"}, "X-Request-Id" => {"annotation" => "str", "source" => "header", "type" => "string"}, "X-Trace-Id" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["X-Client-Version", "X-Request-Id", "X-Trace-Id"], "type" => "object"}) do |_request|
      build_response(content: {"x_client_version" => "1.2.3", "x_request_id" => "req-12345", "x_trace_id" => "trace-abc"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_24_optional_header_with_none_default_missing
    app = Spikard::App.new
    app.get("/items/", handler_name: "headers_24_optional_header_with_none_default_missing", parameter_schema: {"properties" => {"strange-header" => {"annotation" => "str", "default" => nil, "source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"strange_header" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_25_origin_header
    app = Spikard::App.new
    app.get("/headers/origin", handler_name: "headers_25_origin_header", parameter_schema: {"properties" => {"Origin" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Origin"], "type" => "object"}) do |_request|
      build_response(content: {"origin" => "https://example.com"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_26_referer_header
    app = Spikard::App.new
    app.get("/headers/referer", handler_name: "headers_26_referer_header", parameter_schema: {"properties" => {"Referer" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["Referer"], "type" => "object"}) do |_request|
      build_response(content: {"referer" => "https://example.com/page"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_27_user_agent_header_custom_value
    app = Spikard::App.new
    app.get("/items/", handler_name: "headers_27_user_agent_header_custom_value", parameter_schema: {"properties" => {"User-Agent" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["User-Agent"], "type" => "object"}) do |_request|
      build_response(content: {"User-Agent" => "Mozilla/5.0 Custom Browser"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_28_user_agent_header_default_value
    app = Spikard::App.new
    app.get("/items/", handler_name: "headers_28_user_agent_header_default_value", parameter_schema: {"properties" => {"User-Agent" => {"annotation" => "str", "default" => "testclient", "source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"User-Agent" => "testclient"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_29_x_api_key_optional_header_missing
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_29_x_api_key_optional_header_missing", parameter_schema: {"properties" => {"key" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"msg" => "Hello World"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_30_x_api_key_optional_header_success
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_30_x_api_key_optional_header_success", parameter_schema: {"properties" => {"key" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"msg" => "Hello secret"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_headers_31_x_api_key_required_header_missing
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_31_x_api_key_required_header_missing", parameter_schema: {"properties" => {"X-API-Key" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["headers", "x-api-key"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_headers_32_x_api_key_required_header_success
    app = Spikard::App.new
    app.get("/users/me", handler_name: "headers_32_x_api_key_required_header_success", parameter_schema: {"properties" => {"key" => {"annotation" => "str", "source" => "header", "type" => "string"}}, "required" => ["key"], "type" => "object"}) do |_request|
      build_response(content: {"username" => "secret"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_1_delete_remove_resource
    app = Spikard::App.new
    app.delete("/items/{id}", handler_name: "http_methods_1_delete_remove_resource", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_2_delete_resource_not_found
    app = Spikard::App.new
    app.delete("/items/{id}", handler_name: "http_methods_2_delete_resource_not_found", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_3_delete_with_response_body
    app = Spikard::App.new
    app.delete("/items/{id}", handler_name: "http_methods_3_delete_with_response_body", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "message" => "Item deleted successfully", "name" => "Deleted Item"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_4_head_get_metadata_without_body
    app = Spikard::App.new
    app.head("/items/{id}", handler_name: "http_methods_4_head_get_metadata_without_body", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_5_options_cors_preflight_request
    app = Spikard::App.new
    app.options("/items/", handler_name: "http_methods_5_options_cors_preflight_request") do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_6_patch_partial_update
    app = Spikard::App.new
    app.patch("/items/{id}", handler_name: "http_methods_6_patch_partial_update", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"price" => {"type" => "number"}}, "required" => ["price"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "in_stock" => true, "name" => "Existing Item", "price" => 79.99}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_7_patch_update_multiple_fields
    app = Spikard::App.new
    app.patch("/items/{id}", handler_name: "http_methods_7_patch_update_multiple_fields", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"in_stock" => {"type" => "boolean"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["in_stock", "name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "in_stock" => false, "name" => "Updated Name", "price" => 89.99}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_8_put_complete_resource_replacement
    app = Spikard::App.new
    app.put("/items/{id}", handler_name: "http_methods_8_put_complete_resource_replacement", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"description" => {"type" => "string"}, "id" => {"type" => "integer"}, "in_stock" => {"type" => "boolean"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["description", "id", "in_stock", "name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "Completely replaced", "id" => 1, "in_stock" => true, "name" => "Updated Item", "price" => 99.99}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_9_put_create_resource_if_doesn_t_exist
    app = Spikard::App.new
    app.put("/items/{id}", handler_name: "http_methods_9_put_create_resource_if_doesn_t_exist", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"id" => {"type" => "integer"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["id", "name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 999, "name" => "New Item", "price" => 49.99}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_10_put_idempotent_operation
    app = Spikard::App.new
    app.put("/items/{id}", handler_name: "http_methods_10_put_idempotent_operation", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"id" => {"type" => "integer"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["id", "name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "name" => "Fixed Name", "price" => 50.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_http_methods_11_put_missing_required_field
    app = Spikard::App.new
    app.put("/items/{id}", handler_name: "http_methods_11_put_missing_required_field", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"id" => {"type" => "integer"}, "name" => {"type" => "string"}, "price" => {"type" => "string"}}, "required" => ["price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "1", "loc" => ["body", "price"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_http_methods_12_put_validation_error
    app = Spikard::App.new
    app.put("/items/{id}", handler_name: "http_methods_12_put_validation_error", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"$schema" => "https://json-schema.org/draft/2020-12/schema", "properties" => {"id" => {"type" => "integer"}, "name" => {"minLength" => 3, "type" => "string"}, "price" => {"exclusiveMinimum" => 0, "type" => "number"}}, "required" => ["id", "name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "2 validation errors in request", "errors" => [{"input" => "X", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}, {"input" => -10, "loc" => ["body", "price"], "msg" => "Input should be greater than 0", "type" => "greater_than"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_1_29_nested_object_validation_success
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_1_29_nested_object_validation_success", request_schema: {"properties" => {"profile" => {"properties" => {"email" => {"format" => "email", "type" => "string"}, "name" => {"minLength" => 1, "type" => "string"}}, "required" => ["name", "email"], "type" => "object"}}, "required" => ["profile"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_2_30_nested_object_missing_field
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_2_30_nested_object_missing_field", request_schema: {"properties" => {"profile" => {"properties" => {"email" => {"format" => "email", "type" => "string"}, "name" => {"minLength" => 1, "type" => "string"}}, "required" => ["name", "email"], "type" => "object"}}, "required" => ["profile"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"required" => true}, "loc" => ["body", "profile", "email"], "msg" => "Field required", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_3_31_nullable_property_null_value
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_3_31_nullable_property_null_value", request_schema: {"properties" => {"description" => {"type" => ["string", "null"]}, "name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_4_32_schema_ref_definitions
    app = Spikard::App.new
    app.post("/products", handler_name: "json_bodies_4_32_schema_ref_definitions", request_schema: {"definitions" => {"Product" => {"properties" => {"name" => {"type" => "string"}, "price" => {"minimum" => 0, "type" => "number"}}, "required" => ["name", "price"], "type" => "object"}}, "properties" => {"product" => {"$ref" => "#/definitions/Product"}}, "required" => ["product"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_5_33_allof_schema_composition
    app = Spikard::App.new
    app.post("/items", handler_name: "json_bodies_5_33_allof_schema_composition", request_schema: {"allOf" => [{"properties" => {"name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}, {"properties" => {"price" => {"minimum" => 0, "type" => "number"}}, "required" => ["price"], "type" => "object"}]}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_6_34_additional_properties_false
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_6_34_additional_properties_false", request_schema: {"additionalProperties" => false, "properties" => {"email" => {"type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"additional_properties" => false, "unexpected_field" => "extra_field"}, "loc" => ["body", "extra_field"], "msg" => "Additional properties are not allowed", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_7_35_oneof_schema_success
    app = Spikard::App.new
    app.post("/payment", handler_name: "json_bodies_7_35_oneof_schema_success", request_schema: {"oneOf" => [{"properties" => {"credit_card" => {"pattern" => "^[0-9]{16}$", "type" => "string"}}, "required" => ["credit_card"], "type" => "object"}, {"properties" => {"paypal_email" => {"format" => "email", "type" => "string"}}, "required" => ["paypal_email"], "type" => "object"}]}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_8_36_oneof_schema_multiple_match_failure
    app = Spikard::App.new
    app.post("/payment", handler_name: "json_bodies_8_36_oneof_schema_multiple_match_failure", request_schema: {"oneOf" => [{"properties" => {"credit_card" => {"pattern" => "^[0-9]{16}$", "type" => "string"}}, "required" => ["credit_card"], "type" => "object"}, {"properties" => {"paypal_email" => {"format" => "email", "type" => "string"}}, "required" => ["paypal_email"], "type" => "object"}]}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"matched_schemas" => 2}, "loc" => ["body"], "msg" => "Must match exactly one schema (oneOf), but matched 2", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_9_37_oneof_schema_no_match_failure
    app = Spikard::App.new
    app.post("/payment", handler_name: "json_bodies_9_37_oneof_schema_no_match_failure", request_schema: {"oneOf" => [{"properties" => {"credit_card" => {"pattern" => "^[0-9]{16}$", "type" => "string"}}, "required" => ["credit_card"], "type" => "object"}, {"properties" => {"paypal_email" => {"format" => "email", "type" => "string"}}, "required" => ["paypal_email"], "type" => "object"}]}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"matched_schemas" => 0}, "loc" => ["body"], "msg" => "Must match exactly one schema (oneOf), but matched 0", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_10_38_anyof_schema_success
    app = Spikard::App.new
    app.post("/contact", handler_name: "json_bodies_10_38_anyof_schema_success", request_schema: {"anyOf" => [{"required" => ["email"]}, {"required" => ["phone"]}], "properties" => {"name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_11_39_anyof_schema_multiple_match_success
    app = Spikard::App.new
    app.post("/contact", handler_name: "json_bodies_11_39_anyof_schema_multiple_match_success", request_schema: {"anyOf" => [{"required" => ["email"]}, {"required" => ["phone"]}], "properties" => {"email" => {"format" => "email", "type" => "string"}, "name" => {"type" => "string"}, "phone" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_12_40_anyof_schema_failure
    app = Spikard::App.new
    app.post("/contact", handler_name: "json_bodies_12_40_anyof_schema_failure", request_schema: {"anyOf" => [{"required" => ["email"]}, {"required" => ["phone"]}], "properties" => {"email" => {"format" => "email", "type" => "string"}, "name" => {"type" => "string"}, "phone" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"matched_schemas" => 0}, "loc" => ["body"], "msg" => "Must match at least one schema (anyOf), but matched 0", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_13_41_not_schema_success
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_13_41_not_schema_success", request_schema: {"properties" => {"username" => {"not" => {"enum" => ["admin", "root", "system"]}, "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_14_42_not_schema_failure
    app = Spikard::App.new
    app.post("/users", handler_name: "json_bodies_14_42_not_schema_failure", request_schema: {"properties" => {"username" => {"not" => {"enum" => ["admin", "root", "system"]}, "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"prohibited_value" => "admin"}, "loc" => ["body", "username"], "msg" => "Must not match the schema", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_15_43_const_validation_success
    app = Spikard::App.new
    app.post("/api/v1/data", handler_name: "json_bodies_15_43_const_validation_success", request_schema: {"properties" => {"data" => {"type" => "string"}, "version" => {"const" => "1.0", "type" => "string"}}, "required" => ["version", "data"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_16_44_const_validation_failure
    app = Spikard::App.new
    app.post("/api/v1/data", handler_name: "json_bodies_16_44_const_validation_failure", request_schema: {"properties" => {"data" => {"type" => "string"}, "version" => {"const" => "1.0", "type" => "string"}}, "required" => ["version", "data"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"const" => "1.0", "value" => "2.0"}, "loc" => ["body", "version"], "msg" => "Value must be exactly \'1.0\'", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_17_45_minproperties_validation_success
    app = Spikard::App.new
    app.post("/config", handler_name: "json_bodies_17_45_minproperties_validation_success", request_schema: {"minProperties" => 2, "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_18_46_minproperties_validation_failure
    app = Spikard::App.new
    app.post("/config", handler_name: "json_bodies_18_46_minproperties_validation_failure", request_schema: {"minProperties" => 2, "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_properties" => 1, "min_properties" => 2}, "loc" => ["body"], "msg" => "Object must have at least 2 properties", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_19_47_maxproperties_validation_failure
    app = Spikard::App.new
    app.post("/config", handler_name: "json_bodies_19_47_maxproperties_validation_failure", request_schema: {"maxProperties" => 3, "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_properties" => 4, "max_properties" => 3}, "loc" => ["body"], "msg" => "Object must have at most 3 properties", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_20_48_dependencies_validation_success
    app = Spikard::App.new
    app.post("/billing", handler_name: "json_bodies_20_48_dependencies_validation_success", request_schema: {"dependencies" => {"credit_card" => ["billing_address"]}, "properties" => {"billing_address" => {"type" => "string"}, "credit_card" => {"type" => "string"}, "name" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_21_49_dependencies_validation_failure
    app = Spikard::App.new
    app.post("/billing", handler_name: "json_bodies_21_49_dependencies_validation_failure", request_schema: {"dependencies" => {"credit_card" => ["billing_address"]}, "properties" => {"billing_address" => {"type" => "string"}, "credit_card" => {"type" => "string"}, "name" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"dependency" => "credit_card", "required_fields" => ["billing_address"]}, "loc" => ["body"], "msg" => "When \'credit_card\' is present, \'billing_address\' is required", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_22_50_deep_nesting_4_levels
    app = Spikard::App.new
    app.post("/data", handler_name: "json_bodies_22_50_deep_nesting_4_levels", request_schema: {"properties" => {"user" => {"properties" => {"profile" => {"properties" => {"contact" => {"properties" => {"address" => {"properties" => {"street" => {"type" => "string"}}, "required" => ["street"], "type" => "object"}}, "required" => ["address"], "type" => "object"}}, "required" => ["contact"], "type" => "object"}}, "required" => ["profile"], "type" => "object"}}, "required" => ["user"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_json_bodies_23_array_of_objects_success
    app = Spikard::App.new
    app.post("/items/list", handler_name: "json_bodies_23_array_of_objects_success", request_schema: {"additionalProperties" => false, "properties" => {"images" => {"items" => {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "url" => {"type" => "string"}}, "required" => ["url", "name"], "type" => "object"}, "type" => "array"}, "name" => {"type" => "string"}, "tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["name", "tags", "images"], "type" => "object"}) do |_request|
      build_response(content: {"images" => [{"name" => "Front", "url" => "https://example.com/img1.jpg"}, {"name" => "Back", "url" => "https://example.com/img2.jpg"}], "name" => "Product Bundle", "tags" => ["electronics", "gadget"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_24_array_of_primitive_values
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_24_array_of_primitive_values", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "ratings" => {"items" => {"type" => "number"}, "type" => "array"}, "tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["name", "tags", "ratings"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "Product", "ratings" => [4.5, 4.8, 5.0, 4.2], "tags" => ["electronics", "gadget", "new"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_25_body_with_query_parameters
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_25_body_with_query_parameters", parameter_schema: {"properties" => {"limit" => {"source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}, request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"item" => {"name" => "Item", "price" => 42.0}, "limit" => 10}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_26_boolean_field_success
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_26_boolean_field_success", request_schema: {"additionalProperties" => false, "properties" => {"in_stock" => {"type" => "boolean"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price", "in_stock"], "type" => "object"}) do |_request|
      build_response(content: {"in_stock" => true, "name" => "Item", "price" => 42.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_27_date_field_success
    app = Spikard::App.new
    app.post("/events/", handler_name: "json_bodies_27_date_field_success", request_schema: {"additionalProperties" => false, "properties" => {"event_date" => {"type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "event_date"], "type" => "object"}) do |_request|
      build_response(content: {"event_date" => "2024-03-15", "name" => "Conference"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_28_datetime_field_success
    app = Spikard::App.new
    app.post("/events/", handler_name: "json_bodies_28_datetime_field_success", request_schema: {"additionalProperties" => false, "properties" => {"created_at" => {"format" => "date-time", "type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "created_at"], "type" => "object"}) do |_request|
      build_response(content: {"created_at" => "2024-03-15T10:30:00Z", "name" => "Meeting"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_29_deeply_nested_objects
    app = Spikard::App.new
    app.post("/items/nested", handler_name: "json_bodies_29_deeply_nested_objects", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}, "seller" => {"additionalProperties" => false, "properties" => {"address" => {"additionalProperties" => false, "properties" => {"city" => {"type" => "string"}, "country" => {"additionalProperties" => false, "properties" => {"code" => {"type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "code"], "type" => "object"}, "street" => {"type" => "string"}}, "required" => ["street", "city", "country"], "type" => "object"}, "name" => {"type" => "string"}}, "required" => ["name", "address"], "type" => "object"}}, "required" => ["name", "price", "seller"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "Product", "price" => 100.0, "seller" => {"address" => {"city" => "Springfield", "country" => {"code" => "US", "name" => "USA"}, "street" => "123 Main St"}, "name" => "John Doe"}}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_30_empty_json_object
    app = Spikard::App.new
    app.post("/items/optional-all", handler_name: "json_bodies_30_empty_json_object", request_schema: {"additionalProperties" => false, "properties" => {}, "type" => "object"}) do |_request|
      build_response(content: {"description" => nil, "name" => nil, "price" => nil, "tax" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_31_empty_array_validation_fail
    app = Spikard::App.new
    app.post("/items/list-validated", handler_name: "json_bodies_31_empty_array_validation_fail", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "tags" => {"items" => {}, "minItems" => 1, "type" => "array"}}, "required" => ["name", "tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 1}, "input" => [], "loc" => ["body", "tags"], "msg" => "List should have at least 1 item after validation", "type" => "too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_32_enum_field_invalid_value
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_32_enum_field_invalid_value", request_schema: {"additionalProperties" => false, "properties" => {"category" => {"enum" => ["electronics", "clothing", "books"], "type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "category"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"expected" => "\'electronics\', \'clothing\' or \'books\'"}, "input" => "furniture", "loc" => ["body", "category"], "msg" => "Input should be \'electronics\', \'clothing\' or \'books\'", "type" => "enum"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_33_enum_field_success
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_33_enum_field_success", request_schema: {"additionalProperties" => false, "properties" => {"category" => {"type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "category"], "type" => "object"}) do |_request|
      build_response(content: {"category" => "electronics", "name" => "Item"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_34_extra_fields_ignored_no_additionalproperties
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_34_extra_fields_ignored_no_additionalproperties", request_schema: {"additionalProperties" => false, "properties" => {"another_extra" => {"type" => "integer"}, "extra_field" => {"type" => "string"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price", "extra_field", "another_extra"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "Item", "price" => 42.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_35_field_type_validation_invalid_type
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_35_field_type_validation_invalid_type", request_schema: {"additionalProperties" => false, "properties" => {"description" => {"type" => "string"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}, "tax" => {"type" => "number"}}, "required" => ["name", "description", "price", "tax"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not a number", "loc" => ["body", "price"], "msg" => "Input should be a valid number", "type" => "float_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_36_nested_object_success
    app = Spikard::App.new
    app.post("/items/nested", handler_name: "json_bodies_36_nested_object_success", request_schema: {"additionalProperties" => false, "properties" => {"image" => {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "url" => {"type" => "string"}}, "required" => ["url", "name"], "type" => "object"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price", "image"], "type" => "object"}) do |_request|
      build_response(content: {"image" => {"name" => "Product Image", "url" => "https://example.com/image.jpg"}, "name" => "Foo", "price" => 42.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_37_null_value_for_optional_field
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_37_null_value_for_optional_field", request_schema: {"additionalProperties" => false, "properties" => {"description" => {"type" => "null"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}, "tax" => {"type" => "null"}}, "required" => ["name", "price", "description", "tax"], "type" => "object"}) do |_request|
      build_response(content: {"description" => nil, "name" => "Item", "price" => 42.0, "tax" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_38_numeric_ge_validation_fail
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_38_numeric_ge_validation_fail", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"minimum" => 1, "type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"ge" => 1}, "input" => 0.5, "loc" => ["body", "price"], "msg" => "Input should be greater than or equal to 1", "type" => "greater_than_equal"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_39_numeric_le_validation_success
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_39_numeric_le_validation_success", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "Item", "price" => 100.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_40_optional_fields_omitted
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_40_optional_fields_omitted", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"description" => nil, "name" => "Foo", "price" => 35.4, "tax" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_41_patch_partial_update
    app = Spikard::App.new
    app.patch("/items/{id}", handler_name: "json_bodies_41_patch_partial_update", parameter_schema: {"properties" => {"id" => {"source" => "path", "type" => "string"}}, "required" => ["id"], "type" => "object"}, request_schema: {"properties" => {"price" => {"type" => "number"}}, "required" => ["price"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "Original description", "name" => "Original Item", "price" => 45.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_42_required_field_missing_validation_error
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_42_required_field_missing_validation_error", request_schema: {"additionalProperties" => false, "properties" => {"description" => {"type" => "string"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["description", "price", "name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "", "loc" => ["body", "name"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_43_simple_json_object_success
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_43_simple_json_object_success", request_schema: {"additionalProperties" => false, "properties" => {"description" => {"type" => "string"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}, "tax" => {"type" => "number"}}, "required" => ["name", "description", "price", "tax"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "A very nice Item", "name" => "Foo", "price" => 35.4, "tax" => 3.2}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_44_string_max_length_validation_fail
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_44_string_max_length_validation_fail", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"maxLength" => 50, "type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 50}, "input" => "This is a very long name that exceeds the maximum length", "loc" => ["body", "name"], "msg" => "String should have at most 50 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_45_string_min_length_validation_fail
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_45_string_min_length_validation_fail", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"minLength" => 3, "type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_46_string_pattern_validation_fail
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_46_string_pattern_validation_fail", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "sku" => {"pattern" => "^[A-Z]{3}[0-9]{4}$", "type" => "string"}}, "required" => ["name", "sku"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[A-Z]{3}[0-9]{4}$"}, "input" => "ABC-123", "loc" => ["body", "sku"], "msg" => "String should match pattern \'^[A-Z]{3}[0-9]{4}$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_47_string_pattern_validation_success
    app = Spikard::App.new
    app.post("/items/validated", handler_name: "json_bodies_47_string_pattern_validation_success", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "sku" => {"type" => "string"}}, "required" => ["name", "sku"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "Item", "sku" => "ABC1234"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_json_bodies_48_uuid_field_invalid_format
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_48_uuid_field_invalid_format", request_schema: {"additionalProperties" => false, "properties" => {"item_id" => {"format" => "uuid", "type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not-a-valid-uuid", "loc" => ["body", "item_id"], "msg" => "Input should be a valid UUID", "type" => "uuid_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_json_bodies_49_uuid_field_success
    app = Spikard::App.new
    app.post("/items/", handler_name: "json_bodies_49_uuid_field_success", request_schema: {"additionalProperties" => false, "properties" => {"item_id" => {"format" => "uuid", "type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name", "item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716", "name" => "Item"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_1_17_file_magic_number_png_success
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_1_17_file_magic_number_png_success", file_params: {"image" => {"content_type" => ["image/png"], "required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_multipart_2_18_file_magic_number_jpeg_success
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_2_18_file_magic_number_jpeg_success", file_params: {"image" => {"content_type" => ["image/jpeg"], "required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_multipart_3_19_file_mime_spoofing_png_as_jpeg
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_3_19_file_mime_spoofing_png_as_jpeg", file_params: {"image" => {"content_type" => ["image/jpeg"], "required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"declared_mime" => "image/jpeg", "detected_type" => "image/png", "magic_bytes" => "89504e470d0a1a0a"}, "loc" => ["files", "image"], "msg" => "File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_multipart_4_20_file_mime_spoofing_jpeg_as_png
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_4_20_file_mime_spoofing_jpeg_as_png", file_params: {"image" => {"content_type" => ["image/png"], "required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"declared_mime" => "image/png", "detected_type" => "image/jpeg", "magic_bytes" => "ffd8ffe0"}, "loc" => ["files", "image"], "msg" => "File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_multipart_5_21_file_pdf_magic_number_success
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_5_21_file_pdf_magic_number_success", file_params: {"document" => {"content_type" => ["application/pdf"], "required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: nil, status: 201, headers: nil)
    end
    app
  end

  def create_app_multipart_6_22_file_empty_buffer
    app = Spikard::App.new
    app.post("/upload", handler_name: "multipart_6_22_file_empty_buffer", file_params: {"file" => {"required" => true, "validate_magic_numbers" => true}}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"buffer_size" => 0}, "loc" => ["files", "file"], "msg" => "File buffer is empty", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_multipart_7_content_type_validation_invalid_type
    app = Spikard::App.new
    app.post("/files/images-only", handler_name: "multipart_7_content_type_validation_invalid_type", file_params: {"file" => {"content_type" => ["image/jpeg", "image/png", "image/gif"], "required" => true}}, request_schema: {"additionalProperties" => false, "properties" => {"file" => {"format" => "binary", "type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"loc" => ["files", "file"], "msg" => "Invalid content type \'application/x-sh\'. Allowed types: image/jpeg, image/png, image/gif", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_multipart_8_empty_file_upload
    app = Spikard::App.new
    app.post("/files/upload", handler_name: "multipart_8_empty_file_upload", request_schema: {"additionalProperties" => false, "properties" => {"file" => {"format" => "binary", "type" => "string"}}, "required" => ["file"], "type" => "object"}) do |_request|
      build_response(content: {"filename" => "empty.txt", "size" => 0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_9_file_list_upload_array_of_files
    app = Spikard::App.new
    app.post("/files/list", handler_name: "multipart_9_file_list_upload_array_of_files", request_schema: {"additionalProperties" => false, "properties" => {"files" => {"items" => {"format" => "binary", "type" => "string"}, "type" => "array"}}, "required" => ["files"], "type" => "object"}) do |_request|
      build_response(content: {"filenames" => ["file1.txt", "file2.txt"], "total_size" => 35}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_10_file_size_validation_too_large
    app = Spikard::App.new
    app.post("/files/validated", handler_name: "multipart_10_file_size_validation_too_large", request_schema: {"additionalProperties" => false, "properties" => {"file" => {"format" => "binary", "type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"detail" => "File too large. Maximum size is 1MB"}, status: 413, headers: nil)
    end
    app
  end

  def create_app_multipart_11_file_upload_with_custom_headers
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_11_file_upload_with_custom_headers", request_schema: {"additionalProperties" => false, "properties" => {"test2" => {"format" => "binary", "type" => "string"}}, "required" => ["test2"], "type" => "object"}) do |_request|
      build_response(content: {"test2" => {"content" => "<file2 content>", "content_type" => "text/plain", "filename" => "test2.txt", "headers" => [["content-disposition", "form-data; name=\"test2\"; filename=\"test2.txt\""], ["content-type", "text/plain"], ["x-custom", "f2"]], "size" => 15}}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_12_file_upload_without_filename
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_12_file_upload_without_filename", request_schema: {"additionalProperties" => false, "properties" => {"test1" => {"format" => "binary", "type" => "string"}}, "required" => ["test1"], "type" => "object"}) do |_request|
      build_response(content: {"test1" => "<file1 content>"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_13_form_data_without_files
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_13_form_data_without_files", request_schema: {"additionalProperties" => false, "properties" => {"some" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"some" => "data"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_14_image_file_upload
    app = Spikard::App.new
    app.post("/files/image", handler_name: "multipart_14_image_file_upload", request_schema: {"additionalProperties" => false, "properties" => {"image" => {"format" => "binary", "type" => "string"}}, "required" => ["image"], "type" => "object"}) do |_request|
      build_response(content: {"content_type" => "image/jpeg", "filename" => "photo.jpg", "size" => 22}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_15_mixed_files_and_form_data
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_15_mixed_files_and_form_data", request_schema: {"additionalProperties" => false, "properties" => {"active" => {"type" => "string"}, "age" => {"type" => "string"}, "file" => {"format" => "binary", "type" => "string"}, "username" => {"type" => "string"}}, "required" => ["file"], "type" => "object"}) do |_request|
      build_response(content: {"active" => "true", "age" => "25", "file" => {"content" => "file data here", "content_type" => "text/plain", "filename" => "upload.txt", "size" => 14}, "username" => "testuser"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_16_multiple_file_uploads
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_16_multiple_file_uploads", request_schema: {"additionalProperties" => false, "properties" => {"test1" => {"format" => "binary", "type" => "string"}, "test2" => {"format" => "binary", "type" => "string"}}, "required" => ["test1", "test2"], "type" => "object"}) do |_request|
      build_response(content: {"test1" => {"content" => "<file1 content>", "content_type" => "text/plain", "filename" => "test1.txt", "size" => 15}, "test2" => {"content" => "<file2 content>", "content_type" => "text/plain", "filename" => "test2.txt", "size" => 15}}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_17_multiple_values_for_same_field_name
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_17_multiple_values_for_same_field_name", request_schema: {"additionalProperties" => false, "properties" => {"files" => {"items" => {"format" => "binary", "type" => "string"}, "type" => "array"}, "tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["files"], "type" => "object"}) do |_request|
      build_response(content: {"files" => [{"content" => "first file", "content_type" => "text/plain", "filename" => "file1.txt", "size" => 10}, {"content" => "second file", "content_type" => "text/plain", "filename" => "file2.txt", "size" => 11}], "tags" => ["python", "rust", "web"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_18_optional_file_upload_missing
    app = Spikard::App.new
    app.post("/files/optional", handler_name: "multipart_18_optional_file_upload_missing", request_schema: {"additionalProperties" => false, "properties" => {}, "type" => "object"}) do |_request|
      build_response(content: {"file" => nil}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_19_optional_file_upload_provided
    app = Spikard::App.new
    app.post("/files/optional", handler_name: "multipart_19_optional_file_upload_provided", request_schema: {"additionalProperties" => false, "properties" => {"file" => {"format" => "binary", "type" => "string"}}, "required" => ["file"], "type" => "object"}) do |_request|
      build_response(content: {"content_type" => "text/plain", "filename" => "optional.txt", "size" => 21}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_20_pdf_file_upload
    app = Spikard::App.new
    app.post("/files/document", handler_name: "multipart_20_pdf_file_upload", request_schema: {"additionalProperties" => false, "properties" => {"document" => {"format" => "binary", "type" => "string"}}, "required" => ["document"], "type" => "object"}) do |_request|
      build_response(content: {"content_type" => "application/pdf", "filename" => "report.pdf", "size" => 16}, status: 200, headers: nil)
    end
    app
  end

  def create_app_multipart_21_required_file_upload_missing
    app = Spikard::App.new
    app.post("/files/required", handler_name: "multipart_21_required_file_upload_missing", request_schema: {"additionalProperties" => false, "properties" => {"file" => {"format" => "binary", "type" => "string"}}, "required" => ["file"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "required", "loc" => ["body", "file"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_multipart_22_simple_file_upload
    app = Spikard::App.new
    app.post("/", handler_name: "multipart_22_simple_file_upload", request_schema: {"additionalProperties" => false, "properties" => {"test" => {"format" => "binary", "type" => "string"}}, "required" => ["test"], "type" => "object"}) do |_request|
      build_response(content: {"test" => {"content" => "<file content>", "content_type" => "text/plain", "filename" => "test.txt", "size" => 14}}, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_1_openapi_spec_generation_basic
    app = Spikard::App.new
    app.get("/users", handler_name: "openapi_1_openapi_spec_generation_basic", parameter_schema: {"properties" => {"limit" => {"default" => 10, "description" => "Number of items to return", "maximum" => 100, "minimum" => 1, "source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_2_openapi_spec_with_api_key_security_scheme
    app = Spikard::App.new
    app.get("/api/data", handler_name: "openapi_2_openapi_spec_with_api_key_security_scheme", parameter_schema: {"properties" => {"X-API-Key" => {"description" => "API key for authentication", "source" => "header", "type" => "string"}}, "required" => ["X-API-Key"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_3_openapi_spec_with_jwt_security_scheme
    app = Spikard::App.new
    app.get("/protected/data", handler_name: "openapi_3_openapi_spec_with_jwt_security_scheme", parameter_schema: {"properties" => {"Authorization" => {"description" => "JWT token in Bearer format", "source" => "header", "type" => "string"}}, "required" => ["Authorization"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_4_openapi_spec_with_custom_metadata
    app = Spikard::App.new
    app.get("/info", handler_name: "openapi_4_openapi_spec_with_custom_metadata") do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_5_redoc_serving
    app = Spikard::App.new
    app.get("/status", handler_name: "openapi_5_redoc_serving") do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_openapi_6_swagger_ui_serving
    app = Spikard::App.new
    app.get("/health", handler_name: "openapi_6_swagger_ui_serving") do |_request|
      build_response(content: nil, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_1_20_uuid_v3_path_param_success
    app = Spikard::App.new
    app.get("/items/{id}", handler_name: "path_params_1_20_uuid_v3_path_param_success", parameter_schema: {"properties" => {"id" => {"format" => "uuid", "source" => "path", "type" => "string", "uuidVersion" => "3"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {"id" => "e8b5a51d-11c8-3310-a6ab-367563f20686"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_2_21_uuid_v5_path_param_success
    app = Spikard::App.new
    app.get("/items/{id}", handler_name: "path_params_2_21_uuid_v5_path_param_success", parameter_schema: {"properties" => {"id" => {"format" => "uuid", "source" => "path", "type" => "string", "uuidVersion" => "5"}}, "required" => ["id"], "type" => "object"}) do |_request|
      build_response(content: {"id" => "630eb68f-e0fa-5ecc-887a-7c7a62614681"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_3_24_date_format_path_param_success
    app = Spikard::App.new
    app.get("/events/{date}", handler_name: "path_params_3_24_date_format_path_param_success", parameter_schema: {"properties" => {"date" => {"format" => "date", "source" => "path", "type" => "string"}}, "required" => ["date"], "type" => "object"}) do |_request|
      build_response(content: {"date" => "2025-10-30"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_4_25_date_format_invalid_failure
    app = Spikard::App.new
    app.get("/events/{date}", handler_name: "path_params_4_25_date_format_invalid_failure", parameter_schema: {"properties" => {"date" => {"format" => "date", "source" => "path", "type" => "string"}}, "required" => ["date"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"format" => "date", "value" => "2025-13-45"}, "loc" => ["path", "date"], "msg" => "Invalid date format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_5_27_datetime_format_path_param_success
    app = Spikard::App.new
    app.get("/bookings/{timestamp}", handler_name: "path_params_5_27_datetime_format_path_param_success", parameter_schema: {"properties" => {"timestamp" => {"format" => "date-time", "source" => "path", "type" => "string"}}, "required" => ["timestamp"], "type" => "object"}) do |_request|
      build_response(content: {"timestamp" => "2025-10-30T14:30:00Z"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_6_28_duration_format_path_param_success
    app = Spikard::App.new
    app.get("/delays/{duration}", handler_name: "path_params_6_28_duration_format_path_param_success", parameter_schema: {"properties" => {"duration" => {"format" => "duration", "source" => "path", "type" => "string"}}, "required" => ["duration"], "type" => "object"}) do |_request|
      build_response(content: {"duration" => "P1DT2H30M"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_7_29_decimal_path_param_success
    app = Spikard::App.new
    app.get("/prices/{amount}", handler_name: "path_params_7_29_decimal_path_param_success", parameter_schema: {"properties" => {"amount" => {"format" => "decimal", "source" => "path", "type" => "string"}}, "required" => ["amount"], "type" => "object"}) do |_request|
      build_response(content: {"amount" => "19.99"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_8_30_string_minlength_path_success
    app = Spikard::App.new
    app.get("/users/{username}", handler_name: "path_params_8_30_string_minlength_path_success", parameter_schema: {"properties" => {"username" => {"minLength" => 3, "source" => "path", "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"username" => "alice"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_9_31_string_minlength_path_failure
    app = Spikard::App.new
    app.get("/users/{username}", handler_name: "path_params_9_31_string_minlength_path_failure", parameter_schema: {"properties" => {"username" => {"minLength" => 3, "source" => "path", "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 2, "min_length" => 3}, "loc" => ["path", "username"], "msg" => "String length must be at least 3", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_10_32_string_maxlength_path_failure
    app = Spikard::App.new
    app.get("/users/{username}", handler_name: "path_params_10_32_string_maxlength_path_failure", parameter_schema: {"properties" => {"username" => {"maxLength" => 20, "source" => "path", "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 42, "max_length" => 20}, "loc" => ["path", "username"], "msg" => "String length must not exceed 20", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_11_33_string_pattern_path_success
    app = Spikard::App.new
    app.get("/repos/{owner}/{repo}", handler_name: "path_params_11_33_string_pattern_path_success", parameter_schema: {"properties" => {"owner" => {"pattern" => "^[a-zA-Z0-9-]+$", "source" => "path", "type" => "string"}, "repo" => {"pattern" => "^[a-zA-Z0-9-_]+$", "source" => "path", "type" => "string"}}, "required" => ["owner", "repo"], "type" => "object"}) do |_request|
      build_response(content: {"owner" => "spikard-labs", "repo" => "spikard-http"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_12_34_string_pattern_path_failure
    app = Spikard::App.new
    app.get("/repos/{owner}", handler_name: "path_params_12_34_string_pattern_path_failure", parameter_schema: {"properties" => {"owner" => {"pattern" => "^[a-zA-Z0-9-]+$", "source" => "path", "type" => "string"}}, "required" => ["owner"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-zA-Z0-9-]+$", "value" => "invalid@owner"}, "loc" => ["path", "owner"], "msg" => "String does not match pattern", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_13_35_negative_integer_path_param
    app = Spikard::App.new
    app.get("/offset/{value}", handler_name: "path_params_13_35_negative_integer_path_param", parameter_schema: {"properties" => {"value" => {"source" => "path", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => -100}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_14_boolean_path_parameter_true
    app = Spikard::App.new
    app.get("/path/bool/{item_id}", handler_name: "path_params_14_boolean_path_parameter_true", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "boolean"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => true}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_15_boolean_path_parameter_numeric_1
    app = Spikard::App.new
    app.get("/path/bool/{item_id}", handler_name: "path_params_15_boolean_path_parameter_numeric_1", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "boolean"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => true}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_16_date_path_parameter_success
    app = Spikard::App.new
    app.get("/date/{date_param}", handler_name: "path_params_16_date_path_parameter_success", parameter_schema: {"properties" => {"date_param" => {"format" => "date", "source" => "path", "type" => "string"}}, "required" => ["date_param"], "type" => "object"}) do |_request|
      build_response(content: {"date_param" => "2023-07-15"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_17_enum_path_parameter_invalid_value
    app = Spikard::App.new
    app.get("/models/{model_name}", handler_name: "path_params_17_enum_path_parameter_invalid_value", parameter_schema: {"properties" => {"model_name" => {"enum" => ["alexnet", "resnet", "lenet"], "source" => "path", "type" => "string"}}, "required" => ["model_name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"expected" => "\'alexnet\', \'resnet\' or \'lenet\'"}, "input" => "foo", "loc" => ["path", "model_name"], "msg" => "Input should be \'alexnet\', \'resnet\' or \'lenet\'", "type" => "enum"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_18_enum_path_parameter_success
    app = Spikard::App.new
    app.get("/models/{model_name}", handler_name: "path_params_18_enum_path_parameter_success", parameter_schema: {"properties" => {"model_name" => {"enum" => ["alexnet", "lenet", "resnet"], "source" => "path", "type" => "string"}}, "required" => ["model_name"], "type" => "object"}) do |_request|
      build_response(content: {"model_name" => "alexnet"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_19_float_path_parameter_success
    app = Spikard::App.new
    app.get("/path/float/{item_id}", handler_name: "path_params_19_float_path_parameter_success", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "number"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 42.5}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_20_integer_path_parameter_invalid_string
    app = Spikard::App.new
    app.get("/path/int/{item_id}", handler_name: "path_params_20_integer_path_parameter_invalid_string", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "foobar", "loc" => ["path", "item_id"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "type" => "int_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_21_integer_path_parameter_success
    app = Spikard::App.new
    app.get("/path/int/{item_id}", handler_name: "path_params_21_integer_path_parameter_success", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 42}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_22_integer_path_parameter_with_combined_lt_and_gt_constraints_success
    app = Spikard::App.new
    app.get("/path/param-lt-gt/{item_id}", handler_name: "path_params_22_integer_path_parameter_with_combined_lt_and_gt_constraints_success", parameter_schema: {"properties" => {"item_id" => {"exclusiveMaximum" => 3, "exclusiveMinimum" => 1, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 2}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_23_integer_path_parameter_with_ge_constraint_success
    app = Spikard::App.new
    app.get("/path/param-ge/{item_id}", handler_name: "path_params_23_integer_path_parameter_with_ge_constraint_success", parameter_schema: {"properties" => {"item_id" => {"minimum" => 3, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 3}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_24_integer_path_parameter_with_gt_constraint_failure
    app = Spikard::App.new
    app.get("/path/param-gt/{item_id}", handler_name: "path_params_24_integer_path_parameter_with_gt_constraint_failure", parameter_schema: {"properties" => {"item_id" => {"exclusiveMinimum" => 3, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"gt" => 3}, "input" => 2, "loc" => ["path", "item_id"], "msg" => "Input should be greater than 3", "type" => "greater_than"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_25_integer_path_parameter_with_gt_constraint_success
    app = Spikard::App.new
    app.get("/path/param-gt/{item_id}", handler_name: "path_params_25_integer_path_parameter_with_gt_constraint_success", parameter_schema: {"properties" => {"item_id" => {"exclusiveMinimum" => 3, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 42}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_26_integer_path_parameter_with_le_constraint_success
    app = Spikard::App.new
    app.get("/path/param-le/{item_id}", handler_name: "path_params_26_integer_path_parameter_with_le_constraint_success", parameter_schema: {"properties" => {"item_id" => {"maximum" => 3, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 3}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_27_integer_path_parameter_with_lt_constraint_success
    app = Spikard::App.new
    app.get("/path/param-lt/{item_id}", handler_name: "path_params_27_integer_path_parameter_with_lt_constraint_success", parameter_schema: {"properties" => {"item_id" => {"exclusiveMaximum" => 3, "source" => "path", "type" => "integer"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => 2}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_28_multiple_path_parameters_success
    app = Spikard::App.new
    app.get("/{version}/{service_id}/{user_id}/{order_id}", handler_name: "path_params_28_multiple_path_parameters_success", parameter_schema: {"properties" => {"order_id" => {"format" => "uuid", "source" => "path", "type" => "string"}, "service_id" => {"source" => "path", "type" => "integer"}, "user_id" => {"source" => "path", "type" => "string"}, "version" => {"source" => "path", "type" => "number"}}, "required" => ["order_id", "service_id", "user_id", "version"], "type" => "object"}) do |_request|
      build_response(content: {"order_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716", "service_id" => 1, "user_id" => "abc", "version" => 1.0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_29_path_parameter_type_syntax_invalid_uuid
    app = Spikard::App.new
    app.get("/type-syntax/items/{id:uuid}", handler_name: "path_params_29_path_parameter_type_syntax_invalid_uuid") do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not-a-uuid", "loc" => ["path", "id"], "msg" => "Input should be a valid UUID", "type" => "uuid_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_30_path_parameter_type_syntax_with_override
    app = Spikard::App.new
    app.get("/type-syntax/items-count/{count:int}", handler_name: "path_params_30_path_parameter_type_syntax_with_override", parameter_schema: {"properties" => {"count" => {"maximum" => 100, "minimum" => 1, "source" => "path", "type" => "integer"}}, "required" => ["count"], "type" => "object"}) do |_request|
      build_response(content: {"count" => "50"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_31_path_parameter_with_type_syntax_uuid
    app = Spikard::App.new
    app.get("/type-syntax/items/{id:uuid}", handler_name: "path_params_31_path_parameter_with_type_syntax_uuid") do |_request|
      build_response(content: {"id" => "550e8400-e29b-41d4-a716-446655440000"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_32_path_parameter_with_type_syntax_integer
    app = Spikard::App.new
    app.get("/type-syntax/users/{user_id:int}", handler_name: "path_params_32_path_parameter_with_type_syntax_integer") do |_request|
      build_response(content: {"user_id" => "42"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_33_path_type_parameter_file_path
    app = Spikard::App.new
    app.get("/files/{file_path:path}", handler_name: "path_params_33_path_type_parameter_file_path", parameter_schema: {"properties" => {"file_path" => {"source" => "path", "type" => "string"}}, "required" => ["file_path"], "type" => "object"}) do |_request|
      build_response(content: {"file_path" => "home/johndoe/myfile.txt"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_34_string_path_parameter_success
    app = Spikard::App.new
    app.get("/path/str/{item_id}", handler_name: "path_params_34_string_path_parameter_success", parameter_schema: {"properties" => {"item_id" => {"source" => "path", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => "foobar"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_path_params_35_string_path_parameter_with_max_length_failure
    app = Spikard::App.new
    app.get("/path/param-maxlength/{item_id}", handler_name: "path_params_35_string_path_parameter_with_max_length_failure", parameter_schema: {"properties" => {"item_id" => {"maxLength" => 3, "source" => "path", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 3}, "input" => "foobar", "loc" => ["path", "item_id"], "msg" => "String should have at most 3 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_36_string_path_parameter_with_min_length_failure
    app = Spikard::App.new
    app.get("/path/param-minlength/{item_id}", handler_name: "path_params_36_string_path_parameter_with_min_length_failure", parameter_schema: {"properties" => {"item_id" => {"minLength" => 3, "source" => "path", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "fo", "loc" => ["path", "item_id"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_path_params_37_uuid_path_parameter_success
    app = Spikard::App.new
    app.get("/items/{item_id}", handler_name: "path_params_37_uuid_path_parameter_success", parameter_schema: {"properties" => {"item_id" => {"format" => "uuid", "source" => "path", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_1_42_negative_integer_query_param
    app = Spikard::App.new
    app.get("/items/negative", handler_name: "query_params_1_42_negative_integer_query_param", parameter_schema: {"properties" => {"offset" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["offset"], "type" => "object"}) do |_request|
      build_response(content: {"offset" => -10}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_2_43_scientific_notation_float
    app = Spikard::App.new
    app.get("/stats", handler_name: "query_params_2_43_scientific_notation_float", parameter_schema: {"properties" => {"threshold" => {"annotation" => "float", "source" => "query", "type" => "number"}}, "required" => ["threshold"], "type" => "object"}) do |_request|
      build_response(content: {"threshold" => 0.0015}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_3_44_string_minlength_validation_success
    app = Spikard::App.new
    app.get("/search", handler_name: "query_params_3_44_string_minlength_validation_success", parameter_schema: {"properties" => {"term" => {"minLength" => 3, "source" => "query", "type" => "string"}}, "required" => ["term"], "type" => "object"}) do |_request|
      build_response(content: {"term" => "foo"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_4_45_string_minlength_validation_failure
    app = Spikard::App.new
    app.get("/search", handler_name: "query_params_4_45_string_minlength_validation_failure", parameter_schema: {"properties" => {"term" => {"minLength" => 3, "source" => "query", "type" => "string"}}, "required" => ["term"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 2, "min_length" => 3}, "loc" => ["query", "term"], "msg" => "String length must be at least 3", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_5_46_string_maxlength_validation_failure
    app = Spikard::App.new
    app.get("/search", handler_name: "query_params_5_46_string_maxlength_validation_failure", parameter_schema: {"properties" => {"term" => {"maxLength" => 10, "source" => "query", "type" => "string"}}, "required" => ["term"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 21, "max_length" => 10}, "loc" => ["query", "term"], "msg" => "String length must not exceed 10", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_6_47_pattern_validation_email_success
    app = Spikard::App.new
    app.get("/subscribe", handler_name: "query_params_6_47_pattern_validation_email_success", parameter_schema: {"properties" => {"email" => {"pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", "source" => "query", "type" => "string"}}, "required" => ["email"], "type" => "object"}) do |_request|
      build_response(content: {"email" => "user@example.com"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_7_48_pattern_validation_email_failure
    app = Spikard::App.new
    app.get("/subscribe", handler_name: "query_params_7_48_pattern_validation_email_failure", parameter_schema: {"properties" => {"email" => {"pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", "source" => "query", "type" => "string"}}, "required" => ["email"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", "value" => "invalid-email"}, "loc" => ["query", "email"], "msg" => "String does not match pattern", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_8_49_integer_gt_constraint_success
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_8_49_integer_gt_constraint_success", parameter_schema: {"properties" => {"limit" => {"exclusiveMinimum" => 0, "source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}) do |_request|
      build_response(content: {"limit" => 5}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_9_50_integer_gt_constraint_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_9_50_integer_gt_constraint_failure", parameter_schema: {"properties" => {"limit" => {"exclusiveMinimum" => 0, "source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"exclusive_minimum" => 0, "value" => 0}, "loc" => ["query", "limit"], "msg" => "Value must be greater than 0", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_10_51_integer_ge_constraint_boundary
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_10_51_integer_ge_constraint_boundary", parameter_schema: {"properties" => {"offset" => {"minimum" => 0, "source" => "query", "type" => "integer"}}, "required" => ["offset"], "type" => "object"}) do |_request|
      build_response(content: {"offset" => 0}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_11_52_integer_le_constraint_boundary
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_11_52_integer_le_constraint_boundary", parameter_schema: {"properties" => {"limit" => {"maximum" => 100, "source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}) do |_request|
      build_response(content: {"limit" => 100}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_12_53_integer_le_constraint_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_12_53_integer_le_constraint_failure", parameter_schema: {"properties" => {"limit" => {"maximum" => 100, "source" => "query", "type" => "integer"}}, "required" => ["limit"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"maximum" => 100, "value" => 101}, "loc" => ["query", "limit"], "msg" => "Value must not exceed 100", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_13_54_array_minitems_constraint_success
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_13_54_array_minitems_constraint_success", parameter_schema: {"properties" => {"ids" => {"items" => {"type" => "integer"}, "minItems" => 2, "source" => "query", "type" => "array"}}, "required" => ["ids"], "type" => "object"}) do |_request|
      build_response(content: {"ids" => [1, 2, 3]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_14_55_array_minitems_constraint_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_14_55_array_minitems_constraint_failure", parameter_schema: {"properties" => {"ids" => {"items" => {"type" => "integer"}, "minItems" => 2, "source" => "query", "type" => "array"}}, "required" => ["ids"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_items" => 1, "min_items" => 2}, "loc" => ["query", "ids"], "msg" => "Array must contain at least 2 items", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_15_56_array_maxitems_constraint_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_15_56_array_maxitems_constraint_failure", parameter_schema: {"properties" => {"tags" => {"items" => {"type" => "string"}, "maxItems" => 5, "source" => "query", "type" => "array"}}, "required" => ["tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_items" => 6, "max_items" => 5}, "loc" => ["query", "tags"], "msg" => "Array must not contain more than 5 items", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_16_57_boolean_empty_string_coercion
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_16_57_boolean_empty_string_coercion", parameter_schema: {"properties" => {"active" => {"source" => "query", "type" => "boolean"}}, "required" => ["active"], "type" => "object"}) do |_request|
      build_response(content: {"active" => false}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_17_58_format_email_success
    app = Spikard::App.new
    app.get("/subscribe", handler_name: "query_params_17_58_format_email_success", parameter_schema: {"properties" => {"email" => {"format" => "email", "source" => "query", "type" => "string"}}, "required" => ["email"], "type" => "object"}) do |_request|
      build_response(content: {"email" => "user@example.com"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_18_59_format_email_failure
    app = Spikard::App.new
    app.get("/subscribe", handler_name: "query_params_18_59_format_email_failure", parameter_schema: {"properties" => {"email" => {"format" => "email", "source" => "query", "type" => "string"}}, "required" => ["email"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"format" => "email", "value" => "not-an-email"}, "loc" => ["query", "email"], "msg" => "Invalid email format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_19_60_format_ipv4_success
    app = Spikard::App.new
    app.get("/network", handler_name: "query_params_19_60_format_ipv4_success", parameter_schema: {"properties" => {"ip" => {"format" => "ipv4", "source" => "query", "type" => "string"}}, "required" => ["ip"], "type" => "object"}) do |_request|
      build_response(content: {"ip" => "192.168.1.1"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_20_61_format_ipv4_failure
    app = Spikard::App.new
    app.get("/network", handler_name: "query_params_20_61_format_ipv4_failure", parameter_schema: {"properties" => {"ip" => {"format" => "ipv4", "source" => "query", "type" => "string"}}, "required" => ["ip"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"format" => "ipv4", "value" => "999.999.999.999"}, "loc" => ["query", "ip"], "msg" => "Invalid IPv4 address format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_21_62_format_ipv6_success
    app = Spikard::App.new
    app.get("/network/ipv6", handler_name: "query_params_21_62_format_ipv6_success", parameter_schema: {"properties" => {"ip" => {"format" => "ipv6", "source" => "query", "type" => "string"}}, "required" => ["ip"], "type" => "object"}) do |_request|
      build_response(content: {"ip" => "2001:0db8:85a3:0000:0000:8a2e:0370:7334"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_22_63_format_uri_success
    app = Spikard::App.new
    app.get("/redirect", handler_name: "query_params_22_63_format_uri_success", parameter_schema: {"properties" => {"url" => {"format" => "uri", "source" => "query", "type" => "string"}}, "required" => ["url"], "type" => "object"}) do |_request|
      build_response(content: {"url" => "https://example.com/path?query=value"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_23_64_format_uri_failure
    app = Spikard::App.new
    app.get("/redirect", handler_name: "query_params_23_64_format_uri_failure", parameter_schema: {"properties" => {"url" => {"format" => "uri", "source" => "query", "type" => "string"}}, "required" => ["url"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"format" => "uri", "value" => "not a uri"}, "loc" => ["query", "url"], "msg" => "Invalid URI format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_24_65_format_hostname_success
    app = Spikard::App.new
    app.get("/dns", handler_name: "query_params_24_65_format_hostname_success", parameter_schema: {"properties" => {"host" => {"format" => "hostname", "source" => "query", "type" => "string"}}, "required" => ["host"], "type" => "object"}) do |_request|
      build_response(content: {"host" => "api.example.com"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_25_66_multipleof_constraint_success
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_25_66_multipleof_constraint_success", parameter_schema: {"properties" => {"quantity" => {"multipleOf" => 5, "source" => "query", "type" => "integer"}}, "required" => ["quantity"], "type" => "object"}) do |_request|
      build_response(content: {"quantity" => 15}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_26_67_multipleof_constraint_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_26_67_multipleof_constraint_failure", parameter_schema: {"properties" => {"quantity" => {"multipleOf" => 5, "source" => "query", "type" => "integer"}}, "required" => ["quantity"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"multiple_of" => 5, "value" => 17}, "loc" => ["query", "quantity"], "msg" => "Value must be a multiple of 5", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_27_68_array_uniqueitems_success
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_27_68_array_uniqueitems_success", parameter_schema: {"properties" => {"ids" => {"items" => {"type" => "integer"}, "source" => "query", "type" => "array", "uniqueItems" => true}}, "required" => ["ids"], "type" => "object"}) do |_request|
      build_response(content: {"ids" => [1, 2, 3, 4]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_28_69_array_uniqueitems_failure
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_28_69_array_uniqueitems_failure", parameter_schema: {"properties" => {"ids" => {"items" => {"type" => "integer"}, "source" => "query", "type" => "array", "uniqueItems" => true}}, "required" => ["ids"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"duplicate_index" => 2, "duplicate_value" => 2, "unique_items" => true}, "loc" => ["query", "ids"], "msg" => "Array items must be unique", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_29_70_array_separator_pipe
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_29_70_array_separator_pipe", parameter_schema: {"properties" => {"tags" => {"items" => {"type" => "string"}, "separator" => "|", "source" => "query", "type" => "array"}}, "required" => ["tags"], "type" => "object"}) do |_request|
      build_response(content: {"tags" => ["python", "rust", "typescript"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_30_71_array_separator_semicolon
    app = Spikard::App.new
    app.get("/items", handler_name: "query_params_30_71_array_separator_semicolon", parameter_schema: {"properties" => {"colors" => {"items" => {"type" => "string"}, "separator" => ";", "source" => "query", "type" => "array"}}, "required" => ["colors"], "type" => "object"}) do |_request|
      build_response(content: {"colors" => ["red", "green", "blue"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_31_72_array_separator_space
    app = Spikard::App.new
    app.get("/search", handler_name: "query_params_31_72_array_separator_space", parameter_schema: {"properties" => {"keywords" => {"items" => {"type" => "string"}, "separator" => " ", "source" => "query", "type" => "array"}}, "required" => ["keywords"], "type" => "object"}) do |_request|
      build_response(content: {"keywords" => ["rust", "web", "framework"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_32_array_query_parameter_empty_array
    app = Spikard::App.new
    app.get("/query/list-default", handler_name: "query_params_32_array_query_parameter_empty_array", parameter_schema: {"properties" => {"tags" => {"annotation" => "list[str]", "default" => [], "items" => {"type" => "string"}, "source" => "query", "type" => "array"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: [], status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_33_array_query_parameter_single_value
    app = Spikard::App.new
    app.get("/query/list-default", handler_name: "query_params_33_array_query_parameter_single_value", parameter_schema: {"properties" => {"tags" => {"annotation" => "list[str]", "default" => [], "items" => {"type" => "string"}, "source" => "query", "type" => "array"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: ["apple"], status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_34_boolean_query_parameter_numeric_1
    app = Spikard::App.new
    app.get("/query/bool", handler_name: "query_params_34_boolean_query_parameter_numeric_1", parameter_schema: {"properties" => {"flag" => {"annotation" => "bool", "source" => "query", "type" => "boolean"}}, "required" => ["flag"], "type" => "object"}) do |_request|
      build_response(content: {"flag" => true}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_35_boolean_query_parameter_true
    app = Spikard::App.new
    app.get("/query/bool", handler_name: "query_params_35_boolean_query_parameter_true", parameter_schema: {"properties" => {"flag" => {"annotation" => "bool", "source" => "query", "type" => "boolean"}}, "required" => ["flag"], "type" => "object"}) do |_request|
      build_response(content: {"flag" => true}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_36_date_query_parameter_success
    app = Spikard::App.new
    app.get("/query/date", handler_name: "query_params_36_date_query_parameter_success", parameter_schema: {"properties" => {"event_date" => {"annotation" => "str", "format" => "date", "source" => "query", "type" => "string"}}, "required" => ["event_date"], "type" => "object"}) do |_request|
      build_response(content: {"event_date" => "2024-01-15"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_37_datetime_query_parameter_success
    app = Spikard::App.new
    app.get("/query/datetime", handler_name: "query_params_37_datetime_query_parameter_success", parameter_schema: {"properties" => {"timestamp" => {"annotation" => "str", "format" => "date-time", "source" => "query", "type" => "string"}}, "required" => ["timestamp"], "type" => "object"}) do |_request|
      build_response(content: {"timestamp" => "2024-01-15T10:30:00Z"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_38_enum_query_parameter_invalid_value
    app = Spikard::App.new
    app.get("/query/enum", handler_name: "query_params_38_enum_query_parameter_invalid_value", parameter_schema: {"properties" => {"model" => {"annotation" => "str", "enum" => ["alexnet", "resnet", "lenet"], "source" => "query", "type" => "string"}}, "required" => ["model"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"expected" => "\'alexnet\', \'resnet\' or \'lenet\'"}, "input" => "vgg16", "loc" => ["query", "model"], "msg" => "Input should be \'alexnet\', \'resnet\' or \'lenet\'", "type" => "enum"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_39_enum_query_parameter_success
    app = Spikard::App.new
    app.get("/query/enum", handler_name: "query_params_39_enum_query_parameter_success", parameter_schema: {"properties" => {"model" => {"annotation" => "str", "enum" => ["alexnet", "resnet", "lenet"], "source" => "query", "type" => "string"}}, "required" => ["model"], "type" => "object"}) do |_request|
      build_response(content: {"model" => "alexnet"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_40_float_query_param_with_ge_constraint_success
    app = Spikard::App.new
    app.get("/query/float-ge", handler_name: "query_params_40_float_query_param_with_ge_constraint_success", parameter_schema: {"properties" => {"price" => {"annotation" => "float", "minimum" => 0.01, "source" => "query", "type" => "number"}}, "required" => ["price"], "type" => "object"}) do |_request|
      build_response(content: {"price" => 0.01}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_41_integer_query_param_with_ge_constraint_boundary
    app = Spikard::App.new
    app.get("/query/int-ge", handler_name: "query_params_41_integer_query_param_with_ge_constraint_boundary", parameter_schema: {"properties" => {"value" => {"annotation" => "int", "minimum" => 10, "source" => "query", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 10}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_42_integer_query_param_with_gt_constraint_valid
    app = Spikard::App.new
    app.get("/query/int-gt", handler_name: "query_params_42_integer_query_param_with_gt_constraint_valid", parameter_schema: {"properties" => {"value" => {"annotation" => "int", "exclusiveMinimum" => 0, "source" => "query", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 1}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_43_integer_query_param_with_le_constraint_boundary
    app = Spikard::App.new
    app.get("/query/int-le", handler_name: "query_params_43_integer_query_param_with_le_constraint_boundary", parameter_schema: {"properties" => {"value" => {"annotation" => "int", "maximum" => 100, "source" => "query", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 100}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_44_integer_query_param_with_lt_constraint_valid
    app = Spikard::App.new
    app.get("/query/int-lt", handler_name: "query_params_44_integer_query_param_with_lt_constraint_valid", parameter_schema: {"properties" => {"value" => {"annotation" => "int", "exclusiveMaximum" => 50, "source" => "query", "type" => "integer"}}, "required" => ["value"], "type" => "object"}) do |_request|
      build_response(content: {"value" => 49}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_45_integer_with_default_value_not_provided
    app = Spikard::App.new
    app.get("/query/int/default", handler_name: "query_params_45_integer_with_default_value_not_provided", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "default" => 10, "source" => "query", "type" => "integer"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: "foo bar 10", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_46_integer_with_default_value_override
    app = Spikard::App.new
    app.get("/query/int/default", handler_name: "query_params_46_integer_with_default_value_override", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "default" => 10, "source" => "query", "type" => "integer"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: "foo bar 50", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_47_list_of_integers_multiple_values
    app = Spikard::App.new
    app.get("/query/list", handler_name: "query_params_47_list_of_integers_multiple_values", parameter_schema: {"properties" => {"device_ids" => {"annotation" => "list[int]", "items" => {"type" => "integer"}, "source" => "query", "type" => "array"}}, "required" => ["device_ids"], "type" => "object"}) do |_request|
      build_response(content: [1, 2], status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_48_list_of_strings_multiple_values
    app = Spikard::App.new
    app.get("/items/", handler_name: "query_params_48_list_of_strings_multiple_values", parameter_schema: {"properties" => {"q" => {"annotation" => "list[str]", "items" => {"type" => "string"}, "source" => "query", "type" => "array"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"q" => ["foo", "bar"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_49_list_query_parameter_required_but_missing
    app = Spikard::App.new
    app.get("/query/list", handler_name: "query_params_49_list_query_parameter_required_but_missing", parameter_schema: {"properties" => {"device_ids" => {"annotation" => "list[int]", "items" => {"type" => "integer"}, "source" => "query", "type" => "array"}}, "required" => ["device_ids"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["query", "device_ids"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_50_list_with_default_empty_array_no_values_provided
    app = Spikard::App.new
    app.get("/query/list-default", handler_name: "query_params_50_list_with_default_empty_array_no_values_provided", parameter_schema: {"properties" => {"tags" => {"annotation" => "list[str]", "default" => [], "items" => {"type" => "string"}, "source" => "query", "type" => "array"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: [], status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_51_multiple_query_parameters_with_different_types
    app = Spikard::App.new
    app.get("/query/multi-type", handler_name: "query_params_51_multiple_query_parameters_with_different_types", parameter_schema: {"properties" => {"active" => {"annotation" => "bool", "source" => "query", "type" => "boolean"}, "age" => {"annotation" => "int", "source" => "query", "type" => "integer"}, "name" => {"annotation" => "str", "source" => "query", "type" => "string"}, "score" => {"annotation" => "float", "source" => "query", "type" => "number"}}, "required" => ["active", "age", "name", "score"], "type" => "object"}) do |_request|
      build_response(content: {"active" => true, "age" => 30, "name" => "john", "score" => 95.5}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_52_optional_integer_query_parameter_missing
    app = Spikard::App.new
    app.get("/query/int/optional", handler_name: "query_params_52_optional_integer_query_parameter_missing", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: "foo bar None", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_53_optional_query_parameter_with_default_value
    app = Spikard::App.new
    app.get("/query/optional-default", handler_name: "query_params_53_optional_query_parameter_with_default_value", parameter_schema: {"properties" => {"limit" => {"annotation" => "int", "default" => 10, "source" => "query", "type" => "integer"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"limit" => 10}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_54_optional_string_query_parameter_missing
    app = Spikard::App.new
    app.get("/query/optional", handler_name: "query_params_54_optional_string_query_parameter_missing", parameter_schema: {"properties" => {"query" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: "foo bar None", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_55_optional_string_query_parameter_provided
    app = Spikard::App.new
    app.get("/query/optional", handler_name: "query_params_55_optional_string_query_parameter_provided", parameter_schema: {"properties" => {"query" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: "foo bar baz", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_56_query_parameter_with_url_encoded_space
    app = Spikard::App.new
    app.get("/query/basic", handler_name: "query_params_56_query_parameter_with_url_encoded_space", parameter_schema: {"properties" => {"name" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "hello world"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_57_query_parameter_with_url_encoded_special_characters
    app = Spikard::App.new
    app.get("/query/basic", handler_name: "query_params_57_query_parameter_with_url_encoded_special_characters", parameter_schema: {"properties" => {"name" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"name" => "test&value=123"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_58_query_parameter_with_special_characters_url_encoding
    app = Spikard::App.new
    app.get("/test", handler_name: "query_params_58_query_parameter_with_special_characters_url_encoding", parameter_schema: {"properties" => {"email" => {"annotation" => "str", "source" => "query", "type" => "string"}, "special" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => ["email", "special"], "type" => "object"}) do |_request|
      build_response(content: {"email" => "x@test.com", "special" => "&@A.ac"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_59_required_integer_query_parameter_float_value
    app = Spikard::App.new
    app.get("/query/int", handler_name: "query_params_59_required_integer_query_parameter_float_value", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => 42.5, "loc" => ["query", "query"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "type" => "int_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_60_required_integer_query_parameter_invalid_type
    app = Spikard::App.new
    app.get("/query/int", handler_name: "query_params_60_required_integer_query_parameter_invalid_type", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "baz", "loc" => ["query", "query"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "type" => "int_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_61_required_integer_query_parameter_missing
    app = Spikard::App.new
    app.get("/query/int", handler_name: "query_params_61_required_integer_query_parameter_missing", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["query", "query"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_62_required_integer_query_parameter_success
    app = Spikard::App.new
    app.get("/query/int", handler_name: "query_params_62_required_integer_query_parameter_success", parameter_schema: {"properties" => {"query" => {"annotation" => "int", "source" => "query", "type" => "integer"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: "foo bar 42", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_63_required_string_query_parameter_missing
    app = Spikard::App.new
    app.get("/query", handler_name: "query_params_63_required_string_query_parameter_missing", parameter_schema: {"properties" => {"query" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["query", "query"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_64_required_string_query_parameter_success
    app = Spikard::App.new
    app.get("/query", handler_name: "query_params_64_required_string_query_parameter_success", parameter_schema: {"properties" => {"query" => {"annotation" => "str", "source" => "query", "type" => "string"}}, "required" => ["query"], "type" => "object"}) do |_request|
      build_response(content: "foo bar baz", status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_65_string_query_param_with_max_length_constraint_fail
    app = Spikard::App.new
    app.get("/query/str-max-length", handler_name: "query_params_65_string_query_param_with_max_length_constraint_fail", parameter_schema: {"properties" => {"name" => {"annotation" => "str", "maxLength" => 10, "source" => "query", "type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 10}, "input" => "this_is_way_too_long", "loc" => ["query", "name"], "msg" => "String should have at most 10 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_66_string_query_param_with_min_length_constraint_fail
    app = Spikard::App.new
    app.get("/query/str-min-length", handler_name: "query_params_66_string_query_param_with_min_length_constraint_fail", parameter_schema: {"properties" => {"name" => {"annotation" => "str", "minLength" => 3, "source" => "query", "type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["query", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_67_string_query_param_with_regex_pattern_fail
    app = Spikard::App.new
    app.get("/query/pattern", handler_name: "query_params_67_string_query_param_with_regex_pattern_fail", parameter_schema: {"properties" => {"code" => {"annotation" => "str", "pattern" => "^[0-9]{3,}$", "source" => "query", "type" => "string"}}, "required" => ["code"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[0-9]{3,}$"}, "input" => "abc123", "loc" => ["query", "code"], "msg" => "String should match pattern \'^[0-9]{3,}$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_68_string_validation_with_regex_failure
    app = Spikard::App.new
    app.get("/items/", handler_name: "query_params_68_string_validation_with_regex_failure", parameter_schema: {"properties" => {"item_query" => {"annotation" => "str", "pattern" => "^fixedquery$", "source" => "query", "type" => "string"}}, "required" => ["item_query"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^fixedquery$"}, "input" => "nonregexquery", "loc" => ["query", "item_query"], "msg" => "String should match pattern \'^fixedquery$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_69_string_validation_with_regex_success
    app = Spikard::App.new
    app.get("/items/", handler_name: "query_params_69_string_validation_with_regex_success", parameter_schema: {"properties" => {"item_query" => {"annotation" => "str", "pattern" => "^fixedquery$", "source" => "query", "type" => "string"}}, "required" => ["item_query"], "type" => "object"}) do |_request|
      build_response(content: {"item_query" => "fixedquery"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_query_params_70_uuid_query_parameter_invalid_format
    app = Spikard::App.new
    app.get("/query/uuid", handler_name: "query_params_70_uuid_query_parameter_invalid_format", parameter_schema: {"properties" => {"item_id" => {"annotation" => "str", "format" => "uuid", "source" => "query", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not-a-uuid", "loc" => ["query", "item_id"], "msg" => "Input should be a valid UUID", "type" => "uuid_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_query_params_71_uuid_query_parameter_success
    app = Spikard::App.new
    app.get("/query/uuid", handler_name: "query_params_71_uuid_query_parameter_success", parameter_schema: {"properties" => {"item_id" => {"annotation" => "str", "format" => "uuid", "source" => "query", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_status_codes_1_19_413_payload_too_large
    app = Spikard::App.new
    app.post("/upload", handler_name: "status_codes_1_19_413_payload_too_large", request_schema: {"properties" => {"data" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"error" => "Payload Too Large", "message" => "Request body size exceeds maximum allowed size of 1024 bytes"}, status: 413, headers: nil)
    end
    app
  end

  def create_app_status_codes_2_200_ok_success
    app = Spikard::App.new
    app.get("/status-test/{code}", handler_name: "status_codes_2_200_ok_success", parameter_schema: {"properties" => {"code" => {"source" => "path", "type" => "string"}}, "required" => ["code"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "name" => "Item 1"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_status_codes_3_201_created_resource_created
    app = Spikard::App.new
    app.post("/items/", handler_name: "status_codes_3_201_created_resource_created", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"id" => 1, "name" => "New Item"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_status_codes_4_202_accepted_request_accepted_for_processing
    app = Spikard::App.new
    app.post("/tasks/", handler_name: "status_codes_4_202_accepted_request_accepted_for_processing", request_schema: {"additionalProperties" => false, "properties" => {"task" => {"type" => "string"}}, "required" => ["task"], "type" => "object"}) do |_request|
      build_response(content: {"message" => "Task accepted for processing", "task_id" => "abc123"}, status: 202, headers: nil)
    end
    app
  end

  def create_app_status_codes_5_204_no_content_success_with_no_body
    app = Spikard::App.new
    app.delete("/status-test/{code}", handler_name: "status_codes_5_204_no_content_success_with_no_body", parameter_schema: {"properties" => {"code" => {"source" => "path", "type" => "string"}}, "required" => ["code"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 204, headers: nil)
    end
    app
  end

  def create_app_status_codes_6_206_partial_content
    app = Spikard::App.new
    app.get("/files/document.pdf", handler_name: "status_codes_6_206_partial_content") do |_request|
      build_response(content: "binary_data_1024_bytes", status: 206, headers: nil)
    end
    app
  end

  def create_app_status_codes_7_20_414_uri_too_long
    app = Spikard::App.new
    app.get("/data", handler_name: "status_codes_7_20_414_uri_too_long") do |_request|
      build_response(content: {}, status: 200, headers: nil)
    end
    app
  end

  def create_app_status_codes_8_21_431_request_header_fields_too_large
    app = Spikard::App.new
    app.get("/data", handler_name: "status_codes_8_21_431_request_header_fields_too_large", parameter_schema: {"properties" => {"X-Large-Header" => {"source" => "header", "type" => "string"}}, "required" => [], "type" => "object"}) do |_request|
      build_response(content: {"error" => "Request Header Fields Too Large", "message" => "Request headers exceed maximum allowed size of 8192 bytes"}, status: 431, headers: nil)
    end
    app
  end

  def create_app_status_codes_9_22_501_not_implemented
    app = Spikard::App.new
    app.get("/data", handler_name: "status_codes_9_22_501_not_implemented") do |_request|
      build_response(content: nil, status: 405, headers: nil)
    end
    app
  end

  def create_app_status_codes_10_23_503_service_unavailable
    app = Spikard::App.new
    app.get("/data", handler_name: "status_codes_10_23_503_service_unavailable") do |_request|
      build_response(content: {"error" => "Service Unavailable", "message" => "The service is temporarily unavailable. Please try again later."}, status: 503, headers: nil)
    end
    app
  end

  def create_app_status_codes_11_301_moved_permanently_permanent_redirect
    app = Spikard::App.new
    app.get("/old-path", handler_name: "status_codes_11_301_moved_permanently_permanent_redirect") do |_request|
      build_response(content: nil, status: 301, headers: nil)
    end
    app
  end

  def create_app_status_codes_12_302_found_temporary_redirect
    app = Spikard::App.new
    app.get("/temp-redirect", handler_name: "status_codes_12_302_found_temporary_redirect") do |_request|
      build_response(content: nil, status: 302, headers: nil)
    end
    app
  end

  def create_app_status_codes_13_304_not_modified_cached_content_valid
    app = Spikard::App.new
    app.get("/status-test/{code}", handler_name: "status_codes_13_304_not_modified_cached_content_valid", parameter_schema: {"properties" => {"If-None-Match" => {"source" => "header", "type" => "string"}, "code" => {"source" => "path", "type" => "string"}}, "required" => ["code"], "type" => "object"}) do |_request|
      build_response(content: nil, status: 304, headers: nil)
    end
    app
  end

  def create_app_status_codes_14_307_temporary_redirect_method_preserved
    app = Spikard::App.new
    app.post("/redirect-post", handler_name: "status_codes_14_307_temporary_redirect_method_preserved", request_schema: {"additionalProperties" => false, "properties" => {}, "type" => "object"}) do |_request|
      build_response(content: {}, status: 307, headers: nil)
    end
    app
  end

  def create_app_status_codes_15_400_bad_request_invalid_request
    app = Spikard::App.new
    app.post("/items/", handler_name: "status_codes_15_400_bad_request_invalid_request", request_schema: {"type" => "string"}) do |_request|
      build_response(content: {"detail" => "Invalid request format"}, status: 400, headers: nil)
    end
    app
  end

  def create_app_status_codes_16_401_unauthorized_missing_authentication
    app = Spikard::App.new
    app.get("/users/me", handler_name: "status_codes_16_401_unauthorized_missing_authentication") do |_request|
      build_response(content: {"detail" => "Not authenticated"}, status: 401, headers: nil)
    end
    app
  end

  def create_app_status_codes_17_403_forbidden_insufficient_permissions
    app = Spikard::App.new
    app.get("/admin/users", handler_name: "status_codes_17_403_forbidden_insufficient_permissions") do |_request|
      build_response(content: {"detail" => "Not enough permissions"}, status: 403, headers: nil)
    end
    app
  end

  def create_app_status_codes_18_404_not_found_resource_not_found
    app = Spikard::App.new
    app.get("/status-test/{code}", handler_name: "status_codes_18_404_not_found_resource_not_found", parameter_schema: {"properties" => {"code" => {"source" => "path", "type" => "string"}}, "required" => ["code"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "Item not found"}, status: 404, headers: nil)
    end
    app
  end

  def create_app_status_codes_19_408_request_timeout
    app = Spikard::App.new
    app.post("/slow-endpoint", handler_name: "status_codes_19_408_request_timeout", request_schema: {"additionalProperties" => false, "properties" => {"data" => {"type" => "string"}}, "required" => ["data"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "Request timeout"}, status: 408, headers: nil)
    end
    app
  end

  def create_app_status_codes_20_422_unprocessable_entity_validation_error
    app = Spikard::App.new
    app.post("/items/", handler_name: "status_codes_20_422_unprocessable_entity_validation_error", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "string"}}, "required" => ["price", "name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "", "loc" => ["body", "name"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_status_codes_21_429_too_many_requests
    app = Spikard::App.new
    app.get("/api/resource", handler_name: "status_codes_21_429_too_many_requests") do |_request|
      build_response(content: {"detail" => "Rate limit exceeded. Try again in 60 seconds."}, status: 429, headers: nil)
    end
    app
  end

  def create_app_status_codes_22_500_internal_server_error_server_error
    app = Spikard::App.new
    app.get("/error", handler_name: "status_codes_22_500_internal_server_error_server_error") do |_request|
      build_response(content: {"detail" => "Internal server error", "status" => 500, "title" => "Internal Server Error", "type" => "https://spikard.dev/errors/internal-server-error"}, status: 500, headers: nil)
    end
    app
  end

  def create_app_status_codes_23_503_service_unavailable_server_overload
    app = Spikard::App.new
    app.get("/health", handler_name: "status_codes_23_503_service_unavailable_server_overload") do |_request|
      build_response(content: {"detail" => "Service temporarily unavailable"}, status: 503, headers: nil)
    end
    app
  end

  def create_app_url_encoded_1_13_array_field_success
    app = Spikard::App.new
    app.post("/register", handler_name: "url_encoded_1_13_array_field_success", request_schema: {"properties" => {"tags" => {"items" => {"type" => "string"}, "minItems" => 1, "type" => "array"}}, "required" => ["tags"], "type" => "object"}) do |_request|
      build_response(content: {"tags" => ["python", "rust", "typescript"]}, status: 201, headers: nil)
    end
    app
  end

  def create_app_url_encoded_2_14_nested_object_bracket_notation
    app = Spikard::App.new
    app.post("/profile", handler_name: "url_encoded_2_14_nested_object_bracket_notation", request_schema: {"properties" => {"user" => {"properties" => {"age" => {"minimum" => 0, "type" => "integer"}, "email" => {"format" => "email", "type" => "string"}, "name" => {"minLength" => 1, "type" => "string"}}, "required" => ["name", "email"], "type" => "object"}}, "required" => ["user"], "type" => "object"}) do |_request|
      build_response(content: {"user" => {"age" => 30, "email" => "john@example.com", "name" => "John Doe"}}, status: 201, headers: nil)
    end
    app
  end

  def create_app_url_encoded_3_15_special_characters_field_names
    app = Spikard::App.new
    app.post("/data", handler_name: "url_encoded_3_15_special_characters_field_names", request_schema: {"properties" => {"contact.email" => {"format" => "email", "type" => "string"}, "user-name" => {"type" => "string"}}, "type" => "object"}) do |_request|
      build_response(content: {"contact.email" => "john@example.com", "user-name" => "JohnDoe"}, status: 201, headers: nil)
    end
    app
  end

  def create_app_url_encoded_4_16_minlength_validation_failure
    app = Spikard::App.new
    app.post("/users", handler_name: "url_encoded_4_16_minlength_validation_failure", request_schema: {"properties" => {"username" => {"minLength" => 3, "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_length" => 2, "min_length" => 3, "value" => "ab"}, "loc" => ["body", "username"], "msg" => "String length must be at least 3", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_5_17_pattern_validation_failure
    app = Spikard::App.new
    app.post("/accounts", handler_name: "url_encoded_5_17_pattern_validation_failure", request_schema: {"properties" => {"account_id" => {"pattern" => "^ACC-[0-9]{6}$", "type" => "string"}}, "required" => ["account_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^ACC-[0-9]{6}$", "value" => "INVALID123"}, "loc" => ["body", "account_id"], "msg" => "String does not match pattern \'^ACC-[0-9]{6}$\'", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_6_18_integer_minimum_validation_failure
    app = Spikard::App.new
    app.post("/products", handler_name: "url_encoded_6_18_integer_minimum_validation_failure", request_schema: {"properties" => {"quantity" => {"minimum" => 1, "type" => "integer"}}, "required" => ["quantity"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_value" => 0, "minimum" => 1}, "loc" => ["body", "quantity"], "msg" => "Value must be at least 1", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_7_19_array_minitems_validation_failure
    app = Spikard::App.new
    app.post("/tags", handler_name: "url_encoded_7_19_array_minitems_validation_failure", request_schema: {"properties" => {"tags" => {"items" => {"type" => "string"}, "minItems" => 2, "type" => "array"}}, "required" => ["tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"actual_items" => 1, "min_items" => 2}, "loc" => ["body", "tags"], "msg" => "Array must contain at least 2 items", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_8_20_format_email_validation_failure
    app = Spikard::App.new
    app.post("/subscribe", handler_name: "url_encoded_8_20_format_email_validation_failure", request_schema: {"properties" => {"email" => {"format" => "email", "type" => "string"}}, "required" => ["email"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"format" => "email", "value" => "not-an-email"}, "loc" => ["body", "email"], "msg" => "Invalid email format", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_9_21_integer_type_coercion_failure
    app = Spikard::App.new
    app.post("/products", handler_name: "url_encoded_9_21_integer_type_coercion_failure", request_schema: {"properties" => {"price" => {"type" => "integer"}}, "required" => ["price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"value" => "not-a-number"}, "loc" => ["body", "price"], "msg" => "Value is not a valid integer", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_10_22_additional_properties_strict_failure
    app = Spikard::App.new
    app.post("/settings", handler_name: "url_encoded_10_22_additional_properties_strict_failure", request_schema: {"additionalProperties" => false, "properties" => {"theme" => {"enum" => ["light", "dark"], "type" => "string"}}, "required" => ["theme"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"property" => "unknown_field"}, "loc" => ["body", "unknown_field"], "msg" => "Additional properties are not allowed", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_11_boolean_field_conversion
    app = Spikard::App.new
    app.post("/form/", handler_name: "url_encoded_11_boolean_field_conversion", request_schema: {"properties" => {"subscribe" => {"type" => "boolean"}, "username" => {"type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"subscribe" => true, "username" => "johndoe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_12_empty_string_value
    app = Spikard::App.new
    app.post("/form/", handler_name: "url_encoded_12_empty_string_value", request_schema: {"properties" => {"description" => {"type" => "string"}, "username" => {"type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "", "username" => "johndoe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_13_multiple_values_for_same_field
    app = Spikard::App.new
    app.post("/form/tags", handler_name: "url_encoded_13_multiple_values_for_same_field", request_schema: {"properties" => {"tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["tags"], "type" => "object"}) do |_request|
      build_response(content: {"tags" => ["python", "fastapi", "web"]}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_14_numeric_field_type_conversion
    app = Spikard::App.new
    app.post("/form/", handler_name: "url_encoded_14_numeric_field_type_conversion", request_schema: {"properties" => {"age" => {"type" => "integer"}, "username" => {"type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"age" => 30, "username" => "johndoe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_15_oauth2_password_grant_flow
    app = Spikard::App.new
    app.post("/token", handler_name: "url_encoded_15_oauth2_password_grant_flow", request_schema: {"properties" => {"grant_type" => {"type" => "string"}, "password" => {"type" => "string"}, "scope" => {"type" => "string"}, "username" => {"type" => "string"}}, "required" => ["username", "password", "grant_type"], "type" => "object"}) do |_request|
      build_response(content: {"access_token" => "johndoe", "token_type" => "bearer"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_16_optional_field_missing_success
    app = Spikard::App.new
    app.post("/register/", handler_name: "url_encoded_16_optional_field_missing_success", request_schema: {"properties" => {"email" => {"format" => "email", "type" => ["string", "null"]}, "password" => {"type" => "string"}, "username" => {"type" => "string"}}, "required" => ["username", "password"], "type" => "object"}) do |_request|
      build_response(content: {"email" => nil, "username" => "johndoe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_17_pattern_validation_fail
    app = Spikard::App.new
    app.post("/form/validated", handler_name: "url_encoded_17_pattern_validation_fail", request_schema: {"properties" => {"username" => {"pattern" => "^[a-z0-9_]+$", "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-z0-9_]+$"}, "input" => "john doe", "loc" => ["body", "username"], "msg" => "String should match pattern \'^[a-z0-9_]+$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_18_required_field_missing_validation_error
    app = Spikard::App.new
    app.post("/login/", handler_name: "url_encoded_18_required_field_missing_validation_error", request_schema: {"properties" => {"password" => {"type" => "string"}, "username" => {"type" => "string"}}, "required" => ["username", "password"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "", "loc" => ["body", "username"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_19_simple_form_submission_success
    app = Spikard::App.new
    app.post("/login/", handler_name: "url_encoded_19_simple_form_submission_success", request_schema: {"properties" => {"password" => {"type" => "string"}, "username" => {"type" => "string"}}, "required" => ["username", "password"], "type" => "object"}) do |_request|
      build_response(content: {"username" => "johndoe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_20_special_characters_encoding
    app = Spikard::App.new
    app.post("/form/", handler_name: "url_encoded_20_special_characters_encoding", request_schema: {"properties" => {"description" => {"type" => "string"}, "name" => {"type" => "string"}}, "required" => ["name"], "type" => "object"}) do |_request|
      build_response(content: {"description" => "Test & Development", "name" => "John Doe"}, status: 200, headers: nil)
    end
    app
  end

  def create_app_url_encoded_21_string_max_length_validation_fail
    app = Spikard::App.new
    app.post("/form/validated", handler_name: "url_encoded_21_string_max_length_validation_fail", request_schema: {"properties" => {"username" => {"maxLength" => 20, "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 20}, "input" => "this_is_a_very_long_username_that_exceeds_limit", "loc" => ["body", "username"], "msg" => "String should have at most 20 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_url_encoded_22_string_min_length_validation_fail
    app = Spikard::App.new
    app.post("/form/validated", handler_name: "url_encoded_22_string_min_length_validation_fail", request_schema: {"properties" => {"username" => {"minLength" => 3, "type" => "string"}}, "required" => ["username"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["body", "username"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_1_09_multiple_validation_errors
    app = Spikard::App.new
    app.post("/users", handler_name: "validation_errors_1_09_multiple_validation_errors", request_schema: {"properties" => {"age" => {"minimum" => 18, "type" => "integer"}, "email" => {"format" => "email", "type" => "string"}, "name" => {"minLength" => 3, "type" => "string"}}, "required" => ["name", "email", "age"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "3 validation errors in request", "errors" => [{"ctx" => {"ge" => 18}, "input" => 15, "loc" => ["body", "age"], "msg" => "Input should be greater than or equal to 18", "type" => "greater_than_equal"}, {"ctx" => {"pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"}, "input" => "invalid-email", "loc" => ["body", "email"], "msg" => "String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'", "type" => "string_pattern_mismatch"}, {"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_2_10_nested_error_path
    app = Spikard::App.new
    app.post("/profiles", handler_name: "validation_errors_2_10_nested_error_path", request_schema: {"properties" => {"profile" => {"properties" => {"contact" => {"properties" => {"email" => {"format" => "email", "type" => "string"}}, "required" => ["email"], "type" => "object"}}, "required" => ["contact"], "type" => "object"}}, "required" => ["profile"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"}, "input" => "invalid", "loc" => ["body", "profile", "contact", "email"], "msg" => "String should match pattern \'^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_3_array_item_validation_error
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_3_array_item_validation_error", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}, "tags" => {"items" => {"type" => "string"}, "type" => "array"}}, "required" => ["name", "price", "tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => 123, "loc" => ["body", "tags", "2"], "msg" => "Input should be a valid unknown", "type" => "type_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_4_array_max_items_constraint_violation
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_4_array_max_items_constraint_violation", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}, "tags" => {"items" => {"type" => "string"}, "maxItems" => 10, "type" => "array"}}, "required" => ["name", "price", "tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", "tag8", "tag9", "tag10", "tag11"], "loc" => ["body", "tags"], "msg" => "[\"tag1\",\"tag2\",\"tag3\",\"tag4\",\"tag5\",\"tag6\",\"tag7\",\"tag8\",\"tag9\",\"tag10\",\"tag11\"] has more than 10 items", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_5_array_min_items_constraint_violation
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_5_array_min_items_constraint_violation", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}, "tags" => {"items" => {}, "minItems" => 1, "type" => "array"}}, "required" => ["name", "price", "tags"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => [], "loc" => ["body", "tags"], "msg" => "[] has less than 1 item", "type" => "validation_error"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_6_body_field_type_error_string_for_float
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_6_body_field_type_error_string_for_float", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not_a_float", "loc" => ["body", "price"], "msg" => "Input should be a valid number, unable to parse string as a number", "type" => "float_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_7_header_validation_error
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_7_header_validation_error", parameter_schema: {"properties" => {"q" => {"source" => "query", "type" => "string"}, "x-token" => {"source" => "header", "type" => "string"}}, "required" => ["q", "x-token"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["headers", "x-token"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_8_invalid_uuid_format
    app = Spikard::App.new
    app.get("/items/{item_id}", handler_name: "validation_errors_8_invalid_uuid_format", parameter_schema: {"properties" => {"item_id" => {"format" => "uuid", "source" => "path", "type" => "string"}}, "required" => ["item_id"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not-a-uuid", "loc" => ["path", "item_id"], "msg" => "Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0", "type" => "uuid_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_9_invalid_boolean_value
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_9_invalid_boolean_value", parameter_schema: {"properties" => {"is_active" => {"source" => "query", "type" => "boolean"}, "q" => {"source" => "query", "type" => "string"}}, "required" => ["is_active", "q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "maybe", "loc" => ["query", "is_active"], "msg" => "Input should be a valid boolean, unable to interpret input", "type" => "bool_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_10_invalid_datetime_format
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_10_invalid_datetime_format", request_schema: {"additionalProperties" => false, "properties" => {"created_at" => {"format" => "date-time", "type" => "string"}, "name" => {"type" => "string"}, "price" => {"type" => "number"}}, "required" => ["name", "price", "created_at"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not-a-datetime", "loc" => ["body", "created_at"], "msg" => "Input should be a valid datetime", "type" => "datetime_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_11_invalid_enum_value
    app = Spikard::App.new
    app.get("/models/{model_name}", handler_name: "validation_errors_11_invalid_enum_value", parameter_schema: {"properties" => {"model_name" => {"enum" => ["alexnet", "resnet", "lenet"], "source" => "path", "type" => "string"}}, "required" => ["model_name"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"expected" => "\'alexnet\', \'resnet\' or \'lenet\'"}, "input" => "invalid_model", "loc" => ["path", "model_name"], "msg" => "Input should be \'alexnet\', \'resnet\' or \'lenet\'", "type" => "enum"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_12_malformed_json_body
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_12_malformed_json_body", request_schema: {"type" => "string"}) do |_request|
      build_response(content: {"detail" => "Invalid request format"}, status: 400, headers: nil)
    end
    app
  end

  def create_app_validation_errors_13_missing_required_body_field
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_13_missing_required_body_field", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "string"}}, "required" => ["name", "price"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => {"name" => "Item"}, "loc" => ["body", "price"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_14_missing_required_query_parameter
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_14_missing_required_query_parameter", parameter_schema: {"properties" => {"q" => {"source" => "query", "type" => "string"}}, "required" => ["q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => nil, "loc" => ["query", "q"], "msg" => "Field required", "type" => "missing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_15_multiple_validation_errors
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_15_multiple_validation_errors", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"minLength" => 3, "type" => "string"}, "price" => {"exclusiveMinimum" => 0, "type" => "integer"}, "quantity" => {"type" => "integer"}}, "required" => ["name", "price", "quantity"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "3 validation errors in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "X", "loc" => ["body", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}, {"ctx" => {"gt" => 0}, "input" => -10, "loc" => ["body", "price"], "msg" => "Input should be greater than 0", "type" => "greater_than"}, {"input" => "not_a_number", "loc" => ["body", "quantity"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "type" => "int_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_16_nested_object_validation_error
    app = Spikard::App.new
    app.post("/items/", handler_name: "validation_errors_16_nested_object_validation_error", request_schema: {"additionalProperties" => false, "properties" => {"name" => {"type" => "string"}, "price" => {"type" => "number"}, "seller" => {"additionalProperties" => false, "properties" => {"address" => {"additionalProperties" => false, "properties" => {"city" => {"minLength" => 3, "type" => "string"}, "zip_code" => {"minLength" => 5, "type" => "string"}}, "required" => ["city", "zip_code"], "type" => "object"}, "name" => {"minLength" => 3, "type" => "string"}}, "required" => ["name", "address"], "type" => "object"}}, "required" => ["name", "price", "seller"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "3 validation errors in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "SF", "loc" => ["body", "seller", "address", "city"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}, {"ctx" => {"min_length" => 5}, "input" => "123", "loc" => ["body", "seller", "address", "zip_code"], "msg" => "String should have at least 5 characters", "type" => "string_too_short"}, {"ctx" => {"min_length" => 3}, "input" => "Jo", "loc" => ["body", "seller", "name"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_17_numeric_constraint_violation_gt_greater_than
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_17_numeric_constraint_violation_gt_greater_than", parameter_schema: {"properties" => {"price" => {"exclusiveMinimum" => 0, "source" => "query", "type" => "number"}, "q" => {"source" => "query", "type" => "string"}}, "required" => ["price", "q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"gt" => 0}, "input" => "0", "loc" => ["query", "price"], "msg" => "Input should be greater than 0", "type" => "greater_than"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_18_numeric_constraint_violation_le_less_than_or_equal
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_18_numeric_constraint_violation_le_less_than_or_equal", parameter_schema: {"properties" => {"limit" => {"maximum" => 100, "source" => "query", "type" => "integer"}, "q" => {"source" => "query", "type" => "string"}}, "required" => ["limit", "q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"le" => 100}, "input" => "101", "loc" => ["query", "limit"], "msg" => "Input should be less than or equal to 100", "type" => "less_than_equal"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_19_query_param_type_error_string_provided_for_int
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_19_query_param_type_error_string_provided_for_int", parameter_schema: {"properties" => {"q" => {"source" => "query", "type" => "string"}, "skip" => {"source" => "query", "type" => "integer"}}, "required" => ["q", "skip"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"input" => "not_a_number", "loc" => ["query", "skip"], "msg" => "Input should be a valid integer, unable to parse string as an integer", "type" => "int_parsing"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_20_string_max_length_constraint_violation
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_20_string_max_length_constraint_violation", parameter_schema: {"properties" => {"q" => {"maxLength" => 50, "source" => "query", "type" => "string"}}, "required" => ["q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"max_length" => 50}, "input" => "this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter", "loc" => ["query", "q"], "msg" => "String should have at most 50 characters", "type" => "string_too_long"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_21_string_min_length_constraint_violation
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_21_string_min_length_constraint_violation", parameter_schema: {"properties" => {"q" => {"minLength" => 3, "source" => "query", "type" => "string"}}, "required" => ["q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"min_length" => 3}, "input" => "ab", "loc" => ["query", "q"], "msg" => "String should have at least 3 characters", "type" => "string_too_short"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

  def create_app_validation_errors_22_string_regex_pattern_mismatch
    app = Spikard::App.new
    app.get("/items/", handler_name: "validation_errors_22_string_regex_pattern_mismatch", parameter_schema: {"properties" => {"q" => {"pattern" => "^[a-zA-Z0-9_-]+$", "source" => "query", "type" => "string"}}, "required" => ["q"], "type" => "object"}) do |_request|
      build_response(content: {"detail" => "1 validation error in request", "errors" => [{"ctx" => {"pattern" => "^[a-zA-Z0-9_-]+$"}, "input" => "invalid!", "loc" => ["query", "q"], "msg" => "String should match pattern \'^[a-zA-Z0-9_-]+$\'", "type" => "string_pattern_mismatch"}], "status" => 422, "title" => "Request Validation Failed", "type" => "https://spikard.dev/errors/validation-error"}, status: 422, headers: nil)
    end
    app
  end

end
