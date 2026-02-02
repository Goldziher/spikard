# Ruby Binding

Ruby binding built on Magnus for fast FFI integration. Handlers receive path parameters, query parameters, and body as separate arguments via the Rust runtime. The binding provides type safety through RBS type stubs and supports all Spikard features: HTTP routing, WebSockets, Server-Sent Events, gRPC, file uploads, middleware configuration, and test clients.

## Quickstart

```ruby
require 'spikard'

app = Spikard::App.new

app.get '/health' do |_params, _query, _body|
  { status: 'ok' }
end

app.run(port: 8000)
```

## Routes & Handlers

Handlers receive three parameters: path parameters (Hash), query parameters (Hash), and body (Hash). Return a Hash or Spikard::Response object for serialization.

```ruby
# Path parameters
app.get '/users/{id}' do |params, _query, _body|
  { user_id: params[:id] }
end

# Query parameters
app.get '/search' do |_params, query, _body|
  { results: search(query[:q]) }
end

# Body (POST/PUT/PATCH)
app.post '/users' do |_params, _query, body|
  { id: 1, name: body[:name], email: body[:email] }
end
```

## Handler Wrappers

The HandlerWrapper module provides convenience methods to reduce boilerplate:

```ruby
# Handler that receives only the body
app.post '/upload', &Spikard.wrap_body_handler do |body|
  { filename: body[:file].filename }
end

# Handler receiving all params separately (explicit)
app.post '/users/{id}', &Spikard.wrap_handler do |params, query, body|
  { user_id: params[:id], search: query[:q], name: body[:name] }
end

# Handler receiving a context hash
app.post '/webhook', &Spikard.wrap_handler_with_context do |ctx|
  { params: ctx[:params], body: ctx[:body] }
end
```

## Route Options

```ruby
app.post '/users', request_schema: UserSchema, is_async: true do |_params, _query, body|
  { id: 1, **body }
end
```

Supported options:
- `request_schema` - Dry::Schema for request validation
- `response_schema` - Dry::Schema for response validation
- `parameter_schema` - Dry::Schema for path parameters
- `file_params` - Array of file field names in the body
- `is_async` - Boolean, whether handler runs async (default: false)
- `cors` - CORS configuration
- `body_param_name` - Custom parameter name for body content

## Response Types

### Basic Response (Hash)

Return a plain Ruby hash for auto-serialization:

```ruby
app.get '/users/{id}' do |params, _query, _body|
  { id: params[:id], name: 'Alice', role: 'admin' }
end
```

### Spikard::Response

For custom status codes, headers, and cookies:

```ruby
app.post '/users' do |_params, _query, body|
  response = Spikard::Response.new(
    content: { id: 1, name: body[:name] },
    status_code: 201
  )
  response.set_header('Location', '/users/1')
  response.set_cookie('session_id', 'abc123', secure: true, httponly: true)
  response
end
```

Response methods:
- `content` - Response body (Hash, Array, or String)
- `status_code` - HTTP status code (default: 200)
- `headers` - Hash of response headers
- `set_header(name, value)` - Add or replace a header
- `set_cookie(name, value, max_age:, domain:, path:, secure:, httponly:, samesite:)` - Set a cookie

### StreamingResponse

For large responses or streaming data:

```ruby
app.get '/events' do |_params, _query, _body|
  stream = Enumerator.new do |yielder|
    10.times do |i|
      yielder << "data: {\"event\": #{i}}\n\n"
      sleep 0.1
    end
  end
  Spikard::StreamingResponse.new(stream, status_code: 200)
end
```

## Validation

Use Dry::Schema for request validation:

```ruby
require 'dry-schema'
Dry::Schema.load_extensions(:json_schema)

UserSchema = Dry::Schema.JSON do
  required(:name).filled(:string)
  required(:email).filled(:string)
  optional(:age).maybe(:integer)
end

app.post '/users', request_schema: UserSchema do |_params, _query, body|
  { id: 1, name: body[:name], email: body[:email] }
end
```

Validation errors return a 400 response with error details automatically.

## File Uploads

Request bodies containing file fields are automatically converted to UploadFile instances:

```ruby
app.post '/upload', file_params: ['document'] do |_params, _query, body|
  file = body[:document]  # UploadFile instance

  {
    filename: file.filename,
    size: file.size,
    content_type: file.content_type,
    content: file.read
  }
end
```

UploadFile API:
- `filename` - Original filename from the client
- `content_type` - MIME type
- `size` - File size in bytes
- `headers` - Additional multipart headers
- `read(size = nil)` - Read file contents
- `text(encoding: 'UTF-8')` - Read as text with specified encoding
- `seek(offset, whence = IO::SEEK_SET)` - Seek to position
- `tell` / `pos` - Current position
- `rewind` - Seek to beginning
- `close` - Close the file
- `closed?` - Check if closed
- `content` - Raw file contents as String

## Configuration

Configure the server with Spikard::ServerConfig:

