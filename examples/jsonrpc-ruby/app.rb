#!/usr/bin/env ruby

# frozen_string_literal: true

require 'spikard'
require 'json'

# JSON-RPC 2.0 Example Application
#
# This example demonstrates how to use Spikard with JSON-RPC 2.0 method metadata
# to build RPC endpoints with automatic documentation support.
#
# Usage:
#   bundle exec ruby app.rb
#
# Then invoke methods via curl:
#   curl -X POST http://localhost:8000/rpc \
#     -H "Content-Type: application/json" \
#     -d '{"jsonrpc": "2.0", "method": "math.add", "params": {"a": 5, "b": 3}, "id": 1}'

# Create the application
app = Spikard::App.new

# Define JSON-RPC method metadata for math.add
add_info = {
  method_name: 'math.add',
  description: 'Add two numbers and return the result',
  params_schema: {
    type: 'object',
    properties: {
      a: { type: 'number' },
      b: { type: 'number' }
    },
    required: ['a', 'b']
  },
  result_schema: { type: 'number' },
  tags: ['math', 'arithmetic']
}

# Register JSON-RPC route for addition
app.post('/rpc', jsonrpc_method: add_info) do |a:, b:|
  a + b
end

# Define JSON-RPC method metadata for math.subtract
subtract_info = {
  method_name: 'math.subtract',
  description: 'Subtract two numbers and return the result',
  params_schema: {
    type: 'object',
    properties: {
      a: { type: 'number' },
      b: { type: 'number' }
    },
    required: ['a', 'b']
  },
  result_schema: { type: 'number' },
  tags: ['math', 'arithmetic']
}

# Register JSON-RPC route for subtraction
app.post('/rpc', jsonrpc_method: subtract_info) do |a:, b:|
  a - b
end

# Define JSON-RPC method metadata for math.multiply
multiply_info = {
  method_name: 'math.multiply',
  description: 'Multiply two numbers and return the result',
  params_schema: {
    type: 'object',
    properties: {
      a: { type: 'number' },
      b: { type: 'number' }
    },
    required: ['a', 'b']
  },
  result_schema: { type: 'number' },
  tags: ['math', 'arithmetic']
}

# Register JSON-RPC route for multiplication
app.post('/rpc', jsonrpc_method: multiply_info) do |a:, b:|
  a * b
end

# Define JSON-RPC method metadata for user.create
create_user_info = {
  method_name: 'user.create',
  description: 'Create a new user with email and name',
  params_schema: {
    type: 'object',
    properties: {
      email: {
        type: 'string',
        format: 'email'
      },
      name: { type: 'string' }
    },
    required: ['email', 'name']
  },
  result_schema: {
    type: 'object',
    properties: {
      id: { type: 'integer' },
      email: { type: 'string' },
      name: { type: 'string' },
      created_at: { type: 'string', format: 'date-time' }
    }
  },
  tags: ['users', 'admin']
}

# Register JSON-RPC route for user creation
app.post('/rpc', jsonrpc_method: create_user_info) do |email:, name:|
  {
    id: rand(1000..9999),
    email: email,
    name: name,
    created_at: Time.now.iso8601
  }
end

# Define JSON-RPC method metadata for user.getById
get_user_info = {
  method_name: 'user.getById',
  description: 'Get a user by their ID',
  params_schema: {
    type: 'object',
    properties: {
      id: { type: 'integer' }
    },
    required: ['id']
  },
  result_schema: {
    type: 'object',
    properties: {
      id: { type: 'integer' },
      email: { type: 'string' },
      name: { type: 'string' }
    }
  },
  tags: ['users']
}

# Register JSON-RPC route for getting user by ID
app.post('/rpc', jsonrpc_method: get_user_info) do |id:|
  {
    id: id,
    email: "user#{id}@example.com",
    name: "User #{id}"
  }
end

# Health check endpoint (non-JSON-RPC)
app.get('/health') do
  { status: 'healthy' }
end

# Run the application
puts 'Starting Spikard JSON-RPC server on http://localhost:8000'
puts 'Available JSON-RPC methods:'
puts '  - math.add'
puts '  - math.subtract'
puts '  - math.multiply'
puts '  - user.create'
puts '  - user.getById'
puts ''
puts 'Press Ctrl+C to stop'
puts ''

app.run(host: '0.0.0.0', port: 8000)
