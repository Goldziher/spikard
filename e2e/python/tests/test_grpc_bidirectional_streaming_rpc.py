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
        == b'[{"message_id":1,"user_id":10,"username":"alice","message":"Hello, everyone!","timestamp":1704067200000,"acknowledged":true},{"message_id":2,"user_id":20,"username":"bob","message":"Hey Alice, doing great!","timestamp":1704067205000,"acknowledged":true},{"message_id":3,"user_id":10,"username":"alice","message":"How is everyone doing?","timestamp":1704067210000,"acknowledged":true}]'
    )
    assert response.metadata is not None
