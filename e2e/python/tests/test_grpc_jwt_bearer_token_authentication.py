import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_jwt_bearer_token_authentication() -> None:
    """Tests JWT authentication via gRPC metadata. Validates that JWT tokens are properly extracted and validated from authorization header.."""

    from app.main import handle_grpc_jwt_bearer_token_authentication

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyLTEyMyIsImlhdCI6MTUxNjIzOTAyMn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AuthService",
        method_name="SecureAction",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_jwt_bearer_token_authentication(request)

    # Verify response
    assert response.payload == b'{"user_id":"user-123","action":"read"}'
    assert response.metadata is not None
