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

    expected = ["{\"level\":\"example_level\",\"message\":\"example_message\",\"source\":\"example_source\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"system_alert\"}", "{\"body\":\"example_body\",\"priority\":\"example_priority\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"title\":\"example_title\",\"type\":\"user_notification\",\"userId\":\"example_userId\"}", "{\"message\":\"example_message\",\"metadata\":{},\"service\":\"example_service\",\"status\":\"example_status\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"status_update\"}"]
    expect(events.length).to eq(expected.length)
    events.zip(expected).each do |payload, expected_json|
      expect(JSON.parse(payload)).to eq(JSON.parse(expected_json))
    end

    client.close
  end

end
