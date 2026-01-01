# frozen_string_literal: true

module Spikard
  # gRPC support for Spikard
  #
  # This module provides Ruby bindings for handling gRPC requests through
  # Spikard's Rust-based gRPC runtime. Handlers receive protobuf messages
  # as binary strings and use the google-protobuf gem for serialization.
  #
  # @example Basic gRPC handler
  #   require 'spikard/grpc'
  #   require 'user_pb'  # Generated protobuf
  #
  #   class UserServiceHandler < Spikard::Grpc::Handler
  #     def handle_request(request)
  #       case request.method_name
  #       when 'GetUser'
  #         # Deserialize request
  #         req = Example::GetUserRequest.decode(request.payload)
  #
  #         # Process request
  #         user = Example::User.new(id: req.id, name: 'John Doe')
  #
  #         # Serialize response
  #         Spikard::Grpc::Response.new(payload: Example::User.encode(user))
  #       else
  #         raise "Unknown method: #{request.method_name}"
  #       end
  #     end
  #   end
  module Grpc
    # gRPC request object
    #
    # Represents an incoming gRPC request with service/method information
    # and a binary protobuf payload.
    #
    # @!attribute [r] service_name
    #   @return [String] Fully qualified service name (e.g., "mypackage.MyService")
    # @!attribute [r] method_name
    #   @return [String] Method name (e.g., "GetUser")
    # @!attribute [r] payload
    #   @return [String] Binary string containing serialized protobuf message
    # @!attribute [r] metadata
    #   @return [Hash<String, String>] gRPC metadata (headers)
    # rubocop:disable Lint/EmptyClass -- Implementation in Rust via FFI
    class Request
      # These methods are implemented in Rust via Magnus FFI.
      # See: crates/spikard-rb/src/grpc/handler.rs for implementation details.
    end
    # rubocop:enable Lint/EmptyClass

    # gRPC response object
    #
    # Used to return gRPC responses from handlers. The payload should be
    # a binary string containing a serialized protobuf message.
    #
    # @example Creating a response
    #   user = Example::User.new(id: 1, name: 'Alice')
    #   response = Spikard::Grpc::Response.new(payload: Example::User.encode(user))
    #
    # @example Adding metadata
    #   response = Spikard::Grpc::Response.new(payload: encoded_message)
    #   response.metadata = { 'x-custom-header' => 'value' }
    class Response
      # @!attribute [w] metadata
      #   @return [Hash<String, String>] gRPC metadata to include in response

      # Create a new gRPC response
      #
      # @param payload [String] Binary string containing serialized protobuf message
      # @raise [ArgumentError] if payload is not a String
      #
      # Note: Implementation in Rust (Magnus FFI)
      # See: crates/spikard-rb/src/grpc/handler.rs

      # Create an error response
      #
      # @param message [String] Error message
      # @param metadata [Hash<String, String>] Optional gRPC metadata
      # @return [Response] A response with error status
      #
      # @example
      #   response = Spikard::Grpc::Response.error('Method not implemented')
      def self.error(message, metadata = {})
        error_metadata = metadata.merge(
          'grpc-status' => 'INTERNAL',
          'grpc-message' => message
        )
        response = new(payload: '')
        response.metadata = error_metadata
        response
      end
    end

    # Base class for gRPC handlers
    #
    # Subclass this to implement gRPC service handlers. Override
    # {#handle_request} to process incoming requests.
    #
    # @example Implementing a handler
    #   class MyServiceHandler < Spikard::Grpc::Handler
    #     def handle_request(request)
    #       case request.method_name
    #       when 'MethodOne'
    #         # Handle MethodOne
    #         req = MyPackage::MethodOneRequest.decode(request.payload)
    #         resp = MyPackage::MethodOneResponse.new(...)
    #         Spikard::Grpc::Response.new(payload: MyPackage::MethodOneResponse.encode(resp))
    #       when 'MethodTwo'
    #         # Handle MethodTwo
    #         # ...
    #       else
    #         raise "Unknown method: #{request.method_name}"
    #       end
    #     end
    #   end
    class Handler
      # Handle a gRPC request
      #
      # This method must be overridden by subclasses to implement the
      # actual request handling logic.
      #
      # @param request [Spikard::Grpc::Request] The incoming gRPC request
      # @return [Spikard::Grpc::Response] The gRPC response
      # @raise [NotImplementedError] if not overridden by subclass
      def handle_request(request)
        raise NotImplementedError, "#{self.class}#handle_request must be implemented"
      end
    end

    # Service registry for gRPC handlers
    #
    # Manages registration and lookup of gRPC service handlers.
    # Handlers are registered by service name and method.
    #
    # @example Registering a handler
    #   service = Spikard::Grpc::Service.new
    #   handler = UserServiceHandler.new
    #   service.register_handler('mypackage.UserService', handler)
    class Service
      def initialize
        @handlers = {}
      end

      # Register a gRPC handler for a service
      #
      # @param service_name [String] Fully qualified service name
      # @param handler [Spikard::Grpc::Handler] Handler instance
      # @raise [ArgumentError] if service_name is invalid or handler doesn't respond to handle_request
      def register_handler(service_name, handler)
        raise ArgumentError, 'Service name cannot be empty' if service_name.nil? || service_name.empty?

        raise ArgumentError, 'Handler must respond to :handle_request' unless handler.respond_to?(:handle_request)

        @handlers[service_name] = handler
      end

      # Get a handler by service name
      #
      # @param service_name [String] Fully qualified service name
      # @return [Spikard::Grpc::Handler, nil] The handler or nil if not found
      def get_handler(service_name)
        @handlers[service_name]
      end

      # Get all registered service names
      #
      # @return [Array<String>] List of registered service names
      def service_names
        @handlers.keys
      end

      # Check if a service is registered
      #
      # @param service_name [String] Fully qualified service name
      # @return [Boolean] true if the service is registered
      def registered?(service_name)
        @handlers.key?(service_name)
      end
    end
  end
end
