# frozen_string_literal: true

module Spikard
  # Testing helpers that wrap the native Ruby extension.
  module Testing
    module_function

    def create_test_client(app, config: nil)
      unless defined?(Spikard::Native::TestClient)
        raise LoadError, 'Spikard native test client is not available. Build the native extension before running tests.'
      end

      # Use default config if none provided
      config ||= Spikard::ServerConfig.new

      routes_json = JSON.generate(app.route_metadata)
      handlers = app.handler_map.transform_keys(&:to_sym)
      native = Spikard::Native::TestClient.new(routes_json, handlers, config)
      TestClient.new(native)
    end

    # High level wrapper around the native test client.
    class TestClient
      def initialize(native)
        @native = native
      end

      def request(method, path, **options)
        payload = @native.request(method.to_s.upcase, path, options)
        Response.new(payload)
      end

      def close
        @native.close
      end

      %w[get post put patch delete head options trace].each do |verb|
        define_method(verb) do |path, **options|
          request(verb.upcase, path, **options)
        end
      end
    end
  end
end
