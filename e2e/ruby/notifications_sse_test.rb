#!/usr/bin/env ruby
# frozen_string_literal: true

# Test application generated from AsyncAPI specification

require 'net/http'
require 'json'
require 'pathname'

# Load test fixtures
FIXTURES_DIR = Pathname.new(__FILE__).parent.parent + 'testing_data' + 'sse'

def load_fixture(name)
  fixture_path = FIXTURES_DIR + "#{name}.json"
  raise "Fixture not found: #{fixture_path}" unless fixture_path.exist?
  JSON.parse(fixture_path.read)
end

def handle_sse(url)
  puts "Connecting to #{url}..."

  uri = URI(url)
  Net::HTTP.start(uri.host, uri.port) do |http|
    request = Net::HTTP::Get.new(uri)

    http.request(request) do |response|
      puts '✓ Connected'

      response.read_body do |chunk|
        chunk.each_line do |line|
          next unless line.start_with?('data:')

          data = line[5..-1].strip
          begin
            message = JSON.parse(data)
            puts "Received event: #{message}"
          rescue JSON::ParserError
            puts "Received: #{data}"
          end
        end
      end
    end
  end
end

def main
  # Default SSE URI - override with environment variable SSE_URI
  url = ENV['SSE_URI'] || 'http://localhost:8000/notifications'
  handle_sse(url)
end

# Run main function
main
