import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_timeout_with_retry_metadata() -> None:
    """Tests DEADLINE_EXCEEDED status code with retry metadata in response trailers. Indicates whether client should retry.."""

    from app.main import handle_grpc_timeout_with_retry_metadata

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.RetryService",
        method_name="RetryableOperation",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_timeout_with_retry_metadata(request)

    # Verify response
    assert response.metadata is not None
