```ruby
require 'spikard'
require 'spec_helper'

RSpec.describe "Hello endpoint" do
  let(:app) do
    Spikard::App.new.tap do |a|
      a.get('/hello') { { message: 'Hello, World!' } }
    end
  end

  it "returns greeting" do
    client = Spikard::Testing::TestClient.new(app)
    response = client.get('/hello')

    expect(response.status).to eq(200)
    expect(response.json).to eq({ 'message' => 'Hello, World!' })
  end
end
```
