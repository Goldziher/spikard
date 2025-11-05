# frozen_string_literal: true

module Spikard
  # Testing helpers that wrap the native Ruby extension.
  module Testing
    module_function

    def create_test_client(app)
      backend =
        if defined?(Spikard::Native::TestClient)
          routes_json = JSON.generate(app.route_metadata)
          handlers = app.handler_map.transform_keys(&:to_sym)
          Spikard::Native::TestClient.new(routes_json, handlers)
        else
          ShimBackend.new(app)
        end

      TestClient.new(backend)
    end

    # High level wrapper around the native test client.
    class TestClient
      def initialize(backend)
        @backend = backend
      end

      def request(method, path, **options)
        payload = @backend.request(method.to_s.upcase, path, options)
        Response.new(payload)
      end

      %w[get post put patch delete head options trace].each do |verb|
        define_method(verb) do |path, **options|
          request(verb.upcase, path, **options)
        end
      end
    end

    # Pure Ruby fallback when the native extension is unavailable.
    class ShimBackend
      def initialize(app)
        @app = app
      end

      def request(method, path, options)
        route_entry, path_params = find_route(method, path)
        raise ArgumentError, "No route matches #{method} #{path}" unless route_entry

        request_payload = build_request_payload(method, path, path_params, options)
        handler_result = route_entry.handler.call(request_payload)

        normalize_response(handler_result)
      end

      private

      def find_route(method, path)
        @app.routes.each do |entry|
          next unless entry.metadata[:method].to_s.casecmp?(method)

          params = extract_path_params(entry.metadata[:path], path)
          return [entry, params] if params
        end
        nil
      end

      def extract_path_params(pattern, actual_path)
        pattern_parts = split_path(pattern)
        actual_parts = split_path(actual_path)
        return unless pattern_parts.length == actual_parts.length

        match_segments(pattern_parts, actual_parts)
      end

      def split_path(path)
        path.split('/').reject(&:empty?)
      end

      def match_segments(pattern_parts, actual_parts)
        params = {}
        pattern_parts.zip(actual_parts).each do |segment, value|
          if segment.start_with?(':')
            params[segment[1..]] = value
          elsif segment != value
            return nil
          end
        end
        params
      end

      def build_request_payload(method, path, path_params, options)
        {
          method: method,
          path: path,
          params: deep_stringify(path_params),
          query: deep_stringify(options.fetch(:query, {})),
          headers: stringify_keys(options.fetch(:headers, {})),
          cookies: stringify_keys(options.fetch(:cookies, {})),
          body: extract_body(options)
        }.compact
      end

      def extract_body(options)
        if options.key?(:json)
          deep_stringify(options[:json])
        elsif options.key?(:data)
          deep_stringify(options[:data])
        else
          options[:body]
        end
      end

      def normalize_response(result)
        return normalize_spikard_response(result) if result.is_a?(Spikard::Response)

        body = result.nil? ? '' : JSON.generate(deep_stringify(result))
        {
          status_code: 200,
          headers: { 'content-type' => 'application/json' },
          body: body,
          body_text: body.empty? ? nil : body
        }
      end

      def normalize_spikard_response(result)
        content = result.content
        body = content.nil? ? '' : JSON.generate(deep_stringify(content))
        headers = stringify_keys(result.headers || {})
        headers['content-type'] ||= 'application/json' unless body.empty?
        {
          status_code: result.status_code,
          headers: headers,
          body: body,
          body_text: body.empty? ? nil : body
        }
      end

      def stringify_keys(hash)
        hash.each_with_object({}) do |(key, value), acc|
          acc[key.to_s] = value.is_a?(Hash) ? stringify_keys(value) : value
        end
      end

      def deep_stringify(value)
        case value
        when Hash
          value.each_with_object({}) do |(key, val), acc|
            acc[key.to_s] = deep_stringify(val)
          end
        when Array
          value.map { |v| deep_stringify(v) }
        else
          value
        end
      end
    end
  end
end
