import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_chat_conversation() -> None:
    """Tests bidirectional streaming RPC simulating a chat-like service with alternating messages.."""

    from app.main import handle_grpc_bidirectional_streaming_chat_conversation

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
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
    response = await handle_grpc_bidirectional_streaming_chat_conversation(request)

    # Verify response
    assert (
        response.payload
        == b'[{"user":"alice","message":"Hello Bob","timestamp":1000},{"user":"alice","message":"How are you?","timestamp":1001},{"user":"alice","message":"I\'m doing great!","timestamp":1002}]'
    )
    assert response.metadata is not None
