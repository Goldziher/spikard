# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "cors" do
  it "06_cors_preflight_method_not_allowed" do
    app = E2ERubyApp.create_app_cors_1_06_cors_preflight_method_not_allowed
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "DELETE", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(403)
    client.close
  end

  it "07_cors_preflight_header_not_allowed" do
    app = E2ERubyApp.create_app_cors_2_07_cors_preflight_header_not_allowed
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "X-Custom-Header", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(403)
    client.close
  end

  it "08_cors_max_age" do
    app = E2ERubyApp.create_app_cors_3_08_cors_max_age
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-headers"]).to eq("Content-Type")
    expect(response_headers["access-control-allow-methods"]).to eq("POST")
    expect(response_headers["access-control-allow-origin"]).to eq("https://example.com")
    expect(response_headers["access-control-max-age"]).to eq("3600")
    client.close
  end

  it "09_cors_expose_headers" do
    app = E2ERubyApp.create_app_cors_4_09_cors_expose_headers
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://example.com")
    expect(response_headers["access-control-expose-headers"]).to eq("X-Total-Count, X-Request-Id")
    expect(response_headers["x-request-id"]).to eq("abc123")
    expect(response_headers["x-total-count"]).to eq("42")
    client.close
  end

  it "10_cors_origin_null" do
    app = E2ERubyApp.create_app_cors_5_10_cors_origin_null
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Origin" => "null"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"error" => "Origin \'null\' is not allowed"})
    client.close
  end

  it "CORS Private Network Access" do
    app = E2ERubyApp.create_app_cors_6_cors_private_network_access
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/local-resource", headers: {"Access-Control-Request-Method" => "GET", "Access-Control-Request-Private-Network" => "true", "Origin" => "https://public.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-methods"]).to eq("GET, POST")
    expect(response_headers["access-control-allow-origin"]).to eq("https://public.example.com")
    expect(response_headers["access-control-allow-private-network"]).to eq("true")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS Vary header for proper caching" do
    app = E2ERubyApp.create_app_cors_7_cors_vary_header_for_proper_caching
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/cached-resource", headers: {"Cache-Control" => "max-age=3600", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "cacheable resource"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://app.example.com")
    expect(response_headers["cache-control"]).to eq("public, max-age=3600")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS multiple allowed origins" do
    app = E2ERubyApp.create_app_cors_8_cors_multiple_allowed_origins
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Origin" => "https://admin.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "resource data"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://admin.example.com")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS origin case sensitivity" do
    app = E2ERubyApp.create_app_cors_9_cors_origin_case_sensitivity
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Origin" => "https://EXAMPLE.COM"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS preflight for DELETE method" do
    app = E2ERubyApp.create_app_cors_10_cors_preflight_for_delete_method
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/resource/456", headers: {"Access-Control-Request-Method" => "DELETE", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-methods"]).to eq("GET, POST, PUT, PATCH, DELETE")
    expect(response_headers["access-control-allow-origin"]).to eq("https://app.example.com")
    expect(response_headers["access-control-max-age"]).to eq("3600")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS preflight for PUT method" do
    app = E2ERubyApp.create_app_cors_11_cors_preflight_for_put_method
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/api/resource/123", headers: {"Access-Control-Request-Headers" => "Content-Type, X-Custom-Header", "Access-Control-Request-Method" => "PUT", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-headers"]).to eq("Content-Type, X-Custom-Header")
    expect(response_headers["access-control-allow-methods"]).to eq("GET, POST, PUT, PATCH, DELETE")
    expect(response_headers["access-control-allow-origin"]).to eq("https://app.example.com")
    expect(response_headers["access-control-max-age"]).to eq("3600")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS preflight request" do
    app = E2ERubyApp.create_app_cors_12_cors_preflight_request
    client = Spikard::Testing.create_test_client(app)
    response = client.options("/items/", headers: {"Access-Control-Request-Headers" => "Content-Type, X-Custom-Header", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-headers"]).to eq("Content-Type, X-Custom-Header")
    expect(response_headers["access-control-allow-methods"]).to eq("GET, POST, PUT, DELETE, OPTIONS")
    expect(response_headers["access-control-allow-origin"]).to eq("https://example.com")
    expect(response_headers["access-control-max-age"]).to eq("600")
    client.close
  end

  it "CORS regex pattern matching for origins" do
    app = E2ERubyApp.create_app_cors_13_cors_regex_pattern_matching_for_origins
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Origin" => "https://subdomain.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "resource data"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://subdomain.example.com")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS request blocked" do
    app = E2ERubyApp.create_app_cors_14_cors_request_blocked
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/", headers: {"Origin" => "https://malicious-site.com"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"detail" => "CORS request from origin \'https://malicious-site.com\' not allowed"})
    client.close
  end

  it "CORS safelisted headers without preflight" do
    app = E2ERubyApp.create_app_cors_15_cors_safelisted_headers_without_preflight
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/form", headers: {"Accept" => "application/json", "Accept-Language" => "en-US", "Content-Type" => "text/plain", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Success"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://app.example.com")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "CORS wildcard origin" do
    app = E2ERubyApp.create_app_cors_16_cors_wildcard_origin
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/public/data", headers: {"Origin" => "https://random-site.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "public"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("*")
    client.close
  end

  it "CORS with credentials" do
    app = E2ERubyApp.create_app_cors_17_cors_with_credentials
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/user/profile", headers: {"Cookie" => "session=abc123", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "john"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-credentials"]).to eq("true")
    expect(response_headers["access-control-allow-origin"]).to eq("https://app.example.com")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

  it "Simple CORS request" do
    app = E2ERubyApp.create_app_cors_18_simple_cors_request
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/items/", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"items" => []})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["access-control-allow-origin"]).to eq("https://example.com")
    expect(response_headers["vary"]).to eq("Origin")
    client.close
  end

end
