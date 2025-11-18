# frozen_string_literal: true

require 'spec_helper'
require 'json'

RSpec.describe "asyncapi_websocket" do
  it "echoes WebSocket message 1 on /chat" do
    app = E2ERubyApp.create_app_websocket_chat
    client = Spikard::Testing.create_test_client(app)
    ws = client.websocket("/chat")

    message = JSON.parse("{\"text\":\"Hello, everyone!\",\"timestamp\":\"2024-01-15T10:30:00Z\",\"type\":\"message\",\"user\":\"alice\"}")
    ws.send_json(message)
    response = ws.receive_json

    expect(response['validated']).to eq(true)
    message.each do |key, value|
      expect(response[key]).to eq(value)
    end

    ws.close
    client.close
  end

  it "echoes WebSocket message 2 on /chat" do
    app = E2ERubyApp.create_app_websocket_chat
    client = Spikard::Testing.create_test_client(app)
    ws = client.websocket("/chat")

    message = JSON.parse("{\"timestamp\":\"2024-01-15T10:35:00Z\",\"type\":\"userLeft\",\"user\":\"charlie\"}")
    ws.send_json(message)
    response = ws.receive_json

    expect(response['validated']).to eq(true)
    message.each do |key, value|
      expect(response[key]).to eq(value)
    end

    ws.close
    client.close
  end

  it "echoes WebSocket message 3 on /chat" do
    app = E2ERubyApp.create_app_websocket_chat
    client = Spikard::Testing.create_test_client(app)
    ws = client.websocket("/chat")

    message = JSON.parse("{\"timestamp\":\"2024-01-15T10:29:55Z\",\"type\":\"userJoined\",\"user\":\"bob\"}")
    ws.send_json(message)
    response = ws.receive_json

    expect(response['validated']).to eq(true)
    message.each do |key, value|
      expect(response[key]).to eq(value)
    end

    ws.close
    client.close
  end

end
