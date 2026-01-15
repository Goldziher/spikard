# frozen_string_literal: true

require 'spec_helper'
require 'json'
require 'rspec'
require_relative 'support/grpc_test_client'

##
# Parametrized tests for gRPC streaming fixtures.
#
# This module runs all fixtures from testing_data/protobuf/streaming/
# as parametrized tests against the running gRPC server.
#
# Architecture:
#   1. Fixtures are validated by scripts/validate_fixtures.py (schema enforcement)
#   2. Fixtures are loaded by fixture discovery functions (discovery & parsing)
#   3. Tests are parametrized by fixture category (server/client/bidirectional/errors)
#   4. GrpcTestClient executes RPCs against running server
#   5. Responses are validated against expected_response in fixtures
#
# Adding new fixtures:
#   - Add JSON file to testing_data/protobuf/streaming/{category}/
#   - Run: task validate:fixtures
#   - Tests automatically discover and run new fixtures
#

FIXTURES_DIR = File.join(__dir__, '..', '..', '..', 'testing_data', 'protobuf', 'streaming').freeze
GRPC_CODE_NAME_MAP = {
  GRPC::Core::StatusCodes::OK => 'OK',
  GRPC::Core::StatusCodes::CANCELLED => 'CANCELLED',
  GRPC::Core::StatusCodes::UNKNOWN => 'UNKNOWN',
  GRPC::Core::StatusCodes::INVALID_ARGUMENT => 'INVALID_ARGUMENT',
  GRPC::Core::StatusCodes::DEADLINE_EXCEEDED => 'DEADLINE_EXCEEDED',
  GRPC::Core::StatusCodes::NOT_FOUND => 'NOT_FOUND',
  GRPC::Core::StatusCodes::ALREADY_EXISTS => 'ALREADY_EXISTS',
  GRPC::Core::StatusCodes::PERMISSION_DENIED => 'PERMISSION_DENIED',
  GRPC::Core::StatusCodes::RESOURCE_EXHAUSTED => 'RESOURCE_EXHAUSTED',
  GRPC::Core::StatusCodes::FAILED_PRECONDITION => 'FAILED_PRECONDITION',
  GRPC::Core::StatusCodes::ABORTED => 'ABORTED',
  GRPC::Core::StatusCodes::OUT_OF_RANGE => 'OUT_OF_RANGE',
  GRPC::Core::StatusCodes::UNIMPLEMENTED => 'UNIMPLEMENTED',
  GRPC::Core::StatusCodes::INTERNAL => 'INTERNAL',
  GRPC::Core::StatusCodes::UNAVAILABLE => 'UNAVAILABLE',
  GRPC::Core::StatusCodes::DATA_LOSS => 'DATA_LOSS',
  GRPC::Core::StatusCodes::UNAUTHENTICATED => 'UNAUTHENTICATED'
}.freeze

##
# Load all fixtures from a category directory.
#
# @param category [String] The fixture category name (e.g., 'server', 'client')
#
# @return [Array<Hash>] Array of fixture data hashes
#
def load_fixtures_by_category(category)
  category_dir = File.join(FIXTURES_DIR, category)
  return [] unless File.directory?(category_dir)

  fixtures = []
  Dir.glob(File.join(category_dir, '*.json')).each do |fixture_file|
    content = File.read(fixture_file, encoding: 'UTF-8')
    fixture = JSON.parse(content)

    # Skip fixtures marked with "skip": true
    next if fixture['skip']

    fixtures << fixture
  end

  fixtures
end

