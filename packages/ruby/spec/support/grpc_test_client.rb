# frozen_string_literal: true

require "grpc"
require "json"

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
  def initialize(server_address = "localhost:50051")
    @server_address = server_address
    @channel = nil
  end

  ##
  # Connect to gRPC server.
  #
  # Creates an insecure gRPC channel to the server.
  #
  def connect
    @channel = GRPC::Core::Channel.new(
      @server_address,
      nil,
      :this_channel_is_insecure
    )
  end

  ##
  # Close connection to gRPC server.
  #
  def disconnect
    return unless @channel

    @channel.close
    @channel = nil
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
  # Prepare metadata for gRPC call.
  #
  # Converts metadata Hash to gRPC metadata format.
  #
  # @param metadata [Hash<String, String>, nil] Metadata dictionary from fixture
  #
  # @return [Array<Array<String, String>>, nil] List of [key, value] pairs or nil
  #
  def prepare_metadata(metadata)
    return nil if metadata.nil? || metadata.empty?

    # gRPC metadata is an array of [key, value] pairs
    metadata.map { |key, value| [key.to_s, value.to_s] }
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
    raise "Channel not initialized. Use with_connection block." unless @channel

    method_path = "/#{service_name}/#{method_name}"

    # Create unary RPC stub
    stub = create_unary_stub(method_path)

    # Serialize request to JSON bytes
    request_bytes = JSON.generate(request).encode("UTF-8")

    # Call RPC
    response_bytes = stub.call(
      request_bytes,
      metadata: prepare_metadata(metadata),
      timeout: timeout
    )

    # Deserialize response
    JSON.parse(response_bytes.b)
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
    raise "Channel not initialized. Use with_connection block." unless @channel

    method_path = "/#{service_name}/#{method_name}"

    # Create server streaming RPC stub
    stub = create_server_streaming_stub(method_path)

    # Serialize request to JSON bytes
    request_bytes = JSON.generate(request).encode("UTF-8")

    # Call RPC and collect responses
    responses = []
    enum = stub.call(
      request_bytes,
      metadata: prepare_metadata(metadata),
      timeout: timeout
    )

    enum.each do |response_bytes|
      responses << JSON.parse(response_bytes.b)
    end

    responses
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
    raise "Channel not initialized. Use with_connection block." unless @channel

    method_path = "/#{service_name}/#{method_name}"

    # Create client streaming RPC stub
    stub = create_client_streaming_stub(method_path)

    # Create request enumerator
    request_enum = requests.map { |req| JSON.generate(req).encode("UTF-8") }

    # Call RPC
    response_bytes = stub.call(
      request_enum,
      metadata: prepare_metadata(metadata),
      timeout: timeout
    )

    # Deserialize response
    JSON.parse(response_bytes.b)
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
    raise "Channel not initialized. Use with_connection block." unless @channel

    method_path = "/#{service_name}/#{method_name}"

    # Create bidirectional streaming RPC stub
    stub = create_bidirectional_stub(method_path)

    # Create request enumerator
    request_enum = requests.map { |req| JSON.generate(req).encode("UTF-8") }

    # Call RPC and collect responses
    responses = []
    enum = stub.call(
      request_enum,
      metadata: prepare_metadata(metadata),
      timeout: timeout
    )

    enum.each do |response_bytes|
      responses << JSON.parse(response_bytes.b)
    end

    responses
  rescue StandardError => e
    raise_grpc_error(e)
  end

  private

  ##
  # Create unary RPC stub.
  #
  # @param method_path [String] Full method path (e.g., "/service/Method")
  #
  # @return [GRPC::GenericService::Stub] Unary stub
  #
  def create_unary_stub(method_path)
    @channel.create_call(
      method_path,
      nil,
      nil,
      nil,
      {}
    )
  end

  ##
  # Create server streaming RPC stub.
  #
  # @param method_path [String] Full method path (e.g., "/service/Method")
  #
  # @return [GRPC::GenericService::Stub] Server streaming stub
  #
  def create_server_streaming_stub(method_path)
    @channel.create_call(
      method_path,
      nil,
      nil,
      nil,
      {}
    )
  end

  ##
  # Create client streaming RPC stub.
  #
  # @param method_path [String] Full method path (e.g., "/service/Method")
  #
  # @return [GRPC::GenericService::Stub] Client streaming stub
  #
  def create_client_streaming_stub(method_path)
    @channel.create_call(
      method_path,
      nil,
      nil,
      nil,
      {}
    )
  end

  ##
  # Create bidirectional streaming RPC stub.
  #
  # @param method_path [String] Full method path (e.g., "/service/Method")
  #
  # @return [GRPC::GenericService::Stub] Bidirectional stub
  #
  def create_bidirectional_stub(method_path)
    @channel.create_call(
      method_path,
      nil,
      nil,
      nil,
      {}
    )
  end

  ##
  # Convert standard errors to gRPC RpcError for consistent error handling.
  #
  # @param error [StandardError] Original error
  #
  # @raise [GRPC::RpcError] Converted gRPC error
  #
  def raise_grpc_error(error)
    # If already a gRPC error, re-raise as-is
    raise error if error.is_a?(GRPC::RpcError)

    # Wrap other errors as gRPC errors
    raise GRPC::BadStatus.new(GRPC::Core::StatusCodes::INTERNAL, error.message)
  end
end
