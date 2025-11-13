# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "rate_limit" do
  it "Rate limit below threshold succeeds" do
    app = E2ERubyApp.create_app_rate_limit_1_rate_limit_below_threshold_succeeds
    client = Spikard::Testing.create_test_client(app)
    response = client.get("/rate-limit/basic")
    expect(response.status_code).to eq(200)
    expect(response.json).to eq({"request" => "under-limit", "status" => "ok"})
    client.close
  end

  it "Rate limit exceeded returns 429" do
    app = E2ERubyApp.create_app_rate_limit_2_rate_limit_exceeded_returns_429
    client = Spikard::Testing.create_test_client(app)
    1.times do
      warmup_response = client.get("/rate-limit/exceeded")
      expect(warmup_response.status_code).to eq(200)
      sleep 0
    end

    response = client.get("/rate-limit/exceeded")
    expect(response.status_code).to eq(429)
    client.close
  end

end
