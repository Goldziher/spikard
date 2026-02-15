import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_1mb_messages() -> None:
    """Tests server streaming RPC with large message payloads (approximately 1MB each). Validates that the streaming framework can handle large individual messages without truncation or memory issues.."""

    from app.main import handle_grpc_server_streaming_1mb_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamLargeMessages",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_1mb_messages(request)

    # Verify response
    assert response.payload == b'"3 large messages streamed successfully"'
    assert response.metadata is not None
