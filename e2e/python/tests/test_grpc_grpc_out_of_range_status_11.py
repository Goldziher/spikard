import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_out_of_range_status_11() -> None:
    """Tests OUT_OF_RANGE gRPC status code. Returned when a value is outside the acceptable range.."""

    from app.main import handle_grpc_grpc_out_of_range_status_11

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.RangeService",
        method_name="Check",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_out_of_range_status_11(request)

    # Verify response
    assert response.metadata is not None
