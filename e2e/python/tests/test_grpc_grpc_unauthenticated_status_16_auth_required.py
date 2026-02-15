import pytest
from spikard.grpc import GrpcRequest


@pytest.mark.asyncio
async def test_grpc_grpc_unauthenticated_status_16_auth_required() -> None:
    """Tests UNAUTHENTICATED gRPC status code. Returned when the request lacks valid authentication credentials.."""

    from app.main import handle_grpc_grpc_unauthenticated_status_16_auth_required

    # Build gRPC request from fixture
    metadata: dict[str, str] = {
        "content-type": "application/grpc",
    }
    request_payload: bytes = b"{}"
    request = GrpcRequest(
        service_name="example.v1.AuthService",
        method_name="SecureOp",
        payload=request_payload,
        metadata=metadata,
    )

    # Call handler
    response = await handle_grpc_grpc_unauthenticated_status_16_auth_required(request)

    # Verify response
    assert response.metadata is not None
