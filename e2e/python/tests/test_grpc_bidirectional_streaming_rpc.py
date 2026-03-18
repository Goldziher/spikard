import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_rpc() -> None:
    """Tests bidirectional streaming where both client and server send multiple messages. Covers duplex communication patterns.."""

    from app.main import handle_grpc_bidirectional_streaming_rpc

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
        "authorization": "Bearer user-token",
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
    assert (
        response.payload
        == b'[{"acknowledged":true,"message":"Hello, everyone!","message_id":1,"timestamp":1704067200000,"user_id":10,"username":"alice"},{"acknowledged":true,"message":"Hey Alice, doing great!","message_id":2,"timestamp":1704067205000,"user_id":20,"username":"bob"},{"acknowledged":true,"message":"How is everyone doing?","message_id":3,"timestamp":1704067210000,"user_id":10,"username":"alice"}]'
    )
    assert response.metadata is not None
