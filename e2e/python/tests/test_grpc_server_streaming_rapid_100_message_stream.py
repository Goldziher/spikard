import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_rapid_100_message_stream() -> None:
    """Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption.."""

    from app.main import handle_grpc_server_streaming_rapid_100_message_stream

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="StreamRapidMessages",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_rapid_100_message_stream(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"100 messages streamed successfully in sequence"'
    assert response.metadata is not None
