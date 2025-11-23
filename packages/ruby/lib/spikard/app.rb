# frozen_string_literal: true

module Spikard
  RouteEntry = Struct.new(:metadata, :handler)

  # Lifecycle hooks support for Spikard applications
  module LifecycleHooks
    # Register an onRequest lifecycle hook
    #
    # Runs before routing. Can inspect/modify the request or short-circuit with a response.
    #
    # @param hook [Proc] A proc that receives a request and returns either:
    #   - The (possibly modified) request to continue processing
    #   - A Response object to short-circuit the request pipeline
    # @return [Proc] The hook proc (for chaining)
    #
    # @example
    #   app.on_request do |request|
    #     puts "Request: #{request.method} #{request.path}"
    #     request
    #   end
    def on_request(&hook)
      @lifecycle_hooks[:on_request] << hook
      hook
    end

    # Register a preValidation lifecycle hook
    #
    # Runs after routing but before validation. Useful for rate limiting.
    #
    # @param hook [Proc] A proc that receives a request and returns either:
    #   - The (possibly modified) request to continue processing
    #   - A Response object to short-circuit the request pipeline
    # @return [Proc] The hook proc (for chaining)
    #
    # @example
    #   app.pre_validation do |request|
    #     if too_many_requests?
    #       Spikard::Response.new(content: { error: "Rate limit exceeded" }, status_code: 429)
    #     else
    #       request
    #     end
    #   end
    def pre_validation(&hook)
      @lifecycle_hooks[:pre_validation] << hook
      hook
    end

    # Register a preHandler lifecycle hook
    #
    # Runs after validation but before the handler. Ideal for authentication/authorization.
    #
    # @param hook [Proc] A proc that receives a request and returns either:
    #   - The (possibly modified) request to continue processing
    #   - A Response object to short-circuit the request pipeline
    # @return [Proc] The hook proc (for chaining)
    #
    # @example
    #   app.pre_handler do |request|
    #     if invalid_token?(request.headers['Authorization'])
    #       Spikard::Response.new(content: { error: "Unauthorized" }, status_code: 401)
    #     else
    #       request
    #     end
    #   end
    def pre_handler(&hook)
      @lifecycle_hooks[:pre_handler] << hook
      hook
    end

    # Register an onResponse lifecycle hook
    #
    # Runs after the handler executes. Can modify the response.
    #
    # @param hook [Proc] A proc that receives a response and returns the (possibly modified) response
    # @return [Proc] The hook proc (for chaining)
    #
    # @example
    #   app.on_response do |response|
    #     response.headers['X-Frame-Options'] = 'DENY'
    #     response
    #   end
    def on_response(&hook)
      @lifecycle_hooks[:on_response] << hook
      hook
    end

    # Register an onError lifecycle hook
    #
    # Runs when an error occurs. Can customize error responses.
    #
    # @param hook [Proc] A proc that receives an error response and returns a (possibly modified) response
    # @return [Proc] The hook proc (for chaining)
    #
    # @example
    #   app.on_error do |response|
    #     response.headers['Content-Type'] = 'application/json'
    #     response
    #   end
    def on_error(&hook)
      @lifecycle_hooks[:on_error] << hook
      hook
    end

    # Get all registered lifecycle hooks
    #
    # @return [Hash] Dictionary of hook arrays by type
    def lifecycle_hooks
      {
        on_request: @lifecycle_hooks[:on_request].dup,
        pre_validation: @lifecycle_hooks[:pre_validation].dup,
        pre_handler: @lifecycle_hooks[:pre_handler].dup,
        on_response: @lifecycle_hooks[:on_response].dup,
        on_error: @lifecycle_hooks[:on_error].dup
      }
    end
  end

  # Collects route metadata so the Rust engine can execute handlers.
  # rubocop:disable Metrics/ClassLength
  class App
    include LifecycleHooks
    include ProvideSupport

    HTTP_METHODS = %w[GET POST PUT PATCH DELETE OPTIONS HEAD TRACE].freeze
    SUPPORTED_OPTIONS = %i[request_schema response_schema parameter_schema file_params is_async cors].freeze

    attr_reader :routes

    def initialize
      @routes = []
      @websocket_handlers = {}
      @sse_producers = {}
      @dependencies = {}
      @lifecycle_hooks = {
        on_request: [],
        pre_validation: [],
        pre_handler: [],
        on_response: [],
        on_error: []
      }
    end

    def register_route(method, path, handler_name: nil, **options, &block)
      validate_route_arguments!(block, options)
      handler_name ||= default_handler_name(method, path)

      # Extract handler dependencies from block parameters
      handler_dependencies = extract_handler_dependencies(block)

      metadata = build_metadata(method, path, handler_name, options, handler_dependencies)

      @routes << RouteEntry.new(metadata, block)
      block
    end

    HTTP_METHODS.each do |verb|
      define_method(verb.downcase) do |path, handler_name: nil, **options, &block|
        register_route(verb, path, handler_name: handler_name, **options, &block)
      end
    end

    def route_metadata
      # Extract handler dependencies when metadata is requested
      # This allows dependencies to be registered after routes
      @routes.map do |entry|
        metadata = entry.metadata.dup

        # Re-extract dependencies in case they were registered after the route
        handler_dependencies = extract_handler_dependencies(entry.handler)
        metadata[:handler_dependencies] = handler_dependencies unless handler_dependencies.empty?

        metadata
      end
    end

    def handler_map
      map = {}
      @routes.each do |entry|
        name = entry.metadata[:handler_name]
        # Pass raw handler - DI resolution happens in Rust layer
        map[name] = entry.handler
      end
      map
    end

    def default_handler_name(method, path)
      normalized_path = path.gsub(/[^a-zA-Z0-9]+/, '_').gsub(/__+/, '_').sub(/^_+|_+$/, '')
      normalized_path = 'root' if normalized_path.empty?
      "#{method.to_s.downcase}_#{normalized_path}"
    end

    # Register a WebSocket endpoint
    #
    # @param path [String] URL path for the WebSocket endpoint
    # @yield Factory block that returns a WebSocketHandler instance
    # @return [Proc] The factory block (for chaining)
    #
    # @example
    #   app.websocket('/chat') do
    #     ChatHandler.new
    #   end
    def websocket(path, _handler_name: nil, **_options, &factory)
      raise ArgumentError, 'block required for WebSocket handler factory' unless factory

      @websocket_handlers[path] = factory
      factory
    end

    # Register a Server-Sent Events endpoint
    #
    # @param path [String] URL path for the SSE endpoint
    # @yield Factory block that returns a SseEventProducer instance
    # @return [Proc] The factory block (for chaining)
    #
    # @example
    #   app.sse('/notifications') do
    #     NotificationProducer.new
    #   end
    def sse(path, _handler_name: nil, **_options, &factory)
      raise ArgumentError, 'block required for SSE producer factory' unless factory

      @sse_producers[path] = factory
      factory
    end

    # Get all registered WebSocket handlers
    #
    # @return [Hash] Dictionary mapping paths to handler factory blocks
    def websocket_handlers
      @websocket_handlers.dup
    end

    # Get all registered SSE producers
    #
    # @return [Hash] Dictionary mapping paths to producer factory blocks
    def sse_producers
      @sse_producers.dup
    end

    # Run the Spikard server with the given configuration
    #
    # @param config [ServerConfig, Hash, nil] Server configuration
    #   Can be a ServerConfig object, a Hash with configuration keys, or nil to use defaults.
    #   If a Hash is provided, it will be converted to a ServerConfig.
    #   For backward compatibility, also accepts host: and port: keyword arguments.
    #
    # @example With ServerConfig
    #   config = Spikard::ServerConfig.new(
    #     host: '0.0.0.0',
    #     port: 8080,
    #     compression: Spikard::CompressionConfig.new(quality: 9)
    #   )
    #   app.run(config: config)
    #
    # @example With Hash
    #   app.run(config: { host: '0.0.0.0', port: 8080 })
    #
    # @example Backward compatible (deprecated)
    #   app.run(host: '0.0.0.0', port: 8000)
    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
    def run(config: nil, host: nil, port: nil)
      require 'json'

      # Backward compatibility: if host/port are provided directly, create a config
      if config.nil? && (host || port)
        config = ServerConfig.new(
          host: host || '127.0.0.1',
          port: port || 8000
        )
      elsif config.nil?
        config = ServerConfig.new
      elsif config.is_a?(Hash)
        config = ServerConfig.new(**config)
      end

      # Convert route metadata to JSON
      routes_json = JSON.generate(route_metadata)

      # Get handler map
      handlers = handler_map

      # Get lifecycle hooks
      hooks = lifecycle_hooks

      # Get WebSocket handlers and SSE producers
      ws_handlers = websocket_handlers
      sse_prods = sse_producers

      # Get dependencies for DI
      deps = dependencies

      # Call the Rust extension's run_server function
      Spikard::Native.run_server(routes_json, handlers, config, hooks, ws_handlers, sse_prods, deps)

      # Keep Ruby process alive while server runs
      sleep
    rescue LoadError => e
      raise 'Failed to load Spikard extension. ' \
            "Build it with: task build:ruby\n#{e.message}"
    end
    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    private

    def normalize_path(path)
      # Preserve trailing slash for consistent routing
      has_trailing_slash = path.end_with?('/')

      segments = path.split('/').map do |segment|
        if segment.start_with?(':') && segment.length > 1
          "{#{segment[1..]}}"
        else
          segment
        end
      end

      normalized = segments.join('/')
      # Restore trailing slash if original path had one
      has_trailing_slash && !normalized.end_with?('/') ? "#{normalized}/" : normalized
    end

    def validate_route_arguments!(block, options)
      raise ArgumentError, 'block required for route handler' unless block

      unknown_keys = options.keys - SUPPORTED_OPTIONS
      return if unknown_keys.empty?

      raise ArgumentError, "unknown route options: #{unknown_keys.join(', ')}"
    end

    def extract_handler_dependencies(block)
      # Get the block's parameters
      params = block.parameters

      # Extract keyword parameters (dependencies)
      # Parameters come in the format [:req/:opt/:keyreq/:key, :param_name]
      # :keyreq and :key are keyword parameters (required and optional)
      dependencies = []

      params.each do |param_type, param_name|
        # Skip the request parameter (usually first positional param)
        # Only collect keyword parameters
        if [:keyreq, :key].include?(param_type)
          dep_name = param_name.to_s
          # Only add if it's registered in our dependencies
          dependencies << dep_name if @dependencies.key?(dep_name)
        end
      end

      dependencies
    end

    def build_metadata(method, path, handler_name, options, handler_dependencies)
      base = {
        method: method,
        path: normalize_path(path),
        handler_name: handler_name,
        is_async: options.fetch(:is_async, false)
      }

      # Add handler_dependencies if present
      base[:handler_dependencies] = handler_dependencies unless handler_dependencies.empty?

      SUPPORTED_OPTIONS.each_with_object(base) do |key, metadata|
        next if key == :is_async || !options.key?(key)

        metadata[key] = options[key]
      end
    end
  end
  # rubocop:enable Metrics/ClassLength
end
