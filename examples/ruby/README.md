# Spikard Ruby Examples

This directory contains runnable examples demonstrating Spikard Ruby bindings features.

## Prerequisites

1. Ruby 3.2+
2. Spikard Ruby gem installed:
   ```bash
   cd packages/ruby
   bundle install
   ```

## Running Examples

Each example is a standalone Ruby script that can be run directly:

```bash
ruby examples/ruby/01_basic_server.rb
```

Then visit `http://127.0.0.1:8000` in your browser or use curl.

## Examples

### 01. Basic Server (`01_basic_server.rb`)

The simplest possible Spikard application with basic routes.

**Features:**
- Basic server configuration
- Simple GET routes
- Text and JSON responses
- POST endpoint with body parsing

**Try:**
```bash
ruby examples/ruby/01_basic_server.rb

# In another terminal:
curl http://127.0.0.1:8000
curl http://127.0.0.1:8000/health
curl -X POST http://127.0.0.1:8000/echo \
  -H 'Content-Type: application/json' \
  -d '{"message":"hello"}'
```

---

### 02. Validation (`02_validation.rb`)

REST API with JSON request body validation, query parameters, and path parameters.

**Features:**
- JSON request/response handling
- Query parameter parsing
- Path parameters extraction
- Input validation with structured error responses
- HTTP status codes (201, 400, 404)
- DELETE endpoint

**Try:**
```bash
ruby examples/ruby/02_validation.rb

# In another terminal:
# List all users
curl http://127.0.0.1:8000/users

# Filter by name
curl 'http://127.0.0.1:8000/users?name=Alice'

# Get specific user
curl http://127.0.0.1:8000/users/1

# Create a user
curl -X POST http://127.0.0.1:8000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Charlie","email":"charlie@example.com"}'

# Delete a user
curl -X DELETE http://127.0.0.1:8000/users/1
```

---

### 03. Streaming (`03_streaming.rb`)

Real-time server-to-client streaming for large datasets and Server-Sent Events (SSE).

**Features:**
- Streaming large datasets efficiently
- Server-Sent Events (SSE) for real-time updates
- Different streaming formats (NDJSON, CSV, SSE)
- Browser-based HTML demo page
- Streaming response helper

**Try:**
```bash
ruby examples/ruby/03_streaming.rb

# Open in browser: http://127.0.0.1:8000
# Or use curl to see raw streams:

# Stream numbers as newline-delimited JSON
curl http://127.0.0.1:8000/stream/numbers?count=10

# Stream events as SSE (runs for 10 seconds)
curl http://127.0.0.1:8000/stream/events?duration=10

# Stream CSV data
curl http://127.0.0.1:8000/stream/csv
```

---

### 04. WebSocket & SSE (`04_websocket_sse.rb`)

Bidirectional WebSocket communication and advanced SSE patterns.

**Features:**
- WebSocket chat example
- WebSocket notifications stream
- Thread-safe message handling
- SSE-based history replay
- SSE metrics stream
- Browser-based interactive demo

**Try:**
```bash
ruby examples/ruby/04_websocket_sse.rb

# Open in browser: http://127.0.0.1:8000
# Test chat, notifications, and metrics streams interactively
```

---

### 05. Lifecycle Hooks (`05_lifecycle_hooks.rb`)

Demonstrate lifecycle hooks for logging, authentication, and response transformation.

**Features:**
- Request/response logging
- Authentication with Bearer tokens
- Request ID tracking
- Custom error responses
- Authorization checks
- Response header manipulation
- Hook short-circuiting

**Try:**
```bash
ruby examples/ruby/05_lifecycle_hooks.rb

# Open in browser: http://127.0.0.1:8000
# Test public, protected, and admin endpoints

# Or use curl with Bearer tokens:

# Public endpoint (no auth)
curl http://127.0.0.1:8000/public

# Protected endpoint (requires token)
curl -H "Authorization: Bearer alice:secret" http://127.0.0.1:8000/protected

# Admin endpoint (alice only)
curl -H "Authorization: Bearer alice:secret" http://127.0.0.1:8000/admin/stats

# Try unauthorized:
curl http://127.0.0.1:8000/protected  # 401
curl -H "Authorization: Bearer bob:secret" http://127.0.0.1:8000/admin/stats  # 403
```

---

## Feature Status

| Feature | Status | Example |
|---------|--------|---------|
| Basic routing | ✅ Complete | 01, 02 |
| JSON responses | ✅ Complete | 01, 02 |
| Query parameters | ✅ Complete | 02, 03 |
| Path parameters | ✅ Complete | 02 |
| Request validation | ✅ Complete | 02 |
| Streaming responses | ✅ Complete | 03 |
| Server-Sent Events | ✅ Complete | 03, 04 |
| WebSockets | ✅ Complete | 04 |
| Lifecycle hooks | ✅ Complete | 05 |
| Request/response logging | ✅ Complete | 05 |
| Authentication | ✅ Complete | 05 |
| Authorization | ✅ Complete | 05 |
| Error handling | ✅ Complete | 02, 05 |

## Common Patterns

### Error Handling

Return structured error responses:

```ruby
return {
  status: 400,
  body: {
    error: 'Invalid input',
    code: 'validation_error',
    details: { field: 'email' }
  }
}
```

### Query Parameters

Access query params from request:

```ruby
app.get '/search' do |request|
  query = request.query['q']
  # ...
end
```

### Path Parameters

Extract path parameters:

```ruby
app.get '/users/:id' do |request|
  user_id = request.params['id'].to_i
  # ...
end
```

### Request Body Validation

Validate JSON request bodies:

```ruby
app.post '/users' do |request|
  body = request.body

  unless body.is_a?(Hash) && body['email']
    return { status: 400, body: { error: 'Invalid body' } }
  end

  # ...
end
```

### Streaming Responses

Use `Spikard::StreamingResponse` with blocks:

```ruby
app.get '/stream' do |request|
  Spikard::StreamingResponse.new do |stream|
    1000.times do |i|
      stream << "Item #{i}\n"
      sleep(0.01)
    end
  end
end
```

### Lifecycle Hooks

Register hooks for cross-cutting concerns:

```ruby
app.pre_handler do |request|
  # Validate authorization
  unless request.headers['authorization']
    return { status: 401, body: { error: 'Unauthorized' } }
  end
  request
end

app.on_response do |request, response|
  # Add headers to all responses
  response[:headers] ||= {}
  response[:headers]['X-Custom'] = 'value'
  response
end
```

## Need Help?

- Documentation: `packages/ruby/README.md`
- Tests: `packages/ruby/spec/`
- Issues: https://github.com/Goldziher/spikard/issues
