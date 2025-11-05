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
      raise ArgumentError, 'block required for route handler' unless block

      unknown_keys = options.keys - SUPPORTED_OPTIONS
      raise ArgumentError, "unknown route options: #{unknown_keys.join(', ')}" if unknown_keys.any?

      metadata = { method: method, path: path, handler_name: handler_name }
      SUPPORTED_OPTIONS.each { |key| metadata[key] = options[key] }

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
  end
end
