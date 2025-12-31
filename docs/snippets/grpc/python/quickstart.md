```python
# user_handler.py
from spikard import GrpcRequest, GrpcResponse
import user_pb2  # Generated from your .proto file

class UserServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        if request.method_name == "GetUser":
            # Deserialize
            req = user_pb2.GetUserRequest()
            req.ParseFromString(request.payload)

            # Process
            user = user_pb2.User(id=req.user_id, name="Alice", email="alice@example.com")

            # Serialize and return
            return GrpcResponse(payload=user.SerializeToString())
```
