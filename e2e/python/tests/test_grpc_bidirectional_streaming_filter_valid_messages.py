import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_bidirectional_streaming_filter_valid_messages() -> None:
    """Tests bidirectional streaming RPC where server filters out invalid messages during streaming.."""

    from app.main import handle_grpc_bidirectional_streaming_filter_valid_messages

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.FilterService",
        method_name="FilterValid",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_bidirectional_streaming_filter_valid_messages(request)

    # Verify response
    assert response.payload == b'[{"id":"point-1","value":10},{"id":"point-3","value":25}]'
    assert response.metadata is not None
