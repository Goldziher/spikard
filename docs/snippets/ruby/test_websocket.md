```ruby
it "echoes websocket messages" do
  app = Spikard::App.new
  app.websocket('/echo') { |message| message }

  client = Spikard::Testing::TestClient.new(app)
  ws = client.websocket_connect('/echo')

  ws.send_json({ hello: 'world' })
  response = ws.receive_json

  expect(response).to eq({ 'hello' => 'world' })
end
```
