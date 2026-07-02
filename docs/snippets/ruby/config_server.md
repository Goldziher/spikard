```ruby
require "spikard"

config = Spikard::ServerConfig.new(
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  request_timeout: 60,
  max_body_size: 5 * 1024 * 1024  # 5MB
)

app = Spikard::App.new(config: config)

app.get("/health") do
  { status: "ok" }
end

app.run
```
