import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_unavailable_status_14_service_unavailable() -> None:
    """Tests UNAVAILABLE gRPC status code. Returned when the service is temporarily unavailable.."""

    from app.main import handle_grpc_grpc_unavailable_status_14_service_unavailable

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UnavailService",
        method_name="Request",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_unavailable_status_14_service_unavailable(request)

    # Verify response
    assert response.metadata is not None