##
# Generate stream messages based on generator description.
#
# @param stream_generator [String] Description of generation logic
# @param stream_size [Integer] Number of messages to generate
#
# @return [Array<Hash>] List of generated messages
#
def generate_stream(stream_generator, stream_size)
  generator_lower = stream_generator.downcase

  if generator_lower.include?('sequential') || generator_lower.include?('counter')
    # Generate sequential integer messages
    (0...stream_size).map { |i| { 'index' => i, 'value' => "message_#{i}" } }
  elsif generator_lower.include?('random')
    # Generate messages with random data
    (0...stream_size).map { |i| { 'index' => i, 'random_value' => rand(0..1000) } }
  elsif generator_lower.include?('timestamp')
    # Generate messages with timestamps
    (0...stream_size).map { |i| { 'index' => i, 'timestamp' => Time.now.to_f } }
  else
    # Default: simple indexed messages
    (0...stream_size).map { |i| { 'index' => i, 'data' => "item_#{i}" } }
  end
end

##
# Extract service name, method name, and method definition from fixture.
#
# Fixtures are schema-validated, so we trust the structure exists.
#
# @param fixture [Hash] Fixture data (schema-validated)
# @param streaming_mode [String, nil] Expected streaming mode (server_streaming, client_streaming)
#
# @return [Array<String, String, Hash>] Tuple of (service_name, method_name, method_definition)
#
def extract_service_method(fixture, streaming_mode = nil)
  protobuf = fixture['protobuf']
  package = protobuf['package']
  service = protobuf['services'][0]
  # Build fully qualified service name: "example.v1.StreamService"
  service_name = "#{package}.#{service['name']}"

  # Find method matching streaming mode
  methods = service['methods']
  method = if streaming_mode
             methods.find { |m| m[streaming_mode] } || methods[0]
           else
             methods[0]
           end

  method_name = method['name']

  [service_name, method_name, method]
end

##
# Extract and prepare request data from fixture.
#
# Handles both single messages and streams, including stream generation.
#
# @param fixture [Hash] Fixture data (schema-validated)
# @param is_streaming [Boolean] Whether this is a streaming request
#
# @return [Hash, Array<Hash>] Single message dict or list of messages for streaming
#
def extract_request_data(fixture, is_streaming: false)
  request = fixture['request']

  unless is_streaming
    # Server streaming or unary: single message
    return request['message']
  end

  # Client or bidirectional streaming: stream of messages
  return request['stream'] if request['stream']

  # Generate stream if using stream_generator
  if request['stream_generator']
    stream_generator = request['stream_generator']
    stream_size = request['stream_size']
    return generate_stream(stream_generator, stream_size)
  end

  # Fallback: empty stream
  []
end

##
# Validate streaming response against expected response.
#
# @param responses [Array<Hash>] Actual response messages received
# @param expected_response [Hash] Expected response from fixture
#
# @raise [RSpec::Expectations::ExpectationNotMetError] If responses don't match
#
def validate_stream_response(responses, expected_response)
  expected_messages = expected_response['stream']

  return unless expected_messages

  expect(responses.length).to eq(expected_messages.length),
                              "Expected #{expected_messages.length} messages, got #{responses.length}"

  responses.each_with_index do |actual, i|
    expected_msg = expected_messages[i]
    expect(actual).to eq(expected_msg), "Message #{i} mismatch: #{actual.inspect} != #{expected_msg.inspect}"
  end
end

##
# Validate single response message against expected response.
#
# @param response [Hash] Actual response message received
# @param expected_response [Hash] Expected response from fixture
#
# @raise [RSpec::Expectations::ExpectationNotMetError] If response doesn't match
#
def validate_single_response(response, expected_response)
  expected_message = expected_response['message']

  return unless expected_message

  # Skip string descriptions (used for documentation)
  return if expected_message.is_a?(String)

  # Validate message content
  expect(response).to eq(expected_message),
                      "Response mismatch: #{response.inspect} != #{expected_message.inspect}"
end

def normalize_code_name(name)
  name.to_s.strip.upcase
end

def grpc_code_name(code)
  return normalize_code_name(code) if code.is_a?(String) || code.is_a?(Symbol)

  GRPC_CODE_NAME_MAP.fetch(code, code.to_s)
end

