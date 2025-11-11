#!/usr/bin/env ruby
# frozen_string_literal: true

# SSE notifications server using Spikard SSE support

# Add packages/ruby to load path
$LOAD_PATH.unshift(File.expand_path('../../packages/ruby/lib', __dir__))

require 'spikard'

# SSE event producer for notifications
class NotificationProducer < Spikard::SseEventProducer
  def initialize
    super
    @count = 0
    @max_events = 10
  end

  def next_event
    if @count >= @max_events
      puts '[NotificationProducer] Reached max events, ending stream'
      return nil
    end

    # Wait 1 second between events
    sleep 1

    # Create notification event
    event = Spikard::SseEvent.new(
      data: {
        notification: "Notification #{@count}",
        timestamp: @count,
        priority: 'normal'
      },
      event_type: 'notification',
      id: @count.to_s
    )

    puts "[NotificationProducer] Sending event #{@count}"
    @count += 1
    event
  end

  def on_connect
    puts '[NotificationProducer] Client connected to SSE stream'
  end

  def on_disconnect
    puts "[NotificationProducer] Client disconnected (sent #{@count} events)"
  end
end

# Create Spikard app
app = Spikard::App.new

# Register SSE endpoint
app.sse('/notifications') do
  NotificationProducer.new
end

# Run server
if __FILE__ == $PROGRAM_NAME
  puts 'Starting SSE notifications server on http://localhost:8000/notifications'
  config = Spikard::ServerConfig.new(host: '0.0.0.0', port: 8000)
  app.run(config: config)
end
