#!/usr/bin/env ruby
# frozen_string_literal: true

# WebSocket and Advanced SSE Example
#
# Demonstrates WebSocket bidirectional communication and
# advanced Server-Sent Events patterns.

require 'spikard'
require 'json'
require 'thread'

# Create application
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# Simple message store for chat
@messages = []
@message_id = 0
@message_lock = Mutex.new

# WebSocket endpoint for chat
app.websocket '/ws/chat' do |ws|
  puts "Client connected to chat"

  # Send welcome message
  ws.send_json({
    type: 'connected',
    message: "Connected to chat at #{Time.now.iso8601}",
    messageCount: @messages.length
  })

  # Handle incoming messages
  ws.on_message do |message|
    begin
      data = JSON.parse(message)
      user_name = data['user'] || 'Anonymous'
      text = data['message'] || ''

      if text.empty?
        ws.send_json({
          type: 'error',
          error: 'Empty message'
        })
        next
      end

      @message_lock.synchronize do
        @message_id += 1
        msg = {
          id: @message_id,
          user: user_name,
          text: text,
          timestamp: Time.now.iso8601
        }
        @messages << msg

        # Echo back
        ws.send_json({
          type: 'message',
          data: msg,
          totalMessages: @messages.length
        })
      end
    rescue JSON::ParserError
      ws.send_json({
        type: 'error',
        error: 'Invalid message format'
      })
    end
  end

  # Handle disconnect
  ws.on_close do
    puts "Client disconnected from chat"
  end
end

# WebSocket endpoint for notifications
app.websocket '/ws/notifications' do |ws|
  puts "Client connected to notifications"

  ws.send_json({
    type: 'subscribed',
    channel: 'notifications',
    timestamp: Time.now.iso8601
  })

  # Send mock notifications in a background thread
  Thread.new do
    notification_count = 0
    loop do
      notification_count += 1
      begin
        ws.send_json({
          type: 'notification',
          id: notification_count,
          level: notification_count % 3 == 0 ? 'warning' : 'info',
          message: "Notification ##{notification_count}",
          timestamp: Time.now.iso8601
        })
      rescue StandardError
        break
      end
      sleep(2)
    end
  end

  ws.on_message do |message|
    ws.send_json({
      type: 'echo',
      received: message,
      timestamp: Time.now.iso8601
    })
  end

  ws.on_close do
    puts "Client disconnected from notifications"
  end
end

# POST endpoint to retrieve chat history via SSE
app.post '/sse/chat-history' do |request|
  since_id = (request.query['since'] || '0').to_i

  Spikard::StreamingResponse.new(
    content_type: 'text/event-stream',
    headers: {
      'Cache-Control' => 'no-cache',
      'Connection' => 'keep-alive'
    }
  ) do |stream|
    # Send header
    stream << "event: history_start\n"
    stream << "data: #{JSON.dump({ total: @messages.length, since: since_id, timestamp: Time.now.iso8601 })}\n"
    stream << "\n"

    # Stream existing messages
    @message_lock.synchronize do
      @messages.each do |msg|
        next if msg[:id] <= since_id

        stream << "event: message\n"
        stream << "data: #{JSON.dump(msg)}\n"
        stream << "\n"

        sleep(0.01)
      end
    end

    # Send completion
    stream << "event: history_complete\n"
    stream << "data: #{JSON.dump({ total: @messages.length, timestamp: Time.now.iso8601 })}\n"
  end
end

# GET endpoint for metrics stream
app.get '/sse/metrics' do |request|
  interval = (request.query['interval'] || '1000').to_i / 1000.0

  Spikard::StreamingResponse.new(
    content_type: 'text/event-stream',
    headers: {
      'Cache-Control' => 'no-cache',
      'Connection' => 'keep-alive'
    }
  ) do |stream|
    iteration = 0

    while iteration < 60
      cpu_usage = rand * 100
      memory_usage = rand * 100
      request_count = rand(1000).to_i

      stream << "event: metrics\n"
      stream << "data: #{JSON.dump({
        timestamp: Time.now.iso8601,
        cpu: format('%.2f', cpu_usage),
        memory: format('%.2f', memory_usage),
        requests: request_count,
        iteration: iteration
      })}\n"
      stream << "\n"

      iteration += 1
      sleep(interval)
    end

    stream << "event: complete\n"
    stream << "data: #{JSON.dump({ message: 'Metrics collection complete', iterations: iteration })}\n"
  end
