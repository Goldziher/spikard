#!/usr/bin/env ruby
# frozen_string_literal: true

# Ergonomic-layer smoke server (Ruby / Magnus + Dry::Struct).
#
# Exercises the ergonomic typed-handler + DTO API end-to-end: a typed handler
# whose body is a Dry::Struct, hydrated by the ergonomic layer, with request
# validation delegated to the Rust core (invalid bodies -> 422 ProblemDetails).

require "dry-struct"
require "dry-types"
require "spikard"

# Define a Types module with Dry::Types primitives
module Types
  include Dry.Types()
end

# Define the CreateUser DTO
class CreateUser < Dry::Struct
  attribute :name, Types::String
  attribute :age, Types::Integer
end

# Create the ergonomic app
app = Spikard::App.new

# Register a POST /users handler with explicit body: CreateUser
# The handler receives the hydrated CreateUser instance as its first positional param
app.post("/users", body: CreateUser) do |user|
  # Verify we got a real CreateUser instance
  raise "expected CreateUser, got #{user.class}" unless user.is_a?(CreateUser)
  # Return the DTO; it gets serialized to the response body
  user
end

# Start the server (binds 127.0.0.1:8000)
app.run
