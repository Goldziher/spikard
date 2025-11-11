# frozen_string_literal: true

module Spikard
  # Represents a Server-Sent Event.
  #
  # @!attribute [rw] data
  #   @return [Hash] Event data (will be JSON serialized)
  # @!attribute [rw] event_type
  #   @return [String, nil] Optional event type
  # @!attribute [rw] id
  #   @return [String, nil] Optional event ID for client reconnection support
  # @!attribute [rw] retry_ms
  #   @return [Integer, nil] Optional retry timeout in milliseconds
  class SseEvent
    attr_accessor :data, :event_type, :id, :retry_ms

    # Create a new SSE event.
    #
    # @param data [Hash] Event data (will be JSON serialized)
    # @param event_type [String, nil] Optional event type
    # @param id [String, nil] Optional event ID for client reconnection support
    # @param retry_ms [Integer, nil] Optional retry timeout in milliseconds
    def initialize(data:, event_type: nil, id: nil, retry_ms: nil)
      @data = data
      @event_type = event_type
      @id = id
      @retry_ms = retry_ms
    end

    # Convert to hash for JSON serialization.
    #
    # @return [Hash] Hash representation of the event
    def to_h
      {
        data: @data,
        event_type: @event_type,
        id: @id,
        retry: @retry_ms
      }.compact
    end
  end

  # Base class for SSE event producers.
  #
  # Implement this class to generate Server-Sent Events.
  #
  # @example
  #   class NotificationProducer < Spikard::SseEventProducer
  #     def initialize
  #       @count = 0
  #     end
  #
  #     def next_event
  #       sleep 1  # Wait 1 second between events
  #
  #       return nil if @count >= 10  # End stream after 10 events
  #
  #       event = Spikard::SseEvent.new(
  #         data: { message: "Notification #{@count}" },
  #         event_type: 'notification',
  #         id: @count.to_s
  #       )
  #       @count += 1
  #       event
  #     end
  #
  #     def on_connect
  #       puts "Client connected to SSE stream"
  #     end
  #
  #     def on_disconnect
  #       puts "Client disconnected from SSE stream"
  #     end
  #   end
  #
  #   app = Spikard::App.new
  #
  #   app.sse('/notifications') do
  #     NotificationProducer.new
  #   end
  #
  #   app.run
  class SseEventProducer
    # Generate the next event.
    #
    # This method is called repeatedly to produce the event stream.
    #
    # @return [SseEvent, nil] SseEvent when an event is ready, or nil to end the stream.
    def next_event
      raise NotImplementedError, "#{self.class.name} must implement #next_event"
    end

    # Called when a client connects to the SSE endpoint.
    #
    # Override this method to perform initialization when a client connects.
    #
    # @return [void]
    def on_connect
      # Optional hook - default implementation does nothing
    end

    # Called when a client disconnects from the SSE endpoint.
    #
    # Override this method to perform cleanup when a client disconnects.
    #
    # @return [void]
    def on_disconnect
      # Optional hook - default implementation does nothing
    end
  end
end
