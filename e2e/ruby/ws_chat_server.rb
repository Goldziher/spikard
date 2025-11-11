#!/usr/bin/env ruby
# frozen_string_literal: true

# WebSocket chat server using Spikard WebSocket support

# Add packages/ruby to load path
$LOAD_PATH.unshift(File.expand_path('../../packages/ruby/lib', __dir__))

require 'spikard'

# WebSocket handler for chat messages
class ChatHandler < Spikard::WebSocketHandler
  def initialize
    super
    @username = nil
  end

  def handle_message(message)
    msg_type = message['type']
    puts "[ChatHandler] Received message type: #{msg_type}"
    puts "[ChatHandler] Message data: #{message.inspect}"

    case msg_type
    when 'chatMessage'
      # Echo back the chat message
      {
        'type' => 'chatMessage',
        'username' => message['username'] || 'anonymous',
        'message' => message['message'] || '',
        'timestamp' => message['timestamp']
      }

    when 'userJoined'
      # Store username and send acknowledgment
      @username = message['username']
      {
        'type' => 'userJoined',
        'username' => @username,
        'timestamp' => message['timestamp']
      }

    when 'userLeft'
      # Send departure message
      username = @username || message['username'] || 'anonymous'
      {
        'type' => 'userLeft',
        'username' => username,
        'timestamp' => message['timestamp']
      }

    else
      # Unknown message type - return nil to not send response
      nil
    end
  end

  def on_connect
    puts '[ChatHandler] Client connected'
  end

  def on_disconnect
    puts "[ChatHandler] Client disconnected (user: #{@username})"
  end
end

# Create Spikard app
app = Spikard::App.new

# Register WebSocket endpoint
app.websocket('/chat') do
  ChatHandler.new
end

# Run server
if __FILE__ == $PROGRAM_NAME
  puts 'Starting WebSocket chat server on ws://localhost:8000/chat'
  config = Spikard::ServerConfig.new(host: '0.0.0.0', port: 8000)
  app.run(config: config)
end
