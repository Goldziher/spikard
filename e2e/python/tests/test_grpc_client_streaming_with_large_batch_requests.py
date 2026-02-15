import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_client_streaming_with_large_batch_requests() -> None:
    """Tests client streaming RPC with large batch requests. Validates server accumulation of multiple large client messages.."""

    from app.main import handle_grpc_client_streaming_with_large_batch_requests

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.BatchService",
        method_name="ProcessBatch",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_client_streaming_with_large_batch_requests(request)

    # Verify response
    assert response.payload == b'{"batch_id":"batch-large-001","items_processed":100,"total_bytes":5242880}'
    assert response.metadata is not None