```ruby
config = Spikard::ServerConfig.new(
  host: '0.0.0.0',
  port: 8080,
  workers: 4,
  max_body_size: 50 * 1024 * 1024,  # 50MB
  request_timeout: 60,
  enable_request_id: true,
  compression: Spikard::CompressionConfig.new(quality: 9),
  rate_limit: Spikard::RateLimitConfig.new(
    per_second: 100,
    burst: 200,
    ip_based: true
  ),
  static_files: [
    Spikard::StaticFilesConfig.new(
      directory: './public',
      route_prefix: '/static',
      cache_control: 'public, max-age=3600'
    )
  ],
  openapi: Spikard::OpenApiConfig.new(
    enabled: true,
    title: 'My API',
    version: '1.0.0'
  )
)

app.run(config: config)
```

### CompressionConfig

- `gzip` - Enable gzip (default: true)
- `brotli` - Enable brotli (default: true)
- `min_size` - Minimum response size to compress in bytes (default: 1024)
- `quality` - Compression level 0-11 (default: 6)

### RateLimitConfig

Uses Generic Cell Rate Algorithm (GCRA):
- `per_second` - Requests per second
- `burst` - Burst allowance for temporary spikes
- `ip_based` - Apply per IP address (default: true)

### JwtConfig

```ruby
jwt = Spikard::JwtConfig.new(
  secret: 'your-secret',
  algorithm: 'HS256',  # HS256, HS384, HS512, RS256, ES256, PS256, etc.
  audience: ['api.example.com'],
  issuer: 'auth.example.com',
  leeway: 30  # Seconds for exp/nbf/iat claims
)

config = Spikard::ServerConfig.new(jwt_auth: jwt)
```

### ApiKeyConfig

```ruby
api_key = Spikard::ApiKeyConfig.new(
  keys: ['key-1', 'key-2', 'key-3'],
  header_name: 'X-API-Key'  # Default: 'X-API-Key'
)

config = Spikard::ServerConfig.new(api_key_auth: api_key)
```

### StaticFilesConfig

```ruby
static = Spikard::StaticFilesConfig.new(
  directory: './public',
  route_prefix: '/static',
  index_file: true,  # Serve index.html for directories
  cache_control: 'public, max-age=3600'
)
```

### OpenApiConfig

```ruby
openapi = Spikard::OpenApiConfig.new(
  enabled: true,
  title: 'My API',
  version: '1.0.0',
  description: 'API documentation',
  contact: Spikard::ContactInfo.new(
    name: 'API Support',
    email: 'support@example.com',
    url: 'https://example.com'
  ),
  license: Spikard::LicenseInfo.new(
    name: 'MIT',
    url: 'https://opensource.org/licenses/MIT'
  ),
  servers: [
    Spikard::ServerInfo.new(url: 'https://api.example.com', description: 'Production'),
    Spikard::ServerInfo.new(url: 'http://localhost:8000', description: 'Development')
  ]
)
```

Serves Swagger UI at /docs, Redoc at /redoc, and OpenAPI JSON at /openapi.json.

## WebSockets

Implement a WebSocketHandler subclass to handle WebSocket connections:

```ruby
class ChatHandler < Spikard::WebSocketHandler
  def handle_message(message)
    # Echo message back to client
    message
  end

  def on_connect
    puts 'Client connected'
  end

  def on_disconnect
    puts 'Client disconnected'
  end
end

app.websocket('/chat') do
  ChatHandler.new
end
```

WebSocketHandler API:
- `handle_message(message)` - Process incoming JSON message, return response or nil
- `on_connect` - Called when client connects (optional override)
- `on_disconnect` - Called when client disconnects (optional override)

## Server-Sent Events

Implement an SseEventProducer subclass to generate events:

```ruby
class NotificationProducer < Spikard::SseEventProducer
  def initialize
    @count = 0
  end

  def next_event
    return nil if @count >= 10  # Stop after 10 events

    event = Spikard::SseEvent.new(
      data: { message: "Notification #{@count}", timestamp: Time.now },
      event_type: 'notification',
      id: @count.to_s,
      retry_ms: 5000
    )
    @count += 1
    sleep 1  # Wait 1 second between events
    event
  end

  def on_connect
    puts 'Client subscribed'
  end

  def on_disconnect
    puts 'Client unsubscribed'
  end
end

app.sse('/notifications') do
  NotificationProducer.new
end
```

SseEventProducer API:
- `next_event` - Generate the next event; return SseEvent or nil to end stream (required)
- `on_connect` - Called when client connects (optional override)
- `on_disconnect` - Called when client disconnects (optional override)

SseEvent API:
- `data` - Hash to JSON serialize
- `event_type` - Optional event type string
- `id` - Optional event ID for client reconnection
- `retry_ms` - Optional retry timeout in milliseconds

## gRPC

Implement a gRPC handler using protocol buffers:

