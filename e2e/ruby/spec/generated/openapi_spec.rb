# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "openapi" do
  it "OpenAPI spec generation - basic" do
    app = E2ERubyApp.create_app_openapi_1_openapi_spec_generation_basic
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "Test API",
      version: "1.0.0",
      description: "API for testing OpenAPI generation"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/openapi.json", headers: {"accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).not_to be_nil
    expect(response.body_text).to include("info")
    expect(response.body_text).to include("openapi")
    expect(response.body_text).to include("paths")
    client.close
  end

  it "OpenAPI spec with API key security scheme" do
    app = E2ERubyApp.create_app_openapi_2_openapi_spec_with_api_key_security_scheme
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "API Key Protected API",
      version: "1.0.0"
    )
    config.api_key_auth = Spikard::ApiKeyConfig.new(
      header_name: "X-API-Key",
      keys: ["test-key-123"]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/openapi.json", headers: {"accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).not_to be_nil
    expect(response.body_text).to include("components")
    expect(response.body_text).to include("openapi")
    expect(response.body_text).to include("security")
    client.close
  end

  it "OpenAPI spec with JWT security scheme" do
    app = E2ERubyApp.create_app_openapi_3_openapi_spec_with_jwt_security_scheme
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "Secure API",
      version: "1.0.0"
    )
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/openapi.json", headers: {"accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).not_to be_nil
    expect(response.body_text).to include("components")
    expect(response.body_text).to include("openapi")
    expect(response.body_text).to include("security")
    client.close
  end

  it "OpenAPI spec with custom metadata" do
    app = E2ERubyApp.create_app_openapi_4_openapi_spec_with_custom_metadata
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "Complete API",
      version: "2.0.0",
      description: "API with full metadata",
      contact: {"email" => "api@example.com", "name" => "API Team", "url" => "https://example.com/contact"},
      license: {"name" => "MIT", "url" => "https://opensource.org/licenses/MIT"},
      servers: [{"description" => "Production", "url" => "https://api.example.com"}, {"description" => "Development", "url" => "http://localhost:8000"}]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/openapi.json", headers: {"accept" => "application/json"})
    expect(response.status_code).to eq(200)
    expect(response.body_text).not_to be_nil
    expect(response.body_text).to include("info")
    expect(response.body_text).to include("openapi")
    expect(response.body_text).to include("servers")
    client.close
  end

  it "Redoc serving" do
    app = E2ERubyApp.create_app_openapi_5_redoc_serving
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "Status API",
      version: "1.0.0",
      redoc_path: "/redoc"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/redoc", headers: {"accept" => "text/html"})
    expect(response.status_code).to eq(200)
    client.close
  end

  it "Swagger UI serving" do
    app = E2ERubyApp.create_app_openapi_6_swagger_ui_serving
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.openapi = Spikard::OpenApiConfig.new(
      enabled: true,
      title: "Health API",
      version: "1.0.0",
      swagger_ui_path: "/docs"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/docs", headers: {"accept" => "text/html"})
    expect(response.status_code).to eq(200)
    client.close
  end

end
