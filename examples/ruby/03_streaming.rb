#!/usr/bin/env ruby
# frozen_string_literal: true

# Streaming Responses Example
#
# Demonstrates streaming large responses and Server-Sent Events (SSE)
# for real-time server-to-client communication.

require 'spikard'
require 'json'

# Create application
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# GET endpoint streaming numbers as newline-delimited JSON
app.get '/stream/numbers' do |request|
  count = (request.query['count'] || '100').to_i

  # Return a streaming response with generator
  Spikard::StreamingResponse.new do |stream|
    count.times do |i|
      data = {
        number: i + 1,
        squared: (i + 1) ** 2,
        timestamp: Time.now.iso8601
      }
      stream << "#{JSON.dump(data)}\n"

      # Simulate processing
      sleep(0.01)
    end
  end
end

# GET endpoint for Server-Sent Events
app.get '/stream/events' do |request|
  duration = (request.query['duration'] || '30').to_i

  Spikard::StreamingResponse.new(
    content_type: 'text/event-stream',
    headers: {
      'Cache-Control' => 'no-cache',
      'Connection' => 'keep-alive'
    }
  ) do |stream|
    start_time = Time.now
    event_id = 0

    while Time.now - start_time < duration
      event_id += 1
      elapsed = (Time.now - start_time).to_i

      data = {
        id: event_id,
        elapsed: elapsed,
        message: "Event #{event_id} after #{elapsed}s",
        timestamp: Time.now.iso8601
      }

      stream << "event: tick\n"
      stream << "data: #{JSON.dump(data)}\n"
      stream << "\n"

      sleep(1)
    end

    # Send completion event
    stream << "event: complete\n"
    stream << "data: #{JSON.dump({ message: 'Stream complete', totalEvents: event_id })}\n"
  end
end

# GET endpoint for CSV streaming
app.get '/stream/csv' do |request|
  Spikard::StreamingResponse.new(
    content_type: 'text/csv',
    headers: {
      'Content-Disposition' => 'attachment; filename="users.csv"'
    }
  ) do |stream|
    # Write CSV header
    stream << "id,name,email,created_at\n"

    # Generate rows
    1000.times do |i|
      row = "#{i + 1},user_#{i + 1},user_#{i + 1}@example.com,#{Time.now.iso8601}\n"
      stream << row

      # Simulate processing
      sleep(0.01) if (i + 1) % 100 == 0
    end
  end
end

# Browser-friendly demo page
app.get '/' do |request|
  {
    status: 200,
    body: <<~HTML,
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Spikard Ruby Streaming Example</title>
        <style>
          body { font-family: monospace; margin: 20px; }
          .section { margin: 20px 0; padding: 10px; border: 1px solid #ccc; }
          #events { border: 1px solid #999; padding: 10px; height: 200px; overflow-y: auto; background: #f5f5f5; }
          button { padding: 5px 10px; margin: 5px; cursor: pointer; }
        </style>
      </head>
      <body>
        <h1>Spikard Ruby Streaming Example</h1>

        <div class="section">
          <h2>Server-Sent Events (SSE)</h2>
          <button onclick="startSSE()">Start Event Stream</button>
          <button onclick="stopSSE()">Stop Stream</button>
          <div id="events">Events will appear here...</div>
        </div>

        <div class="section">
          <h2>Other Examples</h2>
          <p><a href="/stream/numbers?count=50" target="_blank">Stream 50 Numbers (NDJSON)</a></p>
          <p><a href="/stream/csv" target="_blank">Stream CSV</a></p>
        </div>

        <script>
          let eventSource = null;

          function startSSE() {
            if (eventSource) return;

            const eventLog = document.getElementById('events');
            eventLog.innerHTML = 'Connecting...\\n';

            eventSource = new EventSource('/stream/events?duration=10');

            eventSource.addEventListener('tick', (e) => {
              const data = JSON.parse(e.data);
              eventLog.innerHTML += 'TICK: ' + data.message + '\\n';
              eventLog.scrollTop = eventLog.scrollHeight;
            });

            eventSource.addEventListener('complete', (e) => {
              const data = JSON.parse(e.data);
              eventLog.innerHTML += 'COMPLETE: ' + data.message + '\\n';
              stopSSE();
            });

            eventSource.addEventListener('error', () => {
              eventLog.innerHTML += 'ERROR: Connection closed\\n';
              stopSSE();
            });
          }

          function stopSSE() {
            if (eventSource) {
              eventSource.close();
              eventSource = null;
            }
          }
        </script>
      </body>
      </html>
    HTML
    headers: { 'Content-Type' => 'text/html; charset=utf-8' }
  }
end

puts 'Starting Ruby Streaming Example on http://127.0.0.1:8000'
puts 'Open http://127.0.0.1:8000 in your browser to test SSE'
puts 'Or try:'
puts '  curl http://127.0.0.1:8000/stream/numbers?count=10'
puts '  curl http://127.0.0.1:8000/stream/events?duration=5'
puts '  curl http://127.0.0.1:8000/stream/csv'
puts ''

app.run
