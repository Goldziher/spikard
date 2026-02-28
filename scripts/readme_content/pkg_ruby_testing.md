Use the TestClient for integration tests:

```ruby
client = Spikard::TestClient.new(app)

# HTTP requests
response = client.get("/hello", query: { name: "Alice" })
puts response.status_code  # 200
puts response.json         # { "message" => "Hello, World!" }

# POST, WebSocket, SSE all supported
response = client.post("/users", json: { name: "Bob" })
ws = client.websocket("/chat")
sse = client.sse("/events")

client.close
```
