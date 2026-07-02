```python
from spikard import create_app
from spikard.grpc import GrpcService

# Create app
app = create_app()

# Create service registry
grpc_service = GrpcService()

# Register handler
handler = UserServiceHandler(user_repository=UserRepository())
grpc_service.register_handler("userservice.UserService", handler)

# App is now ready to serve gRPC requests
```
