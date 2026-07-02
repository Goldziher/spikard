```python
from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse
import userservice_pb2  # Generated from proto
from datetime import datetime


class UserServiceHandler(GrpcHandler):
    """UserService gRPC handler implementation."""

    def __init__(self, user_repository):
        """Initialize handler with dependencies."""
        self.user_repository = user_repository

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """
        Handle incoming gRPC requests.

        Routes to appropriate method based on request.method_name.
        """
        if request.method_name == "GetUser":
            return await self._get_user(request)
        elif request.method_name == "CreateUser":
            return await self._create_user(request)
        else:
            raise NotImplementedError(f"Unknown method: {request.method_name}")

    async def _get_user(self, request: GrpcRequest) -> GrpcResponse:
        """Handle GetUser RPC."""
        # 1. Deserialize request
        req = userservice_pb2.GetUserRequest()
        req.ParseFromString(request.payload)

        # 2. Validate input
        if req.id <= 0:
            raise ValueError("User ID must be positive")

        # 3. Business logic
        user = await self.user_repository.find_by_id(req.id)
        if not user:
            raise ValueError(f"User {req.id} not found")

        # 4. Build response
        response_user = userservice_pb2.User()
        response_user.id = user.id
        response_user.name = user.name
        response_user.email = user.email
        response_user.created_at = user.created_at.isoformat()

        # 5. Serialize and return
        return GrpcResponse(
            payload=response_user.SerializeToString(),
            metadata={"x-user-found": "true"}
        )

    async def _create_user(self, request: GrpcRequest) -> GrpcResponse:
        """Handle CreateUser RPC."""
        # 1. Deserialize request
        req = userservice_pb2.CreateUserRequest()
        req.ParseFromString(request.payload)

        # 2. Validate input
        if not req.name or not req.email:
            raise ValueError("Name and email are required")

        # 3. Check authorization from metadata
        auth_token = request.get_metadata("authorization")
        if not auth_token:
            raise PermissionError("Authentication required")

        # 4. Business logic
        user = await self.user_repository.create(
            name=req.name,
            email=req.email
        )

        # 5. Build response
        response_user = userservice_pb2.User()
        response_user.id = user.id
        response_user.name = user.name
        response_user.email = user.email
        response_user.created_at = datetime.utcnow().isoformat()

        # 6. Serialize with metadata
        return GrpcResponse(
            payload=response_user.SerializeToString(),
            metadata={
                "x-user-id": str(user.id),
                "x-created": "true"
            }
        )
```
