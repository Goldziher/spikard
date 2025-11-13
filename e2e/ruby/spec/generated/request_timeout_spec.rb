# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "request_timeout" do
  it "Request completes before timeout" do
    app = E2ERubyApp.create_app_request_timeout_1_request_completes_before_timeout
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/timeouts/fast")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"duration" => "fast", "status" => "ok"})
    client.close
  end

  it "Request exceeds timeout" do
    app = E2ERubyApp.create_app_request_timeout_2_request_exceeds_timeout
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/timeouts/slow")
    expect(response.status_code).to eq(408)
    client.close
  end

end
