#!/usr/bin/env ruby
# frozen_string_literal: true

# Lifecycle Hooks Example
#
# Demonstrates using lifecycle hooks for request logging,
# authentication, error handling, and response transformation.

require 'spikard'
require 'json'

# Create application
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# Simple auth store
AUTHENTICATED_USERS = Set.new(['alice', 'bob']).freeze

# Register lifecycle hooks

# Called at start of request processing
app.on_request do |request|
  puts "[REQUEST] #{request.method} #{request.path} from #{request.ip}"

  # Add request ID
  request_id = request.headers['x-request-id'] || rand(36**7).to_s(36)
  request['_request_id'] = request_id

  request
end

# Called before parameter validation
app.pre_validation do |request|
  # Require Content-Type for POST/PUT
  if ['POST', 'PUT'].include?(request.method) && request.method
    unless request.headers['content-type']
      return {
        status: 400,
        body: {
          error: 'Missing Content-Type header',
          code: 'missing_header'
        }
      }
    end
  end

  request
end

# Called before handler execution - for auth/authorization
app.pre_handler do |request|
  auth_header = request.headers['authorization']

  if auth_header
    scheme, token = auth_header.split(' ', 2)

    if scheme == 'Bearer'
      username = token.split(':').first

      unless AUTHENTICATED_USERS.include?(username)
        return {
          status: 401,
          body: {
            error: 'Unauthorized',
            code: 'invalid_token'
          }
        }
      end

      request['_user'] = { username: username }
    end
  end

  request
end

# Called after successful response
app.on_response do |request, response|
  status = response[:status] || 200
  puts "[RESPONSE] #{status} for #{request.method} #{request.path} (#{request['_request_id']})"

  # Add custom headers
  response[:headers] ||= {}
  response[:headers]['X-Request-ID'] = request['_request_id']
  response[:headers]['X-Response-Time'] = "#{(rand * 100).round(2)}ms"

  response
end

# Called on error
app.on_error do |request, error|
  puts "[ERROR] #{error[:code]} on #{request.method} #{request.path}: #{error[:error]}"

  {
    status: error[:code] == 'unauthorized' ? 401 : 500,
    body: {
      error: error[:error],
      code: error[:code],
      request_id: request['_request_id'] || 'unknown',
      timestamp: Time.now.iso8601
    }
  }
end

# Public endpoint
app.get '/public' do |request|
  {
    message: 'This is a public endpoint',
    timestamp: Time.now.iso8601
  }
end

# Protected endpoint
app.get '/protected' do |request|
  user = request['_user']

  unless user
    return {
      status: 401,
      body: {
        error: 'Unauthorized',
        code: 'no_token'
      }
    }
  end

  {
    message: "Hello, #{user[:username]}!",
    user: user,
    timestamp: Time.now.iso8601
  }
end

# POST endpoint with request transformation
app.post '/echo' do |request|
  body = request.body

  unless body.is_a?(Hash)
    return {
      status: 400,
      body: {
        error: 'Request body must be a JSON object',
        code: 'invalid_body'
      }
    }
  end

  {
    echo: body,
    received_at: Time.now.iso8601,
    user: request['_user']&.dig(:username) || 'anonymous'
  }
end

