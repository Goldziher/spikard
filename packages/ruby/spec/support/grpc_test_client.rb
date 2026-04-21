# frozen_string_literal: true

require 'grpc'
require 'json'

##
# Module for JSON marshaling/unmarshaling in gRPC calls.
#
module GrpcMarshal
  ##
  # Marshal function: converts Hash to JSON bytes.
  #
  # @param obj [Hash] Object to marshal
  #
  # @return [String] JSON-encoded bytes
  #
  def self.marshal(obj)
    JSON.generate(obj).encode('UTF-8')
  end

  ##
  # Unmarshal function: converts JSON bytes to Hash.
  #
  # @param bytes [String] JSON-encoded bytes
  #
  # @return [Hash] Parsed object
  #
  def self.unmarshal(bytes)
    JSON.parse(bytes.b)
  end
end

##
# Module for gRPC test client utilities.
#
module GrpcTestClientUtils
  ##
  # Prepare metadata for gRPC call.
  #
  # @param metadata [Hash<String, String>, nil] Metadata dictionary from fixture
  #
  # @return [Hash<String, String>, nil] Metadata hash or nil
  #
  def self.prepare_metadata(metadata)
    return nil if metadata.nil? || metadata.empty?

    result = {}
    metadata.each do |key, value|
      result[key.to_s] = value.to_s
    end
    result
  end

  ##
  # Build RPC call options including deadline and metadata.
  #
  # @param metadata [Hash<String, String>, nil] Optional metadata headers
  # @param timeout [Float, nil] Optional timeout in seconds
  #
  # @return [Hash] Options hash for gRPC call
  #
  def self.build_rpc_options(metadata, timeout)
    options = { deadline: compute_deadline(timeout) }
    options[:metadata] = prepare_metadata(metadata) if metadata
    options
  end

  ##
  # Compute deadline from timeout in seconds.
  #
  # @param timeout [Float, nil] Timeout in seconds
  #
  # @return [Time, nil] Deadline time or nil for no timeout
  #
  def self.compute_deadline(timeout)
    return nil if timeout.nil?

    Time.now + timeout
  end

  ##
  # Create a request enumerator from a list of requests.
  #
  # @param requests [Array<Hash>] List of request messages
  #
  # @return [Enumerator] Enumerator that yields each request
  #
  def self.create_request_enumerator(requests)
    Enumerator.new do |yielder|
      requests.each { |req| yielder.yield req }
    end
  end

  ##
  # Collect all responses from a stream enumerator.
  #
  # @param enum [Enumerator] Stream enumerator from gRPC call
  #
  # @return [Array<Hash>] List of collected responses
  #
  def self.collect_stream_responses(enum)
    responses = []
    enum.each_with_object(responses) { |response, acc| acc << response }
    responses
  rescue GRPC::BadStatus => e
    e.define_singleton_method(:responses) { responses }
    raise e
  end
end