end

# Demo page
app.get '/' do |request|
  {
    status: 200,
    body: <<~HTML,
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Spikard Ruby WebSocket & SSE Example</title>
        <style>
          * { box-sizing: border-box; }
          body { font-family: monospace; margin: 0; padding: 20px; background: #f0f0f0; }
          .container { max-width: 1200px; margin: 0 auto; }
          .section { background: white; margin: 20px 0; padding: 15px; border-radius: 5px; }
          h2 { margin-top: 0; }
          .chat-box { border: 1px solid #ccc; height: 200px; overflow-y: auto; background: #f9f9f9; padding: 10px; margin: 10px 0; }
          .message { margin: 5px 0; padding: 5px; border-left: 3px solid #007bff; background: #f0f8ff; }
          input { width: 100%; padding: 8px; margin: 5px 0; border: 1px solid #ddd; border-radius: 3px; }
          button { padding: 8px 15px; margin: 5px 5px 5px 0; background: #007bff; color: white; border: none; border-radius: 3px; cursor: pointer; }
          button:hover { background: #0056b3; }
        </style>
      </head>
      <body>
        <div class="container">
          <h1>Spikard Ruby WebSocket & SSE Example</h1>

          <div class="section">
            <h2>WebSocket Chat</h2>
            <div class="chat-box" id="chat"></div>
            <input type="text" id="userName" placeholder="Your name" value="User">
            <input type="text" id="messageInput" placeholder="Type message..." onkeypress="if (event.key === 'Enter') sendMessage()">
            <button onclick="connectChat()">Connect</button>
            <button onclick="sendMessage()">Send</button>
          </div>

          <div class="section">
            <h2>Metrics Stream (SSE)</h2>
            <div id="metrics"></div>
            <button onclick="streamMetrics()">Start Metrics</button>
          </div>
        </div>

        <script>
          let chatWs = null;

          function connectChat() {
            if (chatWs) return;
            chatWs = new WebSocket('ws://localhost:8000/ws/chat');
            chatWs.onmessage = (e) => {
              const msg = JSON.parse(e.data);
              const chatDiv = document.getElementById('chat');
              const html = `<div class="message">\${msg.type}: \${JSON.stringify(msg).substring(0, 80)}...</div>`;
              chatDiv.innerHTML += html;
              chatDiv.scrollTop = chatDiv.scrollHeight;
            };
          }

          function sendMessage() {
            if (!chatWs || chatWs.readyState !== WebSocket.OPEN) return;
            const name = document.getElementById('userName').value || 'User';
            const text = document.getElementById('messageInput').value;
            if (!text) return;
            chatWs.send(JSON.stringify({ user: name, message: text }));
            document.getElementById('messageInput').value = '';
          }

          let metricsEventSource = null;

          function streamMetrics() {
            if (metricsEventSource) return;
            metricsEventSource = new EventSource('/sse/metrics?interval=500');
            metricsEventSource.addEventListener('metrics', (e) => {
              const data = JSON.parse(e.data);
              const metricsDiv = document.getElementById('metrics');
              const html = `<div>\${data.cpu}% CPU | \${data.memory}% Memory | \${data.requests} requests</div>`;
              metricsDiv.innerHTML += html;
              if (metricsDiv.children.length > 20) {
                metricsDiv.removeChild(metricsDiv.firstChild);
              }
            });
          }
        </script>
      </body>
      </html>
    HTML
    headers: { 'Content-Type' => 'text/html; charset=utf-8' }
  }
end

puts 'Starting Ruby WebSocket & SSE Example on http://127.0.0.1:8000'
puts 'Open http://127.0.0.1:8000 in your browser'
puts ''

app.run
