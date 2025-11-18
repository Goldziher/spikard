# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe "asyncapi_sse" do
  it "streams events for /notifications" do
    app = E2ERubyApp.create_app_sse_notifications
    client = Spikard::Testing.create_test_client(app)

    response = client.get("/notifications")
    expect(response.status_code).to eq(200)
    body = response.body_text
    events = body.gsub("\r\n", "\n")
                 .split("\n\n")
                 .select { |chunk| chunk.start_with?("data:") }
                 .map { |chunk| chunk.sub(/^data:\s*/, "").strip }

    expected = ["{\"level\":\"critical\",\"message\":\"Database connection pool exhausted\",\"source\":\"database-service\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}", "[{\"message\":\"example_message\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"example_type\"},{\"message\":\"example_message\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"example_type\"}]", "{\"body\":\"You have received a new direct message\",\"priority\":\"high\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"New message from John\",\"type\":\"user_notification\",\"userId\":\"user_12345\"}", "{\"message\":\"All systems operational\",\"metadata\":{\"region\":\"us-east-1\",\"uptime\":99.99},\"service\":\"payment-gateway\",\"status\":\"operational\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}"]
    expect(events.length).to eq(expected.length)
    events.zip(expected).each do |payload, expected_json|
      expect(JSON.parse(payload)).to eq(JSON.parse(expected_json))
    end

    client.close
  end

end