##
# gRPC Test Client for executing fixtures against running gRPC server.
#
# This module provides a wrapper for executing gRPC streaming fixtures
# in integration tests with support for:
# - All four streaming modes (unary, server, client, bidirectional)
# - Metadata headers (authentication, tracing, etc.)
# - Timeouts per request
# - JSON-encoded messages (compatible with Spikard's gRPC implementation)
#
# Usage:
#   client = GrpcTestClient.new("localhost:50051")
#   responses = client.execute_server_streaming(
#     "example.v1.StreamService",
#     "GetStream",
#     { request_id: "test-001" },
#     metadata: { authorization: "Bearer token" },
#     timeout: 5.0
#   )
#
class GrpcTestClient
  ##
  # Initialize gRPC test client.
  #
  # @param server_address [String] Server address in format "host:port"
  #
  def initialize(server_address = 'localhost:50051')
    @server_address = server_address
    @stub = nil
  end

  ##
  # Connect to gRPC server.
  #
  # Creates an insecure gRPC client stub to the server.
  #
  def connect
    # Create insecure credentials for test environment
    creds = :this_channel_is_insecure

    # Create a ClientStub that will be used for all RPC calls
    @stub = GRPC::ClientStub.new(
      @server_address,
      creds
    )
  end

  ##
  # Close connection to gRPC server.
  #
  def disconnect
    return unless @stub

    # ClientStub doesn't have explicit close, but we clear the reference
    @stub = nil
  end

  ##
  # Execute block with automatic connection lifecycle.
  #
  # @yield Block to execute with connected client
  #
  def with_connection
    connect
    yield self
  ensure
    disconnect
  end

  ##
  # Execute unary RPC from fixture.
  #
  # @param service_name [String] Fully qualified service name (e.g., "example.v1.Service")
  # @param method_name [String] Method name
  # @param request [Hash] Request data as dictionary
  # @param metadata [Hash<String, String>, nil] Optional metadata headers
  # @param timeout [Float, nil] Optional timeout in seconds
  #
  # @return [Hash] Response data as dictionary
  #
  def execute_unary(service_name, method_name, request, metadata: nil, timeout: nil)
    raise 'Stub not initialized. Use with_connection block.' unless @stub

    method_path = "/#{service_name}/#{method_name}"
    options = GrpcTestClientUtils.build_rpc_options(metadata, timeout)

    # Call unary RPC using ClientStub
    @stub.request_response(
      method_path,
      request,
      GrpcMarshal.method(:marshal),
      GrpcMarshal.method(:unmarshal),
      **options
    )
  rescue StandardError => e
    raise_grpc_error(e)
  end

  ##
  # Execute server streaming RPC from fixture.
  #
  # @param service_name [String] Fully qualified service name
  # @param method_name [String] Method name
  # @param request [Hash] Request data as dictionary
  # @param metadata [Hash<String, String>, nil] Optional metadata headers
  # @param timeout [Float, nil] Optional timeout in seconds
  #
  # @return [Array<Hash>] List of response messages
  #
  def execute_server_streaming(service_name, method_name, request, metadata: nil, timeout: nil)
    raise 'Stub not initialized. Use with_connection block.' unless @stub

    method_path = "/#{service_name}/#{method_name}"
    options = GrpcTestClientUtils.build_rpc_options(metadata, timeout)

    enum = @stub.server_streamer(
      method_path,
      request,
      GrpcMarshal.method(:marshal),
      GrpcMarshal.method(:unmarshal),
      **options
    )

    GrpcTestClientUtils.collect_stream_responses(enum)
  rescue StandardError => e
    raise_grpc_error(e)
  end

  ##
  # Execute client streaming RPC from fixture.
  #
  # @param service_name [String] Fully qualified service name
  # @param method_name [String] Method name
  # @param requests [Array<Hash>] List of request messages
  # @param metadata [Hash<String, String>, nil] Optional metadata headers
  # @param timeout [Float, nil] Optional timeout in seconds
  #
  # @return [Hash] Response data as dictionary
  #
  def execute_client_streaming(service_name, method_name, requests, metadata: nil, timeout: nil)
    raise 'Stub not initialized. Use with_connection block.' unless @stub

    method_path = "/#{service_name}/#{method_name}"
    request_enum = GrpcTestClientUtils.create_request_enumerator(requests)
    options = GrpcTestClientUtils.build_rpc_options(metadata, timeout)

    @stub.client_streamer(
      method_path,
      request_enum,
      GrpcMarshal.method(:marshal),
      GrpcMarshal.method(:unmarshal),
      **options
    )
  rescue StandardError => e
    raise_grpc_error(e)
  end

  ##
  # Execute bidirectional streaming RPC from fixture.
  #
  # @param service_name [String] Fully qualified service name
  # @param method_name [String] Method name
  # @param requests [Array<Hash>] List of request messages
  # @param metadata [Hash<String, String>, nil] Optional metadata headers
  # @param timeout [Float, nil] Optional timeout in seconds
  #
  # @return [Array<Hash>] List of response messages
  #
  def execute_bidirectional(service_name, method_name, requests, metadata: nil, timeout: nil)
    raise 'Stub not initialized. Use with_connection block.' unless @stub

    method_path = "/#{service_name}/#{method_name}"
    request_enum = GrpcTestClientUtils.create_request_enumerator(requests)
    options = GrpcTestClientUtils.build_rpc_options(metadata, timeout)

    enum = @stub.bidi_streamer(
      method_path,
      request_enum,
      GrpcMarshal.method(:marshal),
      GrpcMarshal.method(:unmarshal),
      **options
    )

    GrpcTestClientUtils.collect_stream_responses(enum)
  rescue StandardError => e
    raise_grpc_error(e)
  end

  private

  ##
  # Convert standard errors to gRPC error for consistent error handling.
  #
  # @param error [StandardError] Original error
  #
  # @raise [GRPC::BadStatus] Converted gRPC error
  #
  def raise_grpc_error(error)
    # If already a gRPC error, re-raise as-is
    raise error if error.is_a?(GRPC::BadStatus)

    # Wrap other errors as gRPC errors
    raise GRPC::BadStatus.new(GRPC::Core::StatusCodes::INTERNAL, error.message)
  end
end
