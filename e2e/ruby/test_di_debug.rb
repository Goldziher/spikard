$LOAD_PATH.unshift File.expand_path('../packages/ruby/lib', __dir__)
require 'spikard'
require_relative 'app/main'

app = E2ERubyApp.create_app_di_1_async_factory_dependency_success
client = Spikard::Testing.create_test_client(app)
response = client.get("/api/db-status")

puts "Status: #{response.status_code}"
puts "Body: #{response.body}"
puts "Headers: #{response.headers.inspect}"

client.close
