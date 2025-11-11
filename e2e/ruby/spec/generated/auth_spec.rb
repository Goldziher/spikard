# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "auth" do
  it "API key authentication - invalid key" do
    app = E2ERubyApp.create_app_auth_1_api_key_authentication_invalid_key
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.api_key_auth = Spikard::ApiKeyConfig.new(
      header_name: "X-API-Key",
      keys: ["sk_test_123456", "sk_test_789012"]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"X-API-Key" => "invalid_key_12345"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "The provided API key is not valid", "status" => 401, "title" => "Invalid API key", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "API key authentication - missing header" do
    app = E2ERubyApp.create_app_auth_2_api_key_authentication_missing_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.api_key_auth = Spikard::ApiKeyConfig.new(
      header_name: "X-API-Key",
      keys: ["sk_test_123456", "sk_test_789012"]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data")
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Expected \'X-API-Key\' header with valid API key", "status" => 401, "title" => "Missing API key", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "API key authentication - valid key" do
    app = E2ERubyApp.create_app_auth_3_api_key_authentication_valid_key
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.api_key_auth = Spikard::ApiKeyConfig.new(
      header_name: "X-API-Key",
      keys: ["sk_test_123456", "sk_test_789012"]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/api/data", headers: {"X-API-Key" => "sk_test_123456"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "sensitive information", "message" => "Access granted"})
    client.close
  end

  it "JWT authentication - expired token" do
    app = E2ERubyApp.create_app_auth_4_jwt_authentication_expired_token
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key-do-not-use-in-production"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token has expired", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - invalid audience" do
    app = E2ERubyApp.create_app_auth_5_jwt_authentication_invalid_audience
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key-do-not-use-in-production",
      audience: ["https://api.example.com"]
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token audience is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - invalid signature" do
    app = E2ERubyApp.create_app_auth_6_jwt_authentication_invalid_signature
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key-do-not-use-in-production"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token signature is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - missing Authorization header" do
    app = E2ERubyApp.create_app_auth_7_jwt_authentication_missing_authorization_header
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key-do-not-use-in-production"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected/user")
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Expected \'Authorization: Bearer <token>\'", "status" => 401, "title" => "Missing or invalid Authorization header", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - valid token" do
    app = E2ERubyApp.create_app_auth_8_jwt_authentication_valid_token
    config = Spikard::ServerConfig.new
    config.compression = nil
    config.jwt_auth = Spikard::JwtConfig.new(
      algorithm: "HS256",
      secret: "test-secret-key-do-not-use-in-production",
      audience: ["https://api.example.com"],
      issuer: "https://auth.example.com"
    )
    client = Spikard::Testing.create_test_client(app, config: config)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Access granted", "user_id" => "user123"})
    client.close
  end

end
