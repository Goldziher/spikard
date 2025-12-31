```ruby
it "enforces auth middleware" do
  app = Spikard::App.new

  app.pre_handler do |req|
    token = req.headers['authorization']
    unless token&.start_with?('Bearer ')
      return [{ error: 'Unauthorized' }, 401]
    end
    req
  end

  app.get('/protected') { { data: 'secret' } }

  client = Spikard::Testing::TestClient.new(app)

  # Without auth
  response = client.get('/protected')
  expect(response.status).to eq(401)

  # With auth
  response = client.get('/protected', headers: { 'authorization' => 'Bearer token123' })
  expect(response.status).to eq(200)
end
```
