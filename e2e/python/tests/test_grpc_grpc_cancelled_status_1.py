import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_cancelled_status_1() -> None:
    """Tests CANCELLED gRPC status code. Returned when the RPC was cancelled by the client or server.."""

    from app.main import handle_grpc_grpc_cancelled_status_1

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.CancelService",
        method_name="Operation",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_cancelled_status_1(request)

    # Verify response
    assert response.payload == b'{"id":"cancel-001"}'
    assert response.metadata is not None