# Admin endpoint
app.get '/admin/stats' do |request|
  user = request['_user']

  unless user && user[:username] == 'alice'
    return {
      status: 403,
      body: {
        error: 'Admin access required',
        code: 'forbidden'
      }
    }
  end

  {
    admin: true,
    stats: {
      uptime: Process.clock_gettime(Process::CLOCK_MONOTONIC).to_i,
      memory_mb: `ps -o rss= -p #{Process.pid}`.chomp.to_i / 1024
    },
    timestamp: Time.now.iso8601
  }
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
        <title>Spikard Ruby Lifecycle Hooks Example</title>
        <style>
          body { font-family: monospace; margin: 20px; }
          .section { margin: 20px 0; padding: 15px; border: 1px solid #ccc; background: #f9f9f9; }
          input { width: 100%; padding: 8px; margin: 5px 0; border: 1px solid #ddd; }
          button { padding: 8px 15px; margin: 5px 0; background: #007bff; color: white; border: none; cursor: pointer; }
          button:hover { background: #0056b3; }
          pre { background: #f0f0f0; padding: 10px; overflow-x: auto; }
          .success { color: green; }
          .error { color: red; }
        </style>
      </head>
      <body>
        <h1>Spikard Ruby Lifecycle Hooks Example</h1>

        <div class="section">
          <h2>Public Endpoint (No Auth Required)</h2>
          <button onclick="testPublic()">GET /public</button>
          <pre id="public-result"></pre>
        </div>

        <div class="section">
          <h2>Protected Endpoint (Requires Bearer Token)</h2>
          <p>Use format: <code>alice:secret</code> or <code>bob:secret</code></p>
          <input type="text" id="token" placeholder="Enter token" value="alice:secret">
          <button onclick="testProtected()">GET /protected</button>
          <pre id="protected-result"></pre>
        </div>

        <div class="section">
          <h2>Echo Endpoint (POST)</h2>
          <textarea id="body" style="width: 100%; height: 100px;">{"message":"hello"}</textarea>
          <button onclick="testEcho()">POST /echo</button>
          <pre id="echo-result"></pre>
        </div>

        <div class="section">
          <h2>Admin Stats (alice only)</h2>
          <button onclick="testAdmin()">GET /admin/stats</button>
          <pre id="admin-result"></pre>
        </div>

        <script>
          async function testPublic() {
            try {
              const res = await fetch('/public');
              const data = await res.json();
              document.getElementById('public-result').textContent =
                `<span class="success">✓ \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}`;
            } catch (e) {
              document.getElementById('public-result').textContent =
                `<span class="error">✗ Error: \${e.message}</span>`;
            }
          }

          async function testProtected() {
            try {
              const token = document.getElementById('token').value;
              const res = await fetch('/protected', {
                headers: { 'Authorization': `Bearer \${token}` }
              });
              const data = await res.json();
              document.getElementById('protected-result').textContent =
                `<span class="\${res.status === 200 ? 'success' : 'error'}">\${res.status === 200 ? '✓' : '✗'} \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}`;
            } catch (e) {
              document.getElementById('protected-result').textContent =
                `<span class="error">✗ Error: \${e.message}</span>`;
            }
          }

          async function testEcho() {
            try {
              const body = document.getElementById('body').value || '{}';
              const res = await fetch('/echo', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body
              });
              const data = await res.json();
              document.getElementById('echo-result').textContent =
                `<span class="success">✓ \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}`;
            } catch (e) {
              document.getElementById('echo-result').textContent =
                `<span class="error">✗ Error: \${e.message}</span>`;
            }
          }

          async function testAdmin() {
            try {
              const res = await fetch('/admin/stats', {
                headers: { 'Authorization': 'Bearer alice:secret' }
              });
              const data = await res.json();
              document.getElementById('admin-result').textContent =
                `<span class="\${res.status === 200 ? 'success' : 'error'}">\${res.status === 200 ? '✓' : '✗'} \${res.status}</span>\\n\${JSON.stringify(data, null, 2)}`;
            } catch (e) {
              document.getElementById('admin-result').textContent =
                `<span class="error">✗ Error: \${e.message}</span>`;
            }
          }
        </script>
      </body>
      </html>
    HTML
    headers: { 'Content-Type' => 'text/html; charset=utf-8' }
  }
end

puts 'Starting Ruby Lifecycle Hooks Example on http://127.0.0.1:8000'
puts 'Open http://127.0.0.1:8000 in your browser'
puts ''
puts 'Example tokens:'
puts '  alice:secret  (admin)'
puts '  bob:secret    (regular user)'
puts ''

app.run
