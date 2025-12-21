# frozen_string_literal: true

require 'json'
require 'timeout'

module Spikard
  # Testing helpers that wrap the native Ruby extension.
  module Testing
    module_function

    def create_test_client(app, config: nil)
      trace('create_test_client:start')
      ensure_native_test_client!
      config = resolve_test_config(app, config)
      native = build_native_test_client(app, config)
      trace('create_test_client:done')
      TestClient.new(native)
    end

    def ensure_native_test_client!
      return if defined?(Spikard::Native::TestClient)

      raise LoadError, 'Spikard native test client is not available. Build the native extension before running tests.'
    end

    def resolve_test_config(app, config)
      return config if config

      if app.instance_variable_defined?(:@__spikard_test_config)
        return app.instance_variable_get(:@__spikard_test_config)
      end

      Spikard::ServerConfig.new
    end

    def build_native_test_client(app, config)
      routes_json = app.normalized_routes_json
      handlers = app.handler_map.transform_keys(&:to_sym)
      ws_handlers = app.websocket_handlers || {}
      sse_producers = app.sse_producers || {}
      dependencies = app.dependencies || {}
      Spikard::Native::TestClient.new(routes_json, handlers, config, ws_handlers, sse_producers, dependencies)
    end

    # High level wrapper around the native test client.
    class TestClient
      def initialize(native)
        @native = native
      end

      # Factory method for creating test client from an app
      def self.new(app_or_native, config: nil)
        # If passed a native client directly, use it
        return super(app_or_native) if app_or_native.is_a?(Spikard::Native::TestClient)

        # Otherwise, create test client from app
        Spikard::Testing.create_test_client(app_or_native, config: config)
      end

      def request(method, path, **options)
        payload = @native.request(method.to_s.upcase, path, options)
        Response.new(payload)
      end

      def websocket(path)
        Testing.trace("websocket:start #{path}")
        native_ws = @native.websocket(path)
        Testing.trace("websocket:connected #{path}")
        WebSocketTestConnection.new(native_ws)
      end

      def sse(path)
        native_sse = @native.sse(path)
        SseStream.new(native_sse)
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

    # WebSocket test connection wrapper
    class WebSocketTestConnection
      def initialize(native_ws)
        @native_ws = native_ws
      end

      def send_text(text)
        Testing.trace('websocket:send_text')
        @native_ws.send_text(JSON.generate(text))
      end

      def send_json(obj)
        Testing.trace('websocket:send_json')
        @native_ws.send_json(obj)
      end

      def receive_text
        Testing.trace('websocket:receive_text')
        raw = with_timeout { @native_ws.receive_text }
        JSON.parse(raw)
      rescue JSON::ParserError
        raw
      end

      def receive_json
        Testing.trace('websocket:receive_json')
        with_timeout { @native_ws.receive_json }
      end

      def receive_bytes
        receive_text
      end

      def receive_message
        native_msg = @native_ws.receive_message
        WebSocketMessage.new(native_msg)
      end

      def close
        Testing.trace('websocket:close')
        @native_ws.close
      end

      private

      def with_timeout(&)
        timeout_ms = ENV.fetch('SPIKARD_RB_TEST_TIMEOUT_MS', nil)
        return yield unless timeout_ms

        Timeout.timeout(timeout_ms.to_f / 1000.0, &)
      end
    end

    # WebSocket message wrapper
    class WebSocketMessage
      def initialize(native_msg)
        @native_msg = native_msg
      end

      def as_text
        raw = @native_msg.as_text
        return unless raw

        JSON.parse(raw)
      rescue JSON::ParserError
        raw
      end

      def as_json
        @native_msg.as_json
      end

      def as_binary
        @native_msg.as_binary
      end

      def close?
        @native_msg.is_close
      end
    end

    # SSE stream wrapper
    class SseStream
      def initialize(native_sse)
        @native_sse = native_sse
      end

      def body
        @native_sse.body
      end

      def events
        parsed_chunks.map { |chunk| InlineSseEvent.new(chunk) }
      end

      def events_as_json
        parsed_chunks.filter_map do |chunk|
          JSON.parse(chunk)
        rescue JSON::ParserError
          nil
        end
      end

      private

      # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      def parsed_chunks
        raw = body.to_s.gsub("\r\n", "\n")
        events = []
        current = []

        raw.each_line do |line|
          stripped = line.chomp
          if stripped.start_with?('data:')
            current << stripped[5..].strip
          elsif stripped.empty?
            unless current.empty?
              data = current.join("\n").strip
              events << data unless data.empty?
              current = []
            end
          end
        end

        unless current.empty?
          data = current.join("\n").strip
          events << data unless data.empty?
        end

        events
      end
      # rubocop:enable Metrics/AbcSize, Metrics/MethodLength
    end

    # SSE event wrapper
    class SseEvent
      def initialize(native_event)
        @native_event = native_event
      end

      def data
        @native_event.data
      end

      def as_json
        @native_event.as_json
      end
    end

    # Lightweight wrapper for parsed SSE events backed by strings.
    class InlineSseEvent
      attr_reader :data

      def initialize(data)
        @data = data
      end

      def as_json
        JSON.parse(@data)
      end
    end

    def trace(message)
      return unless ENV['SPIKARD_RB_TEST_TRACE'] == '1'

      warn("[spikard-rb-test] #{message}")
    end
  end
end
