import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_internal_status_13_server_error() -> None:
    """Tests INTERNAL gRPC status code. Returned when an internal server error occurs.."""

    from app.main import handle_grpc_grpc_internal_status_13_server_error

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.InternalService",
        method_name="Fail",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_internal_status_13_server_error(request)

    # Verify response
    assert response.status_code == "INTERNAL"
    assert response.metadata is not None
