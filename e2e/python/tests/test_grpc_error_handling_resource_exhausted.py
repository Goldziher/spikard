import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_error_handling_resource_exhausted() -> None:
    """Tests bidirectional streaming RPC exceeding rate limits. Expects RESOURCE_EXHAUSTED status when client attempts to send 100 messages in rapid succession, exceeding the 100 requests/second rate limit threshold.."""

    from app.main import handle_grpc_error_handling_resource_exhausted

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.ErrorTestService",
        method_name="RateLimitedChat",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_error_handling_resource_exhausted(request)

    # Verify response
    assert response.status_code == "RESOURCE_EXHAUSTED"
    assert response.metadata is not None
