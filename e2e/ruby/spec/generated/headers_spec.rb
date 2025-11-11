# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "headers" do
  it "30_bearer_token_format_valid" do
    app = E2ERubyApp.create_app_headers_1_30_bearer_token_format_valid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "31_bearer_token_format_invalid" do
    app = E2ERubyApp.create_app_headers_2_31_bearer_token_format_invalid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected", headers: {"Authorization" => "Bearer invalid token with spaces"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "authorization"])
    client.close
  end

  it "32_bearer_token_missing_prefix" do
    app = E2ERubyApp.create_app_headers_3_32_bearer_token_missing_prefix
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected", headers: {"Authorization" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "authorization"])
    client.close
  end

  it "33_api_key_header_valid" do
    app = E2ERubyApp.create_app_headers_4_33_api_key_header_valid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"X-API-Key" => "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "34_api_key_header_invalid" do
    app = E2ERubyApp.create_app_headers_5_34_api_key_header_invalid
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"X-API-Key" => "invalid-key"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "x-api-key"])
    client.close
  end

  it "Accept header - JSON" do
    app = E2ERubyApp.create_app_headers_6_accept_header_json
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/accept", headers: {"Accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"accept" => "application/json"})
    client.close
  end

  it "Accept-Encoding header" do
    app = E2ERubyApp.create_app_headers_7_accept_encoding_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/accept-encoding", headers: {"Accept-Encoding" => "gzip, deflate, br"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"accept_encoding" => "gzip, deflate, br"})
    client.close
  end

  it "Accept-Language header" do
    app = E2ERubyApp.create_app_headers_8_accept_language_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/accept-language", headers: {"Accept-Language" => "en-US,en;q=0.9"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"accept_language" => "en-US,en;q=0.9"})
    client.close
  end

  it "Authorization header - missing" do
    app = E2ERubyApp.create_app_headers_9_authorization_header_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "authorization"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Authorization header - success" do
    app = E2ERubyApp.create_app_headers_10_authorization_header_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me", headers: {"Authorization" => "Digest foobar"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"credentials" => "foobar", "scheme" => "Digest"})
    client.close
  end

  it "Authorization header - wrong scheme" do
    app = E2ERubyApp.create_app_headers_11_authorization_header_wrong_scheme
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me", headers: {"Authorization" => "Other invalidauthorization"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "authorization"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "Basic authentication - success" do
    app = E2ERubyApp.create_app_headers_12_basic_authentication_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/basic-auth", headers: {"Authorization" => "Basic dXNlcm5hbWU6cGFzc3dvcmQ="})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"password" => "password", "username" => "username"})
    client.close
  end

  it "Bearer token authentication - missing" do
    app = E2ERubyApp.create_app_headers_13_bearer_token_authentication_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/bearer-auth", headers: {})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "authorization"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "Bearer token authentication - success" do
    app = E2ERubyApp.create_app_headers_14_bearer_token_authentication_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/bearer-auth", headers: {"Authorization" => "Bearer valid_token_123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"token" => "valid_token_123"})
    client.close
  end

  it "Content-Type header - application/json" do
    app = E2ERubyApp.create_app_headers_15_content_type_header_application_json
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/content-type", headers: {"Content-Type" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"content_type" => "application/json"})
    client.close
  end

  it "Header case insensitivity - access" do
    app = E2ERubyApp.create_app_headers_16_header_case_insensitivity_access
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/echo", headers: {"content-type" => "application/json"}, json: {"test" => "data"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"content_type_lower" => "application/json", "content_type_mixed" => "application/json", "content_type_upper" => "application/json"})
    client.close
  end

  it "Header regex validation - fail" do
    app = E2ERubyApp.create_app_headers_17_header_regex_validation_fail
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/pattern", headers: {"X-Request-Id" => "invalid-format"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "x-request-id"])
    expect(body['errors'].first['type']).to eq("string_pattern_mismatch")
    client.close
  end

  it "Header regex validation - success" do
    app = E2ERubyApp.create_app_headers_18_header_regex_validation_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/pattern", headers: {"X-Request-Id" => "12345"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"x_request_id" => "12345"})
    client.close
  end

  it "Header validation - max_length constraint fail" do
    app = E2ERubyApp.create_app_headers_19_header_validation_max_length_constraint_fail
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/max-length", headers: {"X-Session-Id" => "this_is_way_too_long_for_validation"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "x-session-id"])
    expect(body['errors'].first['type']).to eq("string_too_long")
    client.close
  end

  it "Header validation - min_length constraint" do
    app = E2ERubyApp.create_app_headers_20_header_validation_min_length_constraint
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/validated", headers: {"X-Token" => "ab"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "x-token"])
    expect(body['errors'].first['type']).to eq("string_too_short")
    client.close
  end

  it "Header with underscore conversion - explicit" do
    app = E2ERubyApp.create_app_headers_21_header_with_underscore_conversion_explicit
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/underscore", headers: {"X-Token" => "secret123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"x_token" => "secret123"})
    client.close
  end

  it "Host header" do
    app = E2ERubyApp.create_app_headers_22_host_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/host", headers: {"Host" => "example.com:8080"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"host" => "example.com:8080"})
    client.close
  end

  it "Multiple custom headers" do
    app = E2ERubyApp.create_app_headers_23_multiple_custom_headers
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/multiple", headers: {"X-Client-Version" => "1.2.3", "X-Request-Id" => "req-12345", "X-Trace-Id" => "trace-abc"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"x_client_version" => "1.2.3", "x_request_id" => "req-12345", "x_trace_id" => "trace-abc"})
    client.close
  end

  it "Multiple header values - X-Token" do
    app = E2ERubyApp.create_app_headers_24_multiple_header_values_x_token
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"x-token" => "foo, bar"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"X-Token values" => ["foo", "bar"]})
    client.close
  end

  it "Optional header with None default - missing" do
    app = E2ERubyApp.create_app_headers_25_optional_header_with_none_default_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"strange_header" => nil})
    client.close
  end

  it "Origin header" do
    app = E2ERubyApp.create_app_headers_26_origin_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/origin", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"origin" => "https://example.com"})
    client.close
  end

  it "Referer header" do
    app = E2ERubyApp.create_app_headers_27_referer_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/headers/referer", headers: {"Referer" => "https://example.com/page"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"referer" => "https://example.com/page"})
    client.close
  end

  it "User-Agent header - custom value" do
    app = E2ERubyApp.create_app_headers_28_user_agent_header_custom_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"User-Agent" => "Mozilla/5.0 Custom Browser"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"User-Agent" => "Mozilla/5.0 Custom Browser"})
    client.close
  end

  it "User-Agent header - default value" do
    app = E2ERubyApp.create_app_headers_29_user_agent_header_default_value
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"User-Agent" => "testclient"})
    client.close
  end

  it "X-API-Key optional header - missing" do
    app = E2ERubyApp.create_app_headers_30_x_api_key_optional_header_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"msg" => "Hello World"})
    client.close
  end

  it "X-API-Key optional header - success" do
    app = E2ERubyApp.create_app_headers_31_x_api_key_optional_header_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me", headers: {"key" => "secret"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"msg" => "Hello secret"})
    client.close
  end

  it "X-API-Key required header - missing" do
    app = E2ERubyApp.create_app_headers_32_x_api_key_required_header_missing
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me")
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["headers", "x-api-key"])
    expect(body['errors'].first['type']).to eq("missing")
    client.close
  end

  it "X-API-Key required header - success" do
    app = E2ERubyApp.create_app_headers_33_x_api_key_required_header_success
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/users/me", headers: {"key" => "secret"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "secret"})
    client.close
  end

end
