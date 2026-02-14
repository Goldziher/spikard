import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_unknown_status_2() -> None:
    """Tests UNKNOWN gRPC status code. Used for errors that do not fit any other status code.."""

    from app.main import handle_grpc_grpc_unknown_status_2

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.UnknownService",
        method_name="Fail",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_unknown_status_2(request)

    # Verify response
    assert response.status_code == "UNKNOWN"
    assert response.metadata is not None
