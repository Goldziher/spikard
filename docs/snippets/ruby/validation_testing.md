```ruby
require "rspec"
require "rack/test"

RSpec.describe "User creation validation" do
  include Rack::Test::Methods

  def app
    @app
  end

  it "accepts valid requests" do
    post "/users", {
      email: "test@example.com",
      age: 25,
      username: "testuser"
    }.to_json, { "CONTENT_TYPE" => "application/json" }

    expect(last_response.status).to eq(200)
  end

  it "rejects invalid email" do
    post "/users", {
      email: "not-an-email",
      age: 25,
      username: "testuser"
    }.to_json, { "CONTENT_TYPE" => "application/json" }

    expect(last_response.status).to eq(422)
    body = JSON.parse(last_response.body)
    expect(body["details"].first["field"]).to include("email")
  end

  it "rejects age below minimum" do
    post "/users", {
      email: "test@example.com",
      age: 16,
      username: "testuser"
    }.to_json, { "CONTENT_TYPE" => "application/json" }

    expect(last_response.status).to eq(422)
  end

  it "rejects missing required fields" do
    post "/users", {
      email: "test@example.com",
      age: 25
    }.to_json, { "CONTENT_TYPE" => "application/json" }

    expect(last_response.status).to eq(422)
  end
end
```
