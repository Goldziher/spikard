# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "request_id" do
  it "Request ID header is preserved" do
    app = E2ERubyApp.create_app_request_id_1_request_id_header_is_preserved
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/request-id/preserved", headers: {"X-Request-ID" => "trace-123"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"echo" => "trace-123", "status" => "preserved"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-request-id"]).to eq("trace-123")
    client.close
  end

  it "Request ID is generated when not provided" do
    app = E2ERubyApp.create_app_request_id_2_request_id_is_generated_when_not_provided
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/request-id/generated")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"status" => "generated"})
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["x-request-id"]).to eq("00000000-0000-4000-8000-000000000000")
    client.close
  end

  it "Request ID middleware can be disabled" do
    app = E2ERubyApp.create_app_request_id_3_request_id_middleware_can_be_disabled
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/request-id/disabled", headers: {"X-Request-ID" => "external-id"})
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"status" => "no-request-id"})
    client.close
  end

end
