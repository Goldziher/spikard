import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_server_streaming_10_messages() -> None:
    """Tests server streaming RPC that returns a normal stream of 10 messages. Validates message ordering and complete stream delivery.."""

    from app.main import handle_grpc_server_streaming_10_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.StreamService",
        method_name="ListItems",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_server_streaming_10_messages(request)

    # Verify response
    assert response.status_code == "OK"
    assert response.payload == b'"10 messages streamed successfully"'
    assert response.metadata is not None