```ruby
require 'spikard/grpc'
require 'user_pb'  # Generated protobuf

class UserServiceHandler < Spikard::Grpc::Handler
  def handle_request(request)
    case request.method_name
    when 'GetUser'
      req = Example::GetUserRequest.decode(request.payload)
      user = Example::User.new(id: req.id, name: 'Alice')
      Spikard::Grpc::Response.new(payload: Example::User.encode(user))
    when 'ListUsers'
      users = Example::UserList.new(users: [
        Example::User.new(id: 1, name: 'Alice'),
        Example::User.new(id: 2, name: 'Bob')
      ])
      Spikard::Grpc::Response.new(payload: Example::UserList.encode(users))
    else
      Spikard::Grpc::Response.error('Method not implemented')
    end
  end
end
```

gRPC::Request API:
- `service_name` - Fully qualified service name (e.g., "mypackage.UserService")
- `method_name` - Method name (e.g., "GetUser")
- `payload` - Binary string containing serialized protobuf message
- `metadata` - Hash of gRPC metadata (headers)

gRPC::Response API:
- `new(payload:)` - Create response with protobuf payload
- `metadata=` - Set response metadata
- `error(message, metadata = {})` - Static method to create error response

## Lifecycle Hooks

Lifecycle hooks run at different stages of request processing:

```ruby
# Before routing
app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request  # Continue or return Spikard::Response to short-circuit
end

# After routing, before validation
app.pre_validation do |request|
  if rate_limited?(request)
    Spikard::Response.new(content: { error: 'Rate limited' }, status_code: 429)
  else
    request
  end
end

# After validation, before handler
app.pre_handler do |request|
  if not authorized?(request)
    Spikard::Response.new(content: { error: 'Unauthorized' }, status_code: 401)
  else
    request
  end
end

# After handler completes
app.on_response do |response|
  response.set_header('X-Custom-Header', 'value')
  response
end

# When an error occurs
app.on_error do |response|
  response.set_header('Content-Type', 'application/json')
  response
end
```

Hook signature:
- Hooks receive a request or response object
- Return the (possibly modified) object to continue
- Return a Spikard::Response to short-circuit processing
- Hooks run synchronously in the request pipeline

## Dependency Injection

Use keyword parameters in handlers for dependency injection:

```ruby
app.provide(:db) do
  # Initialize database
  Database.new
end

app.provide(:logger) do
  Logger.new
end

app.get '/users/{id}', handler: 'get_user'

app.handler :get_user do |params, _query, _body, db:, logger:|
  logger.info("Fetching user #{params[:id]}")
  user = db.find_user(params[:id])
  { id: user.id, name: user.name }
end
```

Dependencies are injected as keyword arguments to handlers.

## Testing

Create a test client to test your application:

```ruby
require 'spikard/testing'

app = Spikard::App.new
app.get '/health' do |_p, _q, _b|
  { status: 'ok' }
end

client = Spikard::TestClient.new(app)

# HTTP requests
response = client.get('/health')
puts response.status_code  # => 200
puts response.json  # => { "status" => "ok" }

# Convenience methods
response = client.post('/users', json: { name: 'Alice', email: 'alice@example.com' })
response = client.put('/users/1', headers: { 'Authorization' => 'Bearer token' }, json: { name: 'Bob' })

# Query parameters and cookies
response = client.get('/search', query: { q: 'test' }, cookies: { session: 'abc123' })

# Raw body
response = client.post('/webhook', raw_body: 'raw data')

# File uploads
response = client.post('/upload', files: { document: { filename: 'doc.pdf', content: pdf_bytes } })

# Close client
client.close
```

TestClient methods:
- `get(path, headers:, body:, json:, data:, raw_body:, files:, query:, cookies:)` - GET request
- `post(path, ...)` - POST request
- `put(path, ...)` - PUT request
- `patch(path, ...)` - PATCH request
- `delete(path, ...)` - DELETE request
- `head(path, ...)` - HEAD request
- `options(path, ...)` - OPTIONS request
- `trace(path, ...)` - TRACE request
- `websocket(path)` - Create WebSocket test connection
- `sse(path)` - Create SSE stream test connection
- `close` - Close test client

WebSocket testing:

```ruby
ws = client.websocket('/chat')
ws.send_json({ type: 'message', text: 'hello' })
msg = ws.receive_json
puts msg  # => { "type" => "message", "text" => "hello" }
ws.close
```

SSE testing:

```ruby
sse = client.sse('/notifications')
events = sse.events_as_json
puts events.first  # => parsed JSON from first event
```

Response methods:
- `status_code` / `status` - HTTP status code
- `headers` - Hash of response headers
- `body_bytes` - Raw response body as bytes
- `body_text` / `text` - Response body as UTF-8 text
- `json` - Parsed JSON response
- `bytes` - Response body as array of byte values

## Deployment

Run the server:

```ruby
app.run(port: 8000)
# or with config
config = Spikard::ServerConfig.new(host: '0.0.0.0', port: 8080)
app.run(config: config)
```

Requirements:
- Ruby 3.2+ (check with `ruby --version`)
- Rust toolchain (for building native extension)
- Build the native extension: `bundle exec rake ext:build`

Deployment checklist:
- Ensure `spikard_rb` native extension is built for the target platform
- Set environment variables for configuration (optional)
- Run `ruby app.rb` or use a process manager (systemd, supervisord, etc.)
