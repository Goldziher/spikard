```ruby
it "streams SSE events" do
  app = Spikard::App.new
  app.sse('/events') do
    3.times.map { |i| { event: 'message', data: { count: i } } }
  end

  client = Spikard::Testing::TestClient.new(app)
  response = client.get('/events')

  # SSE responses return status 200
  expect(response.status).to eq(200)
end
```
