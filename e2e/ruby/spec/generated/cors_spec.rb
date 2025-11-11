# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "cors" do
  it "08_cors_max_age" do
    app = E2ERubyApp.create_app_cors_1_08_cors_max_age
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/api/data", headers: {"Access-Control-Request-Headers" => "Content-Type", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "09_cors_expose_headers" do
    app = E2ERubyApp.create_app_cors_2_09_cors_expose_headers
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "10_cors_origin_null" do
    app = E2ERubyApp.create_app_cors_3_10_cors_origin_null
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"Origin" => "null"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"error" => "Origin \'null\' is not allowed"})
    client.close
  end

  it "CORS preflight request" do
    app = E2ERubyApp.create_app_cors_4_cors_preflight_request
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.options("/items/", headers: {"Access-Control-Request-Headers" => "Content-Type, X-Custom-Header", "Access-Control-Request-Method" => "POST", "Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).to be_nil
    client.close
  end

  it "CORS request blocked" do
    app = E2ERubyApp.create_app_cors_5_cors_request_blocked
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"Origin" => "https://malicious-site.com"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"detail" => "CORS request from origin \'https://malicious-site.com\' not allowed"})
    client.close
  end

  it "CORS wildcard origin" do
    app = E2ERubyApp.create_app_cors_6_cors_wildcard_origin
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/public/data", headers: {"Origin" => "https://random-site.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "public"})
    client.close
  end

  it "CORS with credentials" do
    app = E2ERubyApp.create_app_cors_7_cors_with_credentials
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/user/profile", headers: {"Cookie" => "session=abc123", "Origin" => "https://app.example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"username" => "john"})
    client.close
  end

  it "Simple CORS request" do
    app = E2ERubyApp.create_app_cors_8_simple_cors_request
    config = Spikard::ServerConfig.new
    config.compression = nil
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/items/", headers: {"Origin" => "https://example.com"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"items" => []})
    client.close
  end

end
