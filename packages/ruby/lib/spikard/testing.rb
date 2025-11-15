# frozen_string_literal: true

module Spikard
  # Testing helpers that wrap the native Ruby extension.
  module Testing
    module_function

    def create_test_client(app, config: nil)
      unless defined?(Spikard::Native::TestClient)
        raise LoadError, 'Spikard native test client is not available. Build the native extension before running tests.'
      end

      # Allow generated apps to stash a test config
      if config.nil? && app.instance_variable_defined?(:@__spikard_test_config)
        config = app.instance_variable_get(:@__spikard_test_config)
      end

      # Use default config if none provided
      config ||= Spikard::ServerConfig.new

      routes_json = JSON.generate(app.route_metadata)
      handlers = app.handler_map.transform_keys(&:to_sym)
      ws_handlers = app.websocket_handlers || {}
      sse_producers = app.sse_producers || {}
      native = Spikard::Native::TestClient.new(routes_json, handlers, config, ws_handlers, sse_producers)
      TestClient.new(native)
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
        native_ws = @native.websocket(path)
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
        @native_ws.send_text(text)
      end

      def send_json(obj)
        @native_ws.send_json(obj)
      end

      def receive_text
        @native_ws.receive_text
      end

      def receive_json
        @native_ws.receive_json
      end

      def receive_bytes
        @native_ws.receive_bytes
      end

      def receive_message
        native_msg = @native_ws.receive_message
        WebSocketMessage.new(native_msg)
      end

      def close
        @native_ws.close
      end
    end

    # WebSocket message wrapper
    class WebSocketMessage
      def initialize(native_msg)
        @native_msg = native_msg
      end

      def as_text
        @native_msg.as_text
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
        @native_sse.events.map { |native_event| SseEvent.new(native_event) }
      end

      def events_as_json
        @native_sse.events_as_json
      end
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
  end
end
