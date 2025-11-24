require_relative 'spec/spec_helper'
require_relative 'app/main'

app = E2ERubyApp.create_app_di_7_multiple_dependencies_with_cleanup_success
client = Spikard::Testing.create_test_client(app)
response = client.get("/api/multi-cleanup-test")
puts "Status: #{response.status_code}"
puts "Body: #{response.body}"
if response.status_code != 200
  begin
    puts "JSON: #{response.json.inspect}"
  rescue => e
    puts "JSON Error: #{e.message}"
  end
end
client.close
