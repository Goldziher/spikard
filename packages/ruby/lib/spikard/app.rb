# frozen_string_literal: true

module Spikard
  RouteEntry = Struct.new(:metadata, :handler)

  # Collects route metadata so the Rust engine can execute handlers.
  class App
    HTTP_METHODS = %w[GET POST PUT PATCH DELETE OPTIONS HEAD TRACE].freeze
    SUPPORTED_OPTIONS = %i[request_schema response_schema parameter_schema file_params is_async cors].freeze

    attr_reader :routes

    def initialize
      @routes = []
    end

    def register_route(method, path, handler_name:, **options, &block)
      validate_route_arguments!(block, options)
      metadata = build_metadata(method, path, handler_name, options)

      @routes << RouteEntry.new(metadata, block)
      block
    end

    HTTP_METHODS.each do |verb|
      define_method(verb.downcase) do |path, handler_name:, **options, &block|
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
        map[name] = entry.handler
      end
      map
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

      # Call the Rust extension's run_server function
      Spikard::Native.run_server(routes_json, handlers, config)

      # Keep Ruby process alive while server runs
      sleep
    rescue LoadError => e
      raise 'Failed to load Spikard extension. ' \
            "Build it with: task build:ruby\n#{e.message}"
    end

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

    def build_metadata(method, path, handler_name, options)
      base = {
        method: method,
        path: normalize_path(path),
        handler_name: handler_name,
        is_async: options.fetch(:is_async, false)
      }

      SUPPORTED_OPTIONS.each_with_object(base) do |key, metadata|
        next if key == :is_async || !options.key?(key)

        metadata[key] = options[key]
      end
    end
  end
end
