# Ruby gRPC Handler Tests

Comprehensive test examples for gRPC handlers using RSpec.

```ruby
# spec/user_service_handler_spec.rb
require 'spec_helper'
require 'spikard/grpc'
require 'userservice_pb'
require_relative '../lib/user_service_handler'

RSpec.describe UserServiceHandler do
  let(:user_repository) { instance_double('UserRepository') }
  let(:handler) { described_class.new(user_repository) }

  describe '#handle_request' do
    context 'GetUser' do
      it 'returns an existing user successfully' do
        # Setup mock data
        mock_user = OpenStruct.new(
          id: 1,
          name: 'Alice',
          email: 'alice@example.com',
          created_at: Time.now.utc
        )
        allow(user_repository).to receive(:find_by_id).with(1).and_return(mock_user)

        # Create request
        req = Userservice::GetUserRequest.new(id: 1)
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'GetUser',
          payload: Userservice::GetUserRequest.encode(req)
        )

        # Call handler
        response = handler.handle_request(grpc_request)

        # Deserialize response
        user_response = Userservice::User.decode(response.payload)

        # Assertions
        expect(user_response.id).to eq(1)
        expect(user_response.name).to eq('Alice')
        expect(user_response.email).to eq('alice@example.com')
        expect(response.metadata['x-user-found']).to eq('true')
      end

      it 'returns error for non-existent user' do
        allow(user_repository).to receive(:find_by_id).with(999).and_return(nil)

        # Create request for non-existent user
        req = Userservice::GetUserRequest.new(id: 999)
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'GetUser',
          payload: Userservice::GetUserRequest.encode(req)
        )

        # Call handler - should raise error
        expect { handler.handle_request(grpc_request) }.to raise_error(
          ArgumentError, /not found/
        )
      end

      it 'validates user ID is positive' do
        req = Userservice::GetUserRequest.new(id: 0)
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'GetUser',
          payload: Userservice::GetUserRequest.encode(req)
        )

        expect { handler.handle_request(grpc_request) }.to raise_error(
          ArgumentError, /must be positive/
        )
      end
    end

    context 'CreateUser' do
      it 'creates a new user successfully' do
        mock_user = OpenStruct.new(
          id: 3,
          name: 'Charlie',
          email: 'charlie@example.com'
        )
        allow(user_repository).to receive(:create).and_return(mock_user)

        # Create request with authorization metadata
        req = Userservice::CreateUserRequest.new(
          name: 'Charlie',
          email: 'charlie@example.com'
        )
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'CreateUser',
          payload: Userservice::CreateUserRequest.encode(req),
          metadata: { 'authorization' => 'Bearer valid-token' }
        )

        # Call handler
        response = handler.handle_request(grpc_request)

        # Deserialize response
        user_response = Userservice::User.decode(response.payload)

        # Assertions
        expect(user_response.id).to eq(3)
        expect(user_response.name).to eq('Charlie')
        expect(user_response.email).to eq('charlie@example.com')
        expect(response.metadata['x-user-id']).to eq('3')
        expect(response.metadata['x-created']).to eq('true')
      end

      it 'returns error when name is missing' do
        req = Userservice::CreateUserRequest.new(
          name: '',
          email: 'test@example.com'
        )
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'CreateUser',
          payload: Userservice::CreateUserRequest.encode(req),
          metadata: { 'authorization' => 'Bearer token' }
        )

        expect { handler.handle_request(grpc_request) }.to raise_error(
          ArgumentError, /required/
        )
      end

      it 'returns error when authorization is missing' do
        req = Userservice::CreateUserRequest.new(
          name: 'Test',
          email: 'test@example.com'
        )
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'CreateUser',
          payload: Userservice::CreateUserRequest.encode(req)
        )

        expect { handler.handle_request(grpc_request) }.to raise_error(
          SecurityError, /Authentication required/
        )
      end
    end

    context 'unknown method' do
      it 'raises error for unknown method' do
        grpc_request = Spikard::Grpc::Request.new(
          service_name: 'userservice.v1.UserService',
          method_name: 'DeleteUser',
          payload: ''
        )

        expect { handler.handle_request(grpc_request) }.to raise_error(
          RuntimeError, /Unknown method/
        )
      end
    end
  end
end
```

## Test Patterns

### Using Shared Examples

```ruby
RSpec.shared_examples 'authenticated endpoint' do |method_name|
  it 'requires authentication' do
    grpc_request = Spikard::Grpc::Request.new(
      service_name: 'userservice.v1.UserService',
      method_name: method_name,
      payload: request_payload
    )

    expect { handler.handle_request(grpc_request) }.to raise_error(
      SecurityError, /Authentication required/
    )
  end
end

RSpec.describe UserServiceHandler do
  describe 'CreateUser' do
    let(:request_payload) do
      req = Userservice::CreateUserRequest.new(name: 'Test', email: 'test@example.com')
      Userservice::CreateUserRequest.encode(req)
    end

    include_examples 'authenticated endpoint', 'CreateUser'
  end
end
```

### Testing Error Responses

```ruby
RSpec.describe UserServiceHandler do
  describe 'error handling' do
    it 'handles malformed protobuf gracefully' do
      grpc_request = Spikard::Grpc::Request.new(
        service_name: 'userservice.v1.UserService',
        method_name: 'GetUser',
        payload: 'invalid protobuf data'
      )

      expect { handler.handle_request(grpc_request) }.to raise_error(
        Google::Protobuf::ParseError
      )
    end
  end
end
```

### Using let! for Setup

```ruby
RSpec.describe UserServiceHandler do
  let!(:alice) do
    OpenStruct.new(id: 1, name: 'Alice', email: 'alice@example.com', created_at: Time.now)
  end
  let!(:bob) do
    OpenStruct.new(id: 2, name: 'Bob', email: 'bob@example.com', created_at: Time.now)
  end

  before do
    allow(user_repository).to receive(:find_by_id).with(1).and_return(alice)
    allow(user_repository).to receive(:find_by_id).with(2).and_return(bob)
  end

  it 'retrieves different users' do
    # Test with alice
    req = Userservice::GetUserRequest.new(id: 1)
    grpc_request = Spikard::Grpc::Request.new(
      service_name: 'userservice.v1.UserService',
      method_name: 'GetUser',
      payload: Userservice::GetUserRequest.encode(req)
    )

    response = handler.handle_request(grpc_request)
    user = Userservice::User.decode(response.payload)

    expect(user.name).to eq('Alice')
  end
end
```

## Running Tests

```bash
# Run all tests
bundle exec rspec

# Run with verbose output
bundle exec rspec --format documentation

# Run specific file
bundle exec rspec spec/user_service_handler_spec.rb

# Run specific example
bundle exec rspec spec/user_service_handler_spec.rb:15
```
