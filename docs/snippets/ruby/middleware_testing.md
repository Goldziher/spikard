```ruby
require 'spikard'
require 'rspec'

RSpec.describe 'auth_guard' do
  it 'allows valid token' do
    request = {
      headers: { authorization: 'Bearer valid-jwt-token' },
      method: 'GET',
      path: '/api/users'
    }

    result = auth_guard.call(request)

    expect(result[:context][:user_id]).to be_present
  end

  it 'rejects missing token' do
    request = {
      headers: {},
      method: 'GET',
      path: '/api/users'
    }

    expect { auth_guard.call(request) }.to raise_error(Spikard::HTTPError)
  end
end
```
