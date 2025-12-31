```ruby
require 'zlib'

app = Spikard::App.new

# Simple in-memory rate limiter (use Redis in production)
rate_limits = Hash.new { |h, k| h[k] = [] }

app.on_request do |request|
  # 1. Rate limiting: 100 requests per minute per IP
  client_ip = request[:client_ip] || 'unknown'
  now = Time.now.to_f

  # Clean old entries
  rate_limits[client_ip].reject! { |ts| now - ts >= 60 }

  if rate_limits[client_ip].length >= 100
    raise Spikard::HTTPError.new(429, 'Rate limit exceeded')
  end

  rate_limits[client_ip] << now

  # 2. Normalize headers (lowercase keys)
  if request[:headers]
    request[:headers] = request[:headers].transform_keys(&:downcase)
  end

  # 3. Inject tenant from subdomain
  host = request.dig(:headers, :host) || ''
  tenant = host.include?('.') ? host.split('.')[0] : 'default'
  request[:context] ||= {}
  request[:context][:tenant] = tenant

  # 4. Feature flags from query params
  feature_str = request.dig(:query, :features) || ''
  request[:context][:features] = Set.new(feature_str.split(',').reject(&:empty?))

  request
end

app.on_response do |response|
  # Response compression for large payloads
  body = response[:body] || ''
  if body.bytesize > 1024  # Compress if > 1KB
    response[:body] = Zlib::Deflate.deflate(body)
    response[:headers] ||= {}
    response[:headers][:'content-encoding'] = 'gzip'
  end

  response
end
```
