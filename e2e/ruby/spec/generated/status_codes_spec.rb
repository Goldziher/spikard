# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "status_codes" do
  it "19_413_payload_too_large" do
    app = E2ERubyApp.create_app_status_codes_1_19_413_payload_too_large
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, json: {"data" => "{{ repeat \'x\' 2000 times }}"})
    expect(response.status_code).to eq(413)
    expect(response.json).to eq({"error" => "Payload Too Large", "message" => "Request body size exceeds maximum allowed size of 1024 bytes"})
    client.close
  end

  it "200 OK - Success" do
    app = E2ERubyApp.create_app_status_codes_2_200_ok_success
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"id" => 1, "name" => "Item 1"})
    client.close
  end

  it "201 Created - Resource created" do
    app = E2ERubyApp.create_app_status_codes_3_201_created_resource_created
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: {"name" => "New Item"})
    expect(response.status_code).to eq(201)
    expect(response.json).to eq({"id" => 1, "name" => "New Item"})
    client.close
  end

  it "202 Accepted - Request accepted for processing" do
    app = E2ERubyApp.create_app_status_codes_4_202_accepted_request_accepted_for_processing
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: {"task" => "process_data"})
    expect(response.status_code).to eq(202)
    expect(response.json).to eq({"message" => "Task accepted for processing", "task_id" => "abc123"})
    client.close
  end

  it "204 No Content - Success with no body" do
    app = E2ERubyApp.create_app_status_codes_5_204_no_content_success_with_no_body
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.delete(path)
    expect(response.status_code).to eq(204)
    expect(response.body_text).to be_nil
    client.close
  end

  it "206 Partial Content" do
    app = E2ERubyApp.create_app_status_codes_6_206_partial_content
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, headers: {"Range" => "bytes=0-1023"})
    expect(response.status_code).to eq(206)
    expect(response.json).to eq("binary_data_1024_bytes")
    client.close
  end

  it "20_414_uri_too_long" do
    app = E2ERubyApp.create_app_status_codes_7_20_414_uri_too_long
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({})
    client.close
  end

  it "21_431_request_header_fields_too_large" do
    app = E2ERubyApp.create_app_status_codes_8_21_431_request_header_fields_too_large
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, headers: {"X-Large-Header" => "{{ repeat \'x\' 10000 times }}"})
    expect(response.status_code).to eq(431)
    expect(response.json).to eq({"error" => "Request Header Fields Too Large", "message" => "Request headers exceed maximum allowed size of 8192 bytes"})
    client.close
  end

  it "22_501_not_implemented" do
    app = E2ERubyApp.create_app_status_codes_9_22_501_not_implemented
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.trace(path)
    expect(response.status_code).to eq(405)
    expect(response.body_text).to be_nil
    client.close
  end

  it "23_503_service_unavailable" do
    app = E2ERubyApp.create_app_status_codes_10_23_503_service_unavailable
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(503)
    expect(response.json).to eq({"error" => "Service Unavailable", "message" => "The service is temporarily unavailable. Please try again later."})
    client.close
  end

  it "301 Moved Permanently - Permanent redirect" do
    app = E2ERubyApp.create_app_status_codes_11_301_moved_permanently_permanent_redirect
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(301)
    expect(response.body_text).to be_nil
    client.close
  end

  it "302 Found - Temporary redirect" do
    app = E2ERubyApp.create_app_status_codes_12_302_found_temporary_redirect
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(302)
    expect(response.body_text).to be_nil
    client.close
  end

  it "304 Not Modified - Cached content valid" do
    app = E2ERubyApp.create_app_status_codes_13_304_not_modified_cached_content_valid
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, headers: {"If-None-Match" => "\"abc123\""})
    expect(response.status_code).to eq(304)
    expect(response.body_text).to be_nil
    client.close
  end

  it "307 Temporary Redirect - Method preserved" do
    app = E2ERubyApp.create_app_status_codes_14_307_temporary_redirect_method_preserved
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: {})
    expect(response.status_code).to eq(307)
    expect(response.json).to eq({})
    client.close
  end

  it "400 Bad Request - Invalid request" do
    app = E2ERubyApp.create_app_status_codes_15_400_bad_request_invalid_request
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: "not valid json")
    expect(response.status_code).to eq(400)
    expect(response.json).to eq({"detail" => "Invalid request format"})
    client.close
  end

  it "401 Unauthorized - Missing authentication" do
    app = E2ERubyApp.create_app_status_codes_16_401_unauthorized_missing_authentication
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(401)
    expect(response.json).to eq({"detail" => "Not authenticated"})
    client.close
  end

  it "403 Forbidden - Insufficient permissions" do
    app = E2ERubyApp.create_app_status_codes_17_403_forbidden_insufficient_permissions
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path, headers: {"Authorization" => "Bearer valid_token"})
    expect(response.status_code).to eq(403)
    expect(response.json).to eq({"detail" => "Not enough permissions"})
    client.close
  end

  it "404 Not Found - Resource not found" do
    app = E2ERubyApp.create_app_status_codes_18_404_not_found_resource_not_found
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(404)
    expect(response.json).to eq({"detail" => "Item not found"})
    client.close
  end

  it "408 Request Timeout" do
    app = E2ERubyApp.create_app_status_codes_19_408_request_timeout
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: {"data" => "large_data"})
    expect(response.status_code).to eq(408)
    expect(response.json).to eq({"detail" => "Request timeout"})
    client.close
  end

  it "422 Unprocessable Entity - Validation error" do
    app = E2ERubyApp.create_app_status_codes_20_422_unprocessable_entity_validation_error
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.post(path, headers: {"Content-Type" => "application/json"}, json: {"price" => "not a number"})
    expect(response.status_code).to eq(422)
    body = response.json
    expect(body).to be_a(Hash)
    expect(body['errors']).to be_an(Array)
    expect(body['errors']).not_to be_empty
    expect(body['detail']).to eq("1 validation error in request")
    expect(body['status']).to eq(422)
    expect(body['errors'].first['loc']).to eq(["body", "name"])
    client.close
  end

  it "429 Too Many Requests" do
    app = E2ERubyApp.create_app_status_codes_21_429_too_many_requests
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(429)
    expect(response.json).to eq({"detail" => "Rate limit exceeded. Try again in 60 seconds."})
    client.close
  end

  it "500 Internal Server Error - Server error" do
    app = E2ERubyApp.create_app_status_codes_22_500_internal_server_error_server_error
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(500)
    expect(response.json).to eq({"detail" => "Internal server error", "status" => 500, "title" => "Internal Server Error", "type" => "https://spikard.dev/errors/internal-server-error"})
    client.close
  end

  it "503 Service Unavailable - Server overload" do
    app = E2ERubyApp.create_app_status_codes_23_503_service_unavailable_server_overload
    client = Spikard::Testing.create_test_client(app)
    path = app.route_metadata.first[:path]
    response = client.get(path)
    expect(response.status_code).to eq(503)
    expect(response.json).to eq({"detail" => "Service temporarily unavailable"})
    client.close
  end

end
