#!/usr/bin/env ruby
# Quick test to verify the extension loads

$LOAD_PATH.unshift File.expand_path('lib', __dir__)

require 'spikard'

puts "Spikard VERSION: #{Spikard::VERSION}"
puts "Spikard module loaded successfully!"

# Test creating a simple app
routes = [
  {
    method: 'GET',
    path: '/',
    handler_name: 'root',
    is_async: false
  }
]

handlers = {
  'root' => ->(request) { { message: 'Hello, world!' } }
}

puts "\nCreating app..."
app = Spikard.create_app(routes, handlers)
puts "App created: #{app.class}"

puts "\nCreating test client..."
client = Spikard.create_test_client(app)
puts "Test client created: #{client.class}"

puts "\nMaking test request..."
response = client.get('/')
puts "Response status: #{response.status_code}"
puts "Response body: #{response.json}"

puts "\n✅ All tests passed!"
