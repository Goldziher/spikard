# frozen_string_literal: true

require "spikard"

module Spikard
  # Spikard application builder.
  class App
    def initialize
      # Create a new application with the default server configuration.
      @registrations = []
    end
    def config(config)
      # Set the server configuration.
      @config = config
      self
    end
    def route(builder, &block)
      # Register a route using the provided builder and handler function.
      #
      # # Errors
      #
      # Returns an error if route construction fails or if the handler registration fails.
      @registrations.push(["route", [builder], block])
      self
    end
    def register_route(builder, handler)
      # Register a route callback directly without block syntax.
      @registrations.push(["route", [builder], handler])
      self
    end
    def get(path: String, &block)
      # Register a GET route at the given path.
      @registrations.push(["get", [spikard::Method::Get, path], block])
      self
    end
    def post(path: String, &block)
      # Register a POST route at the given path.
      @registrations.push(["post", [spikard::Method::Post, path], block])
      self
    end
    def put(path: String, &block)
      # Register a PUT route at the given path.
      @registrations.push(["put", [spikard::Method::Put, path], block])
      self
    end
    def patch(path: String, &block)
      # Register a PATCH route at the given path.
      @registrations.push(["patch", [spikard::Method::Patch, path], block])
      self
    end
    def delete(path: String, &block)
      # Register a DELETE route at the given path.
      @registrations.push(["delete", [spikard::Method::Delete, path], block])
      self
    end
    def head(path: String, &block)
      # Register a HEAD route at the given path.
      @registrations.push(["head", [spikard::Method::Head, path], block])
      self
    end
    def options(path: String, &block)
      # Register an OPTIONS route at the given path.
      @registrations.push(["options", [spikard::Method::Options, path], block])
      self
    end
    def connect(path: String, &block)
      # Register a CONNECT route at the given path.
      @registrations.push(["connect", [spikard::Method::Connect, path], block])
      self
    end
    def trace(path: String, &block)
      # Register a TRACE route at the given path.
      @registrations.push(["trace", [spikard::Method::Trace, path], block])
      self
    end
    def run
      # Run the HTTP server using the configured routes.
      #
      # # Errors
      #
      # Returns an error if server construction or execution fails.
      Spikard.app_run(@registrations)
    end
    def into_router
      # Build the underlying Axum router.
      #
      # # Errors
      #
      # Returns an error if server or router construction fails.
      Spikard.app_into_router(@registrations)
    end
  end

end
