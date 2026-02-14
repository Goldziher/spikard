import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_unimplemented_status_12() -> None:
    """Tests UNIMPLEMENTED gRPC status code. Returned when the server does not implement the requested RPC method.."""

    from app.main import handle_grpc_grpc_unimplemented_status_12

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UnimplService",
        method_name="NotYetImplemented",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_unimplemented_status_12(request)

    # Verify response
    assert response.status_code == "UNIMPLEMENTED"
    assert response.metadata is not None
