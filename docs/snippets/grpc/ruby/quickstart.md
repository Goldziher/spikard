```ruby
# user_handler.rb
require 'spikard'
require_relative 'user_service_pb'

class UserServiceHandler
  def handle_request(request)
    if request.method_name == 'GetUser'
      # Deserialize
      req = Userservice::V1::GetUserRequest.decode(request.payload)

      # Process
      user = Userservice::V1::User.new(
        id: req.user_id,
        name: 'Alice',
        email: 'alice@example.com'
      )

      # Serialize and return
      Spikard::Grpc::Response.new(payload: user.to_proto)
    end
  end
end
```
