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

    def normalize_path(path)
      segments = path.split('/').map do |segment|
        if segment.start_with?(':') && segment.length > 1
          "{#{segment[1..]}}"
        else
          segment
        end
      end

      segments.join('/')
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
