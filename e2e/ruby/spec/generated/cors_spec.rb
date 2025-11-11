# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "cors" do
  it "06_cors_preflight_method_not_allowed" do
    app = E2ERubyApp.create_app_cors_1_06_cors_preflight_method_not_allowed
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "DELETE", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(403)
    expect(response.body_text).to be_nil
    client.close
  end

  it "07_cors_preflight_header_not_allowed" do
    app = E2ERubyApp.create_app_cors_2_07_cors_preflight_header_not_allowed
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "X-Custom-Header", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(403)
    expect(response.body_text).to be_nil
    client.close
  end

  it "08_cors_max_age" do
    app = E2ERubyApp.create_app_cors_3_08_cors_max_age
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "09_cors_expose_headers" do
    app = E2ERubyApp.create_app_cors_4_09_cors_expose_headers
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "10_cors_origin_null" do
    app = E2ERubyApp.create_app_cors_5_10_cors_origin_null
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "null"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"error" => "Origin \'null\' is not allowed"})
    client.close
  end

  it "CORS Private Network Access" do
    app = E2ERubyApp.create_app_cors_6_cors_private_network_access
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/local-resource", headers: {"Access-Control-Request-Method" => "GET", "Access-Control-Request-Private-Network" => "true", "Origin" => "https://public.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS Vary header for proper caching" do
    app = E2ERubyApp.create_app_cors_7_cors_vary_header_for_proper_caching
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/cached-resource", headers: {"Cache-Control" => "max-age=3600", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "cacheable resource"})
    client.close
  end

  it "CORS multiple allowed origins" do
    app = E2ERubyApp.create_app_cors_8_cors_multiple_allowed_origins
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "https://admin.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "resource data"})
    client.close
  end

  it "CORS origin case sensitivity" do
    app = E2ERubyApp.create_app_cors_9_cors_origin_case_sensitivity
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "https://EXAMPLE.COM"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS preflight for DELETE method" do
    app = E2ERubyApp.create_app_cors_10_cors_preflight_for_delete_method
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/resource/456", headers: {"Access-Control-Request-Method" => "DELETE", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS preflight for PUT method" do
    app = E2ERubyApp.create_app_cors_11_cors_preflight_for_put_method
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/resource/123", headers: {"Access-Control-Request-Headers" => "Content-Type, X-Custom-Header", "Access-Control-Request-Method" => "PUT", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS preflight request" do
    app = E2ERubyApp.create_app_cors_12_cors_preflight_request
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/items/", headers: {"Access-Control-Request-Headers" => "Content-Type, X-Custom-Header", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS regex pattern matching for origins" do
    app = E2ERubyApp.create_app_cors_13_cors_regex_pattern_matching_for_origins
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "https://subdomain.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "resource data"})
    client.close
  end

  it "CORS request blocked" do
    app = E2ERubyApp.create_app_cors_14_cors_request_blocked
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"Origin" => "https://malicious-site.com"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"detail" => "CORS request from origin \'https://malicious-site.com\' not allowed"})
    client.close
  end

  it "CORS safelisted headers without preflight" do
    app = E2ERubyApp.create_app_cors_15_cors_safelisted_headers_without_preflight
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.post("/api/form", headers: {"Accept" => "application/json", "Accept-Language" => "en-US", "Content-Type" => "text/plain", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Success"})
    client.close
  end

  it "CORS wildcard origin" do
    app = E2ERubyApp.create_app_cors_16_cors_wildcard_origin
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/public/data", headers: {"Origin" => "https://random-site.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "public"})
    client.close
  end

  it "CORS with credentials" do
    app = E2ERubyApp.create_app_cors_17_cors_with_credentials
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/user/profile", headers: {"Cookie" => "session=abc123", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "john"})
    client.close
  end

  it "Simple CORS request" do
    app = E2ERubyApp.create_app_cors_18_simple_cors_request
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"items" => []})
    client.close
  end

end
