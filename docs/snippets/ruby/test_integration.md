```ruby
it "completes user workflow" do
  users_db = {}
  app = Spikard::App.new

  app.post('/users') do |params, _query, body|
    id = users_db.size + 1
    user = { id: id, name: body['name'] }
    users_db[id] = user
    user
  end

  app.get('/users/:id') do |params, _query, _body|
    users_db[params['id'].to_i] || { error: 'Not found' }
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
