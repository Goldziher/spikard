# frozen_string_literal: true

require 'spec_helper'
require_relative '../../app/main'

RSpec.describe "background" do
  it "Background event logging" do
    app = E2ERubyApp.create_app_background_1_background_event_logging
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/background/events", json: {"event" => "alpha"})
    expect(response.status_code).to eq(202)
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json")
    expected_state = {"events" => ["alpha"]}
    attempts = 0
    actual_state = nil
    begin
      state_response = client.get("/background/events")
      expect(state_response.status_code).to eq(200)
      actual_state = state_response.json
      break if actual_state == expected_state
      attempts += 1
      sleep 0.02
    end while attempts < 5
    expect(actual_state).to eq(expected_state)
    client.close
  end

  it "Background event logging - second payload" do
    app = E2ERubyApp.create_app_background_2_background_event_logging_second_payload
    client = Spikard::Testing.create_test_client(app)
    response = client.post("/background/events", json: {"event" => "beta"})
    expect(response.status_code).to eq(202)
    response_headers = response.headers.transform_keys { |key| key.downcase }
    expect(response_headers["content-type"]).to eq("application/json")
    expected_state = {"events" => ["beta"]}
    attempts = 0
    actual_state = nil
    begin
      state_response = client.get("/background/events")
      expect(state_response.status_code).to eq(200)
      actual_state = state_response.json
      break if actual_state == expected_state
      attempts += 1
      sleep 0.02
    end while attempts < 5
    expect(actual_state).to eq(expected_state)
    client.close
  end

end
