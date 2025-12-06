#!/usr/bin/env ruby
# frozen_string_literal: true

# Validation Example
#
# Demonstrates JSON request body validation, query parameter handling,
# path parameters, and structured error responses.

require 'spikard'
require 'json'

# Create application
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# Simple in-memory user store
@users = {
  1 => { id: 1, name: 'Alice', email: 'alice@example.com' },
  2 => { id: 2, name: 'Bob', email: 'bob@example.com' }
}

# GET endpoint returning list of users with optional filtering
app.get '/users' do |request|
  users = @users.values

  # Filter by name if query param provided
  if request.query['name']
    name_filter = request.query['name'].downcase
    users = users.select { |u| u[:name].downcase.include?(name_filter) }
  end

  {
    users: users,
    count: users.length
  }
end

# GET endpoint returning single user by ID
app.get '/users/:id' do |request|
  user_id = request.params['id'].to_i

  # Validate ID is a number
  if user_id.zero? && request.params['id'] != '0'
    return {
      status: 400,
      body: {
        error: 'Invalid user ID',
        code: 'invalid_id',
        details: { received: request.params['id'] }
      }
    }
  end

  user = @users[user_id]
  unless user
    return {
      status: 404,
      body: {
        error: 'User not found',
        code: 'not_found',
        details: { user_id: user_id }
      }
    }
  end

  user
end

# POST endpoint to create a new user
app.post '/users' do |request|
  body = request.body

  # Validate required fields
  unless body.is_a?(Hash)
    return {
      status: 400,
      body: {
        error: 'Request body must be JSON',
        code: 'validation_error',
        details: { received_type: body.class.name }
      }
    }
  end

  unless body['name'].is_a?(String) && !body['name'].empty?
    return {
      status: 400,
      body: {
        error: 'Missing or invalid required field: name',
        code: 'validation_error',
        details: { field: 'name' }
      }
    }
  end

  unless body['email'].is_a?(String) && body['email'].include?('@')
    return {
      status: 400,
      body: {
        error: 'Missing or invalid required field: email',
        code: 'validation_error',
        details: { field: 'email' }
      }
    }
  end

  # Create new user
  new_id = @users.keys.max + 1
  new_user = {
    id: new_id,
    name: body['name'],
    email: body['email']
  }
  @users[new_id] = new_user

  {
    status: 201,
    body: new_user
  }
end

# DELETE endpoint to remove user
app.delete '/users/:id' do |request|
  user_id = request.params['id'].to_i

  unless @users[user_id]
    return {
      status: 404,
      body: {
        error: 'User not found',
        code: 'not_found'
      }
    }
  end

  deleted_user = @users.delete(user_id)
  {
    status: 200,
    body: {
      message: 'User deleted',
      user: deleted_user
    }
  }
end

puts 'Starting Validation Example on http://127.0.0.1:8000'
puts 'Try:'
puts '  curl http://127.0.0.1:8000/users'
puts "  curl 'http://127.0.0.1:8000/users?name=Alice'"
puts '  curl http://127.0.0.1:8000/users/1'
puts "  curl -X POST http://127.0.0.1:8000/users \\"
puts "    -H 'Content-Type: application/json' \\"
puts "    -d '{\"name\":\"Charlie\",\"email\":\"charlie@example.com\"}'"
puts ''

app.run
