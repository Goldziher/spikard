#!/usr/bin/env ruby
# frozen_string_literal: true

# Basic dependency injection example with value dependencies.
#
# This example demonstrates:
# - Registering static value dependencies
# - Auto-injection by parameter name
# - Method chaining with provide()

require 'bundler/setup'
require 'spikard'

app = Spikard::App.new

# Register value dependencies
app.provide('app_name', 'MyApp')
app.provide('version', '1.0.0')
app.provide('max_connections', 100)

# Handler with auto-injected dependencies
#
# The parameters app_name and version are automatically matched
# to registered dependencies by name.
app.get('/config') do |app_name:, version:|
  {
    app: app_name,
    version: version
  }
end

# Handler with integer dependency injection
app.get('/stats') do |max_connections:|
  {
    max_connections: max_connections,
    current_connections: 42
  }
end

# Handler with multiple injected dependencies
app.get('/all') do |app_name:, version:, max_connections:|
  {
    app: app_name,
    version: version,
    max_connections: max_connections
  }
end

puts 'Starting server with dependency injection...'
puts 'Try:'
puts '  curl http://localhost:8000/config'
puts '  curl http://localhost:8000/stats'
puts '  curl http://localhost:8000/all'

app.run(port: 8000)
