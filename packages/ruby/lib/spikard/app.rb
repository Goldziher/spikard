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
      native_hooks.add_on_request(hook)
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
      native_hooks.add_pre_validation(hook)
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
      native_hooks.add_pre_handler(hook)
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
      native_hooks.add_on_response(hook)
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
      native_hooks.add_on_error(hook)
      hook
    end

    private

    def native_hooks
      raise 'Spikard native lifecycle registry unavailable' unless defined?(@native_hooks) && @native_hooks

      @native_hooks
    end
  end

  # Collects route metadata so the Rust engine can execute handlers.
  # rubocop:disable Metrics/ClassLength
  class App
    include LifecycleHooks
    include ProvideSupport

    HTTP_METHODS = %w[GET POST PUT PATCH DELETE OPTIONS HEAD TRACE].freeze
    SUPPORTED_OPTIONS = %i[request_schema response_schema parameter_schema file_params is_async cors
                           body_param_name jsonrpc_method].freeze

    attr_reader :routes

    def initialize
      @routes = []
      @websocket_handlers = {}
      @sse_producers = {}
      @native_hooks = Spikard::Native::LifecycleRegistry.new
      @native_dependencies = Spikard::Native::DependencyRegistry.new
    end

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
    def register_route(method, path, handler_name: nil, **options, &block)
      method = method.to_s
      path = path.to_s
      handler_name = handler_name&.to_s
      validate_route_arguments!(block, handler_name, options)
      metadata = build_route_metadata_for(method, path, handler_name, options, block)

      @routes << RouteEntry.new(metadata, block)
      block
    end
    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    HTTP_METHODS.each do |verb|
      define_method(verb.downcase) do |path, handler_name: nil, **options, &block|
        register_route(verb, path, handler_name: handler_name, **options, &block)
      end
    end

    def route_metadata
      @routes.map(&:metadata)
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

    def normalized_routes_json
      json = JSON.generate(route_metadata)
      if defined?(Spikard::Native) && Spikard::Native.respond_to?(:normalize_route_metadata)
        Spikard::Native.normalize_route_metadata(json)
      else
        json
      end
    end

    def default_handler_name(method, path)
      normalized_path = path.gsub(/[^a-zA-Z0-9]+/, '_').gsub(/__+/, '_')
      # ReDoS mitigation: use bounded quantifier {1,100} instead of + to prevent
      # polynomial time complexity with excessive trailing underscores
      normalized_path = normalized_path.sub(/^_{1,100}/, '').sub(/_{1,100}$/, '')
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
    # rubocop:disable Metrics/MethodLength
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

      routes_json = normalized_routes_json

      # Get handler map
      handlers = handler_map

      # Get lifecycle hooks
      hooks = @native_hooks

      # Get WebSocket handlers and SSE producers
      ws_handlers = websocket_handlers
      sse_prods = sse_producers

      # Get dependencies for DI
      deps = @native_dependencies

      # Call the Rust extension's run_server function
      Spikard::Native.run_server(routes_json, handlers, config, hooks, ws_handlers, sse_prods, deps)

      # Keep Ruby process alive while server runs
      sleep
    rescue LoadError => e
      raise 'Failed to load Spikard extension. ' \
            "Build it with: task build:ruby\n#{e.message}"
    end
    # rubocop:enable Metrics/MethodLength

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

    def validate_route_arguments!(block, handler_name, options)
      if block.nil? && (handler_name.nil? || handler_name.empty?)
        raise ArgumentError, 'block required for route handler'
      end

      unknown_keys = options.keys - SUPPORTED_OPTIONS
      return if unknown_keys.empty?

      raise ArgumentError, "unknown route options: #{unknown_keys.join(', ')}"
    end

    def build_route_metadata_for(method, path, handler_name, options, block)
      if block && native_route_metadata_supported?
        build_native_route_metadata(method, path, handler_name, options, block)
      else
        build_fallback_route_metadata(method, path, handler_name, options, block)
      end
    end

    def native_route_metadata_supported?
      defined?(Spikard::Native) && Spikard::Native.respond_to?(:build_route_metadata)
    end

    def build_native_route_metadata(method, path, handler_name, options, block)
      Spikard::Native.build_route_metadata(
        *native_route_metadata_args(method, path, handler_name, options, block, include_jsonrpc: true)
      )
    rescue ArgumentError => e
      raise unless e.message.include?('wrong number of arguments')

      Spikard::Native.build_route_metadata(
        *native_route_metadata_args(method, path, handler_name, options, block, include_jsonrpc: false)
      )
    end

    def native_route_metadata_args(method, path, handler_name, options, block, include_jsonrpc:)
      args = [
        method,
        path,
        handler_name,
        options[:request_schema],
        options[:response_schema],
        options[:parameter_schema],
        options[:file_params],
        options.fetch(:is_async, false),
        options[:cors],
        options[:body_param_name]&.to_s
      ]
      args << options[:jsonrpc_method] if include_jsonrpc
      args << block
      args
    end

    def build_fallback_route_metadata(method, path, handler_name, options, block)
      handler_name ||= default_handler_name(method, path)
      handler_dependencies = block ? extract_handler_dependencies(block) : []
      build_metadata(method, path, handler_name, options, handler_dependencies)
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
        next unless %i[keyreq key].include?(param_type)

        dep_name = param_name.to_s
        # Collect ALL keyword parameters, not just registered ones
        # This allows the DI system to validate missing dependencies
        dependencies << dep_name
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
