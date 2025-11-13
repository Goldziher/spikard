# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "auth" do
  it "API key authentication - invalid key" do
    app = E2ERubyApp.create_app_auth_1_api_key_authentication_invalid_key
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"X-API-Key" => "invalid_key_12345"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "The provided API key is not valid", "status" => 401, "title" => "Invalid API key", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "API key authentication - missing header" do
    app = E2ERubyApp.create_app_auth_2_api_key_authentication_missing_header
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data")
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Expected \'X-API-Key\' header or \'api_key\' query parameter with valid API key", "status" => 401, "title" => "Missing API key", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "API key authentication - valid key" do
    app = E2ERubyApp.create_app_auth_3_api_key_authentication_valid_key
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"X-API-Key" => "sk_test_123456"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "sensitive information", "message" => "Access granted"})
    client.close
  end

  it "API key in query parameter" do
    app = E2ERubyApp.create_app_auth_4_api_key_in_query_parameter
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data?api_key=sk_test_123456")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "sensitive information", "message" => "Access granted"})
    client.close
  end

  it "API key rotation - old key still valid" do
    app = E2ERubyApp.create_app_auth_5_api_key_rotation_old_key_still_valid
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"X-API-Key" => "sk_test_old_123456"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "sensitive information", "message" => "Access granted"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-api-key-deprecated"]).to eq("true")
    client.close
  end

  it "API key with custom header name" do
    app = E2ERubyApp.create_app_auth_6_api_key_with_custom_header_name
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"X-API-Token" => "sk_test_123456"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"data" => "sensitive information", "message" => "Access granted"})
    client.close
  end

  it "Bearer token without prefix" do
    app = E2ERubyApp.create_app_auth_7_bearer_token_without_prefix
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected", headers: {"Authorization" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDZ9.8yXqZ9jKCR0BwqJc7pN_QvD3mYLxHfWzUeIaGkTnOsA"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Authorization header must use Bearer scheme: \'Bearer <token>\'", "status" => 401, "title" => "Invalid Authorization header format", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - expired token" do
    app = E2ERubyApp.create_app_auth_8_jwt_authentication_expired_token
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoxNjAwMDAwMDAwLCJpYXQiOjE1OTAwMDAwMDB9.n4oBw9XuO2aAJWi1e4Bz9Y_m2iEyJHGAODcetNuwYFo"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token has expired", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - invalid audience" do
    app = E2ERubyApp.create_app_auth_9_jwt_authentication_invalid_audience
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTk5LCJpYXQiOjE3MzEyNTIwMDAsImF1ZCI6WyJodHRwczovL3dyb25nLXNlcnZpY2UuY29tIl19.YR2a9fSJjhen7ksYFI2djSBSC7Pc29FDCloBGhkj3kU"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token audience is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - invalid signature" do
    app = E2ERubyApp.create_app_auth_10_jwt_authentication_invalid_signature
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNTM0MDIzMDA3OTksImlhdCI6MTczMTI1MjAwMH0.invalid_signature_here"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token signature is invalid", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - missing Authorization header" do
    app = E2ERubyApp.create_app_auth_11_jwt_authentication_missing_authorization_header
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/protected/user")
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Expected \'Authorization: Bearer <token>\'", "status" => 401, "title" => "Missing or invalid Authorization header", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT authentication - valid token" do
    app = E2ERubyApp.create_app_auth_12_jwt_authentication_valid_token
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/protected/user", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Access granted", "user_id" => "user123"})
    client.close
  end

  it "JWT invalid issuer" do
    app = E2ERubyApp.create_app_auth_13_jwt_invalid_issuer
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2V2aWwuY29tIn0.mbL5L04_hpaaiz0SPABap6ZWfBLu18aiexBjzwQ1nnA"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Token issuer is invalid, expected \'https://auth.example.com\'", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT malformed token format" do
    app = E2ERubyApp.create_app_auth_14_jwt_malformed_token_format
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected", headers: {"Authorization" => "Bearer invalid.token"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Malformed JWT token: expected 3 parts separated by dots, found 2", "status" => 401, "title" => "Malformed JWT token", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT missing required custom claims" do
    app = E2ERubyApp.create_app_auth_15_jwt_missing_required_custom_claims
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/admin", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"detail" => "Required claims \'role\' and \'permissions\' missing from JWT", "status" => 403, "title" => "Forbidden", "type" => "https://spikard.dev/errors/forbidden"})
    client.close
  end

  it "JWT not before claim in future" do
    app = E2ERubyApp.create_app_auth_16_jwt_not_before_claim_in_future
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsIm5iZiI6MjYyNjc4Mzk0NiwiYXVkIjpbImh0dHBzOi8vYXBpLmV4YW1wbGUuY29tIl0sImlzcyI6Imh0dHBzOi8vYXV0aC5leGFtcGxlLmNvbSJ9.hG4I76_3kJfsbJ_jmxoP1NSYnkcqdyBFcPpdo-jYU4E"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "JWT not valid yet, not before claim is in the future", "status" => 401, "title" => "JWT validation failed", "type" => "https://spikard.dev/errors/unauthorized"})
    client.close
  end

  it "JWT with multiple audiences" do
    app = E2ERubyApp.create_app_auth_17_jwt_with_multiple_audiences
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE2MDAwMDAwMDAsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSIsImh0dHBzOi8vYWRtaW4uZXhhbXBsZS5jb20iXSwiaXNzIjoiaHR0cHM6Ly9hdXRoLmV4YW1wbGUuY29tIn0.9MBL_XccGXfu9cDUnCpQruDMOl2hHYydzeGn-20dQOs"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Access granted", "user_id" => "user123"})
    client.close
  end

  it "Multiple authentication schemes - JWT precedence" do
    app = E2ERubyApp.create_app_auth_18_multiple_authentication_schemes_jwt_precedence
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/data", headers: {"Authorization" => "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyMTIzIiwiZXhwIjoyNjI2NzgzOTQ2LCJpYXQiOjE3NjI3ODM5NDYsImF1ZCI6WyJodHRwczovL2FwaS5leGFtcGxlLmNvbSJdLCJpc3MiOiJodHRwczovL2F1dGguZXhhbXBsZS5jb20ifQ.TpRpCJeXROQ12-ehRCVZm6EgN7Dn6QpfoekxJvnzgQg", "X-API-Key" => "sk_test_123456"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"auth_method" => "jwt", "message" => "Access granted", "user_id" => "user123"})
    client.close
  end

end
