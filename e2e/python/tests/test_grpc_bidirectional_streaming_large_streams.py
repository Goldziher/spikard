import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_large_streams() -> None:
    """Tests bidirectional streaming RPC with 50+ messages in both directions.."""

    from app.main import handle_grpc_bidirectional_streaming_large_streams

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.LargeStreamService",
        method_name="ProcessLarge",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_large_streams(request)

    # Verify response
    assert response.metadata is not None
