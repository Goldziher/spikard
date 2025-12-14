# frozen_string_literal: true

require 'json'
require_relative 'spikard/version'
require_relative 'spikard/response'
require_relative 'spikard/streaming_response'
require_relative 'spikard/test_client'

# Load the native extension
begin
  # Try Ruby version-specific extension first (e.g., spikard/3.3/spikard_rb)
  ruby_version = /(\d+\.\d+)/.match(RUBY_VERSION)
  require_relative "spikard/#{ruby_version}/spikard_rb"
rescue LoadError
  # Fall back to generic extension
  require_relative 'spikard/spikard_rb'
end

# Spikard HTTP framework - Ruby bindings
#
# Spikard provides high-performance HTTP server capabilities through native Rust bindings.
#
# @example Basic server
#   app = Spikard.create_app([
#     {
#       method: 'GET',
#       path: '/',
#       handler_name: 'root',
#       is_async: false
#     }
#   ], {
#     'root' => ->(request) { { message: 'Hello, world!' } }
#   })
#
#   Spikard.run_server(app, '127.0.0.1', 8000)
module Spikard
  class Error < StandardError; end

  # Simple struct to hold app configuration
  App = Struct.new(:routes, :handlers) do
    def to_routes_json
      routes.to_json
    end
  end

  class << self
    # Create a new Spikard application
    #
    # @param routes [Array<Hash>] Array of route definitions
    # @param handlers [Hash] Hash of handler_name => Proc
    # @return [App] Application object
    def create_app(routes, handlers)
      App.new(routes, handlers)
    end

    # Create a test client for the application
    #
    # @param app [App] Application object from create_app
    # @return [TestClient] TestClient instance
    def create_test_client(app)
      TestClient.new(app)
    end

    # Run the HTTP server
    #
    # @param app [App] Application object from create_app
    # @param host [String] Host to bind to (default: "127.0.0.1")
    # @param port [Integer] Port to bind to (default: 8000)
    def run_server(app, host = '127.0.0.1', port = 8000)
      Native.run_server(app.to_routes_json, app.handlers, host, port)
    end
  end
end
