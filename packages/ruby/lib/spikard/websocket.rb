# frozen_string_literal: true

module Spikard
  # Base class for WebSocket message handlers.
  #
  # Implement this class to handle WebSocket connections and messages.
  #
  # @example
  #   class ChatHandler < Spikard::WebSocketHandler
  #     def handle_message(message)
  #       # Echo message back
  #       message
  #     end
  #
  #     def on_connect
  #       puts "Client connected"
  #     end
  #
  #     def on_disconnect
  #       puts "Client disconnected"
  #     end
  #   end
  #
  #   app = Spikard::App.new
  #
  #   app.websocket('/chat') do
  #     ChatHandler.new
  #   end
  #
  #   app.run
  class WebSocketHandler
    # Handle an incoming WebSocket message.
    #
    # @param message [Hash] Parsed JSON message from the client
    # @return [Hash, nil] Optional response message to send back to the client.
    #   Return nil to not send a response.
    def handle_message(message)
      raise NotImplementedError, "#{self.class.name} must implement #handle_message"
    end

    # Called when a client connects.
    #
    # Override this method to perform initialization when a client connects.
    #
    # @return [void]
    def on_connect
      # Optional hook - default implementation does nothing
    end

    # Called when a client disconnects.
    #
    # Override this method to perform cleanup when a client disconnects.
    #
    # @return [void]
    def on_disconnect
      # Optional hook - default implementation does nothing
    end
  end
end
