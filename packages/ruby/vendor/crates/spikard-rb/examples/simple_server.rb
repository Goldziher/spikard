#!/usr/bin/env ruby
# frozen_string_literal: true

# Add lib to load path for development
$LOAD_PATH.unshift File.expand_path('../lib', __dir__)

require 'spikard'
require 'json'

# Define routes
routes = [
  {
    method: 'GET',
    path: '/',
    handler_name: 'root',
    is_async: false
  },
  {
    method: 'GET',
    path: '/hello/:name',
    handler_name: 'hello',
    is_async: false
  },
  {
    method: 'POST',
    path: '/echo',
    handler_name: 'echo',
    is_async: false
  },
  {
    method: 'GET',
    path: '/users/:user_id',
    handler_name: 'get_user',
    is_async: false
  }
]

# Define handlers
handlers = {
  'root' => lambda do |_request|
    {
      message: 'Welcome to Spikard Ruby!',
      version: Spikard::VERSION,
      endpoints: [
        'GET /',
        'GET /hello/:name',
        'POST /echo',
        'GET /users/:user_id'
      ]
    }
  end,

  'hello' => lambda do |request|
    name = request[:params]['name']
    {
      message: "Hello, #{name}!",
      timestamp: Time.now.to_i
    }
  end,

  'echo' => lambda do |request|
    {
      echo: request[:body],
      received_at: Time.now.to_i
    }
  end,

  'get_user' => lambda do |request|
    user_id = request[:params]['user_id']
    {
      user: {
        id: user_id.to_i,
        name: "User #{user_id}",
        email: "user#{user_id}@example.com"
      }
    }
  end
}

# Create app
puts "Creating Spikard app with #{routes.length} routes..."
app = Spikard.create_app(routes, handlers)

# Start server
host = ENV['HOST'] || '127.0.0.1'
port = (ENV['PORT'] || 8000).to_i

puts "Starting server on #{host}:#{port}"
puts 'Press Ctrl+C to stop'
puts

Spikard.run_server(app, host, port)

# Keep the program running
sleep
