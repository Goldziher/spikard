import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_rpc() -> None:
    """Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.."""

    from app.main import handle_grpc_bidirectional_streaming_rpc

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "authorization": "Bearer user-token",
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ChatService",
        method_name="Chat",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_rpc(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.metadata is not None
