```ruby
it "rejects invalid input" do
  app = Spikard::App.new
  app.post('/users') do |req|
    name = req.params['name']
    age = req.params['age']

    raise ArgumentError, 'Invalid age' unless age.is_a?(Integer)

    { name: name, age: age }
  end

  client = Spikard::Testing::TestClient.new(app)
  response = client.post('/users', json: { name: 'Bob', age: 'invalid' })

  expect(response.status).to eq(400)
end
```
