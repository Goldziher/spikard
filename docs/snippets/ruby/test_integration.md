```ruby
it "completes user workflow" do
  users_db = {}
  app = Spikard::App.new

  app.post('/users') do |req|
    id = users_db.size + 1
    user = { id: id, name: req.params['name'] }
    users_db[id] = user
    user
  end

  app.get('/users/:id') do |req|
    users_db[req.params['id'].to_i] || { error: 'Not found' }
  end

  client = Spikard::Testing::TestClient.new(app)

  # Create user
  create_res = client.post('/users', json: { name: 'Alice' })
  user = create_res.json
  expect(user['name']).to eq('Alice')

  # Retrieve user
  get_res = client.get("/users/#{user['id']}")
  expect(get_res.json).to eq(user)
end
```