##
# Validate gRPC error code against expected code.
#
# @param error [GRPC::RpcError] Actual error raised
# @param expected_code [String, Integer] Expected error code
#
# @raise [RSpec::Expectations::ExpectationNotMetError] If error code doesn't match
#
def validate_error_code(error, expected_code)
  if expected_code.is_a?(String)
    actual_code_name = grpc_code_name(error.code)
    expect(normalize_code_name(actual_code_name)).to eq(normalize_code_name(expected_code)),
                                                     "Expected status #{expected_code}, got #{actual_code_name}"
  elsif expected_code.is_a?(Integer)
    actual_code_value = error.code.is_a?(Integer) ? error.code : error.code.to_s.to_i
    expect(actual_code_value).to eq(expected_code),
                                 "Expected status code #{expected_code}, got #{actual_code_value}"
  end
end

##
# Validate gRPC error message against expected message.
#
# @param error [GRPC::RpcError] Actual error raised
# @param expected_message [String] Expected error message
#
# @raise [RSpec::Expectations::ExpectationNotMetError] If error message doesn't match
#
def validate_error_message(error, expected_message)
  error_details = error.details || error.message
  expect(error_details).to include(expected_message),
                           "Expected message '#{expected_message}' not in error details: #{error_details}"
end

##
# Validate gRPC error against expected error.
#
# @param error [GRPC::RpcError] Actual error raised
# @param expected_response [Hash] Expected response from fixture with error field
#
# @raise [RSpec::Expectations::ExpectationNotMetError] If error doesn't match
#
def validate_error_response(error, expected_response)
  expected_error = expected_response['error']
  expected_code = expected_error['code']
  expected_message = expected_error['message']

  validate_error_code(error, expected_code)

  return unless expected_message

  validate_error_message(error, expected_message)
end

def grpc_error_expected?(expected_response)
  return true if expected_response['error']

  false
end

def build_error_expectation(expected_response)
  return expected_response if expected_response['error']

  {
    'error' => {
      'code' => expected_response['status_code'],
      'message' => expected_response['message'].is_a?(String) ? expected_response['message'] : nil
    }
  }
end

# Load fixtures by category
SERVER_STREAMING_FIXTURES = load_fixtures_by_category('server').freeze
CLIENT_STREAMING_FIXTURES = load_fixtures_by_category('client').freeze
BIDIRECTIONAL_FIXTURES = load_fixtures_by_category('bidirectional').freeze
ERROR_FIXTURES = load_fixtures_by_category('errors').freeze

