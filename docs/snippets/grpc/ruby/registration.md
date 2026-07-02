```ruby
require 'spikard'

# Create service registry
service = Spikard::Grpc::Service.new

# Register handler
handler = UserServiceHandler.new(UserRepository.new)
service.register_handler('userservice.UserService', handler)

# Service ready to handle requests
```
