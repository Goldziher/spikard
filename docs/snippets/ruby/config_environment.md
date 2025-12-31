```ruby
require "spikard"

config = Spikard::ServerConfig.new(
  host: ENV.fetch("SPIKARD_HOST", "127.0.0.1"),
  port: ENV.fetch("SPIKARD_PORT", "8000").to_i,
  workers: ENV.fetch("SPIKARD_WORKERS", "1").to_i,
  request_timeout: ENV.fetch("SPIKARD_TIMEOUT", "30").to_i
)

app = Spikard::App.new(config: config)

# Keep secrets in env
api_key = ENV["API_KEY"]
db_url = ENV["DATABASE_URL"]
```
