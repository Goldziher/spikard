```ruby
it "creates user" do
  app = Spikard::App.new
  app.post('/users') do |params, _query, body|
    { id: 1, name: body['name'], email: body['email'] }
  end

  client = Spikard::Testing::TestClient.new(app)
  response = client.post('/users', json: { name: 'Alice', email: 'alice@example.com' })

  expect(response.status).to eq(200)
  data = response.json
  expect(data['name']).to eq('Alice')
end
```
