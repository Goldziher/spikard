```ruby
require 'spikard/grpc'
require 'userservice_pb'  # Generated from proto

class UserServiceHandler < Spikard::Grpc::Handler
  def initialize(user_repository)
    @user_repository = user_repository
  end

  def handle_request(request)
    # Route based on method name
    case request.method_name
    when 'GetUser'
      get_user(request)
    when 'CreateUser'
      create_user(request)
    else
      raise "Unknown method: #{request.method_name}"
    end
  end

  private

  def get_user(request)
    # 1. Deserialize request
    req = Userservice::GetUserRequest.decode(request.payload)

    # 2. Validate input
    raise ArgumentError, 'User ID must be positive' if req.id <= 0

    # 3. Business logic
    user = @user_repository.find_by_id(req.id)
    raise ArgumentError, "User #{req.id} not found" unless user

    # 4. Build response
    response_user = Userservice::User.new(
      id: user.id,
      name: user.name,
      email: user.email,
      created_at: user.created_at.iso8601
    )

    # 5. Serialize and return
    response = Spikard::Grpc::Response.new(
      payload: Userservice::User.encode(response_user)
    )
    response.metadata = { 'x-user-found' => 'true' }
    response
  end

  def create_user(request)
    # 1. Deserialize request
    req = Userservice::CreateUserRequest.decode(request.payload)

    # 2. Validate input
    if req.name.empty? || req.email.empty?
      raise ArgumentError, 'Name and email are required'
    end

    # 3. Check authorization from metadata
    auth_token = request.get_metadata('authorization')
    unless auth_token
      raise SecurityError, 'Authentication required'
    end

    # 4. Business logic
    user = @user_repository.create(
      name: req.name,
      email: req.email
    )

    # 5. Build response
    response_user = Userservice::User.new(
      id: user.id,
      name: user.name,
      email: user.email,
      created_at: Time.now.utc.iso8601
    )

    # 6. Serialize with metadata
    response = Spikard::Grpc::Response.new(
      payload: Userservice::User.encode(response_user)
    )
    response.metadata = {
      'x-user-id' => user.id.to_s,
      'x-created' => 'true'
    }
    response
  end
end
```
