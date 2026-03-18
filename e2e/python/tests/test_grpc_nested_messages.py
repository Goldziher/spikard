import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_nested_messages() -> None:
    """Tests nested message types with complex field hierarchies. Covers nested message definitions and serialization.."""

    from app.main import handle_grpc_nested_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UserService",
        method_name="CreateUser",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_nested_messages(request)

    # Verify response
    assert (
        response.payload
        == b'{"address":{"city":"Springfield","street":"123 Main St","zip_code":"12345"},"email":"bob@example.com","name":"Bob Smith","user_id":456}'
    )
    assert response.metadata is not None
