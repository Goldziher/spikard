# Python gRPC Handler Tests

Comprehensive test examples for gRPC handlers using pytest.

```python
# test_user_handler.py
import pytest
from user_handler import UserServiceHandler
from spikard import GrpcRequest
import user_service_pb2 as pb


@pytest.mark.asyncio
async def test_get_user_success():
    """Test getting an existing user."""
    handler = UserServiceHandler()

    # Create request
    req = pb.GetUserRequest(user_id=1)
    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="GetUser",
        payload=req.SerializeToString(),
    )

    # Call handler
    response = await handler.handle_request(grpc_request)

    # Deserialize response
    user_response = pb.UserResponse()
    user_response.ParseFromString(response.payload)

    # Assertions
    assert user_response.success is True
    assert user_response.user.id == 1
    assert user_response.user.name == "Alice"
    assert user_response.user.email == "alice@example.com"


@pytest.mark.asyncio
async def test_get_user_not_found():
    """Test getting a non-existent user."""
    handler = UserServiceHandler()

    # Create request for non-existent user
    req = pb.GetUserRequest(user_id=999)
    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="GetUser",
        payload=req.SerializeToString(),
    )

    # Call handler
    response = await handler.handle_request(grpc_request)

    # Deserialize response
    user_response = pb.UserResponse()
    user_response.ParseFromString(response.payload)

    # Assertions
    assert user_response.success is False
    assert "not found" in user_response.error_message


@pytest.mark.asyncio
async def test_create_user_success():
    """Test creating a new user."""
    handler = UserServiceHandler()

    # Create request
    req = pb.CreateUserRequest(
        name="Charlie",
        email="charlie@example.com",
        phone="555-1234",
        tags=["developer", "remote"],
    )
    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="CreateUser",
        payload=req.SerializeToString(),
    )

    # Call handler
    response = await handler.handle_request(grpc_request)

    # Deserialize response
    user_response = pb.UserResponse()
    user_response.ParseFromString(response.payload)

    # Assertions
    assert user_response.success is True
    assert user_response.user.id == 3  # Next available ID
    assert user_response.user.name == "Charlie"
    assert user_response.user.email == "charlie@example.com"
    assert user_response.user.phone == "555-1234"
    assert list(user_response.user.tags) == ["developer", "remote"]


@pytest.mark.asyncio
async def test_create_user_validation_error():
    """Test creating a user with missing required fields."""
    handler = UserServiceHandler()

    # Create request with missing email
    req = pb.CreateUserRequest(name="Invalid")
    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="CreateUser",
        payload=req.SerializeToString(),
    )

    # Call handler
    response = await handler.handle_request(grpc_request)

    # Deserialize response
    user_response = pb.UserResponse()
    user_response.ParseFromString(response.payload)

    # Assertions
    assert user_response.success is False
    assert "required" in user_response.error_message


@pytest.mark.asyncio
async def test_unknown_method():
    """Test calling an unknown method."""
    handler = UserServiceHandler()

    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="DeleteUser",  # Not implemented
        payload=b"",
    )

    # Should raise NotImplementedError
    with pytest.raises(NotImplementedError, match="Unknown method"):
        await handler.handle_request(grpc_request)


# Run tests
if __name__ == "__main__":
    pytest.main([__file__, "-v"])
```

## Test Patterns

### Using Fixtures

```python
import pytest
from user_handler import UserServiceHandler

@pytest.fixture
def handler():
    """Create handler with test data."""
    return UserServiceHandler()

@pytest.fixture
def sample_user():
    """Create sample user for tests."""
    return pb.User(
        id=1,
        name="Test User",
        email="test@example.com",
    )

@pytest.mark.asyncio
async def test_with_fixtures(handler, sample_user):
    # Test using fixtures
    pass
```

### Testing Error Cases

```python
@pytest.mark.asyncio
async def test_handles_malformed_payload():
    """Test handler with malformed protobuf."""
    handler = UserServiceHandler()

    # Create request with invalid protobuf
    grpc_request = GrpcRequest(
        service_name="userservice.v1.UserService",
        method_name="GetUser",
        payload=b"invalid protobuf data",
    )

    # Should handle deserialization error gracefully
    with pytest.raises(Exception):  # Or specific protobuf exception
        await handler.handle_request(grpc_request)
```

## Running Tests

```bash
pytest test_user_handler.py -v
```
