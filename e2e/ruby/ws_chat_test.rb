#!/usr/bin/env ruby
# frozen_string_literal: true

# Test application generated from AsyncAPI specification

require 'faye/websocket'
require 'eventmachine'
require 'json'
require 'pathname'

# Load test fixtures
FIXTURES_DIR = Pathname.new(__FILE__).parent.parent + 'testing_data' + 'websockets'

def load_fixture(name)
  fixture_path = FIXTURES_DIR + "#{name}.json"
  raise "Fixture not found: #{fixture_path}" unless fixture_path.exist?
  JSON.parse(fixture_path.read)
end

def validate_message(message, fixture_name)
  fixture = load_fixture(fixture_name)
  schema = fixture['schema'] || {}
  required = schema['required'] || []

  # Basic validation - check required fields
  required.each do |field|
    unless message.key?(field)
      puts "❌ Missing required field: #{field}"
      return false
    end
  end

  puts "✓ Message validated against #{fixture_name}"
  true
rescue => e
  puts "❌ Validation error: #{e.message}"
  false
end

def handle_websocket(uri)
  puts "Connecting to #{uri}..."

  EM.run do
    ws = Faye::WebSocket::Client.new(uri)

    ws.on :open do |_event|
      puts '✓ Connected'

      # Send example messages
      fixture_userJoined = load_fixture('userJoined')
      example_userJoined = fixture_userJoined['examples'][0]
      puts 'Sending userJoined message...'
      ws.send(JSON.generate(example_userJoined))

      fixture_chatMessage = load_fixture('chatMessage')
      example_chatMessage = fixture_chatMessage['examples'][0]
      puts 'Sending chatMessage message...'
      ws.send(JSON.generate(example_chatMessage))

      fixture_userLeft = load_fixture('userLeft')
      example_userLeft = fixture_userLeft['examples'][0]
      puts 'Sending userLeft message...'
      ws.send(JSON.generate(example_userLeft))

    end

    ws.on :message do |event|
      message = JSON.parse(event.data)
      msg_type = message['type'] || 'unknown'
      puts "Received message type: #{msg_type}"

      # Validate based on message type
      validate_message(message, 'userJoined') if msg_type == 'userJoined'
      validate_message(message, 'chatMessage') if msg_type == 'chatMessage'
      validate_message(message, 'userLeft') if msg_type == 'userLeft'
    end

    ws.on :close do |event|
      puts "Connection closed: #{event.code} - #{event.reason}"
      EM.stop
    end

    ws.on :error do |event|
      puts "WebSocket error: #{event.message}"
      EM.stop
    end
  end
end

def main
  # Default WebSocket URI - override with environment variable WS_URI
  uri = ENV['WS_URI'] || 'ws://localhost:8000/chat'
  handle_websocket(uri)
end

# Run main function
main
