# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "lifecycle_hooks" do
  it "Hook Execution Order" do
    app = E2ERubyApp.create_app_lifecycle_hooks_1_hook_execution_order
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/test-hook-order")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"execution_order" => ["first_hook", "second_hook", "third_hook"], "message" => "Hooks executed in order"})
    client.close
  end

  it "Multiple Hooks - All Phases" do
    app = E2ERubyApp.create_app_lifecycle_hooks_2_multiple_hooks_all_phases
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/full-lifecycle", headers: {"Authorization" => "Bearer valid-token-12345", "Content-Type" => "application/json"}, json: {"action" => "update_profile", "user_id" => "user-123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"action" => "update_profile", "message" => "Action completed successfully", "request_id" => ".*", "user_id" => "user-123"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-content-type-options"]).to eq("nosniff")
    expect(response_headers["x-frame-options"]).to eq("DENY")
    expect(response_headers["x-request-id"]).to eq(".*")
    expect(response_headers["x-response-time"]).to eq(".*ms")
    client.close
  end

  it "onError - Error Logging" do
    app = E2ERubyApp.create_app_lifecycle_hooks_3_onerror_error_logging
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/test-error")
    expect(response.status_code).to eq(500)
    expect(response.json).to eq({"error" => "Internal Server Error", "error_id" => ".*", "message" => "An unexpected error occurred"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json")
    client.close
  end

  it "onRequest - Request Logging" do
    app = E2ERubyApp.create_app_lifecycle_hooks_4_onrequest_request_logging
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/test-on-request")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"has_request_id" => true, "message" => "onRequest hooks executed", "request_logged" => true})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-request-id"]).to eq(".*")
    client.close
  end

  it "onResponse - Response Timing" do
    app = E2ERubyApp.create_app_lifecycle_hooks_5_onresponse_response_timing
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/test-timing")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Response with timing info"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-response-time"]).to eq(".*ms")
    client.close
  end

  it "onResponse - Security Headers" do
    app = E2ERubyApp.create_app_lifecycle_hooks_6_onresponse_security_headers
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/test-security-headers")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Response with security headers"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["strict-transport-security"]).to eq("max-age=31536000; includeSubDomains")
    expect(response_headers["x-content-type-options"]).to eq("nosniff")
    expect(response_headers["x-frame-options"]).to eq("DENY")
    expect(response_headers["x-xss-protection"]).to eq("1; mode=block")
    client.close
  end

  it "preHandler - Authentication Failed (Short Circuit)" do
    app = E2ERubyApp.create_app_lifecycle_hooks_7_prehandler_authentication_failed_short_circuit
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected-resource-fail", headers: {"Authorization" => "Bearer invalid-token"})
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"error" => "Unauthorized", "message" => "Invalid or expired authentication token"})
    client.close
  end

  it "preHandler - Authentication Success" do
    app = E2ERubyApp.create_app_lifecycle_hooks_8_prehandler_authentication_success
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/protected-resource", headers: {"Authorization" => "Bearer valid-token-12345"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"authenticated" => true, "message" => "Access granted", "user_id" => "user-123"})
    client.close
  end

  it "preHandler - Authorization Check" do
    app = E2ERubyApp.create_app_lifecycle_hooks_9_prehandler_authorization_check
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/admin-only", headers: {"Authorization" => "Bearer admin-token-67890"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Admin access granted", "role" => "admin", "user_id" => "admin-456"})
    client.close
  end

  it "preHandler - Authorization Forbidden (Short Circuit)" do
    app = E2ERubyApp.create_app_lifecycle_hooks_10_prehandler_authorization_forbidden_short_circuit
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/api/admin-only-forbidden", headers: {"Authorization" => "Bearer user-token-11111"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"error" => "Forbidden", "message" => "Admin role required for this endpoint"})
    client.close
  end

  it "preValidation - Rate Limit Exceeded (Short Circuit)" do
    app = E2ERubyApp.create_app_lifecycle_hooks_11_prevalidation_rate_limit_exceeded_short_circuit
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/test-rate-limit-exceeded", headers: {"Content-Type" => "application/json"}, json: {"data" => "test"})
    expect(response.status_code).to eq(429)
    expect(response.json).to eq({"error" => "Rate limit exceeded", "message" => "Too many requests, please try again later"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["retry-after"]).to eq("60")
    client.close
  end

  it "preValidation - Rate Limiting" do
    app = E2ERubyApp.create_app_lifecycle_hooks_12_prevalidation_rate_limiting
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/api/test-rate-limit", headers: {"Content-Type" => "application/json"}, json: {"data" => "test"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"message" => "Request accepted", "rate_limit_checked" => true})
    client.close
  end

end