RSpec.describe 'gRPC Streaming Fixtures' do
  let(:server_address) { 'localhost:50051' }

  describe 'Server Streaming' do
    SERVER_STREAMING_FIXTURES.each do |fixture|
      it "passes fixture: #{fixture['name']}" do
        client = GrpcTestClient.new(server_address)
        client.with_connection do
          # Extract service and method
          service_name, method_name, = extract_service_method(fixture, 'server_streaming')

          # Extract request data
          request_message = extract_request_data(fixture, is_streaming: false)

          # Extract metadata and timeout
          request = fixture['request']
          metadata = request['metadata']
          handler = fixture['handler'] || {}
          timeout = handler['timeout_ms'] ? (handler['timeout_ms'] / 1000.0) : nil

          expected_response = fixture['expected_response']
          expects_error = expected_response['error']
          responses = []
          error = nil

          begin
            responses = client.execute_server_streaming(
              service_name,
              method_name,
              request_message,
              metadata: metadata,
              timeout: timeout
            )
            raise 'Expected gRPC error but none was raised' if expects_error
          rescue GRPC::BadStatus => e
            raise e unless expects_error

            error = e
            responses = e.responses if e.respond_to?(:responses)
          end

          if expects_error
            validate_error_response(error, expected_response)
            validate_stream_response(responses, expected_response) if expected_response['stream']
          else
            validate_stream_response(responses, expected_response)
          end
        end
      end
    end
  end

  describe 'Client Streaming' do
    CLIENT_STREAMING_FIXTURES.each do |fixture|
      it "passes fixture: #{fixture['name']}" do
        client = GrpcTestClient.new(server_address)
        client.with_connection do
          # Extract service and method
          service_name, method_name, = extract_service_method(fixture, 'client_streaming')

          # Extract request data (stream of messages)
          request_messages = extract_request_data(fixture, is_streaming: true)

          # Extract metadata and timeout
          request = fixture['request']
          metadata = request['metadata']
          handler = fixture['handler'] || {}
          timeout = handler['timeout_ms'] ? (handler['timeout_ms'] / 1000.0) : nil

          # Execute RPC
          response = client.execute_client_streaming(
            service_name,
            method_name,
            request_messages,
            metadata: metadata,
            timeout: timeout
          )

          # Validate response
          expected_response = fixture['expected_response']
          validate_single_response(response, expected_response)
        end
      end
    end
  end

  describe 'Bidirectional Streaming' do
    BIDIRECTIONAL_FIXTURES.each do |fixture|
      it "passes fixture: #{fixture['name']}" do
        client = GrpcTestClient.new(server_address)
        client.with_connection do
          # Extract service and method
          service_name, method_name, = extract_service_method(fixture)

          # Extract request data (stream of messages)
          request_messages = extract_request_data(fixture, is_streaming: true)

          # Extract metadata and timeout
          request = fixture['request']
          metadata = request['metadata']
          handler = fixture['handler'] || {}
          timeout = handler['timeout_ms'] ? (handler['timeout_ms'] / 1000.0) : nil

          expected_response = fixture['expected_response']
          expects_error = grpc_error_expected?(expected_response)
          responses = []
          error = nil

          begin
            responses = client.execute_bidirectional(
              service_name,
              method_name,
              request_messages,
              metadata: metadata,
              timeout: timeout
            )
            raise 'Expected gRPC error but none was raised' if expects_error
          rescue GRPC::BadStatus => e
            raise e unless expects_error

            error = e
            responses = e.responses if e.respond_to?(:responses)
          end

          if expects_error
            validate_error_response(error, build_error_expectation(expected_response))
            validate_stream_response(responses, expected_response) if expected_response['stream']
          else
            validate_stream_response(responses, expected_response)
          end
        end
      end
    end
  end

  describe 'Error Handling' do
    ERROR_FIXTURES.each do |fixture|
      it "passes fixture: #{fixture['name']}" do
        client = GrpcTestClient.new(server_address)
        client.with_connection do
          # Extract service and method
          service_name, method_name, method = extract_service_method(fixture)

          # Determine streaming mode from method
          is_client_streaming = method['client_streaming'] || false
          is_server_streaming = method['server_streaming'] || false

          # Extract request data
          is_streaming = is_client_streaming || (is_client_streaming && is_server_streaming)
          request_data = extract_request_data(fixture, is_streaming: is_streaming)

          # Extract metadata and timeout
          request = fixture['request']
          metadata = request['metadata']
          handler = fixture['handler'] || {}
          timeout = handler['timeout_ms'] ? (handler['timeout_ms'] / 1000.0) : nil

          # Execute RPC and expect error
          error = nil
          begin
            if is_server_streaming && !is_client_streaming
              # Server streaming
              client.execute_server_streaming(
                service_name,
                method_name,
                request_data,
                metadata: metadata,
                timeout: timeout
              )
            elsif is_client_streaming && !is_server_streaming
              # Client streaming
              client.execute_client_streaming(
                service_name,
                method_name,
                request_data,
                metadata: metadata,
                timeout: timeout
              )
            else
              # Bidirectional or unary
              client.execute_bidirectional(
                service_name,
                method_name,
                request_data,
                metadata: metadata,
                timeout: timeout
              )
            end
          rescue GRPC::BadStatus => e
            error = e
          end

          # Verify error was raised
          expect(error).not_to be_nil, 'Expected gRPC error to be raised'

          # Validate error
          expected_response = fixture['expected_response']
          validate_error_response(error, expected_response)
        end
      end
    end
  end
end
