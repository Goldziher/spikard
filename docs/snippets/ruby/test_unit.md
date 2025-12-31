```ruby
it "creates user" do
  app = Spikard::App.new
  app.post('/users') do |req|
    { id: 1, name: req.params['name'], email: req.params['email'] }
  end

  client = Spikard::Testing::TestClient.new(app)
  response = client.post('/users', json: { name: 'Alice', email: 'alice@example.com' })

  expect(response.status).to eq(200)
  data = response.json
  expect(data['name']).to eq('Alice')
end
```
