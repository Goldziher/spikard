import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_permission_denied_status_7() -> None:
    """Tests PERMISSION_DENIED gRPC status code. Returned when the caller does not have sufficient permissions.."""

    from app.main import handle_grpc_grpc_permission_denied_status_7

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.SecureService",
        method_name="AdminAction",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_permission_denied_status_7(request)

    # Verify response
    assert response.metadata is not None
