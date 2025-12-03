#!/usr/bin/env ruby
# frozen_string_literal: true

# Basic Server Example
#
# The simplest possible Spikard application.
# Starts a server on port 8000 with routes that return JSON.

require 'spikard'
require 'json'

# Create application instance
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# Simple GET handler returning plain text
app.get '/' do |request|
  'Hello, World!'
end

# Health check endpoint
app.get '/health' do |request|
  {
    status: 'ok',
    timestamp: Time.now.iso8601
  }
end

# Echo endpoint
app.post '/echo' do |request|
  if request.body.is_a?(Hash)
    {
      echo: request.body,
      received_at: Time.now.iso8601
    }
  else
    {
      status: 400,
      body: {
        error: 'Request body must be JSON',
        code: 'invalid_body'
      }
    }
  end
end

puts 'Starting Spikard Ruby server on http://127.0.0.1:8000'
puts 'Press Ctrl+C to stop'
puts ''

# Run the server
app.run
