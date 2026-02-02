```ruby
require 'spikard'
require 'jwt'

app = Spikard::App.new

app.on_request do |request|
  # Extract token from Authorization header
  auth_header = request.dig(:headers, :authorization) || ""
  unless auth_header.start_with?("Bearer ")
    raise Spikard::HTTPError.new(401, "Missing or invalid authorization header")
  end

  token = auth_header[7..-1]  # Strip "Bearer "

  begin
    # Verify and decode JWT
    payload = JWT.decode(token, "your-secret-key", true, { algorithm: 'HS256' })[0]

    # Enrich context with authenticated user
    request[:context] ||= {}
    request[:context][:user_id] = payload["sub"]
    request[:context][:roles] = payload["roles"] || []

    request
  rescue JWT::DecodeError
    raise Spikard::HTTPError.new(401, "Invalid token")
  end
end
```
