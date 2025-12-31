```ruby
require 'securerandom'
require 'logger'

app = Spikard::App.new
logger = Logger.new(STDOUT)

app.on_request do |request|
  # Generate or propagate request ID
  request_id = request.dig(:headers, :'x-request-id') || SecureRandom.uuid

  # Inject into context for handlers to use
  request[:context] ||= {}
  request[:context][:request_id] = request_id

  # Log request with structured data
  logger.info({
    event: 'request_started',
    request_id: request_id,
    method: request[:method],
    path: request[:path],
    user_agent: request.dig(:headers, :'user-agent')
  }.to_json)

  request
end

app.on_response do |response|
  request_id = response.dig(:context, :request_id)

  logger.info({
    event: 'request_completed',
    request_id: request_id,
    status: response[:status],
    duration_ms: response[:duration_ms]
  }.to_json)

  # Propagate request ID in response headers
  response[:headers] ||= {}
  response[:headers][:'X-Request-ID'] = request_id

  response
end
```
